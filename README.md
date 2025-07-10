# RustStreamTV

**RustStreamTV** is a lightweight, cross-platform media-server written in Rust that streams local video / audio files to your **Samsung Smart TV** (or any modern browser) over your home network.

* No cloud accounts
* No heavyweight database
* No Plex subscription

Just **cargo run — and play**.

---

## ✨ Features

| Status | Feature                                                                               |
|--------|---------------------------------------------------------------------------------------|
| ✅      | Serve any file in a configurable media folder over HTTP                               |
| ✅      | JSON API for device info and media listing                                            |
| ✅      | Streams files with correct `Content-Type` for the TV’s `<video>` / `<audio>` elements |
| ✅      | Auto-discoverable via **SSDP** (TV finds the PC automatically)                        |
| 🟡     | Range / seek support for fast-forward & rewind                                        |
| 🟡     | mDNS / Bonjour discovery (optional)                                                   |
| 🟡     | Thumbnail & metadata extraction (FFmpeg)                                              |
| 🔜     | WebSocket control channel for live play/pause/skip feedback                           |
| 🔜     | On-the-fly transcoding for uncommon codecs                                            |

Legend: **✅ implemented** | **🟡 in progress** | **🔜 planned**

---

## 📸 Demo

<!-- Replace with your own GIF / screenshot -->
🔜

---

## 🏗 Architecture

```text
┌────────────────────────┐      SSDP      ┌─────────────────────────┐
│ Samsung Smart TV App   │◀──────────────▶│ RustStreamTV (your PC)  │
│  • HTML/JS (Tizen)     │   HTTP JSON    │  • Axum HTTP server     │
│  • Remote control UI   │───────────────▶│  • File scanner         │
└────────────────────────┘   Stream media │  • Config TOML          │
                                       │  │  • Tokio async runtime  │
                                       └──┘   
```

## 🖥 Prerequisites
| Requirement | Version                             |
|-------------|-------------------------------------|
| Rust        | stable 1.78+                        |
| Cargo       | comes with Rust                     |
| FFmpeg      | optional – thumbnails / transcoding |
| Samsung TV  | 2016+ model in Developer Mode       |



## 🚀 Quick Start
```bash
    # 1. Clone
git clone https://github.com/hamaMk/RustStreamTv.git
cd ruststreamtv

# 2. Edit configuration
nano config/default.toml     # set your media folder & device name

# 3. Run
cargo run --release
```

You should see:
```bash
    RustStream [Hamandishe-PC] listening on http://your-ip-address:your-port
```

Now install / launch the RustStreamTV app on your Samsung TV.
It will auto-detect “Hamandishe-PC”. Browse & play!


## ⚙️ Configuration (config/default.toml)
```toml
    # Folder to scan for media (recursive)
folder = "/home/hamandishe/Videos"

# TCP port to serve HTTP
port = 8000

# Friendly name shown on TV
device_name = "Hamandishe-PC"
```

You can keep multiple profiles (e.g. livingroom.toml) and pass the path with RUSTSTREAM_CONFIG=/path/to/file.toml.


## 🛠️ Building a Release Binary
```bash
    cargo build --release       # target/release/ruststreamtv
```
On macOS / Linux you can strip symbols:
```bash
    strip target/release/ruststreamtv
```

Copy the binary and config folder to any PC, Mac, or Linux box on your LAN.


## 🔌 API Reference
| Method | Path                | Description                                                |
|--------|---------------------|------------------------------------------------------------|
| GET    | `/device-info`      | Returns `{ "name": "Hamandishe-PC", "platform": "linux" }` |
| GET    | `/media`            | Lists files `[ { name, size, extension, path }, … ]`       |
| GET    | `/media/{filename}` | Streams the file (supports HTTP range soon)                |

All responses are JSON except raw media streams.

## 💡 Developing / Contributing
1. Fork the repo
2. cargo clippy --all-targets --all-features -- -D warnings
3. cargo test
4. Open a PR!

We welcome PRs for mDNS, thumbnails, GUI config, Windows MSI, etc.

## 📜 License
Licensed under the MIT License – [`LICENSE.md`](LICENSE.md) for details.

## 🙌 Credits
* Built with Axum, Tokio, and the Rust community.
* Inspired by open-source media servers everywhere.
* Icons by Lucide.

Give the repo a ⭐ if you find it useful, and show it off on LinkedIn!
Happy streaming! 🚀