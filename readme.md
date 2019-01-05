Structured Exception Handling
=============================

Shitty attempt to get SEH working in Rust on Windows.

This works by handling the SEH code in C++ and calling that from Rust, I'm sure this is in violation of various standards but _it works_!

Probably don't want to mix this with panics. Nasal demons and all that.

```rust
use wineh::unsafe_seh;

#[inline(never)]
fn crash(a: i32, b: i32) -> i32 { a / b }

fn main() {
	unsafe_seh! {
		__try {
			println!("__try");
			let i = crash(0, 0);
			println!("{}", i);
		}
		__except(1) {
			println!("__except");
		}
	}
}
```
