#[derive(Debug)]
enum AstNode {
    Program(Vec<AstNode>),
    PatternActionRule(Option<Box<AstNode>>, Box<AstNode>),
    Pattern(Box<AstNode>),
    PatternExpression(Box<AstNode>),
    Action(Box<AstNode>),
    FunctionDefinition(String, Vec<String>, Box<AstNode>),
    ParameterList(Vec<String>),
    StatementList(Vec<AstNode>),
    Statement(Box<AstNode>),
    IfStatement(Box<AstNode>, Box<AstNode>, Option<Box<AstNode>>),
    WhileStatement(Box<AstNode>, Box<AstNode>),
    ForStatement(
        Box<AstNode>,
        Option<Box<AstNode>>,
        Option<Box<AstNode>>,
        Box<AstNode>,
    ),
    DoWhileStatement(Box<AstNode>, Box<AstNode>),
    ForInitializer(Box<AstNode>),
    ForIterator(Box<AstNode>),
    PrintStatement(Option<Box<AstNode>>, Option<Box<AstNode>>),
    PrintfStatement(Box<AstNode>, Box<AstNode>, Option<Box<AstNode>>),
    NextStatement,
    ExitStatement(Option<Box<AstNode>>),
    ReturnStatement(Option<Box<AstNode>>),
    DeleteStatement(Box<AstNode>),
    VariableAssignment(String, Box<AstNode>),
    ArrayElement(String, Box<AstNode>),
    ExpressionList(Vec<AstNode>),
    Expression(Box<AstNode>),
    LogicalOrExpression(Box<AstNode>, Vec<AstNode>),
    LogicalAndExpression(Box<AstNode>, Vec<AstNode>),
    InclusiveOrExpression(Box<AstNode>, Vec<AstNode>),
    ExclusiveOrExpression(Box<AstNode>, Vec<AstNode>),
    AndExpression(Box<AstNode>, Vec<AstNode>),
    EqualityExpression(Box<AstNode>, String, Box<AstNode>),
    RelationalExpression(Box<AstNode>, String, Box<AstNode>),
    ShiftExpression(Box<AstNode>, String, Box<AstNode>),
    AdditiveExpression(Box<AstNode>, String, Box<AstNode>),
    MultiplicativeExpression(Box<AstNode>, String, Box<AstNode>),
    PrimaryExpression(Box<AstNode>),
    Variable(String),
    Constant(Constant),
    FunctionCall(String, Option<Vec<AstNode>>),
    ArgumentList(Vec<AstNode>),
    Redirection(String),
    IntegerLiteral(String),
    FloatingPointLiteral(String),
    StringLiteral(String),
    Nil
}

#[derive(Debug)]
enum Constant {
    IntegerLiteral(String),
    FloatingPointLiteral(String),
    StringLiteral(String),
}

struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Lexer<'a> {
        Lexer { input, position: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn advance(&mut self) {
        if let Some(ch) = self.input.chars().nth(self.position) {
            self.position += ch.len_utf8();
        }
    }

    fn previous_lexeme(&self) -> &'a str {
        &self.input[..self.position]
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn consume_identifier(&mut self) -> String {
        let start = self.position;
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position].to_string()
    }

    fn consume_string_literal(&mut self) -> String {
        let mut value = String::new();
        self.advance(); 
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance();
                break;
            } else {
                value.push(ch);
                self.advance();
            }
        }
        value
    }

    fn consume_digit_sequence(&mut self) -> String {
        let start = self.position;
        while let Some(ch) = self.peek() {
            if ch.is_digit(10) {
                self.advance();
            } else {
                break;
            }
        }
        self.input[start..self.position].to_string()
    }

    fn consume_floating_point_literal(&mut self) -> String {
        let mut value = String::new();
        value.push_str(&self.consume_digit_sequence());
        if let Some('.') = self.peek() {
            value.push('.');
            self.advance();
            value.push_str(&self.consume_digit_sequence());
        }
        if let Some(ch) = self.peek() {
            if ch == 'e' || ch == 'E' {
                value.push(ch);
                self.advance();
                if let Some(sign) = self.peek() {
                    if sign == '+' || sign == '-' {
                        value.push(sign);
                        self.advance();
                    }
                }
                value.push_str(&self.consume_digit_sequence());
            }
        }
        value
    }

    fn consume_character(&mut self) -> char {
        self.peek().unwrap_or('\0')
    }

    fn consume_redirection(&mut self) -> String {
        let mut redirection = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphabetic() {
                redirection.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        redirection
    }
}

