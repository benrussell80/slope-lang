use slope::run;

fn main() {
    run!("
        let x = -10;

        |x|;
        |x| * -2;
        |x + |x||;
    ");
}