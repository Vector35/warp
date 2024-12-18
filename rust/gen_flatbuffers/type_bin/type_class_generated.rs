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
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_TYPE_CLASS: u8 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_TYPE_CLASS: u8 = 12;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_TYPE_CLASS: [TypeClass; 13] = [
  TypeClass::NONE,
  TypeClass::Void,
  TypeClass::Boolean,
  TypeClass::Integer,
  TypeClass::Character,
  TypeClass::Float,
  TypeClass::Pointer,
  TypeClass::Array,
  TypeClass::Structure,
  TypeClass::Enumeration,
  TypeClass::Union,
  TypeClass::Function,
  TypeClass::Referrer,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct TypeClass(pub u8);
#[allow(non_upper_case_globals)]
impl TypeClass {
  pub const NONE: Self = Self(0);
  pub const Void: Self = Self(1);
  pub const Boolean: Self = Self(2);
  pub const Integer: Self = Self(3);
  pub const Character: Self = Self(4);
  pub const Float: Self = Self(5);
  pub const Pointer: Self = Self(6);
  pub const Array: Self = Self(7);
  pub const Structure: Self = Self(8);
  pub const Enumeration: Self = Self(9);
  pub const Union: Self = Self(10);
  pub const Function: Self = Self(11);
  pub const Referrer: Self = Self(12);

  pub const ENUM_MIN: u8 = 0;
  pub const ENUM_MAX: u8 = 12;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::NONE,
    Self::Void,
    Self::Boolean,
    Self::Integer,
    Self::Character,
    Self::Float,
    Self::Pointer,
    Self::Array,
    Self::Structure,
    Self::Enumeration,
    Self::Union,
    Self::Function,
    Self::Referrer,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::NONE => Some("NONE"),
      Self::Void => Some("Void"),
      Self::Boolean => Some("Boolean"),
      Self::Integer => Some("Integer"),
      Self::Character => Some("Character"),
      Self::Float => Some("Float"),
      Self::Pointer => Some("Pointer"),
      Self::Array => Some("Array"),
      Self::Structure => Some("Structure"),
      Self::Enumeration => Some("Enumeration"),
      Self::Union => Some("Union"),
      Self::Function => Some("Function"),
      Self::Referrer => Some("Referrer"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for TypeClass {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for TypeClass {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<u8>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for TypeClass {
    type Output = TypeClass;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<u8>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for TypeClass {
  type Scalar = u8;
  #[inline]
  fn to_little_endian(self) -> u8 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: u8) -> Self {
    let b = u8::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for TypeClass {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    u8::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for TypeClass {}
pub struct TypeClassUnionTableOffset {}

