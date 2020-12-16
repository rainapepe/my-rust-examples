use std::fs;
use std::path;

pub fn make_path() {
    // let volume = path::PathBuf::from("./volumes");
    println!("path: {:?}", fs::canonicalize("../volumes"));

    // let result = format!("/volumes/teste"); // , volume.to_str().unwrap(), "teste");

    // create_dir_all(result).unwrap()
}
