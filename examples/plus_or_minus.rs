use slope::run;

fn main() {
    // for x^2 - 4x - 12
    run!("
        fn quadratic_formula(a, b, c) = {
            undefined if a == 0 or b ^ 2 - 4 * a * c < 0;
            - b / (2 * a) +/- (b ^ 2 - 4 * a * c) ^ 0.5 / (2 * a) else;
        };

        quadratic_formula(1, -4, -12);
        quadratic_formula(2, 3, 4);
    ");
}