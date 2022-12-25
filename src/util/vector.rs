use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct Vector<const N: usize> {
    pub coords: [i32; N],
}

#[allow(dead_code)]
impl<const N: usize> Vector<N> {
    #[inline]
    pub const fn zero() -> Self {
        Self { coords: [0i32; N] }
    }

    #[inline]
    pub fn abs(mut self) -> Self {
        self.map_in_place(|x| x.abs());
        self
    }

    #[inline]
    pub fn manhattan_distance(&self) -> i32 {
        self.coords.iter().map(|x| x.abs()).sum()
    }

    #[inline]
    pub fn for_each(&mut self, f: impl FnMut(&mut i32)) {
        self.coords.iter_mut().for_each(f);
    }

    #[inline]
    pub fn map_in_place(&mut self, mut f: impl FnMut(&i32) -> i32) {
        self.for_each(|x| *x = f(x))
    }

    #[inline]
    pub fn map(&self, f: impl FnMut(&i32) -> i32) -> Self {
        Self::from_iter(self.coords.iter().map(f))
    }

    #[inline]
    pub fn zip_in_place(&mut self, other: &Vector<N>, mut f: impl FnMut(&mut i32, &i32)) {
        self.coords
            .iter_mut()
            .zip(other.coords.iter())
            .for_each(|(x, y)| f(x, y))
    }

    #[inline]
    pub fn zip_with(&self, other: &Vector<N>, mut f: impl FnMut(&i32, &i32) -> i32) -> Self {
        Self::from_iter(
            self.coords
                .iter()
                .zip(other.coords.iter())
                .map(|(x, y)| f(x, y)),
        )
    }

    fn from_iter(iter: impl Iterator<Item = i32>) -> Self {
        let mut coords = [0i32; N];
        coords
            .iter_mut()
            .zip(iter)
            .for_each(|(dest, src)| *dest = src);
        coords.into()
    }
}

impl<const N: usize> Default for Vector<N> {
    fn default() -> Self {
        Self::zero()
    }
}

impl<const N: usize> Display for Vector<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for (i, coord) in self.coords.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", coord)?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl<const N: usize> Debug for Vector<N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut f = f.debug_tuple(&format!("Vector<{}>", N));
        for coord in self.coords.iter() {
            f.field(coord);
        }
        f.finish()
    }
}

impl<const N: usize> From<[i32; N]> for Vector<N> {
    fn from(coords: [i32; N]) -> Self {
        Self { coords }
    }
}

impl<const N: usize> From<Vector<N>> for [i32; N] {
    fn from(vector: Vector<N>) -> Self {
        vector.coords
    }
}

impl<const N: usize> Add for Vector<N> {
    type Output = Self;

    fn add(self, other: Vector<N>) -> Self {
        self.zip_with(&other, |x, y| x + y)
    }
}

impl<const N: usize> Sub for Vector<N> {
    type Output = Self;

    fn sub(self: Vector<N>, other: Vector<N>) -> Self {
        self.zip_with(&other, |x, y| x - y)
    }
}

impl<const N: usize> Neg for Vector<N> {
    type Output = Self;

    fn neg(self) -> Self {
        self.map(|&x| -x)
    }
}

impl<const N: usize> AddAssign for Vector<N> {
    fn add_assign(&mut self, other: Self) {
        self.zip_in_place(&other, |x, y| x.add_assign(y));
    }
}

impl<const N: usize> SubAssign for Vector<N> {
    fn sub_assign(&mut self, other: Self) {
        self.zip_in_place(&other, |x, y| x.sub_assign(y));
    }
}

impl<const N: usize> Mul<i32> for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        self.map(|x| x * rhs)
    }
}

impl<const N: usize> MulAssign<i32> for Vector<N> {
    fn mul_assign(&mut self, rhs: i32) {
        self.for_each(|x| x.mul_assign(rhs));
    }
}

impl<const N: usize> Div<i32> for Vector<N> {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        self.map(|x| x / rhs)
    }
}

impl<const N: usize> DivAssign<i32> for Vector<N> {
    fn div_assign(&mut self, rhs: i32) {
        self.for_each(|x| x.div_assign(rhs));
    }
}

pub type Vector2D = Vector<2>;

#[allow(dead_code)]
impl Vector2D {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { coords: [x, y] }
    }

    #[inline]
    pub fn x(&self) -> i32 {
        self.coords[0]
    }

    #[inline]
    pub fn x_mut(&mut self) -> &mut i32 {
        &mut self.coords[0]
    }

    #[inline]
    pub fn y(&self) -> i32 {
        self.coords[1]
    }

    #[inline]
    pub fn y_mut(&mut self) -> &mut i32 {
        &mut self.coords[1]
    }

    pub fn neighbours(self) -> impl Iterator<Item = Self> {
        [
            self + Vector2D::new(0, -1),
            self + Vector2D::new(-1, 0),
            self + Vector2D::new(1, 0),
            self + Vector2D::new(0, 1),
        ]
        .into_iter()
    }

    pub fn neighbours_diagonal(self) -> impl Iterator<Item = Self> {
        [
            self + Vector2D::new(-1, -1),
            self + Vector2D::new(0, -1),
            self + Vector2D::new(1, -1),
            self + Vector2D::new(-1, 0),
            self + Vector2D::new(1, 0),
            self + Vector2D::new(-1, 1),
            self + Vector2D::new(0, 1),
            self + Vector2D::new(1, 1),
        ]
        .into_iter()
    }
}

