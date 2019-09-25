# Legion Filament

This is an experiment to see what using Google Filament is like from Rust, in
the context of a Legion-based ECS. FFI is done with the `cpp` crate, and window
creation with the `winit`.

## Building

The Filament deps are absurdly large (2.77GB extracted for Windows) so they are
not committed to git (or even git lfs). You need to download and extract them
yourself.

> Note: This uses the Windows tarball. Other systems can be found at
> https://github.com/google/filament/releases

```sh
# Get the tarball
wget https://github.com/google/filament/releases/download/v1.3.2/filament-20190827-windows.tgz

# Extract `include` and `lib` into `cpp`.
tar -C cpp -xvf filament-20190827-windows.tgz include
tar -C cpp -xvf filament-20190827-windows.tgz lib

# Extract `bin` to the root.
tar -xvf filament-20190827-windows.tgz bin

# Cleanup the tarball
rm *.tgz

# Cargo build (builds C++ as well).
cargo build
```

Building has only been tested on Windows. It uses static linking `/MT`.
