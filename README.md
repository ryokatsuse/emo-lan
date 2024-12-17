# emo-lan

A programming language that uses emoji pictograms to generate HTML. Write your content using emojis, and emo-lan will compile it into clean HTML.

## Features

- 📄 Document declaration using emojis
- 🔤 Text content support
- 🖼️ Image embedding
- Command-line interface
- WebAssembly support

## Syntax

| Emoji | Description | HTML Output |
|-------|-------------|-------------|
| 📄 | Document start (required) | `<!DOCTYPE html>` |
| 🔤text🔤 | Text content | `<p>text</p>` |
| 🖼️(url) | Image | `<img src="url" alt="Image" />` |

### Example

```
📄🔤Hello World🔤🖼️(https://example.com/image.jpg)
```

Generates:

```html
<!DOCTYPE html>
<html>
<body>
<p>Hello World</p>
<img src="https://example.com/image.jpg" alt="Image" />
</body>
</html>
```

## Installation

### Prerequisites

- Rust toolchain (1.56 or later)
- Cargo package manager

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/ryokatsuse/emo-lan
cd emo-lan
```

2. Build the project:
```bash
cargo build
```

3. Run tests:
```bash
cargo test
```

## Usage

### Online Playground

Try emo-lan directly in your browser using our [online playground](https://emo-lan-playground.vercel.app/).

### Command Line Interface

Create a file with `.el` extension and write your emo-lan code:

```bash
emo-lan input.el
```

The compiled HTML will be saved as `output.html`.

### WebAssembly Integration

The library also supports WebAssembly for web browser usage.

## Project Structure

- `cli/`: Command-line interface implementation
- `lexer/`: Tokenization and lexical analysis
- `parser/`: Syntax parsing and HTML generation
- `common/`: Shared types and utilities
- `src/`: Core library and WebAssembly bindings

## License

MIT

## Author

[infixer (ryokatsu0719@gmail.com)](https://github.com/ryokatsuse)


