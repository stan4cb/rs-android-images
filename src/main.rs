extern crate subprocess;

mod adb;

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
    println!("{}", adb::devices());
    //println!("{}", adb::ls("/"));
    //println!("{}", adb::ls_images("/sdcard/Tumblr")); // only jpg and png
}
