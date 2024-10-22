use crate::Build;
use fbcg_rust::fb_type as fb;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VoidClass;

impl Build for VoidClass {
    type FBType<'a> = fb::Void<'a>;

    fn create<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Self::FBType<'a>> {
        fb::Void::create(builder, &fb::VoidArgs {})
    }

    fn size(&self) -> Option<u64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
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
