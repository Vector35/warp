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
// struct BitShift, aligned to 8
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct BitShift(pub [u8; 8]);
impl Default for BitShift { 
  fn default() -> Self { 
    Self([0; 8])
  }
}
impl core::fmt::Debug for BitShift {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("BitShift")
      .field("value", &self.value())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for BitShift {}
impl<'a> flatbuffers::Follow<'a> for BitShift {
  type Inner = &'a BitShift;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a BitShift>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a BitShift {
  type Inner = &'a BitShift;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<BitShift>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for BitShift {
    type Output = BitShift;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        let src = ::core::slice::from_raw_parts(self as *const BitShift as *const u8, <Self as flatbuffers::Push>::size());
        dst.copy_from_slice(src);
    }
    #[inline]
    fn alignment() -> flatbuffers::PushAlignment {
        flatbuffers::PushAlignment::new(8)
    }
}

impl<'a> flatbuffers::Verifiable for BitShift {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> BitShift {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    value: i64,
  ) -> Self {
    let mut s = Self([0; 8]);
    s.set_value(value);
    s
  }

  pub fn value(&self) -> i64 {
    let mut mem = core::mem::MaybeUninit::<<i64 as EndianScalar>::Scalar>::uninit();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    EndianScalar::from_little_endian(unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<<i64 as EndianScalar>::Scalar>(),
      );
      mem.assume_init()
    })
  }

  pub fn set_value(&mut self, x: i64) {
    let x_le = x.to_little_endian();
    // Safety:
    // Created from a valid Table for this object
    // Which contains a valid value in this slot
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const _ as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<<i64 as EndianScalar>::Scalar>(),
      );
    }
  }

}

