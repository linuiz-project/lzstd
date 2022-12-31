use core::marker::PhantomData;

#[inline]
const fn checked_canonical(ptr_addr: usize) -> Option<usize> {
    matches!(ptr_addr >> 47, 0 | 0x1FF).then_some(ptr_addr)
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ptr<T>(*mut T, PhantomData<T>);

impl<T> const TryFrom<usize> for Ptr<T> {
    type Error = usize;

    fn try_from(ptr_addr: usize) -> Result<Self, Self::Error> {
        match checked_canonical(ptr_addr) {
            Some(ptr_addr) => Ok(Ptr(ptr_addr as *mut T, PhantomData)),
            None => Err(ptr_addr),
        }
    }
}

impl<T> TryFrom<*const T> for Ptr<T> {
    type Error = *const T;

    fn try_from(ptr: *const T) -> Result<Self, Self::Error> {
        if checked_canonical(ptr.addr()).is_some() {
            Ok(Ptr(ptr.cast_mut(), PhantomData))
        } else {
            Err(ptr)
        }
    }
}

impl<T> core::ops::Deref for Ptr<T> {
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
