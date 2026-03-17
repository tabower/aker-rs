#[derive(Debug, Clone, Copy)]
pub struct DtbCpu {
    /// CPU ID (hart ID for RISC-V, APIC ID for x86)
    pub id: usize,
    /// NUMA node ID (None if not NUMA system)
    pub numa_id: Option<usize>,
    /// Clock frequency in Hz
    pub freq: Option<u64>,
    /// Is this CPU enabled?
    pub enabled: bool,
}

pub(super) fn read_cpu_id(
    node: &fdt::node::FdtNode,
) -> Option<usize> {
    // Read from the reg attribute first
    if let Some(reg) = node.reg() {
        if let Some(r) = reg.into_iter().next() {
            return Some(r.starting_address as usize);
        }
    }

    // Fallback: Resolve from the node name cpu@N
    node.name
        .split('@')
        .nth(1)
        .and_then(|s| usize::from_str_radix(s, 16).ok())
}

pub(super) fn read_clock_freq(
    node: &fdt::node::FdtNode,
) -> Option<u64> {
    let prop = node.property("clock-frequency")?;
    if prop.value.len() >= 8 {
        let bytes: [u8; 8] = prop.value[..8].try_into().ok()?;
        Some(u64::from_be_bytes(bytes))
    } else if prop.value.len() >= 4 {
        let bytes: [u8; 4] = prop.value[..4].try_into().ok()?;
        Some(u32::from_be_bytes(bytes) as u64)
    } else {
        None
    }
}

pub(super) fn is_node_enabled(node: &fdt::node::FdtNode) -> bool {
    node.property("status")
        .and_then(|p| core::str::from_utf8(p.value).ok())
        .map(|s| {
            let s = s.trim_end_matches('\0');
            s == "okay" || s == "ok"
        })
        .unwrap_or(true)
}
