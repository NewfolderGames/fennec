use num_traits::{ Zero, One };

pub trait Splat<T> where T: Copy {
	fn splat(value: T) -> Self; 
}

pub trait Lerp<T>: Sized{

	fn lerp(self, other: Self, value: T) -> Self;
	fn lerp01(self, other: Self, value: T) -> Self where T: Clamp<T> + Zero + One {
		self.lerp(other, value.clamp01())
	}

}

pub trait Clamp<T>: Sized {

	fn clamp(self, min: T, max: T) -> Self;
	fn clamp01(self) -> Self where T: Zero + One {
		self.clamp(T::zero(), T::one())
	}

}

