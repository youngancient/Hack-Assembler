// takes in XXX.asm , outputs XXX.hack and if XXX.hack already exists, override the existing one

use std::{
    fs::{create_dir_all, File},
    io::{self, BufRead, Write}, path::Path,
};

use crate::{
    parser::{InstructionVariant, Parser},
    symbol_handler::SymbolTable,
    translator::{translate_a_instruction, translate_c_instruction},
};

fn clean_line(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }

    let no_comment = trimmed.split("//").next().unwrap().trim();
    if no_comment.is_empty() {
        return None;
    }

    Some(no_comment)
}


pub fn extract_file_name(file_name_or_path: &str ) -> String {
    file_name_or_path
        .split('/')
        .collect::<Vec<&str>>()
        .last()
        .and_then(|s| s.split('.').collect::<Vec<&str>>().get(0).copied())
        .unwrap_or("")
        .to_string()
}

pub fn assemble(file_name_or_path: &str) -> io::Result<(bool)> {
    let file = File::open(file_name_or_path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;
    let mut parser = Parser::new(); // initialize parser
    let mut symbol_table = SymbolTable::new(); // initialize symbol table

    // first pass -> maps labels only to the symbol table
    for line in &lines {
        if let Some(cleaned) = clean_line(line) {
            let parsed_instruction = parser.parse(cleaned.to_string());
            if let InstructionVariant::Symbol {
                symbol,
                is_variable,
            } = parsed_instruction.variant
            {
                if !is_variable {
                    // the symbol here is not a variable, it is a LABEL
                    // we match the LABEL to the memory address on the next unstruction
                    symbol_table.add_symbol(&symbol, parsed_instruction.line_number, is_variable);
                }
            }
        }
    }
    parser.clear();
    let mut output_string = String::new();
    // second pass ->
    for line in &lines {
        if let Some(cleaned) = clean_line(line) {
            let parsed_instruction = parser.parse(cleaned.to_string());

            match parsed_instruction.variant {
                InstructionVariant::Symbol {
                    symbol,
                    is_variable,
                } => {
                    if is_variable {
                        // the symbol here is a variable
                        // check if the variable is in memory
                        symbol_table.add_symbol(
                            &symbol,
                            symbol_table.get_next_free_address(),
                            is_variable,
                        );
                        let memory_address = symbol_table.get_memory_address(&symbol);
                        output_string.push_str(&translate_a_instruction(&memory_address));
                        output_string.push('\n');
                    }
                }
                InstructionVariant::A(num_string) => match num_string.parse::<u16>() {
                    Ok(num) => {
                        output_string.push_str(&translate_a_instruction(&num));
                        output_string.push('\n');
                    }
                    Err(_) => panic!("Invalid String to number conversion"),
                },
                InstructionVariant::C { comp, dest, jmp } => {
                    output_string.push_str(&translate_c_instruction(&dest, &comp, &jmp));
                    output_string.push('\n');
                }
            }
        }
    }
    // remove the last trailing new line
    if output_string.ends_with("\n"){
        output_string.pop();
    }
    // I/O
    // create directory if it doesnt exist
    let dir = Path::new("output");
    if !dir.exists(){
        create_dir_all(dir)?;
    }
    // create file_path
    let file_path = dir.join(format!("{}.hack",extract_file_name(file_name_or_path)));
    let mut file = File::create(file_path)?;
    file.write_all(output_string.as_bytes())?;

    Ok((true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test() {
        let assembler_1 = assemble("./input/add.asm");
        let assembler_2 = assemble("./input/Max.asm");
        let assembler_3 = assemble("./input/Rect.asm");
        // let assembler_3 = assemble("./input/Pong.asm");
    }

    #[test]
    fn test_extract_file_name(){
        assert_eq!("Rect", extract_file_name("Rect.asm"));
    }

    #[test]
    fn test_extract_file_path(){
        assert_eq!("Rect", extract_file_name("./input/Rect.asm"));
    }
}
