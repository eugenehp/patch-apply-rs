use crate::{Line, Patch};

/// Apply patch to the string from the input file source
pub fn apply(input: String, patch: Patch) -> String {
    let old_lines = input.lines().collect::<Vec<&str>>();
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

    let mut should_add_empty_line = false;

    while old_line < old_lines.len() as u64 {
        // TODO: fixme, this is not perfect
        should_add_empty_line = true;
        let line = old_lines[old_line as usize];
        out.push(line);

        old_line += 1;
    }

    if should_add_empty_line {
        out.push("");
    }

    out.join("\n")
}
