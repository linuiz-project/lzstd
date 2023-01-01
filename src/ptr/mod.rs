mod nonnull;
pub use nonnull::*;

mod aligned;
pub use aligned::*;

#[inline]
pub(self) const fn checked_canonical(ptr_addr: usize) -> Option<usize> {
    matches!(ptr_addr >> 47, 0 | 0x1FF).then_some(ptr_addr)
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ptr<T>(*mut T);

impl<T> const From<NonNullPtr<T>> for Ptr<T> {
    fn from(value: NonNullPtr<T>) -> Self {
        Self(value.as_ptr())
    }
}

impl<T> TryFrom<*mut T> for Ptr<T> {
    type Error = *mut T;

    fn try_from(ptr: *mut T) -> Result<Self, Self::Error> {
        if checked_canonical(ptr.addr()).is_some() {
            Ok(Ptr(ptr))
        } else {
            Err(ptr)
        }
    }
}

impl<T> const core::ops::Deref for Ptr<T> {
    type Target = *mut T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::fmt::Debug for Ptr<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}
