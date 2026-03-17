use crate::libs::endian::read_be_u32_at;

use super::cpu::DtbCpu;
use super::mem::DtbMemRegion;
use super::raw;

pub struct Dtb<'a> {
    raw: raw::RawDtb<'a>,
}

impl<'a> Dtb<'a> {
    const MAGIC: u32 = 0xd00dfeed;
    const MAGIC_OFFSET: usize = 0;
    const SIZE_OFFSET: usize = 4;

    pub fn new(dtb_addr: usize) -> Option<Self> {
        // check magic
        let magic = read_be_u32_at(dtb_addr, Self::MAGIC_OFFSET);
        if magic != Self::MAGIC {
            return None;
        }

        // detected size
        let size = read_be_u32_at(dtb_addr, Self::SIZE_OFFSET);
        if size == 0 {
            return None;
        }

        let raw =
            unsafe { raw::RawDtb::new(dtb_addr, size as usize) };
        Some(Self { raw })
    }

    #[inline(always)]
    pub fn for_each_mem<F>(&self, f: F)
    where
        F: FnMut(DtbMemRegion),
    {
        self.raw.for_each_mem(f);
    }

    #[inline(always)]
    pub fn for_each_cpu<F>(&self, f: F)
    where
        F: FnMut(DtbCpu),
    {
        self.raw.for_each_cpu(f);
    }

    #[inline(always)]
    pub fn cpu_count(&self) -> usize {
        self.raw.cpu_count()
    }

    #[inline(always)]
    pub fn total_size(&self) -> usize {
        self.raw.total_size()
    }
}
