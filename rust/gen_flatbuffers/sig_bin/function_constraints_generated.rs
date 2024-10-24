// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum FunctionConstraintsOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct FunctionConstraints<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for FunctionConstraints<'a> {
  type Inner = FunctionConstraints<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> FunctionConstraints<'a> {
  pub const VT_ADJACENT: flatbuffers::VOffsetT = 4;
  pub const VT_CALL_SITES: flatbuffers::VOffsetT = 6;
  pub const VT_CALLER_SITES: flatbuffers::VOffsetT = 8;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    FunctionConstraints { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args FunctionConstraintsArgs<'args>
  ) -> flatbuffers::WIPOffset<FunctionConstraints<'bldr>> {
    let mut builder = FunctionConstraintsBuilder::new(_fbb);
    if let Some(x) = args.caller_sites { builder.add_caller_sites(x); }
    if let Some(x) = args.call_sites { builder.add_call_sites(x); }
    if let Some(x) = args.adjacent { builder.add_adjacent(x); }
    builder.finish()
  }


  #[inline]
  pub fn adjacent(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<FunctionConstraint<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<FunctionConstraint>>>>(FunctionConstraints::VT_ADJACENT, None)}
  }
  #[inline]
  pub fn call_sites(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<FunctionConstraint<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<FunctionConstraint>>>>(FunctionConstraints::VT_CALL_SITES, None)}
  }
  #[inline]
  pub fn caller_sites(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<FunctionConstraint<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<FunctionConstraint>>>>(FunctionConstraints::VT_CALLER_SITES, None)}
  }
}

impl flatbuffers::Verifiable for FunctionConstraints<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<FunctionConstraint>>>>("adjacent", Self::VT_ADJACENT, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<FunctionConstraint>>>>("call_sites", Self::VT_CALL_SITES, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<FunctionConstraint>>>>("caller_sites", Self::VT_CALLER_SITES, false)?
     .finish();
    Ok(())
  }
}
pub struct FunctionConstraintsArgs<'a> {
    pub adjacent: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<FunctionConstraint<'a>>>>>,
    pub call_sites: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<FunctionConstraint<'a>>>>>,
    pub caller_sites: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<FunctionConstraint<'a>>>>>,
}
impl<'a> Default for FunctionConstraintsArgs<'a> {
  #[inline]
  fn default() -> Self {
    FunctionConstraintsArgs {
      adjacent: None,
      call_sites: None,
      caller_sites: None,
    }
  }
}

pub struct FunctionConstraintsBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> FunctionConstraintsBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_adjacent(&mut self, adjacent: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<FunctionConstraint<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(FunctionConstraints::VT_ADJACENT, adjacent);
  }
  #[inline]
  pub fn add_call_sites(&mut self, call_sites: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<FunctionConstraint<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(FunctionConstraints::VT_CALL_SITES, call_sites);
  }
  #[inline]
  pub fn add_caller_sites(&mut self, caller_sites: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<FunctionConstraint<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(FunctionConstraints::VT_CALLER_SITES, caller_sites);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> FunctionConstraintsBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    FunctionConstraintsBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<FunctionConstraints<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for FunctionConstraints<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("FunctionConstraints");
      ds.field("adjacent", &self.adjacent());
      ds.field("call_sites", &self.call_sites());
      ds.field("caller_sites", &self.caller_sites());
      ds.finish()
  }
}
