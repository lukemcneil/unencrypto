mod traits;

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error, fmt};

pub(crate) type Result<T> = std::result::Result<T, Error>;
pub(crate) type Player = String;
pub(crate) type GameId = String;
pub(crate) type Clues = [String; 3];

#[derive(Serialize, Debug)]
pub(crate) enum Error {
    GameConflict,
    GameNotFound,
    RoundNotInCollectingCluesState,
    RoundNotInCollectingOwnTeamGuessesState,
    RoundNotInCollectingOtherTeamGuessesState,
    InvalidGuess,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GameConflict => write!(f, "game conflict"),
            Self::GameNotFound => write!(f, "game not found"),
            Self::RoundNotInCollectingCluesState => {
                write!(f, "round not in collecting clues state")
            }
            Self::RoundNotInCollectingOwnTeamGuessesState => {
                write!(f, "round not in collecting own team guesses state")
            }
            Self::RoundNotInCollectingOtherTeamGuessesState => {
                write!(f, "round not in collecting other team guesses state")
            }
            Self::InvalidGuess => write!(f, "invalid guess"),
        }
    }
}

impl error::Error for Error {}

#[derive(Deserialize, Serialize)]
pub(crate) struct BadRequest {
    error: String,
    message: String,
}

impl BadRequest {
    fn new(error: Error) -> Self {
        Self {
            error: format!("{error:?}"),
            message: format!("{error}"),
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub(crate) struct CluesData {
    /// The clues
    pub(crate) clues: Clues,
    /// The player
    pub(crate) player: PlayerData,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub(crate) struct GuessData {
    /// The guessed code
    pub(crate) guess: Code,
    /// The player
    pub(crate) player: PlayerData,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub(crate) enum Team {
    Red,
    Blue,
}

impl Team {
    fn other_team(&self) -> Self {
        match self {
            Team::Red => Team::Blue,
            Team::Blue => Team::Red,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub(crate) enum Role {
    Encryptor,
    Decryptor,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub(crate) struct PlayerData {
    /// The player
    pub(crate) player: Player,
    /// The team the player belongs to
    pub(crate) team: Team,
    /// The team the player belongs to
    pub(crate) role: Role,
}

impl PlayerData {
    pub(crate) fn new(player: Player, team: Team, role: Role) -> Self {
        Self { player, team, role }
    }
}

#[derive(PartialEq)]
pub(crate) enum RoundState {
    CollectingClues,
    CollectingOwnTeamGuesses,
    CollectingOtherTeamGuesses,
    Complete,
}

#[derive(Clone, Deserialize, Serialize, Default, PartialEq, Eq, Hash)]
pub(crate) struct Code([u8; 3]);

#[derive(Clone, Deserialize, Serialize, Default)]
pub(crate) struct OneTeamRound {
    /// The code for this round
    pub(crate) code: Code,
    /// The clues for this round
    pub(crate) clues: Option<Clues>,
    /// The guessed code from this team
    pub(crate) own_team_guess: Option<Code>,
    /// The guessed code from the other team
    pub(crate) other_team_guess: Option<Code>,
}

impl OneTeamRound {
    fn new() -> Self {
        Self {
            code: Code::random(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Round {
    /// The round info for the red team
    pub(crate) red_round: OneTeamRound,
    /// The round info for the blue team
    pub(crate) blue_round: OneTeamRound,
}

impl Code {
    fn random() -> Self {
        let mut x = [1, 2, 3, 4];
        x.shuffle(&mut rand::thread_rng());
        Self([x[0], x[1], x[2]])
    }

    fn is_valid(&self) -> bool {
        self.0.iter().all(|&x| (1..=4).contains(&x))
            && self.0[0] != self.0[1]
            && self.0[1] != self.0[2]
            && self.0[2] != self.0[0]
    }
}

impl Round {
    fn new() -> Self {
        Round {
            red_round: OneTeamRound::new(),
            blue_round: OneTeamRound::new(),
        }
    }

    fn state(&self) -> RoundState {
        if self.red_round.clues.is_none() || self.blue_round.clues.is_none() {
            RoundState::CollectingClues
        } else if self.red_round.own_team_guess.is_none()
            || self.blue_round.own_team_guess.is_none()
        {
            RoundState::CollectingOwnTeamGuesses
        } else if self.red_round.other_team_guess.is_none()
            || self.blue_round.other_team_guess.is_none()
        {
            RoundState::CollectingOtherTeamGuesses
        } else {
            RoundState::Complete
        }
    }

    fn get_team_round_mut(&mut self, team: Team) -> &mut OneTeamRound {
        match team {
            Team::Red => &mut self.red_round,
            Team::Blue => &mut self.blue_round,
        }
    }
}

#[derive(Clone, Default, Deserialize, Serialize)]
pub(crate) struct Game {
    /// The list of players in the game
    pub(crate) players: HashMap<Player, PlayerData>,
    /// The list of rounds in the game with the most recent round being the last item in the list
    pub(crate) rounds: Vec<Round>,
}

#[derive(Clone, Default, Deserialize, Serialize, PartialEq, Eq, Debug)]
pub(crate) struct Score {
    pub(crate) red_interceptions: u8,
    pub(crate) blue_interceptions: u8,
    pub(crate) red_miscommunications: u8,
    pub(crate) blue_miscommunications: u8,
}

impl Game {
    pub(crate) fn add_player(&mut self, player: Player, team: Team, role: Role) {
        self.players
            .insert(player.clone(), PlayerData::new(player, team, role));
    }

    pub(crate) fn remove_player(&mut self, player: Player) -> Result<()> {
        self.players.remove(&player);
        Ok(())
    }

    pub(crate) fn give_clues(&mut self, clues: Clues, team: Team) -> Result<()> {
        // Confirm we are collecting clues for the current round
        let state = self.current_round_state();
        if state != RoundState::CollectingClues {
            return Err(Error::RoundNotInCollectingCluesState);
        }
        // Add or replace the clues
        let round = self.current_round_mut();
        let team_round = round.get_team_round_mut(team);
        team_round.clues = Some(clues);
        Ok(())
    }

    pub(crate) fn guess_own_team(&mut self, guess: Code, team: Team) -> Result<()> {
        // Confirm we are collecting guesses for the current round
        if self.current_round_state() != RoundState::CollectingOwnTeamGuesses {
            return Err(Error::RoundNotInCollectingOwnTeamGuessesState);
        }
        // Confirm the guess is a valid code
        if !guess.is_valid() {
            return Err(Error::InvalidGuess);
        }
        // Add or replace the guess
        let round = self.current_round_mut();
        let team_round = round.get_team_round_mut(team);
        team_round.own_team_guess = Some(guess);
        Ok(())
    }

    pub(crate) fn guess_other_team(&mut self, guess: Code, team: Team) -> Result<()> {
        // Confirm we are collecting guesses for the current round
        if self.current_round_state() != RoundState::CollectingOtherTeamGuesses {
            return Err(Error::RoundNotInCollectingOtherTeamGuessesState);
        }
        // Confirm the guess is a valid code
        if !guess.is_valid() {
            return Err(Error::InvalidGuess);
        }
        // Add or replace the guess
        let round = self.current_round_mut();
        let team_round = round.get_team_round_mut(team.other_team());
        team_round.other_team_guess = Some(guess);
        Ok(())
    }

    pub(crate) fn add_round_if_complete(&mut self) {
        if self.current_round_state() == RoundState::Complete {
            self.add_round();
        }
    }

    fn add_round(&mut self) {
        self.rounds.push(Round::new());
    }

    pub(crate) fn current_round(&self) -> &Round {
        let index = self.rounds.len() - 1;
        &self.rounds[index]
    }

    fn current_round_mut(&mut self) -> &mut Round {
        let index = self.rounds.len() - 1;
        &mut self.rounds[index]
    }

    fn current_round_state(&self) -> RoundState {
        let round = self.current_round();
        round.state()
    }

    pub fn get_score(&self) -> Score {
        let mut scores = Score::default();
        for round in &self.rounds {
            if round.state() != RoundState::Complete {
                continue;
            }
            if round.red_round.code != round.red_round.own_team_guess.clone().unwrap() {
                scores.red_miscommunications += 1;
            }
            if round.red_round.code == round.red_round.other_team_guess.clone().unwrap() {
                scores.blue_interceptions += 1;
            }
            if round.blue_round.code != round.blue_round.own_team_guess.clone().unwrap() {
                scores.blue_miscommunications += 1;
            }
            if round.blue_round.code == round.blue_round.other_team_guess.clone().unwrap() {
                scores.red_interceptions += 1;
            }
        }
        scores
    }
}

#[derive(Default)]
pub(crate) struct Games(HashMap<String, Game>);

impl Games {
    #[allow(clippy::map_entry)]
    pub(crate) fn create(
        &mut self,
        game_id: String,
        initial_player: Player,
        team: Team,
        role: Role,
    ) -> Result<()> {
        if self.0.contains_key(&game_id) {
            Err(Error::GameConflict)
        } else {
            let mut game = Game::default();
            game.add_round();
            game.add_player(initial_player, team, role);
            self.0.insert(game_id, game);
            Ok(())
        }
    }

    pub(crate) fn get(&mut self, game_id: &str) -> Result<&mut Game> {
        self.0.get_mut(game_id).ok_or(Error::GameNotFound)
    }

    pub(crate) fn delete(&mut self, game_id: &str) {
        self.0.remove(game_id);
    }
}
