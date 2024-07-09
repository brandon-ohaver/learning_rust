fn main () {
    for int in 0..15 {
        println!("fibonacci({}) => {}", int, fib(int));
    }
}

fn fib(n: i32) -> i32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fib(n-1) + fib(n-2);
    }
}
