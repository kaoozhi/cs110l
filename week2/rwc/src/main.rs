use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let texts = read_file_lines(&filename).expect("File doesn't exist");
    let count_lines = texts.len();
    let count_words: usize = (&texts).into_iter().map(|x| count_words_simple(&x)).sum();
    let count_characters: usize = (&texts).into_iter().map(|x| count_characters(&x)).sum();
    println!(
        "The file contents {} words {} lines and {} characters",
        count_words, count_lines, count_characters
    );
}

fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    // Be sure to delete the #[allow(unused)] line above
    let file = File::open(filename)?;

    let mut lines: Vec<String> = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line_str = line?;
        lines.push(line_str);
    }
    Ok(lines)
}

fn count_words_simple(text: &str) -> usize {
    text.split_whitespace().count()
}

fn count_characters(text: &str) -> usize {
    text.chars().filter(|c| !c.is_whitespace()).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_words_count() {
        let text = "How old are you";
        let count: usize = count_words_simple(text);
        assert_eq!(count, 4);
    }

    #[test]
    fn test_files() {
        let lines_result = read_file_lines(&String::from("handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }
}
