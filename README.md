<!-- 02/06/2026. -->

# **bh_cmark** — A Minimal Dependency-Free CommonMark Parser in Rust

A pure Rust, dependency-free Markdown parser targeting CommonMark compliance.

Parser behaviour is validated against the CommonMark reference implementation [https://spec.commonmark.org/dingus/](https://spec.commonmark.org/dingus/).

## Supported Features

### Emphasis

```markdown
**bold \\Úc Đại Lợi\\**
**Không đọc *sử* không đủ tư cách nói chuyện *chính trị*.**
\\( Chính Ðạo, Việt Nam Niên Biểu, Tập 1A, trang 347 \\)
*****Hello*world****
a*"b"*c
```

### Headers

Header blocks must start on a new line.

```markdown
# **Thỏa Hiệp Án *Fontainebleau 14/09/1946*: ông Hồ cấu kết với Pháp để *tiêu diệt* các đảng quốc gia.**
## ABC \# Ô ô Ơ ơ \# Ư ư Ứ ứ
```

The following contains four trailing spaces:

```markdown
####    
```

### Images

Image blocks must start on a new line.

```markdown
![](relative/path/to/image.png)
![Đây Là Chú Thích Của Hình](relative/path/to/image.png)
![***Mount Fuji* / *富士山, ふじさ, Fujisan* / *Núi Phú Sỹ***](./img/fujisan.png)
```

Captions may contain escape markers. For example:

```markdown
![Mount Fuji / \(富士山, ふじさ, Fujisan\) / \[Núi Phú Sỹ\]](./img/fujisan.png)
```

Escape markers may also be omitted:

```markdown
![Mount Fuji / (富士山, ふじさ, Fujisan) / [Núi Phú Sỹ]](./img/fujisan.png)
```

### Thematic Breaks

Thematic breaks support all CommonMark markers: `*` (asterisk), `-` (dash 
or hyphen), and `_` (underscore).

```markdown
***
****
___
____
---
----
```

Other pathological constructs permitted by CommonMark are also supported.

### Indented Code Blocks

```markdown
    ****
     ****
    self.current += 1;\n// This is a comment.     
```

## Project Goal

The primary objective is to provide a CommonMark-oriented Markdown parser for converting Markdown into PDFs while preserving formatting for inline emphasis, headers, image blocks, and related constructs. This work is part of the [polyglot_pdf](https://github.com/behai-nguyen/polyglot_pdf) series.

As such, the parser output is intended to be consumed by text styling and rendering APIs 
provided by [Pango](https://www.gtk.org/docs/architecture/pango) and [Cairo](https://www.cairographics.org/).

The parser output cannot be used out of the box to generate HTML. See 
[tests/test_commonmark_spec.rs](https://github.com/behai-nguyen/bh_cmark/blob/main/tests/test_commonmark_spec.rs) for how to generate HTML using the parser output.

## Parser Example

```rust
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
```

## Building and Running the Parser Example

The `Cargo.toml` file does contains any `[[bin]]` section, s a result, the default `cargo build` and `cargo run` commands are not expected to work.

To build the parser example:

```bash
cargo build --example example_01
```

To run the parser example:

```bash
cargo run --example example_01
```

# License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. Or you can view the community standard [MIT License online](https://opensource.org/licenses/MIT).