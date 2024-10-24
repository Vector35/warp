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
pub enum SymbolOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Symbol<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Symbol<'a> {
  type Inner = Symbol<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Symbol<'a> {
  pub const VT_NAME: flatbuffers::VOffsetT = 4;
  pub const VT_MODIFIERS: flatbuffers::VOffsetT = 6;
  pub const VT_CLASS_TYPE: flatbuffers::VOffsetT = 8;
  pub const VT_CLASS: flatbuffers::VOffsetT = 10;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Symbol { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr, A: flatbuffers::Allocator + 'bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr, A>,
    args: &'args SymbolArgs<'args>
  ) -> flatbuffers::WIPOffset<Symbol<'bldr>> {
    let mut builder = SymbolBuilder::new(_fbb);
    if let Some(x) = args.class { builder.add_class(x); }
    if let Some(x) = args.name { builder.add_name(x); }
    builder.add_class_type(args.class_type);
    builder.add_modifiers(args.modifiers);
    builder.finish()
  }


  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Symbol::VT_NAME, None)}
  }
  #[inline]
  pub fn modifiers(&self) -> SymbolModifiers {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<SymbolModifiers>(Symbol::VT_MODIFIERS, Some(Default::default())).unwrap()}
  }
  #[inline]
  pub fn class_type(&self) -> SymbolClass {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<SymbolClass>(Symbol::VT_CLASS_TYPE, Some(SymbolClass::NONE)).unwrap()}
  }
  #[inline]
  pub fn class(&self) -> flatbuffers::Table<'a> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(Symbol::VT_CLASS, None).unwrap()}
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn class_as_function_symbol_class(&self) -> Option<FunctionSymbolClass<'a>> {
    if self.class_type() == SymbolClass::FunctionSymbolClass {
      let u = self.class();
      // Safety:
      // Created from a valid Table for this object
      // Which contains a valid union in this slot
      Some(unsafe { FunctionSymbolClass::init_from_table(u) })
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn class_as_data_symbol_class(&self) -> Option<DataSymbolClass<'a>> {
    if self.class_type() == SymbolClass::DataSymbolClass {
      let u = self.class();
      // Safety:
      // Created from a valid Table for this object
      // Which contains a valid union in this slot
      Some(unsafe { DataSymbolClass::init_from_table(u) })
    } else {
      None
    }
  }

}

impl flatbuffers::Verifiable for Symbol<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .visit_field::<SymbolModifiers>("modifiers", Self::VT_MODIFIERS, false)?
     .visit_union::<SymbolClass, _>("class_type", Self::VT_CLASS_TYPE, "class", Self::VT_CLASS, true, |key, v, pos| {
        match key {
          SymbolClass::FunctionSymbolClass => v.verify_union_variant::<flatbuffers::ForwardsUOffset<FunctionSymbolClass>>("SymbolClass::FunctionSymbolClass", pos),
          SymbolClass::DataSymbolClass => v.verify_union_variant::<flatbuffers::ForwardsUOffset<DataSymbolClass>>("SymbolClass::DataSymbolClass", pos),
          _ => Ok(()),
        }
     })?
     .finish();
    Ok(())
  }
}
pub struct SymbolArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
    pub modifiers: SymbolModifiers,
    pub class_type: SymbolClass,
    pub class: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for SymbolArgs<'a> {
  #[inline]
  fn default() -> Self {
    SymbolArgs {
      name: None,
      modifiers: Default::default(),
      class_type: SymbolClass::NONE,
      class: None, // required field
    }
  }
}

pub struct SymbolBuilder<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a, A>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b, A: flatbuffers::Allocator + 'a> SymbolBuilder<'a, 'b, A> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Symbol::VT_NAME, name);
  }
  #[inline]
  pub fn add_modifiers(&mut self, modifiers: SymbolModifiers) {
    self.fbb_.push_slot::<SymbolModifiers>(Symbol::VT_MODIFIERS, modifiers, Default::default());
  }
  #[inline]
  pub fn add_class_type(&mut self, class_type: SymbolClass) {
    self.fbb_.push_slot::<SymbolClass>(Symbol::VT_CLASS_TYPE, class_type, SymbolClass::NONE);
  }
  #[inline]
  pub fn add_class(&mut self, class: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Symbol::VT_CLASS, class);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a, A>) -> SymbolBuilder<'a, 'b, A> {
    let start = _fbb.start_table();
    SymbolBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Symbol<'a>> {
    let o = self.fbb_.end_table(self.start_);
    self.fbb_.required(o, Symbol::VT_CLASS,"class");
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Symbol<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Symbol");
      ds.field("name", &self.name());
      ds.field("modifiers", &self.modifiers());
      ds.field("class_type", &self.class_type());
      match self.class_type() {
        SymbolClass::FunctionSymbolClass => {
          if let Some(x) = self.class_as_function_symbol_class() {
            ds.field("class", &x)
          } else {
            ds.field("class", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        SymbolClass::DataSymbolClass => {
          if let Some(x) = self.class_as_data_symbol_class() {
            ds.field("class", &x)
          } else {
            ds.field("class", &"InvalidFlatbuffer: Union discriminant does not match value.")
          }
        },
        _ => {
          let x: Option<()> = None;
          ds.field("class", &x)
        },
      };
      ds.finish()
  }
}