fn parse_program(lexer: &mut Lexer) -> AstNode {
    let mut program = vec![];
    while lexer.peek().is_some() {
        program.push(parse_pattern_action_rule(lexer));
    }
    AstNode::Program(program)
}

fn parse_pattern_action_rule(lexer: &mut Lexer) -> AstNode {
    let pattern = if lexer.peek() == Some('[') {
        lexer.advance();
        let pattern_expression = parse_pattern_expression(lexer);
        Some(Box::new(AstNode::PatternExpression(Box::new(pattern_expression))))
    } else if lexer.peek() == Some('B') {
        lexer.advance();
        Some(Box::new(AstNode::PatternExpression(Box::new(
            AstNode::Variable("BEGIN".to_string()),
        ))))
    } else if lexer.peek() == Some('E') {
        lexer.advance();
        Some(Box::new(AstNode::PatternExpression(Box::new(
            AstNode::Variable("END".to_string()),
        ))))
    } else {
        None
    };

    let action = parse_action(lexer);
    AstNode::PatternActionRule(pattern, Box::new(action))
}

fn parse_pattern_expression(lexer: &mut Lexer) -> AstNode {
    parse_expression(lexer)
}

fn parse_action(lexer: &mut Lexer) -> AstNode {
    lexer.skip_whitespace();
    assert_eq!(lexer.peek(), Some('{'));
    lexer.advance();
    let statement_list = parse_statement_list(lexer);
    assert_eq!(lexer.peek(), Some('}'));
    lexer.advance();
    AstNode::Action(Box::new(statement_list))
}

fn parse_statement_list(lexer: &mut Lexer) -> AstNode {
    let mut statements = vec![parse_statement(lexer)];
    while lexer.peek() == Some(';') {
        lexer.advance();
        statements.push(parse_statement(lexer));
    }
    AstNode::StatementList(statements)
}

fn parse_statement(lexer: &mut Lexer) -> AstNode {
    if lexer.peek() == Some('i') {
        parse_if_statement(lexer)
    } else if lexer.peek() == Some('w') {
        parse_while_statement(lexer)
    } else if lexer.peek() == Some('f') {
        parse_for_statement(lexer)
    } else if lexer.peek() == Some('d') {
        parse_do_while_statement(lexer)
    } else if lexer.peek() == Some('p') {
        parse_print_statement(lexer)
    } else if lexer.peek() == Some('p') {
        parse_printf_statement(lexer)
    } else if lexer.peek() == Some('n') {
        parse_next_statement(lexer)
    } else if lexer.peek() == Some('e') {
        parse_exit_statement(lexer)
    } else if lexer.peek() == Some('r') {
        parse_return_statement(lexer)
    } else if lexer.peek() == Some('d') {
        parse_delete_statement(lexer)
    } else {
        parse_variable_assignment(lexer)
    }
}

