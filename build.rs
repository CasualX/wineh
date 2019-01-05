
fn main() {
	cc::Build::new()
		.file("src/seh.cpp")
		.compile("seh")
}
