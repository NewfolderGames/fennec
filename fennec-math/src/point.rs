/// Generates point structs.
macro_rules! generate_point {

	($struct:ident, { $( $field:ident ),+ } ) => {
		
		pub struct $struct<T = f32> {
			$( pub $field: T, )+
		}

		impl<T> $struct<T> {
			
			pub fn new($( $field: T ),+) -> Self {
				Self { $( $field: $field ),+ }
			}

		}

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

		let mut point2 = Point2::new(1.1, 2.2);
		
		assert_eq!(point2.x, 1.1);
		assert_eq!(point2.y, 2.2);

		point2.x = 123.456;
		point2.y = 456.789;

		assert_eq!(point2.x, 123.456);
		assert_eq!(point2.y, 456.789);
	
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

