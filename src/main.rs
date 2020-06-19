pub mod fibonacci;
use fibonacci::Fibonacci;
fn main() {
    let mut i = 0;
    let mut num = 0;
    loop {
        num = Fibonacci::of(i);

        if (num > 200) {
            break;
        }

        print!("{}, ", num);
        i += 1;
    }
}
