use bon::bon;

use crate::fb_type as fb;
use crate::r#type::class::TypeClass;
use crate::r#type::guid::TypeGUID;
use crate::r#type::modifier::{TypeModifier, TypeModifierClass};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use rand::distributions::{Alphanumeric, DistString, Distribution, Standard};
use rand::Rng;

pub mod class;
pub mod guid;
pub mod modifier;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ComputedType {
    pub guid: TypeGUID,
    pub ty: Type,
}

impl ComputedType {
    pub fn new(ty: Type) -> ComputedType {
        let guid = TypeGUID::from(&ty);
        Self::new_with_guid(ty, guid)
    }

    pub fn new_with_guid(ty: Type, guid: TypeGUID) -> ComputedType {
        ComputedType { guid, ty }
    }

    pub fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::ComputedType<'a>> {
        let created_ty = self.ty.create(builder);
        let guid_str = builder.create_string(&self.guid.to_string());
        fb::ComputedType::create(
            builder,
            &fb::ComputedTypeArgs {
                guid: Some(guid_str),
                type_: Some(created_ty),
            },
        )
    }
}

// TODO: Clearly this might fail, we should make this TryFrom.
impl From<fb::ComputedType<'_>> for ComputedType {
    fn from(value: fb::ComputedType<'_>) -> Self {
        let ty: Type = value.type_().unwrap().into();
        let guid = value.guid().parse::<TypeGUID>().unwrap();
        Self::new_with_guid(ty, guid)
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Alignment {
    #[default]
    Access,
    Fixed(u16),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Type {
    pub name: Option<String>,
    pub class: Box<TypeClass>,
    // TODO: Type confidence?
    pub confidence: u8,
    pub modifiers: Vec<TypeModifier>,
    pub alignment: Alignment,
    pub ancestors: Vec<TypeGUID>,
}

#[bon]
impl Type {
    #[builder]
    pub fn new<T: Into<String>>(
        name: Option<T>,
        class: impl Into<TypeClass>,
        confidence: Option<u8>,
        modifiers: Option<Vec<TypeModifier>>,
        alignment: Option<Alignment>,
        ancestors: Option<Vec<TypeGUID>>,
    ) -> Self {
        Self {
            name: name.map(|n| n.into()),
            class: Box::new(class.into()),
            confidence: confidence.unwrap_or(u8::MAX),
            modifiers: modifiers.unwrap_or_default(),
            alignment: alignment.unwrap_or_default(),
            ancestors: ancestors.unwrap_or_default(),
        }
    }
}

impl Type {
    /// Copy a type and make it an ancestor, effectively allowing a mutation of the type.
    pub fn from_ancestor(ancestor: &Type) -> Self {
        let mut new_type = ancestor.clone();
        new_type.ancestors.push(TypeGUID::from(ancestor));
        new_type
    }

    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        flatbuffers::root::<fb::Type>(buf).ok().map(Into::into)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut builder = FlatBufferBuilder::new();
        let fb_type = self.create(&mut builder);
        builder.finish_minimal(fb_type);
        builder.finished_data().to_vec()
    }

    pub fn is_const(&self) -> bool {
        self.modifiers
            .contains(&TypeModifier::new(TypeModifierClass::Constant))
    }

    pub fn is_volatile(&self) -> bool {
        self.modifiers
            .contains(&TypeModifier::new(TypeModifierClass::Volatile))
    }

    pub(crate) fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::Type<'a>> {
        let name = self.name.as_ref().map(|n| builder.create_string(n));
        let class_type = self.class.ty();
        let class = self.class.create(builder);

        let mut ancestors = None;
        if !self.ancestors.is_empty() {
            let _ancestors = self
                .ancestors
                .iter()
                .map(|a| builder.create_string(&a.to_string()))
                .collect::<Vec<_>>();
            ancestors = Some(builder.create_vector(&_ancestors));
        }

        let mut modifiers = None;
        if !self.modifiers.is_empty() {
            let _modifiers = self
                .modifiers
                .iter()
                .map(|modifier| modifier.create(builder))
                .collect::<Vec<_>>();
            modifiers = Some(builder.create_vector(&_modifiers));
        }

        fb::Type::create(
            builder,
            &fb::TypeArgs {
                name,
                class_type,
                class: Some(class),
                confidence: self.confidence,
                ancestors,
                modifiers,
                // TODO: Alignment
                alignment_type: Default::default(),
                alignment_: None,
            },
        )
    }

    pub fn size(&self) -> Option<u64> {
        self.class.size()
    }
}

