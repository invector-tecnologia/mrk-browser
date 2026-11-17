# mrk-browser

**mrk-browser** is a modular, hybrid, high-performance mini-browser written in Rust. The project is designed specifically to run natively on **Omarchy Linux**. It offers two independent viewing interfaces, selected through inversion of control at runtime, plus a specialized content transformation pipeline focused on static data conversion.

## 🚀 System Architecture

The repository is organized as a multi-crate Rust workspace that strictly separates the responsibilities of the logic engine from the visual presentation layers:

```text
mrk-browser/
├── Cargo.toml                  # Workspace manifest and global dependencies
├── mrk-core/                   # Core library (network layer and parser)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── network.rs          # Asynchronous HTTP client (Reqwest + Tokio)
│       └── parser.rs           # DOM tokenization engine (HTML -> MDX)
├── src/                        # Main hybrid orchestrator (GUI / TUI)
│   ├── main.rs                 # Entry point and runtime dispatch
│   ├── tauri_app.rs            # Graphical interface initialization and bindings
│   └── ratatui_app.rs          # Terminal render loop and events
└── deploy/                     # Native packaging artifacts
    ├── PKGBUILD                # Arch/Omarchy Linux build manifest
    └── mrk-browser.desktop     # XDG application launcher integration
```

## 🛠️ Operating Modes (Dispatch Mechanism)

The single compiled binary uses the processing primitives of the Rust ecosystem to switch the execution frame based on CLI arguments, in milliseconds:

### 1. Graphical Mode (GUI, default)

When invoked without arguments in the terminal or launched from the desktop application launcher, the core delegates rendering to the **Tauri** runtime.

```bash
mrk-browser
```

- Provides a lightweight, optimized native window based on the operating system's Webview engine.
- Includes a synchronous top navigation bar.
- Displays the converted MDX using controlled injection of web components.

### 2. Terminal Mode (TUI)

When invoked explicitly with the `--tui` flag, the Tauri lifecycle is suspended before graphics memory is allocated, and the modular **Ratatui** panel takes over.

```bash
mrk-browser --tui
```

- Fully text-based rendering, directly in your session's active terminal.
- Keyboard-shortcut-driven interface (keybindings optimized for Helix/Vim).
- Advanced conversion of structured Markdown tags into native text styles (`ratatui::text::Span`), applying logical indentation and colors inferred from the DOM.

## 🔀 Conversion Pipeline: HTML to MDX

The technical heart of `mrk-core` is its asynchronous data processing flow (pipeline parsing):

1. **Asynchronous Ingestion:** The network layer (`network.rs`) uses `reqwest` on the `tokio` runtime to fetch the raw data tree over HTTP/HTTPS.
2. **DOM Tokenization:** The `parser.rs` module consumes the string using stable node trees (the `tl` or `scraper` crates), isolating the structural body (`<body>`) and removing scripts and malicious tracking tags.
3. **MDX Compilation:** The parser rebuilds the tree, turning semantic HTML elements into clean, structured Markdown and injecting support for inline JSX blocks and custom dynamic components.

## 📦 Installation and Uninstallation (Omarchy Linux / Pacman)

The project fully complies with Arch Linux packaging guidelines, which allows native tracking and clean dependency removal through the system-wide package manager.

### Local Build and Installation

To build the native installer package using the recipe in the deploy folder, run:

```bash
cd deploy
makepkg -si
```

**What the installer does under the hood:**

1. Invokes the Rust compiler to build the workspace in optimized production mode (`cargo build --release`).
2. Moves the resulting binary to the system-wide path (`/usr/bin/mrk-browser`).
3. Copies the `.desktop` descriptor file to the system applications directory (`/usr/share/applications/`), making the graphical shortcut available right away in your Linux desktop menu.

### Clean Uninstallation

To fully remove the browser and clean up all generated symbolic links and shortcut files, call the package manager:

```bash
sudo pacman -R mrk-browser
```

## 🔒 Defensive Development Guidelines

- **Error Handling:** Macros that abruptly stop the runtime (`unwrap()`, `expect()`) are strictly forbidden. All network and file I/O operations must propagate failures cleanly through the idiomatic `Result<T, E>` type.
- **Security Isolation:** Any integration test script that simulates complex HTTP requests or disk manipulation must run inside rootless sandboxes set up with **Podman**.