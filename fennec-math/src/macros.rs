
macro_rules! impl_field_function {

	($name:ident, { $( $field:ident ),+ }) => {

		fn $name(self) -> Self {
			Self { $( $field: self.$field.$name() ),+ }
		}

	};

}

//macro_rules! impl_field_operator {


//}
