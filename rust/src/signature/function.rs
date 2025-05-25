use crate::cached_builder::CachedFlatBufferBuilder;
use crate::r#type::Type;
use crate::signature::basic_block::BasicBlockGUID;
use crate::signature::comment::FunctionComment;
use crate::signature::constraint::Constraint;
use crate::signature::variable::FunctionVariable;
use crate::symbol::Symbol;
use crate::{fb_sig as fb, FlatBufferObject};
use flatbuffers::WIPOffset;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::{uuid, Uuid};

pub const NAMESPACE_FUNCTION: Uuid = uuid!("0192a179-61ac-7cef-88ed-012296e9492f");

/// This type is marked `repr(transparent)` to the underlying `[u8; 16]` type, so it is safe to use in FFI.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct FunctionGUID {
    pub guid: Uuid,
}

impl FunctionGUID {
    pub fn from_basic_blocks(basic_blocks: &[BasicBlockGUID]) -> Self {
        let basic_blocks_bytes = basic_blocks
            .iter()
            .fold(Vec::new(), |mut bytes: Vec<u8>, bb| {
                bytes.extend(bb.as_bytes());
                bytes
            });
        let guid = Uuid::new_v5(&NAMESPACE_FUNCTION, &basic_blocks_bytes);
        FunctionGUID { guid }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.guid.as_bytes()
    }
}

impl FromStr for FunctionGUID {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s).map(Into::into)
    }
}

impl TryFrom<&str> for FunctionGUID {
    type Error = uuid::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl From<&[u8]> for FunctionGUID {
    fn from(value: &[u8]) -> Self {
        Self {
            guid: Uuid::new_v5(&NAMESPACE_FUNCTION, value),
        }
    }
}

impl From<Uuid> for FunctionGUID {
    fn from(value: Uuid) -> Self {
        Self { guid: value }
    }
}

impl From<fb::FunctionGUID> for FunctionGUID {
    fn from(value: fb::FunctionGUID) -> Self {
        Self {
            guid: Uuid::from_bytes(value.0),
        }
    }
}

impl From<FunctionGUID> for fb::FunctionGUID {
    fn from(value: FunctionGUID) -> Self {
        Self(value.guid.into_bytes())
    }
}

impl Display for FunctionGUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.guid, f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
    pub guid: FunctionGUID,
    pub symbol: Symbol,
    pub ty: Option<Type>,
    pub constraints: HashSet<Constraint>,
    pub comments: Vec<FunctionComment>,
    pub variables: Vec<FunctionVariable>,
}

impl Function {
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        flatbuffers::root::<fb::Function>(buf)
            .ok()
            .and_then(|f| Function::from_object(&f))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut builder = CachedFlatBufferBuilder::new();
        let fb_type = self.create(&mut builder);
        builder.finish_minimal(fb_type);
        builder.finished_data().to_vec()
    }
}

impl FlatBufferObject for Function {
    type FbType<'fbb> = fb::Function<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let symbol = self.symbol.create(builder);
        let ty = self.ty.as_ref().map(|t| t.create(builder));
        let _constraints: Vec<_> = self
            .constraints
            .iter()
            .map(|constraint| constraint.create(builder))
            .collect();
        let constraints = if _constraints.is_empty() {
            None
        } else {
            Some(builder.create_vector(&_constraints))
        };
        let _comments: Vec<_> = self.comments.iter().map(|c| c.create(builder)).collect();
        let comments = if _comments.is_empty() {
            None
        } else {
            Some(builder.create_vector(&_comments))
        };
        let _variables: Vec<_> = self.variables.iter().map(|v| v.create(builder)).collect();
        let variables = if _variables.is_empty() {
            None
        } else {
            Some(builder.create_vector(&_variables))
        };

        fb::Function::create(
            builder,
            &fb::FunctionArgs {
                guid: Some(&self.guid.into()),
                symbol: Some(symbol),
                type_: ty,
                constraints,
                comments,
                variables,
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let ty = value.type_();
        let constraints: HashSet<Constraint> = value
            .constraints()
            .unwrap_or_default()
            .iter()
            .flat_map(|constraint| Constraint::from_object(&constraint))
            .collect();
        let comments: Vec<FunctionComment> = value
            .comments()
            .unwrap_or_default()
            .iter()
            .flat_map(|comment| FunctionComment::from_object(&comment))
            .collect();
        let variables: Vec<FunctionVariable> = value
            .variables()
            .unwrap_or_default()
            .iter()
            .flat_map(|variable| FunctionVariable::from_object(&variable))
            .collect();
        let func = Self {
            guid: FunctionGUID::from(*value.guid()),
            symbol: Symbol::from_object(&value.symbol()?)?,
            ty: match ty {
                Some(ty) => Type::from_object(&ty),
                None => None,
            },
            constraints,
            comments,
            variables,
        };

        Some(func)
    }
}
