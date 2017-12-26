extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/stb_image_write.c")
        .compile("stb_image_write");
    println!("cargo:rustc-link-lib=static=stb_image_write");
}
