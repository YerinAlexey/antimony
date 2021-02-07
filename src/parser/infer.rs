use super::node_type::*;

/// Try to infer types of variables
///
/// TODO: Global symbol table is passed around randomly.
/// This could probably be cleaned up.
pub(super) fn infer(program: &mut Program) -> Result<(), String> {
    let table = &program.get_symbol_table();
    // TODO: Fix aweful nesting
    for func in &mut program.func {
        if let Statement::Block(statements) = &mut func.body {
            for statement in statements {
                match statement {
                    Statement::Declare(var, expr) => {
                        if let None = &var.ty {
                            if let Some(e) = expr {
                                var.ty = infer_expression(&e, table);
                                // Debug info
                                if let None = var.ty {
                                    dbg!(&var.name);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

/// Function table is needed to infer possible function calls
fn infer_expression(expr: &Expression, table: &SymbolTable) -> Option<Type> {
    match expr {
        Expression::Int(_) => Some(Type::Int),
        Expression::Bool(_) => Some(Type::Bool),
        Expression::Str(_) => Some(Type::Str),
        Expression::FunctionCall(name, _) => infer_function_call(name, table),
        Expression::Array(els) => infer_array(els, table),
        _ => None,
    }
}

fn infer_array(elements: &Vec<Expression>, table: &SymbolTable) -> Option<Type> {
    let types: Vec<Option<Type>> = elements
        .iter()
        .map(|el| infer_expression(el, table))
        .collect();

    // TODO: This approach only relies on the first element.
    // It will not catch that types are possibly inconsistent.
    match types.first().and_then(|ty| ty.to_owned()) {
        Some(ty) => Some(Type::Array(Box::new(ty))),
        None => None,
    }
}

fn infer_function_call(name: &str, table: &SymbolTable) -> Option<Type> {
    match table.get(name) {
        Some(t) => t.to_owned(),
        None => None,
    }
}