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
pub enum EnumerationOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Enumeration<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Enumeration<'a> {
  type Inner = Enumeration<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Enumeration<'a> {
  pub const VT_MEMBER_TYPE: flatbuffers::VOffsetT = 4;
  pub const VT_MEMBERS: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Enumeration { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args EnumerationArgs<'args>
  ) -> flatbuffers::WIPOffset<Enumeration<'bldr>> {
    let mut builder = EnumerationBuilder::new(_fbb);
    if let Some(x) = args.members { builder.add_members(x); }
    if let Some(x) = args.member_type { builder.add_member_type(x); }
    builder.finish()
  }


  #[inline]
  pub fn member_type(&self) -> Type<'a> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<Type>>(Enumeration::VT_MEMBER_TYPE, None).unwrap()}
  }
  #[inline]
  pub fn members(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<EnumerationMember<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<EnumerationMember>>>>(Enumeration::VT_MEMBERS, None)}
  }
}

impl flatbuffers::Verifiable for Enumeration<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<Type>>("member_type", Self::VT_MEMBER_TYPE, true)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<EnumerationMember>>>>("members", Self::VT_MEMBERS, false)?
     .finish();
    Ok(())
  }
}
pub struct EnumerationArgs<'a> {
    pub member_type: Option<flatbuffers::WIPOffset<Type<'a>>>,
    pub members: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<EnumerationMember<'a>>>>>,
}
impl<'a> Default for EnumerationArgs<'a> {
  #[inline]
  fn default() -> Self {
    EnumerationArgs {
      member_type: None, // required field
      members: None,
    }
  }
}

pub struct EnumerationBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> EnumerationBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_member_type(&mut self, member_type: flatbuffers::WIPOffset<Type<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Type>>(Enumeration::VT_MEMBER_TYPE, member_type);
  }
  #[inline]
  pub fn add_members(&mut self, members: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<EnumerationMember<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Enumeration::VT_MEMBERS, members);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> EnumerationBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    EnumerationBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Enumeration<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, Enumeration::VT_MEMBER_TYPE,"member_type");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Enumeration<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Enumeration");
      ds.field("member_type", &self.member_type());
      ds.field("members", &self.members());
      ds.finish()
  }
}
