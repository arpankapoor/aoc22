use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Directory(PathBuf);

impl Directory {
    // am I a parent directory of other?
    fn is_parent(&self, other: &Directory) -> bool {
        other.0.starts_with(self.0.as_path())
    }

    fn get_subdirectories<'a>(
        &'a self,
        dirs: &'a HashMap<Directory, Vec<File>>,
    ) -> impl Iterator<Item = &Directory> {
        dirs.keys()
            .filter_map(|dir| self.is_parent(dir).then_some(dir))
    }
}

#[derive(Debug)]
struct File {
    _name: String,
    size: usize,
}

const TOTAL_SPACE: usize = 70000000;
const REQD_SPACE: usize = 30000000;

pub fn solve(input: String) {
    let mut cwd = PathBuf::from("/");
    // map from directory to list of "direct" files it contains
    let mut dirs = HashMap::new();
    for line in input.lines() {
        if line.starts_with('$') {
            if line[2..].starts_with('c') {
                if line[5..].starts_with('.') {
                    cwd.pop();
                } else {
                    cwd.push(&line[5..]);
                };
            }
        } else {
            // ls output line
            let mut linesplit = line.split(' ');
            let dir_or_size = linesplit.next().unwrap();
            let name = linesplit.next().unwrap();
            match dir_or_size {
                "dir" => {
                    dirs.insert(Directory(cwd.join(name)), Vec::new());
                }
                size => dirs.entry(Directory(cwd.clone())).or_default().push(File {
                    _name: name.to_owned(),
                    size: size.parse().unwrap(),
                }),
            }
        }
    }

    let mut dir_sizes = HashMap::new();
    for dir in dirs.keys() {
        let size: usize = dir
            .get_subdirectories(&dirs)
            .flat_map(|subdir| &dirs[subdir])
            .map(|file| file.size)
            .sum();
        dir_sizes.insert(dir, size);
    }

    let ans1: usize = dir_sizes
        .values()
        .filter_map(|&size| (size <= 100000).then_some(size))
        .sum();

    let free_space = TOTAL_SPACE - dir_sizes[&Directory(PathBuf::from("/"))];
    let to_be_deleted_space = REQD_SPACE - free_space;
    let ans2 = dir_sizes
        .values()
        .filter(|&&sz| sz >= to_be_deleted_space)
        .min()
        .unwrap();
    println!("ans1 = {ans1}, ans2 = {ans2}");
}
