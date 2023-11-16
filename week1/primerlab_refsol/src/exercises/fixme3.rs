// Make me compile!

#[test]
fn calling_call() {
    call_me(3);
}

#[cfg(test)]
fn call_me(num:) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}