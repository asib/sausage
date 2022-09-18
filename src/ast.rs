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
    Function(Box<TypeExpr>, Box<TypeExpr>),
}

#[derive(PartialEq, Debug)]
pub struct VariableDeclaration<'a> {
    pub name: &'a str,
    pub typing: TypeExpr,
}

#[derive(PartialEq, Debug)]
pub enum AST<'a> {
    Variable(&'a str),
    BooleanLiteral(TBoolean),
    StringLiteral(String),
    NumberLiteral(String),
    Let {
        variable_declaration: Box<VariableDeclaration<'a>>,
        variable_expression: Box<AST<'a>>,
        expression: Box<AST<'a>>,
    },
    Function {
        parameter: Box<VariableDeclaration<'a>>,
        body: Box<AST<'a>>,
    },
}
