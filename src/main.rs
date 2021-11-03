use std::process;

fn main() {
    if let Err(e) = whitted::run() {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
