use crate::Ptr;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Page(usize);

impl Page {
    const MIN_ALIGN_SHIFT: u32 = 12;
    const MIN_ALIGN: usize = 1 << Self::MIN_ALIGN_SHIFT;

    #[inline]
    pub const fn index(self) -> usize {
        self.0
    }

    pub const fn 
}



impl<T> TryFrom<Ptr<T>> for Page {
    type Error = Ptr<T>;

    fn try_from(value: Ptr<T>) -> Result<Self, Self::Error> {
        value
            .is_aligned_to(Self::MIN_ALIGN)
            .then_some(Self(value.addr() >> Self::MIN_ALIGN_SHIFT))
            .ok_or(value)
    }
}
