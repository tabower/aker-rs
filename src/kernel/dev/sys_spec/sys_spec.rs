use crate::mm::addr::PhysAddr;
use crate::mm::region::MemRegion;

/// A trait for querying static system hardware
/// specifications.
///
/// This trait provides a uniform interface to retrieve
/// fundamental hardware information such as the number of
/// CPUs and available physical memory regions.
/// It is typically implemented by platform-specific
/// discovery mechanisms (e.g., device tree, ACPI, or
/// firmware tables), which often require a base
/// physical address to parse hardware description data.
pub trait SysSpec {
    /// The base physical address where the hardware
    /// description data is located.
    ///
    /// For example, on systems using a flattened device
    /// tree (FDT), this would be the physical address
    /// of the FDT blob.
    const ADDR: PhysAddr;

    /// Set address
    /// (e.g., device tree may require detection
    /// of corresponding magic, which may fail)
    fn set_addr(&self, addr: PhysAddr) -> bool;

    /// Returns an iterator over the logical CPU
    /// indices present in the system.
    fn cpus(&self) -> impl Iterator<Item = usize>;

    /// Returns an iterator over the physical
    /// memory regions available to the system.
    fn mem_regions(&self) -> impl Iterator<Item = MemRegion>;
}
