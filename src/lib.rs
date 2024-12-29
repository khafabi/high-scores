#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        self.scores
            .to_vec()
            .into_iter()
            .fold(Vec::new(), |mut acc, score| {
                let pos = acc.binary_search(&score).unwrap_or_else(|e| e);
                acc.insert(pos, score);
                acc
            })
            .into_iter()
            .rev()
            .take(3)
            .collect()
    }
}
