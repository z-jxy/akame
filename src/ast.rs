pub enum ASTNode {
    NumberNode { value: i32 },
    BinaryOpNode { op: BinaryOp, left: Box<ASTNode>, right: Box<ASTNode> },
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}
