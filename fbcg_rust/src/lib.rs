#[allow(warnings)]
pub mod gen_flatbuffers;

pub use gen_flatbuffers::sig_bin as fb_sig;
pub use gen_flatbuffers::symbol_bin as fb_symbol;
pub use gen_flatbuffers::type_bin as fb_type;

impl From<u16> for gen_flatbuffers::type_bin::BitWidth {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<&gen_flatbuffers::type_bin::BitWidth> for u16 {
    fn from(value: &gen_flatbuffers::type_bin::BitWidth) -> Self {
        value.value()
    }
}

impl From<u64> for gen_flatbuffers::type_bin::BitSize {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<&gen_flatbuffers::type_bin::BitSize> for u64 {
    fn from(value: &gen_flatbuffers::type_bin::BitSize) -> Self {
        value.value()
    }
}

impl From<u64> for gen_flatbuffers::type_bin::UnsignedBitOffset {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

impl From<&gen_flatbuffers::type_bin::UnsignedBitOffset> for u64 {
    fn from(value: &gen_flatbuffers::type_bin::UnsignedBitOffset) -> Self {
        value.value()
    }
}

impl From<i64> for gen_flatbuffers::type_bin::BitOffset {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<&gen_flatbuffers::type_bin::BitOffset> for i64 {
    fn from(value: &gen_flatbuffers::type_bin::BitOffset) -> Self {
        value.value()
    }
}

impl From<i64> for gen_flatbuffers::type_bin::BitShift {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}

impl From<&gen_flatbuffers::type_bin::BitShift> for i64 {
    fn from(value: &gen_flatbuffers::type_bin::BitShift) -> Self {
        value.value()
    }
}
