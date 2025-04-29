use crate::interpreter::ast::expr::assignment::Assign;
use crate::interpreter::ast::expr::binary::Binary;
use crate::interpreter::ast::expr::grouping::Grouping;
use crate::interpreter::ast::expr::literal::Literal;
use crate::interpreter::ast::expr::logical::Logical;
use crate::interpreter::ast::expr::unary::Unary;
use crate::interpreter::ast::expr::variable::Variable;
use crate::interpreter::ast::expr::{Expr, ExprVisitor};

pub struct AstPrinter;

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary(&mut self, binary: &Binary<String>) -> String {
        self.parenthesize(
            binary.get_op_lexeme(),
            vec![binary.get_left(), binary.get_right()],
        )
    }

    fn visit_grouping(&mut self, grouping: &Grouping<String>) -> String {
        self.parenthesize("group", vec![grouping.get_expr()])
    }

    fn visit_literal(&mut self, literal: &Literal) -> String {
        literal.to_string()
    }

    fn visit_unary(&mut self, unary: &Unary<String>) -> String {
        self.parenthesize(unary.get_op_lexeme(), vec![unary.get_right()])
    }

    fn visit_variable(&mut self, variable: &Variable) -> String {
        self.parenthesize("variable", vec![variable])
    }

    fn visit_assign(&mut self, assign: &Assign<String>) -> String {
        self.parenthesize("assign", vec![assign.get_value()])
    }

    fn visit_logical(&mut self, logical: &Logical<String>) -> String {
        self.parenthesize(
            logical.get_operator().get_lexeme(),
            vec![logical.get_left(), logical.get_right()],
        )
    }
}

impl AstPrinter {
    pub fn print(&mut self, expr: &dyn Expr<String>) -> String {
        expr.accept(self)
    }

    pub fn parenthesize(&mut self, name: &str, exprs: Vec<&dyn Expr<String>>) -> String {
        let mut builder = String::new();
        builder.push('(');
        builder.push_str(name);
        for expr in exprs {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');
        builder
    }
}

#[cfg(test)]
mod tests {
    use crate::b;
    use crate::interpreter::ast::expr::binary::Binary;
    use crate::interpreter::ast::expr::grouping::Grouping;
    use crate::interpreter::ast::expr::literal::Literal;
    use crate::interpreter::ast::expr::unary::Unary;
    use crate::interpreter::ast::printer::AstPrinter;
    use crate::interpreter::scanner::token::object::Object::Number;
    use crate::interpreter::scanner::token::token_type::TokenType::{Minus, Star};
    use crate::interpreter::scanner::token::Token;

    #[test]
    fn test_print() {
        let expr = Binary::new(
            b!(Unary::new(
                Token::new(Minus, "-", None, 1, 1),
                b!(Literal::new(Some(Number(123.))))
            )),
            Token::new(Star, "*", None, 1, 1),
            b!(Grouping::new(b!(Literal::new(Some(Number(45.67)))))),
        );
        assert_eq!("(* (- 123) (group 45.67))", AstPrinter.print(&expr))
    }
}
