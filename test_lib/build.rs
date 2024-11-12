extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/funcs.c")
        .opt_level(2)
        .compile("funcs");
}
