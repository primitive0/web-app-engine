use crate::parser::ast;
use crate::parser::ast::Literal;
use crate::parser::ast::stmt::Statement;

// fn lang_type_to_c_type<'a>(type_: ast::Type<'a>) -> &'static str {
//     match type_ {
//         ast::Type::Int => "int",
//         _ => panic!("failed to map type to c"),
//     }
// }

pub fn generate_c_code(tree: &ast::AST) -> String {
    let mut buf = CodeBuf::new();
    buf.push_external_include("stdio.h");
    buf.push_linebreak();
    buf.push_linebreak();

    let mut main_function_found = false;
    for decl in tree.declarations() {
        if decl.name() == "main" && decl.return_type() == ast::TypeOrVoid::Void {
            main_function_found = true;
            generate_main_function(&mut buf, &decl);
            break;
        }
    }
    if !main_function_found {
        panic!("failed to generated c code: main function not found");
    }

    return buf.into_string();
}

struct CodeBuf {
    content: String,
}

impl CodeBuf {
    fn new() -> CodeBuf {
        CodeBuf {
            content: String::new(),
        }
    }

    fn push_function_decl(&mut self, return_type: &str, name: &str) {
        self.content.push_str(return_type);
        self.content.push_str(" ");
        self.content.push_str(name);
        self.content.push_str("()");
    }

    fn push_spaces(&mut self, count: usize) {
        self.content.extend((0..count).map(|_| ' '));
    }

    fn push_code_block_open(&mut self) {
        self.content.push_str("{");
    }

    fn push_code_block_close(&mut self) {
        self.content.push_str("}");
    }

    fn push_paren_open(&mut self) {
        self.content.push_str("(");
    }

    fn push_paren_close(&mut self) {
        self.content.push_str(")");
    }

    fn push_semicolon(&mut self) {
        self.content.push_str(";");
    }

    fn push_linebreak(&mut self) {
        self.content.push_str("\n");
    }

    fn push_ident(&mut self, ident: &str) {
        self.content.push_str(ident);
    }

    fn push_string_literal(&mut self, string: &str) {
        self.content.push_str("\"");
        self.content.push_str(string);
        self.content.push_str("\"");
    }

    fn push_function_return(&mut self, value: &str) {
        self.content.push_str("return ");
        self.content.push_str(value);
        self.content.push_str(";");
    }

    fn push_external_include(&mut self, what: &str) {
        self.content.push_str("#include <");
        self.content.push_str(what);
        self.content.push_str(">");
    }

    fn into_string(self) -> String {
        self.content
    }
}

fn generate_main_function(code: &mut CodeBuf, decl: &ast::FunctionDeclaration) {
    code.push_function_decl("int", "main");
    code.push_spaces(1);
    code.push_code_block_open();
    code.push_linebreak();

    for stmt in decl.stmts() {
        code.push_spaces(4);
        match stmt {
            Statement::VariableDeclaration(var_decl) => todo!("not supported yet"),
            Statement::FunctionCall(func_call) => {
                if func_call.name() != "print" {
                    todo!("other functions are not supported yet");
                }

                let mut args = func_call.args().iter();
                let string = match args.next().unwrap() {
                    Literal::String(s) => *s,
                    Literal::Integer(_) => panic!("string literal excepted"),
                };
                assert!(args.next().is_none(), "only one arg expected");

                code.push_ident("printf");
                code.push_paren_open();
                code.push_string_literal(string);
                code.push_paren_close();
                code.push_semicolon();
            }
        }
        code.push_linebreak();
    }

    code.push_linebreak();
    code.push_spaces(4);
    code.push_function_return("0");
    code.push_linebreak();
    code.push_code_block_close();
    code.push_linebreak();
}

//
// fn push_spaces(code: &mut String, count: usize) {
//     code.extend((0..count).map(|_| ' '));
// }
//
// fn push_code_block_open(code: &mut String) {
//     code.push_str("{");
// }
//
// fn push_code_block_close(code: &mut String) {
//     code.push_str("}");
// }
//
// fn push_variable_decl(code: &mut String, var_type: &str, name: &str) {
//     code.push_str(var_type);
//     code.push_str(" ");
//     code.push_str(name);
// }
//
// fn push_variable_assign(code: &mut String, value: i32) {
//     code.push_str("= ");
//     // TODO: replace
//     code.push_str(value.to_string().as_str());
// }
//
// fn push_semicolon(code: &mut String) {
//     code.push_str(";");
// }
//
// fn push_linebreak(code: &mut String) {
//     code.push_str("\n");
// }
//
// fn push_function_return(code: &mut String, value: &str) {
//     code.push_str("return ");
//     code.push_str(value);
//     code.push_str(";");
// }
