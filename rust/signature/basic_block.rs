use crate::fb_sig as fb;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::uuid;
use uuid::Uuid;

pub const NAMESPACE_BASICBLOCK: Uuid = uuid!("0192a178-7a5f-7936-8653-3cbaa7d6afe7");

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BasicBlockGUID {
    guid: Uuid,
}

impl BasicBlockGUID {
    pub fn as_bytes(&self) -> &[u8] {
        self.guid.as_bytes()
    }
}

impl FromStr for BasicBlockGUID {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s).map(Into::into)
    }
}

impl From<&[u8]> for BasicBlockGUID {
    fn from(value: &[u8]) -> Self {
        Self {
            guid: Uuid::new_v5(&NAMESPACE_BASICBLOCK, value),
        }
    }
}

impl From<Uuid> for BasicBlockGUID {
    fn from(value: Uuid) -> Self {
        Self { guid: value }
    }
}

impl Display for BasicBlockGUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.guid, f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BasicBlock {
    pub guid: BasicBlockGUID,
}

impl BasicBlock {
    pub fn new(guid: BasicBlockGUID) -> Self {
        Self { guid }
    }

    // TODO: Error checking...
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        flatbuffers::root::<fb::BasicBlock>(buf)
            .ok()
            .map(Into::into)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut builder = FlatBufferBuilder::new();
        let fb_bb = self.create(&mut builder);
        builder.finish_minimal(fb_bb);
        builder.finished_data().to_vec()
    }

    pub(crate) fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::BasicBlock<'a>> {
        let guid = builder.create_string(&self.guid.to_string());
        fb::BasicBlock::create(builder, &fb::BasicBlockArgs { guid: Some(guid) })
    }
}

impl From<fb::BasicBlock<'_>> for BasicBlock {
    fn from(value: fb::BasicBlock<'_>) -> Self {
        let guid = value.guid().parse::<BasicBlockGUID>().unwrap();
        Self { guid }
    }
}
