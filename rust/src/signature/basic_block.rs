use crate::cached_builder::CachedFlatBufferBuilder;
use crate::{fb_sig as fb, FlatBufferObject};
use flatbuffers::WIPOffset;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::uuid;
use uuid::Uuid;

pub const NAMESPACE_BASICBLOCK: Uuid = uuid!("0192a178-7a5f-7936-8653-3cbaa7d6afe7");

/// This type is marked `repr(transparent)` to the underlying `[u8; 16]` type, so it is safe to use in FFI.
#[repr(transparent)]
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

impl From<fb::BasicBlockGUID> for BasicBlockGUID {
    fn from(value: fb::BasicBlockGUID) -> Self {
        Self {
            guid: Uuid::from_bytes(value.0),
        }
    }
}

impl From<BasicBlockGUID> for fb::BasicBlockGUID {
    fn from(value: BasicBlockGUID) -> Self {
        Self(value.guid.into_bytes())
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

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        flatbuffers::root::<fb::BasicBlock>(buf)
            .ok()
            .and_then(|b| BasicBlock::from_object(&b))
    }
}

impl FlatBufferObject for BasicBlock {
    type FbType<'fbb> = fb::BasicBlock<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        fb::BasicBlock::create(
            builder,
            &fb::BasicBlockArgs {
                guid: Some(&self.guid.into()),
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let bb = Self {
            guid: BasicBlockGUID::from(*value.guid()),
        };

        Some(bb)
    }
}
