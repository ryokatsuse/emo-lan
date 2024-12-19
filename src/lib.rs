use wasm_bindgen::prelude::*;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref DOCUMENT_START: Regex = Regex::new(r"📄").unwrap();
    static ref TEXT_PATTERN: Regex = Regex::new(r"🔤([^🔤]*)🔤").unwrap();
    static ref IMAGE_PATTERN: Regex = Regex::new(r"🖼️\[(.*?)\]\((.*?)\)").unwrap();
}

#[wasm_bindgen]
pub fn compile_to_html(input: &str) -> String {
    println!("Raw input: {:?}", input);
    let tokens = lex(input);
    println!("Tokens: {:?}", tokens);
    let ast = parse(tokens);
    println!("AST: {:?}", ast);
    match analyze(&ast) {
        Ok(_) => match compile(&ast) {
            Ok(html) => {
                println!("Generated HTML: {}", html);
                html
            }
            Err(_) => "Compile error".to_string(),
        },
        Err(_) => "Semantic analysis error".to_string(),
    }
}

pub fn lex(input: &str) -> Vec<Token> {
    println!("Raw input to lex: {:?}", input);
    let input = input.trim();
    let mut tokens = Vec::new();

    if DOCUMENT_START.is_match(input) {
        tokens.push(Token::DocumentStart);
    }

    for cap in TEXT_PATTERN.captures_iter(input) {
        if let Some(text) = cap.get(1) {
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

    if tokens.is_empty() {
        tokens.push(Token::Unknown);
    }

    println!("Tokens: {:?}", tokens);
    tokens
}

pub fn parse(tokens: Vec<Token>) -> ASTNode {
    let mut nodes = Vec::new();

    for token in tokens {
        match token {
            Token::DocumentStart => nodes.push(ASTNode::DocumentStart),
            Token::Text(text) => nodes.push(ASTNode::Paragraph(text)),
            Token::Image { url, alt } => nodes.push(ASTNode::Image { url, alt }),
            Token::Unknown => nodes.push(ASTNode::Unknown),
        }
    }

    ASTNode::Document(nodes)
}

pub fn analyze(ast: &ASTNode) -> Result<(), SemanticError> {
    match ast {
        ASTNode::Document(nodes) => {
            if nodes.is_empty() || !matches!(nodes[0], ASTNode::DocumentStart) {
                return Err(SemanticError::MissingDocumentStart);
            }
        }
        _ => {}
    }

    Ok(())
}

pub fn compile(ast: &ASTNode) -> Result<String, CompileError> {
    match ast {
        ASTNode::Document(nodes) => {
            let mut html = String::from("<!DOCTYPE html>\n<html>\n<body>\n");
            for node in nodes {
                html.push_str(&compile_node(node)?);
            }
            html.push_str("</body>\n</html>");
            Ok(html)
        }
        _ => Err(CompileError::GeneralError),
    }
}

fn compile_node(node: &ASTNode) -> Result<String, CompileError> {
    match node {
        ASTNode::DocumentStart => Ok(String::new()),
        ASTNode::Paragraph(text) => Ok(format!("<p>{}</p>\n", text)),
        ASTNode::Image { url, alt } => Ok(format!("<img src=\"{}\" alt=\"{}\" />\n", url, alt)),
        _ => Err(CompileError::GeneralError),
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    DocumentStart,
    Text(String),
    Image { url: String, alt: String },
    Unknown,
}

#[derive(Debug)]
pub enum ASTNode {
    DocumentStart,
    Paragraph(String),
    Image { url: String, alt: String },
    Document(Vec<ASTNode>),
    Unknown,
}

#[derive(Debug)]
pub enum SemanticError {
    MissingDocumentStart,
}

#[derive(Debug)]
pub enum CompileError {
    GeneralError,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_to_html() {
        let input = "📄🔤Hello World🔤🖼️[サンプル画像](https://example.com/image.jpg)";
        let output = compile_to_html(input);
        assert_eq!(
            output,
            r#"<!DOCTYPE html>
<html>
<body>
<p>Hello World</p>
<img src="https://example.com/image.jpg" alt="サンプル画像" />
</body>
</html>"#
        );
    }
}