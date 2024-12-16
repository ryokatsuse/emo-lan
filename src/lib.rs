use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn compile_to_html(input: &str) -> String {
    let tokens = lex(input);
    let ast = parse(tokens);
    match analyze(&ast) {
        Ok(_) => match compile(&ast) {
            Ok(html) => html,
            Err(_) => "Compile error".to_string(),
        },
        Err(_) => "Semantic analysis error".to_string(),
    }
}

// 字句解析
pub fn lex(input: &str) -> Vec<Token> {
    // 簡易的な字句解析の実装
    if input.contains("📄") {
        vec![Token::DocumentStart]
    } else {
        vec![Token::Unknown]
    }
}

// 構文解析
pub fn parse(tokens: Vec<Token>) -> ASTNode {
    // 簡易的な構文解析の実装
    ASTNode::Document(tokens.into_iter().map(|t| match t {
        Token::DocumentStart => ASTNode::Document(vec![]),
        _ => ASTNode::Unknown,
    }).collect())
}

// 意味解析
pub fn analyze(ast: &ASTNode) -> Result<(), SemanticError> {
    // 簡易的な意味解析の実装
    match ast {
        ASTNode::Document(_) => Ok(()),
        _ => Err(SemanticError::MissingDocumentStart),
    }
}

// HTML生成
pub fn compile(ast: &ASTNode) -> Result<String, CompileError> {
    // 簡易的なHTML生成の実装
    match ast {
        ASTNode::Document(_) => Ok("<html><body></body></html>".to_string()),
        _ => Err(CompileError::GeneralError),
    }
}

// 必要な型定義
#[derive(Debug)]
pub enum Token {
    DocumentStart,
    Unknown,
}

#[derive(Debug)]
pub enum ASTNode {
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
        let input = "📄🔤Hello World🔤🖼️(https://example.com/image.jpg)";
        let output = compile_to_html(input);
        println!("Generated HTML:\n{}", output); // HTML出力を表示
        assert_eq!(
            output,
            r#"<!DOCTYPE html>
<html>
<body>
<p>Hello World</p>
<img src="https://example.com/image.jpg" alt="Image" />
</body>
</html>"#
        );
    }
}