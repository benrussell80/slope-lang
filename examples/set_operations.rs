use slope::run;

fn main() {
    run!(r"
        fn any(set) = true in set;
        fn all(set) = not false in set;

        any({ true, false });
        all({ true, false });

        { 1, 2, 3 } \   { 1, 2, 3, 4, 5 };
        { 1, 2, 3 } \/  { 1, 2, 3, 4, 5 };
        { 1, 2, 3 } /\  { 1, 2, 3, 4, 5 };
        { 1, 2, 3 } /_\ { 1, 2, 3, 4, 5 };
        { 1, 2, 3 } <   { 1, 2, 3, 4, 5 };
        { 1, 2, 3 } <=  { 1, 2, 3, 4, 5 };
    ");
}