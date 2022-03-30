extern crate clap;
use clap::{Arg, App};
use std::env;

fn main() {
    let options = App::new("Planetary: GNU Global like tagging system")
        .arg(Arg::new("debug")
            .short('D')
            .long("debug")
            .help("debug dump input file"))
        .get_matches();
    
    if options.is_present("debug") {
        debug_dump_file(env::args().value_of())
    }
    println!("Hello, world!");
}

fn debug_dump_file(file_name: &str) {
    println!("debug_dump_file");
}