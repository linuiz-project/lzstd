pub type Frame = AlignedAddress<0x1000>;
pub type Address = AlignedAddress<0x1>;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlignedAddress<const ALIGN_SHIFT: u32>(usize);

impl<const ALIGN_SHIFT: u32> AlignedAddress<ALIGN_SHIFT> {
    const NON_CANONICAL_MASK: usize = 0xFFF00000_00000000;
    const ALIGN_MASK: usize = 1usize.checked_shl(ALIGN_SHIFT).unwrap_or(0).wrapping_sub(1);

    #[inline]
    pub const fn zero() -> Self {
        Self(0)
    }

    const fn checked_canonical(address: usize) -> Option<Self> {
        ((address & Self::NON_CANONICAL_MASK) == 0).then_some(Self(address))
    }

    /// Constructs a new `Address<Physical>` if the provided address is canonical.
    #[inline]
    pub const fn new(address: usize) -> Option<Self> {
        ((address & Self::ALIGN_MASK) == 0)
            .then_some(address)
            .and_then(Self::checked_canonical)
    }

    #[inline]
    pub const fn new_truncate(address: usize) -> Self {
        Self(address & !Self::NON_CANONICAL_MASK & !Self::ALIGN_MASK)
    }

    #[inline]
    pub const fn from_index(index: usize) -> Option<Self> {
        index
            .checked_shl(ALIGN_SHIFT)
            .and_then(Self::checked_canonical)
    }

    #[inline]
    pub const fn index(self) -> usize {
        self.0.checked_shr(ALIGN_SHIFT).unwrap_or(0)
    }
}

impl From<Address> for usize {
    fn from(value: Address) -> Self {
        value.0
    }
}

impl From<Address> for u64 {
    fn from(value: Address) -> Self {
        value.0 as u64
    }
}
