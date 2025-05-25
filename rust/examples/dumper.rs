use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use warp::chunk::ChunkKind;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <output file>", args[0]);
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let file = warp::WarpFile::from_bytes(&buffer).expect("Failed to parse file");

    for chunk in file.chunks {
        match chunk.kind {
            ChunkKind::Signature(sc) => {
                println!("=== Signature chunk ===");
                for func in sc.functions() {
                    println!("{} | {}", func.symbol.name, func.guid);
                }
            }
            ChunkKind::Type(tc) => {
                println!("=== Type chunk ===");
                for ty in tc.types() {
                    println!(
                        "{} | {}",
                        ty.ty.name.unwrap_or("ANONYMOUS".to_string()),
                        ty.guid
                    );
                }
            }
        }
    }

    Ok(())
}
