use std::fs;
use std::fs::File;  // Cambiamos esto de tokio a std
use std::path::{Path, PathBuf};
use std::error::Error;
use std::io::{Write};
use std::process::Command;
use std::time::Duration;

// Imports de crates externos
use base64;
use whisper_rs::{WhisperContext,FullParams,SamplingStrategy,WhisperContextParameters,WhisperError};
use hound;

const MODEL_PATH: &str = "ggml-base.bin";

#[tokio::main]
async fn main() {

    let api_path = Path::new("./src/apikey.txt");
    let audio_path = Path::new("./build/output.wav");
    let video_path = Path::new("./src/highRes.mp4");
    let apikey = fs::read_to_string(api_path)
        .expect("Should have been able to read the file!");
    let response = request_prompt(apikey.as_str()).await.unwrap();

    match text_to_speech(apikey.trim(), response.as_str()).await {
    Ok(_) => {
        match combine_audio_video(audio_path, video_path) {
            Ok(video) => {
                match generate_subtitles(&audio_path) {
                    Ok(transcription) => {
                        match add_subtitles(&video, transcription) {
                            Ok(final_video) => println!("Video processing completed successfully: {:?}", final_video),
                            Err(e) => eprintln!("Error adding subtitles: {}", e),
                        }
                    },
                    Err(e) => eprintln!("Error generating subtitles: {}", e),
                }
            },
                Err(e) => eprintln!("Error combining audio and video: {}", e),
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn get_audio_duration(audio_path: &Path) -> Result<Duration, Box<dyn Error>> {
    let reader = hound::WavReader::open(audio_path)?;
    let spec = reader.spec();
    let duration = Duration::from_secs_f64(
        reader.duration() as f64 / spec.sample_rate as f64
    );
    Ok(duration)
}


fn scale_whisper_time(whisper_time: i32, max_whisper_time: i32, actual_duration_secs: f64) -> i32 {
    // Convert whisper time to seconds
    let scaled_time = (whisper_time as f64 / max_whisper_time as f64) * actual_duration_secs;

    // Convert to centiseconds (1/100th of a second)
    (scaled_time * 100.0) as i32
}

fn combine_audio_video(audio_path: &Path, video_path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let output_path = PathBuf::from("./build/final_video.mp4");

    Command::new("ffmpeg")
        .args([
            "-i", video_path.to_str().unwrap(),
            "-i", audio_path.to_str().unwrap(),
            "-c:v", "copy",
            "-c:a", "aac",
            output_path.to_str().unwrap()
        ])
        .output()?;

    Ok(output_path)
}

fn generate_subtitles(audio_path: &Path) -> Result<PathBuf, Box<dyn Error>> {

    let mut max_whisper_time = 0;
    let audio_duration = get_audio_duration(audio_path)?;
    let duration_secs = audio_duration.as_secs_f64();

    // Leer el archivo WAV
    let samples: Vec<i16> = hound::WavReader::open(audio_path)
        .unwrap()
        .into_samples::<i16>()
        .map(|x| x.unwrap())
        .collect();

    let language = "en";
    let ctx = WhisperContext::new_with_params(MODEL_PATH, WhisperContextParameters::default())
        .expect("failed to load model");
    let mut state = ctx.create_state().expect("failed to create state");
    let mut params = FullParams::new(SamplingStrategy::BeamSearch { beam_size: 5, patience: 1.0, });
    params.set_language(Some(&language));

    let mut inter = vec![0.0f32; samples.len()];
    convert_integer_to_float_audio(&samples, &mut inter)
        .expect("failed to convert audio data");

    let samples = whisper_rs::convert_stereo_to_mono_audio(&inter)
        .expect("failed to convert audio data");

    state
        .full(params, &samples[..])
        .expect("failed to run model");


    let  base_path = PathBuf::from("./build/subtitles.srt");
    let mut srt_file = File::create(&base_path)?;


    let num_segments = state.full_n_segments()?;

    for i in 0..num_segments {
        let end_time = state.full_get_segment_t1(i)? as i32;
        max_whisper_time = max_whisper_time.max(end_time);
    }

    for i in 0..num_segments {
        let segment = state.full_get_segment_text(i)?;
        // Convertir los tiempos a segundos antes de formatearlos
        let start_time = state.full_get_segment_t0(i)? as i32;
        let end_time = state.full_get_segment_t1(i)? as i32;
        let scaled_start = scale_whisper_time(start_time, max_whisper_time, duration_secs);
        let scaled_end = scale_whisper_time(end_time, max_whisper_time, duration_secs);


        println!("{}--{}",start_time, end_time);
        write!(srt_file, "{}\n{} --> {}\n{}\n\n",
            i + 1,  format_time(scaled_start), format_time(scaled_end), segment)?;
    }

    Ok(base_path)
}

fn convert_integer_to_float_audio(
    samples: &[i16],
    output: &mut [f32],
) -> Result<(), WhisperError> {
    if samples.len() != output.len() {
        return Err(WhisperError::InputOutputLengthMismatch {
            input_len: samples.len(),
            output_len: output.len(),
        });
    }

    for (input, output) in samples.iter().zip(output.iter_mut()) {
        *output = *input as f32 / 32768.0;
    }

    Ok(())
}

fn format_time(time_hundredths: i32) -> String {
    let total_ms = time_hundredths * 10;  // Convertir centésimas de segundo a milisegundos

    let hours = total_ms / 3600000;
    let minutes = (total_ms % 3600000) / 60000;
    let seconds = (total_ms % 60000) / 1000;
    let milliseconds = total_ms % 1000;

    format!("{:02}:{:02}:{:02},{:02}", hours, minutes, seconds, milliseconds / 10)
}

fn add_subtitles(video_path: &Path, subtitles_path: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
    let output_path = PathBuf::from("./build/video_with_subtitles.mp4");

    Command::new("ffmpeg")
        .args([
            "-i", video_path.to_str().unwrap(),
            "-vf", &format!("subtitles={}:force_style='Alignment=2,FontSize=24'", subtitles_path.to_str().unwrap()),
            "-c:v", "libx264",  // Especificar codec de video
            "-c:a", "aac",      // Mantener el audio
            "-y",               // Sobrescribir si existe
            output_path.to_str().unwrap()
        ])
        .output()?;

    Ok(output_path)
}

async fn request_prompt(apikey: &str) -> Result<String, Box<dyn Error>>{
    let query =  format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={apikey}");
    //println!("{}",query);
    let client = reqwest::Client::new();
    let res = client.post(query )
        .body(r#"{
            "contents": [{
                "parts":[{
                "text": "I can't sleep before i get my new story every day. Write me a scary story so I can sleep. In first person. For a speech that should last between 45 segs and 1 min"}]
            }]
        }"#)
        .send()
        .await?;
    if res.status().is_success(){

        let json: serde_json::Value = serde_json::from_str(res.text().await?.as_str()).expect("JSON was not well-formatted");
        if let Some(candidates) = json.get("candidates"){
            if let Some(candidate) = candidates.get(0){
                if let Some(content) = candidate.get("content"){
                    if let Some(parts) = content.get("parts"){
                        if let Some(part) = parts.get(0){
                            if let Some(text) = part.get("text"){
                                return Ok(text.as_str().unwrap_or("No text found").to_string());
                            }
                        }
                    }
                }
            }
        }



    }
    Err("Unable to retrieve text from the response".into())

}
async fn text_to_speech(apikey: &str, text: &str) -> Result<(), Box<dyn Error>> {
    let query = format!("https://texttospeech.googleapis.com/v1/text:synthesize?key={apikey}");

    // Configure the request payload
    let payload = serde_json::json!({
        "input": {
            "text": text
        },
        "voice": {
            "languageCode": "en-GB",
            "name": "en-GB-Journey-D",
        },
        "audioConfig": {
            "audioEncoding": "LINEAR16"
        }
    });

    let client = reqwest::Client::new();
    let res = client.post(query)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&payload)?)
        .send()
        .await?;



// Check response status
    if !res.status().is_success() {
        return Err(format!("Failed to synthesize speech. HTTP Status: {}", res.status()).into());
    }

    // Parse response
    let response_body: serde_json::Value = res.json().await?;
    if let Some(audio_content) = response_body.get("audioContent") {
        // Decode audio content from Base64
        let audio_data = base64::decode(audio_content.as_str().unwrap_or(""))?;

        // Save audio to file
        let mut file = fs::File::create("./build/output.wav")?;
        file.write_all(&audio_data)?;
        println!("Audio content saved to 'output.wav'");
        return Ok(());
    }

    Err("Failed to retrieve audio content from response.".into())
}

