use std::fmt::Display;

pub struct AnalyzeResult {
    decompression_time: u128,
}

impl Display for AnalyzeResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
Decompression time (ms): {}\n
		",
            self.decompression_time
        )
    }
}

pub fn analyze(decompression_time: u128) -> AnalyzeResult {
    AnalyzeResult { decompression_time }
}
