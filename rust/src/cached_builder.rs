//! A space-optimized flatbuffer builder.
//!
//! By space-optimized we mean we provide records of previously constructed buffer objects to re-use.
//! Currently, this is done for the Type table. However, this could be done in the future for other things.

use crate::fb_type as fb;
use crate::r#type::Type;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

/// Caches entries that are common to reduce final size of buffer.
pub struct CachedFlatBufferBuilder<'fbb> {
    pub builder: FlatBufferBuilder<'fbb>,
    /// A cache for deduplicating offsets for `Type` objects.
    pub cached_type_offsets: HashMap<Type, WIPOffset<fb::Type<'fbb>>>,
}

impl<'fbb> CachedFlatBufferBuilder<'fbb> {
    pub fn new() -> Self {
        Self::new_with_builder(FlatBufferBuilder::new())
    }

    /// Creates a new `CachedBufferBuilder` instance, wrapping the provided FlatBufferBuilder.
    pub fn new_with_builder(builder: FlatBufferBuilder<'fbb>) -> Self {
        Self {
            builder,
            cached_type_offsets: HashMap::new(),
        }
    }
}

impl Default for CachedFlatBufferBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'fbb> Deref for CachedFlatBufferBuilder<'fbb> {
    type Target = FlatBufferBuilder<'fbb>;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl DerefMut for CachedFlatBufferBuilder<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.builder
    }
}
