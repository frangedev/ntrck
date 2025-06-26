# 🚀 Quick Start Guide

Get ntrck running in minutes!

## Prerequisites

- **Rust** (latest stable) - [Install here](https://rustup.rs/)
- **Node.js** (optional, for development tools)
- **Python 3** (for simple HTTP server)

## 🛠️ Setup

1. **Clone and navigate to the project:**
   ```bash
   cd ntrck
   ```

2. **Build the WASM module:**
   ```bash
   ./build.sh
   ```

3. **Start the frontend:**
   ```bash
   cd static
   python3 -m http.server 8080
   ```

4. **Open your browser:**
   ```
   http://localhost:8080
   ```

## 🎵 Using ntrck

### Basic Controls
- **Play/Stop**: Click the ▶️ button to start/stop playback
- **BPM**: Adjust tempo with the BPM input
- **Pattern Grid**: Click on step cells to add notes

### Creating Patterns
1. Select a waveform (Sine, Square, Saw, Triangle)
2. Choose a note (C, D, E, F, G, A, B, C5)
3. Set volume with the slider
4. Click on grid cells to place notes
5. Press Play to hear your pattern!

### Pattern Controls
- **New Pattern**: Create a fresh 16-step pattern
- **Clear**: Remove all notes from current pattern

## 🔧 Development

### Project Structure
```
ntrck/
├── wasm/              # Rust audio engine (compiled to WASM)
├── static/            # Frontend (HTML/CSS/JS)
├── server/            # Backend API (optional)
├── samples/           # Audio samples and presets
└── build.sh           # Build script
```

### Building WASM
```bash
cd wasm
wasm-pack build --target web
```

### Running Backend (Optional)
```bash
cd server
cargo run
```

### Development Server
For development with hot reloading:
```bash
# Install a simple dev server
npm install -g live-server

# Run with live reload
live-server static --port=8080
```

## 🐛 Troubleshooting

### WASM Build Issues
- Ensure you have the latest Rust toolchain: `rustup update`
- Install wasm-pack: `cargo install wasm-pack`
- Clear build cache: `cd wasm && cargo clean`

### Audio Issues
- Check browser console for Web Audio API errors
- Ensure browser supports Web Audio API
- Try refreshing the page if audio context fails

### CORS Issues
- Use the provided Python server or live-server
- Don't open HTML files directly in browser

## 📚 Next Steps

- Check out the `samples/` directory for presets
- Explore the Rust/WASM code in `wasm/src/`
- Customize the UI in `static/styles.css`
- Add new features to `static/app.js`

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

Happy tracking! 🎛️ 