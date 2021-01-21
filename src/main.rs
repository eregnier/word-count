use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Entry<'a> {
    word: &'a str, // word: String,
    count: u32,
}

static SEPARATORS: &'static [char] = &[
    ' ', ',', '.', '!', '?', '\'', '"', '\n', '(', ')', '#', '{', '}', '[', ']', '-', ';', ':',
];

fn main() {
    if let Err(err) = try_main() {
        if err.kind() == std::io::ErrorKind::BrokenPipe {
            return;
        }
        // Ignore any error that may occur while writing to stderr.
        let _ = writeln!(std::io::stderr(), "{}", err);
    }
}

fn try_main() -> Result<(), std::io::Error> {
    let mut words: HashMap<String, u32> = HashMap::new();
    let stdin = io::stdin();
    for result in stdin.lock().lines() {
        let line = result?;
        line_processor(line, &mut words)
    }
    output(&mut words)?;
    Ok(())
}

fn line_processor(line: String, words: &mut HashMap<String, u32>) {
    let mut l = line.as_str();
    loop {
        if let Some(pos) = l.find(|c: char| SEPARATORS.contains(&c)) {
            let (head, tail) = l.split_at(pos);
            add_word(head.to_owned(), words);
            l = &tail[1..];
        } else {
            break;
        }
    }
}

fn add_word(word: String, words: &mut HashMap<String, u32>) {
    if word.len() > 0 {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }
}

fn output(words: &mut HashMap<String, u32>) -> Result<(), std::io::Error> {
    let mut stack = Vec::<Entry>::new();

    for (k, v) in words {
        stack.push(Entry {
            word: k.as_str(), // word: k.to_string(),
            count: *v,
        });
    }

    stack.sort_by(|a, b| a.count.cmp(&b.count));

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    while let Some(entry) = stack.pop() {
        writeln!(stdout, "{}\t{}", entry.count, entry.word)?;
    }
    Ok(())
}
