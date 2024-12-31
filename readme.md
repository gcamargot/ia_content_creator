# IA Content Creator
A Rust-based bot designed to automate the creation and distribution of multimedia content. Here's an overview of its workflow:

Text Content Creation: The bot uses prompts to generate text-based stories via the Gemini platform.
Video Content Generation: It uses Gemini Text to Speech and Whistler_RS to create a video
Social Media Uploads: The final output is uploaded to social media platforms, including TikTok, Instagram, and YouTube.

## ⚡ Features

- Custom subtitle rendering with semi-transparent background
- Smart positioning of subtitles within a contained box
- Support for various video formats through FFmpeg
- SRT subtitle file support
- Real-time video processing
- Font customization including size and style

## 🛠️ Tech Stack

- **Rust** - Core programming language
- **FFmpeg** - Video processing engine
- **PathBuf** - Path manipulation
- **Error handling** with custom error types
- **Command** execution system for FFmpeg integration
- **Result type** for robust error management

## 🚀 Getting Started

### Prerequisites

- FFmpeg installed on your system
- Rust 1.70 or higher
- Titan One font installed (or modify the code to use a different font)

### Installation

```bash
# Clone the repository
git clone https://github.com/gcamargot/video-subtitle-overlay

# Navigate to the project directory
cd video-subtitle-overlay

# Build the project
cargo build --release
```

### Basic Usage

```rust
use video_subtitle_overlay::add_subtitles;
use std::path::PathBuf;

fn main() {
    let result = add_subtitles(
        PathBuf::from("input.mp4"),
        PathBuf::from("subtitles.srt"),
        PathBuf::from("output.mp4")
    );
}
```

## 📝 Configuration

Current default settings:
```rust
Font: "Titan One"
FontSize: 24
TextColor: White
OutlineColor: Black
BackgroundOpacity: 0.5
MarginVertical: 20
MarginHorizontal: 40
```

## 🗺️ Roadmap

### Phase 1 - Core Features
- [x] Basic subtitle overlay
- [x] Semi-transparent background
- [x] Custom font support
- [ ] Multiple subtitle formats support
- [ ] Command-line interface

### Phase 2 - Enhanced Features
- [ ] Custom background shapes
- [ ] Dynamic background sizing
- [ ] Multiple subtitle tracks
- [ ] Font fallback system
- [ ] Real-time preview

### Phase 3 - Advanced Features
- [ ] GPU acceleration
- [ ] Batch processing
- [ ] Progress bar
- [ ] Custom effects
- [ ] Web interface

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 🐛 Known Issues

- Fixed background box size
- Limited to SRT subtitle format
- Requires Titan One font
- No support for right-to-left languages yet

## 📋 Requirements

### System Requirements
- FFmpeg 4.0 or higher
- 4GB RAM minimum
- Any modern CPU

### Development Requirements
- Rust 1.70+
- Cargo package manager
- FFmpeg development libraries

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test subtitle_processing
```

## 📖 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## 🙏 Acknowledgments

- FFmpeg team for their amazing video processing tool
- Rust community for support and libraries
- All contributors who have helped shape this project

## 📬 Contact

Project Link: [https://github.com/yourusername/video-subtitle-overlay](https://github.com/yourusername/video-subtitle-overlay)

---

Made with ❤️ by [Your Name]

Feel free to star ⭐ this repository if you find it helpful!
