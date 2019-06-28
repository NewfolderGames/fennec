#![macro_use]

/// Implements a binary operator.
macro_rules! impl_binary_op {
	
	(impl $Ops:ident for $Struct:ident { $method:ident({ $( $field:ident ),+ }) }) => {
		
		impl<T> $Ops for $Struct<T> where T: $Ops<Output = T> {

			type Output = Self;
			fn $method(self, other: Self) -> Self::Output {
				Self::new($( self.$field.$method(other.$field) ),+)
			}

		}
		
		impl<T> $Ops<T> for $Struct<T> where T: $Ops<Output = T> + Copy {

			type Output = Self;
			fn $method(self, other: T) -> Self::Output {
				Self::new($( self.$field.$method(other) ),+)
			}

		}

	};

}

/// Implements a assign operator.
macro_rules! impl_assign_op {
	
	(impl $Ops:ident for $Struct:ident { $method:ident({ $( $field:ident ),+ }) }) => {
	
		impl<T> $Ops for $Struct<T> where T: $Ops<T> {

			fn $method(&mut self, other: Self) {
				$( self.$field.$method(other.$field); )+
			}

		}

		impl<T> $Ops<T> for $Struct<T> where T: $Ops<T> + Copy {

			fn $method(&mut self, other: T) {
				$( self.$field.$method(other); )+
			}

		}

	};

}

/// Implements a unary operator.
macro_rules! impl_unary_op {

	(impl $Ops:ident for $Struct:ident { $method:ident({ $( $field:ident ),+ }) }) => {
		
		impl<T> $Ops for $Struct<T> where T: $Ops<Output = T> {

			type Output = Self;
			fn $method(self) -> Self::Output {
				Self::new($( self.$field.$method() ),+)
			}

		}

	};

}
