use slope::run;

fn main() {
    // this should panic
    run!("
        1 == true;
    ");
}