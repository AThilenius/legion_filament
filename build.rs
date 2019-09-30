extern crate bindgen;
extern crate cc;

use std::{env, path::PathBuf, fs::File};

#[cfg(target_os = "macos")]
fn link() {
  println!("cargo:rustc-link-search=cpp/lib/x86_64");

  println!("cargo:rustc-link-lib=static=filament");
  println!("cargo:rustc-link-lib=static=backend");
  println!("cargo:rustc-link-lib=static=bluegl");
  println!("cargo:rustc-link-lib=static=bluevk");
  println!("cargo:rustc-link-lib=static=filabridge");
  println!("cargo:rustc-link-lib=static=filaflat");
  println!("cargo:rustc-link-lib=static=utils");
  println!("cargo:rustc-link-lib=static=geometry");
  println!("cargo:rustc-link-lib=static=smol-v");
  println!("cargo:rustc-link-lib=static=ibl");

  println!("cargo:rustc-link-lib=framework=Cocoa");
  println!("cargo:rustc-link-lib=framework=Metal");
  println!("cargo:rustc-link-lib=framework=CoreVideo");
}

#[cfg(target_os = "windows")]
fn link() {
  println!("cargo:rustc-link-search=cpp/lib/x86_64/mt");

  println!("cargo:rustc-link-lib=gdi32");
  println!("cargo:rustc-link-lib=user32");
  println!("cargo:rustc-link-lib=opengl32");

  println!("cargo:rustc-link-lib=static=filament");
  println!("cargo:rustc-link-lib=static=backend");
  println!("cargo:rustc-link-lib=static=bluegl");
  println!("cargo:rustc-link-lib=static=filabridge");
  println!("cargo:rustc-link-lib=static=filaflat");
  println!("cargo:rustc-link-lib=static=utils");
  println!("cargo:rustc-link-lib=static=geometry");
  println!("cargo:rustc-link-lib=static=smol-v");
  println!("cargo:rustc-link-lib=static=ibl");
}

#[cfg(target_os = "macos")]
fn cc_build(source: Vec<&str>) {
  cc::Build::new()
    .files(source)
    .include("cpp/include")
    .cpp(true)
    .flag("-std=c++14")
    .static_flag(true)
    .compile("legion_filament_cpp");
}

#[cfg(target_os = "windows")]
fn cc_build(source: Vec<&str>) {
  cc::Build::new()
    .files(source)
    .include("cpp/include")
    .cpp(true)
    .static_crt(true)
    .compile("legion_filament_cpp");
}

/// Use Bindgen to generate bindings from the `wrapper.h` header.
fn generate_bindings() {
  let bindings = bindgen::Builder::default()
    .clang_arg("-Icpp/include")
    .header("cpp/src/wrapper.hpp")
    .generate()
    .expect("Unable to generate bindings");

  // Write the bindings to the $OUT_DIR/bindings.rs file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
}

fn download_filament_prebuilt(url: &str) -> String {
    let temp_dir = env::temp_dir();
    let filename = url.split("/").unwrap().last().unwrap();
    let tar_file_path = temp_dir.join(filename);

    if tar_file_path.exists() {
      return tar_file_path;
    }

    // Download the tarball.
    let f = File::create(&tar_file_path).unwrap();
    let mut writer = BufWriter::new(f);
    let mut easy = Easy::new();
    easy.url(&url).unwrap();
    easy.write_function(move |data| Ok(writer.write(data).unwrap())).unwrap();
    easy.perform().unwrap();

    let response_code = easy.response_code().unwrap();
    if response_code != 200 {
        panic!(
            "Unexpected response code while downloading prebuilt {} for {}",
            response_code, binary_url
        );
    }

    // Extract the tarball.
    let unpacked_dir = download_dir.join(base_name);
    let lib_dir = unpacked_dir.join("lib");
    let framework_library_file = format!("lib{}.so", FRAMEWORK_LIBRARY);
    let library_file = format!("lib{}.so", LIBRARY);
    let framework_library_full_path = lib_dir.join(&framework_library_file);
    let library_full_path = lib_dir.join(&library_file);
    if !framework_library_full_path.exists() || !library_full_path.exists() {
        extract(file_name, &unpacked_dir);
    }
}

fn main() {
  generate_bindings();

  let source = vec!["cpp/src/rendering_system.cc"];

  link();
  cc_build(source);

  // Also re-run if any C++ source changes (useful for dev)
  println!("cargo:rerun-if-changed=cpp/src/**/*");
}
