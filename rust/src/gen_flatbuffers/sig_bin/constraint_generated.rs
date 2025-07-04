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
pub enum ConstraintOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Constraint<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Constraint<'a> {
  type Inner = Constraint<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Constraint<'a> {
  pub const VT_GUID: flatbuffers::VOffsetT = 4;
  pub const VT_OFFSET: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Constraint { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args ConstraintArgs<'args>
  ) -> flatbuffers::WIPOffset<Constraint<'bldr>> {
    let mut builder = ConstraintBuilder::new(_fbb);
    builder.add_offset(args.offset);
    if let Some(x) = args.guid { builder.add_guid(x); }
    builder.finish()
  }


  #[inline]
  pub fn guid(&self) -> &'a ConstraintGUID {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<ConstraintGUID>(Constraint::VT_GUID, None).unwrap()}
  }
  #[inline]
  pub fn offset(&self) -> i64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i64>(Constraint::VT_OFFSET, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for Constraint<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<ConstraintGUID>("guid", Self::VT_GUID, true)?
     .visit_field::<i64>("offset", Self::VT_OFFSET, false)?
     .finish();
    Ok(())
  }
}
pub struct ConstraintArgs<'a> {
    pub guid: Option<&'a ConstraintGUID>,
    pub offset: i64,
}
impl<'a> Default for ConstraintArgs<'a> {
  #[inline]
  fn default() -> Self {
    ConstraintArgs {
      guid: None, // required field
      offset: 0,
    }
  }
}

pub struct ConstraintBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> ConstraintBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_guid(&mut self, guid: &ConstraintGUID) {
    self.fbb_.push_slot_always::<&ConstraintGUID>(Constraint::VT_GUID, guid);
  }
  #[inline]
  pub fn add_offset(&mut self, offset: i64) {
    self.fbb_.push_slot::<i64>(Constraint::VT_OFFSET, offset, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> ConstraintBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    ConstraintBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Constraint<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, Constraint::VT_GUID,"guid");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Constraint<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Constraint");
      ds.field("guid", &self.guid());
      ds.field("offset", &self.offset());
      ds.finish()
  }
}
