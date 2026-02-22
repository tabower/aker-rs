use super::cpu;
use crate::config::CPUMASK_BITMAP_LEN;
use crate::config::MAX_CPUS;

#[derive(Debug, Clone, Copy)]
pub struct CpuMask {
    bits: [usize; CPUMASK_BITMAP_LEN],
}

impl CpuMask {
    pub const fn new() -> Self {
        CpuMask {
            bits: [0; CPUMASK_BITMAP_LEN],
        }
    }

    #[inline]
    fn index_and_bit(cid: cpu::CpuId) -> (usize, usize) {
        let id = cid.get();
        debug_assert!(
            id < MAX_CPUS,
            "CpuId {id} exceeds MAX_CPUS {MAX_CPUS}"
        );
        (id / usize::BITS as usize, id % usize::BITS as usize)
    }

    pub fn test(&self, cid: cpu::CpuId) -> bool {
        let (idx, bit) = Self::index_and_bit(cid);
        (self.bits[idx] & (1 << bit)) != 0
    }

    pub fn set(&mut self, cid: cpu::CpuId) {
        let (idx, bit) = Self::index_and_bit(cid);
        self.bits[idx] |= 1 << bit;
    }

    pub fn clear(&mut self, cid: cpu::CpuId) {
        let (idx, bit) = Self::index_and_bit(cid);
        self.bits[idx] &= !(1 << bit);
    }

    pub fn clear_all(&mut self) {
        self.bits = [0; CPUMASK_BITMAP_LEN];
    }

    pub fn is_empty(&self) -> bool {
        self.bits.iter().all(|&w| w == 0)
    }

    pub fn count(&self) -> usize {
        self.bits.iter().map(|w| w.count_ones() as usize).sum()
    }

    pub fn iter(&self) -> CpuMaskIter<'_> {
        CpuMaskIter {
            mask: self,
            word_idx: 0,
            remaining: self.bits[0],
            nr_cpus: unsafe { cpu::get_nr_cpus() },
        }
    }

    pub fn iter_all(&self) -> CpuMaskIter<'_> {
        CpuMaskIter {
            mask: self,
            word_idx: 0,
            remaining: self.bits[0],
            nr_cpus: MAX_CPUS,
        }
    }
}
pub struct CpuMaskIter<'a> {
    mask: &'a CpuMask,
    word_idx: usize,
    remaining: usize,
    nr_cpus: usize,
}

impl<'a> Iterator for CpuMaskIter<'a> {
    type Item = cpu::CpuId;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Skip words containing only zeros
            while self.remaining == 0 {
                self.word_idx += 1;
                if self.word_idx >= CPUMASK_BITMAP_LEN {
                    return None;
                }
                self.remaining = self.mask.bits[self.word_idx];
            }

            let bit = self.remaining.trailing_zeros() as usize;
            let id = self.word_idx * usize::BITS as usize + bit;

            // Clear this bit, move to next
            self.remaining &= self.remaining - 1;

            // Exceed actual CPU count, stop iteration
            if id >= self.nr_cpus {
                return None;
            }

            return Some(cpu::CpuId::new(id));
        }
    }
}
