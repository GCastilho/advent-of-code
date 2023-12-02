use std::fs;

const FILENAME: &str = "example.txt";
// const FILENAME: &str = "input.txt";

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

impl File {
    fn new(name: &str, size: u32) -> File {
        File { name: String::from(name), size }
    }
}

#[derive(Debug)]
struct Directory<'a> {
    name: String,
    parent: Box<Option<&'a mut Directory<'a>>>,
    files: Vec<File>,
    directories: Vec<&'a mut Directory<'a>>,
}

impl<'a> Directory<'a> {
    fn new(name: &str, parent: Option<&'a mut Directory<'a>>) -> Directory<'a> {
        Directory {
            name: String::from(name),
            parent: Box::new(parent),
            directories: Vec::new(),
            files: Vec::new(),
        }
    }
}

struct Cursor<'a> {
    current: Option<&'a mut Directory<'a>>,
}

impl<'a> Cursor<'a> {
    fn new() -> Cursor<'static> {
        Cursor { current: None }
    }

    fn cd(&mut self, path: &str) {
        let dir = self.current.as_ref();
        let dir = dir.unwrap();
        if path == ".." {
            let dir = dir.parent.as_ref();
            self.current = *dir;
        } else {
            let dir = dir.directories
                .iter()
                .find(|dir| dir.name == path)
                .unwrap();
            self.current = Some(&mut dir);
        }
    }

    fn insert_file(&mut self, file: File) {
        let current = self.current.unwrap();
        current.files.push(file);
    }
}

fn main() {
    let input = fs::read_to_string(FILENAME).unwrap();
}
