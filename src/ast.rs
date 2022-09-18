#[derive(Clone, PartialEq, Debug)]
pub enum TBoolean {
    True,
    False,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TypeExpr {
    Number,
    String,
    Boolean,
}

#[derive(PartialEq, Debug)]
pub enum AST<'a> {
    VariableDeclaration {
        name: &'a str,
        typing: TypeExpr,
    },
    Variable(&'a str),
    BooleanLiteral(TBoolean),
    StringLiteral(String),
    NumberLiteral(String),
    Let {
        variable_declaration: Box<AST<'a>>,
        variable_expression: Box<AST<'a>>,
        expression: Box<AST<'a>>,
    },
}
