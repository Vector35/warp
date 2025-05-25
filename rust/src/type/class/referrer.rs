use bon::Builder;

use crate::cached_builder::CachedFlatBufferBuilder;
use crate::r#type::guid::TypeGUID;
use crate::{fb_type as fb, FlatBufferObject};
use flatbuffers::WIPOffset;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct ReferrerClass {
    pub guid: Option<TypeGUID>,
    pub name: Option<String>,
}

impl ReferrerClass {
    pub fn new(guid: Option<TypeGUID>, name: Option<String>) -> Self {
        Self { guid, name }
    }
}

impl FlatBufferObject for ReferrerClass {
    type FbType<'fbb> = fb::Referrer<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let guid = self.guid.map(fb::TypeGUID::from);
        let name = self.name.as_ref().map(|x| builder.create_string(x));
        fb::Referrer::create(
            builder,
            &fb::ReferrerArgs {
                guid: guid.as_ref(),
                name,
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        Some(Self {
            guid: value.guid().map(|&s| TypeGUID::from(s)),
            name: value.name().map(|x| x.to_owned()),
        })
    }
}
