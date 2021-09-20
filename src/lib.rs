use std::{borrow::Borrow, fmt, vec};

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
    fn get_available_moves(&self, position: (usize, usize), game: &Game) -> Vec<(usize, usize)> {
        return match self {
            Piece::King => Default::default(),
            Piece::Queen => Default::default(),
            Piece::Rook => get_straight_movements(
                position,
                game.board[position.0][position.1].as_ref().unwrap().1,
                game,
            ),
            Piece::Knight => Default::default(),
            Piece::Bishop => Default::default(),
            Piece::Pawn => self.get_pawn_movement(position, game),
        };
    }

    fn get_pawn_movement(&self, position: (usize, usize), game: &Game) -> Vec<(usize, usize)> {
        let mut movements: Vec<(usize, usize)> = Vec::default();
        let mut direction: i32 = 1;
        let colour = game.board[position.0][position.1].as_ref().unwrap().1;

        if (colour == Colour::Black) {
            direction = -1;
        }

        let mut y_position = add_i32_usize(position.1, direction);
        if !y_position.is_none() {
            // Passive move
            movements.append(&mut get_specific_movement(
                (position.0, y_position.unwrap()),
                colour,
                game,
                false,
            ));

            // Attack moves
            let x_position = add_i32_usize(position.0, -1);
            if !x_position.is_none() {
                movements.append(&mut get_specific_movement(
                    (x_position.unwrap(), y_position.unwrap()),
                    colour,
                    game,
                    true,
                ));
            }

            let x_position = add_i32_usize(position.0, 1);
            movements.append(&mut get_specific_movement(
                (x_position.unwrap(), y_position.unwrap()),
                colour,
                game,
                true,
            ));
        }

        // Special move - first move two steps forwards
        if (colour == Colour::White && position.1 == 1)
            || (colour == Colour::Black && position.1 == 6)
        {
            y_position = add_i32_usize(position.1, direction * 2);
            if !y_position.is_none() {
                movements.append(&mut get_specific_movement(
                    (position.0, y_position.unwrap()),
                    colour,
                    game,
                    false,
                ));
            }
        }
        println!("MOVEMENTS {:?}", movements);
        movements
    }
}

// Get movements
fn get_straight_movements(
    position: (usize, usize),
    colour: Colour,
    game: &Game,
) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::default();

    // Left
    for _i in (position.0..0).rev() {
        if game.board[_i][position.1].as_ref().is_none() {
            positions.push((_i, position.1));
            continue;
        }

        let piece_colour = game.board[_i][position.1].as_ref().unwrap().1;
        if piece_colour != colour {
            positions.push((_i, position.1));
        }

        break;
    }

    positions
}

fn get_diagonal_movements(
    position: (usize, usize),
    colour: Colour,
    game: &Game,
) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::default();

    positions
}

fn get_specific_movement(
    position: (usize, usize),
    colour: Colour,
    game: &Game,
    only_other_colour: bool,
) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::default();

    // Make sure that the position is within the board
    if (position.0 < 0 || position.0 >= 8 || position.1 < 0 || position.1 >= 8) {
        return positions;
    }

    if (!only_other_colour && game.board[position.0][position.1].as_ref().is_none())
        || (only_other_colour
            && !game.board[position.0][position.1].as_ref().is_none()
            && game.board[position.0][position.1].as_ref().unwrap().1 != colour)
    {
        positions.push(position);
    }

    positions
}

fn add_i32_usize(value: usize, difference: i32) -> Option<usize> {
    if difference >= 0 {
        return value.checked_add(difference as usize);
    } else {
        return value.checked_sub(-difference as usize);
    }
}

