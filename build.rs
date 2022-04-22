fn main() {
    println!("cargo:rustc-link-lib=kvm");

    let mut builder = cc::Build::new();
    builder.file("foo.c");
    builder.compile("foo");
}
