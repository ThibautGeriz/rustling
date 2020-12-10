use std::cmp;

pub struct BowlingGame {
    frames: Vec<Frame>,
}

impl BowlingGame {
    pub fn new() -> BowlingGame {
        BowlingGame { frames: vec![] }
    }
    pub fn roll(&mut self, pin_fell: u8) {
        match self.frames.last_mut() {
            Some(f) if !f.is_complete() => f.set_second_roll(pin_fell),
            _ => self.frames.push(Frame::new(pin_fell)),
        }
    }

    pub fn get_score_by_frame(&self) -> Vec<Option<u8>> {
        self.frames[..cmp::min(10, self.frames.len())]
            .iter()
            .enumerate()
            .map(|(index, frame)| frame.get_score(self.get_two_rolls_after(index)))
            .collect()
    }

    pub fn get_total_score(&self) -> u16 {
        self.get_score_by_frame()
            .iter()
            .filter(|f| f.is_some())
            .map(|f| f.unwrap() as u16)
            .sum()
    }

    fn get_two_rolls_after(&self, frame_index: usize) -> (Option<u8>, Option<u8>) {
        let next_frame = self.frames.get(frame_index + 1);
        let first_roll = next_frame.map(|f| f.first_roll);
        let second_roll = next_frame
            .map(|f| f.second_roll)
            .flatten()
            .or_else(|| self.frames.get(frame_index + 2).map(|f| f.first_roll));
        (first_roll, second_roll)
    }
}

impl Default for BowlingGame {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Frame {
    first_roll: u8,
    second_roll: Option<u8>,
}

impl Frame {
    fn new(first_roll: u8) -> Frame {
        Frame {
            first_roll,
            second_roll: None,
        }
    }
    fn set_second_roll(&mut self, second_roll: u8) {
        self.second_roll = Some(second_roll)
    }

    fn is_complete(&self) -> bool {
        self.second_roll.is_some() || self.is_strike()
    }

    fn get_score(&self, next_two_rolls: (Option<u8>, Option<u8>)) -> Option<u8> {
        if self.is_strike() {
            return match next_two_rolls {
                (Some(roll1), Some(roll2)) => Some(10 + roll1 + roll2),
                _ => None,
            };
        }
        if self.is_spare() {
            return match next_two_rolls {
                (Some(roll1), _) => Some(10 + roll1),
                _ => None,
            };
        }
        Some(self.first_roll + self.second_roll.unwrap_or(0))
    }

    fn is_strike(&self) -> bool {
        self.first_roll == 10
    }

    fn is_spare(&self) -> bool {
        !self.is_strike() && self.first_roll + self.second_roll.unwrap_or(0) == 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_no_rolls() {
        // given
        let game = BowlingGame::new();

        // when
        let score = game.get_score_by_frame();

        // then
        assert_eq!(score, vec![]);
        assert_eq!(game.get_total_score(), 0);
    }

    #[test]
    fn with_one_roll() {
        // given
        let mut game = BowlingGame::new();
        game.roll(7);

        // when
        let score = game.get_score_by_frame();

        // then
        assert_eq!(score, vec![Some(7)]);
        assert_eq!(game.get_total_score(), 7);
    }

    #[test]
    fn with_two_normal_rolls() {
        // given
        let mut game = BowlingGame::new();
        game.roll(7);
        game.roll(2);

        // when
        let score = game.get_score_by_frame();

        // then
        assert_eq!(score, vec![Some(9)]);
        assert_eq!(game.get_total_score(), 9);
    }

    #[test]
    fn with_three_normal_rolls() {
        // given
        let mut game = BowlingGame::new();
        game.roll(3);
        game.roll(2);
        game.roll(1);

        // when
        let score = game.get_score_by_frame();

        // then
        assert_eq!(score, vec![Some(5), Some(1)]);
        assert_eq!(game.get_total_score(), 6);
    }

    #[test]
    fn with_five_normal_rolls() {
        // given
        let mut game = BowlingGame::new();
        game.roll(3);
        game.roll(2);
        game.roll(1);
        game.roll(8);
        game.roll(8);

        // when
        let score = game.get_score_by_frame();

        // then
        assert_eq!(score, vec![Some(5), Some(9), Some(8)]);
        assert_eq!(game.get_total_score(), 22);
    }

    #[test]
    fn with_one_strike_roll() {
        // given
        let mut game = BowlingGame::new();
        game.roll(10);

        // when
        let score = game.get_score_by_frame();

        // then
        assert_eq!(score, vec![None]);
        assert_eq!(game.get_total_score(), 0);
    }

    #[test]
    fn with_one_strike_with_one_normal_roll() {
        // given
        let mut game = BowlingGame::new();
        game.roll(10);
        game.roll(3);

        // when
        let score = game.get_score_by_frame();

        // then
        assert_eq!(score, vec![None, Some(3)]);
        assert_eq!(game.get_total_score(), 3);
    }

    #[test]
    fn with_one_strike_with_two_normal_rolls() {
        // given
        let mut game = BowlingGame::new();
        game.roll(10);
        game.roll(3);
        game.roll(4);

        // when
        let score = game.get_score_by_frame();

        // then
        assert_eq!(score, vec![Some(17), Some(7)]);
        assert_eq!(game.get_total_score(), 24);
    }

    #[test]
    fn with_three_strikes() {
        // given
        let mut game = BowlingGame::new();
        game.roll(10);
        game.roll(10);
        game.roll(10);

        // when
        let score = game.get_score_by_frame();

        // then
        assert_eq!(score, vec![Some(30), None, None]);
        assert_eq!(game.get_total_score(), 30);
    }

    #[test]
    fn with_a_spare() {
        // given
        let mut game = BowlingGame::new();
        game.roll(3);
        game.roll(7);

        // when
        let score = game.get_score_by_frame();

        // then
        assert_eq!(score, vec![None]);
        assert_eq!(game.get_total_score(), 0);
    }

    #[test]
    fn with_a_spare_and_one_normal_roll() {
        // given
        let mut game = BowlingGame::new();
        game.roll(3);
        game.roll(7);
        game.roll(7);

        // when
        let score = game.get_score_by_frame();

        // then
        assert_eq!(score, vec![Some(17), Some(7)]);
        assert_eq!(game.get_total_score(), 24);
    }

    #[test]
    fn with_10_strike() {
        // given
        let mut game = BowlingGame::new();
        game.roll(10);
        game.roll(10);
        game.roll(10);
        game.roll(10);
        game.roll(10);
        game.roll(10);
        game.roll(10);
        game.roll(10);
        game.roll(10);
        game.roll(10);
        game.roll(10);
        game.roll(10);

        // when
        let score = game.get_score_by_frame();

        // then
        assert_eq!(
            score,
            vec![
                Some(30),
                Some(30),
                Some(30),
                Some(30),
                Some(30),
                Some(30),
                Some(30),
                Some(30),
                Some(30),
                Some(30)
            ]
        );
        assert_eq!(game.get_total_score(), 300);
    }
}
