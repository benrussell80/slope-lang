use slope::run;

fn main() {
    run!("
        fn heaviside(x) = {
            0 if x < 0;
            0.5 if x == 0;
            1 else;
        };

        heaviside(-2);
        heaviside(0.0);
        heaviside(2 / 1);
    ");
}