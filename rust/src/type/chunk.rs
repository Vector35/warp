use crate::cached_builder::CachedFlatBufferBuilder;
use crate::chunk::ChunkHandler;
use crate::fb_type as fb;
use crate::r#type::guid::TypeGUID;
use crate::r#type::{ComputedType, Type};
use crate::FlatBufferObject;
use flatbuffers::VerifierOptions;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Clone, PartialEq)]
pub struct TypeChunk<'fbb> {
    buffer: Cow<'fbb, [u8]>,
    lookup: HashMap<TypeGUID, usize>,
}

impl<'fbb> TypeChunk<'fbb> {
    /// Construct a [`TypeChunk`] using the provided [`CachedFlatBufferBuilder`] and [`Type`] list.
    pub fn new(types: &[Type]) -> Option<TypeChunk<'fbb>> {
        let computed_types: Vec<ComputedType> =
            types.iter().cloned().map(ComputedType::new).collect();
        Self::new_with_computed(&computed_types)
    }

    /// Construct a [`TypeChunk`] using the provided [`CachedFlatBufferBuilder`] and [`ComputedType`] list.
    pub fn new_with_computed(types: &[ComputedType]) -> Option<TypeChunk<'fbb>> {
        let mut builder = CachedFlatBufferBuilder::new();
        // Build the new flatbuffer signature chunk from the given functions.
        let _types: Vec<_> = types.iter().map(|f| f.create(&mut builder)).collect();
        let types = builder.create_vector(&_types);
        let chunk = fb::TypeChunk::create(&mut builder, &fb::TypeChunkArgs { types: Some(types) });

        // Round-trip the flatbuffer signature chunk back to a TypeChunk.
        // NOTE: The returned TypeChunk does not own the underlying buffer, we clone it to avoid annoying lifetime issues.
        builder.finish_minimal(chunk);
        let chunk_data = builder.finished_data().to_vec();
        TypeChunk::from_data(Cow::Owned(chunk_data))
    }

    /// Builds the lookup table for type guids.
    ///
    /// NOTE: Called when constructing the chunk in [`TypeChunk::from_data`].
    fn build_lookup(&mut self) {
        self.lookup = self
            .raw_types()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (idx, ty)| {
                let guid = TypeGUID::from(*ty.guid());
                acc.insert(guid, idx);
                acc
            });
    }

    fn get_raw_type(&self, idx: usize) -> Option<fb::ComputedType> {
        if idx >= self.object().types().len() {
            return None;
        }
        Some(self.object().types().get(idx))
    }

    pub fn object(&self) -> fb::TypeChunk {
        // SAFETY: We have already verified the buffer.
        unsafe { fb::root_as_type_chunk_unchecked(&self.buffer) }
    }

    pub fn raw_types(&self) -> impl Iterator<Item = fb::ComputedType<'_>> + '_ {
        self.object().types().iter()
    }

    /// Retrieve all types as an owned [`ComputedType`].
    ///
    /// This function should not be used in hot paths, instead use [`TypeChunk::raw_types`]
    /// which you should then filter types off of before creating an owned [`Type`]. An example
    /// of a function that does this is [`TypeChunk::type_with_guid`].
    pub fn types(&self) -> impl Iterator<Item = ComputedType> + '_ {
        self.raw_types()
            .flat_map(|ct| ComputedType::from_object(&ct))
    }

    #[must_use]
    pub fn raw_type_with_guid(&self, guid: &TypeGUID) -> Option<fb::Type> {
        self.lookup
            .get(guid)
            .and_then(|idx| self.get_raw_type(*idx))
            .and_then(|ct| ct.type_())
    }

    #[must_use]
    pub fn type_with_guid(&self, guid: &TypeGUID) -> Option<Type> {
        self.raw_type_with_guid(guid)
            .as_ref()
            .and_then(Type::from_object)
    }

    #[must_use]
    pub fn raw_type_with_name(&self, name: &str) -> Vec<fb::ComputedType> {
        self.raw_types()
            .filter_map(|ct| ct.type_().filter(|ty| ty.name() == Some(name)).map(|_| ct))
            .collect()
    }

    #[must_use]
    pub fn type_with_name(&self, name: &str) -> Vec<ComputedType> {
        self.raw_type_with_name(name)
            .iter()
            .flat_map(ComputedType::from_object)
            .collect()
    }
}

impl Debug for TypeChunk<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let types: Vec<_> = self.types().collect();
        f.debug_struct("TypeChunk").field("types", &types).finish()
    }
}

impl<'fbb> ChunkHandler<'fbb> for TypeChunk<'fbb> {
    const VERSION: u16 = 0;

    fn from_data(data: Cow<'fbb, [u8]>) -> Option<Self> {
        // We must verify the buffer before we can create a TypeChunk.
        // See TypeChunk::object for more details.
        let verify_opts = VerifierOptions {
            max_tables: 10_000_000,
            ..Default::default()
        };
        let _verified_object = fb::root_as_type_chunk_with_opts(&verify_opts, &data).ok()?;
        let mut chunk = Self {
            buffer: data,
            lookup: HashMap::new(),
        };
        chunk.build_lookup();
        Some(chunk)
    }

    fn merge(chunks: &[Self]) -> Option<Self> {
        let mut types: Vec<_> = chunks.iter().flat_map(|c| c.types()).collect();
        // Sort by the GUID, then remove duplicates by GUID.
        types.sort_unstable_by(|a, b| a.guid.cmp(&b.guid));
        types.dedup_by_key(|ty| ty.guid);
        Self::new_with_computed(&types)
    }

    fn size(&self) -> u32 {
        self.buffer.len() as u32
    }
}

impl AsRef<[u8]> for TypeChunk<'_> {
    fn as_ref(&self) -> &[u8] {
        &self.buffer
    }
}
