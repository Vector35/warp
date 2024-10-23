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
pub enum PointerOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Pointer<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Pointer<'a> {
  type Inner = Pointer<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Pointer<'a> {
  pub const VT_WIDTH: flatbuffers::VOffsetT = 4;
  pub const VT_SHIFT: flatbuffers::VOffsetT = 6;
  pub const VT_CHILD: flatbuffers::VOffsetT = 8;
  pub const VT_ADDRESSING: flatbuffers::VOffsetT = 10;
  pub const VT_OFFSET: flatbuffers::VOffsetT = 12;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Pointer { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args PointerArgs<'args>
  ) -> flatbuffers::WIPOffset<Pointer<'bldr>> {
    let mut builder = PointerBuilder::new(_fbb);
    if let Some(x) = args.offset { builder.add_offset(x); }
    if let Some(x) = args.child { builder.add_child(x); }
    if let Some(x) = args.shift { builder.add_shift(x); }
    if let Some(x) = args.width { builder.add_width(x); }
    builder.add_addressing(args.addressing);
    builder.finish()
  }


  #[inline]
  pub fn width(&self) -> Option<&'a BitWidth> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<BitWidth>(Pointer::VT_WIDTH, None)}
  }
  #[inline]
  pub fn shift(&self) -> Option<&'a BitShift> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<BitShift>(Pointer::VT_SHIFT, None)}
  }
  #[inline]
  pub fn child(&self) -> Option<Type<'a>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<Type>>(Pointer::VT_CHILD, None)}
  }
  #[inline]
  pub fn addressing(&self) -> PointerAddressing {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<PointerAddressing>(Pointer::VT_ADDRESSING, Some(PointerAddressing::Absolute)).unwrap()}
  }
  #[inline]
  pub fn offset(&self) -> Option<&'a BitOffset> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<BitOffset>(Pointer::VT_OFFSET, None)}
  }
}

impl flatbuffers::Verifiable for Pointer<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<BitWidth>("width", Self::VT_WIDTH, false)?
     .visit_field::<BitShift>("shift", Self::VT_SHIFT, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<Type>>("child", Self::VT_CHILD, false)?
     .visit_field::<PointerAddressing>("addressing", Self::VT_ADDRESSING, false)?
     .visit_field::<BitOffset>("offset", Self::VT_OFFSET, false)?
     .finish();
    Ok(())
  }
}
pub struct PointerArgs<'a> {
    pub width: Option<&'a BitWidth>,
    pub shift: Option<&'a BitShift>,
    pub child: Option<flatbuffers::WIPOffset<Type<'a>>>,
    pub addressing: PointerAddressing,
    pub offset: Option<&'a BitOffset>,
}
impl<'a> Default for PointerArgs<'a> {
  #[inline]
  fn default() -> Self {
    PointerArgs {
      width: None,
      shift: None,
      child: None,
      addressing: PointerAddressing::Absolute,
      offset: None,
    }
  }
}

pub struct PointerBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> PointerBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_width(&mut self, width: &BitWidth) {
    self.fbb_.push_slot_always::<&BitWidth>(Pointer::VT_WIDTH, width);
  }
  #[inline]
  pub fn add_shift(&mut self, shift: &BitShift) {
    self.fbb_.push_slot_always::<&BitShift>(Pointer::VT_SHIFT, shift);
  }
  #[inline]
  pub fn add_child(&mut self, child: flatbuffers::WIPOffset<Type<'b >>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<Type>>(Pointer::VT_CHILD, child);
  }
  #[inline]
  pub fn add_addressing(&mut self, addressing: PointerAddressing) {
    self.fbb_.push_slot::<PointerAddressing>(Pointer::VT_ADDRESSING, addressing, PointerAddressing::Absolute);
  }
  #[inline]
  pub fn add_offset(&mut self, offset: &BitOffset) {
    self.fbb_.push_slot_always::<&BitOffset>(Pointer::VT_OFFSET, offset);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> PointerBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    PointerBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Pointer<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Pointer<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Pointer");
      ds.field("width", &self.width());
      ds.field("shift", &self.shift());
      ds.field("child", &self.child());
      ds.field("addressing", &self.addressing());
      ds.field("offset", &self.offset());
      ds.finish()
  }
}
