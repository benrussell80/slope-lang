#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Location {
    Prefix,
    Infix,
    Postfix,
}