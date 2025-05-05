use colored::*;

pub trait Format {
    fn name(&self) -> &'static str;
    fn fields(&self) -> &'static [&'static str];
}

pub struct SamFormat;

impl Format for SamFormat {
    fn name(&self) -> &'static str {
        "SAM"
    }
    fn fields(&self) -> &'static [&'static str] {
        &[
            "qname", "flag", "rname", "pos", "mapq", "cigar", "rnext", "pnext", "tlen", "seq",
            "qual",
        ]
    }
}

pub struct PafFormat;

impl Format for PafFormat {
    fn name(&self) -> &'static str {
        "PAF"
    }
    fn fields(&self) -> &'static [&'static str] {
        &[
            "qname", "qlen", "qstart", "qend", "strand", "tname", "tlen", "tstart", "tend",
            "matches", "blocklen", "mapq",
        ]
    }
}

pub trait FieldStats {
    fn increment(&mut self, idx: usize);
    fn field_name(&self, idx: usize) -> &'static str;
    fn report(&self);
}

pub struct GenericStats {
    fmt: Box<dyn Format>,
    counts: Vec<usize>,
}

impl GenericStats {
    pub fn new(fmt: Box<dyn Format>) -> Self {
        let len = fmt.fields().len();
        GenericStats {
            fmt,
            counts: vec![0; len],
        }
    }
}

impl FieldStats for GenericStats {
    fn increment(&mut self, idx: usize) {
        if idx >= self.counts.len() {
            self.counts.resize(idx + 1, 0);
        }
        self.counts[idx] += 1;
    }

    fn field_name(&self, idx: usize) -> &'static str {
        if idx < self.fmt.fields().len() {
            self.fmt.fields()[idx]
        } else {
            "???"
        }
    }

    fn report(&self) {
        println!("\n{} field substitutions:", self.fmt.name());
        for (i, &cnt) in self.counts.iter().enumerate() {
            let name = self.field_name(i);
            println!("  {}: {}", name.purple(), cnt);
        }
    }
}
