use std::path::Path;

pub type Solver = fn(&Path) -> String;

pub fn is_full_input(path: &Path) -> bool {
    path.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .contains("input.txt")
}
