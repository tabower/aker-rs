use bitflags::bitflags;
use core::marker::PhantomData;
use core::ptr::NonNull;
use core::sync::atomic::Ordering;
use core::sync::atomic::compiler_fence;

use crate::arch::cpu as arch_cpu;
use crate::arch::cpu::Context;

use crate::cpu::CpuDev;
use crate::cpu::CpuId;

use crate::cpu::numa::NId;

use crate::libs::unsafe_static::UnsafeStatic;

use super::task::Task;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct CpuFlags:u32 {
        /// If no preemption occurs during the interrupt return,
        /// this CPU should reschedule.
        const NEED_RESCHED = 1 << 0;
    }
}

/// representing per-CPU data
///
/// # Safety Invariant
///
/// All accesses require the caller to be pinned to the current CPU,
/// either by disabling preemption or by disabling interrupts.
/// This invariant is enforced at the API boundary through guard
/// types:
/// - [`PreemptGuard`]: obtained via [`preempt_disable`]
/// - [`IrqGuard`]: obtained via [`irq_save`]
///
/// The CPU ID is switched along with every task during a
/// CPU context switch.
///
/// We assume that the CPU ID is always valid at any point
/// in time.

#[repr(C, align(64))]
pub struct Cpu {
    /// Preemption nesting depth.
    ///
    /// Placed at offset 0 to allow single-instruction atomic
    /// manipulation on architectures with limited AMO addressing
    /// (e.g., RISC-V, SPARC, LoongArch).
    npreempt: u32,

    /// Current task on this CPU.
    ///
    /// In interrupt context, it should return `None` if accessed.
    task: Option<NonNull<Task>>,

    /// Architecture-specific CPU context
    context: arch_cpu::Context,

    dev: CpuDev,

    flags: CpuFlags,
}

impl Cpu {
    #[inline(always)]
    pub unsafe fn set_id(&mut self, id: CpuId) {
        self.dev.id = id;
    }

    /// The architecture itself determines where to
    /// place the per-CPU pointers.
    #[inline(always)]
    pub unsafe fn set_this_cpu(cpu_instance: *mut Cpu) {
        unsafe { arch_cpu::set_this_cpu(cpu_instance) }
    }
}

/// An immutable reference to per-CPU data.
///
/// Existence of this type proves the caller is pinned to the
/// current CPU (preemption disabled or interrupts off).
pub struct CpuRef<'a> {
    inner: &'a Cpu,
}

/// A mutable reference to per-CPU data.
pub struct CpuMut<'a> {
    inner: &'a mut Cpu,
}

// Shared accessors (available from both guards)
impl<'a> CpuRef<'a> {
    #[inline(always)]
    pub fn id(&self) -> CpuId {
        self.inner.dev.id
    }

    #[inline(always)]
    pub fn npreempt(&self) -> u32 {
        self.inner.npreempt
    }

    #[inline(always)]
    pub fn nid(&self) -> NId {
        self.inner.dev.nid
    }

    #[inline(always)]
    pub fn ticks(&self) -> u64 {
        self.inner.dev.ticks
    }

    #[inline(always)]
    pub fn flags(&self) -> CpuFlags {
        self.inner.flags
    }

    #[inline(always)]
    pub fn need_resched(&self) -> bool {
        self.inner.flags.contains(CpuFlags::NEED_RESCHED)
    }
}

impl<'a> CpuMut<'a> {
    #[inline(always)]
    pub fn as_ref(&self) -> CpuRef<'_> {
        CpuRef { inner: self.inner }
    }

    #[inline(always)]
    pub fn id(&self) -> CpuId {
        self.inner.dev.id
    }

    /// This function should only be called during CPU initialization
    /// when BOOT_CPU is triggered and should not be used at any other
    /// time. To modify the CPU ID, use `CpuMut->set_id`.
    #[inline(always)]
    pub(super) unsafe fn set_id(&mut self, id: CpuId) {
        self.inner.dev.id = id;
    }

    #[inline(always)]
    pub fn inc_ticks(&mut self) {
        self.inner.dev.ticks += 1;
    }

    #[inline(always)]
    pub fn zero_ticks(&mut self) {
        self.inner.dev.ticks = 0;
    }

    #[inline(always)]
    pub fn set_flags(&mut self, flags: CpuFlags) {
        self.inner.flags = flags;
    }
}

/// RAII guard that disables preemption.
///
/// Provides access to per-CPU data while guaranteeing
/// the current task stays on this CPU.
pub struct PreemptGuard {
    _not_send: PhantomData<*mut ()>,
}

impl PreemptGuard {
    /// Disables preemption and returns a guard.
    #[inline(always)]
    pub fn new() -> Self {
        arch_cpu::amoinc_preempt();
        compiler_fence(Ordering::SeqCst);
        Self {
            _not_send: PhantomData,
        }
    }

    #[inline(always)]
    pub fn cpu(&self) -> CpuRef<'_> {
        CpuRef {
            inner: unsafe { &*arch_cpu::this_cpu_raw() },
        }
    }

    #[inline(always)]
    pub fn cpu_mut(&mut self) -> CpuMut<'_> {
        CpuMut {
            inner: unsafe { &mut *arch_cpu::this_cpu_raw() },
        }
    }
}

impl Drop for PreemptGuard {
    #[inline(always)]
    fn drop(&mut self) {
        compiler_fence(Ordering::SeqCst);
        arch_cpu::amodec_preempt();
    }
}

/// RAII guard that disables interrupts.
///
/// Provides the same per-CPU access as [`PreemptGuard`],
/// plus access to interrupt-related fields.
pub struct IrqGuard {
    saved: usize,
    _not_send: PhantomData<*mut ()>,
}

impl IrqGuard {
    /// Saves interrupt state, disables interrupts.
    #[inline(always)]
    pub fn new() -> Self {
        let saved = arch_cpu::irq_save();
        Self {
            saved,
            _not_send: PhantomData,
        }
    }

    #[inline(always)]
    pub fn cpu(&self) -> CpuRef<'_> {
        CpuRef {
            inner: unsafe { &*arch_cpu::this_cpu_raw() },
        }
    }

    #[inline(always)]
    pub fn cpu_mut(&mut self) -> CpuMut<'_> {
        CpuMut {
            inner: unsafe { &mut *arch_cpu::this_cpu_raw() },
        }
    }

    #[inline(always)]
    pub fn intena(&self) -> bool {
        arch_cpu::irq_get()
    }
}

impl Drop for IrqGuard {
    #[inline(always)]
    fn drop(&mut self) {
        arch_cpu::irq_restore(self.saved);
    }
}

pub(super) static BOOT_CPU: UnsafeStatic<Cpu> =
    UnsafeStatic::new(Cpu {
        npreempt: 0,
        task: None,
        context: Context::new(),

        // Temporary initialization; replace with the correct CPUID
        // during actual initialization.
        dev: CpuDev::new(CpuId::new(0), NId::new(0), 0),
        flags: CpuFlags::empty(),
    });
