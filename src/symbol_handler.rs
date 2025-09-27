// handle symbols in the .asm
// in charge of the symbol table
// does not need to understand anything about the machine or assembly language or what the symbols mean
// the only thing it needs to understand -> maintain association between a symbol and a memory address
// my thought: Looks like a hashmap works great here, for 0(1) lookups and additions

// functions / what it should do
// create an empty symbol table
// add a (symbol,address) pair to the table
// does the table contain a symbol?
// what memory address is associated with the given symbol

// usage
// create a new symbol table
// initialize: add all predefined symbols to the table
// while reading the input, add labels and new variables to the table
// whenever you see "@XXX" command, where XXX , consult the table to replace the XXX symbol with its address

use std::collections::HashMap;


pub fn get_predefined_symbols() -> Vec<(String,u16)>{
    vec![
    ("R0".to_string(),0),
    ("R1".to_string(),1),
    ("R2".to_string(),2),
    ("R3".to_string(),3),
    ("R4".to_string(),4),
    ("R5".to_string(),5),
    ("R6".to_string(),6),
    ("R7".to_string(),7),
    ("R8".to_string(),8),
    ("R9".to_string(),9),
    ("R10".to_string(),10),
    ("R11".to_string(),11),
    ("R12".to_string(),12),
    ("R13".to_string(),13),
    ("R14".to_string(),14),
    ("R15".to_string(),15),
    ("SCREEN".to_string(),16384),
    ("KBD".to_string(),24576),
    ("SP".to_string(),0),
    ("LCL".to_string(),1),
    ("ARG".to_string(),2),
    ("THIS".to_string(),3),
    ("THAT".to_string(),4),
]
}
// memory address go from 0 -> 32767 -> u16
pub struct SymbolTable{
    symbol_table : HashMap<String,u16>,
    next_free_address : u16
}

impl SymbolTable {
    pub fn new() -> Self{
        let mut symbol_table = Self { symbol_table: HashMap::new(), next_free_address: 16 };
        symbol_table.init();
        symbol_table
    }
    pub fn init(&mut self){
        for symbol in get_predefined_symbols(){
            self.symbol_table.insert(symbol.0, symbol.1);
        }
    }
    // checks if symbol is in the table, if in table, ignore
    // else, add to table
    pub fn add_symbol(&mut self,new_symbol: &str, memory_address : u16, is_variable : bool){
        if !self.contains_symbol(&new_symbol) {
            self.symbol_table.insert(new_symbol.to_string(), memory_address);
            if is_variable{
                // increment the free address if the symbol added is a variable
                // since labels dont take up space in memory, they are ignored
                self.increment_next_free_address();
            }
        }
    }

    pub fn contains_symbol(&self,symbol:&str) -> bool{
        self.symbol_table.contains_key(symbol)
    }

    // checks if symbol is in the table
    // returns mem address
    pub fn get_memory_address(&self,symbol:&str) -> u16{
        if let Some(value) = self.symbol_table.get(symbol){
            return *value;
        }else{
            panic!("symbol not registered in symbol table");
        }
    }

    pub fn get_next_free_address(&self) -> u16{
        self.next_free_address
    }
    
    // increments the next free address variable
    fn increment_next_free_address(&mut self){
        if self.next_free_address + 1 > u16::MAX{
            panic!("Spurious dragon Error: cannot exceed {} variables",u16::MAX);
        } else{
            self.next_free_address += 1;
        }
    }
}

#[cfg(test)]

mod tests{
    use super::*;

    fn init_symbol_table() -> SymbolTable{
        SymbolTable::new()
    }

    #[test]
    fn test_init(){
        let symbol_table = init_symbol_table();
        assert_eq!(symbol_table.contains_symbol("R0"),true);
        assert_eq!(symbol_table.contains_symbol("R1"),true);
        assert_eq!(symbol_table.contains_symbol("SCREEN"),true);
    }

    #[test]
    fn test_add_symbol(){
        let mut symbol_table = init_symbol_table();
        assert_eq!(symbol_table.contains_symbol("n"),false);
        symbol_table.add_symbol("n",16,true);
        assert_eq!(symbol_table.contains_symbol("n"),true);
        assert_eq!(symbol_table.get_memory_address("n"),16);
    }
}