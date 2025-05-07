use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

///
/// Immutable Vector
///
#[derive(Clone)]
pub struct VecF {
    elements: Vec<f64>,
}

impl VecF {
    pub fn as2(x: f64, y: f64) -> Self {
        return VecF::new(vec![x, y]);
    }

    pub fn as3(x: f64, y: f64, z: f64) -> Self {
        return VecF::new(vec![x, y, z]);
    }

    pub fn new(elements: Vec<f64>) -> Self {
        return Self { elements };
    }

    pub fn dot(a: &Self, b: &Self) -> f64 {
        return join_vecs(&a.elements, &b.elements, |a, b| a * b)
            .iter()
            .fold(f64::default(), |acc, a| acc + a);
    }

    pub fn dims(&self) -> usize {
        return self.elements.len();
    }

    pub fn len(&self) -> f64 {
        return self
            .elements
            .iter()
            .fold(0., |acc, a| acc + a.powf(2.))
            .sqrt();
    }

    pub fn normalize(&self) -> Self {
        let l = self.len();
        return self / l;
    }
}

impl Display for VecF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return f.write_fmt(format_args!("Vec_f64 {:#?}", self.elements));
    }
}

impl Add for &VecF {
    type Output = VecF;
    fn add(self, rhs: Self) -> Self::Output {
        return VecF {
            elements: join_vecs(&self.elements, &rhs.elements, |a, b| a + b),
        };
    }
}

impl Sub for &VecF {
    type Output = VecF;
    fn sub(self, rhs: Self) -> Self::Output {
        return VecF {
            elements: join_vecs(&self.elements, &rhs.elements, |a, b| a - b),
        };
    }
}

impl Mul<f64> for &VecF {
    type Output = VecF;

    fn mul(self, rhs: f64) -> Self::Output {
        return VecF::new(self.elements.iter().map(|a| a * rhs).collect());
    }
}

impl Div<f64> for &VecF {
    type Output = VecF;

    fn div(self, rhs: f64) -> Self::Output {
        return VecF::new(self.elements.iter().map(|a| a / rhs).collect());
    }
}

fn create_empty_vec<T: Default>(size: usize) -> Vec<T> {
    return (0..size).map(|_| T::default()).collect();
}

fn join_vecs<T, F>(a: &Vec<T>, b: &Vec<T>, join_fn: F) -> Vec<T>
where
    T: Default,
    F: Fn(&T, &T) -> T,
{
    if a.len() != b.len() {
        panic!("Dimension mismatch!");
    }
    let mut out = create_empty_vec::<T>(a.len());
    for idx in 0..a.len() {
        out[idx] = join_fn(&a[idx], &b[idx]);
    }
    return out;
}