pub type Vector3D = Vector<3>;

#[allow(dead_code)]
impl Vector3D {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { coords: [x, y, z] }
    }

    #[inline]
    pub fn x(&self) -> i32 {
        self.coords[0]
    }

    #[inline]
    pub fn y(&self) -> i32 {
        self.coords[1]
    }

    #[inline]
    pub fn z(&self) -> i32 {
        self.coords[2]
    }

    pub fn cross_product(self, other: Vector3D) -> Vector3D {
        // https://en.wikipedia.org/wiki/Cross_product
        Vector3D::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn neighbours(self) -> impl Iterator<Item = Self> {
        [
            self + Vector3D::new(-1, 0, 0),
            self + Vector3D::new(1, 0, 0),
            self + Vector3D::new(0, -1, 0),
            self + Vector3D::new(0, 1, 0),
            self + Vector3D::new(0, 0, -1),
            self + Vector3D::new(0, 0, 1),
        ]
        .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod vector2d {
        use super::*;

        #[test]
        fn test_new() {
            assert_eq!(Vector2D::new(3, 4), Vector2D::from([3, 4]));
        }

        #[test]
        fn test_abs() {
            assert_eq!(Vector2D::new(3, -4).abs(), Vector2D::from([3, 4]));
        }

        #[test]
        fn test_add() {
            assert_eq!(
                Vector2D::new(3, 4) + Vector2D::new(5, 10),
                Vector2D::from([8, 14])
            );
            assert_eq!(
                Vector2D::new(3, 4) + Vector2D::zero(),
                Vector2D::from([3, 4])
            );
        }

        #[test]
        fn test_sub() {
            assert_eq!(
                Vector2D::new(3, 4) - Vector2D::new(5, 10),
                Vector2D::from([-2, -6])
            );
            assert_eq!(
                Vector2D::new(3, 4) - Vector2D::zero(),
                Vector2D::from([3, 4])
            );
        }

        #[test]
        fn test_neg() {
            assert_eq!(-Vector2D::new(3, 4), Vector2D::from([-3, -4]));
        }

        #[test]
        fn test_add_assign() {
            let mut vector = Vector2D::new(3, 4);
            vector += Vector2D::new(10, 20);
            assert_eq!(vector, Vector2D::from([13, 24]));
        }

        #[test]
        fn test_sub_assign() {
            let mut vector = Vector2D::new(3, 4);
            vector -= Vector2D::new(10, 20);
            assert_eq!(vector, Vector2D::from([-7, -16]));
        }

        #[test]
        fn test_mul() {
            assert_eq!(Vector2D::new(3, 4) * 2, Vector2D::from([6, 8]));
        }

        #[test]
        fn test_mul_assign() {
            let mut vector = Vector2D::new(3, 4);
            vector *= 3;
            assert_eq!(vector, Vector2D::from([9, 12]));
        }

        #[test]
        fn test_display() {
            assert_eq!(format!("{}", Vector2D::new(3, 4)), "(3, 4)");
        }
    }

    mod vector3d {
        use super::*;

        #[test]
        fn test_new() {
            assert_eq!(Vector3D::new(3, 4, 5), Vector3D::from([3, 4, 5]));
        }

        #[test]
        fn test_abs() {
            assert_eq!(Vector3D::new(3, -4, -5).abs(), Vector3D::from([3, 4, 5]));
        }

        #[test]
        fn test_add() {
            assert_eq!(
                Vector3D::new(3, 4, 5) + Vector3D::new(5, 10, 15),
                Vector3D::from([8, 14, 20])
            );
            assert_eq!(
                Vector3D::new(3, 4, 5) + Vector3D::zero(),
                Vector3D::from([3, 4, 5])
            );
        }

        #[test]
        fn test_sub() {
            assert_eq!(
                Vector3D::new(3, 4, 5) - Vector3D::new(5, 10, 15),
                Vector3D::from([-2, -6, -10])
            );
            assert_eq!(
                Vector3D::new(3, 4, 5) - Vector3D::zero(),
                Vector3D::from([3, 4, 5])
            );
        }

        #[test]
        fn test_add_assign() {
            let mut vector = Vector3D::new(3, 4, 5);
            vector += Vector3D::new(10, 20, 30);
            assert_eq!(vector, Vector3D::from([13, 24, 35]));
        }

        #[test]
        fn test_sub_assign() {
            let mut vector = Vector3D::new(3, 4, 5);
            vector -= Vector3D::new(10, 20, 30);
            assert_eq!(vector, Vector3D::from([-7, -16, -25]));
        }

        #[test]
        fn test_mul() {
            assert_eq!(Vector3D::new(3, 4, 5) * 2, Vector3D::from([6, 8, 10]));
        }

        #[test]
        fn test_mul_assign() {
            let mut vector = Vector3D::new(3, 4, 5);
            vector *= 3;
            assert_eq!(vector, Vector3D::from([9, 12, 15]));
        }

        #[test]
        fn test_display() {
            assert_eq!(format!("{}", Vector3D::new(3, 4, 5)), "(3, 4, 5)");
        }

        #[test]
        fn test_debug() {
            assert_eq!(
                format!("{:?}", Vector3D::new(3, 4, 5)),
                "Vector<3>(3, 4, 5)"
            );
        }
    }
}
