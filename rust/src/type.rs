use bon::bon;
use std::hash::Hash;

use crate::cached_builder::CachedFlatBufferBuilder;
use crate::r#type::class::{
    ArrayClass, BooleanClass, CharacterClass, EnumerationClass, FloatClass, FunctionClass,
    IntegerClass, PointerClass, ReferrerClass, StructureClass, TypeClass, UnionClass,
};
use crate::r#type::guid::TypeGUID;
use crate::{fb_type as fb, FlatBufferObject, FlatBufferUnion};
use flatbuffers::WIPOffset;

// We re-export bit flags as there is no need to wrap them.
pub use fb::MetadataValueType;
pub use fb::TypeModifiers;

pub mod chunk;
pub mod class;
pub mod guid;

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

    pub fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<fb::ComputedType<'fbb>> {
        let created_ty = self.ty.create(builder);
        fb::ComputedType::create(
            builder,
            &fb::ComputedTypeArgs {
                guid: Some(&self.guid.into()),
                type_: Some(created_ty),
            },
        )
    }
}

impl FlatBufferObject for ComputedType {
    type FbType<'fbb> = fb::ComputedType<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let created_ty = self.ty.create(builder);
        fb::ComputedType::create(
            builder,
            &fb::ComputedTypeArgs {
                guid: Some(&self.guid.into()),
                type_: Some(created_ty),
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let ty: Type = Type::from_object(&value.type_()?)?;
        let guid = TypeGUID::from(*value.guid());
        Some(Self::new_with_guid(ty, guid))
    }
}

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Alignment {
    #[default]
    Access,
    Fixed(u16),
}

impl From<u16> for Alignment {
    fn from(value: u16) -> Self {
        match value {
            0 => Self::Access,
            value => Self::Fixed(value),
        }
    }
}

impl From<Alignment> for u16 {
    fn from(value: Alignment) -> Self {
        match value {
            Alignment::Access => 0,
            Alignment::Fixed(value) => value,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Type {
    pub name: Option<String>,
    pub class: TypeClass,
    // TODO: Make confidence a modifier?
    pub confidence: u8,
    pub modifiers: TypeModifiers,
    // TODO: Make this a HashMap?
    pub metadata: Vec<TypeMetadata>,
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
        modifiers: Option<TypeModifiers>,
        metadata: Option<Vec<TypeMetadata>>,
        alignment: Option<Alignment>,
        ancestors: Option<Vec<TypeGUID>>,
    ) -> Self {
        Self {
            name: name.map(|n| n.into()),
            class: class.into(),
            confidence: confidence.unwrap_or(u8::MAX),
            modifiers: modifiers.unwrap_or_default(),
            metadata: metadata.unwrap_or_default(),
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
        flatbuffers::root::<fb::Type>(buf)
            .ok()
            .and_then(|ty| Type::from_object(&ty))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut builder = CachedFlatBufferBuilder::new();
        let fb_type = self.create(&mut builder);
        builder.finish_minimal(fb_type);
        builder.finished_data().to_vec()
    }

    pub fn is_const(&self) -> bool {
        self.modifiers.contains(TypeModifiers::Constant)
    }

    pub fn is_volatile(&self) -> bool {
        self.modifiers.contains(TypeModifiers::Volatile)
    }

    pub fn size(&self) -> Option<u64> {
        self.class.size()
    }
}

impl FlatBufferObject for Type {
    type FbType<'fbb> = fb::Type<'fbb>;

    /// Serialize the type to the flatbuffer builder.
    ///
    /// Unique to this `create` function is the type cache which will reuse existing common types
    /// such as a 32bit signed integer so that the finished data does not contain duplicate data.
    /// This is extremely important to maintaining a reasonable size for large datasets. One thing this
    /// will not cover is cross-buffer references. The referenced cache item must be in the
    /// buffer to reference.
    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        // Check if we have already created this type, if so, return the existing offset.
        if let Some(existing_offset) = builder.cached_type_offsets.get(self) {
            return *existing_offset;
        }

        let name = self.name.as_ref().map(|n| builder.create_string(n));
        let class_type = self.class.ty();
        let class = self.class.create(builder);

        let mut ancestors = None;
        if !self.ancestors.is_empty() {
            let _ancestors = self
                .ancestors
                .iter()
                .map(|guid| fb::TypeGUID::from(*guid))
                .collect::<Vec<_>>();
            ancestors = Some(builder.create_vector(&_ancestors));
        }

        let mut metadata = None;
        if !self.metadata.is_empty() {
            let _metadata = self
                .metadata
                .iter()
                .map(|metadata| metadata.create(builder))
                .collect::<Vec<_>>();
            metadata = Some(builder.create_vector(&_metadata));
        }

        let offset = fb::Type::create(
            builder,
            &fb::TypeArgs {
                name,
                class_type,
                class: Some(class),
                confidence: self.confidence,
                ancestors,
                modifiers: self.modifiers,
                metadata,
                alignment: self.alignment.into(),
            },
        );

        // Make the offset available for future lookups.
        builder.cached_type_offsets.insert(self.clone(), offset);
        offset
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let name = value.name().map(str::to_string);

        let from_type_class = |type_class: fb::TypeClass| match type_class {
            fb::TypeClass::Void => Some(TypeClass::Void),
            fb::TypeClass::Boolean => {
                let bool = value.class_as_boolean()?;
                let class = BooleanClass::from_object(&bool)?;
                Some(TypeClass::Boolean(class))
            }
            fb::TypeClass::Integer => {
                let int = value.class_as_integer()?;
                let class = IntegerClass::from_object(&int)?;
                Some(TypeClass::Integer(class))
            }
            fb::TypeClass::Character => {
                let character = value.class_as_character()?;
                let class = CharacterClass::from_object(&character)?;
                Some(TypeClass::Character(class))
            }
            fb::TypeClass::Float => {
                let float = value.class_as_float()?;
                let class = FloatClass::from_object(&float)?;
                Some(TypeClass::Float(class))
            }
            fb::TypeClass::Pointer => {
                let ptr = value.class_as_pointer()?;
                let class = PointerClass::from_object(&ptr)?;
                Some(TypeClass::Pointer(class))
            }
            fb::TypeClass::Array => {
                let array = value.class_as_array()?;
                let class = ArrayClass::from_object(&array)?;
                Some(TypeClass::Array(class))
            }
            fb::TypeClass::Structure => {
                let structure = value.class_as_structure()?;
                let class = StructureClass::from_object(&structure)?;
                Some(TypeClass::Structure(class))
            }
            fb::TypeClass::Enumeration => {
                let enumeration = value.class_as_enumeration()?;
                let class = EnumerationClass::from_object(&enumeration)?;
                Some(TypeClass::Enumeration(class))
            }
            fb::TypeClass::Union => {
                let union = value.class_as_union()?;
                let class = UnionClass::from_object(&union)?;
                Some(TypeClass::Union(class))
            }
            fb::TypeClass::Function => {
                let function = value.class_as_function()?;
                let class = FunctionClass::from_object(&function)?;
                Some(TypeClass::Function(class))
            }
            fb::TypeClass::Referrer => {
                let referrer = value.class_as_referrer()?;
                let class = ReferrerClass::from_object(&referrer)?;
                Some(TypeClass::Referrer(class))
            }
            // Somehow we got an unknown type class, possibly malformed data.
            _ => None,
        };

        let class = from_type_class(value.class_type())?;
        let ty = Self {
            name,
            class,
            confidence: value.confidence(),
            modifiers: value.modifiers(),
            metadata: value
                .metadata()
                .unwrap_or_default()
                .iter()
                .flat_map(|meta| TypeMetadata::from_object(&meta))
                .collect(),
            alignment: value.alignment().into(),
            ancestors: value
                .ancestors()
                .unwrap_or_default()
                .iter()
                .map(TypeGUID::from)
                .collect(),
        };

        Some(ty)
    }
}

impl Eq for Type {}

impl Hash for Type {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.class.hash(state);
        self.confidence.hash(state);
        // NOTE: Flatbuffers currently do not add Hash impl for bitfields.
        self.modifiers.bits().hash(state);
        self.alignment.hash(state);
        self.ancestors.hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum TypeMetadataValue {
    Raw(Vec<u8>),
    String(String),
}

impl TypeMetadataValue {
    pub fn value_type(&self) -> MetadataValueType {
        match self {
            TypeMetadataValue::Raw(_) => MetadataValueType::Raw,
            TypeMetadataValue::String(_) => MetadataValueType::String,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            TypeMetadataValue::Raw(value) => value.clone(),
            TypeMetadataValue::String(value) => value.as_bytes().to_vec(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct TypeMetadata {
    pub key: String,
    pub value: TypeMetadataValue,
}

impl TypeMetadata {
    pub fn new_raw(key: String, value: Vec<u8>) -> Self {
        Self {
            key,
            value: TypeMetadataValue::Raw(value),
        }
    }

    pub fn new_string(key: String, value: String) -> Self {
        Self {
            key,
            value: TypeMetadataValue::String(value),
        }
    }
}

impl FlatBufferObject for TypeMetadata {
    type FbType<'fbb> = fb::TypeMetadata<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let key = builder.create_string(&self.key);
        let value_type = self.value.value_type();
        let _value = self.value.to_bytes();
        let value = builder.create_vector(&_value);
        fb::TypeMetadata::create(
            builder,
            &fb::TypeMetadataArgs {
                key: Some(key),
                value_type,
                value: Some(value),
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let key = value.key().to_string();
        let value_bytes = value.value().as_ref()?.bytes();
        let value = match value.value_type() {
            MetadataValueType::Raw => TypeMetadataValue::Raw(value_bytes.to_vec()),
            MetadataValueType::String => {
                TypeMetadataValue::String(String::from_utf8(value_bytes.to_vec()).ok()?)
            }
            // Fallback to raw if unknown type.
            _ => TypeMetadataValue::Raw(value_bytes.to_vec()),
        };
        Some(Self { key, value })
    }
}
