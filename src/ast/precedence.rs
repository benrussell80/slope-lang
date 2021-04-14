#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
#[repr(u8)]
pub enum Precedence {
    Lowest = 0,
    AndOrXor,
    LessGreaterEqualCoa,
    PlusMinus,
    MultDivMod,
    As,
    Not,
    Exponent,
    Postfix,
    Call,
    Hightest = u8::MAX,
}