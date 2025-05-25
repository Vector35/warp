use bon::Builder;

use crate::cached_builder::CachedFlatBufferBuilder;
use crate::r#type::Type;
use crate::{fb_type as fb, FlatBufferObject};
use flatbuffers::WIPOffset;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder)]
#[builder(on(String, into))]
pub struct UnionMember {
    pub name: String,
    pub ty: Box<Type>,
}

impl UnionMember {
    pub fn new(name: String, ty: Type) -> Self {
        Self {
            name,
            ty: Box::new(ty),
        }
    }

    fn size(&self) -> Option<u64> {
        self.ty.size()
    }
}

impl FlatBufferObject for UnionMember {
    type FbType<'fbb> = fb::UnionMember<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let name = builder.create_string(&self.name);
        let member_type = self.ty.create(builder);
        fb::UnionMember::create(
            builder,
            &fb::UnionMemberArgs {
                name: Some(name),
                type_: Some(member_type),
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let member = Self {
            name: value.name()?.to_string(),
            ty: Box::new(Type::from_object(&value.type_())?),
        };

        Some(member)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct UnionClass {
    pub members: Vec<UnionMember>,
}

impl UnionClass {
    pub fn new(members: Vec<UnionMember>) -> Self {
        Self { members }
    }

    pub fn size(&self) -> Option<u64> {
        // Get the largest union member.
        self.members
            .iter()
            .max_by(|x, y| x.size().cmp(&y.size()))
            .map(|z| z.ty.size())?
    }
}

impl FlatBufferObject for UnionClass {
    type FbType<'fbb> = fb::Union<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let resolved_members: Vec<_> = self
            .members
            .iter()
            .map(|member| member.create(builder))
            .collect();
        let union_members = builder.create_vector(&resolved_members);
        fb::Union::create(
            builder,
            &fb::UnionArgs {
                members: Some(union_members),
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let class = Self {
            members: value
                .members()
                .unwrap_or_default()
                .iter()
                .flat_map(|member| FlatBufferObject::from_object(&member))
                .collect(),
        };

        Some(class)
    }
}

#[cfg(test)]
mod tests {
    use crate::r#type::class::{IntegerClass, TypeClass, UnionClass, UnionMember};
    use crate::r#type::guid::TypeGUID;
    use crate::r#type::Type;
    use uuid::{uuid, Uuid};

    const UNION_TYPE_UUID: Uuid = uuid!("1570a765-3626-5c97-a18e-4b4652a77398");

    fn built_union_type() -> Type {
        let void_type = Type::builder()
            .name("my_void".to_owned())
            .class(TypeClass::Void)
            .build();

        let int_class = IntegerClass::builder().width(64).signed(true).build();
        let int_type = Type::builder()
            .name("my_int".to_owned())
            .class(int_class)
            .build();

        let union_class = UnionClass::builder()
            .members(vec![
                UnionMember::new("one".to_owned(), int_type),
                UnionMember::new("two".to_owned(), void_type),
            ])
            .build();
        Type::builder()
            .name("my_union".to_owned())
            .class(union_class)
            .build()
    }

    #[test]
    fn union_size() {
        assert_eq!(Some(64), built_union_type().size());
    }

    #[test]
    fn union_guid_v1() {
        assert_eq!(TypeGUID::from(UNION_TYPE_UUID), built_union_type().into());
    }

    #[test]
    fn union_type() {
        let int_type = Type {
            name: Some("my_int".to_owned()),
            class: TypeClass::Integer(IntegerClass {
                width: Some(64),
                signed: true,
            }),
            confidence: 255,
            modifiers: Default::default(),
            metadata: vec![],
            alignment: Default::default(),
            ancestors: vec![],
        };
        let void_type = Type {
            name: Some("my_void".to_owned()),
            class: TypeClass::Void,
            confidence: 255,
            modifiers: Default::default(),
            metadata: vec![],
            alignment: Default::default(),
            ancestors: vec![],
        };
        assert_eq!(
            Type {
                name: Some("my_union".to_owned()),
                class: TypeClass::Union(UnionClass {
                    members: vec![
                        UnionMember {
                            name: "one".to_owned(),
                            ty: Box::new(int_type)
                        },
                        UnionMember {
                            name: "two".to_owned(),
                            ty: Box::new(void_type)
                        }
                    ]
                }),
                confidence: 255,
                modifiers: Default::default(),
                metadata: vec![],
                alignment: Default::default(),
                ancestors: vec![],
            },
            built_union_type()
        )
    }
}
