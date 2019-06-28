#![macro_use]

macro_rules! impl_construct_trait {
	
	(impl Splat for $Struct:ident { $( $field:ident ),+ }) => {
	
		impl<T> Splat<T> for $Struct<T> where T: Copy {

			fn splat(value: T) -> Self {
				Self { $( $field: value ),+ }
			}

		}

	};

}

macro_rules! impl_number_trait {

	(impl Zero for $Struct:ident { $( $field:ident ),+ }) => {
		
		impl<T> Zero for $Struct<T> where T: Zero + PartialEq {
			
			fn zero() -> Self {
				Self { $( $field: T::zero() ),+ }
			}
			fn is_zero(&self) -> bool {
				*self == Self::zero()
			}
			fn set_zero(&mut self) {
				$( self.$field = T::zero(); )+
			}

		}
	
	};
	
	(impl One for $Struct:ident { $( $field:ident ),+ }) => {
		
		impl<T> One for $Struct<T> where T: One {
			
			fn one() -> Self {
				Self { $( $field: T::one() ),+ }
			}
			fn set_one(&mut self) {
				$( self.$field = T::one(); )+
			}
		
		}
	
	};

}
