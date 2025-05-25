pub(crate) mod cached_builder;
pub mod chunk;
pub mod mock;
pub mod signature;
pub mod symbol;
pub mod target;
pub mod r#type;

#[allow(warnings)]
#[rustfmt::skip]
mod gen_flatbuffers;

use crate::cached_builder::CachedFlatBufferBuilder;
use crate::chunk::Chunk;
use crate::signature::function::FunctionGUID;
use flatbuffers::{Follow, UnionWIPOffset, Verifiable, WIPOffset};
use gen_flatbuffers::sig_bin as fb_sig;
use gen_flatbuffers::symbol_bin as fb_symbol;
use gen_flatbuffers::target_bin as fb_target;
use gen_flatbuffers::type_bin as fb_type;
use gen_flatbuffers::warp as fb_warp;
use std::fmt::{Debug, Display};

/// The current file version.
pub const FILE_VERSION: u16 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct WarpFileHeader {
    pub version: u16,
}

impl WarpFileHeader {
    pub fn new() -> Self {
        Self {
            version: FILE_VERSION,
        }
    }
}

impl Default for WarpFileHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl FlatBufferObject for WarpFileHeader {
    type FbType<'fbb> = fb_warp::FileHeader<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        fb_warp::FileHeader::create(
            builder,
            &fb_warp::FileHeaderArgs {
                version: self.version,
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        Some(Self {
            version: value.version(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WarpFile<'fbb> {
    pub header: WarpFileHeader,
    pub chunks: Vec<Chunk<'fbb>>,
}

impl<'fbb> WarpFile<'fbb> {
    pub fn new(header: WarpFileHeader, chunks: Vec<Chunk<'fbb>>) -> Self {
        Self { header, chunks }
    }

    // TODO: This is a little bad, is there any way we can "stream" the file? We can stream the chunks.
    // TODO: I just need to figure out how the reader and writer streams for pdb and dwarf work in rust.

    pub fn to_owned(&self) -> Self {
        Self {
            header: self.header.clone(),
            chunks: self.chunks.iter().map(|c| c.to_owned()).collect(),
        }
    }

    pub fn from_bytes(bytes: &'fbb [u8]) -> Option<Self> {
        let object = flatbuffers::root::<fb_warp::File>(bytes).ok()?;
        Self::from_object(&object)
    }

    pub fn from_owned_bytes(bytes: Vec<u8>) -> Option<Self> {
        let object = flatbuffers::root::<fb_warp::File>(&bytes).ok()?;
        Self::from_owned_object(&object)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut builder = CachedFlatBufferBuilder::new();
        let root = self.create(&mut builder);
        builder.finish_minimal(root);
        builder.finished_data().to_vec()
    }

    fn create(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<fb_warp::File<'fbb>> {
        let header = self.header.create(builder);
        let _chunks: Vec<_> = self
            .chunks
            .iter()
            .flat_map(|c| c.create_object(builder))
            .collect();
        let chunks = builder.create_vector(&_chunks);
        fb_warp::File::create(
            builder,
            &fb_warp::FileArgs {
                header: Some(header),
                chunks: Some(chunks),
            },
        )
    }

    fn from_object(value: &fb_warp::File<'fbb>) -> Option<Self> {
        let _chunks = value.chunks()?;
        let chunks = _chunks
            .iter()
            .flat_map(|c| Chunk::from_object(&c))
            .collect();
        Some(Self {
            header: WarpFileHeader::from_object(&value.header())?,
            chunks,
        })
    }

    fn from_owned_object(value: &fb_warp::File) -> Option<Self> {
        let _chunks = value.chunks()?;
        let chunks = _chunks
            .iter()
            .flat_map(|c| Chunk::from_owned_object(&c))
            .collect();
        Some(Self {
            header: WarpFileHeader::from_object(&value.header())?,
            chunks,
        })
    }
}

pub(crate) trait FlatBufferObject: Sized {
    type FbType<'fbb>: Follow<'fbb> + Verifiable;

    /// Construct the flat buffer object that [`Self`] represents within the given [`CachedFlatBufferBuilder`].
    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>>;

    #[allow(dead_code)]
    fn to_bytes(&self) -> Vec<u8> {
        let mut builder = CachedFlatBufferBuilder::new();
        let root = self.create(&mut builder);
        builder.finish_minimal(root);
        builder.finished_data().to_vec()
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self>;
}

pub(crate) trait FlatBufferUnion {
    fn create(&self, builder: &mut CachedFlatBufferBuilder) -> WIPOffset<UnionWIPOffset>;
}

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

impl Display for gen_flatbuffers::sig_bin::FunctionGUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", FunctionGUID::from(*self))
    }
}
