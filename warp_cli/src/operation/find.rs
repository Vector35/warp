use crate::operation::OperationHandler;
use crate::utility::open_file;
use clap::Parser;
use std::path::Path;
use warp::chunk::{Chunk, ChunkKind};
use warp::r#type::chunk::TypeChunk;
use warp::r#type::ComputedType;
use warp::signature::chunk::SignatureChunk;
use warp::signature::function::Function;

// TODO: Filter by chunk type.
/// Find an item in the file given a set of filters like name.
#[derive(Clone, Debug, Parser)]
pub struct FindOp {
    /// Find any item with the given string.
    ///
    /// NOTE: This lookup is currently done
    #[arg(index = 1)]
    any: Option<String>,
}

impl FindOp {
    pub fn dump_function(&self, function: &Function) {
        println!("Function: {function:?}");
    }

    pub fn dump_type(&self, ty: &ComputedType) {
        println!("Type: {ty:?}");
    }

    pub fn find_in_signature(&self, func: &Function) -> bool {
        let str = format!("{func:?}");
        str.contains(self.any.as_deref().unwrap_or(""))
    }

    pub fn find_in_signature_chunk(&self, sc: &SignatureChunk) -> Vec<Function> {
        sc.functions()
            .filter(|f| self.find_in_signature(f))
            .collect()
    }

    pub fn find_in_computed_type(&self, computed_ty: &ComputedType) -> bool {
        let str = format!("{computed_ty:?}");
        str.contains(self.any.as_deref().unwrap_or(""))
    }

    pub fn find_in_type_chunk(&self, tc: &TypeChunk) -> Vec<ComputedType> {
        tc.types()
            .filter(|ct| self.find_in_computed_type(ct))
            .collect()
    }

    pub fn find_in_chunk(&self, chunk: &Chunk) {
        match &chunk.kind {
            ChunkKind::Signature(sc) => {
                for found_func in self.find_in_signature_chunk(sc) {
                    self.dump_function(&found_func)
                }
            }
            ChunkKind::Type(tc) => {
                for found_ty in self.find_in_type_chunk(tc) {
                    self.dump_type(&found_ty)
                }
            }
        }
    }
}

impl OperationHandler for FindOp {
    fn run(&self, path: &Path) {
        let file = open_file(path);
        for chunk in file.chunks {
            println!(
                "Searching in chunk ({}, 0x{:x} bytes)",
                chunk.header.chunk_type.variant_name().unwrap_or_default(),
                chunk.header.size
            );
            self.find_in_chunk(&chunk);
        }
    }
}
