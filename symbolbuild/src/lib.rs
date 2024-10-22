use fbcg_rust::fb_symbol as fb;
use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

pub mod prelude {
    pub use crate::{FunctionSymbolClass, Symbol, SymbolClass, SymbolModifier};
}

pub trait Create {
    type FBType<'a>;
    fn create<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Self::FBType<'a>>;
}

#[derive(Clone, Debug, Default)]
pub struct FunctionSymbolClass;

impl Create for FunctionSymbolClass {
    type FBType<'a> = fb::FunctionSymbolClass<'a>;

    fn create<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Self::FBType<'a>> {
        fb::FunctionSymbolClass::create(builder, &fb::FunctionSymbolClassArgs {})
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SymbolModifier {
    External,
    Exported,
}

impl From<SymbolModifier> for fb::SymbolModifiers {
    fn from(value: SymbolModifier) -> Self {
        match value {
            SymbolModifier::External => Self::External,
            SymbolModifier::Exported => Self::Exported,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DataSymbolClass;

impl Create for DataSymbolClass {
    type FBType<'a> = fb::DataSymbolClass<'a>;

    fn create<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Self::FBType<'a>> {
        fb::DataSymbolClass::create(builder, &fb::DataSymbolClassArgs {})
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum SymbolClass {
    Function,
    Data,
}

impl SymbolClass {
    pub fn ty(&self) -> fb::SymbolClass {
        match self {
            SymbolClass::Function => fb::SymbolClass::FunctionSymbolClass,
            SymbolClass::Data => fb::SymbolClass::DataSymbolClass,
        }
    }
}

impl From<fb::FunctionSymbolClass<'_>> for SymbolClass {
    fn from(_value: fb::FunctionSymbolClass<'_>) -> Self {
        Self::Function
    }
}

impl From<fb::DataSymbolClass<'_>> for SymbolClass {
    fn from(_value: fb::DataSymbolClass<'_>) -> Self {
        Self::Data
    }
}

impl Create for SymbolClass {
    type FBType<'a> = UnionWIPOffset;

    fn create<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Self::FBType<'a>> {
        match self {
            SymbolClass::Function => FunctionSymbolClass.create(builder).as_union_value(),
            SymbolClass::Data => DataSymbolClass.create(builder).as_union_value(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Symbol {
    pub name: String,
    pub modifiers: Vec<SymbolModifier>,
    pub class: SymbolClass,
}

impl Symbol {
    pub fn new(
        name: impl Into<String>,
        class: SymbolClass,
        modifiers: Vec<SymbolModifier>,
    ) -> Self {
        Self {
            name: name.into(),
            modifiers,
            class,
        }
    }
}

impl From<fb::Symbol<'_>> for Symbol {
    fn from(value: fb::Symbol<'_>) -> Self {
        // TODO: I would like this conversion to be on `SymbolClass` instead.
        let class = match value.class_type() {
            fb::SymbolClass::FunctionSymbolClass => {
                SymbolClass::from(value.class_as_function_symbol_class().unwrap())
            }
            fb::SymbolClass::DataSymbolClass => {
                SymbolClass::from(value.class_as_data_symbol_class().unwrap())
            }
            _ => unreachable!(),
        };

        let name = value.name().unwrap().to_string();

        // TODO: This is so ugly.
        let mut modifiers: Vec<SymbolModifier> = Vec::new();
        if value.modifiers().contains(fb::SymbolModifiers::External) {
            modifiers.push(SymbolModifier::External);
        }
        if value.modifiers().contains(fb::SymbolModifiers::Exported) {
            modifiers.push(SymbolModifier::Exported);
        }

        Self {
            name,
            modifiers,
            class,
        }
    }
}

impl Create for Symbol {
    type FBType<'a> = fb::Symbol<'a>;

    fn create<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Self::FBType<'a>> {
        let name = builder.create_string(&self.name);
        let class_type = self.class.ty();
        let class = self.class.create(builder);

        let modifiers = self
            .modifiers
            .iter()
            .fold(fb::SymbolModifiers::default(), |m, &b| m | b.into());
        fb::Symbol::create(
            builder,
            &fb::SymbolArgs {
                name: Some(name),
                modifiers,
                class_type,
                class: Some(class),
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flatbuffers::FlatBufferBuilder;

    #[test]
    fn it_works() {
        let mut builder = FlatBufferBuilder::with_capacity(100);
        let symbol = Symbol {
            name: "".to_string(),
            modifiers: vec![],
            class: SymbolClass::Data,
        };
        let created_symbol = symbol.create(&mut builder);
    }
}
