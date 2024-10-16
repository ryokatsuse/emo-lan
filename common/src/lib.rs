// トークンの定義
// Deriveを使うと特定のトレイトを自動的に実装してくれるらしい
// @see https://doc.rust-jp.rs/rust-by-example-ja/trait/derive.html
// Debugと等価比較を使うために必要
#[derive(Debug, PartialEq)]
pub enum Token {
    DocumentStart,    // 📄(DOCTYPE)
    Text(String),     // 🔤(Paragraph)
    Image(String),    // 🖼️(Image URL)
    Unknown,          // 指定されていない絵文字トークン
}

// 抽象構文木 (AST) の定義
#[derive(Debug, PartialEq)]

pub enum ASTNode {
    Document(Vec<ASTNode>),  // 📄(DOCTYPE)
    Paragraph(String),       // 🔤(Paragraph)
    Image(String),           // 🖼️(Image URL)
    Unknown,                 // 指定されていない絵文字トークン
}
