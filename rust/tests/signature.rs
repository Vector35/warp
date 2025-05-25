use uuid::uuid;
use warp::signature::basic_block::BasicBlockGUID;
use warp::signature::function::FunctionGUID;

#[test]
fn function_guid_creation() {
    // Convert a set of basic blocks into a function guid. This should always be the same.
    let bb_guid_0 = BasicBlockGUID::from(uuid!("e930c560-7b77-4f73-8b59-2ef6da75dcd4"));
    let bb_guid_1 = BasicBlockGUID::from(uuid!("3a4bf915-666f-44ad-8a7e-a2fea8f3a62a"));
    let bb_guid_2 = BasicBlockGUID::from(uuid!("0ffbfcd4-ac77-4b47-9696-006fa040167c"));
    let basic_block_guids = [bb_guid_0, bb_guid_1, bb_guid_2];

    let correct_func_guid = FunctionGUID::from(uuid!("1bef6187-74d9-5ebe-a0eb-4dbe6a97e578"));
    let function_guid = FunctionGUID::from_basic_blocks(&basic_block_guids);
    assert_eq!(function_guid, correct_func_guid);
}
