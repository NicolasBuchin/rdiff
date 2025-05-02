use align::{needleman_wunsch_char_align, needleman_wunsch_word_align};
use clap::Parser;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
mod align;

#[derive(Parser)]
struct Args {
    file1: String,
    file2: String,

    #[arg(short = 'c', long = "char")]
    by_char: bool,

    #[arg(long = "stats")]
    stats: bool,
}

#[derive(Default)]
struct DiffStats {
    insertions: usize,
    deletions: usize,
    substitutions: usize,
    changed_lines: usize,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let f1 = BufReader::new(File::open(&args.file1)?);
    let f2 = BufReader::new(File::open(&args.file2)?);

    let lines1: Vec<_> = f1.lines().collect::<Result<_, _>>()?;
    let lines2: Vec<_> = f2.lines().collect::<Result<_, _>>()?;

    let diffs: Vec<_> = lines1
        .par_iter()
        .zip(&lines2)
        .enumerate()
        .filter_map(|(i, (s1, s2))| {
            if s1 != s2 {
                Some((i, s1.clone(), s2.clone()))
            } else {
                None
            }
        })
        .collect();

    let mut total_stats = DiffStats::default();

    for (i, l1, l2) in diffs.iter() {
        let (a1, a2, stats) = if args.by_char {
            needleman_wunsch_char_align(l1, l2)
        } else {
            needleman_wunsch_word_align(l1, l2)
        };
        println!("diff at {}:\n<{}\n>{}", i, a1, a2);

        if args.stats {
            total_stats.changed_lines += 1;
            total_stats.insertions += stats.insertions;
            total_stats.deletions += stats.deletions;
            total_stats.substitutions += stats.substitutions;
        }
    }

    if args.stats {
        println!(
            "\nSummary: {} lines changed, {} insertions, {} deletions, {} substitutions",
            total_stats.changed_lines,
            total_stats.insertions,
            total_stats.deletions,
            total_stats.substitutions
        );
    }

    Ok(())
}
