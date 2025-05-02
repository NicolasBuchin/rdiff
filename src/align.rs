use colored::*;

use crate::DiffStats;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Diag,
    Up,
    Left,
}

pub fn needleman_wunsch_word_align(s1: &str, s2: &str) -> (String, String, DiffStats) {
    let words1: Vec<&str> = s1.split_whitespace().collect();
    let words2: Vec<&str> = s2.split_whitespace().collect();

    let m = words1.len();
    let n = words2.len();

    let match_score = 2;
    let mismatch_penalty = -1;
    let gap_penalty = -2;

    let mut score = vec![vec![0; n + 1]; m + 1];
    let mut backtrack = vec![vec![Direction::Diag; n + 1]; m + 1];

    for i in 1..=m {
        score[i][0] = i as i32 * gap_penalty;
        backtrack[i][0] = Direction::Up;
    }
    for j in 1..=n {
        score[0][j] = j as i32 * gap_penalty;
        backtrack[0][j] = Direction::Left;
    }

    for i in 1..=m {
        for j in 1..=n {
            let diag = score[i - 1][j - 1]
                + if words1[i - 1] == words2[j - 1] {
                    match_score
                } else {
                    mismatch_penalty
                };
            let up = score[i - 1][j] + gap_penalty;
            let left = score[i][j - 1] + gap_penalty;

            let (best, dir) = [
                (diag, Direction::Diag),
                (up, Direction::Up),
                (left, Direction::Left),
            ]
            .into_iter()
            .max_by_key(|(v, _)| *v)
            .unwrap();

            score[i][j] = best;
            backtrack[i][j] = dir;
        }
    }

    let mut aligned1 = Vec::new();
    let mut aligned2 = Vec::new();

    let (mut i, mut j) = (m, n);
    while i > 0 || j > 0 {
        match backtrack[i][j] {
            Direction::Diag => {
                aligned1.push(Some(words1[i - 1]));
                aligned2.push(Some(words2[j - 1]));
                i -= 1;
                j -= 1;
            }
            Direction::Up => {
                aligned1.push(Some(words1[i - 1]));
                aligned2.push(None);
                i -= 1;
            }
            Direction::Left => {
                aligned1.push(None);
                aligned2.push(Some(words2[j - 1]));
                j -= 1;
            }
        }
    }

    aligned1.reverse();
    aligned2.reverse();

    let mut out1 = Vec::new();
    let mut out2 = Vec::new();
    let mut stats = DiffStats::default();

    for (w1, w2) in aligned1.into_iter().zip(aligned2.into_iter()) {
        match (w1, w2) {
            (Some(a), Some(b)) if a == b => {
                out1.push(a.white().to_string());
                out2.push(b.white().to_string());
            }
            (Some(a), Some(b)) => {
                out1.push(a.yellow().to_string());
                out2.push(b.yellow().to_string());
                stats.substitutions += 1;
            }
            (Some(a), None) => {
                out1.push(a.red().to_string());
                stats.deletions += 1;
            }
            (None, Some(b)) => {
                out2.push(b.green().to_string());
                stats.insertions += 1;
            }
            _ => {}
        }
    }

    (out1.join(" "), out2.join(" "), stats)
}

pub fn needleman_wunsch_char_align(s1: &str, s2: &str) -> (String, String, DiffStats) {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();

    let m = chars1.len();
    let n = chars2.len();

    let match_score = 2;
    let mismatch_penalty = -1;
    let gap_penalty = -2;

    let mut score = vec![vec![0; n + 1]; m + 1];
    let mut backtrack = vec![vec![Direction::Diag; n + 1]; m + 1];

    for i in 1..=m {
        score[i][0] = i as i32 * gap_penalty;
        backtrack[i][0] = Direction::Up;
    }
    for j in 1..=n {
        score[0][j] = j as i32 * gap_penalty;
        backtrack[0][j] = Direction::Left;
    }

    for i in 1..=m {
        for j in 1..=n {
            let diag = score[i - 1][j - 1]
                + if chars1[i - 1] == chars2[j - 1] {
                    match_score
                } else {
                    mismatch_penalty
                };

            let up = score[i - 1][j] + gap_penalty;
            let left = score[i][j - 1] + gap_penalty;

            let (best, dir) = [
                (diag, Direction::Diag),
                (up, Direction::Up),
                (left, Direction::Left),
            ]
            .into_iter()
            .max_by_key(|(v, _)| *v)
            .unwrap();

            score[i][j] = best;
            backtrack[i][j] = dir;
        }
    }

    let mut aligned1 = Vec::new();
    let mut aligned2 = Vec::new();
    let (mut i, mut j) = (m, n);

    while i > 0 || j > 0 {
        match backtrack[i][j] {
            Direction::Diag => {
                aligned1.push(Some(chars1[i - 1]));
                aligned2.push(Some(chars2[j - 1]));
                i -= 1;
                j -= 1;
            }
            Direction::Up => {
                aligned1.push(Some(chars1[i - 1]));
                aligned2.push(None);
                i -= 1;
            }
            Direction::Left => {
                aligned1.push(None);
                aligned2.push(Some(chars2[j - 1]));
                j -= 1;
            }
        }
    }

    aligned1.reverse();
    aligned2.reverse();

    let mut out1 = String::new();
    let mut out2 = String::new();
    let mut stats = DiffStats::default();

    for (x1, x2) in aligned1.into_iter().zip(aligned2.into_iter()) {
        match (x1, x2) {
            (Some(a), Some(b)) if a == b => {
                out1.push_str(&a.to_string().white().to_string());
                out2.push_str(&b.to_string().white().to_string());
            }
            (Some(a), Some(b)) => {
                out1.push_str(&a.to_string().yellow().to_string());
                out2.push_str(&b.to_string().yellow().to_string());
                stats.substitutions += 1;
            }
            (Some(a), None) => {
                out1.push_str(&a.to_string().red().to_string());
                // out2.push(' ');
                stats.deletions += 1;
            }
            (None, Some(b)) => {
                // out1.push(' ');
                out2.push_str(&b.to_string().green().to_string());
                stats.insertions += 1;
            }
            _ => {}
        }
    }

    (out1, out2, stats)
}
