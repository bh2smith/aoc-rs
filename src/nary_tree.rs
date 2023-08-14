use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug)]
struct Directory {
    name: String,
    parent: Option<DirectoryRef>,
    children: Vec<Directory>,
    files: Vec<File>,
}

type DirectoryRef = Rc<RefCell<Directory>>;

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

impl Directory {
    fn new(name: &str) -> DirectoryRef {
        let dir = Directory {
            name: name.to_string(),
            parent: None,
            children: Vec::new(),
            files: Vec::new(),
        };
        Rc::new(RefCell::new(dir))
    }

    fn add_file(&mut self, entry: File) {
        self.files.push(entry);
    }

    fn add_dir(&mut self, entry: DirectoryRef) {
        entry.borrow_mut().parent = Some(Rc::clone(&self));
        self.children.push(entry);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_file() {
        let root_directory = Directory::new("root");
        let file = File {
            name: "file.txt".to_string(),
            size: 1024,
        };
        root_directory.borrow_mut().add_file(file);
        println!("{:#?}", root_directory);
    }

    #[test]
    fn add_directory() {
        let mut root_directory = Directory::new("root");

        let sub_directory = Directory::new("sub");
        let file1 = File {
            name: "file1.txt".to_string(),
            size: 1024,
        };
        let file2 = File {
            name: "file2.txt".to_string(),
            size: 2048,
        };

        sub_directory.borrow_mut().add_file(file1);
        root_directory.borrow_mut().add_dir(sub_directory);
        root_directory.borrow_mut().add_file(file2);

        println!("{:#?}", root_directory);
    }
}
