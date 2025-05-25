use crate::{fb_symbol as fb, FlatBufferObject};
use flatbuffers::WIPOffset;
use std::hash::Hash;

use crate::cached_builder::CachedFlatBufferBuilder;
pub use fb::SymbolClass;
pub use fb::SymbolModifiers;

#[derive(Clone, Debug, PartialEq)]
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
}

impl FlatBufferObject for Symbol {
    type FbType<'fbb> = fb::Symbol<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let name = builder.create_string(&self.name);

        fb::Symbol::create(
            builder,
            &fb::SymbolArgs {
                name: Some(name),
                modifiers: self.modifiers,
                class: self.class,
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let sym = Self {
            name: value.name()?.to_string(),
            modifiers: value.modifiers(),
            class: value.class(),
        };

        Some(sym)
    }
}

impl Eq for Symbol {}

impl Hash for Symbol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.modifiers.bits().hash(state);
        self.class.hash(state);
    }
}

impl Ord for Symbol {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // NOTE: Flatbuffers currently do not add Ord impl for bitfields.
        self.name
            .cmp(&other.name)
            .then(self.modifiers.bits().cmp(&other.modifiers.bits()))
            .then(self.class.cmp(&other.class))
    }
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut builder = CachedFlatBufferBuilder::new();
        let symbol = Symbol {
            name: "".to_string(),
            modifiers: SymbolModifiers::empty(),
            class: SymbolClass::Data,
        };
        let _created_symbol = symbol.create(&mut builder);
        // TODO: Add actual tests.
    }
}
