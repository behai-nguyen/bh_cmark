/* 25/05/2026 */

use bh_cmark::scanner::Scanner;
use bh_cmark::parser::parser::Parser;
use bh_cmark::ast::AstBlock;

fn main() {
    let markdown = "**Không đọc *sử* không đủ tư cách nói chuyện *chính trị*.**";
    // let markdown = "# **Thỏa Hiệp Án *Fontainebleau 14/09/1946*: ông Hồ cấu kết \
    //     với Pháp để *tiêu diệt* các đảng quốc gia.**";
    // let markdown = "![***Mount Fuji* / *富士山, ふじさ, Fujisan* / \
    //     *Núi Phú Sỹ***](./img/fujisan.png)";
    // let markdown = "   ***\n____\n----    ";
    // let markdown = "    self.current += 1;\n// This is a comment.";

    // This Markdown contains a syntax error.
    // let markdown = "![a [[nested]] text(img.png)";

    let mut scanner = Scanner::new(markdown);

    let tokens = scanner
        .scan_tokens()
        .expect("Scanning should succeed");
    
    let mut parser = Parser::new(&tokens);

    let parse_output = parser.parse();

    if parse_output.has_error() {
        println!("\n");
        for err in parse_output.errors() {
            println!("{err}");
        }
    }

    println!("\n{:?}", parse_output);

    for block in parse_output.blocks() {
        match block {
            AstBlock::Header { level, content } => {
                println!("\nlevel: {level}");                
                println!("\n{}", content);
            }
            AstBlock::Paragraph { content } => {
                println!("\n{}", content);
            }
            AstBlock::Image { path, alt } => {
                println!("\npath: {path}");
                println!("\n{}", alt);
            }
            AstBlock::Thematic => {
                println!("\nHorizontal break");
            }
            AstBlock::Code { language, content } => {
                println!("\nlanguage: {:?}", language);
                println!("\n{content}");
            }
        }
    }
}
