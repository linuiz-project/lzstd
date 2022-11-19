#[macro_export]
macro_rules! asm_marker {
    ($marker:literal) => {
        core::arch::asm!("push r8", concat!("mov r8, ", $marker), "pop r8", options(nostack, nomem));
    };
}

#[macro_export]
macro_rules! do_once {
    ($do:block) => {{
        use core::sync::atomic::{AtomicBool, Ordering};

        static HAS_DONE: AtomicBool = AtomicBool::new(false);

        if HAS_DONE.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed).is_ok() {
            $do
        }
    }};
}
