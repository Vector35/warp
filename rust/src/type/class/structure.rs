use bon::{bon, Builder};
use std::hash::Hash;

use crate::cached_builder::CachedFlatBufferBuilder;
use crate::r#type::Type;
use crate::{fb_type as fb, FlatBufferObject};
use flatbuffers::WIPOffset;

// We re-export bit flags as there is no need to wrap them.
pub use fb::StructureMemberModifiers;

#[derive(Clone, Debug, PartialEq)]
pub struct StructureMember {
    pub name: Option<String>,
    pub offset: u64,
    pub ty: Box<Type>,
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
            ty: Box::new(ty),
            modifiers: modifiers.unwrap_or_default(),
        }
    }
}

impl StructureMember {
    fn size(&self) -> Option<u64> {
        self.ty.size()
    }
}

impl FlatBufferObject for StructureMember {
    type FbType<'fbb> = fb::StructureMember<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
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

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let member = Self {
            name: value.name().map(str::to_string),
            offset: value.offset().into(),
            ty: Box::new(Type::from_object(&value.type_())?),
            modifiers: value.modifiers(),
        };

        Some(member)
    }
}

impl Eq for StructureMember {}

impl Hash for StructureMember {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.offset.hash(state);
        self.ty.hash(state);
        // NOTE: Flatbuffers currently do not add Hash impl for bitfields.
        self.modifiers.bits().hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct StructureClass {
    pub members: Vec<StructureMember>,
}

impl StructureClass {
    pub fn new(members: Vec<StructureMember>) -> Self {
        Self { members }
    }

    pub fn size(&self) -> Option<u64> {
        self.members.iter().fold(None, |size, member| {
            // If an unknown member size is encountered, the structure size has to be unknown.
            Some((member.offset + member.size()?).max(size.unwrap_or(0)))
        })
    }
}

impl FlatBufferObject for StructureClass {
    type FbType<'fbb> = fb::Structure<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
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

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let class = Self {
            members: value
                .members()
                .unwrap_or_default()
                .iter()
                .flat_map(|value| FlatBufferObject::from_object(&value))
                .collect(),
        };

        Some(class)
    }
}
