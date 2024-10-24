use crate::fb_type as fb;
use bon::Builder;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct IntegerClass {
    /// Width in bits
    pub width: Option<u16>,
    pub signed: bool,
}

impl IntegerClass {
    pub(crate) fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::Integer<'a>> {
        fb::Integer::create(
            builder,
            &fb::IntegerArgs {
                width: self.width.map(Into::into).as_ref(),
                signed: self.signed,
            },
        )
    }

    pub fn size(&self) -> Option<u64> {
        self.width.map(|w| w as u64)
    }
}

impl From<fb::Integer<'_>> for IntegerClass {
    fn from(value: fb::Integer<'_>) -> Self {
        Self {
            width: value.width().map(Into::into),
            signed: value.signed(),
        }
    }
}

impl Distribution<IntegerClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> IntegerClass {
        // 90% chance this type will have a width.
        let width = match rng.gen_bool(0.9) {
            true => Some(rng.gen_range(1..=256)),
            false => None,
        };
        IntegerClass {
            width,
            signed: rng.gen_bool(0.5),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::r#type::class::{IntegerClass, TypeClass};
    use crate::r#type::guid::TypeGUID;
    use crate::r#type::{Alignment, Type};
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
                class: Box::from(TypeClass::Integer(IntegerClass {
                    width: Some(64),
                    signed: true
                })),
                confidence: u8::MAX,
                modifiers: vec![],
                alignment: Alignment::Access,
                ancestors: vec![],
            },
            built_int_type()
        )
    }
}
