
use std::path::PathBuf;

#[get("/ls")]
pub fn ls_empty() -> String {
    ::adb::ls("/")
}

#[get("/ls/<p..>")]
pub fn ls(p: PathBuf) -> String {
    let mut n = p.to_str().expect("ex").to_string();

    n = n.replace("\\", "/");

    println!("ls -> {}", &n);

    ::adb::ls(&n)
}