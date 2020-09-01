//! This is a small shim library for passing closures where you need a format trait.
//!
//! ## Example
//!
//! ```rust
//! use fn_formats::DebugFmt;
//!
//! let formattable = DebugFmt(|f| {
//!     f.debug_struct("StructName")
//!         .field("list", &DebugFmt(|f| f.debug_list().entries(&[1, 2, 3]).finish()))
//!         .field("set", &DebugFmt(|f| f.debug_set().entries(&[4, 5, 6]).finish()))
//!         .finish()
//! });
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
#![doc(html_root_url = "https://docs.rs/fn-formats/0.0.3")]

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
///
/// # Example
///
/// ```rust
/// use fn_formats::DebugFmt;
///
/// let debug = DebugFmt(|f| write!(f, "debug"));
///
/// assert_eq!(format!("{:?}", debug), "debug");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct DebugFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Debug for DebugFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

/// ```rust
/// let _: fn_formats::DebugFmt<_> = (|f: &mut core::fmt::Formatter| Ok(())).into();
/// ```
impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for DebugFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`Display`] by calling the stored closure.
///
/// [`Display`]: https://doc.rust-lang.org/stable/core/fmt/trait.Display.html
///
/// # Example
///
/// ```rust
/// use fn_formats::DisplayFmt;
///
/// let display = DisplayFmt(|f| write!(f, "display"));
///
/// assert_eq!(format!("{}", display), "display");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct DisplayFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Display for DisplayFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

/// ```rust
/// let _: fn_formats::DisplayFmt<_> = (|f: &mut core::fmt::Formatter| Ok(())).into();
/// ```
impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for DisplayFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`Debug`] and [`Display`] by calling the stored closure.
///
/// [`Debug`]: https://doc.rust-lang.org/stable/core/fmt/trait.Debug.html
/// [`Display`]: https://doc.rust-lang.org/stable/core/fmt/trait.Display.html
///
/// # Example
///
/// ```rust
/// use fn_formats::DebugDisplayFmt;
///
/// let debug_display = DebugDisplayFmt(|f| write!(f, "debug or display"));
///
/// assert_eq!(format!("{:?}", debug_display), "debug or display");
/// assert_eq!(format!("{}", debug_display), "debug or display");
/// ```
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

/// ```rust
/// let _: fn_formats::DebugDisplayFmt<_> = (|f: &mut core::fmt::Formatter| Ok(())).into();
/// ```
impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for DebugDisplayFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`Debug`] and [`Display`] by calling the respective stored closure.
///
/// [`Debug`]: https://doc.rust-lang.org/stable/core/fmt/trait.Debug.html
/// [`Display`]: https://doc.rust-lang.org/stable/core/fmt/trait.Display.html
///
/// # Example
///
/// ```rust
/// use fn_formats::DebugDisplayFmtSeparate;
///
/// let debug_display = DebugDisplayFmtSeparate {
///     debug: |f| write!(f, "debug"),
///     display: |f| write!(f, "display"),
/// };
///
/// assert_eq!(format!("{:?}", debug_display), "debug");
/// assert_eq!(format!("{}", debug_display), "display");
/// ```
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
///
/// # Example
///
/// ```rust
/// use fn_formats::BinaryFmt;
///
/// let binary = BinaryFmt(|f| write!(f, "binary"));
///
/// assert_eq!(format!("{:b}", binary), "binary");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct BinaryFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Binary for BinaryFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

/// ```rust
/// let _: fn_formats::BinaryFmt<_> = (|f: &mut core::fmt::Formatter| Ok(())).into();
/// ```
impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for BinaryFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`LowerExp`] by calling the stored closure.
///
/// [`LowerExp`]: https://doc.rust-lang.org/stable/core/fmt/trait.LowerExp.html
///
/// # Example
///
/// ```rust
/// use fn_formats::LowerExpFmt;
///
/// let lower_exp = LowerExpFmt(|f| write!(f, "lower exp"));
///
/// assert_eq!(format!("{:e}", lower_exp), "lower exp");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct LowerExpFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> LowerExp for LowerExpFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

/// ```rust
/// let _: fn_formats::LowerExpFmt<_> = (|f: &mut core::fmt::Formatter| Ok(())).into();
/// ```
impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for LowerExpFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`LowerHex`] by calling the stored closure.
///
/// [`LowerHex`]: https://doc.rust-lang.org/stable/core/fmt/trait.LowerHex.html
///
/// # Example
///
/// ```rust
/// use fn_formats::LowerHexFmt;
///
/// let lower_hex = LowerHexFmt(|f| write!(f, "lower hex"));
///
/// assert_eq!(format!("{:x}", lower_hex), "lower hex");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct LowerHexFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> LowerHex for LowerHexFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

/// ```rust
/// let _: fn_formats::LowerHexFmt<_> = (|f: &mut core::fmt::Formatter| Ok(())).into();
/// ```
impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for LowerHexFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`Octal`] by calling the stored closure.
///
/// [`Octal`]: https://doc.rust-lang.org/stable/core/fmt/trait.Octal.html
///
/// # Example
///
/// ```rust
/// use fn_formats::OctalFmt;
///
/// let octal = OctalFmt(|f| write!(f, "octal"));
///
/// assert_eq!(format!("{:o}", octal), "octal");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct OctalFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Octal for OctalFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

