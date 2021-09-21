use std::{borrow::Borrow, cmp::min, fmt, vec};

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
            Piece::King => self.get_king_movement(position, game),
            Piece::Queen => {
                let mut movements: Vec<(usize, usize)> = get_straight_movements(position, game);
                let mut diagonal_movements: Vec<(usize, usize)> =
                    get_diagonal_movements(position, game);
                movements.append(&mut diagonal_movements);
                movements
            }
            Piece::Rook => get_straight_movements(position, game),
            Piece::Knight => self.get_knight_movement(position, game),
            Piece::Bishop => get_diagonal_movements(position, game),
            Piece::Pawn => self.get_pawn_movement(position, game),
        };
    }

    fn get_king_movement(&self, position: (usize, usize), game: &Game) -> Vec<(usize, usize)> {
        let mut movements: Vec<(usize, usize)> = Vec::default();
        let colour = game.board[position.0][position.1].as_ref().unwrap().1;

        // Circle around the king
        let offsets = vec![
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ];

        movements.append(&mut self.get_movements_from_array(position, offsets, colour, game));

        movements
    }

    fn get_knight_movement(&self, position: (usize, usize), game: &Game) -> Vec<(usize, usize)> {
        let mut movements: Vec<(usize, usize)> = Vec::default();
        let colour = game.board[position.0][position.1].as_ref().unwrap().1;

        // Circle around the king
        let offsets = vec![
            (-2, 1),
            (-1, 2),
            (1, 2),
            (2, 1),
            (2, -1),
            (1, -2),
            (-1, -2),
            (-2, -1),
        ];

        movements.append(&mut self.get_movements_from_array(position, offsets, colour, game));

        movements
    }

    fn get_movements_from_array(
        &self,
        position: (usize, usize),
        offsets: Vec<(i32, i32)>,
        colour: Colour,
        game: &Game,
    ) -> Vec<(usize, usize)> {
        let mut movements: Vec<(usize, usize)> = Default::default();

        // Check each position if its valid
        for _offset in offsets.iter() {
            let x_position = add_i32_usize(position.0, _offset.0);
            let y_position = add_i32_usize(position.1, _offset.1);
            if !x_position.is_none() && !y_position.is_none() {
                // Passive move
                movements.append(&mut get_specific_movement(
                    (x_position.unwrap(), y_position.unwrap()),
                    colour,
                    game,
                    MovementMode::Both,
                ));
            }
        }

        movements
    }

    fn get_pawn_movement(&self, position: (usize, usize), game: &Game) -> Vec<(usize, usize)> {
        let mut movements: Vec<(usize, usize)> = Vec::default();
        let mut direction: i32 = 1;
        let colour = game.board[position.0][position.1].as_ref().unwrap().1;

        if colour == Colour::Black {
            direction = -1;
        }

        let mut y_position = add_i32_usize(position.1, direction);
        if !y_position.is_none() {
            // Passive move
            let mut forward_move: Vec<(usize, usize)> = get_specific_movement(
                (position.0, y_position.unwrap()),
                colour,
                game,
                MovementMode::OnlyEmpty,
            );
            let forward_move_valid = !forward_move.is_empty();
            movements.append(&mut forward_move);

            // Attack moves
            let x_position = add_i32_usize(position.0, -1);
            if !x_position.is_none() {
                movements.append(&mut get_specific_movement(
                    (x_position.unwrap(), y_position.unwrap()),
                    colour,
                    game,
                    MovementMode::OnlyDifferent,
                ));
            }

            let x_position = add_i32_usize(position.0, 1);
            movements.append(&mut get_specific_movement(
                (x_position.unwrap(), y_position.unwrap()),
                colour,
                game,
                MovementMode::OnlyDifferent,
            ));

            // Special move - first move two steps forwards
            if forward_move_valid {
                if (colour == Colour::White && position.1 == 1)
                    || (colour == Colour::Black && position.1 == 6)
                {
                    y_position = add_i32_usize(position.1, direction * 2);
                    if !y_position.is_none() {
                        movements.append(&mut get_specific_movement(
                            (position.0, y_position.unwrap()),
                            colour,
                            game,
                            MovementMode::OnlyEmpty,
                        ));
                    }
                }
            }
        }

        movements
    }
}

// Get movements
fn get_straight_movements(position: (usize, usize), game: &Game) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::default();
    let colour = game.board[position.0][position.1].as_ref().unwrap().1;

    // Left
    let distance_to_edge = position.0;
    for _i in 1..distance_to_edge {
        let x_position = position.0 - _i;
        println!("LEFT {}{}", x_position, position.1);
        if game.board[x_position][position.1].as_ref().is_none() {
            positions.push((x_position, position.1));
            continue;
        }

        let piece_colour = game.board[x_position][position.1].as_ref().unwrap().1;
        if piece_colour != colour {
            positions.push((x_position, position.1));
        }

        break;
    }

    // Right
    for _i in (position.0 + 1)..8 {
        println!("RIGHT {}{}", _i, position.1);
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

    // Down
    let distance_to_edge = position.1;
    for _i in 1..distance_to_edge {
        let y_position = position.1 - _i;
        println!("DOWN {}{}", position.0, y_position);
        if game.board[position.0][y_position].as_ref().is_none() {
            positions.push((position.0, y_position));
            continue;
        }

        let piece_colour = game.board[position.0][y_position].as_ref().unwrap().1;
        if piece_colour != colour {
            positions.push((position.0, y_position));
        }

        break;
    }

    // Up
    for _i in (position.1 + 1)..8 {
        println!("UP {}{}", position.0, _i);
        if game.board[position.0][_i].as_ref().is_none() {
            positions.push((position.0, _i));
            continue;
        }

        let piece_colour = game.board[position.0][_i].as_ref().unwrap().1;
        if piece_colour != colour {
            positions.push((position.0, _i));
        }

        break;
    }

    positions
}

