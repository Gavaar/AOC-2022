use std::{collections::HashSet};

#[derive(Debug)]
pub struct Cave {
    pub falling_rock: Option<Vec<(usize, usize)>>,
    rock_pos: (usize, usize),
    state: Vec<HashSet<i32>>,
}

impl Cave {
    fn new() -> Self {
        Self {
            falling_rock: None,
            rock_pos: (0, 0),
            state: vec![HashSet::new(); 7],
        }
    }

    fn rock_x_range(&self) -> (usize, usize) {
        let start_check = self.rock_pos.0;
        let end_check = self.rock_pos.0 + self.falling_rock.as_ref().unwrap().len();
        
        (start_check, end_check)
    }

    fn move_right(&mut self) {
        let current_rock = self.falling_rock.as_ref();
        let (start, end) = self.rock_x_range();
        let mut can_move = true;

        if end >= 7 {
            can_move = false;
        } else {
            for ind in start..end {
                let rock_ind = ind - self.rock_pos.0;
                let (piece_min, piece_max) = current_rock.unwrap()[rock_ind];
                let actual_min = piece_min as i32 + self.rock_pos.1 as i32;
                let actual_max = piece_max as i32 + self.rock_pos.1 as i32;
    
                for height in actual_min..actual_max {
                    if self.state[ind + 1].contains(&height) {
                        can_move = false;
                    }
                }
            }
        }

        if can_move {
            self.rock_pos.0 += 1;
        }
    }

    fn move_left(&mut self) {
        let current_rock = self.falling_rock.as_ref();
        let (start, end) = self.rock_x_range();
        let mut can_move = true;

        if start == 0 {
            can_move = false;
        } else {
            for ind in start..end {
                let rock_ind = ind - self.rock_pos.0;
                let (piece_min, piece_max) = current_rock.unwrap()[rock_ind];
                let actual_min = piece_min as i32 + self.rock_pos.1 as i32;
                let actual_max = piece_max as i32 + self.rock_pos.1 as i32;
    
                for height in actual_min..actual_max {
                    if self.state[ind - 1].contains(&height) {
                        can_move = false;
                    }
                }
            }
        }

        if can_move {
            self.rock_pos.0 -= 1;
        }
    }

    fn move_down(&mut self) -> bool {
        let (start, end) = self.rock_x_range();
        let mut can_move = true;

        if self.rock_pos.1 == 0 {
            can_move = false;
        } else {
            for ind in start..end {
                let rock_ind = ind - self.rock_pos.0;
                let (piece_min, _) = self.falling_rock.as_ref().unwrap()[rock_ind];
                let next_height = piece_min as i32 + self.rock_pos.1 as i32 - 1;
    
                if self.state[ind].contains(&next_height) {
                    can_move = false;
                }
            }
        }

        if can_move { self.rock_pos.1 -= 1 };

        return can_move;
    }

    fn add_new_rock_to_state(&mut self) {
        let (start, end) = self.rock_x_range();

        for ind in start..end {
            let rock_ind = ind - self.rock_pos.0;
            let (piece_min, piece_max) = self.falling_rock.as_ref().unwrap()[rock_ind];
            let actual_min = piece_min as i32 + self.rock_pos.1 as i32;
            let actual_max = piece_max as i32 + self.rock_pos.1 as i32;

            for height in actual_min..actual_max {
                self.state[ind].insert(height);
            }
        }

        for state in &mut self.state {
            if state.len() > 10 {
                state.remove(state.clone().iter().min().unwrap());
            }
        }
    }

    pub fn height(&self) -> i32 {
        self.state.iter().map(|inner| match inner.iter().max() {
            Some(v) => *v,
            None => -1,
        }).max().unwrap()
    }
    
    pub fn insert_rock(&mut self, rock: &Vec<(usize, usize)>) {
        self.falling_rock = Some(rock.to_owned());
        self.rock_pos = (2, (self.height() + 4) as usize);
    }

    pub fn act_on(&mut self, char: char) {
        let current_rock = self.falling_rock.as_ref();
        if current_rock == None { return println!("Cant act without a rock"); }
        
        if char == '<' { self.move_left(); }
        if char == '>' { self.move_right(); }
        let moved = self.move_down();
        
        if !moved {
            self.add_new_rock_to_state();
            self.falling_rock = None;
        }
    }
}

pub fn create() -> Cave {
    Cave::new()
}
