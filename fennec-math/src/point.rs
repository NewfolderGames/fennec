use std::ops::*;

// Macros.

/// Generates point structs.
macro_rules! generate_point {

	($Point:ident, { $( $element:ident ),+ } ) => {
		
		#[derive(Copy, Clone)]
		pub struct $Point<T = f32> {
			$( pub $element: T, )+
		}

		impl<T> $Point<T> {
			
			pub fn new($( $element: T ),+) -> Self {
				Self { $( $element: $element ),+ }
			}

		}

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



// Make point structs.

generate_point!(Point1, { x });
generate_point!(Point2, { x, y });
generate_point!(Point3, { x, y, z });
generate_point!(Point4, { x, y, z, w });



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
						
					let add = p + p;
					let sub = p - p;
					let mul = p * p;
					let div = p / p;
					let rem = p % p;
						
					$(
						assert_eq!(add.$field, $input + $input);
						assert_eq!(sub.$field, $input - $input);
						assert_eq!(mul.$field, $input * $input);
						assert_eq!(div.$field, $input / $input);
						assert_eq!(rem.$field, $input % $input);
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

