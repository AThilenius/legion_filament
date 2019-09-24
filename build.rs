extern crate cpp_build;

fn main() {
    let include_path = "cpp/include";
    let lib_path = "cpp/lib";
    cpp_build::Config::new().include(include_path).build("src/main.rs");

    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=opengl32");

    println!("cargo:rustc-link-search={}", lib_path);
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
