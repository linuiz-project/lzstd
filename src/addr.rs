use crate::PAGE_ALIGN_MASK;

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
    pub const fn from_offset(offset: usize) -> Option<Self> {
        offset
            .checked_shl(ALIGN_SHIFT)
            .and_then(Self::checked_canonical)
    }

    #[inline]
    pub const fn offset(self) -> usize {
        self.0.checked_shr(ALIGN_SHIFT).unwrap_or(0)
    }
}

impl<const ALIGN_SHIFT: u32> From<AlignedAddress<ALIGN_SHIFT>> for usize {
    fn from(address: AlignedAddress<ALIGN_SHIFT>) -> Self {
        address.0
    }
}

impl<const ALIGN_SHIFT: u32> From<AlignedAddress<ALIGN_SHIFT>> for u64 {
    fn from(address: AlignedAddress<ALIGN_SHIFT>) -> Self {
        address.0 as u64
    }
}

pub trait AddressKind: Sized {
    type InitType;
    type ReprType;

    fn new(init: Self::InitType) -> Option<Self::ReprType>;
}

const fn checked_phys_canonical(address: usize) -> bool {
    const NON_CANONICAL_MASK: usize = 0xFFF00000_00000000;

    (address & NON_CANONICAL_MASK) == 0
}

pub struct Physical;
impl AddressKind for Physical {
    type InitType = usize;
    type ReprType = usize;

    fn new(init: Self::InitType) -> Option<Self::ReprType> {
        checked_phys_canonical(init).then_some(init)
    }
}

pub struct Frame;
impl AddressKind for Frame {
    type InitType = usize;
    type ReprType = usize;

    fn new(init: Self::InitType) -> Option<Self::ReprType> {
        (((init & PAGE_ALIGN_MASK) == 0) && checked_phys_canonical(init)).then_some(init)
    }
}

pub struct Address<Kind: AddressKind>(Kind::ReprType);

impl<Kind: AddressKind> Address<Kind> {
    pub fn new(init: Kind::InitType) -> Option<Self> {
        Kind::new(init).map(Self)
    }
}
