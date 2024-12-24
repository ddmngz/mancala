#![allow(dead_code)]

use std::fmt;
use std::mem;

const PIT_SIZE: usize = 14;
const PIT_LAST: usize = PIT_SIZE - 1;

#[derive(Copy, Clone)]
struct Pit {
    stone_count: u8,
}

impl Pit {
    const fn take(&mut self) -> u8 {
        mem::replace(&mut self.stone_count, 0)
    }

    const fn new() -> Self {
        Self { stone_count: 2 }
    }
    const fn empty() -> Self {
        Self { stone_count: 0 }
    }
}

pub struct Board {
    // should always have playerid 1
    player_1: Player,
    slots: [Pit; PIT_SIZE],
    // should always have playerid 2
    player_2: Player,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub const fn new() -> Self {
        Self {
            player_1: Player { id: ID::One },
            player_2: Player { id: ID::Two },
            slots: [Pit::new(); PIT_SIZE],
        }
    }

    pub fn turn_avalanche(&mut self, player: &ID, start: &StartPit) {
        let index = start.as_index(player);
        self.avalanche_inner(index, player)
    }

    fn avalanche_inner(&mut self, index: usize, player: &ID) {
        println!("{}", self);
        let stones = self.slots[index].take();
        // go again if we end on zero
        if let AvalancheTurnResult::Continue { ended_at } =
            self.take_turn(stones, Self::next_pit(index, player), player)
        {
            self.avalanche_inner(ended_at, player);
        }
    }

    const fn take_turn(&mut self, stones: u8, index: usize, player: &ID) -> AvalancheTurnResult {
        let remaining = Self::add_stone(&mut self.slots[index], stones);
        if remaining == 0 {
            if self.slots[index].stone_count == 1 {
                AvalancheTurnResult::Done
            } else {
                AvalancheTurnResult::Continue { ended_at: index }
            }
        } else {
            self.take_turn(remaining, Self::next_pit(index, player), player)
        }
    }

    const fn add_stone(pit: &mut Pit, stones: u8) -> u8 {
        pit.stone_count += 1;
        stones - 1
    }

    const fn next_pit(index: usize, player: &ID) -> usize {
        let player1_pit = 0;
        let player2_pit = PIT_LAST;
        let next = index + 1;
        if next == player2_pit {
            if player.eq(&ID::Two) {
                PIT_LAST
            } else {
                0
            }
        } else if next == player1_pit {
            if player.eq(&ID::One) {
                0
            } else {
                1
            }
        } else {
            next
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        /*
        format_args!("[{:width$}]", self.stone_count, width = spacing)
        */

        let p1_slot = self.slots[StartPit::Mine.as_index(&ID::One)].stone_count;
        let p2_slot = self.slots[StartPit::Mine.as_index(&ID::Two)].stone_count;

        let p1_tens_place = p1_slot / 10;
        let p1_ones_place = p1_slot % 10;

        let p2_tens_place = p2_slot / 10;
        let p2_ones_place = p2_slot % 10;

        write!(
            f,
            "[{p1_tens_place:2^}] [{:2^}] [{:2^}] [{:2^}] [{:2^}] [{:2^}] [{:2^}] [{p2_tens_place:2^}]\n[{p1_ones_place:2^}] [{:2^}] [{:2^}] [{:2^}] [{:2^}] [{:2^}] [{:2^}] [{p2_ones_place:2^}]
            ", self.slots[1].stone_count, self.slots[2].stone_count,self.slots[3].stone_count, self.slots[4].stone_count, 
            self.slots[5].stone_count, self.slots[6].stone_count,self.slots[8].stone_count, self.slots[9].stone_count,
            self.slots[10].stone_count, self.slots[11].stone_count,self.slots[12].stone_count, self.slots[13].stone_count,



        )
    }
}

enum AvalancheTurnResult {
    Done,
    Continue { ended_at: usize },
}

pub enum StartPit {
    Mine,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl StartPit {
    const fn as_index(&self, player: &ID) -> usize {
        use StartPit::{Five, Four, Mine, One, Six, Three, Two};
        match player {
            ID::One => match self {
                Mine => 0,
                One => 1,
                Two => 2,
                Three => 3,
                Four => 4,
                Five => 5,
                Six => 6,
            },
            ID::Two => match self {
                Mine => 7,
                One => 8,
                Two => 9,
                Three => 10,
                Four => 11,
                Five => 12,
                Six => 13,
            },
        }
    }
}

pub enum ID {
    One,
    Two,
}

impl ID {
    const fn eq(&self, other: &ID) -> bool {
        matches!((self, other), (ID::One, ID::One) | (ID::Two, ID::Two))
    }
}

struct Player {
    id: ID,
}

const fn board_slice<'a>(player: &Player, board: &'a Board) -> &'a [Pit; 6] {
    match player.id {
        ID::One => board.slots.first_chunk::<6>().unwrap(),
        ID::Two => board.slots.last_chunk::<6>().unwrap(),
    }
}

struct BoardSlice<'a>(&'a [Pit; 7]);

impl<'a> BoardSlice<'a> {
    const fn new(player: &Player, board: &'a Board) -> Self {
        Self(match player.id {
            ID::One => board.slots.first_chunk::<7>().expect("unreachable"),
            ID::Two => board.slots.last_chunk::<7>().expect("unreachable"),
        })
    }

    const fn mine(&self) -> &Pit {
        &self.0[0]
    }

    const fn rest(&self) -> &[Pit; 6] {
        match &self.0.last_chunk::<6>() {
            Some(chunk) => chunk,
            None => unreachable!(),
        }
    }

    const fn start_pit(&self, pit: &StartPit) -> &Pit {
        use StartPit::{Five, Four, Mine, One, Six, Three, Two};
        match pit {
            Mine => self.mine(),
            One => &self.rest()[0],
            Two => &self.rest()[1],
            Three => &self.rest()[2],
            Four => &self.rest()[3],
            Five => &self.rest()[4],
            Six => &self.rest()[5],
        }
    }
}

struct Game {
    board: Board,
    player_1: Player,
    player_2: Player,
}
