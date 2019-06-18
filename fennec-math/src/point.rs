pub trait Point {

	fn abs(self) -> Self;
	fn ceil(self) -> Self;
	fn floor(self) -> Self;
	fn fract(self) -> Self;
	fn round(self) -> Self;
	fn trunc(self) -> Self;

}

macro_rules! make_point {
	($struct:ident, { $( $field:ident ),+ } ) => {
		
		pub struct $struct {
			$( pub $field: f32, )+
		}

		impl $struct {
			
			pub fn new($( $field: f32 ),+) -> Self {
				Self { $( $field: $field ),+ }
			}

		}

		impl Point for $struct {

			impl_field_function!(abs, { $( $field ),+ });
			impl_field_function!(ceil, { $( $field ),+ });
			impl_field_function!(floor, { $( $field ),+ });
			impl_field_function!(fract, { $( $field ),+ });
			impl_field_function!(round, { $( $field ),+ });
			impl_field_function!(trunc, { $( $field ),+ });

		}

	}
}

make_point!(Point2, { x, y });
make_point!(Point3, { x, y, z });



#[cfg(test)]
mod tests {

	use super::{ Point2, Point3 };

	#[test]
	fn point2_0() {

		let mut point2 = Point2::new(1.1, 2.2);
		
		assert_eq!(point2.x, 1.1);
		assert_eq!(point2.y, 2.2);

		point2.x = 123.456;
		point2.y = 456.789;

		assert_eq!(point2.x, 123.456);
		assert_eq!(point2.y, 456.789);

	}

	#[test]
	fn point3_0() {

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

