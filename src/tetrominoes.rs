use crate::piece::Position;
use rand::RngExt;

pub enum Tetromino {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

impl Tetromino {
    pub fn random() -> Self {
        match rand::rng().random_range(0..7) {
            0 => Self::I,
            1 => Self::O,
            2 => Self::T,
            3 => Self::S,
            4 => Self::Z,
            5 => Self::J,
            _ => Self::L,
        }
    }
}

pub fn get_orientations(tetromino: &Tetromino) -> &'static [[Position; 4]] {
    match tetromino {
        Tetromino::I => &I_ORIENTATIONS,
        Tetromino::O => &O_ORIENTATIONS,
        Tetromino::T => &T_ORIENTATIONS,
        Tetromino::J => &J_ORIENTATIONS,
        Tetromino::L => &L_ORIENTATIONS,
        Tetromino::S => &S_ORIENTATIONS,
        Tetromino::Z => &Z_ORIENTATIONS,
    }
}

const I_ORIENTATIONS: [[Position; 4]; 2] = [
    // Horizontal
    [
        Position { x: -2, y: 0 },
        Position { x: -1, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 1, y: 0 },
    ],
    // Vertical
    [
        Position { x: 0, y: -2 },
        Position { x: 0, y: -1 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 1 },
    ],
];

const O_ORIENTATIONS: [[Position; 4]; 1] = [[
    Position { x: -1, y: 0 },
    Position { x: 0, y: 0 },
    Position { x: -1, y: 1 },
    Position { x: 0, y: 1 },
]];

const T_ORIENTATIONS: [[Position; 4]; 4] = [
    // Down
    [
        Position { x: -1, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 1, y: 0 },
        Position { x: 0, y: 1 },
    ],
    // Left
    [
        Position { x: 0, y: -1 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 1 },
        Position { x: -1, y: 0 },
    ],
    // Up
    [
        Position { x: -1, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 1, y: 0 },
        Position { x: 0, y: -1 },
    ],
    // Right
    [
        Position { x: 0, y: -1 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 1 },
        Position { x: 1, y: 0 },
    ],
];

const L_ORIENTATIONS: [[Position; 4]; 4] = [
    // Up
    [
        Position { x: -1, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 1, y: 0 },
        Position { x: 1, y: 1 },
    ],
    // Right
    [
        Position { x: 0, y: -1 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 1 },
        Position { x: -1, y: 1 },
    ],
    // Down
    [
        Position { x: -1, y: -1 },
        Position { x: -1, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 1, y: 0 },
    ],
    // Left
    [
        Position { x: 1, y: -1 },
        Position { x: 0, y: -1 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 1 },
    ],
];

const J_ORIENTATIONS: [[Position; 4]; 4] = [
    // Up
    [
        Position { x: -1, y: 1 },
        Position { x: -1, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 1, y: 0 },
    ],
    // Right
    [
        Position { x: 0, y: -1 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 1 },
        Position { x: -1, y: -1 },
    ],
    // Down
    [
        Position { x: -1, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 1, y: 0 },
        Position { x: 1, y: -1 },
    ],
    // Left
    [
        Position { x: 1, y: 1 },
        Position { x: 0, y: -1 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 1 },
    ],
];

const Z_ORIENTATIONS: [[Position; 4]; 2] = [
    // Horizontal
    [
        Position { x: -1, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 1 },
        Position { x: 1, y: 1 },
    ],
    // Vertical
    [
        Position { x: 1, y: -1 },
        Position { x: 1, y: 0 },
        Position { x: 0, y: 0 },
        Position { x: 0, y: 1 },
    ],
];

const S_ORIENTATIONS: [[Position; 4]; 2] = [
    // Horizontal
    [
        Position { x: -1, y: 1 },
        Position { x: 0, y: 1 },
        Position { x: 0, y: 0 },
        Position { x: 1, y: 0 },
    ],
    // Vertical
    [
        Position { x: 0, y: -1 },
        Position { x: 0, y: 0 },
        Position { x: 1, y: 0 },
        Position { x: 1, y: 1 },
    ],
];
