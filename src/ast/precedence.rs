#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
#[repr(u8)]
pub enum Precedence {
    Lowest = 0,
    AndOrXor,
    LessGreaterEqualCoa,
    As,
    Not,
    In,
    PlusMinus,
    MultDivMod,
    Negative,
    Exponent,
    Postfix,
    Call,
    Hightest = u8::MAX,
}