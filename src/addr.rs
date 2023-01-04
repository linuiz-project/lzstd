use crate::{PAGE_ALIGN_MASK, PAGE_ALIGN_SHIFT};

const PHYS_NON_CANONICAL_MASK: usize = 0xFFF0_0000_0000_0000;
const VIRT_NON_CANONICAL_MASK: usize = 0xFFFF_0000_0000_0000;

const fn checked_phys_canonical(address: usize) -> bool {
    (address & PHYS_NON_CANONICAL_MASK) == 0
}

const fn checked_virt_canonical(address: usize) -> bool {
    matches!(address >> 47, 0 | 0x1FF)
}

const fn virt_truncate(address: usize) -> usize {
    (((address << 16) as isize) >> 16) as usize
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
        (((init & PAGE_ALIGN_MASK) == 0) && checked_phys_canonical(init)).then_some(init)
    }

    fn new_truncate(init: Self::InitType) -> Self::ReprType {
        init & !PHYS_NON_CANONICAL_MASK & !PAGE_ALIGN_MASK
    }
}
impl IndexableAddressKind for Frame {
    fn from_index(index: usize) -> Option<Self::ReprType> {
        (index <= !PHYS_NON_CANONICAL_MASK).then_some(index << PAGE_ALIGN_SHIFT)
    }

    fn index(repr: Self::ReprType) -> usize {
        repr >> PAGE_ALIGN_SHIFT
    }
}

pub struct Virtual;
impl AddressKind for Virtual {
    type InitType = usize;
    type ReprType = usize;

    fn new(init: Self::InitType) -> Option<Self::ReprType> {
        checked_virt_canonical(init).then_some(init)
    }

    fn new_truncate(init: Self::InitType) -> Self::ReprType {
        virt_truncate(init)
    }
}
impl PtrableAddressKind for Virtual {
    fn from_ptr<T>(ptr: *mut T) -> Self::ReprType {
        ptr.addr()
    }

    fn as_ptr(repr: Self::ReprType) -> *mut u8 {
        repr as *mut u8
    }
}

pub struct Page;
impl AddressKind for Page {
    type InitType = usize;
    type ReprType = usize;

    fn new(init: Self::InitType) -> Option<Self::ReprType> {
        (((init & PAGE_ALIGN_MASK) == 0) && checked_phys_canonical(init)).then_some(init)
    }

    fn new_truncate(init: Self::InitType) -> Self::ReprType {
        init & !PHYS_NON_CANONICAL_MASK & !PAGE_ALIGN_MASK
    }
}
impl PtrableAddressKind for Page {
    fn from_ptr<T>(ptr: *mut T) -> Self::ReprType {
        ptr.addr()
    }

    fn as_ptr(repr: Self::ReprType) -> *mut u8 {
        repr as *mut u8
    }
}
impl IndexableAddressKind for Page {
    fn from_index(index: usize) -> Option<Self::ReprType> {
        (index <= !VIRT_NON_CANONICAL_MASK).then_some(index << PAGE_ALIGN_SHIFT)
    }

    fn index(repr: Self::ReprType) -> usize {
        repr >> PAGE_ALIGN_SHIFT
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

impl<Init, Repr, Kind: AddressKind<InitType = Init, ReprType = Repr>> Default for Address<Kind>
where
    Repr: Default,
{
    fn default() -> Self {
        Self(Repr::default())
    }
}
