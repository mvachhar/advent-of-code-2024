use std::fmt;
use std::fmt::Display;
use std::convert::TryFrom;

use crate::board::BoardIndex;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirVec(pub isize, pub isize);

impl DirVec {
    pub fn add(&self, other: &DirVec) -> DirVec {
        DirVec(self.0 + other.0, self.1 + other.1)
    }

    pub fn neg(&self) -> DirVec {
        DirVec(-self.0, -self.1)
    }

    pub fn sub(&self, other: &DirVec) -> DirVec {
        self.add(&other.neg())
    }
}

impl Display for DirVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}, {}>", self.0, self.1)
    }
}

pub const DIR_LEFT: DirVec = DirVec(-1, 0);
pub const DIR_RIGHT: DirVec = DirVec(1, 0);
pub const DIR_UP: DirVec = DirVec(0, -1);
pub const DIR_DOWN: DirVec = DirVec(0, 1);

impl TryFrom<&BoardIndex> for DirVec {
    type Error = <isize as TryFrom<usize>>::Error;

    fn try_from(index: &BoardIndex) -> Result<Self, Self::Error> {
        let x = isize::try_from(index.raw()[1])?;
        let y = isize::try_from(index.raw()[0])?;
        return Ok(DirVec(x, y));
    }
}
