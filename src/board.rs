use std::fmt::Display;

macro_rules! try_add {
    ($moves: expr, $board: expr, $from: expr, $( $remove_to:expr ),*) => {
        {
            $(
                if $board.active($remove_to.0) && !$board.active($remove_to.1) {
                    $moves.push(Move::new($from, $remove_to.0, $remove_to.1))
                }
            )*
        }
    };
}

pub struct Board {
    inner: u64,
}

impl Board {
    pub const STARTING_BOARD: Self = Self {
        inner: 0b111111111111111111111111111111110,
    };

    const FINISHED_BOARD: Self = Self {
        inner: 0b000000000000000000000000000000001,
    };

    pub fn active(&self, index: u64) -> bool {
        (self.inner & (1u64 << index)) != 0
    }

    pub fn play_move(&mut self, r#move: Move) {
        let combination = r#move.combine();

        self.inner ^= combination;
    }

    pub fn finished(&self) -> bool {
        self.inner == Self::FINISHED_BOARD.inner
    }

    pub fn all_legal_moves(&self) -> Vec<Move> {
        let mut moves = Vec::with_capacity(10);

        let mut inner = self.inner;

        while inner != 0 {
            let from = pop_lsb(&mut inner);

            self.all_moves_from(from, &mut moves);
        }

        moves
    }

    fn all_moves_from(&self, from: u64, moves: &mut Vec<Move>) {
        match from {
            0 => try_add!(moves, self, from, (1, 2), (9, 10), (17, 18), (25, 26)),
            1 => try_add!(moves, self, from, (2, 3), (8, 14), (0, 17), (32, 29)),
            2 => try_add!(moves, self, from, (1, 0)),
            3 => try_add!(moves, self, from, (2, 1)),
            4 => try_add!(moves, self, from, (7, 8), (3, 5)),
            5 => try_add!(moves, self, from, (3, 4), (6, 32)),
            6 => try_add!(moves, self, from, (2, 7), (32, 25)),
            7 => try_add!(moves, self, from, (8, 9), (2, 6)),
            8 => try_add!(moves, self, from, (7, 4), (14, 13), (9, 16), (1, 32)),
            9 => try_add!(moves, self, from, (8, 7), (10, 11), (16, 22), (0, 25)),
            10 => try_add!(moves, self, from, (9, 0)),
            11 => try_add!(moves, self, from, (10, 9)),
            12 => try_add!(moves, self, from, (11, 13), (15, 16)),
            13 => try_add!(moves, self, from, (11, 12), (14, 8)),
            14 => try_add!(moves, self, from, (10, 15), (8, 1)),
            15 => try_add!(moves, self, from, (10, 14), (16, 17)),
            16 => try_add!(moves, self, from, (9, 8), (15, 12), (22, 23), (17, 24)),
            17 => try_add!(moves, self, from, (0, 1), (16, 15), (18, 19), (24, 30)),
            18 => try_add!(moves, self, from, (17, 0)),
            19 => try_add!(moves, self, from, (18, 17)),
            20 => try_add!(moves, self, from, (21, 24), (19, 23)),
            21 => try_add!(moves, self, from, (24, 25), (18, 22)),
            22 => try_add!(moves, self, from, (16, 9), (18, 21)),
            23 => try_add!(moves, self, from, (22, 16), (19, 20)),
            24 => try_add!(moves, self, from, (25, 32), (17, 16), (21, 20), (30, 31)),
            25 => try_add!(moves, self, from, (32, 6), (0, 9), (24, 21), (26, 27)),
            26 => try_add!(moves, self, from, (25, 0)),
            27 => try_add!(moves, self, from, (26, 25)),
            28 => try_add!(moves, self, from, (29, 32), (27, 31)),
            29 => try_add!(moves, self, from, (32, 1), (26, 30)),
            30 => try_add!(moves, self, from, (26, 29), (24, 17)),
            31 => try_add!(moves, self, from, (27, 28), (30, 24)),
            32 => try_add!(moves, self, from, (6, 5), (1, 8), (25, 24), (29, 28)),
            _ => panic!("Illegal 'from' index {}", from),
        };
        /*self.try_add(
            from,
            match from {
                0 => &[(1, 2), (9, 10), (17, 18), (25, 26)],
                1 => &[(2, 3), (8, 14), (0, 17), (32, 29)],
                2 => &[(1, 0)],
                3 => &[(2, 1)],
                4 => &[(7, 8), (3, 5)],
                5 => &[(3, 4), (6, 32)],
                6 => &[(2, 7), (32, 25)],
                7 => &[(8, 9), (2, 6)],
                8 => &[(7, 4), (14, 13), (9, 16), (1, 32)],
                9 => &[(8, 7), (10, 11), (16, 22), (0, 25)],
                10 => &[(9, 0)],
                11 => &[(10, 9)],
                12 => &[(11, 13), (15, 16)],
                13 => &[(11, 12), (14, 8)],
                14 => &[(10, 15), (8, 1)],
                15 => &[(10, 14), (16, 17)],
                16 => &[(9, 8), (15, 12), (22, 23), (17, 24)],
                17 => &[(0, 1), (16, 15), (18, 19), (24, 30)],
                18 => &[(17, 0)],
                19 => &[(18, 17)],
                20 => &[(21, 24), (19, 23)],
                21 => &[(24, 25), (18, 22)],
                22 => &[(16, 9), (18, 21)],
                23 => &[(22, 16), (19, 20)],
                24 => &[(25, 32), (17, 16), (21, 20), (30, 31)],
                25 => &[(32, 6), (0, 9), (24, 21), (26, 27)],
                26 => &[(25, 0)],
                27 => &[(26, 25)],
                28 => &[(29, 32), (27, 31)],
                29 => &[(32, 1), (26, 30)],
                30 => &[(26, 29), (24, 17)],
                31 => &[(27, 28), (30, 24)],
                32 => &[(6, 5), (1, 8), (25, 24), (29, 28)],
                _ => panic!("Illegal 'from' index {}", from),
            },
            moves,
        )*/
    }

