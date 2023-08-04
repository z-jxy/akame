use crate::ast::{ASTNode, BinaryOp};

pub struct Interpreter;

impl Interpreter {
    pub fn visit(&self, node: ASTNode) -> i32 {
        match node {
            ASTNode::NumberNode { value } => value,
            ASTNode::BinaryOpNode { op, left, right } => {
                let left_value = self.visit(*left);
                let right_value = self.visit(*right);

                match op {
                    BinaryOp::Add => left_value + right_value,
                    BinaryOp::Subtract => left_value - right_value,
                    BinaryOp::Multiply => left_value * right_value,
                    BinaryOp::Divide => left_value / right_value,
                }
            }
        }
    }
}
