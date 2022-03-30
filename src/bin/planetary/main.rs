extern crate clap;
use clap::{Arg, Command};

fn main() {
    let options = Command::new("Planetary: GNU Global like tagging system")
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .takes_value(true)
                .value_name("FILE")
                .help("debug dump input file")
        )
        .get_matches();
    
    if let Some(v) = options.value_of("debug") {
        debug_dump_file(v);
    }
    println!("Hello, world!");
}

fn debug_dump_file(file_name: &str) {
    println!("debug_dump_file: {}", file_name);

    
}