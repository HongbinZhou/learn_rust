extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/collatz.c")
        .compile("collatz");

    // Instead of do build in build.rs, we can directly link libcollatz by
    // search given path:
    // println!(r"cargo:rustc-link-search=.");
}
