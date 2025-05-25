use crate::cached_builder::CachedFlatBufferBuilder;
use crate::{fb_type as fb, FlatBufferObject};
use flatbuffers::WIPOffset;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VoidClass;

impl FlatBufferObject for VoidClass {
    type FbType<'fbb> = fb::Void<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        fb::Void::create(builder, &fb::VoidArgs {})
    }

    fn from_object(_value: &Self::FbType<'_>) -> Option<Self> {
        Some(Self)
    }
}

#[cfg(test)]
mod tests {
    use crate::r#type::class::TypeClass;
    use crate::r#type::guid::TypeGUID;
    use crate::r#type::Type;
    use uuid::{uuid, Uuid};

    const VOID_TYPE_UUID: Uuid = uuid!("c37a394d-750b-5e89-b09e-539859c7a9bd");

    fn built_void_type() -> Type {
        Type::builder()
            .name("my_void".to_owned())
            .class(TypeClass::Void)
            .build()
    }

    #[test]
    fn void_guid_v1() {
        assert_eq!(TypeGUID::from(VOID_TYPE_UUID), built_void_type().into());
    }

    #[test]
    fn void_type() {
        assert_eq!(
            Type {
                name: Some("my_void".to_owned()),
                class: TypeClass::Void,
                confidence: u8::MAX,
                modifiers: Default::default(),
                metadata: vec![],
                alignment: Default::default(),
                ancestors: vec![],
            },
            built_void_type()
        )
    }
}
