//! This is a small shim library for passing closures where you need a format trait.
//!
//! ## Example
//!
//! ```rust
//! use fn_formats::DebugFmt;
//!
//! let formattable: DebugFmt<_> = (|f: &mut core::fmt::Formatter| {
//!     f.debug_struct("StructName")
//!         .field("list", &DebugFmt(|f| f.debug_list().entries(&[1, 2, 3]).finish()))
//!         .field("set", &DebugFmt(|f| f.debug_set().entries(&[4, 5, 6]).finish()))
//!         .finish()
//! }).into();
//!
//! assert_eq!(format!("{:?}", formattable), "StructName { list: [1, 2, 3], set: {4, 5, 6} }");
//! ```
//!
//! There are also [`From`] implementations where applicable:
//!
//! ```rust
//! use fn_formats::ComprehensiveFmt;
//! let _: ComprehensiveFmt<_> = (|f: &mut core::fmt::Formatter| Ok(())).into();
//! ```
//!
//! [`From`]: https://doc.rust-lang.org/stable/std/convert/trait.From.html

#![no_std]

#[cfg(doctest)]
pub mod readme {
	doc_comment::doctest!("../README.md");
}

use core::fmt::{
	self, Binary, Debug, Display, Formatter, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex,
};

/// Implements [`Debug`] by calling the stored closure.
///
/// [`Debug`]: https://doc.rust-lang.org/stable/core/fmt/trait.Debug.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct DebugFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Debug for DebugFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for DebugFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`Display`] by calling the stored closure.
///
/// [`Display`]: https://doc.rust-lang.org/stable/core/fmt/trait.Display.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct DisplayFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Display for DisplayFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for DisplayFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`Debug`] and [`Display`] by calling the stored closure.
///
/// [`Debug`]: https://doc.rust-lang.org/stable/core/fmt/trait.Debug.html
/// [`Display`]: https://doc.rust-lang.org/stable/core/fmt/trait.Display.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct DebugDisplayFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Debug for DebugDisplayFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Display for DebugDisplayFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for DebugDisplayFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`Debug`] and [`Display`] by calling the respective stored closure.
///
/// [`Debug`]: https://doc.rust-lang.org/stable/core/fmt/trait.Debug.html
/// [`Display`]: https://doc.rust-lang.org/stable/core/fmt/trait.Display.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct DebugDisplayFmtSeparate<DebugFmt, DisplayFmt>
where
	DebugFmt: Fn(&mut Formatter) -> fmt::Result,
	DisplayFmt: Fn(&mut Formatter) -> fmt::Result,
{
	pub debug: DebugFmt,
	pub display: DisplayFmt,
}

impl<
		DebugFmt: Fn(&mut Formatter) -> fmt::Result,
		DisplayFmt: Fn(&mut Formatter) -> fmt::Result,
	> Debug for DebugDisplayFmtSeparate<DebugFmt, DisplayFmt>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		(self.debug)(f)
	}
}

impl<
		DebugFmt: Fn(&mut Formatter) -> fmt::Result,
		DisplayFmt: Fn(&mut Formatter) -> fmt::Result,
	> Display for DebugDisplayFmtSeparate<DebugFmt, DisplayFmt>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		(self.display)(f)
	}
}

