use common::{Token, ASTNode};

// 構文解析を実行する
pub fn parse(tokens: Vec<Token>) -> ASTNode {
    let mut nodes = Vec::new();

    for token in tokens {
        match token {
            Token::DocumentStart => {}
            Token::Text(text) => nodes.push(ASTNode::Paragraph(text)),
            Token::Image(url) => nodes.push(ASTNode::Image(url)),
            Token::Unknown => nodes.push(ASTNode::Unknown),
        }
    }

    ASTNode::Document(nodes)
}