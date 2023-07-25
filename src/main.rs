use std::{
    fs::{create_dir, read, read_dir, remove_file},
    io::Cursor,
    path::PathBuf,
};

use sb3::SB3;

mod sb3;

fn main() {
    // create /project
    match create_dir("target/project") {
        Ok(_) => println!("Created target/project directory"),
        Err(_) => {
            // remove the contents of /project
            for entry in read_dir("target/project").unwrap() {
                let entry = entry.unwrap();
                remove_file(entry.path()).unwrap();
            }
        }
    }

    let target_dir = PathBuf::from("target/project");

    let archive = read("project.sb3").unwrap();

    zip_extract::extract(Cursor::new(archive), &target_dir, true).unwrap();

    let json = read("target/project/project.json").unwrap();

    let project: SB3 = serde_json::from_slice(&json).unwrap();

    println!("{:#?}", project);
}
