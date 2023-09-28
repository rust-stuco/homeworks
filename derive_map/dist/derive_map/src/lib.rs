pub use derive_map_proc::*;

pub trait Map {
    type Item;
    /// Do a map operation, calling `f` on all values of type `Item` contained
    /// in `self` and replacing them in-place
    fn map_inplace(&mut self, f: &mut impl FnMut(&mut Self::Item));

    /// Map operation that takes ownership during the map
    fn map(self, f: &mut impl FnMut(Self::Item) -> Self::Item) -> Self
    where
        Self: Sized;
}

impl<T> Map for Vec<T> {
    type Item = T;
    fn map_inplace(&mut self, f: &mut impl FnMut(&mut Self::Item)) {
        self.iter_mut().for_each(f)
    }

    fn map(self, f: &mut impl FnMut(Self::Item) -> Self::Item) -> Self {
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

    fn map(self, f: &mut impl FnMut(Self::Item) -> Self::Item) -> Self {
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

    /*
    impl<T, U> Map for Vec3<T>
    where
        T: Map<Item = U>,
    {
        type Item = U;
        fn map_inplace(&mut self, f: &mut impl FnMut(&mut Self::Item)) {
            self.x.map_inplace(f);
            self.y.map_inplace(f);
            self.z.map_inplace(f);
        }

        fn map(self, f: &mut impl FnMut(Self::Item) -> Self::Item) -> Self
        where
            Self: Sized,
        {
            Self {
                x: self.x.map(f),
                y: self.y.map(f),
                z: self.z.map(f),
            }
        }
    }
    */

    #[test]
    fn test_simple_vec3_map_inplace() {
        let mut v = Vec3 {
            x: 1.0f32,
            y: 2.0f32,
            z: 3.0f32,
        };
        v.map_inplace(&mut |a| *a = *a * 2.0f32);
        let v2 = Vec3 {
            x: 2.0f32,
            y: 4.0f32,
            z: 6.0f32,
        };
        assert_eq!(v, v2);
    }

    #[test]
    fn test_simple_vec3_map() {
        let v = Vec3 {
            x: 1.0f32,
            y: 2.0f32,
            z: 3.0f32,
        };
        let v = v.map(&mut |a| a * 2.0f32);
        let v2 = Vec3 {
            x: 2.0f32,
            y: 4.0f32,
            z: 6.0f32,
        };
        assert_eq!(v, v2);
    }

    #[test]
    fn test_vec3_vec_map_inplace() {
        let mut v = Vec3 {
            x: vec![1.0f32],
            y: vec![2.0, 3.0],
            z: vec![4.0, 5.0, 6.0],
        };
        v.map_inplace(&mut |f| *f = *f + 0.5f32);
        let v2 = Vec3 {
            x: vec![1.5f32],
            y: vec![2.5, 3.5],
            z: vec![4.5, 5.5, 6.5],
        };
        assert_eq!(v, v2);
    }

    #[test]
    fn test_vec3_vec_map() {
        let v = Vec3 {
            x: vec![1.0f32],
            y: vec![2.0, 3.0],
            z: vec![4.0, 5.0, 6.0],
        };
        let v = v.map(&mut |f| f + 0.5f32);
        let v2 = Vec3 {
            x: vec![1.5f32],
            y: vec![2.5, 3.5],
            z: vec![4.5, 5.5, 6.5],
        };
        assert_eq!(v, v2);
    }
}
