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

	use super::{ Point2, Point3 };

	#[test]
	fn point2() {

		{
			let mut point2 = Point2::new(1.1, 2.2);
		
			assert_eq!(point2.x, 1.1);
			assert_eq!(point2.y, 2.2);

			point2.x = 123.456;
			point2.y = 456.789;

			assert_eq!(point2.x, 123.456);
			assert_eq!(point2.y, 456.789);
		}
		{
			let point2 = Point2::new(1.2, 3.4);
			let point2_other = Point2::new(2.4, 6.8);

			let add = point2 + point2_other;
			let sub = point2 - point2_other;
			let mul = point2 * point2_other;
			let div = point2 / point2_other;
			let rem = point2 % point2_other;
			
			assert_eq!(add.x, 1.2 + 2.4);
			assert_eq!(add.y, 3.4 + 6.8);
		}

	}

	#[test]
	fn point3() {

		let mut point3 = Point3::new(1.1, 2.2, 3.3);
		
		assert_eq!(point3.x, 1.1);
		assert_eq!(point3.y, 2.2);
		assert_eq!(point3.z, 3.3);

		point3.x = 123.456;
		point3.y = 456.789;
		point3.z = 789.101;

		assert_eq!(point3.x, 123.456);
		assert_eq!(point3.y, 456.789);
		assert_eq!(point3.z, 789.101);

	}

}

