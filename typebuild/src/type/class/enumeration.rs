use bon::Builder;

use crate::r#type::Type;
use crate::Build;
use fbcg_rust::fb_type as fb;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use rand::distributions::{Alphanumeric, DistString, Distribution, Standard};
use rand::Rng;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder)]
#[builder(on(String, into))]
pub struct EnumerationMember {
    pub name: Option<String>,
    pub constant: u64,
}

impl From<fb::EnumerationMember<'_>> for EnumerationMember {
    fn from(value: fb::EnumerationMember<'_>) -> Self {
        Self {
            name: value.name().map(str::to_string),
            constant: value.constant(),
        }
    }
}

impl Build for EnumerationMember {
    type FBType<'a> = fb::EnumerationMember<'a>;

    fn create<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Self::FBType<'a>> {
        let name = self.name.as_ref().map(|n| builder.create_string(n));
        fb::EnumerationMember::create(
            builder,
            &fb::EnumerationMemberArgs {
                name,
                constant: self.constant,
            },
        )
    }

    // TODO: This is another reason why this trait should be split out
    /// [EnumerationMember]'s do not know their type.
    fn size(&self) -> Option<u64> {
        None
    }
}

impl Distribution<EnumerationMember> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnumerationMember {
        EnumerationMember {
            name: Some(Alphanumeric.sample_string(rng, 16)),
            constant: rng.gen(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct EnumerationClass {
    pub member_type: Type,
    pub members: Vec<EnumerationMember>,
}

impl EnumerationClass {
    pub fn new(member_type: Type, members: Vec<EnumerationMember>) -> Self {
        Self {
            member_type,
            members,
        }
    }
}

impl From<fb::Enumeration<'_>> for EnumerationClass {
    fn from(value: fb::Enumeration<'_>) -> Self {
        Self {
            member_type: value.member_type().into(),
            members: value.members().unwrap().iter().map(Into::into).collect(),
        }
    }
}

impl Build for EnumerationClass {
    type FBType<'a> = fb::Enumeration<'a>;

    fn create<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Self::FBType<'a>> {
        let enum_type = self.member_type.create(builder);
        // Resolve then create all member constants. Take the prior constant when `None`.
        let created_members: Vec<_> = self
            .members
            .iter()
            .map(|member| member.create(builder))
            .collect();
        let enum_members = builder.create_vector(&created_members);
        fb::Enumeration::create(
            builder,
            &fb::EnumerationArgs {
                member_type: Some(enum_type),
                members: Some(enum_members),
            },
        )
    }

    fn size(&self) -> Option<u64> {
        None
    }
}

impl Distribution<EnumerationClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EnumerationClass {
        let rand_member_len = rng.gen_range(0..20);
        EnumerationClass {
            member_type: rng.gen(),
            members: rng.sample_iter(Standard).take(rand_member_len).collect(),
        }
    }
}
