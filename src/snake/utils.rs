use std::ops::{Index, IndexMut};

/// Simple two-dimensional array implementation
pub struct TwoDimensionalMap<T: Default + Clone> {
    width: usize,
    height: usize,
    vals: Vec<T>,
}

impl <T: Default + Clone> Index<(usize, usize)> for TwoDimensionalMap<T> {

    type Output = T;

    fn index(&self, coords: (usize, usize)) -> &T {
        let x = coords.0;
        let y = coords.1;

        assert!(x < self.width);
        assert!(y < self.height);
        &self.vals[y * self.height + x]
    }

}

impl <T: Default + Clone> IndexMut<(usize, usize)> for TwoDimensionalMap<T> {

    fn index_mut(&mut self, coords: (usize, usize)) -> &mut T {
        let x = coords.0;
        let y = coords.1;

        assert!(x < self.width);
        assert!(y < self.height);
        &mut self.vals[y * self.height + x]
    }

}

impl <T: Default + Clone> TwoDimensionalMap<T> {

    pub fn new(width: usize, height: usize) -> TwoDimensionalMap<T>
    {
        TwoDimensionalMap {
            width: width,
            height: height,
            vals: vec![Default::default(); width * height]
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_be_able_to_add_values() {
        let mut arr = TwoDimensionalMap::<i32>::new(30, 30);

        arr[(25, 20)] = 15;
        arr[(13, 9)]  = -12;

        assert_eq!(arr[(25, 20)], 15);
        assert_eq!(arr[(13, 9)], -12);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_out_of_bounds_requested_x() {
        let mut arr = TwoDimensionalMap::<i32>::new(30, 30);

        arr[(31, 15)];
    }

    #[test]
    #[should_panic]
    fn should_panic_when_out_of_bounds_requested_y() {
        let mut arr = TwoDimensionalMap::<i32>::new(30, 30);

        arr[(15, 31)];
    }

}
