use flatbuffers::{FlatBufferBuilder, UnionWIPOffset, WIPOffset};

#[derive(Clone, Debug, Default)]
pub struct FunctionSymbolClass;

impl FunctionSymbolClass {
    pub(crate) fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<crate::gen_flatbuffers::symbol_bin::FunctionSymbolClass<'a>> {
        crate::gen_flatbuffers::symbol_bin::FunctionSymbolClass::create(
            builder,
            &crate::gen_flatbuffers::symbol_bin::FunctionSymbolClassArgs {},
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct DataSymbolClass;

impl DataSymbolClass {
    fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<crate::gen_flatbuffers::symbol_bin::DataSymbolClass<'a>> {
        crate::gen_flatbuffers::symbol_bin::DataSymbolClass::create(
            builder,
            &crate::gen_flatbuffers::symbol_bin::DataSymbolClassArgs {},
        )
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum SymbolClass {
    Function,
    Data,
}

impl SymbolClass {
    pub fn ty(&self) -> crate::gen_flatbuffers::symbol_bin::SymbolClass {
        match self {
            SymbolClass::Function => {
                crate::gen_flatbuffers::symbol_bin::SymbolClass::FunctionSymbolClass
            }
            SymbolClass::Data => crate::gen_flatbuffers::symbol_bin::SymbolClass::DataSymbolClass,
        }
    }

    pub fn create(&self, builder: &mut FlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        match self {
            SymbolClass::Function => FunctionSymbolClass.create(builder).as_union_value(),
            SymbolClass::Data => DataSymbolClass.create(builder).as_union_value(),
        }
    }
}

impl From<crate::gen_flatbuffers::symbol_bin::FunctionSymbolClass<'_>> for SymbolClass {
    fn from(_value: crate::gen_flatbuffers::symbol_bin::FunctionSymbolClass<'_>) -> Self {
        Self::Function
    }
}

impl From<crate::gen_flatbuffers::symbol_bin::DataSymbolClass<'_>> for SymbolClass {
    fn from(_value: crate::gen_flatbuffers::symbol_bin::DataSymbolClass<'_>) -> Self {
        Self::Data
    }
}
