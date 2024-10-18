// トークンの定義
// Deriveを使うと特定のトレイトを自動的に実装してくれるらしい
// @see https://doc.rust-jp.rs/rust-by-example-ja/trait/derive.html
// Debugと等価比較を使うために必要
#[derive(Debug, PartialEq)]
pub enum Token {
    DocumentStart,    // 📄(DOCTYPE)
    Text(String),     // 🔤(Paragraph)
    Image(String),    // 🖼️(Image URL)
    Unknown,          // 不明なトークン
}

// 抽象構文木 (AST) の定義

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    DocumentStart,            // ドキュメント開始を表すノード
    Document(Vec<ASTNode>),    // ドキュメントルート
    Paragraph(String),         // テキスト要素
    Image(String),             // 画像要素
    Unknown,                   // 不明な要素
}