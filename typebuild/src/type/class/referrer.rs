use bon::Builder;

use crate::guid::TypeGUID;
use crate::Build;
use fbcg_rust::fb_type as fb;
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

impl Build for ReferrerClass {
    type FBType<'a> = fb::Referrer<'a>;

    fn create<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Self::FBType<'a>> {
        let guid = self
            .guid
            .as_ref()
            .map(|x| builder.create_string(&x.to_string()));
        let name = self.name.as_ref().map(|x| builder.create_string(x));
        fb::Referrer::create(builder, &fb::ReferrerArgs { guid, name })
    }

    // NOTE: Must resolve the reference to get the size.
    fn size(&self) -> Option<u64> {
        None
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
