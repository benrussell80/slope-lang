use slope::run;

fn main() {
    run!("
        # calculate e ^ x using taylor series expansion with `n` terms of precision
        fn exp(x, n) = {
            1 if n == 0;
            x ^ n / n! + exp(x, n - 1) else;
        };
        
        # are we within 0.00001 of the correct answer?
        # E is the built-in value for the exponential number
        |exp(2, 15) - E ^ 2| < 1 / 100000;
    ");
}