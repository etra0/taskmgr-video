fn main() {
    cc::Build::new()
        .file("src/interceptor.asm")
        .compile("interceptor");
    println!("cargo:rerun-if-changed=interceptor.asm");
}
