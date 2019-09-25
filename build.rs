extern crate cc;
extern crate cpp_build;

fn main() {
    cc::Build::new()
        .files(vec!["cpp/src/lib.cc"])
        .include("cpp/include")
        .cpp(true)
        .static_flag(true)
        .static_crt(true)
        .compile("legion_filament_cpp");

    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=opengl32");

    println!("cargo:rustc-link-search=cpp/lib/x86_64/mt");
    println!("cargo:rustc-link-lib=static=filament");
    println!("cargo:rustc-link-lib=static=backend");
    println!("cargo:rustc-link-lib=static=bluegl");
    println!("cargo:rustc-link-lib=static=filabridge");
    println!("cargo:rustc-link-lib=static=filaflat");
    println!("cargo:rustc-link-lib=static=utils");
    println!("cargo:rustc-link-lib=static=geometry");
    println!("cargo:rustc-link-lib=static=smol-v");
    println!("cargo:rustc-link-lib=static=ibl");

    cpp_build::Config::new()
        .include("cpp/include")
        .build("src/main.rs");
}
