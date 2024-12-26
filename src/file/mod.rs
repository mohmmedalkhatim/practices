use std::{
    fmt::format, fs::{DirBuilder, DirEntry, File, ReadDir}, io::{Read, Seek, Write}, os::windows::fs::FileExt, path::Path
};

pub fn init() -> Result<(), String> {
    let mut build = DirBuilder::new();
    let mut path = Path::new("./information");
    let _ = build.recursive(true).create(path);
    for item in ["name.txt", "info.txt", "readme.md", "cv.doc"] {
        let mut file = File::create(format!("{}/{}", path.display(), item)).unwrap();
        let mut s = String::new();
    }
    if path.is_dir() {
        let info = path.read_dir().unwrap();
        for file in info {
            println!("{:?}", file.unwrap().path())
        }
    }

    let file = File::create(path);
    println!("{}", path.display());
    let file = File::create("./names.txt").unwrap();
    Ok(())
}
