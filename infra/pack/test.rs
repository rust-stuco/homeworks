fn foo<T>(x: T) -> T {
    x;
    y
}

fn bar(xs: &[u8]) -> usize {
    #[cfg(refsol)]
    {
        xs.len()
    }
}

#[cfg(refsol)]
fn a() {}
