
macro_rules! impl_binary_op {
	
	(impl $ops:ident for $struct:ident { $method:ident({ $( $field:ident ),+ }) }) => {
		
		impl<T> $ops for $struct<T> where T: $ops<Output = T> {

			type Output = Self;
			fn $method(self, other: Self) -> Self::Output {
				Self::new($( self.$field.$method(other.$field) ),+)
			}

		}

	};

}

macro_rules! impl_assign_op {
	
	(impl $ops:ident for $struct:ident { $method:ident({ $( $field:ident ),+ }) }) => {
	
		impl<T> $ops for $struct<T> where T: $ops<T> {

			fn $method(&mut self, other: Self) {
				$( self.$field.$method(other.$field); )+
			}

		}

	};

}

macro_rules! impl_unary_op {

	(impl $ops:ident for $struct:ident { $method:ident({ $( $field:ident ),+ }) }) => {
		
		impl<T> $ops for $struct<T> where T: $ops<Output = T> {

			type Output = Self;
			fn $method(self) -> Self::Output {
				Self::new($( self.$field.$method() ),+)
			}

		}

	};

}
