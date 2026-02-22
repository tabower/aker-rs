/// Responsible for checking the number of preemptions.
/// When the preemption count is 0 and the scheduling timing is
/// appropriate (e.g., when it is necessary to switch to run another
/// task), it will enter the scheduler for rescheduling.
///
/// Located on the interrupt return path; the environment must be in
/// an interrupt-disabled state. Called when returning from a specific
/// architecture interrupt.
#[unsafe(no_mangle)]
pub extern "C" fn preempt_schedule_irq() {}
