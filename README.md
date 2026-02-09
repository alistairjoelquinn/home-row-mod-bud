## 🚧 WIP

An app to help you analyse your tap timings, in order to best configure your home row mod settings. This is in active development, and is currently not ready for use.

Select the modifier for each of the home row keys (or leave blank if none is assigned).

<img width="1016" height="759" alt="Screenshot 2026-02-09 at 09 36 09" src="https://github.com/user-attachments/assets/ea8ec805-da63-401c-a253-b25ffeadb9ff" />

Then complete the typing test to receive feedback about your timing, with suggested values to use in your QMK / ZMK firmware build.

<img width="1028" height="767" alt="Screenshot 2026-02-09 at 09 36 26" src="https://github.com/user-attachments/assets/a171a25b-675a-4a74-aab6-0b9e775b4d01" />

## Running locally

Requires [Rust](https://www.rust-lang.org/tools/install).

```sh
cargo run
```

To build a release binary:

```sh
cargo build --release
```
