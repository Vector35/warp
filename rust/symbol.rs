pub mod class;

use crate::fb_symbol as fb;
use crate::symbol::class::SymbolClass;
use flatbuffers::{FlatBufferBuilder, WIPOffset};

pub use fb::SymbolModifiers;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Symbol {
    pub name: String,
    pub modifiers: SymbolModifiers,
    pub class: SymbolClass,
}

impl Symbol {
    pub fn new(name: impl Into<String>, class: SymbolClass, modifiers: SymbolModifiers) -> Self {
        Self {
            name: name.into(),
            modifiers,
            class,
        }
    }

    pub(crate) fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::Symbol<'a>> {
        let name = builder.create_string(&self.name);
        let class_type = self.class.ty();
        let class = self.class.create(builder);

        fb::Symbol::create(
            builder,
            &fb::SymbolArgs {
                name: Some(name),
                modifiers: self.modifiers,
                class_type,
                class: Some(class),
            },
        )
    }
}

impl From<fb::Symbol<'_>> for Symbol {
    fn from(value: fb::Symbol<'_>) -> Self {
        let name = value.name().unwrap().to_string();
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

        Self {
            name,
            modifiers: value.modifiers(),
            class,
        }
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
            modifiers: SymbolModifiers::empty(),
            class: SymbolClass::Data,
        };
        let _created_symbol = symbol.create(&mut builder);
        // TODO: Add actual tests.
    }
}
