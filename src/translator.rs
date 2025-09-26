// convert mnemonic to binary
// understands how to convert some part or all of .asm to binary
// dont need to worry about how the mnemonic was generated or how the code was parsed
// strictly converts a given mnemonic to binary

// functions for comp, dest , jmp and one that uses the three functions to return the binary string for C Instructions
// function for returning the binary string for A intruction

use crate::parser::Instruction;

// returns a 7-bit binary string repping the comp bit
// a c1 c2 c3 c4 c5 c6
pub fn comp(comp_instruction :&str) -> String{
    let normalized_string:String;
    let mut a_bit = String::with_capacity(1);
    if comp_instruction.contains("M"){
        a_bit.push('1');
        normalized_string = comp_instruction.chars().map(|char| {if char == 'M' { 'A'} else {char}}).collect();
    }else{
        a_bit.push('0');
        normalized_string = comp_instruction.to_string();
    }
    match normalized_string.as_str() {
        "0"   => format!("{}101010", a_bit),
        "1"   => format!("{}111111", a_bit),
        "-1"  => format!("{}111010", a_bit),
        "D"   => format!("{}001100", a_bit),
        "A"   => format!("{}110000", a_bit),
        "!D"  => format!("{}001101", a_bit),
        "!A"  => format!("{}110001", a_bit),
        "-D"  => format!("{}001111", a_bit),
        "-A"  => format!("{}110011", a_bit),
        "D+1" => format!("{}011111", a_bit),
        "A+1" => format!("{}110111", a_bit),
        "D-1" => format!("{}001110", a_bit),
        "A-1" => format!("{}110010", a_bit),
        "D+A" => format!("{}000010", a_bit),
        "D-A" => format!("{}010011", a_bit),
        "D&A" => format!("{}000000", a_bit),
        "D|A" => format!("{}010101", a_bit),
        _ => panic!("invalid comp instruction"),
    }
}

// returns a 3-bit binary string repping the dest bit
pub fn dest(dest_instruction: &str) -> String{
    match dest_instruction {
        "" => String::from("000"),
        "M" => String::from("001"),
        "D" => String::from("010"),
        "MD" => String::from("011"),
        "A" => String::from("100"),
        "AM" => String::from("101"),
        "AD" => String::from("110"),
        "AMD" => String::from("111"),
        _ => panic!("invalid jump instruction")
    }
}

// returns a 3-bit binary string repping the jump bit
pub fn jmp(jump_instruction:&str) -> String{
    match jump_instruction {
        "" => String::from("000"),
        "JGT" => String::from("001"),
        "JEQ" => String::from("010"),
        "JGE" => String::from("011"),
        "JLT" => String::from("100"),
        "JNE" => String::from("101"),
        "JLE" => String::from("110"),
        "JMP" => String::from("111"),
        _ => panic!("invalid jump instruction")
    }
}

pub fn translate(){
    
}

// write tests