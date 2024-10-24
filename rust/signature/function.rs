use crate::fb_sig as fb;
use crate::r#type::Type;
use crate::signature::basic_block::{BasicBlock, BasicBlockGUID};
use crate::signature::function::constraints::FunctionConstraints;
use crate::symbol::Symbol;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uuid::{uuid, Uuid};

pub mod constraints;

pub const NAMESPACE_FUNCTION: Uuid = uuid!("0192a179-61ac-7cef-88ed-012296e9492f");

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

impl Display for FunctionGUID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.guid, f)
    }
}

// TODO: bytemuck compat?
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Function {
    pub guid: FunctionGUID,
    pub symbol: Symbol,
    pub ty: Type,
    pub constraints: FunctionConstraints,
    pub entry: Option<BasicBlock>,
}

impl Function {
    // TODO: Just have these bare fns?
    // TODO: Error checking...
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        flatbuffers::root::<fb::Function>(buf).ok().map(Into::into)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut builder = FlatBufferBuilder::new();
        let fb_func = self.create(&mut builder);
        builder.finish_minimal(fb_func);
        builder.finished_data().to_vec()
    }

    // TODO: How do we want to hide these? We need them to be public for fb extensions.
    pub(crate) fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::Function<'a>> {
        let symbol = self.symbol.create(builder);
        let ty = self.ty.create(builder);
        let constraints = self.constraints.create(builder);
        let entry = self.entry.as_ref().map(|e| e.create(builder));
        let guid = builder.create_string(&self.guid.to_string());
        fb::Function::create(
            builder,
            &fb::FunctionArgs {
                guid: Some(guid),
                symbol: Some(symbol),
                type_: Some(ty),
                constraints: Some(constraints),
                entry,
            },
        )
    }
}

impl From<fb::Function<'_>> for Function {
    fn from(value: fb::Function<'_>) -> Self {
        let ty = value.type_();
        let guid = value.guid().parse::<FunctionGUID>().unwrap();
        Self {
            guid,
            symbol: value.symbol().unwrap().into(),
            ty: ty.unwrap().into(),
            constraints: value.constraints().unwrap().into(),
            entry: value.entry().map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::r#type::class::TypeClass;
    use crate::r#type::Type;
    use crate::signature::function::constraints::FunctionConstraints;
    use crate::signature::function::{Function, FunctionGUID};
    use crate::signature::Data;
    use crate::symbol::class::SymbolClass;
    use crate::symbol::{Symbol, SymbolModifiers};
    use uuid::{uuid, Uuid};

    const EMPTY_FN_UUID: Uuid = uuid!("db867a3e-416a-5d7f-aa6d-b8ae6be36da2");

    fn empty_fn_guid() -> FunctionGUID {
        FunctionGUID::from_basic_blocks(&[])
    }

    #[test]
    fn empty_function_guid() {
        assert_eq!(FunctionGUID::from(EMPTY_FN_UUID), empty_fn_guid());
    }

    fn empty_function() -> Function {
        Function {
            guid: empty_fn_guid(),
            symbol: Symbol::new("test", SymbolClass::Data, SymbolModifiers::empty()),
            ty: Type::builder()
                .name("aghhgh")
                .class(TypeClass::Void)
                .build(),
            constraints: FunctionConstraints::default(),
            entry: None,
        }
    }

    #[test]
    fn test_data_from_bytes() {
        let signature = empty_function();
        let data = Data {
            functions: vec![signature.clone()],
            types: vec![],
        };
        let buf = data.to_bytes();
        let signatures = Data::from_bytes(&buf);
        assert_eq!(Some(data), signatures)
    }

    #[test]
    fn test_function_from_bytes() {
        let signature = empty_function();
        let bytes = signature.to_bytes();
        let from_bytes_sig = Function::from_bytes(&bytes).unwrap();
        assert_eq!(signature, from_bytes_sig)
    }
}
