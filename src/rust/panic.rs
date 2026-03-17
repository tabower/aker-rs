use core::panic::PanicInfo;

use crate::pr_error;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        pr_error!(
            "KERNEL PANIC at {}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        );
    } else {
        pr_error!("KERNEL PANIC at unknown location");
    }

    if let Some(msg) = info.message().as_str() {
        pr_error!("  Message: {}", msg);
    } else {
        pr_error!("  Message: {}", info.message());
    }

    loop {
        core::hint::spin_loop();
    }
}
