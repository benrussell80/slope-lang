use slope::run;

fn main() {
    run!("
        # this is a comment
        let value = 42;  # this is also a comment
    ");
}