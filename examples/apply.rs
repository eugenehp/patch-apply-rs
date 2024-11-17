//! Demonstrates how to apply a parsed diff to a file

use std::{fs, path::PathBuf};

use patch::{Line, Patch};

fn apply(patch: Patch, old: &str) -> String {
    let old_lines = old.lines().collect::<Vec<&str>>();
    let mut out: Vec<&str> = vec![];
    let mut old_line = 0;
    for hunk in patch.hunks {
        while old_line < hunk.old_range.start - 1 {
            out.push(old_lines[old_line as usize]);
            old_line += 1;
        }
        old_line += hunk.old_range.count;
        for line in hunk.lines {
            match line {
                Line::Add(s) | Line::Context(s) => out.push(s),
                Line::Remove(_) | Line::EndOfFile(_) => {}
            }
        }
    }
    out.join("\n")
}

fn match_file(input: &str, output: &str, diff: &str) {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/files");
    let input_text = fs::read_to_string(dir.join(input)).unwrap();
    let output_text = fs::read_to_string(dir.join(output)).unwrap();
    let patch_text = fs::read_to_string(dir.join(diff)).unwrap();
    let patch = Patch::from_single(&patch_text).unwrap();
    let new = apply(patch.clone(), input_text.as_str());
    assert_eq!(new, output_text);
    println!("{diff} applied to {input} matches {output}");
}

fn main() {
    match_file("input.txt", "output1.txt", "diff1.patch");
    match_file("input.txt", "output2.txt", "diff2.patch");
    match_file("input.txt", "output3.txt", "diff3.patch");
}
