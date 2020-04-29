fn function_to_adapt(x: u32) -> u32 { x }

macro_rules! function_adaptor {
	($f:expr $(, $a:expr )*) => {
		$f($($a),*)
	};
}

fn main() {
    println!("{}", function_adaptor!(function_to_adapt, 3));
}
