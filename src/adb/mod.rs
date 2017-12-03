#![allow(dead_code)]

use subprocess::{Popen, PopenConfig, Redirection};

pub struct Adb {}


// count and filter devices
pub fn devices() -> String {
    run_exec(&[
        "platform-tools/adb.exe",
        "devices",
    ]).expect("devices failed")
}

pub fn ls(target_folder: &str) -> String {
    run_exec(&[
        "platform-tools/adb.exe",
        "shell",
        "ls",
        target_folder,
    ]).expect("ls failed")
}

pub fn ls_images(target_folder: &str) -> String {
    run_exec(&[
        "platform-tools/adb.exe",
        "shell",
        "ls",
        target_folder,
        "| grep -E 'jpg|png'",
    ]).expect("ls failed")
}

fn run_exec(run: &[&str]) -> Option<String> {
    match Popen::create(
        run,
        PopenConfig {
            stdout: Redirection::Pipe,
            ..Default::default()
        },
    ) {
        Ok(mut adb) => {
            let (out, _) = adb.communicate(None).expect("communicate failed");

            let _ = adb.wait();

            out
        }
        Err(error) => {
            println!("Err open {}", error);
            None
        }
    }
}
