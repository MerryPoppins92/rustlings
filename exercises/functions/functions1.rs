// functions1.rs
// Make me compile! Execute `rustlings hint functions1` for hints :)

fn main() {
    call_me(3);
}
fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}


