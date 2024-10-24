use crate::fb_sig as fb;
use crate::signature::function::FunctionGUID;
use crate::symbol::Symbol;
use flatbuffers::{FlatBufferBuilder, WIPOffset};
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct FunctionConstraint {
    pub guid: Option<FunctionGUID>,
    pub symbol: Option<Symbol>,
    pub offset: i64,
}

impl FunctionConstraint {
    pub fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::FunctionConstraint<'a>> {
        let guid = self
            .guid
            .map(|guid| builder.create_string(&guid.to_string()));
        let symbol = self.symbol.as_ref().map(|symbol| symbol.create(builder));
        fb::FunctionConstraint::create(
            builder,
            &fb::FunctionConstraintArgs {
                guid,
                symbol,
                offset: self.offset,
            },
        )
    }
}

impl From<fb::FunctionConstraint<'_>> for FunctionConstraint {
    fn from(value: fb::FunctionConstraint<'_>) -> Self {
        let guid = value
            .guid()
            .map(|guid| guid.parse::<FunctionGUID>().unwrap());
        Self {
            guid,
            symbol: value.symbol().map(Symbol::from),
            offset: value.offset(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct FunctionConstraints {
    pub adjacent: HashSet<FunctionConstraint>,
    pub call_sites: HashSet<FunctionConstraint>,
    pub caller_sites: HashSet<FunctionConstraint>,
}

impl FunctionConstraints {
    pub fn create<'a>(
        &self,
        builder: &mut FlatBufferBuilder<'a>,
    ) -> WIPOffset<fb::FunctionConstraints<'a>> {
        let _adjacent: Vec<_> = self
            .adjacent
            .iter()
            .map(|constraint| constraint.create(builder))
            .collect();
        let adjacent = if _adjacent.is_empty() {
            None
        } else {
            Some(builder.create_vector(&_adjacent))
        };

        let _call_sites: Vec<_> = self
            .call_sites
            .iter()
            .map(|constraint| constraint.create(builder))
            .collect();
        let call_sites = if _call_sites.is_empty() {
            None
        } else {
            Some(builder.create_vector(&_call_sites))
        };

        let _caller_sites: Vec<_> = self
            .caller_sites
            .iter()
            .map(|constraint| constraint.create(builder))
            .collect();
        let caller_sites = if _caller_sites.is_empty() {
            None
        } else {
            Some(builder.create_vector(&_caller_sites))
        };

        fb::FunctionConstraints::create(
            builder,
            &fb::FunctionConstraintsArgs {
                adjacent,
                call_sites,
                caller_sites,
            },
        )
    }
}

impl From<fb::FunctionConstraints<'_>> for FunctionConstraints {
    fn from(value: fb::FunctionConstraints<'_>) -> Self {
        let adjacent: HashSet<FunctionConstraint> = value
            .adjacent()
            .unwrap_or_default()
            .iter()
            .map(|constraint| constraint.into())
            .collect();
        let call_sites: HashSet<FunctionConstraint> = value
            .call_sites()
            .unwrap_or_default()
            .iter()
            .map(|constraint| constraint.into())
            .collect();
        let caller_sites: HashSet<FunctionConstraint> = value
            .caller_sites()
            .unwrap_or_default()
            .iter()
            .map(|constraint| constraint.into())
            .collect();

        Self {
            adjacent,
            call_sites,
            caller_sites,
        }
    }
}
