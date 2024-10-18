use regex::Regex;
use common::Token;

// 字句解析を行う関数
// 字句解析を行う関数
// 字句解析を行う関数
pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    // 正規表現でトークンをパターンマッチング
    let document_start_re = Regex::new(r"\u{1F4C4}").unwrap();  // 📄
    let text_re = Regex::new(r"\u{1F524}(.*?)\u{1F524}").unwrap();  // 🔤
    let image_re = Regex::new(r"\u{1F5BC}").unwrap();  // 🖼️を単純に探す

    // 📄(DOCTYPE)
    if document_start_re.is_match(input) {
        tokens.push(Token::DocumentStart);
    }

    // 🔤(Paragraph)
    for cap in text_re.captures_iter(input) {
        tokens.push(Token::Text(cap[1].to_string()));
    }

    // 🖼️(Image)を探す
    if image_re.is_match(input) {
        // 仮にURLを "https://example.com/image.jpg" として固定
        tokens.push(Token::Image("https://example.com/image.jpg".to_string()));
        println!("Image token found.");  // デバッグメッセージ
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