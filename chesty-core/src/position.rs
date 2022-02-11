use core::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// A way of representing any possible position
pub struct Position(u8);

impl Position {
    #[must_use]
    /// Creates a new position off a given x and y
    pub const fn new(x: u8, y: u8) -> Self {
        Self((x << 3) + y)
    }
    #[must_use]
    /// Attempts to parse `input`, returns the parsed
    /// result if successful
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
    /// Gets the x coordinate of `self`
    pub const fn x(&self) -> u8 {
        self.0 >> 3
    }
    #[must_use]
    /// Gets the y coordinate of `self`
    pub const fn y(&self) -> u8 {
        self.0 & 7
    }
    #[must_use]
    /// Gets the index of `self`
    pub(crate) const fn index(&self) -> u8 {
        self.0
    }
    #[must_use]
    /// Returns `true` if x and y are less than 64, false if not
    pub const fn is_valid(&self) -> bool {
        self.index() < 64
    }
    /// Returns an iterator over all positions on the chess board
    pub fn positions() -> impl Iterator<Item = Self> {
        (0..64).map(|i| Self(i))
    }
    #[must_use]
    /// Performs `self + (x, y)`, returning if it exists within the
    /// bounds of the board
    pub fn checked_add_to(&self, x: i8, y: i8) -> Option<Self> {
        let (x0, y0) = (self.x(), self.y());
        let (x, y) = (x0.checked_add_signed(x)?, y0.checked_add_signed(y)?);

        let to_return = Self::new(x, y);

        to_return.is_valid().then(|| to_return)
    }
    #[must_use]
    /// Performs `self += (x, y)`, returning if it exists within the
    /// bounds of the board
    pub fn checked_translate(&mut self, x: i8, y: i8) -> Option<()> {
        self.checked_add_to(x, y).map(|p| *self = p)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}", self.x(), self.y())
    }
}

pub fn position_to_u16(positions: (Position, Position)) -> u16 {
    unsafe { core::mem::transmute(positions) }
}

pub fn u16_to_position(positions: u16) -> (Position, Position) {
    unsafe { core::mem::transmute(positions) }
}

#[test]
fn move_test() {
    assert_eq!(Position::new(0, 0), Position(0));

    let position = Position::new(0, 5);

    assert_eq!(position.x(), 0);
    assert_eq!(position.y(), 5);
}

#[test]
fn uci_test() {
    let position = Position::from_uci("e4").unwrap();
    assert_eq!(position, Position::new(4, 3));
}
