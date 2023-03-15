# `#[derive(Map)]`

In this assignment, you will be writing a procedural macro for automatically
generating a trait implementation for certain Rust structs! Details follow.

## Specification of `Map`

```rust
trait Map<Item> {
  /// Do a map operation, calling `f` on all values of type `Item` contained
  /// in `self` and replacing them in-place
  fn map_inplace(&mut self, f: impl FnMut(Item) -> Item);
  
  /// Map operation that takes ownership during the map
  fn map(mut self, f: impl FnMut(Item) -> Item) -> Self
  where Self: Sized
  {
    self.map_inplace(f);
    self
  }
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
struct Vec3 {
  x: f32,
  y: f32,
  z: f32,
}

impl Map<f32> for Vec3 {
  fn map_inplace(&mut self, f: impl FnMut(f32) -> f32) {
    self.x = f(self.x);
    self.y = f(self.y);
    self.z = f(self.z);
  }
}

#[test]
fn test_simple_vec3_map() {
  let v = Vec3 { x: 1.0f32, y: 2.0f32, z: 3.0f32 };
  let v2 = v.map(|a| a * 2.0f32);
  let v2_expected = Vec3 { x: 2.0f32, y: 4.0f32, z: 6.0f32 };
  assert_eq!(v2, v2_expected);
}
```

like `PartialEq` and `Debug`, the `Map` implementation is pretty rote and can
get hairy.

## Specification of `#[derive(Map)]`

Given a `Map<T>` trait to implement, generate an implementation where the
`map_inplace` calls the passed-in function on every field of the struct, in the
order it was declared in. You do not have to support tuple structs or enums (although that is a
fun challenge).

As a convenience, there are default implementations of `Map` for all integer
literals and many standard library types.

### Using `#[derive(Map)]`

```rust
// When the type argument is provided, only generate `impl Map<T> for Type`
#[derive(PartialEq, Debug, Map<f32>)]
struct Vec3 {
  x: f32,
  y: f32,
  z: f32,
}

// When the type argument is not provided, the underlying type is assumed to
// be polymorphic in one variable; generate `impl<T> Map<T> for Type<T>`
#[derive(Map)]
struct VecPair<T> {
  vec1: Vec<T>,
  vec2: Vec<T>,
}
```

## Shell Code

We have provided you with:
* A Cargo workspace with a proc-macro crate and a testing crate
* Code for a `#[derive(Map)]` proc macro that does nothing, with the correct
  type signature
* Code to test your implementation. This will probably be "does the test crate
  compile at all" since the traits will need to get implemented by the proc
  macro.

## Turning In

Submit homework on Gradescope, as usual
