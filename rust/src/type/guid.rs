use crate::fb_type as fb;
use crate::r#type::Type;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use uuid::{uuid, Uuid};

pub const NAMESPACE_TYPEBIN: Uuid = uuid!("01929b90-72e6-73e6-9da1-2b6462e407a6");

/// This type is marked `repr(transparent)` to the underlying `[u8; 16]` type, so it is safe to use in FFI.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct TypeGUID {
    guid: Uuid,
}

impl FromStr for TypeGUID {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s).map(Into::into)
    }
}

impl From<Type> for TypeGUID {
    fn from(value: Type) -> Self {
        Self::from(&value)
    }
}

impl From<&Type> for TypeGUID {
    fn from(value: &Type) -> Self {
        Self {
            guid: Uuid::new_v5(&NAMESPACE_TYPEBIN, &value.to_bytes()),
        }
    }
}

impl From<Uuid> for TypeGUID {
    fn from(value: Uuid) -> Self {
        Self { guid: value }
    }
}

impl From<fb::TypeGUID> for TypeGUID {
    fn from(value: fb::TypeGUID) -> Self {
        Self {
            guid: Uuid::from_bytes(value.0),
        }
    }
}

impl From<&fb::TypeGUID> for TypeGUID {
    fn from(value: &fb::TypeGUID) -> Self {
        Self::from(*value)
    }
}

impl From<TypeGUID> for fb::TypeGUID {
    fn from(value: TypeGUID) -> Self {
        Self(value.guid.into_bytes())
    }
}

impl Display for TypeGUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.guid, f)
    }
}
