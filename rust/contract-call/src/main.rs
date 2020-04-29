fn function_to_adapt(x: u32) -> u32 { x }
fn _function_no_args() -> u32 { 5 }

macro_rules! function_adaptor {
	($f:expr $(, $a:expr)*) => {
		$f($($a),*)
	};
}

fn main() {
    println!("{}", function_adaptor!(function_to_adapt, 3));
}
