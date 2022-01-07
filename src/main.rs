use rand::Rng;

pub enum Player {
    One,
    Two,
}

#[derive(Default, Debug)]
pub struct MancalaBoard {
    wells: [Well; 14],
}

impl MancalaBoard {
    pub fn new() -> MancalaBoard {
        let mut new_board: MancalaBoard = Default::default();
        new_board.wells = Default::default();

        for i in 0..14 {
            if i != 6 && i != 13 {
                new_board.wells[i] = Well::new(4, (i as i32) + 1);
            } else if i == 6 {
                new_board.wells[i] = Well::new(0, (i as i32) + 1);
            } else if i == 13 {
                new_board.wells[i] = Well::new(0, 0);
            }
        }

        return new_board;
    }

    pub fn move_well(&mut self, well: usize, player: Player) -> bool {
        dbg!(well);

        //If the provided well is empty, just let the player go again
        if self.wells[well].stones == 0 {
            return true;
        }

        //'pick up' the stones to start moving them
        let mut total_stones = self.wells[well].stones;

        dbg!(total_stones);

        //In doing so, empty the current well
        self.wells[well].stones = 0;

        //Start moving through the wells
        let mut current_well = self.wells[well].adjacent_well as usize;
        dbg!(current_well);

        //Iterate over and over on the board, dropping stones one by one.
        while total_stones > 0 {
            self.wells[current_well].stones = self.wells[current_well].stones + 1;
            dbg!(self.wells[current_well].stones);
            total_stones = total_stones - 1;
            dbg!(total_stones);
            current_well = self.wells[current_well].adjacent_well as usize;
            dbg!(current_well);
        }

        //When we're done, we technically have a reference here to the NEXT well, so we back that
        //up to represent the well we actually ended on. Unless we're on well 0, then we have to
        //backtrace to the P2 score well.

        if current_well != 0 {
            current_well = current_well - 1;
        } else {
            current_well = 13;
        }
        dbg!(current_well);

        if current_well != 13 && current_well != 6 {
            if self.wells[current_well].stones == 1
                && self.wells[MancalaBoard::reflective_index(current_well)].stones != 0
                && MancalaBoard::ended_on_opponents_side(&player, current_well)
            {
                self.move_well_to_score(&player, current_well);
                self.move_well_to_score(&player, MancalaBoard::reflective_index(current_well));
            }
        }

        return MancalaBoard::go_again(player, current_well) && !self.game_over();
    }

    pub fn move_well_to_score(&mut self, player: &Player, well: usize) {
        match player {
            Player::One => {
                self.wells[6].stones = self.wells[6].stones + self.wells[well].stones;
                self.wells[well].stones = 0;
            }
            Player::Two => {
                self.wells[13].stones = self.wells[13].stones + self.wells[well].stones;
                self.wells[well].stones = 0;
            }
        }
    }

    fn reflective_index(ind: usize) -> usize {
        return (12 - ind) as usize;
    }

    fn go_again(player: Player, well: usize) -> bool {
        match player {
            Player::One => return well == 6,
            Player::Two => return well == 13,
        }
    }

    fn ended_on_opponents_side(player: &Player, end_well: usize) -> bool {
        match player {
            Player::One => return end_well > 6,
            Player::Two => return end_well < 6,
        }
    }

    pub fn game_over(&self) -> bool {
        return (self.wells[0].stones == 0
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
                && self.wells[12].stones == 0);
    }

    pub fn get_score(&self) -> (i32, i32) {
        return (
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
        );
    }
}

#[derive(Default, Debug)]
pub struct Well {
    stones: i32,
    adjacent_well: i32,
}

impl Well {
    pub fn new(total_stones: i32, next_well: i32) -> Well {
        return Well {
            stones: total_stones,
            adjacent_well: next_well,
        };
    }
}

fn main() {
    let mut board = MancalaBoard::new();
    while !board.game_over() {
        while board.move_well(rand::thread_rng().gen_range(0..6), Player::One) {}
        while board.move_well(rand::thread_rng().gen_range(7..13), Player::Two) {}
    }

    println!("{:#?}", board);
    println!("{:#?}", board.get_score());
}
