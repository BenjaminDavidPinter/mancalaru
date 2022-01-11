use rand::Rng;
use std::io;


static MAX_DEPTH: i32 = 10;

#[derive(Debug, Clone)]
pub enum Player {
    One,
    Two,
}

#[derive(Default, Debug, Clone)]
pub struct MancalaBoard {
    wells: [Well; 14],
}

impl MancalaBoard {
    pub fn new() -> MancalaBoard {
        let mut new_board: MancalaBoard = MancalaBoard {
            wells: Default::default(),
        };
        for i in 0..14 {
            if i != 6 && i != 13 {
                new_board.wells[i] = Well::new(4, (i as i32) + 1);
            } else if i == 6 {
                new_board.wells[i] = Well::new(0, (i as i32) + 1);
            } else if i == 13 {
                new_board.wells[i] = Well::new(0, 0);
            }
        }

        new_board
    }

    pub fn get_best_move(&self, player: Player, max_depth: i32) -> (usize, f32) {
        let min_iter: i32;
        let max_iter: i32;
        let other_player: Player;
        match player {
            Player::One => {
                min_iter = 0;
                max_iter = 6;
                other_player = Player::Two;
            }
            Player::Two => {
                min_iter = 7;
                max_iter = 13;
                other_player = Player::One;
            }
        }

        let mut best_move: usize = min_iter as usize;
        let mut best_score: f32 = f32::MIN;

        for i in min_iter..max_iter {
            let i = i as usize;
            if self.wells[i].stones == 0 {
                continue;
            }
            let mut test_board = self.clone();
            let go_again = test_board.move_well(i, &player.clone());
            let mut score = test_board.grade_board(&player);

            if go_again && max_depth < MAX_DEPTH {
                score += test_board.get_best_move(player.clone(), max_depth + 1).1;
            } else if max_depth < MAX_DEPTH {
                score -= test_board.get_best_move(other_player.clone(), max_depth+1).1;
            } 

            if score > best_score {
                best_score = score;
                best_move = i;
            }
        }
        (best_move, best_score)
    }

    pub fn grade_board(&self, player: &Player) -> f32 {
        let current_score = self.get_score();
        let diff_score: i32;

        match player {
            Player::One => {
                diff_score = current_score.0 - current_score.1;
            }
            Player::Two => {
                diff_score = current_score.1 - current_score.0;
            }
        }

        diff_score as f32
    }

    pub fn move_well(&mut self, well: usize, player: &Player) -> bool {
        //If the provided well is empty, just let the player go again
        if self.wells[well].stones == 0 {
            return false;
        }

        //'pick up' the stones to start moving them
        let mut total_stones = self.wells[well].stones;

        //In doing so, empty the current well
        self.wells[well].stones = 0;

        //Start moving through the wells
        let mut current_well = self.wells[well].adjacent_well as usize;

        //Iterate over and over on the board, dropping stones one by one.
        while total_stones > 0 {
            match player {
                Player::One => {
                    if current_well == 13 {
                        current_well = 0;
                        continue;
                    }
                }
                Player::Two => {
                    if current_well == 6 {
                        current_well = 7;
                        continue;
                    }
                }
            }
            self.wells[current_well].stones += 1;
            total_stones -= 1;
            current_well = self.wells[current_well].adjacent_well as usize;
        }

        //When we're done, we technically have a reference here to the NEXT well, so we back that
        //up to represent the well we actually ended on. Unless we're on well 0, then we have to
        //backtrace to the P2 score well.

        if current_well != 0 {
            current_well -= 1;
        } else {
            current_well = 13;
        }

        if (current_well != 13 && current_well != 6)
            && self.wells[current_well].stones == 1
                && self.wells[MancalaBoard::reflective_index(current_well)].stones != 0
                && !MancalaBoard::ended_on_opponents_side(player, current_well)
                {
                    self.move_well_to_score(player, current_well);
                    self.move_well_to_score(player, MancalaBoard::reflective_index(current_well));
                }

        MancalaBoard::go_again(player, current_well) && !self.game_over()
    }

    pub fn move_well_to_score(&mut self, player: &Player, well: usize) {
        match player {
            Player::One => {
                self.wells[6].stones += self.wells[well].stones;
                self.wells[well].stones = 0;
            }
            Player::Two => {
                self.wells[13].stones += self.wells[well].stones;
                self.wells[well].stones = 0;
            }
        }
    }

