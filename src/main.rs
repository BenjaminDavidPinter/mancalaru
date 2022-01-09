use rand::Rng;

#[derive(Debug)]
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
        let mut new_board: MancalaBoard = MancalaBoard { wells: Default::default() };
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

    pub fn get_best_move(&self, player: Player) -> (usize, f32) {
        match player {
            Player::One => {
                let mut best_move: usize = 0;
                let mut best_score: f32 = f32::MIN;
                for i in 0..6 {
                    if self.wells[i].stones == 0 {
                        continue;
                    }
                    let mut test_board = self.clone();
                    let go_again = test_board.move_well(i, &player);
                    let mut score = test_board.grade_board(&player);
                    if go_again {
                        score += 5.0;
                    }
                    if score > best_score {
                        best_score = score;
                        best_move = i;
                    }
                }
                (best_move, best_score)
            }
            Player::Two => {
                let mut best_move: usize = 0;
                let mut best_score: f32 = f32::MIN;
                for i in 7..13 {
                    if self.wells[i].stones == 0 {
                        continue;
                    }
                    let mut test_board = self.clone();
                    let go_again = test_board.move_well(i, &player);
                    let mut score = test_board.grade_board(&player);
                    if go_again {
                        score += 5.0;
                    }
                    if score > best_score {
                        best_score = score;
                        best_move = i;
                    }
                }
                (best_move, best_score)
            }
        }
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
                Player::One => if current_well == 13 { current_well = 0; continue; }
                Player::Two => if current_well == 6 { current_well = 7; continue; } 
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

            if (current_well != 13 && current_well != 6 ) && self.wells[current_well].stones == 1
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
        (self.wells[0].stones
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
                + self.wells[13].stones)
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
    let mut scores = vec!((0,0); 0);
    let mut absolute_total_moves = 0;
    for _i in 0..1000 {
       let mut board = MancalaBoard::new();
       let mut total_moves = 0;

        while !board.game_over() {
            let mut best_move = board.get_best_move(Player::One);

            total_moves += 1;
            while board.move_well(best_move.0, &Player::One) {
                total_moves += 1;
                best_move = board.get_best_move(Player::One)
            }

            total_moves += 1;
            while board.move_well(rand::thread_rng().gen_range(7..13), &Player::Two) { total_moves += 1;}
        }
        scores.push(board.get_score());
        absolute_total_moves += total_moves;
    }

    let mut p1_wins = 0;
    let mut p2_wins = 0;
    for score in &scores {
        if score.0 > score.1 {
            p1_wins += 1;
        } else {
            p2_wins += 1;
        }
    }

    println!("P1 Wins: {}", p1_wins);
    println!("P2 Wins: {}", p2_wins);

    println!("Average Moves: {}", absolute_total_moves/scores.len());
}

#[test]
pub fn test_go_again_p1(){
    let mut test_board = MancalaBoard::new();
    let go_again = test_board.move_well(2, &Player::One);
    assert!(go_again);
}

#[test]
pub fn test_go_again_p2() {
    let mut test_board = MancalaBoard::new(); 
    let go_again = test_board.move_well(9, &Player::Two);
    assert!(go_again);
}

#[test]
pub fn test_steal_p1(){
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
