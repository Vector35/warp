use crate::r#type::Type;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use uuid::{uuid, Uuid};

pub const NAMESPACE_TYPEBIN: Uuid = uuid!("01929b90-72e6-73e6-9da1-2b6462e407a6");

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

impl Display for TypeGUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.guid, f)
    }
}

impl Distribution<TypeGUID> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TypeGUID {
        let rand_ty: Type = rng.gen();
        TypeGUID::from(rand_ty)
    }
}
