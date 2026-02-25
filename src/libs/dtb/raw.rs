use super::cpu::*;
use super::mem::*;
use super::numa::read_numa_id;
use fdt::Fdt;

pub(super) struct RawDtb<'a> {
    pub(super) fdt: Fdt<'a>,
}

impl<'a> RawDtb<'a> {
    // SAFETY: The caller must ensure that dtb_addr points to
    // valid DTB memory that has not been released.
    pub(super) unsafe fn new(dtb_addr: usize, size: usize) -> Self {
        let dtb_slice = unsafe {
            core::slice::from_raw_parts(dtb_addr as *const u8, size)
        };
        let fdt = Fdt::new(dtb_slice).expect("Invalid FDT blob");
        Self { fdt }
    }

    pub(super) fn for_each_mem<F>(&self, mut f: F)
    where
        F: FnMut(DtbMemRegion),
    {
        for node in self.fdt.find_all_nodes("/memory") {
            let numa_id = read_numa_id(&node);
            let hotpluggable =
                node.property("hotpluggable").is_some();

            let Some(reg) = node.reg() else { continue };

            for r in reg {
                f(DtbMemRegion {
                    base: r.starting_address as usize,
                    size: r.size.unwrap_or(0),
                    numa_id,
                    kind: DtbMemKind::Ram,
                    hotpluggable,
                });
            }
        }
    }

    pub(super) fn for_each_cpu<F>(&self, mut f: F)
    where
        F: FnMut(Cpu),
    {
        let Some(cpus_node) = self.fdt.find_node("/cpus") else {
            return;
        };

        // cpu@0, cpu@1, ...
        for node in cpus_node.children() {
            if node.name.split('@').next() != Some("cpu") {
                continue;
            }

            let Some(id) = read_cpu_id(&node) else {
                continue;
            };

            f(Cpu {
                id,
                numa_id: read_numa_id(&node),
                freq: read_clock_freq(&node),
                enabled: is_node_enabled(&node),
            });
        }
    }

    pub(super) fn cpu_count(&self) -> usize {
        let mut count = 0;
        self.for_each_cpu(|_| {
            count += 1;
        });
        count
    }

    pub(super) fn total_size(&self) -> usize {
        self.fdt.total_size()
    }
}
