pub fn c() {
    println!("PATCHED")
}

pub fn home_dir() -> Option<std::path::PathBuf> {
    Some(std::path::PathBuf::from("hello"))
}