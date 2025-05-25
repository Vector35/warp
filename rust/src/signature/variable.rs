use crate::cached_builder::CachedFlatBufferBuilder;
use crate::fb_sig as fb;
use crate::gen_flatbuffers::type_bin::LocationClass;
use crate::r#type::class::function::{Location, RegisterLocation, StackLocation};
use crate::r#type::Type;
use crate::{FlatBufferObject, FlatBufferUnion};
use flatbuffers::WIPOffset;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionVariable {
    pub offset: i64,
    pub location: Location,
    pub name: Option<String>,
    pub ty: Option<Type>,
}

impl FlatBufferObject for FunctionVariable {
    type FbType<'fbb> = fb::FunctionVariable<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let location_type = self.location.ty();
        let location = self.location.create(builder);
        let ty = self.ty.as_ref().map(|t| t.create(builder));
        let name = self.name.as_ref().map(|n| builder.create_string(n));
        fb::FunctionVariable::create(
            builder,
            &fb::FunctionVariableArgs {
                offset: self.offset,
                name,
                location_type,
                location: Some(location),
                type_: ty,
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let location = match value.location_type() {
            LocationClass::RegisterLocation => {
                let register_loc = value.location_as_register_location()?;
                Location::Register(RegisterLocation::from_object(&register_loc)?)
            }
            LocationClass::StackLocation => {
                let stack_loc = value.location_as_stack_location()?;
                Location::Stack(StackLocation::from_object(&stack_loc)?)
            }
            // NOTE: Unknown location class, probable malformed data.
            _ => return None,
        };

        let ty = value.type_().and_then(|ty| Type::from_object(&ty));
        Some(Self {
            offset: value.offset(),
            location,
            name: value.name().map(|s| s.to_string()),
            ty,
        })
    }
}
