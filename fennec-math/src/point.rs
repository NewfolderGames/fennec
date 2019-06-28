use std::ops::*;
use num_traits::{ Zero, One };

use crate::traits::{ Splat };

// Macros.

/// Generate a point struct.
macro_rules! generate_point {

	($Point:ident { $( $element:ident ),+ } ) => {

		// Struct.

		#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
		pub struct $Point<T> {
			$( pub $element: T ),+
		}

		// Constructors.

		impl<T> $Point<T> {
			
			/// Create a new point.
			pub fn new($( $element: T ),+) -> Self {
				Self { $( $element: $element ),+ }
			}

		}

		impl_construct_trait!(impl Splat for $Point { $( $element ),+ });

		// Number.

		impl_number_trait!(impl Zero for $Point { $( $element ),+ });
		impl_number_trait!(impl One for $Point { $( $element ),+ });

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

	use num_traits::{ Zero, One };

	use crate::traits::*;
	use crate::point::*;

	macro_rules! test_point {

		($name:ident, $Point:ident { $( $field:ident:$value:literal ),+ }) => {
			
			#[test]
			fn $name() {
				
				{
					
					let p1 = $Point::new($( $value ),+);
					let p2 = $Point::splat(123.456);
					let p3 = $Point::<f32>::zero();
					let p4 = $Point::<f32>::one();

					$(
						assert_eq!(p1.$field, $value);
						assert_eq!(p2.$field, 123.456);
						assert_eq!(p3.$field, 0.0);
						assert_eq!(p4.$field, 1.0);
					)+

				}

				{
					let p = $Point::new($( $value ),+);

					$( assert_eq!(p.$field, $value); )+
					
					let mut add = p + p;
					let mut sub = p - p;
					let mut mul = p * p;
					let mut div = p / p;
					let mut rem = p % p;
					
					$(
						assert_eq!(add.$field, $value + $value);
						assert_eq!(sub.$field, $value - $value);
						assert_eq!(mul.$field, $value * $value);
						assert_eq!(div.$field, $value / $value);
						assert_eq!(rem.$field, $value % $value);
					)+

					add += p;
					sub -= p;
					mul *= p;
					div /= p;
					rem %= p;
					
					$(
						assert_eq!(add.$field, $value + $value + $value);
						assert_eq!(sub.$field, $value - $value - $value);
						assert_eq!(mul.$field, $value * $value * $value);
						assert_eq!(div.$field, $value / $value / $value);
						assert_eq!(rem.$field, $value % $value % $value);
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

