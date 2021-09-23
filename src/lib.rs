use std::{cmp::min, vec};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    Checkmate,
    GameOver,
}

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
    fn get_available_moves(
        &self,
        position: (usize, usize),
        check_for_check: bool,
        game: &mut Game,
    ) -> Vec<(usize, usize)> {
        let available_moves: Vec<(usize, usize)> = match self {
            Piece::King => self.get_king_movement(position, game),
            Piece::Queen => {
                let mut movements: Vec<(usize, usize)> =
                    self.get_straight_movements(position, game);
                let mut diagonal_movements: Vec<(usize, usize)> =
                    self.get_diagonal_movements(position, game);
                movements.append(&mut diagonal_movements);
                movements
            }
            Piece::Rook => self.get_straight_movements(position, game),
            Piece::Knight => self.get_knight_movement(position, game),
            Piece::Bishop => self.get_diagonal_movements(position, game),
            Piece::Pawn => self.get_pawn_movement(position, game),
        };

        let mut valid_moves: Vec<(usize, usize)> = Default::default();

        if check_for_check {
            let mut check_printed = false;

            for _move in available_moves.iter() {
                // Test move
                let piece = game.board[position.0][position.1]
                    .as_ref()
                    .unwrap()
                    .to_owned();
                let target_piece = game.board[_move.0][_move.1];

                game.board[_move.0][_move.1] = Some(piece);
                game.board[position.0][position.1] = None;

                // Check for check
                let self_checked = check_for_checked(piece.1, game);

                let checked_opponent = check_for_checked(
                    if piece.1 == Colour::White {
                        Colour::Black
                    } else {
                        Colour::White
                    },
                    game,
                );

                // Revert move
                game.board[position.0][position.1] = Some(piece);

                if target_piece.is_none() {
                    game.board[_move.0][_move.1] = None;
                } else {
                    game.board[_move.0][_move.1] = Some(target_piece.unwrap());
                }

                // Only add valid moves
                if self_checked && !checked_opponent {
                    if !check_printed {
                        println!("MOVING PIECE WOULD CAUSE CHECK: {:?}", position);
                        check_printed = true;
                    }

                    continue;
                }

                valid_moves.push(_move.to_owned());
            }

            return valid_moves;
        }

        available_moves
    }

    fn get_king_movement(&self, position: (usize, usize), game: &mut Game) -> Vec<(usize, usize)> {
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

    fn get_knight_movement(
        &self,
        position: (usize, usize),
        game: &mut Game,
    ) -> Vec<(usize, usize)> {
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
        game: &mut Game,
    ) -> Vec<(usize, usize)> {
        let mut movements: Vec<(usize, usize)> = Default::default();

        // Check each position if its valid
        for _offset in offsets.iter() {
            let x_position = add_i32_usize(position.0, _offset.0);
            let y_position = add_i32_usize(position.1, _offset.1);
            if !x_position.is_none() && !y_position.is_none() {
                // Passive move
                movements.append(&mut self.get_specific_movement(
                    (x_position.unwrap(), y_position.unwrap()),
                    colour,
                    game,
                    MovementMode::Both,
                ));
            }
        }

        movements
    }

    fn get_pawn_movement(&self, position: (usize, usize), game: &mut Game) -> Vec<(usize, usize)> {
        let mut movements: Vec<(usize, usize)> = Vec::default();
        let mut direction: i32 = 1;
        let colour = game.board[position.0][position.1].as_ref().unwrap().1;

        if colour == Colour::Black {
            direction = -1;
        }

        let mut y_position = add_i32_usize(position.1, direction);
        if !y_position.is_none() {
            // Passive move
            let mut forward_move: Vec<(usize, usize)> = self.get_specific_movement(
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
                movements.append(&mut self.get_specific_movement(
                    (x_position.unwrap(), y_position.unwrap()),
                    colour,
                    game,
                    MovementMode::OnlyDifferent,
                ));
            }

            let x_position = add_i32_usize(position.0, 1);
            movements.append(&mut self.get_specific_movement(
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
                        movements.append(&mut self.get_specific_movement(
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

    fn get_straight_movements(
        &self,
        position: (usize, usize),
        game: &mut Game,
    ) -> Vec<(usize, usize)> {
        let mut positions: Vec<(usize, usize)> = Vec::default();
        let colour = game.board[position.0][position.1].as_ref().unwrap().1;

        // Left
        let distance_to_edge = position.0 + 1;
        for _i in 1..distance_to_edge {
            let x_position = position.0 - _i;

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
        let distance_to_edge = position.1 + 1;
        for _i in 1..distance_to_edge {
            let y_position = position.1 - _i;

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

    fn get_diagonal_movements(
        &self,
        position: (usize, usize),
        game: &mut Game,
    ) -> Vec<(usize, usize)> {
        let mut positions: Vec<(usize, usize)> = Vec::default();
        let colour = game.board[position.0][position.1].as_ref().unwrap().1;

        // Top left
        let min_distance_to_edge = min(position.0, 7 - position.1);
        positions.append(&mut self.get_moves_in_direction(
            min_distance_to_edge,
            position,
            true,
            false,
            colour,
            game,
        ));

        // Top right
        let min_distance_to_edge = min(7 - position.0, 7 - position.1);
        positions.append(&mut self.get_moves_in_direction(
            min_distance_to_edge,
            position,
            false,
            false,
            colour,
            game,
        ));

        // Bottom left
        let min_distance_to_edge = min(position.0, position.1);
        positions.append(&mut self.get_moves_in_direction(
            min_distance_to_edge,
            position,
            true,
            true,
            colour,
            game,
        ));

        // Bottom right
        let min_distance_to_edge = min(7 - position.0, position.1);
        positions.append(&mut self.get_moves_in_direction(
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
        &self,
        min_distance_to_edge: usize,
        position: (usize, usize),
        x_negative: bool,
        y_negative: bool,
        colour: Colour,
        game: &mut Game,
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

    fn get_specific_movement(
        &self,
        position: (usize, usize),
        colour: Colour,
        game: &mut Game,
        movement_mode: MovementMode,
    ) -> Vec<(usize, usize)> {
        let mut positions: Vec<(usize, usize)> = Vec::default();

        // Make sure that the position is within the board
        if position.0 >= 8 || position.1 >= 8 {
            return positions;
        }

        if ((movement_mode == MovementMode::OnlyEmpty || movement_mode == MovementMode::Both)
            && game.board[position.0][position.1].as_ref().is_none())
            || ((movement_mode == MovementMode::OnlyDifferent
                || movement_mode == MovementMode::Both)
                && !game.board[position.0][position.1].as_ref().is_none()
                && game.board[position.0][position.1].as_ref().unwrap().1 != colour)
        {
            positions.push(position);
        }

        positions
    }
}

fn check_for_checked(colour_to_be_checked: Colour, game: &mut Game) -> bool {
    // Loop through board and see if any opponent piece has a move that takes the king. Checked colour is the colour to check if they can check the opponent
    for _x in 0..8 {
        for _y in 0..8 {
            if game.board[_x][_y].as_ref().is_none()
                || game.board[_x][_y].as_ref().unwrap().1 == colour_to_be_checked
            {
                continue;
            }

            // Get all moves for the piece
            let piece = game.board[_x][_y].as_ref().unwrap().to_owned();
            let piece_moves = piece.0.get_available_moves((_x, _y), false, game);

            // Check if move conquers the oponents king
            for _move in piece_moves.iter() {
                let target_piece = game.board[_move.0][_move.1].as_ref();
                if target_piece.is_some()
                    && target_piece.unwrap().1 == colour_to_be_checked
                    && target_piece.unwrap().0 == Piece::King
                {
                    return true;
                }
            }
        }
    }

    false
}

fn check_for_checkmate(colour_to_be_checked: Colour, game: &mut Game) -> bool {
    // Loop through board and see if a colour has no available moves
    for _x in 0..8 {
        for _y in 0..8 {
            if game.board[_x][_y].as_ref().is_none()
                || game.board[_x][_y].as_ref().unwrap().1 != colour_to_be_checked
            {
                continue;
            }

            // Get all moves for the piece
            let piece = game.board[_x][_y].as_ref().unwrap().to_owned();
            let piece_moves = piece.0.get_available_moves((_x, _y), true, game);

            // A single possible moves means that the colour is not in checkmat
            if piece_moves.len() > 0 {
                return false;
            }
        }
    }

    true
}

#[derive(Copy, Clone, PartialEq)]
pub enum MovementMode {
    OnlyEmpty,
    OnlyDifferent,
    Both,
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

        println!(
            "TURN: {}",
            if self.turn == Colour::White {
                "White"
            } else {
                "Black"
            }
        );

        self.move_made = false;
    }

    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        let mut game = Game {
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

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        if self.state == GameState::Checkmate {
            self.state = GameState::GameOver;
        }

        if self.move_made {
            return None;
        }

        let from = parse_position(_from);
        let to = parse_position(_to);

        if !check_for_colour(self.board[from.0][from.1].as_ref(), self.turn) {
            return None;
        }

        let piece = self.board[from.0][from.1].as_ref().unwrap().0;
        let available_moves: Vec<(usize, usize)> = piece.get_available_moves(from, true, self);

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

        // Update game state and check for check(mate)
        // All valid moves either doesn't cause check, check only the opponent of check both so a self check test is not required
        let checked = check_for_checked(
            if piece.1 == Colour::White {
                Colour::Black
            } else {
                Colour::White
            },
            self,
        );

        if checked {
            let checkmated = check_for_checkmate(
                if piece.1 == Colour::White {
                    Colour::Black
                } else {
                    Colour::White
                },
                self,
            );

            if checkmated {
                self.state = GameState::Checkmate;
            } else {
                self.state = GameState::Check;
            }
        } else {
            self.state = GameState::InProgress;
        }

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

        Some(self.state)
    }

    /// Set the piece type that a peasant becames following a promotion.
    /// Possible values: "Queen", "Bishop", "Knight", "Rook"
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
    /// new positions of that piece.
    /// Changed &self to &mut self
    pub fn get_possible_moves(&mut self, _position: String) -> Option<Vec<String>> {
        let position = parse_position(_position);
        if !check_for_colour(self.board[position.0][position.1].as_ref(), self.turn) {
            return None;
        }

        let piece = self.board[position.0][position.1].as_ref().unwrap().0;
        let available_moves = piece.get_available_moves(position, true, self);
        let mut formatted_moves: Vec<String> = Default::default();

        // Parse moves to printed format
        for _move in available_moves.iter() {
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

// --------------------------
// ######### TESTS ##########
// --------------------------

// cargo test -- --nocapture --test-threads=1
#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // Check a full game of chess
    #[test]
    fn test_whole_chess_game() {
        let mut game = Game::new();

        test_move("a2", "a3", &mut game);
        test_move("c7", "c5", &mut game);

        test_move("e2", "e3", &mut game);
        test_move("d8", "a5", &mut game);

        test_move("h2", "h3", &mut game);
        test_move("h7", "h5", &mut game);

        test_move("e3", "e4", &mut game);
        test_move("h8", "h6", &mut game);

        test_move("a1", "a2", &mut game);
        test_move("h6", "f6", &mut game);

        test_invalid_move("d2", "d3", &mut game);

        test_move("c2", "c3", &mut game);
        test_move("g8", "h6", &mut game);

        test_move("d2", "d4", &mut game);
        test_move("c5", "c4", &mut game);

        test_move("e4", "e5", &mut game);
        test_move("a5", "e5", &mut game);

        assert_eq!(game.get_game_state(), GameState::Check);

        test_invalid_move("e1", "e2", &mut game);

        assert_eq!(game.get_game_state(), GameState::Check);

        test_move("e1", "d2", &mut game);

        assert_eq!(game.get_game_state(), GameState::InProgress);

        test_move("e5", "e2", &mut game);

        test_move("d2", "e2", &mut game);
        test_move("d7", "d5", &mut game);

        test_move("d1", "d3", &mut game);
        test_move("h5", "h4", &mut game);

        test_move("d3", "e3", &mut game);
        test_move("a7", "a6", &mut game);

        test_move("e3", "e5", &mut game);
        test_move("a6", "a5", &mut game);

        test_move("e5", "d5", &mut game);
        test_move("a5", "a4", &mut game);

        test_move("d5", "a5", &mut game);
        test_move("g7", "g6", &mut game);

        test_move("d4", "d5", &mut game);
        test_move("h6", "f5", &mut game);

        test_move("d5", "d6", &mut game);
        test_move("f8", "h6", &mut game);

        test_move("d6", "d7", &mut game);
        test_move("e8", "f8", &mut game);

        test_move("d7", "d8", &mut game);
        game.set_promotion("Queen".to_string());
        test_move("f8", "g7", &mut game);

        test_move("b2", "b3", &mut game);
        test_move("g6", "g5", &mut game);

        test_move("h1", "h2", &mut game);
        test_move("g5", "g4", &mut game);

        test_move("h2", "h1", &mut game);
        test_move("a8", "a6", &mut game);

        test_move("h1", "h2", &mut game);
        test_move("a6", "d6", &mut game);

        test_move("a2", "b2", &mut game);
        test_move("h6", "c1", &mut game);

        test_move("h2", "h1", &mut game);
        test_move("f6", "e6", &mut game);

        test_move("a5", "e5", &mut game);
        test_move("e6", "e5", &mut game);

        test_move("d8", "f8", &mut game);
        test_move("g7", "f8", &mut game);

        assert_eq!(game.get_game_state(), GameState::Checkmate);

        test_invalid_move("e2", "e3", &mut game);
        assert_eq!(game.get_game_state(), GameState::GameOver);
    }

    fn test_move(_from: &str, _to: &str, game: &mut Game) {
        // Test if move is valid
        assert_eq!(
            game.make_move(_from.to_string(), _to.to_string()).is_none(),
            false
        );
    }

    fn test_invalid_move(_from: &str, _to: &str, game: &mut Game) {
        // Test if move is invalid
        assert_eq!(
            game.make_move(_from.to_string(), _to.to_string()).is_none(),
            true
        );
    }
}
