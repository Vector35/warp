use crate::cached_builder::CachedFlatBufferBuilder;
use crate::fb_target as fb;
use crate::FlatBufferObject;
use flatbuffers::WIPOffset;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Target {
    pub architecture: Option<String>,
    pub platform: Option<String>,
}

impl FlatBufferObject for Target {
    type FbType<'fbb> = fb::Target<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let architecture = self.architecture.clone().map(|n| builder.create_string(&n));
        let platform = self.platform.clone().map(|n| builder.create_string(&n));
        fb::Target::create(
            builder,
            &fb::TargetArgs {
                architecture,
                platform,
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        Some(Self {
            architecture: value.architecture().map(|n| n.to_string()),
            platform: value.platform().map(|n| n.to_string()),
        })
    }
}