/// Implements [`Binary`] by calling the stored closure.
///
/// [`Binary`]: https://doc.rust-lang.org/stable/core/fmt/trait.Binary.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct BinaryFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Binary for BinaryFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for BinaryFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`LowerExp`] by calling the stored closure.
///
/// [`LowerExp`]: https://doc.rust-lang.org/stable/core/fmt/trait.LowerExp.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct LowerExpFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> LowerExp for LowerExpFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for LowerExpFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`LowerHex`] by calling the stored closure.
///
/// [`LowerHex`]: https://doc.rust-lang.org/stable/core/fmt/trait.LowerHex.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct LowerHexFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> LowerHex for LowerHexFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for LowerHexFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`Octal`] by calling the stored closure.
///
/// [`Octal`]: https://doc.rust-lang.org/stable/core/fmt/trait.Octal.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct OctalFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Octal for OctalFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for OctalFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`Pointer`] by calling the stored closure.
///
/// [`Pointer`]: https://doc.rust-lang.org/stable/core/fmt/trait.Pointer.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct PointerFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Pointer for PointerFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for PointerFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`UpperExp`] by calling the stored closure.
///
/// [`UpperExp`]: https://doc.rust-lang.org/stable/core/fmt/trait.UpperExp.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct UpperExpFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> UpperExp for UpperExpFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for UpperExpFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`UpperHex`] by calling the stored closure.
///
/// [`UpperHex`]: https://doc.rust-lang.org/stable/core/fmt/trait.UpperHex.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct UpperHexFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> UpperHex for UpperHexFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for UpperHexFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements all format traits by calling the stored closure.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct ComprehensiveFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Binary for ComprehensiveFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Debug for ComprehensiveFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Display for ComprehensiveFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> LowerExp for ComprehensiveFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> LowerHex for ComprehensiveFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Octal for ComprehensiveFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Pointer for ComprehensiveFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> UpperExp for ComprehensiveFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> UpperHex for ComprehensiveFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for ComprehensiveFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements all format traits by calling the respective stored closure.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct ComprehensiveFmtSeparate<
	BinaryFmt: Fn(&mut Formatter) -> fmt::Result,
	DebugFmt: Fn(&mut Formatter) -> fmt::Result,
	DisplayFmt: Fn(&mut Formatter) -> fmt::Result,
	LowerExpFmt: Fn(&mut Formatter) -> fmt::Result,
	LowerHexFmt: Fn(&mut Formatter) -> fmt::Result,
	OctalFmt: Fn(&mut Formatter) -> fmt::Result,
	PointerFmt: Fn(&mut Formatter) -> fmt::Result,
	UpperExpFmt: Fn(&mut Formatter) -> fmt::Result,
	UpperHexFmt: Fn(&mut Formatter) -> fmt::Result,
> {
	pub binary: BinaryFmt,
	pub debug: DebugFmt,
	pub display: DisplayFmt,
	pub lower_exp: LowerExpFmt,
	pub lower_hex: LowerHexFmt,
	pub octal: OctalFmt,
	pub pointer: PointerFmt,
	pub upper_exp: UpperExpFmt,
	pub upper_hex: UpperHexFmt,
}

impl<
		BinaryFmt: Fn(&mut Formatter) -> fmt::Result,
		DebugFmt: Fn(&mut Formatter) -> fmt::Result,
		DisplayFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerExpFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerHexFmt: Fn(&mut Formatter) -> fmt::Result,
		OctalFmt: Fn(&mut Formatter) -> fmt::Result,
		PointerFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperExpFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperHexFmt: Fn(&mut Formatter) -> fmt::Result,
	> Binary
	for ComprehensiveFmtSeparate<
		BinaryFmt,
		DebugFmt,
		DisplayFmt,
		LowerExpFmt,
		LowerHexFmt,
		OctalFmt,
		PointerFmt,
		UpperExpFmt,
		UpperHexFmt,
	>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		(self.binary)(f)
	}
}

impl<
		BinaryFmt: Fn(&mut Formatter) -> fmt::Result,
		DebugFmt: Fn(&mut Formatter) -> fmt::Result,
		DisplayFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerExpFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerHexFmt: Fn(&mut Formatter) -> fmt::Result,
		OctalFmt: Fn(&mut Formatter) -> fmt::Result,
		PointerFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperExpFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperHexFmt: Fn(&mut Formatter) -> fmt::Result,
	> Debug
	for ComprehensiveFmtSeparate<
		BinaryFmt,
		DebugFmt,
		DisplayFmt,
		LowerExpFmt,
		LowerHexFmt,
		OctalFmt,
		PointerFmt,
		UpperExpFmt,
		UpperHexFmt,
	>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		(self.debug)(f)
	}
}

impl<
		BinaryFmt: Fn(&mut Formatter) -> fmt::Result,
		DebugFmt: Fn(&mut Formatter) -> fmt::Result,
		DisplayFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerExpFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerHexFmt: Fn(&mut Formatter) -> fmt::Result,
		OctalFmt: Fn(&mut Formatter) -> fmt::Result,
		PointerFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperExpFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperHexFmt: Fn(&mut Formatter) -> fmt::Result,
	> Display
	for ComprehensiveFmtSeparate<
		BinaryFmt,
		DebugFmt,
		DisplayFmt,
		LowerExpFmt,
		LowerHexFmt,
		OctalFmt,
		PointerFmt,
		UpperExpFmt,
		UpperHexFmt,
	>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		(self.display)(f)
	}
}

