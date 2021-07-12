#[path = "build/generate.rs"]
mod generate;

fn main() {
    generate::generate().expect("Failed to generate from build.rs");

    // panic!("test");
}
