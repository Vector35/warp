use bon::Builder;

use crate::cached_builder::CachedFlatBufferBuilder;
use crate::r#type::Type;
use crate::{fb_type as fb, FlatBufferObject};
use flatbuffers::WIPOffset;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder)]
#[builder(on(String, into))]
pub struct EnumerationMember {
    pub name: Option<String>,
    pub constant: u64,
}

impl FlatBufferObject for EnumerationMember {
    type FbType<'fbb> = fb::EnumerationMember<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let name = self.name.as_ref().map(|n| builder.create_string(n));
        fb::EnumerationMember::create(
            builder,
            &fb::EnumerationMemberArgs {
                name,
                constant: self.constant,
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        Some(Self {
            name: value.name().map(str::to_string),
            constant: value.constant(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct EnumerationClass {
    pub member_type: Box<Type>,
    pub members: Vec<EnumerationMember>,
}

impl EnumerationClass {
    pub fn new(member_type: Type, members: Vec<EnumerationMember>) -> Self {
        Self {
            member_type: Box::new(member_type),
            members,
        }
    }
}

impl EnumerationClass {
    pub fn size(&self) -> Option<u64> {
        self.member_type.size()
    }
}

impl FlatBufferObject for EnumerationClass {
    type FbType<'fbb> = fb::Enumeration<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
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

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let class = Self {
            member_type: Box::new(Type::from_object(&value.member_type())?),
            members: value
                .members()?
                .iter()
                .flat_map(|member| EnumerationMember::from_object(&member))
                .collect(),
        };

        Some(class)
    }
}
