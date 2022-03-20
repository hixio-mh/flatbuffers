// automatically generated by the FlatBuffers compiler, do not modify
extern crate flatbuffers;
use std::mem;
use std::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
pub enum TestSimpleTableWithEnumOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct TestSimpleTableWithEnum<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TestSimpleTableWithEnum<'a> {
  type Inner = TestSimpleTableWithEnum<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> TestSimpleTableWithEnum<'a> {
  pub const VT_COLOR: flatbuffers::VOffsetT = 4;

  pub const fn get_fully_qualified_name() -> &'static str {
    "MyGame.Example.TestSimpleTableWithEnum"
  }

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    TestSimpleTableWithEnum { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args TestSimpleTableWithEnumArgs
  ) -> flatbuffers::WIPOffset<TestSimpleTableWithEnum<'bldr>> {
    let mut builder = TestSimpleTableWithEnumBuilder::new(_fbb);
    builder.add_color(args.color);
    builder.finish()
  }

  pub fn unpack(&self) -> TestSimpleTableWithEnumT {
    let color = self.color();
    TestSimpleTableWithEnumT {
      color,
    }
  }

  #[inline]
  pub fn color(&self) -> Color {
    self._tab.get::<Color>(TestSimpleTableWithEnum::VT_COLOR, Some(Color::Green)).unwrap()
  }
}

impl flatbuffers::Verifiable for TestSimpleTableWithEnum<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<Color>("color", Self::VT_COLOR, false)?
     .finish();
    Ok(())
  }
}
pub struct TestSimpleTableWithEnumArgs {
    pub color: Color,
}
impl<'a> Default for TestSimpleTableWithEnumArgs {
  #[inline]
  fn default() -> Self {
    TestSimpleTableWithEnumArgs {
      color: Color::Green,
    }
  }
}
pub struct TestSimpleTableWithEnumBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TestSimpleTableWithEnumBuilder<'a, 'b> {
  #[inline]
  pub fn add_color(&mut self, color: Color) {
    self.fbb_.push_slot::<Color>(TestSimpleTableWithEnum::VT_COLOR, color, Color::Green);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> TestSimpleTableWithEnumBuilder<'a, 'b> {
    let start = _fbb.start_table();
    TestSimpleTableWithEnumBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<TestSimpleTableWithEnum<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl std::fmt::Debug for TestSimpleTableWithEnum<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut ds = f.debug_struct("TestSimpleTableWithEnum");
      ds.field("color", &self.color());
      ds.finish()
  }
}
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub struct TestSimpleTableWithEnumT {
  pub color: Color,
}
impl Default for TestSimpleTableWithEnumT {
  fn default() -> Self {
    Self {
      color: Color::Green,
    }
  }
}
impl TestSimpleTableWithEnumT {
  pub fn pack<'b>(
    &self,
    _fbb: &mut flatbuffers::FlatBufferBuilder<'b>
  ) -> flatbuffers::WIPOffset<TestSimpleTableWithEnum<'b>> {
    let color = self.color;
    TestSimpleTableWithEnum::create(_fbb, &TestSimpleTableWithEnumArgs{
      color,
    })
  }
}