fn get_diagonal_movements(position: (usize, usize), game: &Game) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::default();
    let colour = game.board[position.0][position.1].as_ref().unwrap().1;

    // Top left
    let min_distance_to_edge = min(position.0, 7 - position.1);
    positions.append(&mut get_moves_in_direction(
        min_distance_to_edge,
        position,
        true,
        false,
        colour,
        game,
    ));

    // Top right
    let min_distance_to_edge = min(7 - position.0, 7 - position.1);
    positions.append(&mut get_moves_in_direction(
        min_distance_to_edge,
        position,
        false,
        false,
        colour,
        game,
    ));

    // Bottom left
    let min_distance_to_edge = min(position.0, position.1);
    positions.append(&mut get_moves_in_direction(
        min_distance_to_edge,
        position,
        true,
        true,
        colour,
        game,
    ));

    // Bottom right
    let min_distance_to_edge = min(7 - position.0, position.1);
    positions.append(&mut get_moves_in_direction(
        min_distance_to_edge,
        position,
        false,
        true,
        colour,
        game,
    ));

    positions
}

fn get_moves_in_direction(
    min_distance_to_edge: usize,
    position: (usize, usize),
    x_negative: bool,
    y_negative: bool,
    colour: Colour,
    game: &Game,
) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Default::default();

    // Loop until reaching position at the edge of the board
    for _offset in 1..(min_distance_to_edge + 1) {
        let x_position: usize;
        let y_position: usize;

        if x_negative {
            x_position = position.0 - _offset;
        } else {
            x_position = position.0 + _offset;
        }

        if y_negative {
            y_position = position.1 - _offset;
        } else {
            y_position = position.1 + _offset;
        }

        if game.board[x_position][y_position].as_ref().is_none() {
            positions.push((x_position, y_position));
            continue;
        }

        let piece_colour = game.board[x_position][y_position].as_ref().unwrap().1;
        if piece_colour != colour {
            positions.push((x_position, y_position));
        }

        break;
    }

    positions
}

#[derive(Copy, Clone, PartialEq)]
pub enum MovementMode {
    OnlyEmpty,
    OnlyDifferent,
    Both,
}

fn get_specific_movement(
    position: (usize, usize),
    colour: Colour,
    game: &Game,
    movement_mode: MovementMode,
) -> Vec<(usize, usize)> {
    let mut positions: Vec<(usize, usize)> = Vec::default();

    // Make sure that the position is within the board
    if position.0 >= 8 || position.1 >= 8 {
        return positions;
    }

    if ((movement_mode == MovementMode::OnlyEmpty || movement_mode == MovementMode::Both)
        && game.board[position.0][position.1].as_ref().is_none())
        || ((movement_mode == MovementMode::OnlyDifferent || movement_mode == MovementMode::Both)
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
        println!("TURN: {}", (self.turn == Colour::White));
        self.move_made = false;
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

        game.print_board(None);

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

    //"<file>(a-h)<rank>(1-8)"
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
        println!("AVAILABLE MOVES: {:?}", available_moves);
        println!("PROPOSED MOVE: {:?} -> {:?}", from, to);
        // Check if proposed move is valid
        if !available_moves.contains(&to) {
            return None;
        }

        self.print_board(Some(available_moves));

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

        self.print_board(None);

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

    fn print_board(&self, available_moves: Option<Vec<(usize, usize)>>) {
        println!("");
        println!(". a b c d e f g h");

        // Convert each piece to a unicode character and print it
        for _y in 0..8 {
            print!("{} ", _y + 1);

            for _x in 0..8 {
                if !available_moves.is_none()
                    && available_moves.as_ref().unwrap().contains(&(_x, _y))
                {
                    if self.board[_x][_y].is_none() {
                        // Movement
                        print!("x");
                    } else {
                        // Attack
                        print!("X");
                    }
                } else if self.board[_x][_y].is_none() {
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

        test_move("a2", "a3", &mut game);
        test_move("c7", "c5", &mut game);

        test_move("d2", "d3", &mut game);
        test_move("d8", "a5", &mut game);

        test_move("c1", "e3", &mut game);
        test_move("h7", "h5", &mut game);

        test_move("e3", "c5", &mut game);
        test_move("h8", "h6", &mut game);

        test_move("b1", "d2", &mut game);
        test_move("h6", "f6", &mut game);

        test_move("c5", "d6", &mut game);
        test_move("f6", "d6", &mut game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    fn test_move(_from: &str, _to: &str, game: &mut Game) {
        // Test if move is valid
        assert_eq!(
            game.make_move(_from.to_string(), _to.to_string()).is_none(),
            false
        );
    }
}
