use crate::fb_sig as fb;
use crate::r#type::ComputedType;
use crate::signature::function::Function;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub mod basic_block;
pub mod function;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Data {
    pub functions: Vec<Function>,
    pub types: Vec<ComputedType>,
}

impl Data {
    pub fn new(functions: Vec<Function>, types: Vec<ComputedType>) -> Self {
        Self { functions, types }
    }

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        let opts = flatbuffers::VerifierOptions {
            // Trust me bro.
            max_tables: 10_000_000,
            ..Default::default()
        };
        flatbuffers::root_with_opts::<fb::Data>(&opts, buf)
            .ok()
            .map(Into::into)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut builder = FlatBufferBuilder::new();
        let fb_data = self.create(&mut builder);
        builder.finish_minimal(fb_data);
        builder.finished_data().to_vec()
    }

    pub(crate) fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::Data<'a>> {
        let _functions: Vec<_> = self.functions.iter().map(|f| f.create(builder)).collect();
        let functions = builder.create_vector(&_functions);
        let _types: Vec<_> = self.types.iter().map(|f| f.create(builder)).collect();
        let types = builder.create_vector(&_types);
        fb::Data::create(
            builder,
            &fb::DataArgs {
                functions: Some(functions),
                types: Some(types),
            },
        )
    }
}

impl From<fb::Data<'_>> for Data {
    fn from(value: fb::Data<'_>) -> Self {
        Self {
            functions: value.functions().unwrap().iter().map(Into::into).collect(),
            // TODO: I think we can make this look better...
            types: value
                .types()
                .map(|types| types.iter().map(Into::into).collect())
                .unwrap_or_default(),
        }
    }
}
