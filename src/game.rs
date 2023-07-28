use crate::ws::Play;
use crate::{Client, Clients, Db, GAME_SIZE};
use serde::{Deserialize, Serialize};
use warp::ws::Message;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum Colour {
    Red,
    Black,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Square {
    value: Colour,
    direction: Direction,
}

type Squares = Vec<Vec<Option<Square>>>;

#[derive(Serialize, Deserialize, Debug)]
struct Game {
    squares: Squares,
    winner: Option<Colour>,
    current_player: Option<Colour>,
    your_colour: Colour,
}

pub async fn play_piece(
    client: &mut Client,
    clients: &Clients,
    sockets: &Sockets,
    play: Option<Play>,
    db: &Db,
) {
    if let Some(sender) = &client.sender {
        let mut game = db.write().await.get_game(client.user_id).await;
        let mut squares: Squares = serde_json::from_value(game.squares).unwrap();
        let winner = calculate_winner(&squares);
        let current_player = calculate_current_player(&squares, winner);
        if let Some(play) = play {
            place_piece(current_player, play, &mut squares);
            game.squares = serde_json::to_value(&squares).unwrap();
            db.write().await.save_game(game.clone()).await;
        }

        let your_colour;
        let other_colour;
        let other_player_id;
        if game.player_red_id == Some(client.user_id) {
            your_colour = Colour::Red;
            other_colour = Colour::Black;
            other_player_id = game.player_black_id;
        } else {
            your_colour = Colour::Black;
            other_colour = Colour::Red;
            other_player_id = game.player_red_id;
        };

        //FIXME: This is inefficient, we shouldn't do this calculation
        // twice in a single play
        let winner = calculate_winner(&squares);

        let current_player = calculate_current_player(&squares, winner);
        let game = Game {
            squares: squares.to_vec(),
            current_player,
            winner,
            your_colour,
        };
        let payload = serde_json::to_string(&game).unwrap();
        let _ = sender.send(Ok(Message::text(&payload)));

        /*
        // Also send it to the other player, if there is one.
        if let Some(id) = other_player_id {
            if let Some(client) = clients.write().await.get_mut(&id) {
                if let Some(sender) = &client.sender {

                    let _ = sender.send(Ok(Message::text(&payload)));
                }
            }
        }
         */
    }
}

fn place_piece(current_player: Option<Colour>, play: Play, squares: &mut Squares) {
    if let Some(colour) = current_player {
        let row = &mut squares[play.row];
        let square = match play.direction {
            Direction::Right => row.iter_mut().find(|square| square.is_none()),
            Direction::Left => row.iter_mut().rfind(|square| square.is_none()),
        };
        if let Some(square) = square {
            *square = Some(Square {
                value: colour,
                direction: play.direction,
            })
        }
    }
}

fn calculate_winner(squares: &Squares) -> Option<Colour> {
    let mut winner;
    let mut win_count = 0;

    // Horizontal
    for i in 0..GAME_SIZE {
        winner = None;
        for j in 0..GAME_SIZE {
            let square = &squares[i][j];
            tally_counts(&square, &mut winner, &mut win_count);
            if win_count == 4 {
                return winner;
            }
        }
    }

    // Vertical
    for j in 0..GAME_SIZE {
        winner = None;
        for i in 0..GAME_SIZE {
            let square = &squares[i][j];
            tally_counts(&square, &mut winner, &mut win_count);
            if win_count == 4 {
                return winner;
            }
        }
    }

    // Diagonal going up
    // Diagonal going down
    None
}

fn tally_counts(square: &Option<Square>, winner: &mut Option<Colour>, win_count: &mut i32) {
    if let Some(square) = square {
        match winner {
            None => {
                *winner = Some(square.value);
                *win_count = 1;
            }
            Some(winner_colour) => {
                if *winner_colour == square.value {
                    *win_count += 1;
                } else {
                    *winner = Some(square.value);
                    *win_count = 1;
                }
            }
        }
    }
}

fn calculate_current_player(squares: &Squares, winner: Option<Colour>) -> Option<Colour> {
    if winner.is_some() {
        return None;
    }

    let mut red_sum = 0;
    let mut black_sum = 0;

    squares
        .iter()
        .flatten()
        .flatten()
        .for_each(|square| match square.value {
            Colour::Red => red_sum += 1,
            Colour::Black => black_sum += 1,
        });

    if red_sum == black_sum && red_sum + black_sum == GAME_SIZE * GAME_SIZE {
        // Board is full, nobody's turn
        None
    } else if red_sum > black_sum {
        Some(Colour::Black)
    } else {
        Some(Colour::Red)
    }
}
