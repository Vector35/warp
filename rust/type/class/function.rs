use crate::fb_type as fb;
use crate::r#type::Type;
use bon::{bon, Builder};
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use rand::distributions::{Alphanumeric, DistString, Distribution, Standard};
use rand::Rng;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RegisterLocation;

impl From<fb::RegisterLocation<'_>> for RegisterLocation {
    fn from(_value: fb::RegisterLocation<'_>) -> Self {
        Self
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StackLocation {
    offset: i64,
}

impl From<fb::StackLocation<'_>> for StackLocation {
    fn from(_value: fb::StackLocation<'_>) -> Self {
        Self {
            offset: _value.offset().into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Location {
    Register(RegisterLocation),
    Stack(StackLocation),
}

impl From<fb::FunctionMemberLocation<'_>> for Location {
    fn from(value: fb::FunctionMemberLocation<'_>) -> Self {
        match value.class_type() {
            fb::LocationClass::RegisterLocation => {
                let register_loc = value.class_as_register_location().unwrap();
                Self::Register(register_loc.into())
            }
            fb::LocationClass::StackLocation => {
                let stack_loc = value.class_as_stack_location().unwrap();
                Self::Stack(stack_loc.into())
            }
            _ => unreachable!(),
        }
    }
}

// TODO: Add Builder derive?
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionMember {
    pub name: Option<String>,
    pub ty: Type,
    // TODO: Technically optional.
    pub locations: Vec<Location>,
}

impl From<fb::FunctionMember<'_>> for FunctionMember {
    fn from(value: fb::FunctionMember<'_>) -> Self {
        Self {
            name: value.name().map(str::to_string),
            ty: value.type_().into(),
            locations: value
                .locations()
                .unwrap_or_default()
                .iter()
                .map(Into::into)
                .collect(),
        }
    }
}

#[bon]
impl FunctionMember {
    #[builder]
    pub fn new<T: Into<String>>(
        name: Option<T>,
        ty: Type,
        locations: Option<Vec<Location>>,
    ) -> Self {
        Self {
            name: name.map(|n| n.into()),
            ty,
            locations: locations.unwrap_or_default(),
        }
    }
}

impl FunctionMember {
    fn create<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<fb::FunctionMember<'a>> {
        let name = self.name.as_ref().map(|n| builder.create_string(n));
        let member_type = self.ty.create(builder);
        fb::FunctionMember::create(
            builder,
            &fb::FunctionMemberArgs {
                name,
                type_: Some(member_type),
                // TODO: Location.
                ..Default::default()
            },
        )
    }
}

impl Distribution<FunctionMember> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FunctionMember {
        FunctionMember {
            name: Some(Alphanumeric.sample_string(rng, 16)),
            ty: rng.gen(),
            // TODO: Generate locations randomly? I don't know...
            locations: vec![],
        }
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

    fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::CallingConvention<'a>> {
        let name = builder.create_string(&self.name);
        fb::CallingConvention::create(builder, &fb::CallingConventionArgs { name: Some(name) })
    }
}

impl From<fb::CallingConvention<'_>> for CallingConvention {
    fn from(_value: fb::CallingConvention<'_>) -> Self {
        Self {
            name: _value.name().unwrap().to_string(),
        }
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

    pub(crate) fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::Function<'a>> {
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
}

impl From<fb::Function<'_>> for FunctionClass {
    fn from(value: fb::Function<'_>) -> Self {
        Self {
            calling_convention: value.calling_convention().map(Into::into),
            in_members: value.in_members().unwrap().iter().map(Into::into).collect(),
            out_members: value
                .out_members()
                .unwrap()
                .iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl Distribution<FunctionClass> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FunctionClass {
        let rand_in_member_len = rng.gen_range(0..8);
        // TODO: Generate functions with multiple out?
        let rand_out_member_len = rng.gen_range(0..1);
        FunctionClass {
            calling_convention: None,
            in_members: rng.sample_iter(Standard).take(rand_in_member_len).collect(),
            out_members: rng
                .sample_iter(Standard)
                .take(rand_out_member_len)
                .collect(),
        }
    }
}
