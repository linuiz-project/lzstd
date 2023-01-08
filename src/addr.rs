use crate::{PAGE_MASK, PAGE_SHIFT, PHYS_NON_CANONICAL_MASK};
use core::fmt;

const fn checked_phys_canonical(address: usize) -> bool {
    (address & PHYS_NON_CANONICAL_MASK) == 0
}

pub trait AddressKind: Sized {
    type InitType;
    type ReprType: Copy;

    fn new(init: Self::InitType) -> Option<Self::ReprType>;
    fn new_truncate(init: Self::InitType) -> Self::ReprType;
}

pub trait PtrableAddressKind: AddressKind {
    fn from_ptr<T>(ptr: *mut T) -> Self::ReprType;
    fn as_ptr(repr: Self::ReprType) -> *mut u8;
}

pub trait IndexableAddressKind: AddressKind {
    fn from_index(index: usize) -> Option<Self::ReprType>;
    fn index(repr: Self::ReprType) -> usize;
}

pub trait DefaultableAddressKind: AddressKind {
    fn default() -> Self::ReprType;
}

pub struct Physical;
impl AddressKind for Physical {
    type InitType = usize;
    type ReprType = usize;

    fn new(init: Self::InitType) -> Option<Self::ReprType> {
        checked_phys_canonical(init).then_some(init)
    }

    fn new_truncate(init: Self::InitType) -> Self::ReprType {
        init & !PHYS_NON_CANONICAL_MASK
    }
}

pub struct Frame;
impl AddressKind for Frame {
    type InitType = usize;
    type ReprType = usize;

    fn new(init: Self::InitType) -> Option<Self::ReprType> {
        (((init & PAGE_MASK) == 0) && checked_phys_canonical(init)).then_some(init)
    }

    fn new_truncate(init: Self::InitType) -> Self::ReprType {
        init & !PHYS_NON_CANONICAL_MASK & !PAGE_MASK
    }
}
impl IndexableAddressKind for Frame {
    fn from_index(index: usize) -> Option<Self::ReprType> {
        (index <= !PHYS_NON_CANONICAL_MASK).then_some(index << PAGE_SHIFT.get())
    }

    fn index(repr: Self::ReprType) -> usize {
        repr >> PAGE_SHIFT.get()
    }
}

pub struct Address<Kind: AddressKind>(Kind::ReprType);

impl<Kind: AddressKind> Address<Kind> {
    pub fn new(init: Kind::InitType) -> Option<Self> {
        Kind::new(init).map(Self)
    }

    pub fn new_truncate(init: Kind::InitType) -> Self {
        Self(Kind::new_truncate(init))
    }

    pub fn get(self) -> Kind::ReprType {
        self.0
    }
}

impl<Kind: PtrableAddressKind> Address<Kind> {
    pub fn from_ptr<T>(ptr: *mut T) -> Self {
        Self(Kind::from_ptr(ptr))
    }

    pub fn as_ptr(self) -> *mut u8 {
        Kind::as_ptr(self.0)
    }
}

impl<Kind: IndexableAddressKind> Address<Kind> {
    pub fn from_index(index: usize) -> Option<Self> {
        Kind::from_index(index).map(Self)
    }

    pub fn index(self) -> usize {
        Kind::index(self.0)
    }
}

impl<Repr: Default, I, K: AddressKind<InitType = I, ReprType = Repr>> Default for Address<K> {
    fn default() -> Self {
        Self(Repr::default())
    }
}

impl<Repr: Clone, I, K: AddressKind<InitType = I, ReprType = Repr>> Clone for Address<K> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Repr: Copy, I, K: AddressKind<InitType = I, ReprType = Repr>> Copy for Address<K> {}

impl<Repr: PartialEq, I, K: AddressKind<InitType = I, ReprType = Repr>> PartialEq for Address<K> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<Repr: Eq, I, K: AddressKind<InitType = I, ReprType = Repr>> Eq for Address<K> {}

impl<I, Repr: fmt::Debug, K: AddressKind<InitType = I, ReprType = Repr>> fmt::Debug for Address<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Address").field(&self.0).finish()
    }
}