    fn reflective_index(ind: usize) -> usize {

        (12 - ind) as usize
    }

    fn go_again(player: &Player, well: usize) -> bool {
        match player {
            Player::One => well == 6,
            Player::Two => well == 13,
        }
    }

    fn ended_on_opponents_side(player: &Player, end_well: usize) -> bool {
        match player {
            Player::One => end_well > 6,
            Player::Two => end_well < 6,
        }
    }

    pub fn game_over(&self) -> bool {
        (self.wells[0].stones == 0
         && self.wells[1].stones == 0
         && self.wells[2].stones == 0
         && self.wells[3].stones == 0
         && self.wells[4].stones == 0
         && self.wells[5].stones == 0)
            || (self.wells[7].stones == 0
                && self.wells[8].stones == 0
                && self.wells[9].stones == 0
                && self.wells[10].stones == 0
                && self.wells[11].stones == 0
                && self.wells[12].stones == 0)
    }

    pub fn get_score(&self) -> (i32, i32) {
        (
            self.wells[0].stones
            + self.wells[1].stones
            + self.wells[2].stones
            + self.wells[3].stones
            + self.wells[4].stones
            + self.wells[5].stones
            + self.wells[6].stones,
            self.wells[7].stones
            + self.wells[8].stones
            + self.wells[9].stones
            + self.wells[10].stones
            + self.wells[11].stones
            + self.wells[12].stones
            + self.wells[13].stones,
            )
    }
}

#[derive(Default, Debug, Clone)]
pub struct Well {
    stones: i32,
    adjacent_well: i32,
}

impl Well {
    pub fn new(total_stones: i32, next_well: i32) -> Well {
        Well {
            stones: total_stones,
            adjacent_well: next_well,
        }
    }
}

fn main() {
    let mut board = MancalaBoard::new();

    while !board.game_over() {
        let mut best_move = board.get_best_move(Player::One, 0);
        println!("{:#?}", best_move);
        while board.move_well(best_move.0, &Player::One) {
            best_move = board.get_best_move(Player::One, 0);
            println!("{:#?}", best_move);
        }

        let mut player_move = String::new();
        println!("Enter a move:");
        io::stdin().read_line(&mut player_move).expect("Failed to read number");
        let mut player_move: usize = player_move.trim().parse().expect("That isn't a number");
        while board.move_well(player_move, &Player::Two) {
            println!("Go again:");
            let mut player_move_input = String::new();
            io::stdin().read_line(&mut player_move_input).expect("Failed to read number");
            player_move = player_move_input.trim().parse().expect("That isn't a number");
        }

        println!("{:#?}", board);
    }
    println!("{:#?}", board.get_score());
}

#[test]
pub fn test_go_again_p1() {
    let mut test_board = MancalaBoard::new();
    let go_again = test_board.move_well(2, &Player::One);
    assert!(go_again);

    let mut test_board = MancalaBoard::new();
    let go_again = test_board.move_well(0, &Player::One);
    assert!(!go_again);
}

#[test]
pub fn test_go_again_p2() {
    let mut test_board = MancalaBoard::new();
    let go_again = test_board.move_well(9, &Player::Two);
    assert!(go_again);
}

#[test]
pub fn test_steal_p1() {
    let mut test_board = MancalaBoard::new();
    test_board.wells[5].stones = 0;
    test_board.wells[7].stones = 10;
    let go_again = test_board.move_well(1, &Player::One);
    assert!(!go_again);

    assert!(test_board.wells[5].stones == 0);
    assert!(test_board.wells[7].stones == 0);
    assert!(test_board.wells[6].stones == 11);
}

#[test]
pub fn test_steal_p2() {
    let mut test_board = MancalaBoard::new();
    test_board.wells[12].stones = 0;
    test_board.wells[0].stones = 10;
    let go_again = test_board.move_well(8, &Player::Two);
    assert!(!go_again);

    assert!(test_board.wells[12].stones == 0);
    assert!(test_board.wells[0].stones == 0);
    assert!(test_board.wells[13].stones == 11);
}

#[test]
pub fn test_wrap_around_steal() {
    let mut test_board = MancalaBoard::new();
    test_board.wells[5].stones = 8;
    test_board.wells[0].stones = 0;
    test_board.move_well(5, &Player::One);
    assert!(test_board.wells[0].stones == 0);
    assert!(test_board.wells[12].stones == 0);
    dbg!(test_board.wells[6].stones);
    assert!(test_board.wells[6].stones == 7);
}
