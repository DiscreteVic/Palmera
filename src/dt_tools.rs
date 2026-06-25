use std::path::Path;
use std::process;
use std::fs;
use std::fs::File;
use std::io::Write;


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

fn get_dt(chunk: &Vec<u8>, offset: u32, size: u32) -> &[u8]
{
   &chunk[offset as usize..(offset+size) as usize]
}

pub struct DeviceTree  
{
    pub offset: u32,
    pub size: u32,
}

impl DeviceTree
{
    pub fn find(file: &Path) -> DeviceTree {
        let f: Vec<u8> = fs::read(file).unwrap();
        let pattern: Vec<u8> = vec![0xd0, 0x0d, 0xfe, 0xed];
    
        let result = binary_kmp_algo(&pattern, &f);
        if result.len() == 0
        {
            println!("ERR: Device tree not found!");
            process::exit(1);
        }

        let offset = result[0];
    
        let dt_size = get_dt_header_field_size(offset, &f);

        DeviceTree {offset: offset, size: dt_size}
    }

    pub fn extract(&self, ifile: &Path, ofile: String) {
        let f: Vec<u8> = fs::read(ifile).unwrap();
        let mut fo = File::create(&ofile).unwrap();
        let _ = fo.write_all(get_dt(&f, self.offset, self.size));  
    }
}
