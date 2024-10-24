use bon::Builder;

use crate::fb_type as fb;
use crate::r#type::Type;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use rand::distributions::{Alphanumeric, DistString, Distribution, Standard};
use rand::Rng;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder)]
#[builder(on(String, into))]
pub struct UnionMember {
    pub name: String,
    pub ty: Type,
}

impl UnionMember {
    pub fn new(name: String, ty: Type) -> Self {
        Self { name, ty }
    }

    fn create<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<fb::UnionMember<'a>> {
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

    fn size(&self) -> Option<u64> {
        self.ty.size()
    }
}

impl From<fb::UnionMember<'_>> for UnionMember {
    fn from(value: fb::UnionMember<'_>) -> Self {
        Self {
            name: value.name().unwrap().to_string(),
            ty: value.type_().into(),
        }
    }
}

impl Distribution<UnionMember> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UnionMember {
        UnionMember {
            name: Alphanumeric.sample_string(rng, 16),
            ty: rng.gen(),
        }
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

    pub(crate) fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::Union<'a>> {
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

    pub fn size(&self) -> Option<u64> {
        // Get the largest union member.
        self.members
            .iter()
            .max_by(|x, y| x.size().cmp(&y.size()))
            .map(|z| z.ty.size())?
    }
}

impl From<fb::Union<'_>> for UnionClass {
    fn from(value: fb::Union<'_>) -> Self {
        Self {
            members: value.members().unwrap().iter().map(Into::into).collect(),
        }
    }
}

impl Distribution<UnionClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UnionClass {
        let rand_in_member_len = rng.gen_range(0..20);
        UnionClass {
            members: rng.sample_iter(Standard).take(rand_in_member_len).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::r#type::class::{IntegerClass, TypeClass, UnionClass, UnionMember};
    use crate::r#type::guid::TypeGUID;
    use crate::r#type::{Alignment, Type};
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
            class: Box::from(TypeClass::Integer(IntegerClass {
                width: Some(64),
                signed: true,
            })),
            confidence: 255,
            modifiers: vec![],
            alignment: Alignment::Access,
            ancestors: vec![],
        };
        let void_type = Type {
            name: Some("my_void".to_owned()),
            class: Box::from(TypeClass::Void),
            confidence: 255,
            modifiers: vec![],
            alignment: Alignment::Access,
            ancestors: vec![],
        };
        assert_eq!(
            Type {
                name: Some("my_union".to_owned()),
                class: Box::from(TypeClass::Union(UnionClass {
                    members: vec![
                        UnionMember {
                            name: "one".to_owned(),
                            ty: int_type
                        },
                        UnionMember {
                            name: "two".to_owned(),
                            ty: void_type
                        }
                    ]
                })),
                confidence: 255,
                modifiers: vec![],
                alignment: Alignment::Access,
                ancestors: vec![]
            },
            built_union_type()
        )
    }
}
