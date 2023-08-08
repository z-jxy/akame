#[cfg(test)]
mod tests {
    use crate::{llvm::ast::{Expr, BinaryOp}, parsers::parse_program};
    use crate::llvm::ast::Stmt;

    #[test]
    fn test_function_declarations() -> anyhow::Result<()> {
        let input = r#"
        fn hello(num) {
            return num + 5;
        }

        fn main() {
            let ee = "hello";
            let e4 = std::args;
            let number = hello(3);
            printd(number);
        }
        "#;

        let parsed = parse_program(input)?;  // Assuming you have a function called `parse_program`

        let expected = vec![
            Stmt::FunctionDeclaration {
                ident: "hello".to_string(),
                params: vec!["num".to_string()],
                body: vec![Stmt::Return(Expr::Infix(
                    Box::new(Expr::Ident("num".to_string())),
                    BinaryOp::Add,  // Assuming you have an Operator enum
                    Box::new(Expr::Num(5)),
                ))],
            },
            Stmt::FunctionDeclaration {
                ident: "main".to_string(),
                params: vec![],
                body: vec![
                    Stmt::Assignment {
                        ident: "ee".to_string(),
                        expr: Expr::Str("hello".to_string()),
                    },
                    Stmt::Assignment {
                        ident: "e4".to_string(),
                        expr: Expr::QualifiedIdent(vec!["std".to_string(), "args".to_string()]),
                    },
                    Stmt::Assignment {
                        ident: "number".to_string(),
                        expr: Expr::Call(
                            "hello".to_string(),
                            vec![Expr::Num(3)],
                        ),
                    },
                    Stmt::Expression(Expr::Call(
                        "printd".to_string(),
                        vec![Expr::Ident("number".to_string())],
                    )),
                ],
            },
        ];

        assert_eq!(parsed, expected);
        Ok(())
    }
}
