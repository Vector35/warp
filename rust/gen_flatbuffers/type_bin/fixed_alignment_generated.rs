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
pub enum FixedAlignmentOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct FixedAlignment<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for FixedAlignment<'a> {
  type Inner = FixedAlignment<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> FixedAlignment<'a> {
  pub const VT_WIDTH: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    FixedAlignment { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args FixedAlignmentArgs<'args>
  ) -> flatbuffers::WIPOffset<FixedAlignment<'bldr>> {
    let mut builder = FixedAlignmentBuilder::new(_fbb);
    if let Some(x) = args.width { builder.add_width(x); }
    builder.finish()
  }


  #[inline]
  pub fn width(&self) -> Option<&'a BitWidth> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<BitWidth>(FixedAlignment::VT_WIDTH, None)}
  }
}

impl flatbuffers::Verifiable for FixedAlignment<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<BitWidth>("width", Self::VT_WIDTH, false)?
     .finish();
    Ok(())
  }
}
pub struct FixedAlignmentArgs<'a> {
    pub width: Option<&'a BitWidth>,
}
impl<'a> Default for FixedAlignmentArgs<'a> {
  #[inline]
  fn default() -> Self {
    FixedAlignmentArgs {
      width: None,
    }
  }
}

pub struct FixedAlignmentBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> FixedAlignmentBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_width(&mut self, width: &BitWidth) {
    self.fbb_.push_slot_always::<&BitWidth>(FixedAlignment::VT_WIDTH, width);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> FixedAlignmentBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    FixedAlignmentBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<FixedAlignment<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for FixedAlignment<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("FixedAlignment");
      ds.field("width", &self.width());
      ds.finish()
  }
}