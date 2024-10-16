use lexer::Token;

// 抽象構文木 (AST) の定義
#[derive(Debug)]
pub enum ASTNode {
    Document(Vec<ASTNode>),  // 📄(DOCTYPE)
    Paragraph(String),       // 🔤(Paragraph)
    Image(String),           // 🖼️(Image URL)
    Unknown,                 // 指定されていない絵文字トークン
}

// 構文解析を実行する
pub fn parse(tokens: Vec<Token>) -> ASTNode {
    let mut nodes = Vec::new();

    for token in tokens {
        match token {
            Token::DocumentStart => {
                // ドキュメントの開始時には何もしない？
            }
            Token::Text(text) => {
                nodes.push(ASTNode::Paragraph(text));
            }
            Token::Image(url) => {
                nodes.push(ASTNode::Image(url));
            }
            Token::Unknown => {
                nodes.push(ASTNode::Unknown);
            }
        }
    }

    ASTNode::Document(nodes)
}