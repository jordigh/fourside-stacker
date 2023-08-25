use std::collections::HashMap;

use crate::ws::Play;
use crate::{Client, Db, Sockets, GAME_SIZE, WIN_LENGTH};
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
    Win,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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
    your_name: String,
    their_name: String,
}

pub async fn play_piece(
    client: &Client,
    clients: &HashMap<String, Client>,
    sockets: &Sockets,
    play: Option<Play>,
    db: &Db,
) {
    let is_ai_game = client.username == "AI";
    let mut game = if is_ai_game {
        db.write().await.get_ai_game().await
    } else {
        db.write().await.get_game(client.user_id).await
    };
    let mut squares: Squares = serde_json::from_value(game.squares.clone()).unwrap();
    let mut current_player = calculate_current_player(&squares);
    let have_play = play.is_some() && !game.finished;
    if let Some(play) = play {
        place_piece(current_player, play, &mut squares);
        if is_ai_game {
            let ai_play = Play {
                row: 0,
                direction: Direction::Left,
            };
            place_piece(Some(Colour::Black), ai_play, &mut squares)
        }
        game.squares = serde_json::to_value(&squares).unwrap();
    }

    let your_colour;
    let other_colour;
    let that_player_id;
    if game.player_red_id == Some(client.user_id) {
        your_colour = Colour::Red;
        other_colour = Colour::Black;
        that_player_id = game.player_black_id;
    } else {
        your_colour = Colour::Black;
        other_colour = Colour::Red;
        that_player_id = game.player_red_id;
    };
    let your_name = db.read().await.get_player_by_id(client.user_id).await.name;
    let their_name = match that_player_id {
        Some(player_id) => db.read().await.get_player_by_id(player_id).await.name,
        None => String::from(""),
    };

    let winner = calculate_winner(&mut squares);
    if have_play {
        // Select next player if a play was made
        match winner {
            Some(_) => current_player = None,
            None => match current_player {
                Some(Colour::Red) => current_player = Some(Colour::Black),
                Some(Colour::Black) => current_player = Some(Colour::Red),
                None => (),
            },
        }
    }

    game.finished = game.finished || current_player.is_none();
    db.write().await.save_game(game).await;

    let this_payload = serde_json::to_string(&Game {
        squares: squares.to_vec(),
        current_player,
        winner,
        your_colour,
        your_name: your_name.clone(),
        their_name: their_name.clone(),
    })
    .unwrap();
    let that_payload = serde_json::to_string(&Game {
        squares: squares.to_vec(),
        current_player,
        winner,
        your_colour: other_colour,
        your_name: their_name,
        their_name: your_name,
    })
    .unwrap();

    notify_players(Some(client.user_id), this_payload, clients, sockets).await;
    notify_players(that_player_id, that_payload, clients, sockets).await;
}

fn place_piece(current_player: Option<Colour>, play: Play, squares: &mut Squares) {
    if let Some(colour) = current_player {
        let row = &mut squares[play.row];
        let square = match play.direction {
            Direction::Right => row.iter_mut().find(|square| square.is_none()),
            Direction::Left => row.iter_mut().rfind(|square| square.is_none()),
            Direction::Win => None,
        };
        if let Some(square) = square {
            *square = Some(Square {
                value: colour,
                direction: play.direction,
            })
        }
    }
}

async fn notify_players(
    player_id: Option<i32>,
    payload: String,
    clients: &HashMap<String, Client>,
    sockets: &Sockets,
) {
    if let Some(player_id) = player_id {
        if let Some(sockets) = sockets.read().await.get(&player_id) {
            for uuid in sockets {
                if let Some(client) = clients.get(uuid) {
                    if let Some(sender) = &client.sender {
                        println!("Notifying {} of play at {}", client.username, uuid);
                        sender.send(Ok(Message::text(&payload))).unwrap();
                    }
                }
            }
        }
    }
}

fn calculate_winner(squares: &mut Squares) -> Option<Colour> {
    const SEARCH_LIMIT: usize = GAME_SIZE - WIN_LENGTH + 1;

    // horizontal
    for i in 0..GAME_SIZE {
        for j in 0..SEARCH_LIMIT {
            if let Some(winner) = check_win(squares, i, j, 0, 1) {
                return Some(winner);
            }
        }
    }

    // vertical
    for i in 0..SEARCH_LIMIT {
        for j in 0..GAME_SIZE {
            if let Some(winner) = check_win(squares, i, j, 1, 0) {
                return Some(winner);
            }
        }
    }

    // diagonal
    for i in 0..SEARCH_LIMIT {
        for j in 0..SEARCH_LIMIT {
            if let Some(winner) = check_win(squares, i, j, 1, 1) {
                return Some(winner);
            }
        }
    }

    // anti-diagonal
    for i in 0..SEARCH_LIMIT {
        for j in (WIN_LENGTH - 1)..GAME_SIZE {
            if let Some(winner) = check_win(squares, i, j, 1, -1) {
                return Some(winner);
            }
        }
    }

    None
}

