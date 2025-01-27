use argparse::{ArgumentParser, Store};
use lalrpop_util::lalrpop_mod;
use std::env::args;
use std::fs::read_to_string;
use std::io::Result;

mod token;
mod ast;
mod sysy; // generate_in_source_tree

fn main() {
    let mut koopa_src_file_path = String::new();
    let mut koopa_output_file_path = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("SysY compiler");
        ap.refer(&mut koopa_src_file_path)
        .add_option(&["-koopa"], Store, "Koopa source file");
        ap.refer(&mut koopa_output_file_path).add_option(&["-o", "--output"], Store, "Output file path");
        ap.parse_args_or_exit();
    }

    println!("Hello, world!");
}
