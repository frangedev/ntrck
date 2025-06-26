# 🎛️ ntrck

**ntrck** is a low-level, high-performance music tracker web app built in **Rust** and **WebAssembly**. It focuses on precision timing, minimal UI, and raw audio power — inspired by classic trackers, built with modern systems programming.

---

## 🚀 Features

- 🔉 Real-time audio synthesis and scheduling with **Web Audio API**
- ⚙️ DSP and sequencing core written in **Rust → WebAssembly**
- 🖥️ UI built using **Vanilla JavaScript + Canvas**
- 🎼 Tracker-style step sequencing grid
- ⏱️ Sample-accurate event timing
- 💾 Optional backend in Rust with Actix Web (for multi-user sessions or save/load)

---

## 📦 Tech Stack

| Layer      | Tech                     |
|------------|--------------------------|
| Audio DSP  | `Rust` → `WASM`          |
| Frontend   | `HTML` + `JS` + `Canvas` |
| Audio API  | `Web Audio API`          |
| Backend    | `Rust + Actix` *(optional)* |
| Storage    | `SQLite / PostgreSQL` *(optional)* |

---

## 🧪 Getting Started

### 🛠 Build WASM (Rust)
```bash
wasm-pack build --target web
````

### 🌐 Serve Frontend

```bash
python3 -m http.server 8080
# or use any static file server
```

### 🧪 Run Backend (if needed)

```bash
cargo run --bin server
```

---

## 📁 Project Structure

```
ntrck/
├── wasm/              # Rust audio + sequencing logic compiled to WebAssembly
├── static/            # HTML, JS, and Canvas-based frontend
├── server/            # (Optional) Actix Web backend for session handling
├── samples/           # Default sound pack or module loader
└── README.md
```

---

## 🧠 Inspiration

* FastTracker II, LSDJ, MilkyTracker
* Love for tick-precise pattern sequencing
* Hacker-style web audio tools with low-overhead performance

---

## 🤘 Contribute

Pull requests welcome. Got tracker brain? Let's build it together.

## 🚧 Future Improvements

- Pattern management UI (list, select, delete, save/load patterns)
- Song mode: chain patterns for full song playback
- Pattern export/import (JSON)
- Download/upload pattern buttons in UI
- Metronome/click option
- User-facing error banners/popups in frontend
- More granular error types in Rust/WASM
- Backend: persist sessions/patterns to SQLite or file
- Backend: pattern CRUD endpoints
- Backend: user authentication (optional)
- Audio engine: add effects (delay, reverb, filter)
- Mobile/touch UI improvements
- Accessibility improvements
- Internationalization/localization

## 🐞 Known Issues & Limitations

- Only one pattern can be played at a time (no song mode)
- No persistent storage for patterns or sessions
- No authentication or user management
- No UI for pattern switching, saving, or loading
- Error handling is basic; user-facing errors are not always shown
- Audio engine is minimal (no effects, limited waveform support)
- Backend is optional and not integrated with frontend yet

---

See `QUICKSTART.md` for setup instructions and `CONTRIBUTE.md` for how to help!

---

## 📜 License

MIT License
