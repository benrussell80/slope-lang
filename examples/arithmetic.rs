use slope::run;

fn main() {
    run!("
        2 + 2;
        12 - 5;
        13 / 2;
        16 * 3;
        2 ^ 6;
        7 % 4;
    ");
}