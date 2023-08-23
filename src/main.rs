use std::borrow::Borrow;
use std::process::{Command, Stdio};
use std::io::{Read, BufRead, BufReader, Write};
use std::fs::File;

#[derive(Debug)]
struct Entry {
    word: String,
    meaning: String,
    example: String
}
impl Borrow<str> for Entry {
    fn borrow(&self) -> &str {
        &self.word
    }
}
fn say(word: String) -> (bool, String) {
    let mut cmd = Command::new("eSpeak\\espeak-ng.exe")
        .arg(word)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to run command");

    // Capture stdout
    let mut stdout = String::new();
    cmd.stdout.take().unwrap().read_to_string(&mut stdout).unwrap();

    // Capture stderr
    let mut stderr = String::new();
    cmd.stderr.take().unwrap().read_to_string(&mut stderr).unwrap();

    // Wait for the command to finish and check the result
    let status = cmd.wait().expect("Failed to wait for command");

    if status.success() {
        return (true, stdout);
    } else {
        return (false, stderr);
    }
}

fn handle(code: (bool, String)) {
    if code.0 == false {
        panic!("Found error: > {} < ", code.1)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("wordlist.txt")?;
    let reader = BufReader::new(file);
    let mut entries = Vec::new();
    let mut current_entry = Entry {
        word: String::new(),
        meaning: String::new(),
        example: String::new(),
    };
    let mut line_count = 0;

    for line_result in reader.lines() {
        let line = line_result?;
        
        match line_count % 3 {
            0 => current_entry.word = line,
            1 => current_entry.meaning = line,
            2 => {
                current_entry.example = line;
                entries.push(current_entry);
                current_entry = Entry {
                    word: String::new(),
                    meaning: String::new(),
                    example: String::new(),
                };
            },
            _ => (),
        }
        
        line_count += 1;
    }

    println!("Welcome to the hopefully great spelling program!");
    println!("For each word: Enter your spelling, type 1 for it's meaning, type 2 for it's use or type 0 to exit.");
    println!("You have {} words to spell correctly,", entries.len());
    
    println!("Let's begin!\n");

    let mut words: Vec<Entry> = entries;
    //let mut approved: Vec<Entry> = vec![];
    while words.len() != 0 {
        let mut i = 0;
        let mut c = 1;
        while i < words.len() {
            let word = &words[i].word;
            let example = &words[i].example;
            println!("Total words left: {}. current word: {}.", words.len(), c);
            handle(say(word.to_string()));
            let stdin = std::io::stdin();
            let mut input = String::new();
            print!("> ");
            std::io::stdout().flush().expect("Failed to flush stdout");
            stdin.read_line(&mut input).expect("Failed to read line");
            match input.trim() {
                "0" => {panic!("Exit");}
                "1" => {println!("{}", words[i].meaning); continue;},
                "2" => {print!("Now playing it's use."); handle(say(example.to_string())); continue;}
                _ => {},
            };
            if input.trim() == words[i].word {
                println!("You got it correct: {}\n", input.trim());
                words.remove(i);
                c = c + 1;
                continue;
            } else {
                println!("You entered: {}, the actual spelling was: {}.\n", input.trim(), words[i].word);
                c = c + 1;
            }
            
            i = i + 1;
        }
    }
    let stdin = std::io::stdin();
    let mut input = String::new();
    println!("Well done! Enter anything to continue.");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let mut value: u8 = 1;
    stdin.read_line(&mut input).expect("Failed to read line");

    Ok(())
}