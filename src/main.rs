use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::str;

fn read_u16(buf: &Vec<u8>, offset: usize) -> u16 {
    let a = buf[offset + 0] as u16;
    let b = buf[offset + 1] as u16;

    (a | (b << 8))
}

fn read_u32(buf: &Vec<u8>, offset: usize) -> u32 {
    let a = buf[offset + 0] as u32;
    let b = buf[offset + 1] as u32;
    let c = buf[offset + 2] as u32;
    let d = buf[offset + 3] as u32;

    (a | (b << 8) | (c << 16) | (d << 24))
}

fn print_usage() {
    println!("the_end_ex.exe is a command-line tool for unpacking gpak archives for the game \"The End Is Nigh\"");
    println!("created by Sergii 'iOrange' in 2019");
    println!("");
    println!("usage: the_end_ex path_to_gpak output_folder_path");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print_usage();
    } else {
        let gpak_name = &args[1];
        let out_folder_path = &args[2];

        let mut file = fs::File::open(gpak_name).expect("couldn't open input file!");

        let mut content: Vec<u8> = Vec::new();
        let _ = file
            .read_to_end(&mut content)
            .expect("couldn't read file content");

        let num_files = read_u32(&content, 0);
        let mut offset: usize = 4;

        let output_path = Path::new(&out_folder_path);

        let mut file_records = Vec::<(String, usize)>::new();

        for _i in 0..num_files {
            let name_len = read_u16(&content, offset) as usize;
            offset += 2;
            let file_name = str::from_utf8(&content[offset..(offset + name_len)]).unwrap();
            offset += name_len;
            let file_size = read_u32(&content, offset) as usize;
            offset += 4;

            file_records.push((String::from(file_name), file_size));
        }

        for record in file_records {
            let file_name = record.0.as_str();
            let file_size = record.1;

            let full_path = output_path.join(file_name);
            let file_folder = full_path.parent().unwrap();
            fs::create_dir_all(&file_folder).expect("couldn't make folders!");

            let mut writer =
                BufWriter::new(fs::File::create(full_path).expect("couldn't create output file!"));
            writer
                .write(&content[offset..(offset + file_size)])
                .expect("couldn't write to output file");

            println!("unpacking file {} of size {}", &file_name, file_size);

            offset += file_size;
        }
    }
}
