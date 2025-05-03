use crate::DiffStats;
use colored::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Diag,
    Up,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Cell {
    score: i32,
    from: Direction,
}

const MATCH_SCORE: i32 = 2;
const MISMATCH_PENALTY: i32 = -1;
const GAP_PENALTY: i32 = -2;

pub fn needleman_wunsch_word_align(s1: &str, s2: &str) -> (String, String, DiffStats) {
    let words1: Vec<&str> = s1.split(' ').collect();
    let words2: Vec<&str> = s2.split(' ').collect();

    let m = words1.len();
    let n = words2.len();

    let mut matrix = vec![
        Cell {
            score: 0,
            from: Direction::Diag
        };
        (m + 1) * (n + 1)
    ];
    let idx = |i: usize, j: usize| i * (n + 1) + j;

    for i in 1..=m {
        matrix[idx(i, 0)] = Cell {
            score: (i as i32) * GAP_PENALTY,
            from: Direction::Up,
        };
    }
    for j in 1..=n {
        matrix[idx(0, j)] = Cell {
            score: (j as i32) * GAP_PENALTY,
            from: Direction::Left,
        };
    }

    for i in 1..=m {
        for j in 1..=n {
            let match_or_mismatch = if words1[i - 1] == words2[j - 1] {
                MATCH_SCORE
            } else {
                MISMATCH_PENALTY
            };
            let diag = matrix[idx(i - 1, j - 1)].score + match_or_mismatch;
            let up = matrix[idx(i - 1, j)].score + GAP_PENALTY;
            let left = matrix[idx(i, j - 1)].score + GAP_PENALTY;

            let (best_score, best_direction) = if diag >= up && diag >= left {
                (diag, Direction::Diag)
            } else if up >= diag && up >= left {
                (up, Direction::Up)
            } else {
                (left, Direction::Left)
            };

            matrix[idx(i, j)] = Cell {
                score: best_score,
                from: best_direction,
            };
        }
    }

    let mut out1 = Vec::with_capacity(m);
    let mut out2 = Vec::with_capacity(n);
    let mut stats = DiffStats::default();
    let (mut i, mut j) = (m, n);

    while i > 0 || j > 0 {
        let cell = matrix[idx(i, j)];
        match cell.from {
            Direction::Diag => {
                let w1 = words1[i - 1];
                let w2 = words2[j - 1];
                if w1 == w2 {
                    out1.push(w1.white().to_string());
                    out2.push(w2.white().to_string());
                } else {
                    out1.push(w1.yellow().to_string());
                    out2.push(w2.yellow().to_string());
                    stats.substitutions += 1;
                }
                i -= 1;
                j -= 1;
            }
            Direction::Up => {
                let w1 = words1[i - 1];
                out1.push(w1.red().to_string());
                stats.deletions += 1;
                i -= 1;
            }
            Direction::Left => {
                let w2 = words2[j - 1];
                out2.push(w2.green().to_string());
                stats.insertions += 1;
                j -= 1;
            }
        }
    }

    out1.reverse();
    out2.reverse();

    (out1.join(" "), out2.join(" "), stats)
}

pub fn needleman_wunsch_char_align(s1: &str, s2: &str) -> (String, String, DiffStats) {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();

    let m = chars1.len();
    let n = chars2.len();

    let mut matrix = vec![
        Cell {
            score: 0,
            from: Direction::Diag
        };
        (m + 1) * (n + 1)
    ];
    let idx = |i: usize, j: usize| i * (n + 1) + j;

    for i in 1..=m {
        matrix[idx(i, 0)] = Cell {
            score: (i as i32) * GAP_PENALTY,
            from: Direction::Up,
        };
    }
    for j in 1..=n {
        matrix[idx(0, j)] = Cell {
            score: (j as i32) * GAP_PENALTY,
            from: Direction::Left,
        };
    }

    for i in 1..=m {
        for j in 1..=n {
            let match_or_mismatch = if chars1[i - 1] == chars2[j - 1] {
                MATCH_SCORE
            } else {
                MISMATCH_PENALTY
            };
            let diag = matrix[idx(i - 1, j - 1)].score + match_or_mismatch;
            let up = matrix[idx(i - 1, j)].score + GAP_PENALTY;
            let left = matrix[idx(i, j - 1)].score + GAP_PENALTY;

            let (best_score, best_direction) = if diag >= up && diag >= left {
                (diag, Direction::Diag)
            } else if up >= diag && up >= left {
                (up, Direction::Up)
            } else {
                (left, Direction::Left)
            };

            matrix[idx(i, j)] = Cell {
                score: best_score,
                from: best_direction,
            };
        }
    }

    let mut out1 = Vec::with_capacity(m);
    let mut out2 = Vec::with_capacity(n);
    let mut stats = DiffStats::default();
    let (mut i, mut j) = (m, n);

    while i > 0 || j > 0 {
        let cell = matrix[idx(i, j)];
        match cell.from {
            Direction::Diag => {
                let c1 = chars1[i - 1];
                let c2 = chars2[j - 1];
                if c1 == c2 {
                    out1.push(c1.to_string().white().to_string());
                    out2.push(c2.to_string().white().to_string());
                } else {
                    out1.push(c1.to_string().yellow().to_string());
                    out2.push(c2.to_string().yellow().to_string());
                    stats.substitutions += 1;
                }
                i -= 1;
                j -= 1;
            }
            Direction::Up => {
                let c1 = chars1[i - 1];
                out1.push(c1.to_string().red().to_string());
                stats.deletions += 1;
                i -= 1;
            }
            Direction::Left => {
                let c2 = chars2[j - 1];
                out2.push(c2.to_string().green().to_string());
                stats.insertions += 1;
                j -= 1;
            }
        }
    }

    out1.reverse();
    out2.reverse();

    (out1.join(""), out2.join(""), stats)
}
