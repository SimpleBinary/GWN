use crate::scanner::Token;
use crate::typ::Typ;

pub struct Decl {
    node: DeclKind,
}

pub enum DeclKind {
    Constant(Box<ConstantDecl>),
    Evaluated(Box<EvaluatedDecl>)
}

pub struct ConstantDecl {
    name: Token,
    type_name: Token,
    
    expr: Expr,
}

pub struct EvaluatedDecl {
    expr: Expr,
}

pub struct Expr {
    node: ExprKind,
    typ: Typ
}

pub enum ExprKind {
    Constant(Box<ConstantExpr>),

    Unary(Box<UnaryExpr>),

    Arithmetic(Box<ArithmeticExpr>),
    Comparison(Box<ComparisonExpr>),
    Logical(Box<LogicalExpr>),

    Apply(Box<ApplyExpr>),

    Func(Box<FuncExpr>),

    Literal(Literal),
}

pub struct ConstantExpr {
    name: Token
}

pub struct UnaryExpr {
    operator: Token,
    operand: Expr,
}

pub struct ArithmeticExpr {
    operator: Token,
    left: Expr,
    right: Expr,
}

pub struct ComparisonExpr {
    operator: Token,
    left: Expr,
    right: Expr,
}

pub struct LogicalExpr {
    operator: Token,
    left: Expr,
    right: Expr,
}

pub struct ApplyExpr {
    operator: Token,
    func: Expr,
    arg: Expr,
}

pub struct FuncExpr {
    cases: Vec<FuncCase>
}

pub struct FuncCase {
    guards: Vec<FuncGuard>
}

pub struct FuncGuard {
    param: Pattern,
    value: Expr,
}

pub enum Pattern {
    Literal(Literal),
    Identifier(Box<Token>)
}

pub enum Literal {
    Integer(i32),
    Float(f64),
    Bool(bool),
    String(Box<String>),
}