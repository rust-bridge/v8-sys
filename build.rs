use std::process::Command;
use std::env;
use std::path::Path;
use std::io::ErrorKind;

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            panic!(format!("failed to execute command: {}", e));
        }
        Err(e) => panic!(format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        panic!(format!("command did not execute successfully, got: {}", status));
    }
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    run(Command::new("g++").args(&["src/v8_glue.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/v8_glue.o", out_dir)));
    run(Command::new("ar").args(&["crus", "libv8_glue.a", "v8_glue.o"])
                      .current_dir(&Path::new(&out_dir)));

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=v8_glue");
}
