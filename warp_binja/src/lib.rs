use binaryninja::architecture::Architecture;
use binaryninja::basicblock::BasicBlock as BNBasicBlock;
use binaryninja::binaryview::BinaryViewExt;
use binaryninja::function::{Function as BNFunction, NativeBlock};
use binaryninja::llil;
use binaryninja::llil::{ExprInfo, FunctionMutability, NonSSA, NonSSAVariant, VisitorAction};
use binaryninja::rc::Ref as BNRef;
use signaturebuild::prelude::*;

use crate::cache::{
    cached_adjacency_constraints, cached_call_site_constraints, cached_function_guid,
};
use crate::convert::{from_bn_symbol, from_bn_type};

pub mod cache;
pub mod convert;
mod matcher;
/// Only used when compiled for cdylib target.
mod plugin;

pub fn build_function<A: Architecture, M: FunctionMutability, V: NonSSAVariant>(
    func: &BNFunction,
    llil: &llil::Function<A, M, NonSSA<V>>,
) -> Option<Function> {
    let bn_fn_ty = func.function_type();
    Some(Function {
        guid: cached_function_guid(func, llil)?,
        symbol: from_bn_symbol(&func.symbol()),
        // TODO: Confidence should be derived from function type.
        ty: from_bn_type(&func.view(), bn_fn_ty, 255),
        constraints: FunctionConstraints {
            // NOTE: Adding adjacent only works if analysis is complete.
            adjacent: cached_adjacency_constraints(func),
            call_sites: cached_call_site_constraints(func),
            // TODO: Add caller sites (when adjacent and call sites are minimal)
            // NOTE: Adding caller sites only works if analysis is complete.
            caller_sites: Default::default(),
        },
        // TODO: We need more than one entry block.
        entry: entry_basic_block_guid(func, llil).map(BasicBlock::new),
    })
}

pub fn entry_basic_block_guid<A: Architecture, M: FunctionMutability, V: NonSSAVariant>(
    func: &BNFunction,
    llil: &llil::Function<A, M, NonSSA<V>>,
) -> Option<BasicBlockGUID> {
    // NOTE: This is not actually the entry point. This is the highest basic block.
    let first_basic_block = sorted_basic_blocks(func).into_iter().next()?;
    basic_block_guid(&first_basic_block, llil)
}

/// Basic blocks sorted from high to low.
pub fn sorted_basic_blocks(func: &BNFunction) -> Vec<BNRef<BNBasicBlock<NativeBlock>>> {
    let mut basic_blocks = func
        .basic_blocks()
        .iter()
        .map(|bb| bb.clone())
        .collect::<Vec<_>>();
    basic_blocks.sort_by_key(|f| f.raw_start());
    basic_blocks
}

pub fn function_guid<A: Architecture, M: FunctionMutability, V: NonSSAVariant>(
    func: &BNFunction,
    llil: &llil::Function<A, M, NonSSA<V>>,
) -> Option<FunctionGUID> {
    // TODO: Sort the basic blocks.
    let basic_blocks = sorted_basic_blocks(func);
    let basic_block_guids = basic_blocks
        .iter()
        .filter_map(|bb| basic_block_guid(bb, llil))
        .collect::<Vec<_>>();
    Some(FunctionGUID::from_basic_blocks(&basic_block_guids))
}

pub fn basic_block_guid<A: Architecture, M: FunctionMutability, V: NonSSAVariant>(
    basic_block: &BNBasicBlock<NativeBlock>,
    llil: &llil::Function<A, M, NonSSA<V>>,
) -> Option<BasicBlockGUID> {
    let func = basic_block.function();
    let view = func.view();
    let arch = func.arch();
    let max_instr_len = arch.max_instr_len();
    // TODO: Add all the hacks here to remove stuff like function prolog...
    // TODO mov edi, edi on windows x86
    // TODO: Ugh i really dislike the above and REALLY don't wanna do that.
    // TODO: The above invalidates our "all function bytes" approach.
    // TODO: Could we keep the bytes and just zero mask them? At least then we don't completely get rid of them.

    let basic_block_range = basic_block.raw_start()..basic_block.raw_end();
    let mut basic_block_bytes = Vec::with_capacity(basic_block_range.count());
    for instr_addr in basic_block.into_iter() {
        let mut instr_bytes = view.read_vec(instr_addr, max_instr_len);
        if let Some(instr_info) = arch.instruction_info(&instr_bytes, instr_addr) {
            let instr_len = instr_info.len();
            instr_bytes.truncate(instr_len);
            if let Some(instr_llil) = llil.instruction_at(instr_addr) {
                if instr_llil.visit_tree(&mut |_expr, expr_info| match expr_info {
                    ExprInfo::ConstPtr(_) | ExprInfo::ExternPtr(_) => VisitorAction::Halt,
                    _ => VisitorAction::Descend,
                }) == VisitorAction::Halt
                {
                    // Found a variant instruction, mask off entire instruction.
                    instr_bytes.fill(0);
                }
            }
            // Add the instructions bytes to the functions bytes
            basic_block_bytes.extend(instr_bytes);
        }
    }

    Some(BasicBlockGUID::from(basic_block_bytes.as_slice()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use binaryninja::binaryview::BinaryViewExt;
    use binaryninja::headless::Session;
    use std::sync::OnceLock;

    static INIT: OnceLock<Session> = OnceLock::new();

    fn get_session<'a>() -> &'a Session {
        // TODO: This is not shared between other test modules, should still be fine (mutex in core now).
        INIT.get_or_init(|| Session::new())
    }

    #[test]
    fn simple_signature() {
        let session = get_session();
        let bv = session.load(env!("TEST_BIN_LIBRARY_OBJ")).unwrap();
        assert_eq!(bv.functions().len(), 11);

        let mut valid_guids: Vec<&str> = vec![
            "405b94b7-6d73-5af5-9192-dd615a67afc5",
            "623a8338-34d6-5a6e-8c4e-36a1a071117e",
            "6cd81a21-6967-5c90-b73e-5a810f835a84",
            "89aaed99-1b17-5938-8be1-046825d89071",
            "905fa3b0-3571-58ed-b81f-7cf62bdcfe49",
            "9a3e480c-5ebd-5278-8e33-4a6e982167fb",
            "a25a06fb-fb60-542c-9b11-4c286dbc607b",
            "c8a64fb9-841b-5759-ab80-739b407ba7bc",
            "f631b282-0174-5bdd-846d-c0514f2539e1",
            "fa2d7ebf-d187-5592-bfc1-ee41614437b3",
            "fa2d7ebf-d187-5592-bfc1-ee41614437b3",
        ];

        // TODO: Do not use caching here. Test non cached.
        let mut guids = bv
            .functions()
            .into_iter()
            .filter_map(|func| cached_function_guid(&func, func.low_level_il().ok()?.as_ref()))
            .map(|guid| guid.to_string())
            .collect::<Vec<String>>();

        valid_guids.sort();
        guids.sort();

        assert_eq!(valid_guids, guids);
    }
}
