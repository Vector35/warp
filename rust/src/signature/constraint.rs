use crate::cached_builder::CachedFlatBufferBuilder;
use crate::signature::function::FunctionGUID;
use crate::symbol::Symbol;
use crate::{fb_sig as fb, FlatBufferObject};
use flatbuffers::WIPOffset;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::{uuid, Uuid};

pub const NAMESPACE_CONSTRAINT: Uuid = uuid!("019701f3-e89c-7afa-9181-371a5e98a576");

/// For a [`Constraint`] which is unrelated (represented as `None`), we use [`i64::MAX`] when storing in the flatbuffer.
pub const UNRELATED_OFFSET: i64 = i64::MAX;

/// This type is marked `repr(transparent)` to the underlying `[u8; 16]` type, so it is safe to use in FFI.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ConstraintGUID {
    pub guid: Uuid,
}

impl ConstraintGUID {
    // TODO: From value
    // TODO: From string

    pub fn from_function_guid(guid: &FunctionGUID) -> Self {
        // TODO: Should we prefix the bytes with some tag so that function guid cant collide with something else?
        let guid = Uuid::new_v5(&NAMESPACE_CONSTRAINT, guid.as_bytes());
        ConstraintGUID { guid }
    }

    pub fn from_symbol(symbol: &Symbol) -> Self {
        // TODO: Should we prefix the bytes with some tag so that symbol cant collide with something else?
        let guid = Uuid::new_v5(&NAMESPACE_CONSTRAINT, symbol.name.as_bytes());
        ConstraintGUID { guid }
    }

    pub fn from_value(value: u64) -> Self {
        // TODO: Should we prefix the bytes with some tag so that value cant collide with something else?
        let guid = Uuid::new_v5(&NAMESPACE_CONSTRAINT, &value.to_le_bytes());
        ConstraintGUID { guid }
    }
}

impl FromStr for ConstraintGUID {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s).map(Into::into)
    }
}

impl TryFrom<&str> for ConstraintGUID {
    type Error = uuid::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl From<&[u8]> for ConstraintGUID {
    fn from(value: &[u8]) -> Self {
        Self {
            guid: Uuid::new_v5(&NAMESPACE_CONSTRAINT, value),
        }
    }
}

impl From<Uuid> for ConstraintGUID {
    fn from(value: Uuid) -> Self {
        Self { guid: value }
    }
}

impl From<fb::ConstraintGUID> for ConstraintGUID {
    fn from(value: fb::ConstraintGUID) -> Self {
        Self {
            guid: Uuid::from_bytes(value.0),
        }
    }
}

impl From<ConstraintGUID> for fb::ConstraintGUID {
    fn from(value: ConstraintGUID) -> Self {
        Self(value.guid.into_bytes())
    }
}

impl Display for ConstraintGUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.guid, f)
    }
}

// TODO: Add a wrapper container type for the hashset and provided helpers
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Constraint {
    pub guid: ConstraintGUID,
    pub offset: Option<i64>,
}

impl Constraint {
    pub fn from_function(guid: &FunctionGUID, offset: Option<i64>) -> Self {
        Self {
            guid: ConstraintGUID::from_function_guid(guid),
            offset,
        }
    }

    pub fn from_symbol(symbol: &Symbol, offset: Option<i64>) -> Self {
        Self {
            guid: ConstraintGUID::from_symbol(symbol),
            offset,
        }
    }
}

impl FlatBufferObject for Constraint {
    type FbType<'fbb> = fb::Constraint<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        fb::Constraint::create(
            builder,
            &fb::ConstraintArgs {
                guid: Some(&self.guid.into()),
                offset: self.offset.unwrap_or(UNRELATED_OFFSET),
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let constraint = Self {
            guid: ConstraintGUID::from(*value.guid()),
            offset: match value.offset() {
                UNRELATED_OFFSET => None,
                _ => Some(value.offset()),
            },
        };

        Some(constraint)
    }
}
