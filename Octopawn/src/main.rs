use std::fmt;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
struct State {
    board: String,
    side: char,
}

fn negamax(state: &State, tt: &mut HashMap<String, i32>) -> i32 {
    let state_string = format!("{}{}", state.board, state.side);

    if let Some(&value) = tt.get(&state_string) {
        return value;
    }

    // Check if the current state is a win for the opponent
    if is_opponent_win(state) {
        tt.insert(state_string.clone(), -1);
        return -1;
    }

    let legal_moves = generate_legal_moves(state);

    // If no legal moves, it's a win for the opponent
    if legal_moves.is_empty() {
        tt.insert(state_string.clone(), -1);
        return -1;
    }

    let mut v = None;
    for &(old_pos, new_pos) in &legal_moves {
        let new_board = make_move(&state, &old_pos, &new_pos);
        let new_state = State {
            board: new_board,
            side: opponent(state.side),
        };

        let v_prime = -negamax(&new_state, tt);

        if v.is_none() || v_prime > v.unwrap() {
            v = Some(v_prime);
        }
    }

    tt.insert(state_string, v.unwrap_or(0));
    v.unwrap_or(0)
}

fn is_opponent_win(state: &State) -> bool {
    let opponent_pawn = if state.side == 'W' { 'p' } else { 'P' };

    // Find the index of the opponent's pawn
    if let Some(index) = state.board.chars().position(|c| c == opponent_pawn) {
        let row = index / 4;
        let col = index % 4;

        // Check if the opponent's pawn has reached the winning position
        if state.side == 'W' && col == 3 {
            return true;
        } else if state.side == 'B' && col == 0 {
            return true;
        }
    }

    false
}

fn generate_legal_moves(state: &State) -> Vec<(usize,usize)> {
    let mut legal_moves:Vec<(usize,usize)> = Vec::new();
    let mut state_char: char = 'p';
    let mut opponent_char: char = 'P';
    if state.side == 'B' {
        state_char = 'p';
        opponent_char = 'P';
    } else {
        state_char = 'P';
        opponent_char = 'p';
    }

    for (index, c) in state.board.chars().enumerate() {
        if c == state_char {
            if state.side == 'W' {
                if index > 3 {
                    if state.board.chars().nth(index-4) == Some('.') {
                        // Check free spot ahead
                        legal_moves.push((index, index - 4));
                    } else if state.board.chars().nth(index-3) == Some(opponent_char) {
                        // Check diagonal
                        legal_moves.push((index, index - 3));
                    } else if index > 4 && state.board.chars().nth(index-5) == Some(opponent_char) {
                        // Check diagonal
                        legal_moves.push((index, index - 5));
                    }
                }

            } else {
                if index < 12 {
                    if state.board.chars().nth(index+4) == Some('.') {
                        // check ahead
                        legal_moves.push((index, index + 4));

                    } else if state.board.chars().nth(index+3) == Some(opponent_char) {
                        // check diagonal
                        legal_moves.push((index, index + 3));

                    } else if index < 11 && state.board.chars().nth(index+5) == Some(opponent_char) {
                        // check diagonal
                        legal_moves.push((index, index + 5));
                    }
                }

            }
        }
    }
    legal_moves
}

fn make_move(state: &State, old_pos: &usize, new_pos: &usize) -> String {
    let mut new_board = state.board.chars().collect::<Vec<char>>();

    // Update the board to reflect the move
    new_board[*new_pos] = new_board[*old_pos];
    new_board[*old_pos] = '.';

    new_board.iter().collect()
}

fn opponent(side: char) -> char {
    if side == 'W' {
        'B'
    } else {
        'W'
    }
}

fn main() {
    let mut tt: HashMap<String, i32> = HashMap::new();
    let initial_state = State {
        board: String::from("pppp........PPPP"),
        side: 'W',
    };

    negamax(&initial_state, &mut tt);

    // Write the result to a JSON file
    println!("{:?}",tt);
    let mut file = File::create("4pawn.json").expect("Unable to create file");
    let json_result = serde_json::to_string_pretty(&tt).expect("JSON serialization failed");
    file.write_all(json_result.as_bytes()).expect("Unable to write to file");
}