#![no_std]
#![allow(improper_ctypes)]

#[cfg(test)]
#[macro_use]
extern crate std;

#[repr(C)]
pub struct Seh {
	pub context_pointer: usize,
	pub try_handler: unsafe fn(context_pointer: usize),
	pub except_code: unsafe fn(context_pointer: usize) -> i32,
	pub except_handler: unsafe fn(context_pointer: usize),
}

#[allow(dead_code)]
extern "stdcall" {
	pub fn seh(seh: *const Seh);
	fn read(i_ptr: *const i32) -> i32;
	fn div(a: i32, b: i32) -> i32;
}

#[macro_export]
macro_rules! unsafe_seh {
	(
		__try $try_handler:tt
		__except($except_code:expr) $except_handler:tt
	) => {
		unsafe fn try_handler(_context_pointer: usize) $try_handler;
		unsafe fn except_handler(_context_pointer: usize) $except_handler;
		unsafe fn except_code(_context_pointer: usize) -> i32 { $except_code }
		let seh = $crate::Seh {
			context_pointer: 0,
			except_code,
			try_handler,
			except_handler,
		};
		unsafe { $crate::seh(&seh); }
	};
	(
		$context:ident: $ty:ty = $e:expr;
		__try $try_handler:tt
		__except($except_code:expr) $except_handler:tt
	) => {
		unsafe fn try_handler(context_pointer: usize) {
			#[allow(unused_variables)]
			let $context = &mut *(context_pointer as *mut $ty);
			$try_handler
		}
		unsafe fn except_handler(context_pointer: usize) {
			#[allow(unused_variables)]
			let $context = &mut *(context_pointer as *mut $ty);
			$except_handler
		}
		unsafe fn except_code(context_pointer: usize) -> i32 {
			#[allow(unused_variables)]
			let $context = &mut *(context_pointer as *mut $ty);
			$except_code
		}
		let mut context = $e;
		let seh = $crate::Seh {
			context_pointer: &mut context as *mut _ as usize,
			except_code,
			try_handler,
			except_handler,
		};
		unsafe { $crate::seh(&seh); }
	};
}

#[test]
fn test_nullptr_deref() {
	unsafe_seh! {
		__try {
			println!("__try");
			read(0 as *const i32);
		}
		__except(1) {
			println!("__except");
		}
	}
}

#[test]
fn test_div_by_zero() {
	unsafe_seh! {
		__try {
			println!("__try");
			div(0, 0);
		}
		__except(1) {
			println!("__except");
		}
	}
}
