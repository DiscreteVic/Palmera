use std::env;
use std::path::Path;
use std::process;
use std::fs;
use std::io::BufReader;

fn print_initial_prompt()
{
 println!(" _____      _                          ");
 println!("|  __ \\    | |                         ");
 println!("| |__) |_ _| |_ __ ___   ___ _ __ __ _ ");
 println!("|  ___/ _` | | '_ ` _ \\ / _ \\ '__/ _` |");
 println!("| |  | (_| | | | | | | |  __/ | | (_| |");
 println!("|_|   \\__,_|_|_| |_| |_|\\___|_|  \\__,_|");
 }
                                        
                                        



fn binary_kmp_algo(pattern: &Vec<u8>, chunk: &Vec<u8>) -> Vec<u32>
{
    let mut founds: Vec<u32> = vec![];
    let n_checks: usize = pattern.len();
    let mut pattern_checks: usize = n_checks;
    

    for (idx, val) in chunk.iter().enumerate()
    {
        let chunk_byte: u8 = *val;

        if chunk_byte == pattern[n_checks - pattern_checks] 
        {
            pattern_checks -= 1;
            if pattern_checks == 0
            {
                founds.push((idx - (n_checks - 1)) as u32);
                pattern_checks = n_checks;
            }
        }
        else
        {
            pattern_checks = n_checks;
        } 
    }
    founds
}


fn get_dt_header_field_size(offset: u32, chunk: &Vec<u8>) -> u32
{
    let mut size:u32 = 0;
    for i in 0..4
    {
        let byte: u8 = chunk[(offset + 4 + i) as usize];
        size = size | ((byte as u32) << (8*(3-i)));
    }
    size
}

fn main() {
    //println!("--- Palmera CLI ---");
    print_initial_prompt();
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1
    {
        println!("ERR: Missing arguments!");
        process::exit(1);
    }
    let in_file_path = Path::new(&args[1]);

    if !in_file_path.exists()
    {
        println!("ERR: File not found!");
        process::exit(1);
    }
    
    let f: Vec<u8> = fs::read(in_file_path).unwrap();

    let pattern: Vec<u8> = vec![0xd0, 0x0d, 0xfe, 0xed];
    
    let result = binary_kmp_algo(&pattern, &f);
    if result.len() == 0
    {
        println!("ERR: Device tree not found!");
        process::exit(1);
    }

    let offset = result[0];
    
    let dt_size = get_dt_header_field_size(offset, &f);

    println!("-- Device tree found!!");
    println!("-- Offset: {:#x}", offset);
    println!("-- Size: {} bytes" , dt_size);


}
