use std::fmt::Display;

#[derive(Clone)]
pub struct DisplayableVec(pub Vec<u8>);

impl DisplayableVec {
    pub fn new(value: &[u8]) -> Self {
        Self(value.to_vec())
    }
}

impl Display for DisplayableVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bytes in self.0.chunks(4) {
            let s: Vec<String> = bytes.iter().map(|&b| b.to_string()).collect();
            let s = s.join(" ");

            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}
