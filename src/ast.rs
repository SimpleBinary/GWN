use crate::scanner::Token;
use crate::typ::Typ;

#[derive(Debug)]
pub enum Decl {
    Constant(Box<ConstantDecl>),
    Evaluated(Box<EvaluatedDecl>)
}

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
    name: Token,
    type_name: Token,
    
    expr: Expr,
}

into_decl!(ConstantDecl, Constant);

#[derive(Debug)]
pub struct EvaluatedDecl {
    expr: Expr,
}

into_decl!(EvaluatedDecl, Evaluated);

#[derive(Debug)]
pub struct Expr {
    pub node: ExprKind,
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

#[derive(Debug)]
pub enum ExprKind {
    Constant(Box<ConstantExpr>),

    Unary(Box<UnaryExpr>),

    Binary(Box<BinaryExpr>),
    Logical(Box<LogicalExpr>),

    Apply(Box<ApplyExpr>),

    Func(Box<FuncExpr>),

    Literal(Literal),
}

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

#[derive(Debug)]
pub struct FuncCase {
    guards: Vec<FuncGuard>
}

#[derive(Debug)]
pub struct FuncGuard {
    param: Pattern,
    value: Expr,
}

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    Float(f64),
    Bool(bool),
    String(Box<String>),
}

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