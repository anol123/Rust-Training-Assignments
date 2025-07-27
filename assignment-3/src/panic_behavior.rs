// To demonstrate panic behavior: "unwind" vs "abort"
// Set in Cargo.toml with [profile] settings

pub fn trigger_panic_unwind() {
    println!("Triggering panic (unwind mode)...");
    panic!("This is an unwind panic.");
}

// To test abort behavior, set in Cargo.toml:
// [profile.release]
// panic = "abort"
pub fn trigger_panic_abort() {
    println!("Triggering panic (abort mode)...");
    panic!("This is an abort panic.");
}