static FILES: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    board: [[Option<(Piece, Colour)>; 8]; 8],
    turn: Colour,
    move_made: bool,
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
fn parse_position(_position: String) -> (usize, usize) {
    let file = FILES
        .iter()
        .position(|&s| s == _position.chars().nth(0).unwrap().to_string())
        .unwrap();

    // Offset by one as input is 1-8 whilst array is 0-7
    let rank = (_position.chars().nth(1).unwrap().to_digit(10).unwrap() - 1) as usize;
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

        self.move_made = true;
    }

    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        let mut game = Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            board: Default::default(),
            turn: Colour::White,
            move_made: false,
        };

        // Set default pieces
        game.set_default_pieces(Colour::White, 0, 1);
        game.set_default_pieces(Colour::Black, 7, 6);

        game.print_board();

        game
    }

    fn set_default_pieces(&mut self, colour: Colour, main_row: usize, pawn_row: usize) {
        self.board[0][main_row] = Some((Piece::Rook, colour));
        self.board[1][main_row] = Some((Piece::Knight, colour));
        self.board[2][main_row] = Some((Piece::Bishop, colour));

        self.board[3][main_row] = Some((Piece::Queen, colour));
        self.board[4][main_row] = Some((Piece::King, colour));

        self.board[5][main_row] = Some((Piece::Bishop, colour));
        self.board[6][main_row] = Some((Piece::Knight, colour));
        self.board[7][main_row] = Some((Piece::Rook, colour));

        for _i in 0..8 {
            self.board[_i][pawn_row] = Some((Piece::Pawn, colour));
        }
    }

    //"<file>(a-h)<rank>(0-8)"
    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        if self.move_made {
            return None;
        }

        let from = parse_position(_from);
        let to = parse_position(_to);

        if !check_for_colour(self.board[from.0][from.1].as_ref(), self.turn) {
            return None;
        }

        let piece = self.board[from.0][from.1].as_ref().unwrap().0;
        let available_moves: Vec<(usize, usize)> = piece.get_available_moves(from, self);
        println!("{:?}", available_moves);
        // Check if proposed move is valid
        if !available_moves.contains(&to) {
            return None;
        }

        // Make actual move
        let piece = self.board[from.0][from.1].as_ref().unwrap().to_owned();
        self.board[to.0][to.1] = Some(piece);
        self.board[from.0][from.1] = None;

        // Check if should be promoted
        let mut should_promote: bool = false;

        if piece.0 == Piece::Pawn {
            if (piece.1 == Colour::White && to.1 == 7) || (piece.1 == Colour::Black && to.1 == 0) {
                should_promote = true;
            }
        }

        // Change turn
        if !should_promote {
            self.change_turn();
        }

        self.print_board();

        Some(GameState::InProgress)
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        let piece = parse_piece(&_piece);

        if piece.is_none() {
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

        if promoted {
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
    pub fn get_possible_moves(&self, _position: String) -> Option<Vec<String>> {
        let position = parse_position(_position);
        if !check_for_colour(self.board[position.0][position.1].as_ref(), self.turn) {
            return None;
        }

        let piece = self.board[position.0][position.1].as_ref().unwrap().0;
        let available_moves = piece.get_available_moves(position, self.borrow());
        let mut formatted_moves: Vec<String> = Default::default();

        // Parse moves to printed format
        for (_i, _move) in available_moves.iter().enumerate() {
            formatted_moves.push(format!("{}{}", FILES[_move.0], _move.1 + 1));
        }

        Some(formatted_moves)
    }

    fn print_board(&self) {
        println!("");
        println!(". a b c d e f g h");

        // Convert each piece to a unicode character and print it
        for _y in 0..8 {
            print!("{} ", _y + 1);

            for _x in 0..8 {
                if self.board[_x][_y].is_none() {
                    print!("*");
                } else {
                    let piece = self.board[_x][_y].as_ref().unwrap();
                    match piece.1 {
                        Colour::White => match piece.0 {
                            Piece::King => print!("♔"),
                            Piece::Queen => print!("♕"),
                            Piece::Rook => print!("♖"),
                            Piece::Bishop => print!("♗"),
                            Piece::Knight => print!("♘"),
                            Piece::Pawn => print!("♙"),
                        },
                        Colour::Black => match piece.0 {
                            Piece::King => print!("♚"),
                            Piece::Queen => print!("♛"),
                            Piece::Rook => print!("♜"),
                            Piece::Bishop => print!("♝"),
                            Piece::Knight => print!("♞"),
                            Piece::Pawn => print!("♟"),
                        },
                    }
                }

                print!(" ");
            }

            // Jump to next line
            println!("");
        }
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
        write!(f, "TEST")
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
        println!("{:?}", game.get_possible_moves("a2".to_string()));
        game.make_move("a2".to_string(), "a3".to_string());
        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}
