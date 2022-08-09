#[macro_export]
/// If the passed Option is Some(val), evaluates to val. If the passed option is None, causes the enclosing function to return. An optional second argument specifies what to return.
macro_rules! unwrap_or_return {
	( $e:expr ) => {
		match $e {
			Some(x) => x,
			None => return,
		}
	};
	( $e:expr, $r:expr ) => {
		match $e {
			Some(x) => x,
			None => return $r,
		}
	};
}
