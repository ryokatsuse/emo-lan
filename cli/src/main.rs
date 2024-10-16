use lexer::lex;
use parser::parse;

fn main() {
    let input = "📄🔤Hello World🔤🖼️(https://example.com/image.jpg)";
    // 字句解析
    let tokens = lex(input);
    println!("Tokens: {:?}", tokens);

     // 構文解析
    let ast = parse(tokens);
    println!("AST: {:?}", ast);
}