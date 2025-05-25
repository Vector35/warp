use crate::cached_builder::CachedFlatBufferBuilder;
use crate::r#type::Type;
use crate::{fb_type as fb, FlatBufferObject};
use bon::bon;
use flatbuffers::WIPOffset;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum PointerAddressing {
    #[default]
    Absolute = 0,
    RelativeBase,
    RelativeSelf,
}

impl TryFrom<fb::PointerAddressing> for PointerAddressing {
    // TODO: Actual error type
    type Error = ();

    fn try_from(value: fb::PointerAddressing) -> Result<Self, Self::Error> {
        match value {
            fb::PointerAddressing::Absolute => Ok(PointerAddressing::Absolute),
            fb::PointerAddressing::RelativeBase => Ok(PointerAddressing::RelativeBase),
            fb::PointerAddressing::RelativeSelf => Ok(PointerAddressing::RelativeSelf),
            _ => Err(()),
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PointerClass {
    pub width: Option<u16>,
    pub child_type: Box<Type>,
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
            child_type: Box::new(child_type),
            addressing: addressing.unwrap_or_default(),
        }
    }
}

impl PointerClass {
    pub fn size(&self) -> Option<u64> {
        self.width.map(|w| w as u64)
    }
}

impl FlatBufferObject for PointerClass {
    type FbType<'fbb> = fb::Pointer<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
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

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let class = Self {
            width: value.width().map(Into::into),
            child_type: Box::new(Type::from_object(&value.child()?)?),
            addressing: value.addressing().try_into().ok()?,
        };

        Some(class)
    }
}
