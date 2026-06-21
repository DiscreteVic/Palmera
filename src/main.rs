use std::env;
use std::path::Path;
use std::process;
use std::fs::File;

fn binary_kmp_algo<T: std::io::Read>(pattern: Vec<u8>, chunk: T) -> Vec<u32>
{
    let mut founds: Vec<u32> = vec![];
    let n_checks: usize = pattern.len();
    let mut pattern_checks: usize = n_checks;
    for (idx, val) in chunk.bytes().enumerate()
    {
        let chunk_byte: u8 = val.ok().expect("Binary reading error.");
        if chunk_byte == pattern[n_checks - pattern_checks] 
        {
            pattern_checks -= 1;
            if pattern_checks == 0
            {
                founds.push((idx - (n_checks - 1)) as u32);
                break
            }
        }
        else
        {
            pattern_checks = n_checks;
        } 
    }
    founds
}



fn main() {
    println!("--- Palmera CLI ---");
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1
    {
        println!("Missing arguments!");
        process::exit(1);
    }
    let in_file_path = Path::new(&args[1]);

    if !in_file_path.exists()
    {
        println!("File not found!");
        process::exit(1);
    }
    
    let f = File::open(in_file_path).unwrap();

    let pattern: Vec<u8> = vec![0xd0, 0x0d, 0xfe, 0xed];
    
    let result = binary_kmp_algo(pattern, f);
    for i in result
    {
        print!("{} ",i);
    }
}
