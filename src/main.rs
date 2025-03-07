use std::{
    default,
    ops::{AddAssign, Deref, Mul, MulAssign},
};

pub struct Vector<T>(Box<Vec<T>>);

impl<T> Vector<T>
where
    T: AddAssign<T> + Mul<T, Output = T> + Copy + Default + Deref,
{
    pub fn with2<P: AddAssign<T> + Mul<T, Output = T> + Copy + Default + Deref>(
        x: P,
        y: P,
    ) -> Vector<P> {
        return Vector::<P>(Box::new(vec![x, y]));
    }

    pub fn with3<P: AddAssign<T> + Mul<T, Output = T> + Copy + Default + Deref>(
        x: P,
        y: P,
        z: P,
    ) -> Vector<P> {
        return Vector::<P>(Box::new(vec![x, y, z]));
    }

    pub fn add(&mut self, scalar: T) {
        for idx in 0..self.len() {
            if let Some(component) = self.0.get_mut(idx) {
                *component += scalar;
            }
        }
    }

    pub fn dot(&self, b: Vector<T>) -> T {
        let mut product: T = T::default();
        if self.len() != b.len() {
            panic!("The vector lengths are incompatible!");
        }
        for idx in 0..self.len() {
            let c1 = self.0.get(idx);
            let c2 = b.0.get(idx);
            let c1_val = *c1.unwrap_or(&T::default());
            let c2_val = *c2.unwrap_or(&T::default());
            product += c1_val * c2_val;
        }
        return product;
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }
}

fn main() {
    println!("Hello, world!");
    let a = &Vector::with2(5.0, 5.0);
}