    fn try_add(&self, from: u64, remove_to: &[(u64, u64)], moves: &mut Vec<Move>) {
        for (remove, to) in remove_to {
            if self.active(*remove) && !self.active(*to) {
                moves.push(Move::new(from, *remove, *to))
            }
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::STARTING_BOARD
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "todo, ngl chief <3")
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    from: u64,
    remove: u64,
    to: u64,
}

impl Move {
    fn new(from: u64, remove: u64, to: u64) -> Self {
        Self { from, remove, to }
    }
    fn combine(&self) -> u64 {
        (1u64 << self.from) | (1u64 << self.remove) | (1u64 << self.to)
    }
}

#[must_use]
pub const fn bitscan_forward(board: u64) -> u64 {
    const INDEX64: [u64; 64] = [
        0, 47, 1, 56, 48, 27, 2, 60, 57, 49, 41, 37, 28, 16, 3, 61, 54, 58, 35, 52, 50, 42, 21, 44,
        38, 32, 29, 23, 17, 11, 4, 62, 46, 55, 26, 59, 40, 36, 15, 53, 34, 51, 20, 43, 31, 22, 10,
        45, 25, 39, 14, 33, 19, 30, 9, 24, 13, 18, 8, 12, 7, 6, 5, 63,
    ];
    const DEBRUIJN64: u64 = 0x03f79d71b4cb0a89;

    INDEX64[(((board ^ (board - 1)).wrapping_mul(DEBRUIJN64)) >> 58) as usize]
}

pub fn pop_lsb(mask: &mut u64) -> u64 {
    let index = bitscan_forward(*mask);

    *mask ^= 1 << index;

    index as u64
}
