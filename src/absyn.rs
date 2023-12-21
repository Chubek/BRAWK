type Identifier = String;
type Param = String;

#[derive(Debug)]
enum BinaryOp {
    Exp,
    Mul,
    Div,
    Rem,
    Add,
    Sub,
    Ge,
    Gt,
    Le,
    Lt,
    Eq,
    Ne,
    Ere,
    Nre,
    And,
    Or,
    Shr,
    Shl,
    AddAssign,
    SubAssign,
    MulAssign,
    RemAssign,
    DivAssign,
    ExpAssign,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
}

#[derive(Debug)]
enum UnaryOp {
    Incr,
    Decr,
    Neg,
    Pos,
    BitwiseNot,
    Not,
}

#[derive(Debug)]
enum Expr {
    Variable(Identifier),
    Constant(Box<Expr>),
    Number(i32),
    StringLiteral(String),
    Regex(String),
    Lvalue(Box<Lvalue>),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    PostfixOp(Box<Expr>, UnaryOp),
    TernaryOp(Box<Expr>, Box<Expr>, Box<Expr>),
    FunctionCall(Identifier, Vec<Expr>),
}

#[derive(Debug)]
enum Item {
    SoloAction(Action),
    PatternAction(Pattern, Action),
    NameFunction(String, Option<Vec<Param>>, Action),
    IdentFunction(Identifier, Option<Vec<Param>>, Action),
}

#[derive(Debug)]
enum SpecialPattern {
    Begin,
    End,
}

#[derive(Debug)]
enum Pattern {
    NormalPattern(Box<Expr>, Option<Box<Expr>>),
    SpecialPattern(SpecialPattern),
}

#[derive(Debug)]
enum Action {
    EmptyAction,
    BlockAction(Vec<Statement>),
}

#[derive(Debug)]
enum Statement {
    IfStatement(Box<Expr>, Box<Statement>),
    ElseStatement(Box<Statement>),
    WhileStatement(Box<Expr>, Box<Statement>),
    ForStatement(
        Option<Box<Statement>>,
        Option<Box<Expr>>,
        Option<Box<Statement>>,
        Box<Statement>,
    ),
    ForInStatement(Identifier, Identifier, Box<Statement>),
    TerminatableStatement(Box<TerminatableStatement>),
}

#[derive(Debug)]
enum TerminatableStatement {
    SimpleStatement(Box<SimpleStatement>),
    Break,
    Continue,
    Next,
    Exit(Option<Box<Expr>>),
    Return(Option<Box<Expr>>),
    DoWhileStatement(Box<Statement>, Box<Expr>),
}

#[derive(Debug)]
enum SimpleStatement {
    PrintAssign(Box<Lvalue>, Box<PrintExpr>),
    Assignment(Box<Lvalue>, Box<Expr>),
    Delete(String, Vec<Expr>),
    ExprStatement(Box<Expr>),
    PrintStatement(Vec<PrintExpr>, Option<Box<OutputRedirection>>),
    PrintfStatement(String, Vec<PrintExpr>, Option<Box<OutputRedirection>>),
}

#[derive(Debug)]
enum PrintExpr {
    UnaryPrintExpr(Box<PrintExpr>),
    NonUnaryPrintExpr(Vec<PrintExpr>),
}

#[derive(Debug)]
enum Getline {
    SimpleGet,
    SimpleGetWithLval(Box<Lvalue>),
    SimpleGetWithExpr(Box<Getline>, Box<Expr>),
    SimpleGetWithPipe(Box<Expr>, Box<SimpleGet>),
}

#[derive(Debug)]
enum Lvalue {
    SimpleName(Identifier),
    ArrayAccess(Identifier, Vec<Expr>),
    FieldRef(Box<Expr>),
}

#[derive(Debug)]
enum SimpleGet {
    Getline,
    GetlineWithLvalue(Box<Lvalue>),
}

#[derive(Debug)]
enum OutputRedirection {
    OutputRedirect(String, Box<Expr>),
    AppendRedirect(Box<Expr>),
    PipeRedirect(Box<Expr>),
}

#[derive(Debug)]
struct Program(Vec<Item>);

impl Expr {
    fn variable(identifier: &str) -> Self {
        Expr::Variable(identifier.to_string())
    }

    fn constant(expr: Expr) -> Self {
        Expr::Constant(Box::new(expr))
    }

    fn number(value: i32) -> Self {
        Expr::Number(value)
    }

    fn string_literal(value: &str) -> Self {
        Expr::StringLiteral(value.to_string())
    }

    fn regex(value: &str) -> Self {
        Expr::Regex(value.to_string())
    }

    fn lvalue(lvalue: Lvalue) -> Self {
        Expr::Lvalue(Box::new(lvalue))
    }

    fn binary_op(left: Expr, op: BinaryOp, right: Expr) -> Self {
        Expr::BinaryOp(Box::new(left), op, Box::new(right))
    }

