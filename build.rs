fn main() {
    println!("cargo:rustc-link-lib=z");

    let mut builder = cc::Build::new();
    builder.file("foo.c");
    builder.compile("foo");
}
