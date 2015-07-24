extern crate gcc;

fn main() {
    gcc::Config::new().cpp(true).file("src/v8_glue.c").compile("libv8_glue.a");
}