    fn unary_op(op: UnaryOp, expr: Expr) -> Self {
        Expr::UnaryOp(op, Box::new(expr))
    }

    fn postfix_op(expr: Expr, op: UnaryOp) -> Self {
        Expr::PostfixOp(Box::new(expr), op)
    }

    fn ternary_op(cond: Expr, true_expr: Expr, false_expr: Expr) -> Self {
        Expr::TernaryOp(Box::new(cond), Box::new(true_expr), Box::new(false_expr))
    }

    fn function_call(identifier: &str, args: Vec<Expr>) -> Self {
        Expr::FunctionCall(identifier.to_string(), args)
    }
}

impl Item {
    fn solo_action(action: Action) -> Self {
        Item::SoloAction(action)
    }

    fn pattern_action(pattern: Pattern, action: Action) -> Self {
        Item::PatternAction(pattern, action)
    }

    fn name_function(name: &str, params: Option<Vec<Param>>, action: Action) -> Self {
        Item::NameFunction(name.to_string(), params, action)
    }

    fn ident_function(identifier: &str, params: Option<Vec<Param>>, action: Action) -> Self {
        Item::IdentFunction(identifier.to_string(), params, action)
    }
}

impl Pattern {
    fn normal_pattern(expr: Expr, expr_opt: Option<Expr>) -> Self {
        Pattern::NormalPattern(Box::new(expr), expr_opt.map(Box::new))
    }

    fn special_pattern(special_pattern: SpecialPattern) -> Self {
        Pattern::SpecialPattern(special_pattern)
    }
}

impl Action {
    fn empty_action() -> Self {
        Action::EmptyAction
    }

    fn block_action(statements: Vec<Statement>) -> Self {
        Action::BlockAction(statements)
    }
}

impl Statement {
    fn if_statement(condition: Expr, body: Statement) -> Self {
        Statement::IfStatement(Box::new(condition), Box::new(body))
    }

    fn else_statement(body: Statement) -> Self {
        Statement::ElseStatement(Box::new(body))
    }

    fn while_statement(condition: Expr, body: Statement) -> Self {
        Statement::WhileStatement(Box::new(condition), Box::new(body))
    }

    fn for_statement(
        initialization: Option<Statement>,
        condition: Option<Expr>,
        iteration: Option<Statement>,
        body: Statement,
    ) -> Self {
        Statement::ForStatement(
            initialization.map(Box::new),
            condition.map(Box::new),
            iteration.map(Box::new),
            Box::new(body),
        )
    }

    fn for_in_statement(identifier: &str, iterator: &str, body: Statement) -> Self {
        Statement::ForInStatement(identifier.to_string(), iterator.to_string(), Box::new(body))
    }

    fn terminatable_statement(terminatable_statement: TerminatableStatement) -> Self {
        Statement::TerminatableStatement(Box::new(terminatable_statement))
    }
}

impl TerminatableStatement {
    fn simple_statement(simple_statement: SimpleStatement) -> Self {
        TerminatableStatement::SimpleStatement(Box::new(simple_statement))
    }

    fn break_statement() -> Self {
        TerminatableStatement::Break
    }

    fn continue_statement() -> Self {
        TerminatableStatement::Continue
    }

    fn next_statement() -> Self {
        TerminatableStatement::Next
    }

    fn exit_statement(expr: Option<Expr>) -> Self {
        TerminatableStatement::Exit(expr.map(Box::new))
    }

    fn return_statement(expr: Option<Expr>) -> Self {
        TerminatableStatement::Return(expr.map(Box::new))
    }

    fn do_while_statement(body: Statement, condition: Expr) -> Self {
        TerminatableStatement::DoWhileStatement(Box::new(body), Box::new(condition))
    }
}

impl SimpleStatement {
    fn print_assign(lvalue: Lvalue, print_expr: PrintExpr) -> Self {
        SimpleStatement::PrintAssign(Box::new(lvalue), Box::new(print_expr))
    }

    fn assignment(lvalue: Lvalue, expr: Expr) -> Self {
        SimpleStatement::Assignment(Box::new(lvalue), Box::new(expr))
    }

    fn delete(name: &str, expr_list: Vec<Expr>) -> Self {
        SimpleStatement::Delete(name.to_string(), expr_list)
    }

    fn expr_statement(expr: Expr) -> Self {
        SimpleStatement::ExprStatement(Box::new(expr))
    }

    fn print_statement(
        print_expr_list: Vec<PrintExpr>,
        redirection: Option<OutputRedirection>,
    ) -> Self {
        SimpleStatement::PrintStatement(print_expr_list, redirection.map(Box::new))
    }

    fn printf_statement(
        format_string: &str,
        print_expr_list: Vec<PrintExpr>,
        redirection: Option<OutputRedirection>,
    ) -> Self {
        SimpleStatement::PrintfStatement(
            format_string.to_string(),
            print_expr_list,
            redirection.map(Box::new),
        )
    }
}
