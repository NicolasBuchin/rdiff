# rdiff

A fast, parallel diff tool written in Rust that highlights differences **word-by-word** or **character-by-character** using a Smith-Waterman-style alignment.

## Installation

```bash
git clone https://github.com/NicolasBuchin/rdiff.git 
cd rdiff
cargo build --release
````

This will build the binary at `target/release/rdiff`.

## Usage

```bash
rdiff <file1> <file2>
```

### Options

* `-c`, `--char` — Compare lines character-by-character (default is word-by-word)

Colors:

* **White**: Identical parts
* **Yellow**: Substitutions
* **Red**: Deletions
* **Green**: Insertions

## 📄 License

MIT

