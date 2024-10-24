use bon::Builder;

use crate::fb_type as fb;
use crate::r#type::guid::TypeGUID;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use rand::distributions::{Alphanumeric, DistString, Distribution, Standard};
use rand::Rng;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct ReferrerClass {
    pub guid: Option<TypeGUID>,
    pub name: Option<String>,
}

impl ReferrerClass {
    pub fn new(guid: Option<TypeGUID>, name: Option<String>) -> Self {
        Self { guid, name }
    }

    pub(crate) fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::Referrer<'a>> {
        let guid = self
            .guid
            .as_ref()
            .map(|x| builder.create_string(&x.to_string()));
        let name = self.name.as_ref().map(|x| builder.create_string(x));
        fb::Referrer::create(builder, &fb::ReferrerArgs { guid, name })
    }
}

// TODO: We really should make this TryFrom
impl From<fb::Referrer<'_>> for ReferrerClass {
    fn from(value: fb::Referrer<'_>) -> Self {
        Self {
            guid: value.guid().map(|s| s.parse().unwrap()),
            name: value.name().map(|x| x.to_owned()),
        }
    }
}

impl Distribution<ReferrerClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ReferrerClass {
        ReferrerClass {
            guid: rng.gen(),
            name: Some(Alphanumeric.sample_string(rng, 16)),
        }
    }
}
