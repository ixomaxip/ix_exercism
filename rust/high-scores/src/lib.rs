use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
    top_k: usize
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores {scores: scores, top_k: 3}
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
        let mut result: Vec<u32> = self.scores().iter()
            .fold(BinaryHeap::new(), |mut heap, &x| {
                heap.push(Reverse(x));
                if heap.len() > self.top_k {
                    heap.pop();
                }
                heap
            }).iter().map(|rev| rev.0).collect();
        result.sort_by_key(|x| Reverse(*x));
        result
    }
}
