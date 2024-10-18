use std::{fs::File, io::Write};

use common::{Token, ASTNode};

// 意味解析用のエラー型
#[derive(Debug, PartialEq)]  // PartialEqを追加して比較できるようにする
pub enum SemanticError {
    DocumentStartMissing, // 📄 が最初にない場合
    InvalidImageUrl,      // 不正な画像URL
}

// HTMLのコンパイルエラー型
#[derive(Debug)]
pub enum CompileError {
    InvalidAST,
}


// 構文解析を実行する
pub fn parse(tokens: Vec<Token>) -> ASTNode {
    let mut nodes = Vec::new();

    for token in tokens {
        match token {
            Token::DocumentStart => nodes.push(ASTNode::DocumentStart), // DocumentStartノードを追加
            Token::Text(text) => nodes.push(ASTNode::Paragraph(text)),
            Token::Image(url) => nodes.push(ASTNode::Image(url)),
            Token::Unknown => nodes.push(ASTNode::Unknown),
        }
    }

    ASTNode::Document(nodes)
}

// 意味解析を行う関数
pub fn analyze(ast: &ASTNode) -> Result<(), SemanticError> {
    match ast {
        ASTNode::Document(nodes) => {
            // 最初の要素が DocumentStart であるかを確認
            if nodes.is_empty() || !matches!(nodes[0], ASTNode::DocumentStart) {
                return Err(SemanticError::DocumentStartMissing);
            }

            // 各ノードを再帰的にチェック
            for node in nodes.iter() {
                analyze(node)?;  // 再帰的に各ノードを解析
            }
        }
        ASTNode::Image(url) => {
            // 画像URLの検証
            if !url.starts_with("http://") && !url.starts_with("https://") {
                return Err(SemanticError::InvalidImageUrl);
            }
        }
        _ => {}
    }

    Ok(())
}

// ASTをHTMLに変換する関数
pub fn compile(ast: &ASTNode) -> Result<String, CompileError> {
    match ast {
        ASTNode::Document(nodes) => {
            let mut html = String::from("<!DOCTYPE html>\n<html>\n<body>\n");
            for node in nodes {
                html.push_str(&compile_node(node)?);  // 各ノードをHTMLに変換
            }
            html.push_str("</body>\n</html>");
            Ok(html)
        }
        _ => Err(CompileError::InvalidAST),  // ドキュメントでない場合はエラー
    }
}

// 各ASTノードをHTMLに変換する関数
fn compile_node(node: &ASTNode) -> Result<String, CompileError> {
    match node {
        ASTNode::DocumentStart => Ok(String::new()),  // DocumentStartはHTMLとしては無視
        ASTNode::Paragraph(text) => Ok(format!("<p>{}</p>\n", text)),  // Paragraph -> <p>
        ASTNode::Image(url) => {
            // Image -> <img>
            println!("Compiling image with URL: {}", url); // デバッグ用出力
            Ok(format!("<img src=\"{}\" alt=\"Image\" />\n", url))
        }
        _ => Err(CompileError::InvalidAST),
    }
}

pub fn save_to_file(filename: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;  // ファイルを作成
    file.write_all(content.as_bytes())?;  // コンテンツを書き込む
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_document() {
        let ast = ASTNode::Document(vec![
            ASTNode::DocumentStart,
            ASTNode::Paragraph("Hello".to_string()),
            ASTNode::Image("https://example.com/image.jpg".to_string())
        ]);
        assert!(analyze(&ast).is_ok());
    }

    #[test]
    fn test_missing_document_start() {
        let ast = ASTNode::Document(vec![
            ASTNode::Paragraph("Hello".to_string())
        ]);
        assert_eq!(analyze(&ast), Err(SemanticError::DocumentStartMissing));
    }

    #[test]
    fn test_invalid_image_url() {
        let ast = ASTNode::Document(vec![
            ASTNode::DocumentStart,
            ASTNode::Image("ftp://example.com/image.jpg".to_string())
        ]);
        assert_eq!(analyze(&ast), Err(SemanticError::InvalidImageUrl));
    }

    #[test]
    fn test_compile_valid_document() {
        let ast = ASTNode::Document(vec![
            ASTNode::DocumentStart,
            ASTNode::Paragraph("Hello".to_string()),
            ASTNode::Image("https://example.com/image.jpg".to_string())
        ]);
        let result = compile(&ast).unwrap();
        let expected_html = r#"<!DOCTYPE html>
<html>
<body>
<p>Hello</p>
<img src="https://example.com/image.jpg" alt="Image" />
</body>
</html>"#;
        assert_eq!(result.trim(), expected_html.trim());
    }
}