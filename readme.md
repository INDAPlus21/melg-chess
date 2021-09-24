Documentation chess engine:

| **Function** | **Description** |
|--------------|-----------------|
| `pub fn new() -> Game` | Initialises a new board with pieces. |
| `pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState>` | If the current game state is `InProgress` and the move is legal, moves a piece and return the resulting state of the game. |
| `pub fn set_promotion(&mut self, _piece: String) -> ()` | Sets the piece type that a peasant becames following a promotion. |
| `pub fn get_game_state(&self) -> GameState` | Gets the current game state. |
| `pub fn get_possible_moves(&mut self, _position: String) -> Optional<Vec<String>>` | If a piece is standing on the given tile, returns all possible new positions of that piece. |