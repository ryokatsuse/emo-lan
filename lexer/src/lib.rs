use common::Token;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref DOCUMENT_START: Regex = Regex::new(r"📄").unwrap();
    static ref TEXT_PATTERN: Regex = Regex::new(r"🔤([^🔤]*)🔤").unwrap();
    static ref IMAGE_PATTERN: Regex = Regex::new(r"🖼️\[(.*?)\]\((.*?)\)").unwrap();
}

pub fn lex(input: &str) -> Vec<Token> {
    println!("Raw input to lex: {:?}", input);
    let input = input.trim();
    println!("Trimmed input to lex: {:?}", input);
    let mut tokens = Vec::new();

    if DOCUMENT_START.is_match(input) {
        println!("DocumentStart matched");
        tokens.push(Token::DocumentStart);
    }

    for cap in TEXT_PATTERN.captures_iter(input) {
        if let Some(text) = cap.get(1) {
            println!("Text matched: {}", text.as_str());
            tokens.push(Token::Text(text.as_str().to_string()));
        }
    }

    for cap in IMAGE_PATTERN.captures_iter(input) {
        if let (Some(alt), Some(url)) = (cap.get(1), cap.get(2)) {
            tokens.push(Token::Image {
                url: url.as_str().to_string(),
                alt: alt.as_str().to_string(),
            });
        }
    }

    // トークンが生成されない場合はUnknownを追加する
    if tokens.is_empty() {
        println!("No tokens matched");
        tokens.push(Token::Unknown);
    }

    println!("Tokens: {:?}", tokens);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let input = "📄🔤Hello World🔤🖼️[alt text](https://example.com/image.jpg)";
        let tokens = lex(input);
        println!("Tokens: {:?}", tokens);
        assert_eq!(tokens, vec![
            Token::DocumentStart,
            Token::Text("Hello World".to_string()),
            Token::Image {
                url: "https://example.com/image.jpg".to_string(),
                alt: "alt text".to_string()
            }
        ]);
    }

    #[test]
    fn test_unknown() {
        let input = "🚀";
        let tokens = lex(input);
        println!("Tokens: {:?}", tokens);
        assert_eq!(tokens, vec![Token::Unknown]);
    }
}