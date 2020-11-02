pub use srcpos::*;

#[cfg(feature = "srcpos_get_derive")]
pub use srcpos_get_derive::*;

/// Calculate Loc
pub trait GetLoc {
    /// Calculate Loc
    fn loc(&self) -> Loc;
}

/// Calculate Pos
pub trait GetPos {
    /// Calculate Pos
    fn pos(&self) -> Pos;
}

impl<T: GetLoc> GetLoc for &T {
    fn loc(&self) -> Loc {
        (**self).loc()
    }
}

impl<T: GetLoc> GetLoc for &mut T {
    fn loc(&self) -> Loc {
        (**self).loc()
    }
}

impl<T: GetPos> GetPos for &T {
    fn pos(&self) -> Pos {
        (**self).pos()
    }
}

impl<T: GetPos> GetPos for &mut T {
    fn pos(&self) -> Pos {
        (**self).pos()
    }
}

impl GetLoc for Loc {
    fn loc(&self) -> Loc {
        *self
    }
}

impl GetPos for Pos {
    fn pos(&self) -> Pos {
        *self
    }
}

impl GetPos for Loc {
    fn pos(&self) -> Pos {
        self.from
    }
}
