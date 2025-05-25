use crate::cached_builder::CachedFlatBufferBuilder;
use crate::{fb_type as fb, FlatBufferObject};
use bon::Builder;
use flatbuffers::WIPOffset;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct IntegerClass {
    /// Width in bits
    pub width: Option<u16>,
    pub signed: bool,
}

impl IntegerClass {
    pub fn size(&self) -> Option<u64> {
        self.width.map(|w| w as u64)
    }
}

impl FlatBufferObject for IntegerClass {
    type FbType<'fbb> = fb::Integer<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        fb::Integer::create(
            builder,
            &fb::IntegerArgs {
                width: self.width.map(Into::into).as_ref(),
                signed: self.signed,
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        Some(Self {
            width: value.width().map(Into::into),
            signed: value.signed(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::r#type::class::{IntegerClass, TypeClass};
    use crate::r#type::guid::TypeGUID;
    use crate::r#type::Type;
    use uuid::{uuid, Uuid};

    const INT_TYPE_UUID: Uuid = uuid!("ec8805ad-b101-5d8c-a033-c94610d036c1");

    fn built_int_type() -> Type {
        let int_class = IntegerClass::builder().width(64).signed(true).build();
        Type::builder()
            .name("my_int".to_owned())
            .class(int_class)
            .build()
    }

    #[test]
    fn int_guid_v1() {
        assert_eq!(TypeGUID::from(INT_TYPE_UUID), built_int_type().into());
    }

    #[test]
    fn int_type() {
        assert_eq!(
            Type {
                name: Some("my_int".to_owned()),
                class: TypeClass::Integer(IntegerClass {
                    width: Some(64),
                    signed: true
                }),
                confidence: u8::MAX,
                modifiers: Default::default(),
                metadata: vec![],
                alignment: Default::default(),
                ancestors: vec![],
            },
            built_int_type()
        )
    }
}