fn check_win(squares: &mut Squares, i: usize, j: usize, dx: isize, dy: isize) -> Option<Colour> {
    let colours = (0..WIN_LENGTH as isize).fold(Vec::new(), |mut streak, k| {
        streak
            .push(squares[((i as isize) + (k * dx)) as usize][((j as isize) + (k * dy)) as usize]);
        streak
    });

    if let Some(first) = colours[0] {
        let is_win_streak = colours.iter().all(|square| {
            if let Some(square) = square {
                square.value == first.value
            } else {
                false
            }
        });
        if is_win_streak {
            mark_win(squares, i, j, dx, dy);
            return Some(first.value);
        }
    }

    None
}

fn mark_win(squares: &mut Squares, i: usize, j: usize, dx: isize, dy: isize) {
    for k in 0..WIN_LENGTH as isize {
        let square =
            &mut squares[((i as isize) + (k * dx)) as usize][((j as isize) + (k * dy)) as usize];
        if let Some(square) = square {
            square.direction = Direction::Win
        }
    }
}

fn calculate_current_player(squares: &Squares) -> Option<Colour> {
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

    dbg!(red_sum);
    dbg!(black_sum);
    if red_sum + black_sum == GAME_SIZE * GAME_SIZE {
        // Board is full, nobody's turn
        None
    } else if red_sum > black_sum {
        Some(Colour::Black)
    } else {
        Some(Colour::Red)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const B: Option<Square> = Some(Square {
        value: Colour::Black,
        direction: Direction::Left,
    });
    const R: Option<Square> = Some(Square {
        value: Colour::Red,
        direction: Direction::Left,
    });

    #[test]
    fn test_calculate_current_player() {
        let u: Option<Square> = None;

        let empty: Squares = vec![
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
        ];
        assert_eq!(calculate_current_player(&empty), Some(Colour::Red));

        let draw: Squares = vec![
            vec![R, B, R, B, R, B, R],
            vec![B, R, B, R, B, R, B],
            vec![R, R, B, B, R, R, R],
            vec![B, B, R, B, R, B, B],
            vec![B, R, R, B, B, R, R],
            vec![B, B, B, R, B, R, R],
            vec![R, B, B, B, R, R, R],
        ];
        assert_eq!(calculate_current_player(&draw), None);
    }

    #[test]
    fn test_calculate_win() {
        let u: Option<Square> = None;

        let mut empty: Squares = vec![
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
        ];
        assert_eq!(calculate_winner(&mut empty), None);

        let mut no_win: Squares = vec![
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, R, u, u, u, u, u],
            vec![u, R, R, R, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, B],
            vec![u, u, u, u, B, B, B],
        ];
        assert_eq!(calculate_winner(&mut no_win), None);

        let mut vert: Squares = vec![
            vec![u, u, u, u, u, u, u],
            vec![u, u, B, u, u, u, u],
            vec![u, u, B, u, u, u, u],
            vec![u, u, B, u, u, u, u],
            vec![u, u, B, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, R, R, R],
        ];
        assert_eq!(calculate_winner(&mut vert), Some(Colour::Black));
        assert_eq!(vert[4][2].unwrap().direction, Direction::Win);
        assert_eq!(vert[6][6].unwrap().direction, Direction::Left);

        let mut horz: Squares = vec![
            vec![u, u, u, u, u, u, u],
            vec![u, u, B, u, u, u, u],
            vec![u, u, B, u, u, u, u],
            vec![u, u, B, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, R, R, R, R],
        ];
        assert_eq!(calculate_winner(&mut horz), Some(Colour::Red));
        assert_eq!(horz[6][6].unwrap().direction, Direction::Win);
        assert_eq!(horz[1][2].unwrap().direction, Direction::Left);

        let mut diag1: Squares = vec![
            vec![u, u, u, u, u, u, u],
            vec![u, u, B, u, u, u, u],
            vec![u, u, B, u, u, u, u],
            vec![u, u, B, R, u, u, u],
            vec![u, u, u, u, R, u, u],
            vec![u, u, u, u, u, R, u],
            vec![u, u, u, u, u, u, R],
        ];
        assert_eq!(calculate_winner(&mut diag1), Some(Colour::Red));
        assert_eq!(diag1[3][3].unwrap().direction, Direction::Win);
        assert_eq!(diag1[2][2].unwrap().direction, Direction::Left);

        let mut diag2: Squares = vec![
            vec![u, u, u, u, u, u, B],
            vec![u, u, u, u, u, B, u],
            vec![u, u, u, u, B, u, u],
            vec![u, u, u, B, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, R, R, R],
        ];
        assert_eq!(calculate_winner(&mut diag2), Some(Colour::Black));
        assert_eq!(diag2[2][4].unwrap().direction, Direction::Win);
        assert_eq!(diag2[6][5].unwrap().direction, Direction::Left);

        let mut diag3: Squares = vec![
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, u, u, u, u],
            vec![u, u, u, B, u, u, u],
            vec![u, u, B, u, u, u, u],
            vec![u, B, u, u, u, u, u],
            vec![B, u, u, u, R, R, R],
        ];
        assert_eq!(calculate_winner(&mut diag3), Some(Colour::Black));
        assert_eq!(diag3[6][0].unwrap().direction, Direction::Win);
        assert_eq!(diag3[6][5].unwrap().direction, Direction::Left);
    }
}
