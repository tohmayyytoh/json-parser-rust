use std::io::{self, BufRead};

// TODO (json-tokenize): implement per the lesson description.
#[derive(Debug)]
enum Token {
    PUNCT,
    STRING,
    NUMBER,
    TRUE,
    FALSE,
    EOF,
    NULL,
}

fn main() {
    let stdin = io::stdin();
    // This is for getting the input
    // this will contain all the words, escape sequences,

    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() {
            continue;
        }
        let mut character = l.char_indices().peekable();
        while let Some((i, c)) = character.next() {
            match c {
                '[' | ']' | '{' | '}' | ',' | ':' => println!("{:?} {c}", Token::PUNCT),
                '"' => {
                    let i_temp = i;
                    let mut string = Vec::new();
                    while i > i_temp {
                        string.push(c);
                        if c == '"' {
                            break;
                        }
                    }
                    println!("{:?} {string:?}", Token::STRING);
                }
                _ => continue,
            }
        }

        /*for c in l.chars() {
            if ['{', '}', '[', ']', ':', ','].contains(&c) {
                println!("{:?} {c}", Token::PUNCT);
            } else if l == true || l == false {
                /*let mut string = Vec::new();
                loop {
                    if c == '"' {
                        break;
                    }
                    string.push(c);
                }
                println!("{:#?}", string);
                println!("{:?} {c}", Token::PUNCT);*/
            }
        }*/
        println!("{:?}", Token::EOF);
    }
}
