pub struct Stats {
    correct_answers: i32,
    pub total_questions: i32,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            correct_answers: 0,
            total_questions: 0,
        }
    }

    pub fn record_result(&mut self, correct: bool) {
        self.total_questions += 1;
        if correct {
            self.correct_answers += 1;
        }
    }

    pub fn get_percentage_correct(&self) -> f64{
        (self.total_questions > 0)
            .then(|| (self.correct_answers as f64 / self.total_questions as f64) * 100.0)
            .unwrap_or(0.0)
    }
}
