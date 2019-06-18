#[macro_use]
mod macros;

mod point;


pub use point::{ Point2, Point3 };


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
