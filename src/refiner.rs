use std::path::PathBuf;
use std::collections::HashSet;
// The refiner to filter out multiple repetive paths to avoid re run
pub fn refiner(paths: &Vec<PathBuf>) -> Vec<PathBuf> {
    // Hash sets only allow a single unique value
    // say the paths are : vec![A, B, C , D, A, B, C, D] --- remeber
    let mut seen = HashSet::new();

    // the output bin
    let mut refined_path = Vec::new();

    for path in paths {
        // clone each path so that the ownership don't changes
        let normalized_path = path.clone();

        // ---- continue;
        // now for each iteration it will copy the path ...
        // insert it into the HashSet
        // but if the hashset recieves the same value it will..
        // fail by producing false (else true)
        if seen.insert(normalized_path.clone()) {
            // this will only be triggerd if insertion works
            refined_path.push(normalized_path);
        }
    }

    // now return the pure path
    refined_path
}
