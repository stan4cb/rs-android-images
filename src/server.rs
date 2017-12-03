
#[get("/ls")]
pub fn ls_empty() -> String {
    ::adb::ls("/")
}

#[get("/ls/<p>")]
pub fn ls(p: String) -> String {
    ::adb::ls(&p)
}