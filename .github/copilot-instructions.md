# delta-t-zaman Development Guidelines

Auto-generated from all feature plans. Last updated: 2026-04-12

## Active Technologies
- Rust stable >= 1.77.2 (backend) + TypeScript 5.x (frontend) + Style Dictionary 4.x (build-time), SunCalc, tauri-plugin-notification 2.3.3, tauri-plugin-store 2.x, tauri-plugin-autostart 2.x, tauri-plugin-os 2.x, tauri-plugin-single-instance 2.x, tauri-plugin-positioner 2.x (001-tauri-world-clock)
- tauri-plugin-store (JSON key-value: zones.dat, alarms.dat, preferences.dat) (001-tauri-world-clock)

## Project Structure

```text
src/
tokens/
src-tauri/
tests/
```

## Commands

cargo test
cargo clippy
npm run build:tokens

## Code Style

TypeScript 5.x (vanilla, no framework): Follow standard conventions
Rust stable >= 1.77.2: Follow Clippy recommendations

## Recent Changes
- 001-tauri-world-clock: Added Rust stable >= 1.77.2 (backend) + TypeScript 5.x (frontend) + Style Dictionary 4.x (build-time), SunCalc, tauri-plugin-notification 2.3.3, tauri-plugin-store 2.x, tauri-plugin-autostart 2.x, tauri-plugin-os 2.x, tauri-plugin-single-instance 2.x, tauri-plugin-positioner 2.x

<!-- MANUAL ADDITIONS START -->
<!-- MANUAL ADDITIONS END -->
