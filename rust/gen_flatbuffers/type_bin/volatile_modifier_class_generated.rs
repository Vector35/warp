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
pub enum VolatileModifierClassOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct VolatileModifierClass<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for VolatileModifierClass<'a> {
  type Inner = VolatileModifierClass<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> VolatileModifierClass<'a> {

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    VolatileModifierClass { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    _args: &'args VolatileModifierClassArgs
  ) -> flatbuffers::WIPOffset<VolatileModifierClass<'bldr>> {
    let mut builder = VolatileModifierClassBuilder::new(_fbb);
    builder.finish()
  }

}

impl flatbuffers::Verifiable for VolatileModifierClass<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .finish();
    Ok(())
  }
}
pub struct VolatileModifierClassArgs {
}
impl<'a> Default for VolatileModifierClassArgs {
  #[inline]
  fn default() -> Self {
    VolatileModifierClassArgs {
    }
  }
}

pub struct VolatileModifierClassBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> VolatileModifierClassBuilder<'a, 'b, A> {
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> VolatileModifierClassBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    VolatileModifierClassBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<VolatileModifierClass<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for VolatileModifierClass<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("VolatileModifierClass");
      ds.finish()
  }
}
