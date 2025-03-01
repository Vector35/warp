use uuid::{uuid, Uuid};
use warp::r#type::ComputedType;
use warp::signature::function::{Function, FunctionGUID, NAMESPACE_FUNCTION};
use warp::signature::Data;
use warp::symbol::class::SymbolClass;
use warp::symbol::Symbol;

fn create_test_function(name: String) -> Function {
    Function {
        symbol: Symbol {
            name: name.to_string(),
            modifiers: Default::default(),
            class: SymbolClass::Function,
        },
        guid: Uuid::new_v5(&NAMESPACE_FUNCTION, &name.as_bytes()).into(),
        constraints: Default::default(),
        ty: rand::random(),
    }
}

use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <output file>", args[0]);
        std::process::exit(1);
    }

    let output_file = &args[1];
    let mut file = File::create(output_file).expect("Unable to create file");

    let function_count = 50;
    let type_count = 50;

    let functions = (0..function_count)
        .map(|i| create_test_function(format!("func_{}", i)))
        .collect();

    let types = (0..type_count)
        .map(|_| ComputedType::new(rand::random()))
        .collect();

    let first_data = Data::new(functions, types);
    file.write_all(&first_data.to_bytes())
        .expect("Unable to write file");
}
