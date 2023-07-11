use std::{collections::VecDeque, io::Write};

const MEMORY_SIZE: usize = 256;

struct Memory {
    data: [u8; MEMORY_SIZE],
    ptr: usize,
}

impl Memory {
    fn new() -> Self {
        Self {
            data: [0u8; MEMORY_SIZE],
            ptr: 0usize,
        }
    }

    fn inc(&mut self) {
        self.data[self.ptr] = self.data[self.ptr].wrapping_add(1);
    }

    fn dec(&mut self) {
        self.data[self.ptr] = self.data[self.ptr].wrapping_sub(1);
    }

    fn inc_ptr(&mut self) {
        self.ptr = (self.ptr + 1) % MEMORY_SIZE;
    }

    fn dec_ptr(&mut self) {
        self.ptr = (self.ptr + MEMORY_SIZE - 1) % MEMORY_SIZE;
    }

    fn get(&self) -> u8 {
        self.data[self.ptr]
    }

    fn set(&mut self, value: u8) {
        self.data[self.ptr] = value;
    }

    fn is_zero(&self) -> bool {
        self.data[self.ptr] == 0
    }
}

#[derive(Clone, Copy)]
enum Token {
    Period,
    Exclamation,
    Question,
}

#[derive(Clone, Copy)]
struct Pair {
    first: Token,
    second: Token,
}

fn prefix_from(s: &str, pattern: &str, from: usize) -> bool {
    for (i, c) in pattern.chars().enumerate() {
        if s.chars().nth(from + i) != Some(c) {
            return false;
        }
    }
    true
}

struct Runtime {
    source_pairs: Vec<Pair>,
    input_stream: VecDeque<char>,
}

impl Runtime {
    fn parse(
        source: &str,
        period: &str,
        question: &str,
        exclamation: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let token_kinds = [
            (Token::Period, period),
            (Token::Exclamation, exclamation),
            (Token::Question, question),
        ];
        let mut pairs = Vec::new();
        let mut stack = None;
        for i in 0..source.len() {
            for (token, pattern) in token_kinds {
                if prefix_from(source, pattern, i) {
                    match stack {
                        None => {
                            stack = Some(token);
                        }
                        Some(prev_token) => {
                            stack = None;
                            pairs.push(Pair {
                                first: prev_token,
                                second: token,
                            });
                        }
                    }
                }
            }
        }
        if stack.is_some() {
            return Err("Unmatched token".into());
        }
        Ok(Self {
            source_pairs: pairs,
            input_stream: VecDeque::new(),
        })
    }

    fn get(&self, pc: usize) -> Option<Pair> {
        self.source_pairs.get(pc).cloned()
    }

    fn input(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        print!("input: ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        for c in input.chars() {
            self.input_stream.push_back(c);
        }
        Ok(())
    }

    fn input_char(&mut self) -> Result<char, Box<dyn std::error::Error>> {
        loop {
            if let Some(c) = self.input_stream.pop_front() {
                return Ok(c);
            }
            self.input()?;
        }
    }

