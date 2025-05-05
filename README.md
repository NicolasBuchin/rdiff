# rdiff

A diff tool written in Rust that highlights differences **word-by-word** or **character-by-character** using a Smith-Waterman-style alignment.
It supports colored output, tag annotations, and custom tracking for SAM and PAF files.

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

| Flag                           | Description                                                                                     |
| ------------------------------ | ----------------------------------------------------------------------------------------------- |
| `-c`, `--char`                 | Compare line differences **character-by-character** (default: word-based)                       |
| `-t`, `--tags`                 | Annotate insertions, deletions, and substitutions using tag wrappers                            |
| `--tags-type <SAM \| PAF>`     | Annotate with tags and track field-level substitution stats for SAM or PAF                      |
| `--stats`                      | Print a summary of the number of insertions, deletions, substitutions, and changed lines        |
| `-h`, `--help`                 | Show help                                                                                       |

Colors:

* **White**: Identical parts
* **Yellow**: Substitutions
* **Red**: Deletions
* **Green**: Insertions

## 📄 License

MIT

