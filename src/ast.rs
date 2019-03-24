// gwn::ast: The internal representation of the abstract syntax tree.

use crate::scanner::Token;
use crate::typ::Typ;

// Decl: The top-level AST node
#[derive(Debug)]
pub enum Decl {
    // A constant declaration, e.g. `a = 2`, `foo = {x | x + 1}`
    Constant(Box<ConstantDecl>),

    // Just a normal expression that is evaluated, e.g. `print <- "Foo"`
    Evaluated(Box<EvaluatedDecl>)
}

// Generate an implementation for the `Into<Decl>` trait so that any
// variant of a Decl such as a `ConstantDecl` may be converted back to 
// the Decl enum by calling `variant.into::<Decl>()`
macro_rules! into_decl {
    ($from:ident, $to:ident) => {
        impl Into<Decl> for $from {
            fn into(self) -> Decl {
                Decl::$to(Box::new(self))
            }
        }
    }
}

#[derive(Debug)]
pub struct ConstantDecl {
    // The name of the constant
    name: Token,

    // The typename provided. Empty if left out.
    type_name: Token,
    
    // The constant's value
    value: Expr,
}

into_decl!(ConstantDecl, Constant);

#[derive(Debug)]
pub struct EvaluatedDecl {
    // The expression to be evaluated
    expr: Expr,
}

into_decl!(EvaluatedDecl, Evaluated);

// Represents a single expression, like `2`, `42 + e`, `foo`,
// `eat <- "food"`, etc, plus its type, e.g. Int.
#[derive(Debug)]
pub struct Expr {
    // The actual node
    pub node: ExprKind,

    // The type of the expression, e.g. Int
    pub typ: Typ
}

impl Expr {
    pub fn new(node: ExprKind, typ: Typ) -> Expr {
        Expr {
            node,
            typ
        }
    }
}

// The 'real' representation of an expression (without the type).
#[derive(Debug)]
pub enum ExprKind {
    // A constant, like `foo` or `boof`
    Constant(Box<ConstantExpr>),

    // Unary operations, like `-42`, `not true`
    Unary(Box<UnaryExpr>),

    // Binary operations, such as `2 + 2`, `31 <= foo`
    Binary(Box<BinaryExpr>),

    // Logical binary operations, e.g. `true and false`, `false or false`
    Logical(Box<LogicalExpr>),

    // Applying an argument to a function, e.g. `21 -> fib`
    Apply(Box<ApplyExpr>),

    // A function literal, like `{x y | x + y}`
    Func(Box<FuncExpr>),

    // A regular literal value, e.g. `"goo"`, `42.42`
    Literal(Literal),
}

// Generate an implementation for the `Into<Expr>` trait so that any
// variant of an Expr such as a `ConstantExpr` may be converted back to 
// the Decl enum by calling `variant.into::<Expr>()`
macro_rules! into_expr {
    ($from:ident, $to:ident) => {
        impl Into<Expr> for $from {
            fn into(self) -> Expr {
                Expr::new(
                    ExprKind::$to(Box::new(self)),
                    Typ::Unknown
                )
            }
        }
    }
}

#[derive(Debug)]
pub struct ConstantExpr {
    // The name of the constant
    name: Token
}

into_expr!(ConstantExpr, Constant);

#[derive(Debug)]
pub struct UnaryExpr {
    operator: Token,
    operand: Expr,
}

into_expr!(UnaryExpr, Unary);

#[derive(Debug)]
pub struct BinaryExpr {
    operator: Token,
    left: Expr,
    right: Expr,
}

into_expr!(BinaryExpr, Binary);

#[derive(Debug)]
pub struct LogicalExpr {
    operator: Token,
    left: Expr,
    right: Expr,
}

into_expr!(LogicalExpr, Logical);

#[derive(Debug)]
pub struct ApplyExpr {
    operator: Token,
    func: Expr,
    arg: Expr,
}

into_expr!(ApplyExpr, Apply);

#[derive(Debug)]
pub struct FuncExpr {
    cases: Vec<FuncCase>
}

into_expr!(FuncExpr, Func);

// The different cases of a function. Say we had the following code:
// ```
// {0 | "is zero"},
// {1 | "is one"},
// {x | "is not zero or one"}
// ```
// Each comma-seperated body is a 'case'.

#[derive(Debug)]
pub struct FuncCase {
    param: Pattern
    guards: Vec<FuncGuard>
}

// For each case in a function, there may be any amount of guards.
// Consider:
// ```
// {x | x == 0 ? "is zero",
//      x == 1 ? "is one",
//      else ? "is not zero or one"}
// ```
// This case has 3 guards. The condition is before the question mark,
// while the value is after. Multiple guards are seperated by commas.

// This function appears to have no guards at all:
// `{x | x + 1}`
// However, this is just syntactic sugar for:
// `{x | true ? x + 1}`
// So, it actually has 1 guard that will always execute.
#[derive(Debug)]
pub struct FuncGuard {
    condition: Expr,
    value: Expr,
}

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    Float(f64),
    Bool(bool),
    String(Box<String>),
}

// We don't use the auto-generated implementation of Into<Expr>
// here, as Literal is not wrapped in a Box.
impl Into<Expr> for Literal {
    fn into(self) -> Expr {
        let typ = match self {
            Literal::Int(_) => Typ::Int,
            Literal::Float(_) => Typ::Float,
            Literal::Bool(_) => Typ::Bool,
            Literal::String(_) => Typ::String,
        };

        let node = ExprKind::Literal(self);

        Expr::new(node, typ)
    }
}

#[derive(Debug)]
pub enum Pattern {
    Literal(Literal),
    Identifier(Box<Token>)
}