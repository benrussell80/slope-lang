use slope::run;

fn main() {
    run!("
        fn fib(i) = {
            1 if i == 0 or i == 1;
            fib(i - 2) + fib(i - 1) else;
        };
        
        fib(0);
        fib(1);
        fib(2);
        fib(3);
        fib(4);
    ");
}