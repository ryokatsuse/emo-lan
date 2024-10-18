use lexer::lex;
use parser::{parse, analyze, compile, save_to_file};

fn main() {
    let input = "\u{1F4C4}\u{1F524}Hello World\u{1F524}\u{1F5BC}(https://example.com/image.jpg)";
    // 字句解析
    let tokens = lex(input);
    println!("Tokens: {:?}", tokens);

    // 構文解析
    let ast = parse(tokens);
    println!("AST: {:?}", ast);

    // 意味解析
    match analyze(&ast) {
        Ok(_) => println!("Semantic analysis passed!"),
        Err(e) => {
            println!("Semantic error: {:?}", e);
            return;
        }
    }

    // コンパイル
    match compile(&ast) {
        Ok(html) => {
            // コンソールにHTMLを表示
            println!("Generated HTML:\n{}", html);
            // ファイルにHTMLを保存
            if let Err(e) = save_to_file("output.html", &html) {
                println!("Failed to save HTML to file: {:?}", e);
            } else {
                println!("HTML successfully saved to output.html");
            }
        }
        Err(e) => {
            println!("Compile error: {:?}", e);
        }
    }
}