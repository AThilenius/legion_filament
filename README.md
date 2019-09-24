# Legion Filament

This is an experiment to see what using Google Filament is like from Rust, in
the context of a Legion-based ECS. FFI is done with the `cpp` crate, and window
creation with the `winit`.

## Building (Windows only)
Download the `*windows.tgz` from https://github.com/google/filament/releases and
un-tar it. Copy all of `lib/x86_64/mt` into the local project under `cpp/lib`.
The libs are not committed to git because they are absurdly large.
