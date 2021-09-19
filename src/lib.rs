use std::{borrow::Borrow, fmt};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver,
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */
#[derive(Copy, Clone, PartialEq)]
pub enum Colour {
    White,
    Black,
}

// Rook = Torn
// Knight = Häst
// Bishop = Löpare
// Pawn = Bonde
#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

impl Piece {
    fn get_available_moves(&self, game: Game) {
        match self {
            Piece::King => {}
            Piece::Queen => {}
            Piece::Rook => {}
            Piece::Knight => {}
            Piece::Bishop => {}
            Piece::Pawn => {}
        }
    }
}

static FILES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    board: [[Option<(Piece, Colour)>; 8]; 8],
    turn: Colour,
}

// Check if piece is correct colour for turn and not empty
fn check_for_colour(position: Option<&(Piece, Colour)>, turn: Colour) -> bool {
    if position.as_ref().is_none() {
        return false;
    }

    if position.as_ref().unwrap().1 != turn {
        return false;
    }

    return true;
}

// Parse data
fn parse_position(_position: String) -> (u32, u32) {
    let file = FILES
        .iter()
        .position(|&s| s == _position.chars().nth(0).unwrap().to_string())
        .unwrap() as u32;

    // Offset by one as input is 1-8 whilst array is 0-7
    let rank = _position.chars().nth(1).unwrap().to_digit(10).unwrap() - 1;
    return (file, rank);
}

fn parse_piece(input: &str) -> Option<Piece> {
    match input {
        "Queen" => return Some(Piece::Queen),
        "Bishop" => return Some(Piece::Bishop),
        "Knight" => return Some(Piece::Knight),
        "Rook" => return Some(Piece::Rook),
        _ => return None,
    }
}

impl Game {
    fn change_turn(&mut self) {
        if self.turn == Colour::White {
            self.turn = Colour::Black;
        } else {
            self.turn = Colour::White;
        }
    }

    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            board: Default::default(),
            turn: Colour::White,
        }
    }

    //"<file>(a-h)<rank>(0-8)"
    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        let from = parse_position(_from);
        let to = parse_position(_to);

        if (!check_for_colour(
            self.board[from.0 as usize][from.1 as usize].as_ref(),
            self.turn,
        )) {
            return None;
        }

        // Check if should be promoted
        let should_promote: bool = false;

        // Change turn
        if (!should_promote) {
            self.change_turn();
        }

        Some(GameState::InProgress)
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        let piece = parse_piece(&_piece);

        if (piece.is_none()) {
            return;
        }

        // Find pawn to promote
        let promoted: bool = match self.turn {
            Colour::White => {
                // Check top row
                self.promote_piece(piece, 7, Colour::White)
            }
            Colour::Black => {
                // Check bottom row
                self.promote_piece(piece, 0, Colour::Black)
            }
        };

        if (promoted) {
            self.change_turn();
        }
    }

    fn promote_piece(&mut self, piece: Option<Piece>, row: usize, colour: Colour) -> bool {
        // Check top row (white) / bottom row (black)
        for _i in 0..8 {
            let checked_piece = self.board[_i][row].as_ref();
            if !checked_piece.is_none()
                && checked_piece.unwrap().0 == Piece::Pawn
                && checked_piece.unwrap().1 == colour
            {
                self.board[_i][row] = Some((piece.unwrap(), colour));
                return true;
            }
        }

        return false;
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _postion: String) -> Option<Vec<String>> {
        None
    }
}

/// Implement print routine for Game.
///
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */

        write!(f, "")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

// cargo test -- --nocapture --test-threads=1
#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let mut game = Game::new();
        //parse_position("f3".to_string());
        game.make_move("f3".to_string(), "h3".to_string());
        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}
