use std::env;
use hack_assembler::assembler;

fn main() {
    const MAX_NO_OF_FILES: u16 = 10;
    let args: Vec<String> = env::args().collect();
    // args.len() - 1, because the first argument is a reference to the target program
    if args.len() - 1 > MAX_NO_OF_FILES.into() {
        panic!(
            "Too many files, Expected Max number of files is {}, found {}",
            MAX_NO_OF_FILES,
            args.len()
        );
    }
    for arg in args.iter().skip(1) {
        let file_name = assembler::extract_file_name(&arg);
        println!("Assembling file : {}.asm",file_name);
        let result = assembler::assemble(&arg);
        match result {
            Ok(_) => println!("Assembling successful ✅ Check output/{}.hack",file_name),
            Err(e) => println!("Failed to Assemble {}.asm : {}",file_name,e)
        }
    }
    
}
