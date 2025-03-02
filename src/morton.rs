use crate::geom::BMask3;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct MortonKey(u64);

impl MortonKey {
    pub const LEVELS: u32 = (u64::BITS - 1) / 3;

    pub const fn root() -> MortonKey {
        MortonKey(0)
    }

    pub const fn none() -> MortonKey {
        MortonKey(0)
    }

    pub const fn is_none(&self) -> bool {
        self.0 != 0
    }

    pub const fn parent(&self) -> MortonKey {
        MortonKey(self.0 >> 3)
    }

    pub const fn child(&self, index: BMask3) -> MortonKey {
        MortonKey((self.0 << 3) | (index.bits() as u64))
    }

    pub fn level(&self) -> u32 {
        self.0.checked_ilog2().unwrap_or(0)
    }
}
