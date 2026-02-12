use crate::player::PlayerTrait;

pub struct SimulatedPlayer {
    the_number: u32,
}
impl SimulatedPlayer {
    pub fn new(number: u32) -> SimulatedPlayer {
        SimulatedPlayer {
            the_number: number
        }
    }
}
impl PlayerTrait for SimulatedPlayer {

    fn ask_if_equal(&mut self, guess: u32) -> bool {
        self.the_number == guess
    }

    fn ask_to_compare(&mut self, guess: u32) -> i32 {
        if self.the_number == guess {
            0
        } else if self.the_number < guess {
            -1
        } else {
            1
        }
    }

}




#[cfg(test)]
mod part1_tests {
    use crate::part1::Part1;
    use crate::part3::SimulatedPlayer;
    use crate::player::Player;
    use crate::strategies::Strategy;

    #[test]
    fn the_min() {
        let min = 0;
        let max = 100;
        let number = min;

        // We create a simulated player
        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = Part1::guess_the_number(&mut player, min, max);
        assert_eq!(answer, number);
        assert_eq!(player.steps(), 1);
    }

    #[test]
    fn the_max() {
        let min = 0;
        let max = 100;
        let number = max - 1;

        // We create a simulated player
        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = Part1::guess_the_number(&mut player, min, max);
        assert_eq!(answer, number);
        assert!(player.steps() <= max);
    }

    #[test]
    fn a_different_number() {
        let min = 0;
        let max = 100;
        let number = 50;

        // We create a simulated player
        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = Part1::guess_the_number(&mut player, min, max);
        assert_eq!(answer, number);
        assert!(player.steps() <= max);
    }
}




#[cfg(test)]
mod bad_strategy_tests {
    use crate::part3::SimulatedPlayer;
    use crate::player::Player;
    use crate::strategies::{BadStrategy, Strategy};

    #[test]
    fn the_min() {
        let min = 0;
        let max = 100;
        let number = min;

        // We create a simulated player
        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = BadStrategy::guess_the_number(&mut player, min, max);
        assert_eq!(answer, number);
    }

    #[test]
    fn the_max() {
        let min = 0;
        let max = 100;
        let number = max - 1;

        // We create a simulated player
        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = BadStrategy::guess_the_number(&mut player, min, max);
        assert_eq!(answer, number);
    }

    #[test]
    fn a_different_number() {
        let min = 0;
        let max = 100;
        let number = todo!("`the_min` and `the_max` are not enough: the `BadStrategy` satisfies them, even though it is wrong. Add your own test that demonstrate that BadStrategy does not work!");

        // We create a simulated player
        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = BadStrategy::guess_the_number(&mut player, min, max);
        assert_eq!(answer, number);
    }
}

#[cfg(test)]
mod part2_tests {
    use crate::part2::Part2;
    use crate::part3::SimulatedPlayer;
    use crate::player::Player;
    use crate::strategies::Strategy;

    #[test]
    fn the_min() {
        let min = 0;
        let max = 100;
        let number = min;

        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = Part2::guess_the_number(&mut player, min, max);

        assert_eq!(answer, number);
        assert!(player.steps() <= 10); // binary search max ~log2(100) = 7 steps, so 10 is safe
    }

    #[test]
    fn the_max() {
        let min = 0;
        let max = 100;
        let number = max - 1;

        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = Part2::guess_the_number(&mut player, min, max);

        assert_eq!(answer, number);
        assert!(player.steps() <= 10);
    }

    #[test]
    fn a_different_number() {
        let min = 0;
        let max = 100;
        let number = 50;

        let mut player = Player::new(SimulatedPlayer::new(number));
        let answer = Part2::guess_the_number(&mut player, min, max);

        assert_eq!(answer, number);
        assert!(player.steps() <= 10);
    }
}
