use align::*;
use clap::Parser;
use format::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
mod align;
mod format;

#[derive(Parser)]
struct Args {
    file1: String,
    file2: String,

    #[arg(short = 'c', long = "char")]
    char: bool,

    #[arg(short = 't', long = "tags")]
    tags: bool,

    #[arg(long = "tags-type")]
    tags_type: Option<String>,

    #[arg(long = "stats")]
    stats: bool,
}

#[derive(Default)]
struct DiffStats {
    insertions: usize,
    deletions: usize,
    substitutions: usize,
    changed_lines: usize,

    field_stats: Option<Box<dyn FieldStats>>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let use_tags = args.tags || args.tags_type.is_some();

    let mut stats = DiffStats::default();
    let fmt: Option<Box<dyn Format>> =
        args.tags_type
            .as_ref()
            .and_then(|s| match s.to_uppercase().as_str() {
                "SAM" => Some(Box::new(SamFormat) as Box<dyn Format>),
                "PAF" => Some(Box::new(PafFormat) as Box<dyn Format>),
                other => {
                    eprintln!("Warning: unknown tags-type '{}', using basic tags", other);
                    None
                }
            });
    if let Some(f) = fmt {
        stats.field_stats = Some(Box::new(GenericStats::new(f)));
    }

    let f1 = BufReader::new(File::open(&args.file1)?);
    let f2 = BufReader::new(File::open(&args.file2)?);

    for (i, (l1, l2)) in f1.lines().zip(f2.lines()).enumerate() {
        let l1 = l1?;
        let l2 = l2?;

        if l1 == l2 {
            continue;
        }

        stats.changed_lines += 1;

        let (a1, a2) = if args.char {
            char_align(&l1, &l2, &mut stats)
        } else {
            word_align(&l1, &l2, use_tags, &mut stats)
        };

        println!("diff at {}:\n<{}\n>{}", i, a1, a2);
    }

    if args.stats {
        println!(
            "\nSummary: {} lines changed, {} insertions, {} deletions, {} substitutions",
            stats.changed_lines, stats.insertions, stats.deletions, stats.substitutions
        );

        if let Some(ref fs) = stats.field_stats {
            fs.report();
        }
    }

    Ok(())
}
