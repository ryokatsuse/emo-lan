use lexer::lex;
use parser::{parse, analyze, compile, save_to_file};
use std::env;
use std::fs;

fn main() {
    // コマンドライン引数からファイルパスを取得
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: emo-lan <file.el>");
        return;
    }

    // .elファイルを読み込む
    let filename = &args[1];
    println!("Attempting to read file: {}", filename); // デバッグ出力
    let content = fs::read_to_string(filename).expect("Failed to read the .el file");

    // 読み込んだ内容をinputとして扱う
    let input = content.trim(); // ファイル内容をそのまま処理

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