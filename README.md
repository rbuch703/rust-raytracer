# rust-raytracer
A basic raytracing image generator written in Rust.

The 3D scene is currently compiled directly into the application and can be modified by manipulating the `objects` vector in `main.rs`.

## Compilation
Execute `cargo build` from within the source folder. This builds the application (in debug mode) and any dependencies.

## Execution
Execute `cargo run` from within the source folder. This creates the raytraced image `image.png` in the current folder.
