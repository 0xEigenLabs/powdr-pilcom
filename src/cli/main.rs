//! Poring from https://github.com/powdr-labs/powdr.git.
use std::rc::Rc;

use powdr_pilcom::pilcom::export::export;
use powdr_pilcom::pil_analyzer::pil_analyzer;
use powdr_pilcom::number::GoldilocksField;
use starky::types::PIL;
use std::path::Path;

//use super::*;
use starky::types::load_json;
use std::fs;
use std::fs::File;
use std::io::Write;
use powdr_pilcom::pil_analyzer::{analyze_ast, analyze_file, analyze_string};

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version = "0.1.6", about, long_about = None)]
struct Cli {
    #[arg(short, long = "pil_file", default_value = "../../src/test-data/fibonacci.pil")]
    pil_file: String,
    #[arg(short, long = "out_file", default_value = "./test.fib.pil.json")]
    out_file: String,
    
}


pub fn compile_pil_from_str(pil_str: &str) -> PIL {
    let analyze = pil_analyzer::analyze_string::<GoldilocksField>(pil_str);
    export(&Rc::new(analyze))
}
pub fn compile_pil_from_file(pil_file: &str) -> PIL {
    let analyze = pil_analyzer::analyze_file::<GoldilocksField>(Path::new(pil_file));
    export(&Rc::new(analyze))
}


 
fn main() {
    env_logger::init();
    
    let args = Cli::parse();
    log::info!("The input pil file :{}", &args.pil_file);
    log::info!("The results will be saved to the file :{}", &args.out_file);

    // The compiling results: pil_json
    let results = compile_pil_from_file(&args.pil_file);

    /*
    let path = Path::new(&args.pil_file)
            .canonicalize()
            .unwrap();

    let pil_str = fs::read_to_string(path).unwrap();
    // The target and actual pil_json
    let results = compile_pil_from_str(&pil_str);
    */


    // save the compiling results
    let mut file = File::create(Path::new(&args.out_file)).unwrap();
    let input = serde_json::to_string_pretty(&results).unwrap();
    write!(file, "{}", input).unwrap();
    
}
