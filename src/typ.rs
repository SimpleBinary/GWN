// The type of an expression
#[derive(Debug)]
pub enum Typ {
    // This expression's type hasn't been resolved yet
    Unknown,

    // Primitives
    Int,
    Float,
    Bool,
    String,

    Func(Box<Typ>, Box<Typ>),
}