use bon::Builder;

use crate::fb_type as fb;
use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct DescriptorModifierClass {
    pub description: String,
}

impl DescriptorModifierClass {
    pub fn new(description: String) -> Self {
        Self { description }
    }
}

impl DescriptorModifierClass {
    fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::DescriptorModifierClass<'a>> {
        let description = builder.create_string(&self.description);
        fb::DescriptorModifierClass::create(
            builder,
            &fb::DescriptorModifierClassArgs {
                description: Some(description),
            },
        )
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct MetadataModifierClass<T> {
    pub key: String,
    pub value: T,
}

impl<T> MetadataModifierClass<T> {
    pub fn new(key: String, value: T) -> Self {
        Self { key, value }
    }
}

impl<T: Sized + AsRef<[u8]>> MetadataModifierClass<T> {
    fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::MetadataModifierClass<'a>> {
        let key = builder.create_string(&self.key);
        let value = builder.create_vector(self.value.as_ref());
        fb::MetadataModifierClass::create(
            builder,
            &fb::MetadataModifierClassArgs {
                key: Some(key),
                value: Some(value),
            },
        )
    }
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum TypeModifierClass {
    Constant,
    Volatile,
    Descriptor(DescriptorModifierClass),
    StringMetadata(MetadataModifierClass<String>),
    RawMetadata(MetadataModifierClass<Vec<u8>>),
}

impl TypeModifierClass {
    pub fn ty(&self) -> fb::TypeModifierClass {
        match self {
            TypeModifierClass::Constant => fb::TypeModifierClass::ConstantModifierClass,
            TypeModifierClass::Volatile => fb::TypeModifierClass::VolatileModifierClass,
            TypeModifierClass::Descriptor(_) => fb::TypeModifierClass::DescriptorModifierClass,
            TypeModifierClass::StringMetadata(_) | TypeModifierClass::RawMetadata(_) => {
                fb::TypeModifierClass::MetadataModifierClass
            }
        }
    }

    fn create_constant<'a>(
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::ConstantModifierClass<'a>> {
        fb::ConstantModifierClass::create(builder, &fb::ConstantModifierClassArgs {})
    }

    fn create_volatile<'a>(
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::VolatileModifierClass<'a>> {
        fb::VolatileModifierClass::create(builder, &fb::VolatileModifierClassArgs {})
    }

    fn create(&self, builder: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        match self {
            TypeModifierClass::Constant => Self::create_constant(builder).as_union_value(),
            TypeModifierClass::Volatile => Self::create_volatile(builder).as_union_value(),
            TypeModifierClass::Descriptor(class) => class.create(builder).as_union_value(),
            TypeModifierClass::StringMetadata(class) => class.create(builder).as_union_value(),
            TypeModifierClass::RawMetadata(class) => class.create(builder).as_union_value(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct TypeModifier {
    pub class: TypeModifierClass,
}

impl TypeModifier {
    pub fn new(class: TypeModifierClass) -> Self {
        Self { class }
    }

    pub fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::TypeModifier<'a>> {
        let class_type = self.class.ty();
        let class = self.class.create(builder);
        fb::TypeModifier::create(
            builder,
            &fb::TypeModifierArgs {
                class_type,
                class: Some(class),
            },
        )
    }
}
