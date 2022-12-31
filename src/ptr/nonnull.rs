use core::ptr::NonNull;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct NonNullPtr<T>(NonNull<T>);

impl<T> TryFrom<*mut T> for NonNullPtr<T> {
    type Error = *mut T;

    fn try_from(ptr: *mut T) -> Result<Self, Self::Error> {
        match super::checked_canonical(ptr.addr()).and(NonNull::new(ptr)) {
            Some(ptr) => Ok(NonNullPtr(ptr)),
            None => Err(ptr),
        }
    }
}

impl<T> TryFrom<NonNull<T>> for NonNullPtr<T> {
    type Error = NonNull<T>;

    fn try_from(ptr: NonNull<T>) -> Result<Self, Self::Error> {
        if super::checked_canonical(ptr.addr().get()).is_some() {
            Ok(Self(ptr))
        } else {
            Err(ptr)
        }
    }
}

impl<T> const core::ops::Deref for NonNullPtr<T> {
    type Target = NonNull<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::fmt::Debug for NonNullPtr<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}
