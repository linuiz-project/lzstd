use crate::Ptr;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AlignedPtr<T, const ALIGN_SHIFT: u32>(*mut T);

impl<T, const ALIGN_SHIFT: u32> TryFrom<Ptr<T>> for AlignedPtr<T, ALIGN_SHIFT> {
    type Error = Ptr<T>;

    fn try_from(ptr: Ptr<T>) -> Result<Self, Self::Error> {
        1usize
            .checked_shl(ALIGN_SHIFT)
            .and_then(|align| ptr.is_aligned_to(align).then_some(Self(*ptr)))
            .ok_or(ptr)
    }
}

impl<T, const ALIGN_SHIFT: u32> TryFrom<*mut T> for AlignedPtr<T, ALIGN_SHIFT> {
    type Error = *mut T;

    fn try_from(ptr: *mut T) -> Result<Self, Self::Error> {
        1usize
            .checked_shl(ALIGN_SHIFT)
            .and_then(|align| {
                (super::checked_canonical(ptr.addr()).is_some() && ptr.is_aligned_to(align))
                    .then_some(Self(ptr))
            })
            .ok_or(ptr)
    }
}
