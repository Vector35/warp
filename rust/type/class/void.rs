use crate::fb_type as fb;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VoidClass;

impl VoidClass {
    pub(crate) fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::Void<'a>> {
        fb::Void::create(builder, &fb::VoidArgs {})
    }
}

#[cfg(test)]
mod tests {
    use crate::r#type::class::TypeClass;
    use crate::r#type::guid::TypeGUID;
    use crate::r#type::{Alignment, Type};
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
                class: Box::from(TypeClass::Void),
                confidence: u8::MAX,
                modifiers: vec![],
                alignment: Alignment::Access,
                ancestors: vec![],
            },
            built_void_type()
        )
    }
}
