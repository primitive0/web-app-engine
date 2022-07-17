pub mod expr;
pub mod stmt;
use stmt::Statement;

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

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum TypeOrVoid<'a> {
    Void,
    Type(Type<'a>),
}

#[derive(Copy, Clone, Debug)]
pub struct Ident<'a> {
    pub name: &'a str,
}

impl<'a> PartialEq<&str> for Ident<'a> {
    fn eq(&self, other: &&str) -> bool {
        *self.name == **other
    }
}

// #[derive(Copy, Clone, Debug)]
// pub struct IntegerLiteral {
//     pub value: i32,
// }
//
// impl IntegerLiteral {
//     pub fn new(value: i32) -> IntegerLiteral {
//         IntegerLiteral { value }
//     }
// }

#[derive(Copy, Clone, Debug)]
pub enum Literal<'a> {
    Integer(i32),
    String(&'a str),
}

impl<'a> Literal<'a> {
    pub fn integer(val: i32) -> Literal<'a> {
        Literal::Integer(val)
    }

    pub fn string(val: &'a str) -> Literal<'a> {
        Literal::String(val)
    }
}

#[derive(Debug)]
pub struct FunctionArg<'a> {
    arg_type: Type<'a>,
    name: Ident<'a>,
}

impl<'a> FunctionArg<'a> {
    pub fn new(arg_type: Type<'a>, name: Ident<'a>) -> FunctionArg<'a> {
        FunctionArg { arg_type, name }
    }

    pub fn arg_type(&self) -> Type<'a> {
        self.arg_type
    }

    pub fn name(&self) -> Ident<'a> {
        self.name
    }
}

#[derive(Debug)]
pub struct FunctionDeclaration<'a> {
    return_type: TypeOrVoid<'a>,
    name: Ident<'a>,
    args: Vec<FunctionArg<'a>>,
    stmts: Vec<Statement<'a>>,
}

impl<'a> FunctionDeclaration<'a> {
    pub fn new(
        return_type: TypeOrVoid<'a>,
        name: Ident<'a>,
        args: Vec<FunctionArg<'a>>,
        stmts: Vec<Statement<'a>>,
    ) -> FunctionDeclaration<'a> {
        FunctionDeclaration {
            return_type,
            name,
            args,
            stmts,
        }
    }

    pub fn return_type(&self) -> TypeOrVoid<'a> {
        self.return_type
    }

    pub fn name(&self) -> Ident<'a> {
        self.name
    }

    pub fn args(&self) -> &[FunctionArg<'a>] {
        self.args.as_slice()
    }

    pub fn stmts(&self) -> &[Statement<'a>] {
        self.stmts.as_slice()
    }
}

#[derive(Debug)]
pub struct AST<'a> {
    declarations: Vec<FunctionDeclaration<'a>>,
}

impl<'a> AST<'a> {
    pub fn new(declarations: Vec<FunctionDeclaration<'a>>) -> AST<'a> {
        AST { declarations }
    }

    pub fn declarations(&self) -> &[FunctionDeclaration<'a>] {
        self.declarations.as_slice()
    }
}
