use std::collections::HashSet;
use std::path::PathBuf;
pub fn refiner(paths: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut seen = HashSet::new();

    let mut refined_path = Vec::new();

    for path in paths {
        let normalized_path = path.clone();
        if seen.insert(normalized_path.clone()) {
            refined_path.push(normalized_path);
        }
    }
    refined_path
}
