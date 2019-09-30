# Legion Filament

This is an experiment to see what using Google Filament is like from Rust, in
the context of a Legion-based ECS. This is **not** a wrapper around Filament, or
a general purpose crate. The actual interaction with Filament is done in C++,
which is wrapped via `bindgen` for Rust usage. Windowing is done with `winit`
although Filament itself handles all context creation and management.

## Building

_Only Windows and OSX are currently supported._

### Filament deps (Required)

The Filament deps are absurdly large (2.77GB extracted for Windows) so they are
not committed to git (or even git lfs). You need to download and extract them
yourself:

- Download and extract the tarball for Windows or OSX from:
  https://github.com/google/filament/releases
- Copy the `lib` and `include` folders to `cpp` (`./cpp/lib/` and
  `./cpp/include/` respectively).
- _Optional: If you wish to re-compile materials from source, copy the `bin`
  folder to the root (`./bin`)._

### Running

```sh
cargo run
```