fn parse_if_statement(lexer: &mut Lexer) -> AstNode {
    assert_eq!(lexer.peek(), Some('i'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('('));
    lexer.advance();
    let condition = parse_expression(lexer);
    assert_eq!(lexer.peek(), Some(')'));
    lexer.advance();
    let if_body = parse_statement(lexer);
    let else_body = if lexer.peek() == Some('e') {
        lexer.advance();
        Some(Box::new(parse_statement(lexer)))
    } else {
        None
    };
    AstNode::IfStatement(Box::new(condition), Box::new(if_body), else_body)
}

fn parse_while_statement(lexer: &mut Lexer) -> AstNode {
    assert_eq!(lexer.peek(), Some('w'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('('));
    lexer.advance();
    let condition = parse_expression(lexer);
    assert_eq!(lexer.peek(), Some(')'));
    lexer.advance();
    let body = parse_statement(lexer);
    AstNode::WhileStatement(Box::new(condition), Box::new(body))
}

fn parse_for_statement(lexer: &mut Lexer) -> AstNode {
    assert_eq!(lexer.peek(), Some('f'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('('));
    lexer.advance();
    let initializer = if lexer.peek() != Some(';') {
        Box::new(parse_for_initializer(lexer))
    } else {
        Box::new(AstNode::Nil)
    };
    assert_eq!(lexer.peek(), Some(';'));
    lexer.advance();
    let condition = if lexer.peek() != Some(';') {
        Some(Box::new(parse_expression(lexer)))
    } else {
        None
    };
    assert_eq!(lexer.peek(), Some(';'));
    lexer.advance();
    let iterator = if lexer.peek() != Some(')') {
        Some(Box::new(parse_for_iterator(lexer)))
    } else {
        None
    };
    assert_eq!(lexer.peek(), Some(')'));
    lexer.advance();
    let body = parse_statement(lexer);
    AstNode::ForStatement(initializer, condition, iterator, Box::new(body))
}

fn parse_do_while_statement(lexer: &mut Lexer) -> AstNode {
    assert_eq!(lexer.peek(), Some('d'));
    lexer.advance();
    let body = parse_statement(lexer);
    assert_eq!(lexer.peek(), Some('w'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('('));
    lexer.advance();
    let condition = parse_expression(lexer);
    assert_eq!(lexer.peek(), Some(')'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some(';'));
    lexer.advance();
    AstNode::DoWhileStatement(Box::new(body), Box::new(condition))
}

fn parse_for_initializer(lexer: &mut Lexer) -> AstNode {
    if lexer.peek().unwrap().is_alphabetic() {
        parse_variable_assignment(lexer)
    } else {
        parse_expression(lexer)
    }
}

fn parse_for_iterator(lexer: &mut Lexer) -> AstNode {
    parse_expression(lexer)
}

fn parse_print_statement(lexer: &mut Lexer) -> AstNode {
    assert_eq!(lexer.peek(), Some('p'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('r'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('i'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('n'));
    lexer.advance();

    let expression_list = if lexer.peek() != Some('{') {
        None
    } else {
        Some(Box::new(parse_expression_list(lexer)))
    };

    let redirection = if lexer.peek() == Some('>') {
        Some(Box::new(AstNode::Redirection(parse_redirection(lexer))))
    } else {
        None
    };

    AstNode::PrintStatement(expression_list, redirection)
}

fn parse_printf_statement(lexer: &mut Lexer) -> AstNode {
    assert_eq!(lexer.peek(), Some('p'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('r'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('i'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('n'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('t'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('f'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('('));
    lexer.advance();
    let format_string = parse_format_string(lexer);
    assert_eq!(lexer.peek(), Some(','));
    lexer.advance();
    let expression_list = parse_expression_list(lexer);
    assert_eq!(lexer.peek(), Some(')'));
    lexer.advance();
    let redirection = if lexer.peek() == Some('>') {
        lexer.advance();
        Some(Box::new(AstNode::Redirection(parse_redirection(lexer))))
    } else {
        None
    };
    AstNode::PrintfStatement(Box::new(format_string), Box::new(expression_list), redirection)
}

fn parse_next_statement(lexer: &mut Lexer) -> AstNode {
    assert_eq!(lexer.peek(), Some('n'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('e'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('x'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('t'));
    lexer.advance();
    AstNode::NextStatement
}

fn parse_exit_statement(lexer: &mut Lexer) -> AstNode {
    assert_eq!(lexer.peek(), Some('e'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('x'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('i'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('t'));
    lexer.advance();
    let expression = if lexer.peek() != Some('{') {
        Some(Box::new(parse_expression(lexer)))
    } else {
        None
    };
    AstNode::ExitStatement(expression)
}

fn parse_return_statement(lexer: &mut Lexer) -> AstNode {
    assert_eq!(lexer.peek(), Some('r'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('e'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('t'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('u'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('r'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('n'));
    lexer.advance();
    let expression = if lexer.peek() != Some('{') {
        Some(Box::new(parse_expression(lexer)))
    } else {
        None
    };
    AstNode::ReturnStatement(expression)
}

fn parse_delete_statement(lexer: &mut Lexer) -> AstNode {
    assert_eq!(lexer.peek(), Some('d'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('e'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('l'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('e'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('t'));
    lexer.advance();
    assert_eq!(lexer.peek(), Some('e'));
    lexer.advance();
    let array_element = parse_array_element(lexer);
    AstNode::DeleteStatement(Box::new(array_element))
}

fn parse_variable_assignment(lexer: &mut Lexer) -> AstNode {
    let identifier = parse_identifier(lexer);
    assert_eq!(lexer.peek(), Some('='));
    lexer.advance();
    let expression = parse_expression(lexer);
    AstNode::VariableAssignment(identifier, Box::new(expression))
}

fn parse_array_element(lexer: &mut Lexer) -> AstNode {
    let identifier = parse_identifier(lexer);
    assert_eq!(lexer.peek(), Some('['));
    lexer.advance();
    let expression_list = parse_expression_list(lexer);
    assert_eq!(lexer.peek(), Some(']'));
    lexer.advance();
    AstNode::ArrayElement(identifier, Box::new(expression_list))
}

fn parse_expression_list(lexer: &mut Lexer) -> AstNode {
    let mut expressions = vec![parse_expression(lexer)];
    while lexer.peek() == Some(',') {
        lexer.advance();
        expressions.push(parse_expression(lexer));
    }
    AstNode::ExpressionList(expressions)
}

fn parse_expression(lexer: &mut Lexer) -> AstNode {
    parse_logical_or_expression(lexer)
}

fn parse_logical_or_expression(lexer: &mut Lexer) -> AstNode {
    let mut operands = vec![parse_logical_and_expression(lexer)];

    while lexer.peek() == Some('|') {
        lexer.advance();

        if lexer.peek() == Some('|') {
            lexer.advance();
            operands.push(parse_logical_and_expression(lexer));
        } else {
            break;
        }
    }

    if operands.len() == 1 {
        operands.pop().unwrap()
    } else {
        AstNode::LogicalOrExpression(Box::new(operands.remove(0)), operands)
    }
}

fn parse_logical_and_expression(lexer: &mut Lexer) -> AstNode {
    let mut operands = vec![parse_inclusive_or_expression(lexer)];

    while lexer.peek() == Some('&') {
        lexer.advance();
        operands.push(parse_inclusive_or_expression(lexer));
    }

    if operands.len() == 1 {
        operands.pop().unwrap()
    } else {
        AstNode::LogicalAndExpression(
            Box::new(operands.remove(0)),
            operands,
        )
    }
}

fn parse_inclusive_or_expression(lexer: &mut Lexer) -> AstNode {
    let mut operands = vec![parse_exclusive_or_expression(lexer)];
    while lexer.peek() == Some('|') {
        lexer.advance();
        operands.push(parse_exclusive_or_expression(lexer));
    }
    if operands.len() == 1 {
        operands.pop().unwrap()
    } else {
        AstNode::InclusiveOrExpression(Box::new(operands.remove(0)), operands)
    }
}

fn parse_exclusive_or_expression(lexer: &mut Lexer) -> AstNode {
    let mut operands = vec![parse_and_expression(lexer)];
    while lexer.peek() == Some('^') {
        lexer.advance();
        operands.push(parse_and_expression(lexer));
    }
    if operands.len() == 1 {
        operands.pop().unwrap()
    } else {
        AstNode::ExclusiveOrExpression(Box::new(operands.remove(0)), operands)
    }
}

fn parse_and_expression(lexer: &mut Lexer) -> AstNode {
    let mut operands = vec![parse_equality_expression(lexer)];

    while lexer.peek() == Some('&') {
        lexer.advance();

        if lexer.peek() == Some('&') {
            lexer.advance();
            operands.push(parse_equality_expression(lexer));
        } else {
            break;
        }
    }

    if operands.len() == 1 {
        operands.pop().unwrap()
    } else {
        AstNode::AndExpression(Box::new(operands.remove(0)), operands)
    }
}

fn parse_equality_expression(lexer: &mut Lexer) -> AstNode {
    let mut operands = vec![parse_relational_expression(lexer)];

    while lexer.peek() == Some('=') || lexer.peek() == Some('!') {
        lexer.advance();
        lexer.advance();

        operands.push(AstNode::EqualityExpression(
            Box::new(operands.pop().unwrap()),
            lexer.previous_lexeme().to_string(),
            Box::new(parse_relational_expression(lexer)),
        ));
    }

    if operands.len() == 1 {
        operands.pop().unwrap()
    } else {
        AstNode::EqualityExpression(
            Box::new(operands.remove(0)),
            "".to_string(),
            Box::new(operands.remove(0)),
        )
    }
}


fn parse_relational_expression(lexer: &mut Lexer) -> AstNode {
    let mut operands = vec![parse_shift_expression(lexer)];

    while matches!(
        lexer.peek(),
        Some('<') | Some('>') | Some('=')
    ) {
        let operator = lexer.peek().unwrap_or_default().to_string();

        lexer.advance();

        if matches!(lexer.peek(), Some('=') | Some('>')) {
            lexer.advance();
        }

        operands.push(AstNode::RelationalExpression(
            Box::new(operands.pop().unwrap()),
            operator,
            Box::new(parse_shift_expression(lexer)),
        ));
    }

    if operands.len() == 1 {
        operands.pop().unwrap()
    } else {
        AstNode::RelationalExpression(
            Box::new(operands.remove(0)),
            "".to_string(),
            Box::new(operands.remove(0)),
        )
    }
}

fn parse_shift_expression(lexer: &mut Lexer) -> AstNode {
    let mut operands = vec![parse_additive_expression(lexer)];

    while matches!(
        lexer.peek(),
        Some('<') | Some('>')
    ) {
        let operator = lexer.peek().unwrap_or_default().to_string();

        lexer.advance();

        if matches!(lexer.peek(), Some('<') | Some('>')) {
            lexer.advance();
        }

        operands.push(AstNode::ShiftExpression(
            Box::new(operands.pop().unwrap()),
            operator,
            Box::new(parse_additive_expression(lexer)),
        ));
    }

    if operands.len() == 1 {
        operands.pop().unwrap()
    } else {
        AstNode::ShiftExpression(
            Box::new(operands.remove(0)),
            "".to_string(),
            Box::new(operands.remove(0)),
        )
    }
}

fn parse_additive_expression(lexer: &mut Lexer) -> AstNode {
    let mut operands = vec![parse_multiplicative_expression(lexer)];

    while matches!(
        lexer.peek(),
        Some('+') | Some('-')
    ) {
        let operator = lexer.peek().unwrap_or_default().to_string();

        lexer.advance();

        operands.push(AstNode::AdditiveExpression(
            Box::new(operands.pop().unwrap()),
            operator,
            Box::new(parse_multiplicative_expression(lexer)),
        ));
    }

    if operands.len() == 1 {
        operands.pop().unwrap()
    } else {
        AstNode::AdditiveExpression(
            Box::new(operands.remove(0)),
            "".to_string(),
            Box::new(operands.remove(0)),
        )
    }
}


fn parse_multiplicative_expression(lexer: &mut Lexer) -> AstNode {
    let mut operands = vec![parse_primary_expression(lexer)];

    while matches!(
        lexer.peek(),
        Some('*') | Some('/') | Some('%')
    ) {
        let operator = lexer.peek().unwrap_or_default().to_string();

        lexer.advance();

        operands.push(AstNode::MultiplicativeExpression(
            Box::new(operands.pop().unwrap()),
            operator,
            Box::new(parse_primary_expression(lexer)),
        ));
    }

    if operands.len() == 1 {
        operands.pop().unwrap()
    } else {
        AstNode::MultiplicativeExpression(
            Box::new(operands.remove(0)),
            "".to_string(),
            Box::new(operands.remove(0)),
        )
    }
}


fn parse_primary_expression(lexer: &mut Lexer) -> AstNode {
    if lexer.peek().unwrap().is_alphabetic() {
        parse_variable(lexer)
    } else if lexer.peek().unwrap().is_alphabetic() {
        parse_variable(lexer)
    } else if lexer.peek().unwrap().is_digit(10) {
        parse_constant(lexer)
    } else if lexer.peek() == Some('"') {
        parse_string_literal(lexer)
    } else if lexer.peek() == Some('(') {
        lexer.advance();
        let expression = parse_expression(lexer);
        assert_eq!(lexer.peek(), Some(')'));
        lexer.advance();
        expression
    } else {
        panic!("Unexpected token while parsing primary expression")
    }
}

fn parse_variable(lexer: &mut Lexer) -> AstNode {
    AstNode::Variable(parse_identifier(lexer))
}

fn parse_constant(lexer: &mut Lexer) -> AstNode {
    if lexer.peek().unwrap().is_digit(10) {
        AstNode::Constant(Constant::IntegerLiteral(parse_integer_literal(lexer)))
    } else if lexer.peek() == Some('.') {
        AstNode::Constant(Constant::FloatingPointLiteral(
            parse_floating_point_literal(lexer),
        ))
    } else {
        panic!("Unexpected token while parsing constant")
    }
}

fn parse_integer_literal(lexer: &mut Lexer) -> String {
    lexer.consume_digit_sequence()
}

fn parse_floating_point_literal(lexer: &mut Lexer) -> String {
    lexer.consume_floating_point_literal()
}

fn parse_string_literal(lexer: &mut Lexer) -> AstNode {
    AstNode::Constant(Constant::StringLiteral(lexer.consume_string_literal()))
}

fn parse_function_call(lexer: &mut Lexer) -> AstNode {
    let identifier = parse_identifier(lexer);
    assert_eq!(lexer.peek(), Some('('));
    lexer.advance();
    let argument_list = if lexer.peek() != Some(')') {
        Some(parse_argument_list(lexer))
    } else {
        None
    };
    assert_eq!(lexer.peek(), Some(')'));
    lexer.advance();
    AstNode::FunctionCall(identifier, argument_list)
}

fn parse_argument_list(lexer: &mut Lexer) -> AstNode {
    let mut arguments = vec![parse_expression(lexer)];
    while lexer.peek() == Some(',') {
        lexer.advance();
        arguments.push(parse_expression(lexer));
    }
    AstNode::ArgumentList(arguments)
}

fn parse_redirection(lexer: &mut Lexer) -> String {
    lexer.consume_redirection()
}

fn parse_identifier(lexer: &mut Lexer) -> String {
    lexer.consume_identifier()
}

fn parse_format_string(lexer: &mut Lexer) -> AstNode {
    assert_eq!(lexer.peek(), Some('"'));
    lexer.advance();
    let mut format_string = String::new();
    while lexer.peek() != Some('"') {
        format_string.push(lexer.consume_character());
    }
    lexer.advance();
    AstNode::Constant(Constant::StringLiteral(format_string))
}
