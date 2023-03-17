# `#[derive(Map)]`

In this assignment, you will be writing a procedural macro for automatically
generating a trait implementation for certain Rust structs! Details follow.

## Specification of `Map`

```rust
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
```

Note that this trait is less flexible than the usual
`fn map<T, U>(self, f: impl FnMut(T) -> U)`, because we don't yet have a way to
express "polymorphic `Self`" in the manner we desire, and other alternatives
are messy. However, this flexibility lets us implement `Map` for more types, so
this is ok.

### Usage

```rust
#[derive(PartialEq, Debug)]
struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

/// `x`, `y`, and `z` could be collections, so we should be able to map over
/// the subcollections too!
impl<T, U> Map for Vec3<T> where T: Map<Item=U> {
    type Item = U;
    fn map_inplace(&mut self, f: &mut impl FnMut(&mut Self::Item)) {
        self.x.map_inplace(f);
        self.y.map_inplace(f);
        self.z.map_inplace(f);
    }

    fn map(self, f: &mut impl FnMut(Self::Item) -> Self::Item) -> Self
        where
            Self: Sized {
        Self {
            x: self.x.map(f),
            y: self.y.map(f),
            z: self.z.map(f),
        }
    }
}

/// Generate a default implementation for floats
impl Map for f32 {
    type Item = f32;
    fn map_inplace(&mut self, f: &mut impl FnMut(&mut Self::Item)) {
        f(self)
    }
    
    fn map(self, f: &mut impl FnMut(Self::Item) -> Self::Item) -> Self {
        f(self)
    }
}

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
```

like `PartialEq` and `Debug`, the `Map` implementation is pretty rote and can
get hairy for larger types. So, let's make a proc macro for it!

## Specification of `#[derive(Map)]`

Given a `Map` trait to implement, generate an implementation that looks like
the above example for `Vec3<T>`; that is, an implementation with the
appropriate type bounds, and whose functions call `map_inplace` and `map`
sequentially on the struct fields in the order they were declared. You do not
have to support tuple structs, enums, or unions (fun challenges, ordered from
least to most difficult).

As a convenience, there are default implementations of `Map` for all integer
literals and the `Vec` standard library type.

## Shell Code

We have provided you with:
* A Cargo workspace with a `proc-macro` crate and a testing crate
* Code for a `#[derive(Map)]` proc macro that does nothing, with the correct
  type signature
* Code to test your implementation. This will probably be "does the test crate
  compile at all" since the traits will need to get implemented by the proc
  macro.

## Turning In

Submit homework on Gradescope (TODO: link)
