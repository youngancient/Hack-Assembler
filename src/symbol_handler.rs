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