
macro_rules! impl_function {
	($name:ident, { $( $field:ident ),+ }) => {

		fn $name(self) -> Self {
			Self::new($( self.$field.$name() ),+)
		}

	};
}