impl<
		BinaryFmt: Fn(&mut Formatter) -> fmt::Result,
		DebugFmt: Fn(&mut Formatter) -> fmt::Result,
		DisplayFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerExpFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerHexFmt: Fn(&mut Formatter) -> fmt::Result,
		OctalFmt: Fn(&mut Formatter) -> fmt::Result,
		PointerFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperExpFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperHexFmt: Fn(&mut Formatter) -> fmt::Result,
	> LowerExp
	for ComprehensiveFmtSeparate<
		BinaryFmt,
		DebugFmt,
		DisplayFmt,
		LowerExpFmt,
		LowerHexFmt,
		OctalFmt,
		PointerFmt,
		UpperExpFmt,
		UpperHexFmt,
	>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		(self.lower_exp)(f)
	}
}

impl<
		BinaryFmt: Fn(&mut Formatter) -> fmt::Result,
		DebugFmt: Fn(&mut Formatter) -> fmt::Result,
		DisplayFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerExpFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerHexFmt: Fn(&mut Formatter) -> fmt::Result,
		OctalFmt: Fn(&mut Formatter) -> fmt::Result,
		PointerFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperExpFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperHexFmt: Fn(&mut Formatter) -> fmt::Result,
	> LowerHex
	for ComprehensiveFmtSeparate<
		BinaryFmt,
		DebugFmt,
		DisplayFmt,
		LowerExpFmt,
		LowerHexFmt,
		OctalFmt,
		PointerFmt,
		UpperExpFmt,
		UpperHexFmt,
	>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		(self.lower_hex)(f)
	}
}

impl<
		BinaryFmt: Fn(&mut Formatter) -> fmt::Result,
		DebugFmt: Fn(&mut Formatter) -> fmt::Result,
		DisplayFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerExpFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerHexFmt: Fn(&mut Formatter) -> fmt::Result,
		OctalFmt: Fn(&mut Formatter) -> fmt::Result,
		PointerFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperExpFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperHexFmt: Fn(&mut Formatter) -> fmt::Result,
	> Octal
	for ComprehensiveFmtSeparate<
		BinaryFmt,
		DebugFmt,
		DisplayFmt,
		LowerExpFmt,
		LowerHexFmt,
		OctalFmt,
		PointerFmt,
		UpperExpFmt,
		UpperHexFmt,
	>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		(self.octal)(f)
	}
}

impl<
		BinaryFmt: Fn(&mut Formatter) -> fmt::Result,
		DebugFmt: Fn(&mut Formatter) -> fmt::Result,
		DisplayFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerExpFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerHexFmt: Fn(&mut Formatter) -> fmt::Result,
		OctalFmt: Fn(&mut Formatter) -> fmt::Result,
		PointerFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperExpFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperHexFmt: Fn(&mut Formatter) -> fmt::Result,
	> Pointer
	for ComprehensiveFmtSeparate<
		BinaryFmt,
		DebugFmt,
		DisplayFmt,
		LowerExpFmt,
		LowerHexFmt,
		OctalFmt,
		PointerFmt,
		UpperExpFmt,
		UpperHexFmt,
	>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		(self.pointer)(f)
	}
}

impl<
		BinaryFmt: Fn(&mut Formatter) -> fmt::Result,
		DebugFmt: Fn(&mut Formatter) -> fmt::Result,
		DisplayFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerExpFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerHexFmt: Fn(&mut Formatter) -> fmt::Result,
		OctalFmt: Fn(&mut Formatter) -> fmt::Result,
		PointerFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperExpFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperHexFmt: Fn(&mut Formatter) -> fmt::Result,
	> UpperExp
	for ComprehensiveFmtSeparate<
		BinaryFmt,
		DebugFmt,
		DisplayFmt,
		LowerExpFmt,
		LowerHexFmt,
		OctalFmt,
		PointerFmt,
		UpperExpFmt,
		UpperHexFmt,
	>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		(self.upper_exp)(f)
	}
}

impl<
		BinaryFmt: Fn(&mut Formatter) -> fmt::Result,
		DebugFmt: Fn(&mut Formatter) -> fmt::Result,
		DisplayFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerExpFmt: Fn(&mut Formatter) -> fmt::Result,
		LowerHexFmt: Fn(&mut Formatter) -> fmt::Result,
		OctalFmt: Fn(&mut Formatter) -> fmt::Result,
		PointerFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperExpFmt: Fn(&mut Formatter) -> fmt::Result,
		UpperHexFmt: Fn(&mut Formatter) -> fmt::Result,
	> UpperHex
	for ComprehensiveFmtSeparate<
		BinaryFmt,
		DebugFmt,
		DisplayFmt,
		LowerExpFmt,
		LowerHexFmt,
		OctalFmt,
		PointerFmt,
		UpperExpFmt,
		UpperHexFmt,
	>
{
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		(self.upper_hex)(f)
	}
}
