use crate::parser::ast;

fn lang_type_to_c_type<'a>(type_: ast::Type<'a>) -> &'static str {
   match type_ {
       ast::Type::Int => "int",
       _ => panic!("failed to map type to c"),
   }
}

pub fn generate_c_code(tree: ast::AST) -> String {
    let mut code = String::new();

    push_function_decl(&mut code, "int", "main");
    push_spaces(&mut code, 1);
    push_code_block_open(&mut code);
    push_linebreak(&mut code);

    for decl in tree.declarations() {
        let var_type = lang_type_to_c_type(decl.var_type());
        let name = decl.name().name;
        let value = decl.value().value;
        push_spaces(&mut code, 4);
        push_variable_decl(&mut code, var_type, name);
        push_spaces(&mut code, 1);
        push_variable_assign(&mut code, value);
        push_semicolon(&mut code);
        push_linebreak(&mut code);
    }

    push_linebreak(&mut code);
    push_spaces(&mut code, 4);
    code.push_str("return 0");
    push_semicolon(&mut code);
    push_linebreak(&mut code);
    push_code_block_close(&mut code);
    push_linebreak(&mut code);

    return code;
}

// TODO: use iterators for less allocations

fn push_function_decl(code: &mut String, return_type: &str, name: &str) {
    code.push_str(return_type);
    code.push_str(" ");
    code.push_str(name);
    code.push_str("()");
}

fn push_spaces(code: &mut String, count: usize) {
    code.extend((0..count).map(|_| ' '));
}

fn push_code_block_open(code: &mut String) {
    code.push_str("{");
}

fn push_code_block_close(code: &mut String) {
    code.push_str("}");
}

fn push_variable_decl(code: &mut String, var_type: &str, name: &str) {
    code.push_str(var_type);
    code.push_str(" ");
    code.push_str(name);
}

fn push_variable_assign(code: &mut String, value: i32) {
    code.push_str("= ");
    // TODO: replace
    code.push_str(value.to_string().as_str());
}

fn push_semicolon(code: &mut String) {
    code.push_str(";");
}

fn push_linebreak(code: &mut String) {
    code.push_str("\n");
}
