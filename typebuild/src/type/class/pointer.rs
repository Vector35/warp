use crate::r#type::Type;
use crate::Build;
use bon::bon;
use fbcg_rust::fb_type as fb;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum PointerAddressing {
    #[default]
    Absolute = 0,
    RelativeBase,
    RelativeSelf,
}

impl From<fb::PointerAddressing> for PointerAddressing {
    fn from(value: fb::PointerAddressing) -> Self {
        match value {
            fb::PointerAddressing::Absolute => PointerAddressing::Absolute,
            fb::PointerAddressing::RelativeBase => PointerAddressing::RelativeBase,
            fb::PointerAddressing::RelativeSelf => PointerAddressing::RelativeSelf,
            _ => unreachable!(),
        }
    }
}

impl From<PointerAddressing> for fb::PointerAddressing {
    fn from(value: PointerAddressing) -> Self {
        match value {
            PointerAddressing::Absolute => Self::Absolute,
            PointerAddressing::RelativeBase => Self::RelativeBase,
            PointerAddressing::RelativeSelf => Self::RelativeSelf,
        }
    }
}

impl Distribution<PointerAddressing> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PointerAddressing {
        match rng.gen_range(0..3) {
            1 => PointerAddressing::RelativeBase,
            0 => PointerAddressing::RelativeSelf,
            _ => PointerAddressing::Absolute,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PointerClass {
    pub width: Option<u16>,
    pub child_type: Type,
    pub addressing: PointerAddressing,
    // TODO: Pointer modifiers etc...
}

#[bon]
impl PointerClass {
    #[builder]
    pub fn new(
        width: Option<u16>,
        child_type: Type,
        addressing: Option<PointerAddressing>,
    ) -> Self {
        Self {
            width,
            child_type,
            addressing: addressing.unwrap_or_default(),
        }
    }
}

impl From<fb::Pointer<'_>> for PointerClass {
    fn from(value: fb::Pointer<'_>) -> Self {
        Self {
            width: value.width().map(Into::into),
            child_type: value.child().map(Into::into).unwrap(),
            addressing: value.addressing().into(),
        }
    }
}

impl Build for PointerClass {
    type FBType<'a> = fb::Pointer<'a>;

    fn create<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Self::FBType<'a>> {
        let child_type = self.child_type.create(builder);
        fb::Pointer::create(
            builder,
            &fb::PointerArgs {
                width: self.width.map(Into::into).as_ref(),
                // TODO: Shift
                shift: None,
                child: Some(child_type),

                addressing: self.addressing.into(),
                // TODO: Offset
                offset: None,
            },
        )
    }

    fn size(&self) -> Option<u64> {
        self.width.map(|w| w as u64)
    }
}

impl Distribution<PointerClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PointerClass {
        // 90% chance this type will have a width.
        let width = match rng.gen_bool(0.9) {
            true => Some(rng.gen_range(1..=256)),
            false => None,
        };
        PointerClass {
            width,
            child_type: rng.gen(),
            addressing: rng.gen(),
        }
    }
}
