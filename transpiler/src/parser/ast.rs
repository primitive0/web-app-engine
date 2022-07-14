#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Type<'a> {
    Byte,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    Double,
    Financial,
    Bool,
    String,
    Rune,
    Dyn,
    Custom { name: &'a str },
}

#[derive(Copy, Clone, Debug)]
pub enum TypeOrVoid<'a> {
    Void,
    Type(Type<'a>),
}

#[derive(Copy, Clone, Debug)]
pub struct Ident<'a> {
    pub name: &'a str,
}

#[derive(Copy, Clone, Debug)]
pub struct IntegerLiteral {
    pub value: i32,
}

impl IntegerLiteral {
    pub fn new(value: i32) -> IntegerLiteral {
        IntegerLiteral { value }
    }
}

#[derive(Debug)]
pub struct VariableDeclaration<'a> {
    var_type: Type<'a>,
    name: Ident<'a>,
    value: IntegerLiteral,
}

impl<'a> VariableDeclaration<'a> {
    pub fn var_type(&self) -> Type<'a> {
        self.var_type
    }

    pub fn name(&self) -> Ident<'a> {
        self.name
    }

    pub fn value(&self) -> IntegerLiteral {
        self.value
    }
}

impl<'a> VariableDeclaration<'a> {
    pub fn new(var_type: Type<'a>, name: Ident<'a>, value: IntegerLiteral) -> VariableDeclaration<'a> {
        VariableDeclaration { var_type, name, value }
    }
}

#[derive(Debug)]
pub struct AST<'a> {
    declarations: Vec<VariableDeclaration<'a>>
}

impl<'a> AST<'a> {
    pub fn new(declarations: Vec<VariableDeclaration<'a>>) -> AST<'a> {
        AST {
            declarations
        }
    }

    pub fn declarations(&self) -> &[VariableDeclaration<'a>] {
        self.declarations.as_slice()
    }
}
