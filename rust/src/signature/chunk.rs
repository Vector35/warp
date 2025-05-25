use crate::cached_builder::CachedFlatBufferBuilder;
use crate::chunk::ChunkHandler;
use crate::fb_sig as fb;
use crate::signature::function::{Function, FunctionGUID};
use crate::FlatBufferObject;
use flatbuffers::VerifierOptions;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;
// TODO: Rename 'fbb to 'a the lifetime is no longer tied to the flatbuffer builder.

#[derive(Clone, PartialEq)]
pub struct SignatureChunk<'fbb> {
    buffer: Cow<'fbb, [u8]>,
    lookup: HashMap<FunctionGUID, Vec<usize>>,
}

impl<'fbb> SignatureChunk<'fbb> {
    /// Construct a [`SignatureChunk`] using the provided [`CachedFlatBufferBuilder`] and [`Function`] list.
    pub fn new(functions: &[Function]) -> Option<SignatureChunk<'fbb>> {
        let mut builder = CachedFlatBufferBuilder::new();
        // Build the new flatbuffer signature chunk from the given functions.
        let _functions: Vec<_> = functions.iter().map(|f| f.create(&mut builder)).collect();
        let functions = builder.create_vector(&_functions);
        let chunk = fb::SignatureChunk::create(
            &mut builder,
            &fb::SignatureChunkArgs {
                functions: Some(functions),
            },
        );

        // Round-trip the flatbuffer signature chunk back to a SignatureChunk.
        // NOTE: The returned SignatureChunk does not own the underlying buffer, we clone it to avoid annoying lifetime issues.
        builder.finish_minimal(chunk);
        let chunk_data = builder.finished_data().to_vec();
        SignatureChunk::from_data(Cow::Owned(chunk_data))
    }

    /// Builds the lookup table for function guids.
    ///
    /// NOTE: Called when constructing the chunk in [`SignatureChunk::from_data`].
    fn build_lookup(&mut self) {
        self.lookup = self
            .raw_functions()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (idx, f)| {
                let guid = FunctionGUID::from(*f.guid());
                acc.entry(guid).or_default().push(idx);
                acc
            });
    }

    fn get_raw_function(&self, idx: usize) -> Option<fb::Function> {
        if idx >= self.object().functions().len() {
            return None;
        }
        Some(self.object().functions().get(idx))
    }

    pub fn object(&self) -> fb::SignatureChunk {
        // SAFETY: We have already verified the buffer.
        unsafe { fb::root_as_signature_chunk_unchecked(&self.buffer) }
    }

    pub fn raw_functions(&self) -> impl Iterator<Item = fb::Function<'_>> + '_ {
        self.object().functions().iter()
    }

    /// Retrieve all functions as an owned [`Function`].
    ///
    /// This function should not be used in hot paths, instead use [`SignatureChunk::raw_functions`]
    /// which you should then filter functions off of before creating an owned [`Function`]. An example
    /// of a function that does this is [`SignatureChunk::functions_with_guid`].
    pub fn functions(&self) -> impl Iterator<Item = Function> + '_ {
        self.raw_functions().flat_map(|f| Function::from_object(&f))
    }

    pub fn raw_functions_with_guid<'a>(
        &'a self,
        guid: &FunctionGUID,
    ) -> impl Iterator<Item = fb::Function<'a>> + 'a {
        self.lookup
            .get(guid)
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|idx| self.get_raw_function(idx))
    }

    pub fn functions_with_guid<'a>(
        &'a self,
        guid: &FunctionGUID,
    ) -> impl Iterator<Item = Function> + 'a {
        self.raw_functions_with_guid(guid)
            .flat_map(|f| Function::from_object(&f))
    }

    pub fn from_owned_data(data: Vec<u8>) -> Option<Self> {
        Self::from_data(Cow::Owned(data))
    }

    // TODO: Other types of lookups: by symbol, by constraints, by type?
}

impl Debug for SignatureChunk<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let functions: Vec<_> = self.functions().collect();
        f.debug_struct("SignatureChunk")
            .field("functions", &functions)
            .finish()
    }
}

impl<'fbb> ChunkHandler<'fbb> for SignatureChunk<'fbb> {
    const VERSION: u16 = 0;

    fn from_data(data: Cow<'fbb, [u8]>) -> Option<Self> {
        // We must verify the buffer before we can create a SignatureChunk.
        // See SignatureChunk::object for more details.
        let verify_opts = VerifierOptions {
            max_tables: 10_000_000,
            ..Default::default()
        };
        let _verified_object = fb::root_as_signature_chunk_with_opts(&verify_opts, &data).ok()?;
        let mut chunk = Self {
            buffer: data,
            lookup: HashMap::new(),
        };
        chunk.build_lookup();
        Some(chunk)
    }

    fn merge(chunks: &[Self]) -> Option<Self> {
        let mut functions: Vec<_> = chunks.iter().flat_map(|c| c.functions()).collect();
        // First sort by name, this will get us in the "perceived" order, useful for users.
        // Second sort by guid, this will get us in the "dedupe" order, useful for deduplicating.
        functions.sort_unstable_by(|a, b| {
            a.symbol
                .name
                .cmp(&b.symbol.name)
                .then_with(|| a.guid.cmp(&b.guid))
        });
        functions.dedup_by(|a, b| {
            // Different guid, we want to keep both.
            if a.guid != b.guid || a.symbol.name != b.symbol.name {
                return false;
            }

            // Keep constraints from both functions.
            // We assume this is the same function if they share the same name but different constraints.
            // We assume the function is different if they both have a type and it differs.
            match (&a.ty, &b.ty) {
                (Some(a_ty), None) => {
                    // Copy over a's type as well, since b has no type.
                    b.ty = Some(a_ty.clone());
                    b.constraints.extend(a.constraints.clone());
                    true
                }
                (Some(a_ty), Some(b_ty)) => {
                    if a_ty == b_ty {
                        // Same type - merge constraints and deduplicate
                        b.constraints.extend(a.constraints.clone());
                        true
                    } else {
                        false
                    }
                }
                (None, Some(_)) | (None, None) => {
                    b.constraints.extend(a.constraints.clone());
                    true
                }
            }
        });
        Self::new(&functions)
    }

    fn size(&self) -> u32 {
        self.buffer.len() as u32
    }
}

impl AsRef<[u8]> for SignatureChunk<'_> {
    fn as_ref(&self) -> &[u8] {
        &self.buffer
    }
}
