pub use derive_map_proc::*;

pub trait Map {
    type Item;
    /// Do a map operation, calling `f` on all values of type `Item` contained
    /// in `self` and replacing them in-place
    fn map_inplace(&mut self, f: &mut impl FnMut(&mut Self::Item));

    /// Map operation that takes ownership during the map
    fn map(self, f: impl FnMut(Self::Item) -> Self::Item) -> Self
    where
        Self: Sized;
}

impl<T> Map for Vec<T> {
    type Item = T;
    fn map_inplace(&mut self, f: &mut impl FnMut(&mut Self::Item)) {
        self.iter_mut().for_each(f)
    }

    fn map(self, f: impl FnMut(Self::Item) -> Self::Item) -> Self {
        self.into_iter().map(f).collect()
    }
}

trait Simple: Copy {}

macro_rules! simple {
    () => {};
    ($ty:ty, $($tys:ty,)*) => {
        impl Simple for $ty {}
        simple!{$($tys,)*}
    };
}

simple! {
    /* signed integer types */
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    /* unsigned integer types */
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    /* floating point types */
    f32,
    f64,
    /* misc */
    char,
    (),
}

impl<T: Simple> Map for T {
    type Item = T;
    fn map_inplace(&mut self, f: &mut impl FnMut(&mut Self::Item)) {
        f(self);
    }

    fn map(self, mut f: impl FnMut(Self::Item) -> Self::Item) -> Self {
        f(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Map, PartialEq, Debug)]
    struct Vec3<T> {
        x: T,
        y: T,
        z: T,
    }

    #[test]
    fn test_simple_vec3_map_inplace() {
        let mut v = Vec3 {
            x: 1.0f32,
            y: 2.0f32,
            z: 3.0f32,
        };
        Map::map_inplace(&mut v, &mut |a| *a = *a * 2.0f32);
        let v2 = Vec3 {
            x: 2.0f32,
            y: 4.0f32,
            z: 6.0f32,
        };
        assert_eq!(v, v2);
    }

    #[test]
    fn test_simple_vec3_map() {
        let mut v = Vec3 {
            x: 1.0f32,
            y: 2.0f32,
            z: 3.0f32,
        };
        let v = Map::map(v, |a| a * 2.0f32);
        let v2 = Vec3 {
            x: 2.0f32,
            y: 4.0f32,
            z: 6.0f32,
        };
        assert_eq!(v, v2);
    }

    #[derive(Map, PartialEq, Debug)]
    struct VecPair<T>(Vec<T>, Vec<T>);

    #[test]
    fn test_pair_vec_map_inplace() {
        let mut v = VecPair(vec![0i32, 2i32], vec![5i32]);
        v.map_inplace(&mut |f| *f = *f / 2);
        let v2 = VecPair(vec![0i32, 1i32], vec![2i32]);
        assert_eq!(v, v2);
    }

    #[test]
    fn test_pair_vec_map() {
        let mut v = VecPair(vec![0i32, 2i32], vec![5i32]);
        let v = v.map(|f| f / 2);
        let v2 = VecPair(vec![0i32, 1i32], vec![2i32]);
        assert_eq!(v, v2);
    }
}
