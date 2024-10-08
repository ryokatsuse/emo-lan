use regex::Regex;

// トークンの定義
// Deriveを使うと特定のトレイトを自動的に実装してくれるらしい
// @see https://doc.rust-jp.rs/rust-by-example-ja/trait/derive.html
// Debugと等価比較を使うために必要
#[derive(Debug, PartialEq)]

// 全絵文字トークンの定義
// TODO: あとで増えたときの管理をどうするか検討する
pub enum Token {
    DocumentStart,    // 📄(DOCTYPE)
    Text(String),     // 🔤(Paragraph)
    Image(String),    // 🖼️(Image URL)
    Unknown,          // 指定されていない絵文字トークン
}

// 字句解析を行う関数
pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    // 正規表現でトークンをパターンマッチング
    // TODO: 増えると大変なのでどうするかは後で考える
    let document_start_re = Regex::new(r"\u{1F4C4}").unwrap();  // 📄
    let text_re = Regex::new(r"\u{1F524}(.*?)\u{1F524}").unwrap();  // 🔤
    let image_re = Regex::new(r"\u{1F5BC}\((.*?)\)").unwrap();  // 🖼️


    // 📄(DOCTYPE)
    if document_start_re.is_match(input) {
        tokens.push(Token::DocumentStart);
    }

    // 🔤(Paragraph) pタグ
    for cap in text_re.captures_iter(input) {
        tokens.push(Token::Text(cap[1].to_string()));
    }

    // 画像（🖼️(URL)）
    for cap in image_re.captures_iter(input) {
        tokens.push(Token::Image(cap[1].to_string()));
    }

    // 絵文字トークンがない場合はUnknown
    if tokens.is_empty() {
        tokens.push(Token::Unknown);
    }

    tokens
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let input = "\u{1F4C4}\u{1F524}Hello World\u{1F524}\u{1F5BC}(https://example.com/image.jpg)";
        let tokens = lex(input);
        assert_eq!(tokens, vec![
            Token::DocumentStart,
            Token::Text("Hello World".to_string()),
            Token::Image("https://example.com/image.jpg".to_string())
        ]);
    }

    #[test]
    fn test_unknown() {
        let input = "\u{1F680}";  // 🚀
        let tokens = lex(input);
        assert_eq!(tokens, vec![Token::Unknown]);
    }
}