use std::collections::VecDeque;
use std::io::{self, BufRead};

fn failure(message: &str) {
    eprintln!("Formatting failed! {}", message);
}

fn main() {
    // get all lines of table
    let all_lines: Vec<String> = io::stdin().lock().lines().map_while(Result::ok).collect();

    // fail on empty input
    if all_lines.is_empty() {
        for line in &all_lines {
            println!("{}", line)
        }
        failure("No lines to format.");
        return;
    }

    // split into each field
    // TODO: investigate if bug here
    let mut split_lines: Vec<Vec<&str>> = all_lines
        .iter()
        .map(|line| line.split('|').map(|s| s.trim()).collect::<Vec<&str>>())
        .map(|fields| {
            if fields.len() > 2 {
                fields[1..fields.len() - 1].to_owned()
            } else {
                fields
            }
        })
        .collect();

    // check to make sure each line has the same number of fields
    let mut num_fields_lines: VecDeque<usize> = split_lines.iter().map(|v| v.len()).collect();
    // empty input already failed out
    let num_fields_first = unsafe { num_fields_lines.pop_front().unwrap_unchecked() };
    for num_fields in num_fields_lines {
        if num_fields != num_fields_first {
            failure("Mismatched number of fields.");
            return;
        }
    }

    // remove divider line to get just the text fields
    split_lines.remove(1);

    // get longest length in each field
    let mut max_field_lengths = vec![0; num_fields_first];
    for line in &split_lines {
        for (i, field) in line.iter().enumerate() {
            if field.len() > max_field_lengths[i] {
                max_field_lengths[i] = field.len();
            }
        }
    }

    // print it all back out!
    for (i, line) in split_lines.iter().enumerate() {
        for (j, field) in line.iter().enumerate() {
            print!("| {1:<0$}", max_field_lengths[j] + 1, field);
        }
        println!("|");

        // separator row
        if i == 0 {
            for field_length in &max_field_lengths {
                print!("| {:-<1$} ", "", field_length);
            }
            println!("|");
        }
    }
}
