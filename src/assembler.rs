// takes in XXX.asm , outputs XXX.hack and if XXX.hack already exists, override the existing one

use std::{
    fs::File,
    io::{self, BufRead},
};

use crate::parser::Parser;

pub fn assemble(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut parser = Parser::new();

    // first pass
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
        let parsed_instruction = parser.parse(line_with_no_comment.to_string());
        println!("{}",parsed_instruction.rep());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test() {
        let assembler = assemble("Max.asm");
        
    }
}
