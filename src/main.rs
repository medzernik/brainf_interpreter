use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::string::String;

use clap::Parser;

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
/*
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

 */

//TODO: Make the triager sequentially count increases in [] and then execute into appropriate buffer position.
fn triage_commands(contents_vector: Vec<char>) {
    let mut loop_counter = 0;
    let mut data_index = 0;
    let mut code_index = 0;

    let mut data_array: [u8; 30000] = [0; 30000];

    let length_vector_input = &contents_vector.len();
    println!("Length of vector: {}", length_vector_input);

    while code_index < contents_vector.len() {
        match contents_vector[code_index] {
            '>' => {
                data_index += 1;
            }
            '<' => {
                data_index -= 1;
            }
            '+' => {
                data_array[data_index] += 1;
            }
            '-' => {
                data_array[data_index] -= 1;
            }
            '.' => {
                println!("{}", data_array[data_index]);
            }
            ',' => {
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let input: u8 = input.trim().parse().expect("Please type a number!");
                data_array[data_index] = input;
            }
            '[' => {
                if data_array[data_index] == 0 {
                    code_index += 1;
                    while contents_vector[code_index] != ']' && loop_counter > 0 {
                        if contents_vector[code_index] == '[' {
                            loop_counter += 1;
                        } else if contents_vector[code_index] == ']' {
                            loop_counter -= 1;
                        }
                        code_index += 1;
                    }
                };
            }
            ']' => {
                if data_array[data_index] != 0 {
                    code_index -= 1;
                    while contents_vector[code_index] != '[' && loop_counter > 0 {
                        if contents_vector[code_index] == '[' {
                            loop_counter += 1;
                        } else if contents_vector[code_index] == ']' {
                            loop_counter -= 1;
                        }
                        code_index -= 1;
                    }
                    code_index += 1;
                };
            }
            _ => {}
        };

        code_index += 1;
    }
}
