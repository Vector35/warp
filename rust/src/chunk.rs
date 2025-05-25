use crate::cached_builder::CachedFlatBufferBuilder;
use crate::gen_flatbuffers::warp as fb;
use crate::r#type::chunk::TypeChunk;
use crate::signature::chunk::SignatureChunk;
use crate::target::Target;
use crate::FlatBufferObject;
pub use fb::ChunkType;
pub use fb::CompressionType;
use flatbuffers::WIPOffset;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use itertools::Itertools;
use std::borrow::Cow;
use std::io::{Read, Write};

// TODO: Splitting and merging.
pub trait ChunkHandler<'fbb>: Sized + AsRef<[u8]> + Clone
where
    Self: 'fbb,
{
    /// The associated version for this chunk type.
    const VERSION: u16;

    fn from_owned_data(data: Vec<u8>) -> Option<Self> {
        Self::from_data(Cow::Owned(data))
    }

    fn from_data(data: Cow<'fbb, [u8]>) -> Option<Self>;

    /// Split the chunk into smaller chunks based on the number of chunk items.
    ///
    /// If not implemented for the chunk type, this will return a copy of the passed chunk.
    fn split(&self, _count: usize) -> Vec<Self> {
        vec![self.clone()]
    }

    /// Merge multiple chunks into one unified chunk.
    fn merge(chunks: &[Self]) -> Option<Self>;

    /// Size of the raw chunk data.
    fn size(&self) -> u32;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ChunkHeader {
    pub version: u16,
    pub chunk_type: ChunkType,
    pub compression_type: CompressionType,
    pub size: u32,
    pub target: Target,
}

impl ChunkHeader {
    pub fn from_chunk_kind(
        kind: &ChunkKind,
        compression_type: CompressionType,
        target: Target,
    ) -> Self {
        Self {
            version: kind.version(),
            chunk_type: kind.chunk_type(),
            compression_type,
            size: kind.size(),
            target,
        }
    }

    /// Encode the data using the header information.
    pub fn encode_data<'a>(&self, data: &'a [u8]) -> Option<Cow<'a, [u8]>> {
        match self.compression_type {
            CompressionType::None => Some(Cow::Borrowed(data)),
            CompressionType::Zstd => {
                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(data).ok()?;
                Some(Cow::Owned(encoder.finish().ok()?))
            }
            _ => None,
        }
    }

    /// Decode the data using the header information.
    pub fn decode_data<'a>(&self, data: &'a [u8]) -> Option<Cow<'a, [u8]>> {
        match self.compression_type {
            CompressionType::None => Some(Cow::Borrowed(data)),
            CompressionType::Zstd => {
                let mut decoder = ZlibDecoder::new(data);
                let mut decompressed = Vec::new();
                decoder.read_to_end(&mut decompressed).ok()?;
                Some(Cow::Owned(decompressed))
            }
            _ => None,
        }
    }
}

