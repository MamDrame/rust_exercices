use std::ops::Add;

#[derive(Debug, Eq, PartialEq)]
pub struct Garage<T: Add<Output = T> + Copy> {
    pub left: Option<T>,
    pub right: Option<T>,
}

impl<T: Add<Output = T> + Copy> Garage<T> {
    pub fn move_to_right(&mut self) {
        if let (Some(left), Some(right)) = (self.left, self.right) {
            self.right = Some(left + right);
        }
        self.left = None;
    }

    pub fn move_to_left(&mut self) {
        if let (Some(left), Some(right)) = (self.right, self.left) {
            self.left = Some(left + right);
        }
        self.right = None;
    }
}
