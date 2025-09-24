// read a file and get the different commands in it and breaks them into parts
// should not understand the context (meaning of commands etc.)
// reads the input -> break into parts
// how?
// implemented as a struct
// constructor accepts the filename as arg
// reads the text file
// get the next command in the file
// break a line of instruction into components based on the different types: A-command, C-command, Label

// there are 3 types of instructions: A instruction, C instruction, Symbols
// focus on A and C rn

use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
pub enum InstructionVariant {
    A(String),
    C {
        comp: String,
        dest: String,
        jmp: String,
    },
    Symbol(String),
}
#[derive(Debug)]
pub struct Instruction {
    pub variant: InstructionVariant,
    pub line_number: u32,
}


impl Instruction {
    pub fn rep(&self) -> String {
        match &self.variant {
            InstructionVariant::A(addr) => format!("@{}", addr),
            InstructionVariant::C { comp, dest, jmp } => format!("{}={};{}", dest, comp, jmp),
            InstructionVariant::Symbol(symbol) => format!("({})", symbol),
            _ => panic!("invalid"),
        }
    }
}

pub struct Parser {
    pub instructions: Vec<Instruction>,
    pub instructions_count: u32,
}

impl Parser {
    pub fn new(file_path: &str) -> io::Result<Parser> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut parser_state = Parser {
            instructions: Vec::new(),
            instructions_count: 0,
        };

        for line in reader.lines() {
            let line = line?;
            let trimmed_line = line.trim();
            if trimmed_line.is_empty() {
                continue;
            }
            let line_with_no_comment = trimmed_line.split("//").next().unwrap().trim();
            if line_with_no_comment.is_empty() {
                continue;
            }

            let mut cleaned_line: String = line_with_no_comment
                .chars()
                .filter(|char| !char.is_whitespace())
                .collect();
            let first_char = cleaned_line.chars().nth(0).unwrap();
            if first_char == '@' {
                // A instruction or variable symbol
                cleaned_line = cleaned_line.chars().filter(|char| *char != '@').collect();
                parser_state.instructions.push(Instruction {
                    variant: InstructionVariant::A(cleaned_line),
                    line_number: parser_state.instructions_count,
                });
                parser_state.instructions_count += 1;
            } else if first_char == '(' {
                // label symbol
                cleaned_line = cleaned_line
                    .chars()
                    .filter(|char| *char != '(' && *char != ')')
                    .collect();
                parser_state.instructions.push(Instruction {
                    variant: InstructionVariant::Symbol(cleaned_line),
                    line_number: parser_state.instructions_count,
                });
                parser_state.instructions_count += 1;
            } else {
                // C instruction
                // println!("{:?}",cleaned_line);
                // first split
                let splitted: Vec<&str> = cleaned_line.split("=").collect();
                let mut dest: String = String::new();
                let mut comp: String = String::new();
                let mut jmp: String = String::new();

                // splitted contains dest, comp and jmp
                let jmp_statement: &str;

                if splitted.len() == 2 {
                    // has a comp and JMP statement
                    dest = splitted[0].to_string();
                    jmp_statement = splitted[1];
                }
                // only has a JMP statement
                else if splitted.len() == 1 {
                    jmp_statement = splitted[0];
                } else {
                    panic!("invalid instruction count!")
                }
                // second split
                let splitted_jmp_statement: Vec<&str> = jmp_statement.split(";").collect();
                // println!("{:?}",splitted_jmp_statement);
                if splitted_jmp_statement.len() == 2 {
                    comp = splitted_jmp_statement[0].to_string();
                    jmp = splitted_jmp_statement[1].to_string();
                } else if splitted_jmp_statement.len() == 1 {
                    comp = splitted_jmp_statement[0].to_string();
                }

                // so i m thinking , depending on how I implement the mnemonic converter,
                // the case where dest = "" or jmp = "", i wanted to set them to "null" but
                // i dont think it's necessary, keeping this here in case.
                parser_state.instructions.push(Instruction {
                    variant: InstructionVariant::C { comp, dest, jmp },
                    line_number: parser_state.instructions_count,
                });
            }
            parser_state.instructions_count += 1;
        }
        Ok(parser_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test() {
        let parser = Parser::new("Max.asm");
        match parser {
            Ok(p) => println!("{:?}",p.instructions),
            Err(_) => println!("error!")
        }
    }
}