    fn run(&mut self, memory: &mut Memory) -> Result<(), Box<dyn std::error::Error>> {
        let mut pc = 0usize;
        let mut loop_addresses = Vec::new();

        while let Some(Pair { first, second }) = self.get(pc) {
            match (first, second) {
                // increment value pointed by the pointer
                (Token::Period, Token::Period) => memory.inc(),
                // decrement value pointed by the pointer
                (Token::Exclamation, Token::Exclamation) => memory.dec(),
                // increment the pointer
                (Token::Period, Token::Question) => memory.inc_ptr(),
                // decrement the pointer
                (Token::Question, Token::Period) => memory.dec_ptr(),
                // input value
                (Token::Period, Token::Exclamation) => {
                    let c = self.input_char()?;
                    if c < 128 as char {
                        memory.set(c as u8);
                    } else {
                        return Err("input is not ascii".into());
                    }
                }
                // print value
                (Token::Exclamation, Token::Period) => {
                    let m = memory.get();
                    if m < 128 {
                        print!("{}", m as char);
                    } else {
                        print!("\x1b[1m\\{}\x1b[0m", m);
                    }
                }
                // if the value pointed by the pointer is zero, jump to the matching ?!
                (Token::Exclamation, Token::Question) => {
                    if memory.is_zero() {
                        'found_close: {
                            while let Some(pair) = self.get(pc) {
                                if let Pair {
                                    first: Token::Question,
                                    second: Token::Exclamation,
                                } = pair
                                {
                                    break 'found_close;
                                }
                                pc += 1;
                            }
                            return Err("a pair with the pattern ? ! is not found".into());
                        }
                    } else {
                        loop_addresses.push(pc);
                    }
                }
                // if the value pointed by the pointer is not zero, jump to the matching !?
                (Token::Question, Token::Exclamation) => {
                    if !memory.is_zero() {
                        let Some(pc_) = loop_addresses.last() else {
                            return Err("a pair with the pattern ! ? is not found".into());
                        };
                        pc = *pc_;
                    } else {
                        loop_addresses.pop();
                    }
                }
                // undefined pair
                (Token::Question, Token::Question) => {
                    print!("***偉業***");
                }
            }
            pc += 1;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read config.json
    let config = std::fs::read_to_string("config.json").unwrap();
    let config: serde_json::Value = serde_json::from_str(&config).unwrap();
    let config = config.as_object().unwrap();

    let period = config.get("Ook.").unwrap().as_str().unwrap();
    let exclamation = config.get("Ook!").unwrap().as_str().unwrap();
    let question = config.get("Ook?").unwrap().as_str().unwrap();

    let mut memory = Memory::new();

    println!("Misskey.Systems Ook! Interpreter!");
    println!(
        r#"Commands:
    <raw code>: run the code
    #exit: exit the interpreter
    #load <filename>: load the file
    #show: show the current memory
    #reset: reset the memory
    #doc: show the operation manual of Misskey.Systems Ook!
    #transpile <filename>: transpile the code of brainfuck to Misskey.Systems Ook!
    "#
    );

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.starts_with("#exit") {
            break;
        } else if input.starts_with("#show") {
            for i in 0..MEMORY_SIZE {
                if i == memory.ptr {
                    print!("\x1b[1m[");
                } else {
                    print!(" ");
                }
                print!("{:02x}", memory.data[i]);
                if i == memory.ptr {
                    print!("]\x1b[0m");
                } else {
                    print!(" ");
                }
                if i % 16 == 15 {
                    println!();
                }
            }
            println!();
        } else if input.starts_with("#reset") {
            memory = Memory::new();
        } else if input.starts_with("#doc") {
            println!("Misskey.Systems Ook! Operation Manual");
            println!(
                "{} {} ; increment value pointed by the pointer",
                period, period
            );
            println!(
                "{} {} ; decrement value pointed by the pointer",
                exclamation, exclamation
            );
            println!("{} {} ; increment the pointer", period, question);
            println!("{} {} ; decrement the pointer", question, period);
            println!("{} {} ; input value", period, exclamation);
            println!("{} {} ; print value", exclamation, period);
            println!(
                "{} {} ; if the value pointed by the pointer is zero, jump to the matching {} {}",
                exclamation, question, question, exclamation
            );
            println!("{} {} ; if the value pointed by the pointer is not zero, jump to the matching {} {}", question, exclamation, exclamation, question);
            println!(
                "{} {} ; take a picture of your anus even though it's not New Year's Day",
                question, question
            );
        } else if input.starts_with("#transpile") {
            let rel_path = input.split_at(10).1.trim();
            let current = std::env::current_dir()?;
            let current = current.to_str().unwrap();
            let source = std::fs::read_to_string(format!("{}\\{}", current, rel_path));
            if let Ok(source) = source {
                let mut source = source;
                source = source.replace('+', format!("{}{}", period, period).as_str());
                source = source.replace('-', format!("{}{}", exclamation, exclamation).as_str());
                source = source.replace('>', format!("{}{}", period, question).as_str());
                source = source.replace('<', format!("{}{}", question, period).as_str());
                source = source.replace('.', format!("{}{}", exclamation, period).as_str());
                source = source.replace(',', format!("{}{}", period, exclamation).as_str());
                source = source.replace('[', format!("{}{}", exclamation, question).as_str());
                source = source.replace(']', format!("{}{}", question, exclamation).as_str());
                let source_wo_ext = input.split_at(10).1.trim().split('.').next().unwrap();
                let dest = format!("{}.mook", source_wo_ext);
                std::fs::write(dest, source)?;
            } else {
                println!("File not found!");
            }
        } else {
            let source = if input.starts_with("#load") {
                let s = std::fs::read_to_string(input.split_at(5).1.trim());
                if let Ok(s) = s {
                    s
                } else {
                    println!("File not found!");
                    continue;
                }
            } else {
                input
            };

            let Ok(mut runtime) = Runtime::parse(&source, period, question, exclamation) else {
            println!("Parse error!");
            continue;
        };
            let res = runtime.run(&mut memory);
            println!();
            if let Err(e) = res {
                println!("Runtime error! {}", e);
            }
        }
    }
    println!("Bye!");
    Ok(())
}
