#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate subprocess;

mod adb;
mod server;

//adb pull /sdcard/Download/PCS2173113117.pdf
//&["platform-tools/adb.exe", "shell", "ls", "-a -l"],
//&["platform-tools/adb.exe", "devices"],

/*
        &[
            "platform-tools/adb.exe",
            "shell",
            "ls /sdcard -R | grep -E 'jpg|png'",
        ],
*/

fn main() {
    rocket::ignite().mount("/", routes![server::ls, server::ls_empty]).launch();
    //println!("{}", adb::ls("/"));
    //println!("{}", adb::ls_images("/sdcard/Tumblr")); // only jpg and png
}
