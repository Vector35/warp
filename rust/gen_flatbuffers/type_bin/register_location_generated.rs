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
pub enum RegisterLocationOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct RegisterLocation<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for RegisterLocation<'a> {
  type Inner = RegisterLocation<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> RegisterLocation<'a> {
  pub const VT_ID: flatbuffers::VOffsetT = 4;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    RegisterLocation { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args RegisterLocationArgs
  ) -> flatbuffers::WIPOffset<RegisterLocation<'bldr>> {
    let mut builder = RegisterLocationBuilder::new(_fbb);
    builder.add_id(args.id);
    builder.finish()
  }


  #[inline]
  pub fn id(&self) -> u64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<u64>(RegisterLocation::VT_ID, Some(0)).unwrap()}
  }
}

impl flatbuffers::Verifiable for RegisterLocation<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<u64>("id", Self::VT_ID, false)?
     .finish();
    Ok(())
  }
}
pub struct RegisterLocationArgs {
    pub id: u64,
}
impl<'a> Default for RegisterLocationArgs {
  #[inline]
  fn default() -> Self {
    RegisterLocationArgs {
      id: 0,
    }
  }
}

pub struct RegisterLocationBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> RegisterLocationBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_id(&mut self, id: u64) {
    self.fbb_.push_slot::<u64>(RegisterLocation::VT_ID, id, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> RegisterLocationBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    RegisterLocationBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<RegisterLocation<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for RegisterLocation<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("RegisterLocation");
      ds.field("id", &self.id());
      ds.finish()
  }
}