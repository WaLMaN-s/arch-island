fn main() {
    cc::Build::new()
        .file("src/x11_helper.c")
        .compile("arch_island_x11");

    println!("cargo:rustc-link-lib=xcb");
    println!("cargo:rustc-link-lib=xcb-ewmh");
    println!("cargo:rustc-link-lib=xcb-icccm");

    tauri_build::build();
}
