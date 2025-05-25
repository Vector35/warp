use clap::Subcommand;
use std::path::Path;

pub mod find;
pub mod merge;

pub trait OperationHandler {
    fn run(&self, path: &Path);
}

#[derive(Clone, Debug, Subcommand)]
pub enum Operation {
    Find(find::FindOp),
    Merge(merge::MergeOp),
}

impl Operation {
    pub fn run(&self, path: &Path) {
        match self {
            Operation::Find(op) => op.run(path),
            Operation::Merge(op) => op.run(path),
        }
    }
}
