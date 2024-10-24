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
pub enum StructureMemberOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct StructureMember<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for StructureMember<'a> {
  type Inner = StructureMember<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> StructureMember<'a> {
  pub const VT_NAME: flatbuffers::VOffsetT = 4;
  pub const VT_OFFSET: flatbuffers::VOffsetT = 6;
  pub const VT_TYPE_: flatbuffers::VOffsetT = 8;
  pub const VT_MODIFIERS: flatbuffers::VOffsetT = 10;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    StructureMember { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args StructureMemberArgs<'args>
  ) -> flatbuffers::WIPOffset<StructureMember<'bldr>> {
    let mut builder = StructureMemberBuilder::new(_fbb);
    if let Some(x) = args.type_ { builder.add_type_(x); }
    if let Some(x) = args.offset { builder.add_offset(x); }
    if let Some(x) = args.name { builder.add_name(x); }
    builder.add_modifiers(args.modifiers);
    builder.finish()
  }


  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(StructureMember::VT_NAME, None)}
  }
  #[inline]
  pub fn offset(&self) -> &'a UnsignedBitOffset {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<UnsignedBitOffset>(StructureMember::VT_OFFSET, None).unwrap()}
  }
  #[inline]
  pub fn type_(&self) -> Type<'a> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<Type>>(StructureMember::VT_TYPE_, None).unwrap()}
  }
  #[inline]
  pub fn modifiers(&self) -> StructureMemberModifiers {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<StructureMemberModifiers>(StructureMember::VT_MODIFIERS, Some(Default::default())).unwrap()}
  }
}

impl flatbuffers::Verifiable for StructureMember<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .visit_field::<UnsignedBitOffset>("offset", Self::VT_OFFSET, true)?
     .visit_field::<flatbuffers::ForwardsUOffset<Type>>("type_", Self::VT_TYPE_, true)?
     .visit_field::<StructureMemberModifiers>("modifiers", Self::VT_MODIFIERS, false)?
     .finish();
    Ok(())
  }
}
pub struct StructureMemberArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub offset: Option<&'a UnsignedBitOffset>,
    pub type_: Option<flatbuffers::WIPOffset<Type<'a>>>,
    pub modifiers: StructureMemberModifiers,
}
impl<'a> Default for StructureMemberArgs<'a> {
  #[inline]
  fn default() -> Self {
    StructureMemberArgs {
      name: None,
      offset: None, // required field
      type_: None, // required field
      modifiers: Default::default(),
    }
  }
}

pub struct StructureMemberBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> StructureMemberBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(StructureMember::VT_NAME, name);
  }
  #[inline]
  pub fn add_offset(&mut self, offset: &UnsignedBitOffset) {
    self.fbb_.push_slot_always::<&UnsignedBitOffset>(StructureMember::VT_OFFSET, offset);
  }
  #[inline]
  pub fn add_type_(&mut self, type_: flatbuffers::WIPOffset<Type<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Type>>(StructureMember::VT_TYPE_, type_);
  }
  #[inline]
  pub fn add_modifiers(&mut self, modifiers: StructureMemberModifiers) {
    self.fbb_.push_slot::<StructureMemberModifiers>(StructureMember::VT_MODIFIERS, modifiers, Default::default());
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> StructureMemberBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    StructureMemberBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<StructureMember<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, StructureMember::VT_OFFSET,"offset");
    self.fbb_.required(o, StructureMember::VT_TYPE_,"type_");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for StructureMember<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("StructureMember");
      ds.field("name", &self.name());
      ds.field("offset", &self.offset());
      ds.field("type_", &self.type_());
      ds.field("modifiers", &self.modifiers());
      ds.finish()
  }
}