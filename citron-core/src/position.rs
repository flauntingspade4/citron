use core::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position(u8);

impl Position {
    #[must_use]
    pub const fn new(x: u8, y: u8) -> Self {
        Self((y << 3) + x)
    }
    pub(crate) const fn from_u8(position: u8) -> Self {
        Self(position)
    }
    #[must_use]
    pub fn to_uci(&self) -> (char, char) {
        (
            match self.x() {
                0 => 'a',
                1 => 'b',
                2 => 'c',
                3 => 'd',
                4 => 'e',
                5 => 'f',
                6 => 'g',
                7 => 'h',
                _ => panic!(),
            },
            match self.y() {
                0 => '1',
                1 => '2',
                2 => '3',
                3 => '4',
                4 => '5',
                5 => '6',
                6 => '7',
                7 => '8',
                _ => panic!(),
            },
        )
    }
    #[must_use]
    pub fn from_uci(input: &str) -> Option<Self> {
        let mut chars = input.chars();

        let x = match chars.next()? {
            'a' | 'A' => 0,
            'b' | 'B' => 1,
            'c' | 'C' => 2,
            'd' | 'D' => 3,
            'e' | 'E' => 4,
            'f' | 'F' => 5,
            'g' | 'G' => 6,
            'h' | 'H' => 7,
            _ => return None,
        };

        let y = match chars.next()? {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return None,
        };

        Some(Self::new(x, y))
    }
    #[must_use]
    pub const fn x(&self) -> u8 {
        self.0 & 7
    }
    #[must_use]
    pub const fn y(&self) -> u8 {
        self.0 >> 3
    }
    #[must_use]
    pub const fn index(&self) -> u8 {
        self.0
    }
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        self.0 < 64
    }
    pub fn positions() -> impl Iterator<Item = Self> {
        (0..64).map(Self)
    }
    #[must_use]
    pub fn from_bitmap(bitmap: u64) -> Self {
        if bitmap == 0 {
            println!("Wtaf");
        }
        Self(crate::magic::bitscan_forward(bitmap) as u8)
    }
    #[must_use]
    pub const fn to_bitmap(&self) -> u64 {
        1 << self.0
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.x(), self.y())
    }
}

#[test]
fn move_test() {
    assert_eq!(Position::new(0, 0), Position(0));

    let position = Position::new(0, 5);

    assert_eq!(position.x(), 0);
    assert_eq!(position.y(), 5);

    assert_eq!(Position::new(5, 0), Position(5));

    let position = Position::new(5, 5);

    assert_eq!(position.x(), 5);
    assert_eq!(position.y(), 5);

    assert!(Position::new(0, 0).is_valid());
    assert!(Position::new(2, 3).is_valid());
    assert!(Position::new(7, 7).is_valid());
    assert!(!Position::new(9, 12).is_valid());
}

#[test]
fn uci_test() {
    let position = Position::from_uci("e4").unwrap();
    assert_eq!(position, Position::new(4, 3));

    let position = Position::from_u8(28);
    assert_eq!(position.to_uci(), ('e', '4'));

    let position = Position::from_uci("a1").unwrap();
    assert_eq!(position, Position::new(0, 0));

    let position = Position::from_u8(0);
    assert_eq!(position.to_uci(), ('a', '1'));

    let position = Position::from_uci("h8").unwrap();
    assert_eq!(position.to_uci(), ('h', '8'));

    let position = Position::from_u8(63);
    assert_eq!(position.to_uci(), ('h', '8'));

    let position = Position::from_uci("b2").unwrap();
    assert_eq!(position.to_uci(), ('b', '2'));

    let position = Position::from_u8(9);
    assert_eq!(position.to_uci(), ('b', '2'));
}
