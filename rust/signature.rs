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

    pub fn merge(entries: &[Data]) -> Data {
        let mut merged_data = Data::default();
        merged_data
            .types
            .extend(entries.iter().flat_map(|e| &e.types).cloned());
        // Now sort and remove types with the same guid.
        merged_data
            .types
            .sort_unstable_by(|a, b| a.guid.cmp(&b.guid));
        merged_data.types.dedup_by_key(|ty| ty.guid);
        merged_data
            .functions
            .extend(entries.iter().flat_map(|e| &e.functions).cloned());
        // Now sort and remove functions with the same symbol and guid.
        merged_data
            .functions
            .sort_unstable_by(|a, b| a.symbol.name.cmp(&b.symbol.name));
        merged_data.functions.dedup_by(|a, b| {
            if a.guid == b.guid {
                // Keep `a`s constraints.
                b.constraints
                    .adjacent
                    .extend(a.constraints.adjacent.clone());
                b.constraints
                    .call_sites
                    .extend(a.constraints.call_sites.clone());
                b.constraints
                    .caller_sites
                    .extend(a.constraints.caller_sites.clone());
                true
            } else {
                false
            }
        });
        merged_data
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::r#type::guid::TypeGUID;
    use crate::r#type::ComputedType;
    use crate::signature::function::{Function, FunctionGUID};
    use crate::symbol::class::SymbolClass;
    use crate::symbol::Symbol;
    use uuid::{uuid, Uuid};

    const FUNC1_GUID: Uuid = uuid!("6b50fa09-c8c5-4e88-b317-5a96c01c52ee");
    const FUNC2_GUID: Uuid = uuid!("e0565a4e-d730-4073-916c-fa6cb8ad2407");
    const FUNC3_GUID: Uuid = uuid!("5a7eb124-b786-4aa8-af2f-ccffbb600d21");

    const TYPE1_GUID: Uuid = uuid!("7aee6520-0443-4a91-910e-da068571fa7a");
    const TYPE2_GUID: Uuid = uuid!("9e8a58f0-757d-4fa6-8c41-a4da023c5a32");
    const TYPE3_GUID: Uuid = uuid!("f81a46df-ad7b-4d7b-a4a7-23ed22ab01ec");

    // Used with `test_merge` test.
    fn create_sample_function<T: Into<FunctionGUID>>(name: &str, guid: T) -> Function {
        Function {
            symbol: Symbol {
                name: name.to_string(),
                modifiers: Default::default(),
                class: SymbolClass::Function,
            },
            guid: guid.into(),
            constraints: Default::default(),
            ty: rand::random(),
            entry: None,
        }
    }

    // Used with `test_merge` test.
    fn create_sample_computed_type<T: Into<TypeGUID>>(guid: T) -> ComputedType {
        let mut comp_ty = ComputedType::new(rand::random());
        comp_ty.guid = guid.into(); // Adjust the guid for testing.
        comp_ty
    }

    #[test]
    fn test_merge() {
        let first_data = Data::new(
            vec![
                create_sample_function("func1", FUNC1_GUID),
                create_sample_function("func2", FUNC2_GUID),
            ],
            vec![
                create_sample_computed_type(TYPE1_GUID),
                create_sample_computed_type(TYPE2_GUID),
            ],
        );

        let second_data = Data::new(
            vec![
                create_sample_function("func2", FUNC2_GUID),
                create_sample_function("func3", FUNC3_GUID),
            ],
            vec![
                create_sample_computed_type(TYPE1_GUID),
                create_sample_computed_type(TYPE3_GUID),
            ],
        );

        let merged_data = Data::merge(&[first_data, second_data]);
        assert_eq!(
            merged_data.functions.len(),
            3,
            "{:#?}",
            merged_data.functions
        );
        assert_eq!(merged_data.types.len(), 3, "{:#?}", merged_data.types);
    }
}
