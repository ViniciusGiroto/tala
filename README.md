# Tala
Tala -- named after the Tagalog goddess of stars -- is a software for designing and simulating simple optical systems.

# Compiling and Running
Tala is written in Rust and can be compiled using the following command:

```bash
cargo build --release
```

or run from source

```bash
cargo run --release
```

# Notes
GPU ray tracing is disabled by default. It is meant to be used in the future for ray tracing simulations, but I am currently waiting on some rust-gpu updates to allow for better Rust to SPIR-V support.
