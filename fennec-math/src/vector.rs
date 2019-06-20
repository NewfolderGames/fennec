use std::ops::*;
use num_traits::{ Zero, One };

// Macros.

/// Generates a vector struct.
macro_rules! generate_vector {
	
	($Vector:ident { $( $element:ident ),+ }) => {

		// Struct.

		#[derive(Clone, Copy, Debug)]
		pub struct $Vector<T = f32> {
			$( pub $element: T ),+
		}

		// Base functions.

		impl<T> $Vector<T> {
			
			pub fn new($( $element:T ),+) -> Self {
				Self { $( $element: $element ),+ }
			}

		}

		impl<T> $Vector<T> where T: Copy {

			/// Create a new vector with all its elements set to input value.
			pub fn splat(value: T) -> Self {
				Self { $( $element: value ),+ }
			}

		}

		impl<T> $Vector<T> where T: Zero {

			/// Create a new vector with all its elements set to zero.
			pub fn zero() -> Self {
				Self { $( $element: T::zero() ),+ }
			}

		}

		impl<T> $Vector<T> where T: One {

			/// Create a new vector with all its elements set to one.
			pub fn one() -> Self {
				Self { $( $element: T::one() ),+ }
			}

		}

		// Operators.

		impl_binary_op!(impl Add for $Vector { add({ $( $element ),+ }) });
		impl_binary_op!(impl Sub for $Vector { sub({ $( $element ),+ }) });
		impl_binary_op!(impl Mul for $Vector { mul({ $( $element ),+ }) });
		impl_binary_op!(impl Div for $Vector { div({ $( $element ),+ }) });
		impl_binary_op!(impl Rem for $Vector { rem({ $( $element ),+ }) });

		impl_assign_op!(impl AddAssign for $Vector { add_assign({ $( $element ),+ }) });
		impl_assign_op!(impl SubAssign for $Vector { sub_assign({ $( $element ),+ }) });
		impl_assign_op!(impl MulAssign for $Vector { mul_assign({ $( $element ),+ }) });
		impl_assign_op!(impl DivAssign for $Vector { div_assign({ $( $element ),+ }) });
		impl_assign_op!(impl RemAssign for $Vector { rem_assign({ $( $element ),+ }) });

		impl_unary_op!(impl Neg for $Vector { neg({ $( $element ),+ }) });

	};

}



// Make vector structs.

generate_vector!(Vector1 { x });
generate_vector!(Vector2 { x, y });
generate_vector!(Vector3 { x, y, z });
generate_vector!(Vector4 { x, y, z, w });

