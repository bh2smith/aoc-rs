use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::{Rc, Weak};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct File {
    // name: String,
    fsize: i64,
}

#[derive(Debug, Clone)]
struct Entry(Rc<RefCell<TreeNode>>);

#[derive(Debug)]
struct TreeNode {
    value: String,
    dsize: i64,
    parent: Option<Weak<RefCell<TreeNode>>>,
    children: Vec<Entry>,
    files: Vec<File>,
}

impl Entry {
    fn new(value: &str) -> Self {
        Self(Rc::new(RefCell::new(TreeNode {
            value: value.to_string(),
            parent: None,
            children: vec![],
            files: vec![],
            dsize: 0,
        })))
    }

    fn child_names(&self) -> HashSet<String> {
        self.0
            .borrow()
            .children
            .iter()
            .map(|ch| ch.0.borrow().value.clone())
            .collect()
    }

    fn add_child(&self, child: Entry) {
        let parent = Rc::downgrade(&self.0);
        child.0.borrow_mut().parent = Some(parent);
        self.0.borrow_mut().children.push(child);
        self.0.borrow_mut().dsize = self.sum_files();
    }

    pub fn add_file(&self, file: File) {
        // println!("{:?}", file);
        self.0.borrow_mut().files.push(file.clone());
        self.0.borrow_mut().dsize = self.sum_files();
    }

    pub fn sum_files(&self) -> i64 {
        let borrowed_self = self.0.borrow();
        borrowed_self.files.iter().map(|f| f.fsize).sum::<i64>()
            + borrowed_self
                .children
                .iter()
                .map(|child| child.sum_files())
                .sum::<i64>()
    }

    fn find_child(&self, child_name: &str) -> Self {
        for child in &self.0.borrow().children {
            if child.0.borrow().value == child_name {
                return child.clone();
            }
        }
        panic!("No child found!");
    }

    fn get_parent(&self) -> Option<Entry> {
        self.0
            .borrow()
            .parent
            .as_ref()
            .map(|weak_parent| Self(weak_parent.upgrade().unwrap()))
    }

    pub fn cd(&self, cd_line: &str) -> Entry {
        // println!("Changing directories from {:?} to {}", self.0.borrow().value, cd_line);
        match cd_line {
            ".." => self.get_parent().expect("should exist!"),
            &_ => {
                if !self.child_names().contains(cd_line) {
                    let new_dir = Entry::new(cd_line);
                    self.add_child(new_dir.clone());
                    new_dir
                } else {
                    self.find_child(cd_line)
                }
            }
        }
    }

    fn traverse_dirs(self) -> Vec<Entry> {
        let mut dirs = vec![self.clone()];
        for child in self.0.borrow().children.iter() {
            dirs.append(&mut child.clone().traverse_dirs())
        }
        dirs
    }

    fn update_dsizes(&self) {
        // Updates directory sizes of all
        self.0.borrow_mut().dsize = self.sum_files();
        for child in self.0.borrow_mut().children.iter() {
            child.update_dsizes()
        }
    }
}

pub fn puzzle1(input: &str) -> i64 {
    let tree = parse_input(input);
    // when adding a file to a subdirectory, it does not update the parent dir sizes!
    tree.update_dsizes();
    // println!("{:?}", tree);
    tree.traverse_dirs()
        .iter()
        .map(|entry| {
            let size = entry.0.borrow().dsize;
            if entry.0.borrow().dsize < 100000 {
                return size;
            }
            0
        })
        .sum()
}

pub fn puzzle2(input: &str) -> i64 {
    let tree = parse_input(input);
    // when adding a file to a subdirectory, it does not update the parent dir sizes!
    tree.update_dsizes();
    let fs_size = tree.0.borrow().dsize;
    let target = fs_size - 40_000_000;
    tree.traverse_dirs()
        .iter()
        .map(|entry| {
            let size = entry.0.borrow().dsize;
            if size > target {
                return size;
            }
            i64::MAX
        })
        .min()
        .unwrap()
}

fn parse_input(input: &str) -> Entry {
    let mut command_strings: Vec<Vec<&str>> = input
        .trim()
        .split('\n')
        .map(|c_str| c_str.trim().split(' ').collect())
        .collect();
    command_strings.reverse();
    let root = Entry::new("/");
    let mut pwd = root.clone();
    command_strings.pop(); // First line is always cd /

    while let Some(line) = command_strings.pop() {
        if line[0] == "$" {
            // println!("Parsing Line: {:?}", line);
            if line[1] == "cd" {
                pwd = pwd.cd(line[2]);
            } else {
                // ls
                while let Some(line) = command_strings.pop() {
                    if line[0] == "$" {
                        if line[1] == "cd" {
                            pwd = pwd.cd(line[2]);
                        }
                        continue;
                    }
                    match line[0] {
                        "dir" => {
                            let new_dir = Entry::new(line[1]);
                            pwd.add_child(new_dir);
                        }
                        &_ => {
                            pwd.add_file(File {
                                // name: line[1].to_string(),
                                fsize: i64::from_str(line[0]).unwrap(),
                            })
                        }
                    }
                }
            }
        }
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn input_parsing() {
        parse_input(SAMPLE_INPUT);
    }

    #[test]
    fn node_add_child() {
        let node = Entry::new("A");
        let child = Entry::new("B");
        node.add_child(child.clone());
        // let parent = child.get_parent();
    }

    #[test]
    fn node_sum_files() {
        let node = Entry::new("A");
        node.add_file(File { fsize: 1 });
        let child = Entry::new("B");
        let child2 = Entry::new("C");
        let child3 = Entry::new("D");
        child.add_file(File { fsize: 2 });
        child.add_file(File { fsize: 3 });
        child2.add_file(File { fsize: 4 });
        child2.add_file(File { fsize: 5 });
        child3.add_file(File { fsize: 6 });
        node.add_child(child.clone());
        node.add_child(child2);
        child.add_child(child3);
        println!("{:?}", node);
    }

    #[test]
    fn puzzle1() {
        assert_eq!(super::puzzle1(SAMPLE_INPUT), 95437);
    }

    #[test]
    fn puzzle2() {
        assert_eq!(super::puzzle2(SAMPLE_INPUT), 24933642);
    }
}
