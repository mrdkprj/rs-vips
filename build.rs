fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=dylib=libvips");
    }
    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:rustc-link-lib=vips");
        println!("cargo:rustc-link-lib=glib-2.0");
        println!("cargo:rustc-link-lib=gobject-2.0");
    }
}
