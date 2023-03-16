pub use derive_map_proc::*;

pub trait Map {
    type Item;
    /// Do a map operation, calling `f` on all values of type `Item` contained
    /// in `self` and replacing them in-place
    fn map_inplace(&mut self, f: &mut impl FnMut(&mut Self::Item));

    /*
    /// Map operation that takes ownership during the map
    fn map(self, f: impl FnMut(Self::Item) -> Self::Item) -> Self
    where
        Self: Sized;
    */
}

impl<T> Map for Vec<T> {
    type Item = T;
    fn map_inplace(&mut self, f: &mut impl FnMut(&mut Self::Item)) {
        self.iter_mut().for_each(f)
    }

    /*
    fn map(self, f: impl FnMut(Self::Item) -> Self::Item) -> Self {
        todo!()
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(PartialEq, Debug)]
    struct Vec3 {
        x: f32,
        y: f32,
        z: f32,
    }

    impl Map for Vec3 {
        type Item = f32;
        fn map_inplace(&mut self, f: &mut impl FnMut(&mut f32)) {
            f(&mut self.x);
            f(&mut self.y);
            f(&mut self.z);
        }
    }

    #[test]
    fn test_simple_vec3_map() {
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

    #[derive(Map, PartialEq, Debug)]
    struct VecPair<T>(Vec<T>, Vec<T>);

    #[test]
    fn test_pair_vec_map() {
        let mut v = VecPair(vec![0i32, 2i32], vec![5i32]);
        v.map_inplace(&mut |f| *f = *f / 2);
        let v2 = VecPair(vec![0i32, 1i32], vec![2i32]);
        assert_eq!(v, v2);
    }
}
