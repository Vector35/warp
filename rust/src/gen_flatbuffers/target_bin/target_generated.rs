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
pub enum TargetOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Target<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Target<'a> {
  type Inner = Target<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Target<'a> {
  pub const VT_ARCHITECTURE: flatbuffers::VOffsetT = 4;
  pub const VT_PLATFORM: flatbuffers::VOffsetT = 6;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Target { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args TargetArgs<'args>
  ) -> flatbuffers::WIPOffset<Target<'bldr>> {
    let mut builder = TargetBuilder::new(_fbb);
    if let Some(x) = args.platform { builder.add_platform(x); }
    if let Some(x) = args.architecture { builder.add_architecture(x); }
    builder.finish()
  }


  #[inline]
  pub fn architecture(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Target::VT_ARCHITECTURE, None)}
  }
  #[inline]
  pub fn platform(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Target::VT_PLATFORM, None)}
  }
}

impl flatbuffers::Verifiable for Target<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("architecture", Self::VT_ARCHITECTURE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("platform", Self::VT_PLATFORM, false)?
     .finish();
    Ok(())
  }
}
pub struct TargetArgs<'a> {
    pub architecture: Option<flatbuffers::WIPOffset<&'a str>>,
    pub platform: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for TargetArgs<'a> {
  #[inline]
  fn default() -> Self {
    TargetArgs {
      architecture: None,
      platform: None,
    }
  }
}

pub struct TargetBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> TargetBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_architecture(&mut self, architecture: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Target::VT_ARCHITECTURE, architecture);
  }
  #[inline]
  pub fn add_platform(&mut self, platform: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Target::VT_PLATFORM, platform);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> TargetBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    TargetBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Target<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Target<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Target");
      ds.field("architecture", &self.architecture());
      ds.field("platform", &self.platform());
      ds.finish()
  }
}
