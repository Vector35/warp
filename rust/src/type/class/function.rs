use crate::cached_builder::CachedFlatBufferBuilder;
use crate::r#type::Type;
use crate::{fb_type as fb, FlatBufferObject, FlatBufferUnion};
use bon::{bon, Builder};
use flatbuffers::{UnionWIPOffset, WIPOffset};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RegisterLocation {
    pub id: u64,
}

impl FlatBufferObject for RegisterLocation {
    type FbType<'fbb> = fb::RegisterLocation<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        fb::RegisterLocation::create(builder, &fb::RegisterLocationArgs { id: self.id })
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        Some(Self { id: value.id() })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StackLocation {
    /// Offset from the stack pointer, in bits.
    pub offset: i64,
}

impl FlatBufferObject for StackLocation {
    type FbType<'fbb> = fb::StackLocation<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        fb::StackLocation::create(
            builder,
            &fb::StackLocationArgs {
                offset: Some(&self.offset.into()),
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        Some(Self {
            offset: value.offset().into(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Location {
    Register(RegisterLocation),
    Stack(StackLocation),
}

impl Location {
    pub fn ty(&self) -> fb::LocationClass {
        match self {
            Location::Register(_) => fb::LocationClass::RegisterLocation,
            Location::Stack(_) => fb::LocationClass::StackLocation,
        }
    }
}

impl FlatBufferUnion for Location {
    fn create(&self, builder: &mut CachedFlatBufferBuilder) -> WIPOffset<UnionWIPOffset> {
        match self {
            Location::Register(loc) => loc.create(builder).as_union_value(),
            Location::Stack(loc) => loc.create(builder).as_union_value(),
        }
    }
}

// TODO: Add Builder derive?
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionMember {
    pub name: Option<String>,
    pub ty: Box<Type>,
    pub location: Option<Location>,
}

#[bon]
impl FunctionMember {
    #[builder]
    pub fn new<T: Into<String>>(name: Option<T>, ty: Type, location: Option<Location>) -> Self {
        Self {
            name: name.map(|n| n.into()),
            ty: Box::new(ty),
            location,
        }
    }
}

impl FlatBufferObject for FunctionMember {
    type FbType<'fbb> = fb::FunctionMember<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let name = self.name.as_ref().map(|n| builder.create_string(n));
        let member_type = self.ty.create(builder);
        let location_type = self.location.as_ref().map(|l| l.ty()).unwrap_or_default();
        let location = self.location.as_ref().map(|l| l.create(builder));
        fb::FunctionMember::create(
            builder,
            &fb::FunctionMemberArgs {
                name,
                type_: Some(member_type),
                location_type,
                location,
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let location = match value.location_type() {
            fb::LocationClass::RegisterLocation => {
                let register_loc = value.location_as_register_location()?;
                Some(Location::Register(RegisterLocation::from_object(
                    &register_loc,
                )?))
            }
            fb::LocationClass::StackLocation => {
                let stack_loc = value.location_as_stack_location()?;
                Some(Location::Stack(StackLocation::from_object(&stack_loc)?))
            }
            _ => None,
        };

        let class = Self {
            name: value.name().map(str::to_string),
            ty: Box::new(Type::from_object(&value.type_())?),
            location,
        };

        Some(class)
    }
}

// TODO: Move calling convention to another crate?
// TODO: It might have a seperate store... we want to refer to the uuid, but that means we have to do whole
// TODO: database updates when the calling convention changes.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct CallingConvention {
    pub name: String,
}

impl CallingConvention {
    // Currently we only support calling convention by name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl FlatBufferObject for CallingConvention {
    type FbType<'fbb> = fb::CallingConvention<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let name = builder.create_string(&self.name);
        fb::CallingConvention::create(builder, &fb::CallingConventionArgs { name: Some(name) })
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        Some(Self {
            name: value.name()?.to_string(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Builder)]
pub struct FunctionClass {
    pub calling_convention: Option<CallingConvention>,
    pub in_members: Vec<FunctionMember>,
    pub out_members: Vec<FunctionMember>,
}

impl FunctionClass {
    pub fn new(
        calling_convention: Option<CallingConvention>,
        in_members: Vec<FunctionMember>,
        out_members: Vec<FunctionMember>,
    ) -> Self {
        Self {
            calling_convention,
            in_members,
            out_members,
        }
    }
}

impl FlatBufferObject for FunctionClass {
    type FbType<'fbb> = fb::Function<'fbb>;

    fn create<'fbb>(
        &self,
        builder: &mut CachedFlatBufferBuilder<'fbb>,
    ) -> WIPOffset<Self::FbType<'fbb>> {
        let calling_convention = self
            .calling_convention
            .as_ref()
            .map(|cc| cc.create(builder));
        let _created_in_members: Vec<_> = self
            .in_members
            .iter()
            .map(|member| member.create(builder))
            .collect();
        let created_in_members = builder.create_vector(&_created_in_members);
        let _created_out_members: Vec<_> = self
            .out_members
            .iter()
            .map(|member| member.create(builder))
            .collect();
        let created_out_members = builder.create_vector(&_created_out_members);

        fb::Function::create(
            builder,
            &fb::FunctionArgs {
                calling_convention,
                in_members: Some(created_in_members),
                out_members: Some(created_out_members),
            },
        )
    }

    fn from_object(value: &Self::FbType<'_>) -> Option<Self> {
        let class = Self {
            calling_convention: value
                .calling_convention()
                .and_then(|cc| FlatBufferObject::from_object(&cc)),
            in_members: value
                .in_members()
                .unwrap_or_default()
                .iter()
                .flat_map(|member| FlatBufferObject::from_object(&member))
                .collect(),
            out_members: value
                .out_members()
                .unwrap_or_default()
                .iter()
                .flat_map(|member| FlatBufferObject::from_object(&member))
                .collect(),
        };

        Some(class)
    }
}