/// ```rust
/// let _: fn_formats::OctalFmt<_> = (|f: &mut core::fmt::Formatter| Ok(())).into();
/// ```
impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for OctalFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`Pointer`] by calling the stored closure.
///
/// [`Pointer`]: https://doc.rust-lang.org/stable/core/fmt/trait.Pointer.html
///
/// # Example
///
/// ```rust
/// use fn_formats::PointerFmt;
///
/// let pointer = PointerFmt(|f| write!(f, "pointer"));
///
/// assert_eq!(format!("{:p}", pointer), "pointer");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct PointerFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> Pointer for PointerFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

/// ```rust
/// let _: fn_formats::PointerFmt<_> = (|f: &mut core::fmt::Formatter| Ok(())).into();
/// ```
impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for PointerFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`UpperExp`] by calling the stored closure.
///
/// [`UpperExp`]: https://doc.rust-lang.org/stable/core/fmt/trait.UpperExp.html
///
/// # Example
///
/// ```rust
/// use fn_formats::UpperExpFmt;
///
/// let upper_exp = UpperExpFmt(|f| write!(f, "upper exp"));
///
/// assert_eq!(format!("{:E}", upper_exp), "upper exp");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct UpperExpFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> UpperExp for UpperExpFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

/// ```rust
/// let _: fn_formats::UpperExpFmt<_> = (|f: &mut core::fmt::Formatter| Ok(())).into();
/// ```
impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for UpperExpFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements [`UpperHex`] by calling the stored closure.
///
/// [`UpperHex`]: https://doc.rust-lang.org/stable/core/fmt/trait.UpperHex.html
///
/// # Example
///
/// ```rust
/// use fn_formats::UpperHexFmt;
///
/// let upper_hex = UpperHexFmt(|f| write!(f, "upperhex"));
///
/// assert_eq!(format!("{:X}", upper_hex), "upperhex");
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
pub struct UpperHexFmt<Fmt>(pub Fmt)
where
	Fmt: Fn(&mut Formatter) -> fmt::Result;

impl<Fmt: Fn(&mut Formatter) -> fmt::Result> UpperHex for UpperHexFmt<Fmt> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.0(f)
	}
}

/// ```rust
/// let _: fn_formats::UpperHexFmt<_> = (|f: &mut core::fmt::Formatter| Ok(())).into();
/// ```
impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for UpperHexFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements all format traits by calling the stored closure.
///
/// # Example
///
/// ```rust
/// use fn_formats::ComprehensiveFmt;
///
/// let comprehensive = ComprehensiveFmt(|f| write!(f, "fmt"));
///
/// assert_eq!(format!("{:b}", comprehensive), "fmt");
/// assert_eq!(format!("{:?}", comprehensive), "fmt");
/// assert_eq!(format!("{}", comprehensive), "fmt");
/// assert_eq!(format!("{:e}", comprehensive), "fmt");
/// assert_eq!(format!("{:x}", comprehensive), "fmt");
/// assert_eq!(format!("{:o}", comprehensive), "fmt");
/// assert_eq!(format!("{:p}", comprehensive), "fmt");
/// assert_eq!(format!("{:E}", comprehensive), "fmt");
/// assert_eq!(format!("{:X}", comprehensive), "fmt");
/// ```
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

/// ```rust
/// let _: fn_formats::ComprehensiveFmt<_> = (|f: &mut core::fmt::Formatter| Ok(())).into();
/// ```
impl<Fmt: Fn(&mut Formatter) -> fmt::Result> From<Fmt> for ComprehensiveFmt<Fmt> {
	fn from(fmt: Fmt) -> Self {
		Self(fmt)
	}
}

/// Implements all format traits by calling the respective stored closure.
///
/// # Example
///
/// ```rust
/// use fn_formats::ComprehensiveFmtSeparate;
///
/// let comprehensive = ComprehensiveFmtSeparate {
///     binary: |f| write!(f, "binary"),
///     debug: |f| write!(f, "debug"),
///     display: |f| write!(f, "display"),
///     lower_exp: |f| write!(f, "lower exp"),
///     lower_hex: |f| write!(f, "lower hex"),
///     octal: |f| write!(f, "octal"),
///     pointer: |f| write!(f, "pointer"),
///     upper_exp: |f| write!(f, "upper exp"),
///     upper_hex: |f| write!(f, "upper hex"),
/// };
///
/// assert_eq!(format!("{:b}", comprehensive), "binary");
/// assert_eq!(format!("{:?}", comprehensive), "debug");
/// assert_eq!(format!("{}", comprehensive), "display");
/// assert_eq!(format!("{:e}", comprehensive), "lower exp");
/// assert_eq!(format!("{:x}", comprehensive), "lower hex");
/// assert_eq!(format!("{:o}", comprehensive), "octal");
/// assert_eq!(format!("{:p}", comprehensive), "pointer");
/// assert_eq!(format!("{:E}", comprehensive), "upper exp");
/// assert_eq!(format!("{:X}", comprehensive), "upper hex");
/// ```
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
