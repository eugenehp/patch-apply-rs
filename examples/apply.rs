//! Demonstrates how to apply a parsed diff to a file

use std::{fs, path::PathBuf};

use patch_apply::{apply, Patch};

fn match_file(input: &str, output: &str, diff: &str) {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/files");
    let input_text = fs::read_to_string(dir.join(input)).unwrap();
    let output_text = fs::read_to_string(dir.join(output)).unwrap();
    let patch_text = fs::read_to_string(dir.join(diff)).unwrap();
    let patch = Patch::from_single(&patch_text).unwrap();
    let new = apply(input_text, patch.clone());
    assert_eq!(new, output_text);
    println!("{diff} applied to {input} matches {output}");

    // print!("{new}")
}

fn main() {
    match_file("input.txt", "output1.txt", "diff1.patch");
    match_file("input.txt", "output2.txt", "diff2.patch");
    match_file("input.txt", "output3.txt", "diff3.patch");
    match_file("input_cmake.txt", "output_cmake.txt", "cmake.patch");
}
