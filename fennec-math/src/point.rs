use std::ops::*;
use num_traits::{ Zero, One };

// Macros.

/// Generate a point struct.
macro_rules! generate_point {

	($Point:ident { $( $element:ident ),+ } ) => {

		// Struct.

		#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
		pub struct $Point<T = f32> {
			$( pub $element: T ),+
		}

		// Construct functions.

		impl<T> $Point<T> {
			
			/// Create a new point.
			pub fn new($( $element: T ),+) -> Self {
				Self { $( $element: $element ),+ }
			}

		}

		impl<T> $Point<T> where T: Copy {

			/// Create a new point with all its elements set to input value.
			pub fn splat(value: T) -> Self {
				Self { $( $element: value ),+ }
			}

		}

		impl<T> $Point<T> where T: Zero {

			/// Create a new point with all its elements set to zero.
			pub fn zero() -> Self {
				Self { $( $element: T::zero() ),+ }
			}

		}

		impl<T> $Point<T> where T: One {

			/// Create a new point with all its elements set to one.
			pub fn one() -> Self {
				Self { $( $element: T::one() ),+ }
			}

		}

		// Operators.

		impl_binary_op!(impl Add for $Point { add({ $( $element ),+ }) });
		impl_binary_op!(impl Sub for $Point { sub({ $( $element ),+ }) });
		impl_binary_op!(impl Mul for $Point { mul({ $( $element ),+ }) });
		impl_binary_op!(impl Div for $Point { div({ $( $element ),+ }) });
		impl_binary_op!(impl Rem for $Point { rem({ $( $element ),+ }) });
	
		impl_assign_op!(impl AddAssign for $Point { add_assign({ $( $element ),+ }) });
		impl_assign_op!(impl SubAssign for $Point { sub_assign({ $( $element ),+ }) });
		impl_assign_op!(impl MulAssign for $Point { mul_assign({ $( $element ),+ }) });
		impl_assign_op!(impl DivAssign for $Point { div_assign({ $( $element ),+ }) });
		impl_assign_op!(impl RemAssign for $Point { rem_assign({ $( $element ),+ }) });

		impl_unary_op!(impl Neg for $Point { neg({ $( $element ),+ }) });

	}

}



// Generate point structs.

generate_point!(Point1 { x });
generate_point!(Point2 { x, y });
generate_point!(Point3 { x, y, z });
generate_point!(Point4 { x, y, z, w });



// Tests

#[cfg(test)]
mod tests {

	use super::{ Point1, Point2, Point3, Point4 };

	macro_rules! test_point {

		($name:ident, $Point:ident { $( $field:ident:$input:literal ),+ }) => {
			
			#[test]
			fn $name() {

				{
					let p = $Point::new($( $input ),+);

					$( assert_eq!(p.$field, $input); )+
						
					let mut add = p + p;
					let mut sub = p - p;
					let mut mul = p * p;
					let mut div = p / p;
					let mut rem = p % p;
					
					$(
						assert_eq!(add.$field, $input + $input);
						assert_eq!(sub.$field, $input - $input);
						assert_eq!(mul.$field, $input * $input);
						assert_eq!(div.$field, $input / $input);
						assert_eq!(rem.$field, $input % $input);
					)+

					add += p;
					sub -= p;
					mul *= p;
					div /= p;
					rem %= p;
					
					$(
						assert_eq!(add.$field, $input + $input + $input);
						assert_eq!(sub.$field, $input - $input - $input);
						assert_eq!(mul.$field, $input * $input * $input);
						assert_eq!(div.$field, $input / $input / $input);
						assert_eq!(rem.$field, $input % $input % $input);
					)+

				}

			}

		};

	}

	test_point!(test_point1, Point1 { x: 1.2 });
	test_point!(test_point2, Point2 { x: 1.2, y: 2.3 });
	test_point!(test_point3, Point3 { x: 1.2, y: 2.3, z: 3.4 });
	test_point!(test_point4, Point4 { x: 1.2, y: 2.3, z: 3.4, w: 4.5 });

}

