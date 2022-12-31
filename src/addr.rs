pub type Frame = AlignedAddress<0x1000>;
pub type Page = AlignedAddress<0x1000>;
pub type Address = AlignedAddress<0x1>;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AlignedAddress<const ALIGN: usize>(usize);

impl<const ALIGN: usize> AlignedAddress<ALIGN> {
    const NON_CANONICAL_BITS: usize = 0xFFF00000_00000000;
    const CANONICAL_BITS: usize = !Self::NON_CANONICAL_BITS;

    #[inline]
    pub const fn zero() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn is_canonical(address: usize) -> bool {
        (address & Self::NON_CANONICAL_BITS) == 0
    }

    /// Constructs a new `Address<Physical>` if the provided address is canonical.
    #[inline]
    pub fn new(address: usize) -> Option<Self> {
        debug_assert!(ALIGN.is_power_of_two());

        (Self::is_canonical(address) && address).then_some(Self(address))
    }

    #[inline]
    pub const fn new_truncate(address: usize) -> Self {
        Self(address & Self::CANONICAL_BITS)
    }

    #[inline]
    pub const fn is_frame_aligned(self) -> bool {
        (self.0 & 0xFFF) == 0
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
