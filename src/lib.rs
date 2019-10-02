#![no_std]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod ctypes {
	pub type c_char = u8;
	pub type c_schar = i8;
	pub type c_uchar = u8;
	pub type c_int = i32;
	pub type c_uint = u32;
	pub type c_long = i32;
	pub type c_ulong = u32;
	pub type c_longlong = i64;
	pub type c_ulonglong = u64;
	pub type c_ushort = u16;
	pub type c_short = i16;

	#[repr(u8)]
	pub enum c_void {
		// Two dummy variants so the #[repr] attribute can be used.
		#[doc(hidden)]
		__variant1,
		#[doc(hidden)]
		__variant2,
	}
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
