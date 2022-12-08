use std::{collections::BTreeMap, path::PathBuf};

static INPUT: &str = include_str!(concat!("./", module_path!(), "_input.txt"));
static INPUT_TEST: &str = include_str!(concat!("./", module_path!(), "_test.txt"));

#[derive(Debug, Clone)]
enum FsEntry {
    /// Name and file size.
    File(String, usize),

    /// Name and size of dir tree, initialized to 0.
    Dir(String, usize),
}

impl FsEntry {
    fn new_file(name: impl Into<String>, size: &str) -> Self {
        // panics if size is not a parsable integer
        Self::File(name.into(), size.parse().unwrap())
    }

    fn new_dir(name: impl Into<String>) -> Self {
        Self::Dir(name.into(), 0)
    }

    fn is_dir(&self) -> bool {
        matches!(self, Self::Dir(..))
    }

    fn size(&self) -> usize {
        match self {
            FsEntry::File(_, size) => *size,
            FsEntry::Dir(_, size) => *size,
        }
    }
}

fn main() {
    let test = std::env::args()
        .skip(1)
        .next()
        .map_or(false, |flag| flag == "--test");

    let input = if test { INPUT_TEST } else { INPUT };

    let mut path = PathBuf::new();

    let mut file_tree = BTreeMap::<String, FsEntry>::new();
    file_tree.insert("/".to_owned(), FsEntry::new_dir("/"));

    let mut state = State::ExpectCmd;

    for line in input.lines() {
        match state {
            State::ExpectCmd => {
                if let Some(new_state) = process_cmd(line, &mut path) {
                    state = new_state;
                }
            }

            State::ListingFiles => match list_file(line) {
                Ok(FsEntry::File(file_name, size)) => {
                    let mut path = path.clone();
                    path.push(file_name.clone());
                    file_tree.insert(path.display().to_string(), FsEntry::File(file_name, size));
                }

                Ok(FsEntry::Dir(dir_name, _)) => {
                    let mut path = path.clone();
                    path.push(dir_name.clone());
                    file_tree.insert(path.display().to_string(), FsEntry::new_dir(dir_name));
                }

                Err(new_state) => {
                    // not a file listing, go back to reading commands
                    state = new_state;
                    process_cmd(line, &mut path);
                }
            },
        }
    }

    populate_sizes(&mut file_tree);

    // problem A

    let solution_a = file_tree
        .iter()
        .filter_map(|(_, entry)| match entry {
            FsEntry::Dir(_, size) if *size <= 100_000 => Some(*size),
            _ => None,
        })
        .sum::<usize>();

    println!("solution A = {solution_a}");

    // problem B

    let max_disk_space = 70_000_000_usize;
    let space_needed = 30_000_000_usize;
    let space_used = file_tree["/"].size();

    println!("space used: {space_used}");

    let free_space = max_disk_space - space_used;
    println!("free space: {free_space}");

    let deletion_size_needed = space_needed - free_space;
    println!("need to free up: {deletion_size_needed}");

    let solution_b = file_tree
        .iter()
        .filter_map(|(_, entry)| match entry {
            FsEntry::Dir(_, size) if *size >= deletion_size_needed => Some(*size),
            _ => None,
        })
        .min()
        .unwrap();

    println!("solution B = {solution_b}");
}

#[derive(Debug, Clone, Copy)]
enum State {
    ExpectCmd,
    ListingFiles,
}

fn process_cmd(line: &str, path: &mut PathBuf) -> Option<State> {
    let Some(cmd) = line.strip_prefix("$ ") else {
        eprintln!("warning: expected command on line: {line}");
        return None;
    };

    if cmd == "ls" {
        return Some(State::ListingFiles);
    }

    if cmd == "cd .." {
        path.pop();
    } else if let Some(dir) = cmd.strip_prefix("cd ") {
        path.push(dir);
    }

    None
}

fn list_file(line: &str) -> Result<FsEntry, State> {
    if line.starts_with("$ ") {
        return Err(State::ExpectCmd);
    }

    if let Some(dir) = line.strip_prefix("dir ") {
        return Ok(FsEntry::new_dir(dir));
    }

    match line.split_once(' ') {
        Some((size, name)) => Ok(FsEntry::new_file(name, size)),
        None => panic!("invalid line format: {line}"),
    }
}

fn populate_sizes(file_tree: &mut BTreeMap<String, FsEntry>) {
    let mut entries = file_tree
        .iter()
        .map(|(path, entry)| (path.clone(), entry.clone()))
        .collect::<Vec<_>>();

    // sort by length, longest to shortest
    entries.sort_by(|(a, _), (b, _)| b.len().cmp(&a.len()));

    let dirs = entries
        .iter()
        .filter_map(|(path, entry)| entry.is_dir().then(|| path.clone()));

    for dir in dirs {
        let dir_size = entries
            .iter()
            .filter_map(|(path, entry)| (path.starts_with(&dir)).then_some(entry.size()))
            .sum::<usize>();

        match file_tree.get_mut(&dir).unwrap() {
            FsEntry::Dir(_, size) => *size = dir_size,
            FsEntry::File(..) => unreachable!(),
        }
    }
}
