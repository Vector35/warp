use bon::{bon, Builder};

use crate::fb_type as fb;
use crate::r#type::Type;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use rand::distributions::{Alphanumeric, DistString, Distribution, Standard};
use rand::Rng;

// We re-export bit flags as there is no need to wrap them.
pub use fb::StructureMemberModifiers;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructureMember {
    pub name: Option<String>,
    pub offset: u64,
    pub ty: Type,
    pub modifiers: StructureMemberModifiers,
}

#[bon]
impl StructureMember {
    #[builder]
    pub fn new<S: Into<String>>(
        name: Option<S>,
        offset: u64,
        ty: Type,
        modifiers: Option<StructureMemberModifiers>,
    ) -> Self {
        Self {
            name: name.map(Into::into),
            offset,
            ty,
            modifiers: modifiers.unwrap_or_default(),
        }
    }
}

impl StructureMember {
    fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::StructureMember<'a>> {
        let name = self.name.as_ref().map(|n| builder.create_string(n));
        let member_type = self.ty.create(builder);
        fb::StructureMember::create(
            builder,
            &fb::StructureMemberArgs {
                name,
                offset: Some(&self.offset.into()),
                type_: Some(member_type),
                modifiers: self.modifiers,
            },
        )
    }

    fn size(&self) -> Option<u64> {
        self.ty.size()
    }
}

impl From<fb::StructureMember<'_>> for StructureMember {
    fn from(value: fb::StructureMember<'_>) -> Self {
        Self {
            name: value.name().map(str::to_string),
            offset: value.offset().into(),
            ty: value.type_().into(),
            modifiers: value.modifiers(),
        }
    }
}

impl Distribution<StructureMember> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> StructureMember {
        let mut modifiers = StructureMemberModifiers::empty();
        // 50% chance structure member is internal.
        modifiers.set(StructureMemberModifiers::Internal, rng.gen_bool(0.5));
        StructureMember {
            name: Some(Alphanumeric.sample_string(rng, 16)),
            offset: rng.gen(),
            // TODO: This is causing a recursion issue...
            ty: rng.gen(),
            modifiers,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct StructureClass {
    pub members: Vec<StructureMember>,
}

impl From<fb::Structure<'_>> for StructureClass {
    fn from(value: fb::Structure<'_>) -> Self {
        Self {
            members: value.members().unwrap().iter().map(Into::into).collect(),
        }
    }
}

impl StructureClass {
    pub fn new(members: Vec<StructureMember>) -> Self {
        Self { members }
    }
}

impl StructureClass {
    pub(crate) fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::Structure<'a>> {
        let created_members: Vec<_> = self
            .members
            .iter()
            .map(|member| member.create(builder))
            .collect();
        let struct_members = builder.create_vector(&created_members);
        fb::Structure::create(
            builder,
            &fb::StructureArgs {
                members: Some(struct_members),
            },
        )
    }

    pub fn size(&self) -> Option<u64> {
        self.members.iter().fold(None, |size, member| {
            // If an unknown member size is encountered, the structure size has to be unknown.
            Some((member.offset + member.size()?).max(size.unwrap_or(0)))
        })
    }
}

impl Distribution<StructureClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> StructureClass {
        let rand_in_member_len = rng.gen_range(0..1);
        StructureClass {
            members: rng
                .sample_iter::<StructureMember, _>(Standard)
                .take(rand_in_member_len)
                .collect(),
        }
    }
}
