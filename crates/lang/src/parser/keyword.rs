pub const Define: &str = "define";
pub const Import: &str = "import";
pub const Display: &str = "display";
pub const Struct: &str = "struct";

pub const If: &str = "if";
pub const And: &str = "and";
pub const Or: &str = "or";

pub const Let: &str = "let";
pub const LetStar: &str = "let*";
pub const LetRec: &str = "letrec";
pub const Match: &str = "match";
pub const Lambda: &str = "lambda";
pub const Begin: &str = "begin";
pub const Cond: &str = "cond";
pub const Quote: &str = "quote";
pub const Section: &str = "section";

pub const RESERVED_WORDS: [&str; 15] = [
    And, Begin, Cond, Define, If, Import, Lambda, Let, LetStar, LetRec, Match, Or, Quote, Section,
    Struct,
];
