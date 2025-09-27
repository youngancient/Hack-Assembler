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

#[derive(Debug, Clone, PartialEq)]
pub enum InstructionVariant {
    A(String),
    C {
        comp: String,
        dest: String,
        jmp: String,
    },
    Symbol{ symbol : String, is_variable : bool},
}
#[derive(Debug, Clone)]
pub struct Instruction {
    pub variant: InstructionVariant,
    pub line_number: u16,
}

impl Instruction {
    pub fn rep(&self) -> String {
        match &self.variant {
            InstructionVariant::A(addr) => format!("@{}", addr),
            InstructionVariant::C { comp, dest, jmp } => {
                let mut build_string = String::new();
                if !dest.is_empty() {
                    build_string.push_str(&format!("{}=", dest));
                }
                build_string.push_str(&comp);
                if !jmp.is_empty() {
                    build_string.push_str(&format!(";{}", jmp));
                }
                build_string
            }
            InstructionVariant::Symbol { symbol, is_variable } => {
                if *is_variable {
                    format!("@{}", symbol)
                } else {
                    format!("({})", symbol)
                }
            }
        }
    }
}

pub struct Parser {
    pub instructions_count: u16,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            instructions_count: 0,
        }
    }
    pub fn parse(&mut self, instruction: String) -> Instruction {
        if instruction.len() < 1 {
            panic!("invalid!!!")
        }
        let mut cleaned_line: String = instruction
            .chars()
            .filter(|char| !char.is_whitespace())
            .collect();
        let first_char = cleaned_line.chars().nth(0).unwrap();
        let parsed: Instruction;

        if first_char == '@' {
            // A instruction or variable symbol
            cleaned_line = cleaned_line.chars().filter(|char| *char != '@').collect();
            // check if the instruction is strictly an A-instruction like @10 -> checks 10
            //  cos we could have @var -> checks var
            if let Ok(_) = cleaned_line.parse::<u32>() {
                // cleaned_line is a number and hence a valid A-instruction
                parsed = Instruction {
                    variant: InstructionVariant::A(cleaned_line),
                    line_number: self.instructions_count,
                };
            } else {
                parsed = Instruction {
                    variant: InstructionVariant::Symbol { symbol: cleaned_line, is_variable: true }, // it's a variable symbol
                    line_number: self.instructions_count, 
                };
            }
            self.instructions_count += 1;
        } else if first_char == '(' {
            // label symbol
            cleaned_line = cleaned_line
                .chars()
                .filter(|char| *char != '(' && *char != ')')
                .collect();
            parsed = Instruction {
                variant: InstructionVariant::Symbol { symbol: cleaned_line, is_variable: false }, // it's not a variable , it's a label
                line_number: self.instructions_count,
            };
        } else {
            // C instruction
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
            parsed = Instruction {
                variant: InstructionVariant::C { comp, dest, jmp },
                line_number: self.instructions_count,
            };
            self.instructions_count += 1;
        }
        // println!("{} line number is {}", parsed.rep(),parsed.line_number);
        return parsed;
    }
    pub fn clear(&mut self){
        self.instructions_count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parser_init() -> Parser {
        Parser::new()
    }

    fn sample_c_instruction() -> String {
        "D=D-M".to_string()
    }
    fn sample_a_instruction() -> String {
        "@10".to_string()
    }
    fn sample_symbol() -> String {
        "(ITSR0)".to_string()
    }
    fn sample_symbol_variable() -> String {
        "@var".to_string()
    }

    #[test]
    fn parser_init_test() {
        let parser = parser_init();
        assert_eq!(parser.instructions_count, 0);
    }

    #[test]
    #[should_panic]
    fn parser_test_invalid_instruction() {
        let mut parser = parser_init();
        parser.parse("".to_string());
    }

    #[test]
    fn parser_test_c_instruction() {
        let mut parser = parser_init();
        let parsed = parser.parse(sample_c_instruction());
        assert_eq!(parser.instructions_count, 1);
        assert_eq!(parsed.line_number, 0);
        assert_eq!(
            parsed.variant,
            InstructionVariant::C {
                comp: "D-M".to_string(),
                dest: "D".to_string(),
                jmp: String::new()
            }
        );
        assert_eq!(parsed.rep(), sample_c_instruction());
    }

    #[test]
    fn parser_test_a_instruction() {
        let mut parser = parser_init();
        let parsed = parser.parse(sample_a_instruction());
        assert_eq!(parser.instructions_count, 1);
        assert_eq!(parsed.line_number, 0);
        assert_eq!(parsed.variant, InstructionVariant::A("10".to_string()));
        assert_eq!(parsed.rep(), sample_a_instruction());
    }

    #[test]
    fn parser_test_variable_symbol() {
        let mut parser = parser_init();
        let parsed = parser.parse(sample_symbol_variable());
        assert_eq!(parser.instructions_count, 1);
        assert_eq!(parsed.line_number, 0);
        assert_eq!(
            parsed.variant,
            InstructionVariant::Symbol { symbol: "var".to_string(), is_variable: true }
        );
        assert_eq!(parsed.rep(), sample_symbol_variable());
    }

    #[test]
    fn parser_test_symbol() {
        let mut parser = parser_init();
        let parsed = parser.parse(sample_symbol());
        assert_eq!(parser.instructions_count, 0);
        assert_eq!(parsed.line_number, 1);
        assert_eq!(
            parsed.variant,
            InstructionVariant::Symbol { symbol: "ITSR0".to_string(), is_variable: false }
        );
        assert_eq!(parsed.rep(), sample_symbol());
    }
}
