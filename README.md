## CHIP-8 on WebAssembly

A minimal, readable CHIP‑8 runtime written in Rust and compiled to WebAssembly. This repository is a small, pragmatic starter you can fork to build multiple CHIP‑8 apps that run in the browser.

### What’s inside
- Rust core with a compact `Chip8` VM (memory, registers, stack, display, timers)
- WebAssembly bindings via `wasm-bindgen`
- Simple 64×32 Canvas renderer
- Demo: move a green square with WASD

### Current state
This template boots with a tiny program preloaded into memory:
- A 4×4 sprite at `0x300`
- A short program at `0x200`: `ANNN (I=0x300)`, set `V0/V1`, `DRW V0,V1,4`, then loop

For the browser demo, the render loop resets `PC` to the `DRW` instruction each frame and executes one `tick()` to redraw at the new `V0/V1` based on WASD input. Implemented opcodes so far include:
- `00E0` (CLS)
- `1NNN` (JP addr)
- `ANNN` (LD I, addr)
- `DXYN` (DRW Vx, Vy, n)

### Project layout
```
.
├── src/lib.rs        # CHIP-8 core + wasm exports
├── index.html        # Browser UI + input + render loop
├── pkg/              # Generated JS bindings and .wasm (from wasm-bindgen/wasm-pack)
├── Cargo.toml
└── Cargo.lock
```

### Quick start
Prerequisites:
- Rust toolchain
- Any static file server (to serve `.wasm` with the right MIME type)

Run without rebuilding (using the existing `pkg/`):
```bash
# from the repository root
python3 -m http.server 8000
# or:
npx serve .
```
Open `http://localhost:8000` and use WASD to move the green square.

### Rebuild the WebAssembly package
Option A — wasm-pack (recommended):
```bash
cargo install wasm-pack          # if not installed
wasm-pack build --target web --out-dir pkg --release
```

Option B — cargo + wasm-bindgen-cli:
```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
cargo install wasm-bindgen-cli   # if not installed
wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/chip8_wasm.wasm
```

### Browser-side API (from wasm)
Exports you can call from JS (see `index.html`):
- `new Chip8()`
- `chip8.tick()` — fetch/decode/execute one opcode
- `chip8.get_display_ptr()` — returns a pointer to the 64×32 display buffer
- `chip8.move_v0(delta: i8)`, `chip8.move_v1(delta: i8)` — tweak `V0/V1` for demo
- `chip8.reset_pc_for_test()` — resets `PC` to the demo `DRW` for the render loop

Display buffer:
- Backed by a Rust `[bool; 64*32]`, read in JS as `Uint8Array(wasm.memory.buffer, ptr, 64*32)`.

### Building real CHIP‑8 apps on top
Suggested next steps for your fork:
- ROM loader: load a `.ch8` binary into memory at `0x200`
- Timers: update `delay_timer` and `sound_timer` at 60 Hz
- Keypad: map browser keys to CHIP‑8’s 16‑key keypad
- CPU: implement the remaining opcodes and proper cycle stepping
- Rendering: consider storing the display as `u8` for faster JS interop
- Loop control: remove `reset_pc_for_test()` and run a real fetch/decode/execute loop

### Troubleshooting
- Blank screen: serve via a local web server (not `file://`) so `.wasm` loads with the correct MIME type. Also ensure `I` points to a valid sprite before `DRW`.
- Fetch error or MIME warning: use a server like `python3 -m http.server` or `npx serve` from the repo root.

### Why this template?
The goal is clarity and a small surface area so you can quickly branch off and focus on the CHIP‑8 app or feature you want to build, without getting lost in scaffolding.

---
If you fork this for multiple CHIP‑8 apps, consider keeping each app in its own branch or directory and sharing the core VM code as a small crate or module. Happy hacking!
*** End Patch