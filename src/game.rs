use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum GameError {
    #[error("the current player has already played this round")]
    AlreadyMoved,
    #[error("the given player id is not involved in this game")]
    InvalidPlayer,
    #[error("the game is already over")]
    GameEnded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::Rock => "rock",
                Self::Paper => "paper",
                Self::Scissors => "scissors",
            }
        )
    }
}

impl Move {
    pub fn wins_against(self, other: Self) -> bool {
        match (self, other) {
            (Self::Rock, Self::Rock) => false,
            (Self::Rock, Self::Paper) => false,
            (Self::Rock, Self::Scissors) => true,
            (Self::Paper, Self::Paper) => false,
            (Self::Paper, Self::Scissors) => false,
            (Self::Paper, Self::Rock) => true,
            (Self::Scissors, Self::Scissors) => false,
            (Self::Scissors, Self::Rock) => false,
            (Self::Scissors, Self::Paper) => true,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct Round(Option<Move>, Option<Move>);

impl Round {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_complete(&self) -> bool {
        self.0.is_some() && self.1.is_some()
    }

    pub fn is_draw(&self) -> bool {
        self.is_complete() && self.0 == self.1
    }

    pub fn is_player_1_win(&self) -> bool {
        self.is_complete() && self.0.unwrap().wins_against(self.1.unwrap())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: usize,
    pub player1: i32,
    pub player2: i32,
    pub rounds: Vec<Round>,
    pub winner: Option<i32>,
}

impl Game {
    pub fn new(id: usize, player1: i32, player2: i32) -> Self {
        Self {
            id,
            player1,
            player2,
            rounds: vec![Default::default()],
            winner: None,
        }
    }

    fn current_round(&mut self) -> &mut Round {
        let len = self.rounds.len();
        &mut self.rounds[len - 1]
    }

    pub fn play1(&mut self, action: Move) -> Result<bool, GameError> {
        let mut round = self.current_round();

        if round.0.is_some() {
            return Err(GameError::AlreadyMoved);
        }
        round.0 = Some(action);

        if !round.is_complete() {
            return Ok(false);
        }

        if self.check_end() {
            Ok(true)
        } else {
            self.rounds.push(Default::default());
            Ok(false)
        }
    }

    pub fn play2(&mut self, action: Move) -> Result<bool, GameError> {
        let mut round = self.current_round();

        if round.1.is_some() {
            return Err(GameError::AlreadyMoved);
        }
        round.1 = Some(action);

        if !round.is_complete() {
            return Ok(false);
        }

        if self.check_end() {
            Ok(true)
        } else {
            self.rounds.push(Default::default());
            Ok(false)
        }
    }

    pub fn play_by_id(&mut self, player_id: i32, action: Move) -> Result<bool, GameError> {
        if self.winner.is_some() {
            Err(GameError::GameEnded)
        } else if self.player1 == player_id {
            self.play1(action)
        } else if self.player2 == player_id {
            self.play2(action)
        } else {
            Err(GameError::InvalidPlayer)
        }
    }

    pub fn check_end(&mut self) -> bool {
        let p1_wins: u8 = self
            .rounds
            .iter()
            .map(|round| round.is_player_1_win() as u8)
            .sum();
        let p2_wins: u8 = self
            .rounds
            .iter()
            .map(|round| (!round.is_player_1_win() && !round.is_draw()) as u8)
            .sum();

        if p1_wins >= 2 {
            self.winner = Some(self.player1);
            true
        } else if p2_wins >= 2 {
            self.winner = Some(self.player2);
            true
        } else {
            false
        }
    }
}
