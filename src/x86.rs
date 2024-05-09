#![allow(unused_imports)]

use core::intrinsics;

// NOTE These functions are implemented using assembly because they using a custom
// calling convention which can't be implemented using a normal Rust function

// NOTE These functions are never mangled as they are not tested against compiler-rt

intrinsics! {
    #[naked]
    #[cfg(all(
        any(all(windows, target_env = "gnu"), target_os = "uefi"),
        not(feature = "no-asm")
    ))]
    pub unsafe extern "C" fn __chkstk() {
        core::arch::asm!(
            "jmp __alloca", // Jump to __alloca since fallthrough may be unreliable"
            options(noreturn, att_syntax)
        );
    }

    #[naked]
    #[cfg(all(
        any(all(windows, target_env = "gnu"), target_os = "uefi"),
        not(feature = "no-asm")
    ))]
    pub unsafe extern "C" fn _alloca() {
        // __chkstk and _alloca are the same function
        core::arch::asm!(
            "push   %ecx",
            "cmp    $0x1000,%eax",
            "lea    8(%esp),%ecx", // esp before calling this routine -> ecx
            "jb     1f",
            "2:",
            "sub    $0x1000,%ecx",
            "test   %ecx,(%ecx)",
            "sub    $0x1000,%eax",
            "cmp    $0x1000,%eax",
            "ja     2b",
            "1:",
            "sub    %eax,%ecx",
            "test   %ecx,(%ecx)",
            "lea    4(%esp),%eax",  // load pointer to the return address into eax
            "mov    %ecx,%esp",     // install the new top of stack pointer into esp
            "mov    -4(%eax),%ecx", // restore ecx
            "push   (%eax)",        // push return address onto the stack
            "sub    %esp,%eax",     // restore the original value in eax
            "ret",
            options(noreturn, att_syntax)
        );
    }
}
