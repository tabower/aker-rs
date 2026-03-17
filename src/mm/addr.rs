use core::fmt;
use core::ops::Add;
use core::ops::AddAssign;
use core::ops::Sub;
use core::ops::SubAssign;

use crate::arch::boot::linker;

use super::align::AlignOps;
use super::page::PAGE_SHIFT;
use super::vm::consts::PT_ENTRIES;
use super::vm::consts::PT_LEVEL_BITS;
use super::vm::level::PageLevel;

/// Physical memory address
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct PhysAddr(usize);

/// Virtual memory address
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct VirtAddr(usize);

/// Physical page number
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct PhysPageNum(usize);

/// Virtual page number
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
#[repr(transparent)]
pub struct VirtPageNum(usize);

impl PhysAddr {
    #[inline(always)]
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    #[inline(always)]
    pub fn from_ptr<T>(ptr: *const T) -> Self {
        Self(ptr as usize)
    }

    #[inline(always)]
    pub const fn as_usize(&self) -> usize {
        self.0
    }

    #[inline(always)]
    pub const fn as_ptr<T>(&self) -> *const T {
        self.0 as *const T
    }

    #[inline(always)]
    pub const fn as_mut_ptr<T>(&self) -> *mut T {
        self.0 as *mut T
    }

    #[inline(always)]
    pub const fn to_ppn(&self) -> PhysPageNum {
        PhysPageNum(self.0 >> PAGE_SHIFT)
    }

    #[inline]
    pub const fn to_virt(&self) -> VirtAddr {
        VirtAddr(linker::p2v_linear(self.0))
    }
}

impl AlignOps for PhysAddr {
    #[inline]
    fn raw(&self) -> usize {
        self.0
    }
    #[inline]
    fn from_raw(v: usize) -> Self {
        Self(v)
    }
}

impl VirtAddr {
    #[inline(always)]
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    #[inline(always)]
    pub fn from_ptr<T>(ptr: *const T) -> Self {
        Self(ptr as usize)
    }

    #[inline(always)]
    pub const fn as_usize(&self) -> usize {
        self.0
    }

    #[inline(always)]
    pub const fn as_ptr<T>(&self) -> *const T {
        self.0 as *const T
    }

    #[inline(always)]
    pub const fn as_mut_ptr<T>(&self) -> *mut T {
        self.0 as *mut T
    }

    #[inline(always)]
    pub const fn to_ppn(&self) -> VirtPageNum {
        VirtPageNum(self.0 >> PAGE_SHIFT)
    }

    /// Convert to physical address
    ///
    /// ** TODO **
    ///
    /// This is currently implemented as a simple linear
    /// mapping, which is sufficient for our current
    /// needs. However, in the future, we may want to
    /// support more complex mappings (e.g., for user
    /// space, or high vaddr space).
    #[inline(always)]
    pub const fn to_phys(&self) -> PhysAddr {
        PhysAddr(linker::v2p_linear(self.0))
    }

    #[inline]
    pub const fn is_kernel(&self) -> bool {
        self.0 >= linker::vma_base()
    }

    #[inline]
    pub const fn is_user(&self) -> bool {
        !self.is_kernel()
    }
}

impl AlignOps for VirtAddr {
    #[inline]
    fn raw(&self) -> usize {
        self.0
    }
    #[inline]
    fn from_raw(v: usize) -> Self {
        Self(v)
    }
}

impl PhysPageNum {
    #[inline(always)]
    pub const fn new(ppn: usize) -> Self {
        Self(ppn)
    }

    #[inline(always)]
    pub const fn as_usize(&self) -> usize {
        self.0
    }

    #[inline(always)]
    pub const fn to_addr(&self) -> PhysAddr {
        PhysAddr(self.0 << PAGE_SHIFT)
    }

    #[inline(always)]
    pub const fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

impl VirtPageNum {
    #[inline(always)]
    pub const fn new(vpn: usize) -> Self {
        Self(vpn)
    }

