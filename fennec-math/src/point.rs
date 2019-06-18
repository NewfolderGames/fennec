use std::ops::*;

// Macros.

macro_rules! impl_operator_self {
	
	($ops:ident, $method:ident, $operator:tt, $struct:ident, { $( $field:ident ),+ }) => {
	
		impl<T: $ops<Output = T>> $ops for $struct<T> {

			type Output = Self;
			fn $method(self, other: Self) -> Self::Output {
				Self::new($( self.$field $operator other.$field ),+)
			}

		}

	};

}

/// Generates point structs.
macro_rules! generate_point {

	($struct:ident, { $( $field:ident ),+ } ) => {
		
		#[derive(Copy, Clone)]
		pub struct $struct<T = f32> {
			$( pub $field: T, )+
		}

		impl<T> $struct<T> {
			
			pub fn new($( $field: T ),+) -> Self {
				Self { $( $field: $field ),+ }
			}

		}

		impl_operator_self!(Add, add, +, $struct, { $( $field ),+ });
		impl_operator_self!(Sub, sub, -, $struct, { $( $field ),+ });
		impl_operator_self!(Mul, mul, *, $struct, { $( $field ),+ });
		impl_operator_self!(Div, div, /, $struct, { $( $field ),+ });
		impl_operator_self!(Rem, rem, %, $struct, { $( $field ),+ });

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

