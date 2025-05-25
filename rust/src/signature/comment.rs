use crate::cached_builder::CachedFlatBufferBuilder;
use crate::fb_sig as fb;
use crate::FlatBufferObject;
use flatbuffers::WIPOffset;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct FunctionComment {
    pub offset: i64,
    pub text: String,
}

impl FlatBufferObject for FunctionComment {
    type FbType<'fbb> = fb::FunctionComment<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let text = builder.create_string(&self.text);
        fb::FunctionComment::create(
            builder,
            &fb::FunctionCommentArgs {
                offset: self.offset,
                text: Some(text),
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        Some(Self {
            offset: value.offset(),
            text: value.text().to_string(),
        })
    }
}
