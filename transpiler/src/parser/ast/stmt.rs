use super::*;

#[derive(Debug)]
pub struct VariableDeclaration<'a> {
    var_type: Type<'a>,
    name: Ident<'a>,
    value: Literal<'a>,
}

impl<'a> VariableDeclaration<'a> {
    pub fn var_type(&self) -> Type<'a> {
        self.var_type
    }

    pub fn name(&self) -> Ident<'a> {
        self.name
    }

    pub fn value(&self) -> Literal<'a> {
        self.value
    }
}

impl<'a> VariableDeclaration<'a> {
    pub fn new(var_type: Type<'a>, name: Ident<'a>, value: Literal<'a>) -> VariableDeclaration<'a> {
        VariableDeclaration { var_type, name, value }
    }
}

#[derive(Debug)]
pub struct FunctionCall<'a> {
    name: Ident<'a>,
    args: Vec<Literal<'a>>,
}

impl<'a> FunctionCall<'a> {
    pub fn new(name: Ident<'a>, args: Vec<Literal<'a>>) -> FunctionCall<'a> {
        FunctionCall { name, args }
    }

    pub fn name(&self) -> Ident<'a> {
        self.name
    }

    pub fn args(&self) -> &[Literal<'a>] {
        self.args.as_slice()
    }
}

#[derive(Debug)]
pub enum Statement<'a> {
    VariableDeclaration(VariableDeclaration<'a>),
    FunctionCall(FunctionCall<'a>),
}

impl<'a> Statement<'a> {
    pub fn var_decl(val: VariableDeclaration<'a>) -> Statement<'a> {
        Statement::VariableDeclaration(val)
    }

    pub fn function_call(val: FunctionCall<'a>) -> Statement<'a> {
        Statement::FunctionCall(val)
    }
}
