use std::fs::read_to_string;

fn run_cat(path: String) {
    match read_to_string(path) {
        Ok(content) => print!("{}", content),
        Err(reason) => println!("{}", reason),
    }
}

fn main() {
    run_cat("src/rc/chiikawa.aa.txt".to_string());
}