    #[inline(always)]
    pub const fn as_usize(&self) -> usize {
        self.0
    }

    #[inline(always)]
    pub const fn to_addr(&self) -> VirtAddr {
        VirtAddr(self.0 << PAGE_SHIFT)
    }

    #[inline(always)]
    pub const fn next(&self) -> Self {
        Self(self.0 + 1)
    }

    /// Offset within the range covered by one entry at the given
    /// level
    #[inline]
    pub const fn offset_within(&self, level: PageLevel) -> usize {
        self.0 & (level.pages_per_entry() - 1)
    }

    ///  RiscV  SV39 VPN (27 bits):  [vpn2 | vpn1 | vpn0]
    ///                        9bit   9bit   9bit
    /// PTE (level 0):  shift  0, mask 0x1FF → bits [8:0]
    /// PMD (level 1):  shift  9, mask 0x1FF → bits [17:9]
    /// PUD (level 2):  shift 18, mask 0x1FF → bits [26:18]
    #[inline]
    pub const fn level_index(&self, level: PageLevel) -> usize {
        (self.0 >> (level.as_usize() * PT_LEVEL_BITS))
            & (PT_ENTRIES - 1)
    }
}

// Conversions between Addr and PageNum

impl From<PhysAddr> for PhysPageNum {
    #[inline]
    fn from(pa: PhysAddr) -> Self {
        pa.to_ppn()
    }
}

impl From<PhysPageNum> for PhysAddr {
    #[inline]
    fn from(ppn: PhysPageNum) -> Self {
        ppn.to_addr()
    }
}

impl From<VirtAddr> for VirtPageNum {
    #[inline]
    fn from(va: VirtAddr) -> Self {
        VirtPageNum(va.0 >> PAGE_SHIFT)
    }
}

impl From<VirtPageNum> for VirtAddr {
    #[inline]
    fn from(vpn: VirtPageNum) -> Self {
        vpn.to_addr()
    }
}

macro_rules! impl_addr_traits {
    ($($t:ty),*) => { $(

        impl fmt::Debug for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, concat!(stringify!($t), "({:#x})"), self.0)
            }
        }

        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }

        impl fmt::LowerHex for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::LowerHex::fmt(&self.0, f)
            }
        }

        impl fmt::UpperHex for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::UpperHex::fmt(&self.0, f)
            }
        }

        // Self + usize
        impl Add<usize> for $t {
            type Output = Self;
            #[inline]
            fn add(self, rhs: usize) -> Self { Self(self.0 + rhs) }
        }

        // Self - usize
        impl Sub<usize> for $t {
            type Output = Self;
            #[inline]
            fn sub(self, rhs: usize) -> Self { Self(self.0 - rhs) }
        }

        // Self - Self = usize (distance)
        impl Sub for $t {
            type Output = usize;
            #[inline]
            fn sub(self, rhs: Self) -> usize { self.0 - rhs.0 }
        }

        impl AddAssign<usize> for $t {
            #[inline]
            fn add_assign(&mut self, rhs: usize) { self.0 += rhs; }
        }

        impl SubAssign<usize> for $t {
            #[inline]
            fn sub_assign(&mut self, rhs: usize) { self.0 -= rhs; }
        }

        // From/Into usize
        impl From<usize> for $t {
            #[inline]
            fn from(v: usize) -> Self { Self(v) }
        }

        impl From<$t> for usize {
            #[inline]
            fn from(v: $t) -> usize { v.0 }
        }

        // Compare with usize
        impl PartialEq<usize> for $t {
            #[inline]
            fn eq(&self, other: &usize) -> bool { self.0 == *other }
        }

        impl PartialOrd<usize> for $t {
            #[inline]
            fn partial_cmp(&self, other: &usize) -> Option<core::cmp::Ordering> {
                self.0.partial_cmp(other)
            }
        }

    )* };
}

impl_addr_traits!(PhysAddr, VirtAddr, PhysPageNum, VirtPageNum);
