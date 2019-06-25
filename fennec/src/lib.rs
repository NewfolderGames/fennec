extern crate fennec_math as math;

#[cfg(test)]
mod tests {
	
	use math::{ Point2 };

	#[test]
	fn math() {
	
		let point = Point2::new(1.2, 3.4);

		assert_eq!(point.x, 1.2);
		assert_eq!(point.y, 3.4);

	}

}
