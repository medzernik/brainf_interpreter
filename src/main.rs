use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::string;
use std::string::String;

use clap::Parser;
use regex::Regex;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let file_contents_char = load_file(&args.file);

    triage_commands(file_contents_char);

    let file_contents_string = load_file(&args.file);


    println!("Test output:\n");
    println!("{:?}", file_contents_string);
}

/// This function loads the file and populates an array of chars. Then it is returned and can be processed further.
/// usage:
/// ```
/// let _ = loadfile("examples\01-hello-world.bf");
/// ```
fn load_file(file_name: &String) -> Vec<char> {
    let mut file_contents_char: Vec<char> = Vec::new();

    let file = File::open(file_name).expect("error opening the file");
    let reader = BufReader::new(file);

    //old function
    for line in reader.lines() {
        for ch in line.expect("Unable to read line").chars() {
            file_contents_char.push(ch);
        }
    }

    return file_contents_char;
}

///Load the file into a string (one lined)
fn load_file_string(file_name: &String) -> String {
    let mut file_contents_char = String::new();

    let file = File::open(file_name).expect("error opening the file");
    let reader = BufReader::new(file);

    //old function
    for line in reader.lines() {
        for ch in line.expect("Unable to read line").chars() {
            file_contents_char.push(ch);
        }
    }

    return file_contents_char;
}


//TODO: Make the triager sequentially count increases in [] and then execute into appropriate buffer position.
fn triage_commands(contents_vector: Vec<char>) {
    let


    enum CommandKind {
        IncrementPointer,
        DecrementPointer,
        IncrementByte,
        DecrementByte,
        OutputByte,
        AcceptByte,
        SkipToAfterMatch,
        SkipToBeforeMatch,
        Comment,
    }

    for character in contents_vector {
        match character {
            '>' => {}
            '<' => {}
            '+' => {}
            '-' => {}
            '.' => {}
            ',' => {}
            '[' => {}
            ']' => {}
            _ => {},
        }
    }
}