impl FlatBufferObject for ChunkHeader {
    type FbType<'fbb> = fb::ChunkHeader<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let target = self.target.create(builder);
        fb::ChunkHeader::create(
            builder,
            &fb::ChunkHeaderArgs {
                version: self.version,
                type_: self.chunk_type,
                compression_type: self.compression_type,
                size: self.size,
                target: Some(target),
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let header = Self {
            version: value.version(),
            chunk_type: value.type_(),
            compression_type: value.compression_type(),
            size: value.size(),
            target: value
                .target()
                .and_then(|t| Target::from_object(&t))
                .unwrap_or_default(),
        };

        Some(header)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChunkKind<'fbb> {
    Signature(SignatureChunk<'fbb>),
    Type(TypeChunk<'fbb>),
}

impl ChunkKind<'_> {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            ChunkKind::Signature(sc) => sc.as_ref(),
            ChunkKind::Type(tc) => tc.as_ref(),
        }
    }

    pub fn version(&self) -> u16 {
        match self {
            ChunkKind::Signature(_) => SignatureChunk::VERSION,
            ChunkKind::Type(_) => TypeChunk::VERSION,
        }
    }

    pub fn chunk_type(&self) -> ChunkType {
        match self {
            ChunkKind::Signature(_) => ChunkType::Signatures,
            ChunkKind::Type(_) => ChunkType::Types,
        }
    }

    pub fn size(&self) -> u32 {
        match self {
            ChunkKind::Signature(sc) => sc.size(),
            ChunkKind::Type(tc) => tc.size(),
        }
    }

    pub fn to_owned(&self) -> Self {
        match self {
            ChunkKind::Signature(sc) => ChunkKind::Signature(sc.to_owned()),
            ChunkKind::Type(tc) => ChunkKind::Type(tc.to_owned()),
        }
    }

    pub fn split(&self, count: usize) -> Vec<Self> {
        match self {
            ChunkKind::Signature(sc) => sc
                .split(count)
                .into_iter()
                .map(ChunkKind::Signature)
                .collect(),
            ChunkKind::Type(tc) => tc.split(count).into_iter().map(ChunkKind::Type).collect(),
        }
    }

    // TODO: This is kinda awful.
    pub fn merge(chunks: &[Self]) -> Vec<Self> {
        let signatures = chunks
            .iter()
            .filter_map(|c| match c {
                ChunkKind::Signature(sc) => Some(sc),
                _ => None,
            })
            .cloned()
            .collect::<Vec<_>>();

        let types = chunks
            .iter()
            .filter_map(|c| match c {
                ChunkKind::Type(tc) => Some(tc),
                _ => None,
            })
            .cloned()
            .collect::<Vec<_>>();

        let mut merged = Vec::new();
        if !signatures.is_empty() {
            if let Some(merged_signatures) = SignatureChunk::merge(&signatures) {
                merged.push(ChunkKind::Signature(merged_signatures));
            }
        }
        if !types.is_empty() {
            if let Some(merged_types) = TypeChunk::merge(&types) {
                merged.push(ChunkKind::Type(merged_types));
            }
        }
        merged
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Chunk<'fbb> {
    pub header: ChunkHeader,
    pub kind: ChunkKind<'fbb>,
}

impl<'fbb> Chunk<'fbb> {
    pub fn new(kind: ChunkKind<'fbb>, compression_type: CompressionType) -> Chunk<'fbb> {
        Self::new_with_target(kind, compression_type, Target::default())
    }

    pub fn new_with_target(
        kind: ChunkKind<'fbb>,
        compression_type: CompressionType,
        target: Target,
    ) -> Chunk<'fbb> {
        // We can pull the header from the chunk kind, as most of the data is derived.
        let header = ChunkHeader::from_chunk_kind(&kind, compression_type, target);
        Self::new_with_header(header, kind)
    }

    pub fn new_with_header(header: ChunkHeader, kind: ChunkKind<'fbb>) -> Chunk<'fbb> {
        Self { header, kind }
    }

    // TODO: Maybe some unnecessary clones here.
    pub fn merge(chunks: &[Self], compression_type: CompressionType) -> Vec<Self> {
        // Chunks with the same target are going to be grouped, returning a list of the merged chunks.
        chunks
            .iter()
            .into_group_map_by(|chunk| chunk.header.target.clone())
            .into_iter()
            .map(|(target, chunks)| {
                (
                    target,
                    chunks
                        .into_iter()
                        .map(|chunk| chunk.kind.clone())
                        .collect::<Vec<_>>(),
                )
            })
            .flat_map(|(target, chunk_kinds)| {
                ChunkKind::merge(&chunk_kinds)
                    .into_iter()
                    .map(move |merged_kind| {
                        Chunk::new_with_target(merged_kind, compression_type, target.clone())
                    })
            })
            .collect()
    }

    pub(crate) fn create_object(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> Option<WIPOffset<fb::Chunk<'fbb>>> {
        let header = self.header.create(builder);
        let encoded_data = self.header.encode_data(self.kind.as_bytes())?;
        let data = builder.create_vector(&encoded_data);

        Some(fb::Chunk::create(
            builder,
            &fb::ChunkArgs {
                header: Some(header),
                data: Some(data),
            },
        ))
    }

    pub(crate) fn from_object(value: &fb::Chunk<'fbb>) -> Option<Chunk<'fbb>> {
        let header = ChunkHeader::from_object(&value.header())?;
        let decoded_data = header.decode_data(value.data()?.bytes())?;
        let kind = match header.chunk_type {
            ChunkType::Signatures => ChunkKind::Signature(SignatureChunk::from_data(decoded_data)?),
            ChunkType::Types => ChunkKind::Type(TypeChunk::from_data(decoded_data)?),
            // The chunk type is unhandled.
            _ => return None,
        };

        Some(Self { header, kind })
    }

    // TODO: This duplicate code is awful.
    pub(crate) fn from_owned_object(value: &fb::Chunk) -> Option<Chunk<'fbb>> {
        let header = ChunkHeader::from_object(&value.header())?;
        let decoded_data = header.decode_data(value.data()?.bytes())?.to_vec();
        let kind = match header.chunk_type {
            ChunkType::Signatures => {
                ChunkKind::Signature(SignatureChunk::from_owned_data(decoded_data)?)
            }
            ChunkType::Types => ChunkKind::Type(TypeChunk::from_owned_data(decoded_data)?),
            // The chunk type is unhandled.
            _ => return None,
        };

        Some(Self { header, kind })
    }
}