impl From<fb::Type<'_>> for Type {
    fn from(value: fb::Type<'_>) -> Self {
        let name = value.name().map(str::to_string);

        let from_type_class = |type_class: fb::TypeClass| match type_class {
            fb::TypeClass::Void => Some(TypeClass::Void),
            fb::TypeClass::Boolean => {
                let bool = value.class_as_boolean()?;
                Some(TypeClass::Boolean(bool.into()))
            }
            fb::TypeClass::Integer => {
                let int = value.class_as_integer()?;
                Some(TypeClass::Integer(int.into()))
            }
            fb::TypeClass::Character => {
                let charcter = value.class_as_character()?;
                Some(TypeClass::Character(charcter.into()))
            }
            fb::TypeClass::Float => {
                let float = value.class_as_float()?;
                Some(TypeClass::Float(float.into()))
            }
            fb::TypeClass::Pointer => {
                let ptr = value.class_as_pointer()?;
                Some(TypeClass::Pointer(ptr.into()))
            }
            fb::TypeClass::Array => {
                let array = value.class_as_array()?;
                Some(TypeClass::Array(array.into()))
            }
            fb::TypeClass::Structure => {
                let structure = value.class_as_structure()?;
                Some(TypeClass::Structure(structure.into()))
            }
            fb::TypeClass::Enumeration => {
                let enumeration = value.class_as_enumeration()?;
                Some(TypeClass::Enumeration(enumeration.into()))
            }
            fb::TypeClass::Union => {
                let union = value.class_as_union()?;
                Some(TypeClass::Union(union.into()))
            }
            fb::TypeClass::Function => {
                let function = value.class_as_function()?;
                Some(TypeClass::Function(function.into()))
            }
            fb::TypeClass::Referrer => {
                let referrer = value.class_as_referrer()?;
                Some(TypeClass::Referrer(referrer.into()))
            }
            _ => unreachable!(),
        };

        let class = from_type_class(value.class_type()).unwrap();

        Self {
            name,
            class: Box::new(class),
            confidence: value.confidence(),
            // TODO: Get modifiers
            modifiers: vec![],
            // TODO: Get alignment
            alignment: Default::default(),
            ancestors: value
                .ancestors()
                .unwrap_or_default()
                .iter()
                .filter_map(|a| a.parse().ok())
                .collect(),
        }
    }
}

impl Distribution<Type> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Type {
        // 90% chance this type will have a name.
        let name = match rng.gen_bool(0.9) {
            true => Some(Alphanumeric.sample_string(rng, 16)),
            false => None,
        };
        Type {
            name,
            class: Box::new(rng.gen()),
            // class: Box::new(TypeClass::Void),
            confidence: rng.gen(),
            // TODO: Modifiers
            modifiers: vec![],
            // TODO: Alignment
            alignment: Default::default(),
            // TODO: Ancestors
            ancestors: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::r#type::guid::TypeGUID;
    use crate::r#type::Type;
    use rand::Rng;
    use std::collections::HashSet;

    #[test]
    fn test_determinism() {
        let mut rng = rand::thread_rng();
        // Test to make sure that for any given type the uuid will always be the same.
        for _ in 0..1000 {
            let random_type: Type = rng.gen();
            let mut guids: HashSet<TypeGUID> = HashSet::new();
            for _ in 0..100 {
                guids.insert(random_type.to_owned().into());
            }
            assert_eq!(guids.len(), 1, "{:?}", guids);
            let buf = random_type.to_bytes();
            let fb_rand_type = Type::from_bytes(buf.as_slice()).unwrap();
            assert_eq!(random_type, fb_rand_type);
            assert_eq!(TypeGUID::from(random_type), TypeGUID::from(fb_rand_type));
        }
    }
}
