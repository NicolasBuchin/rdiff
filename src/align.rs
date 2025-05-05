use crate::DiffStats;
use colored::*;

#[derive(Clone, Copy)]
enum Direction {
    Diag,
    Up,
    Left,
}

#[derive(Clone, Copy)]
struct Cell {
    score: i32,
    from: Direction,
}

const MATCH_SCORE: i32 = 2;
const MISMATCH_PENALTY: i32 = -1;
const GAP_PENALTY: i32 = -2;

fn wrap_tag(content: &str, tag: char, word_pos: Option<usize>, stats: &mut DiffStats) -> String {
    match tag {
        '+' => {
            stats.insertions += 1;
            format!("+[{}]", content.green())
        }
        '-' => {
            stats.deletions += 1;
            format!("-[{}]", content.red())
        }
        '~' => {
            stats.substitutions += 1;

            if let (Some(pos), Some(ref mut fs)) = (word_pos, &mut stats.field_stats) {
                fs.increment(pos);

                let field = fs.field_name(pos);
                return format!("{}:~[{}]", field.purple(), content.yellow());
            }

            format!("~[{}]", content.yellow())
        }
        _ => unreachable!(),
    }
}

fn split_with_whitespace(s: &str) -> (Vec<&str>, Vec<&str>) {
    let mut words = Vec::new();
    let mut spaces = Vec::new();

    let mut last = 0;
    let mut in_space = false;

    for (i, c) in s.char_indices() {
        if c.is_whitespace() {
            if !in_space {
                words.push(&s[last..i]);
                last = i;
                in_space = true;
            }
        } else if in_space {
            spaces.push(&s[last..i]);
            last = i;
            in_space = false;
        }
    }

    if !in_space {
        words.push(&s[last..]);
        spaces.push("");
    }

    (words, spaces)
}

pub fn word_align(s1: &str, s2: &str, use_tags: bool, stats: &mut DiffStats) -> (String, String) {
    let (w1, sp1) = split_with_whitespace(s1);
    let (w2, sp2) = split_with_whitespace(s2);

    let (mat, m, n, idx) = compute_matrix(&w1, &w2);

    let mut o1 = Vec::new();
    let mut o2 = Vec::new();
    let (mut i, mut j) = (m, n);

    while i > 0 || j > 0 {
        let Cell { from, .. } = mat[idx(i, j)];

        match from {
            Direction::Diag => {
                let a = w1[i - 1];
                let b = w2[j - 1];

                let sa = sp1.get(i - 1).copied().unwrap_or(" ");
                let sb = sp2.get(j - 1).copied().unwrap_or(" ");

                if a == b {
                    o1.push(format!("{}{}", a.white(), sa));
                    o2.push(format!("{}{}", b.white(), sb));
                } else if use_tags {
                    o1.push(format!("{}{}", wrap_tag(a, '~', Some(i - 1), stats), sa));
                    o2.push(format!("{}{}", wrap_tag(b, '~', Some(j - 1), stats), sb));
                } else {
                    o1.push(format!("{}{}", a.yellow(), sa));
                    o2.push(format!("{}{}", b.yellow(), sb));
                    stats.substitutions += 1;
                }

                i -= 1;
                j -= 1;
            }
            Direction::Up => {
                let a = w1[i - 1];
                let sa = sp1.get(i - 1).copied().unwrap_or(" ");

                if use_tags {
                    o1.push(format!("{}{}", wrap_tag(a, '-', None, stats), sa));
                } else {
                    o1.push(format!("{}{}", a.red(), sa));
                    stats.deletions += 1;
                }

                i -= 1;
            }
            Direction::Left => {
                let b = w2[j - 1];
                let sb = sp2.get(j - 1).copied().unwrap_or(" ");

                if use_tags {
                    o2.push(format!("{}{}", wrap_tag(b, '+', None, stats), sb));
                } else {
                    o2.push(format!("{}{}", b.green(), sb));
                    stats.insertions += 1;
                }

                j -= 1;
            }
        }
    }

    o1.reverse();
    o2.reverse();

    (o1.concat(), o2.concat())
}

pub fn char_align(s1: &str, s2: &str, stats: &mut DiffStats) -> (String, String) {
    let c1: Vec<_> = s1.chars().collect();
    let c2: Vec<_> = s2.chars().collect();

    let (mat, m, n, idx) = compute_matrix(&c1, &c2);

    let mut o1 = Vec::new();
    let mut o2 = Vec::new();
    let (mut i, mut j) = (m, n);

    while i > 0 || j > 0 {
        let Cell { from, .. } = mat[idx(i, j)];

        match from {
            Direction::Diag => {
                let x = c1[i - 1];
                let y = c2[j - 1];

                if x == y {
                    o1.push(x.to_string().white().to_string());
                    o2.push(y.to_string().white().to_string());
                } else {
                    o1.push(x.to_string().yellow().to_string());
                    o2.push(y.to_string().yellow().to_string());
                    stats.substitutions += 1;
                }

                i -= 1;
                j -= 1;
            }
            Direction::Up => {
                let x = c1[i - 1];

                o1.push(x.to_string().red().to_string());
                stats.deletions += 1;

                i -= 1;
            }
            Direction::Left => {
                let y = c2[j - 1];

                o2.push(y.to_string().green().to_string());
                stats.insertions += 1;

                j -= 1;
            }
        }
    }

    o1.reverse();
    o2.reverse();

    (o1.concat(), o2.concat())
}

fn compute_matrix<T: PartialEq>(
    a: &[T],
    b: &[T],
) -> (Vec<Cell>, usize, usize, impl Fn(usize, usize) -> usize) {
    let m = a.len();
    let n = b.len();

    let idx = move |i, j| i * (n + 1) + j;

    let mut mat = vec![
        Cell {
            score: 0,
            from: Direction::Diag
        };
        (m + 1) * (n + 1)
    ];

    for i in 1..=m {
        mat[idx(i, 0)] = Cell {
            score: (i as i32) * GAP_PENALTY,
            from: Direction::Up,
        };
    }
    for j in 1..=n {
        mat[idx(0, j)] = Cell {
            score: (j as i32) * GAP_PENALTY,
            from: Direction::Left,
        };
    }

    for i in 1..=m {
        for j in 1..=n {
            let matching = if a[i - 1] == b[j - 1] {
                MATCH_SCORE
            } else {
                MISMATCH_PENALTY
            };

            let d = mat[idx(i - 1, j - 1)].score + matching;
            let u = mat[idx(i - 1, j)].score + GAP_PENALTY;
            let l = mat[idx(i, j - 1)].score + GAP_PENALTY;

            let (score, from) = if l >= u && l >= d {
                (l, Direction::Left)
            } else if u >= l && u >= d {
                (u, Direction::Up)
            } else {
                (d, Direction::Diag)
            };

            mat[idx(i, j)] = Cell { score, from };
        }
    }

    (mat, m, n, idx)
}
