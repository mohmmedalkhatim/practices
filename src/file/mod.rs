use std::{
    fmt::format,
    fs::{DirBuilder, DirEntry, File, ReadDir},
    io::{Read, Seek, Write},
    path::Path,
};

pub fn init() -> Result<(), String> {
    let mut build = DirBuilder::new();
    let mut path = Path::new("./information");
    let _ = build.recursive(true).create(path);
    for item in ["name.txt", "info.txt", "readme.md", "cv.doc"] {
        let mut file = File::create(format!("{}/{}", path.display(), item)).unwrap();
        let mut s = String::new();
        for i in 0..12 {
            let _ = writeln!(file, "hello\t");
        }
    }

    if path.is_dir() {
        let info = path.read_dir().unwrap();
    }

    let file = File::create(path);
    println!("{}", path.display());
    let file = File::create("./names.txt").unwrap();
    Ok(())
}
