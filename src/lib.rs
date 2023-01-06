#![no_std]
#![feature(
    let_chains,                         // #53667 <https://github.com/rust-lang/rust/issues/53667>
    if_let_guard,                       // #51114 <https://github.com/rust-lang/rust/issues/51114>
    extern_types,                       // #43467 <https://github.com/rust-lang/rust/issues/43467>
    strict_provenance,                  // #95228 <https://github.com/rust-lang/rust/issues/95228>
    pointer_is_aligned,                 // #96284 <https://github.com/rust-lang/rust/issues/96284>
    ptr_as_uninit,                      // #75402 <https://github.com/rust-lang/rust/issues/75402>
    const_option,
    const_option_ext,
    const_bool_to_option,
    const_try,
    const_trait_impl,
    const_ptr_as_ref,
    const_mut_refs,
    const_nonnull_new,
)]

mod addr;
pub use addr::*;

mod macros;

pub mod mem;

use core::num::NonZeroUsize;

pub struct ReadOnly;
pub struct WriteOnly;
pub struct ReadWrite;

#[cfg(target_arch = "x86_64")]
pub const PAGE_ALIGN_SHIFT: u32 = 12;
#[cfg(target_arch = "x86_64")]
const PAGE_ALIGN_MASK: usize = 1usize
    .checked_shl(PAGE_ALIGN_SHIFT)
    .unwrap_or(0)
    .wrapping_sub(1);

pub const KIBIBYTE: u64 = 0x400; // 1024
pub const MIBIBYTE: u64 = KIBIBYTE * KIBIBYTE;
pub const GIBIBYTE: u64 = MIBIBYTE * MIBIBYTE;
pub const PT_L4_ENTRY_MEM: u64 = 1 << 9 << 9 << 9 << 12;

#[inline]
pub const fn to_kibibytes(value: u64) -> u64 {
    value / KIBIBYTE
}

#[inline]
pub const fn to_mibibytes(value: u64) -> u64 {
    value / MIBIBYTE
}

#[inline]
pub const fn align_up(value: usize, alignment: NonZeroUsize) -> usize {
    let alignment_mask = alignment.get() - 1;
    if value & alignment_mask == 0 {
        value
    } else {
        (value | alignment_mask) + 1
    }
}

#[inline]
pub const fn align_up_div(value: usize, alignment: NonZeroUsize) -> usize {
    ((value + alignment.get()) - 1) / alignment.get()
}

#[inline]
pub const fn align_down(value: usize, alignment: NonZeroUsize) -> usize {
    value & !(alignment.get() - 1)
}

#[inline]
pub const fn align_down_div(value: usize, alignment: NonZeroUsize) -> usize {
    align_down(value, alignment) / alignment.get()
}

extern "C" {
    pub type LinkerSymbol;
}

impl LinkerSymbol {
    #[inline]
    pub fn as_ptr<T>(&'static self) -> *const T {
        self as *const _ as *const T
    }
}

pub struct IndexRing {
    current: usize,
    max: usize,
}

impl IndexRing {
    pub fn new(max: usize) -> Self {
        Self { current: 0, max }
    }

    pub fn index(&self) -> usize {
        self.current
    }

    pub fn increment(&mut self) {
        self.current = self.next_index();
    }

    pub fn next_index(&self) -> usize {
        (self.current + 1) % self.max
    }
}

impl core::fmt::Debug for IndexRing {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_tuple("Index Ring")
            .field(&format_args!("{}/{}", self.current, self.max - 1))
            .finish()
    }
}
