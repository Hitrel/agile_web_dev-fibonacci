
fn fibonacci(n: i128) -> i128 {
    if n < 0 {
        return fib(-n).0;
    } else {
        return fib(n).0;
    }
}

fn fib(n: i128) -> (i128, i128) {
    if n == 0 {
        return (0, 1);
    } else {
        let (f_n, f_n_1) = fib(n / 2);
        let f_2n = f_n * (f_n_1 * 2 - f_n);
        let f_2n_1 = f_n_1 * f_n_1 + f_n * f_n;

        if n % 2 == 0 {
            return (f_2n, f_2n_1);
        } else {
            return (f_2n_1, f_2n + f_2n_1);
        }
    }
}

pub struct Fibonacci;

impl Fibonacci {
    pub fn of (n: i128) -> i128 {
        return fibonacci(n);
    }
}