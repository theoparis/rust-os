use core::{
	ptr::NonNull,
	sync::atomic::{AtomicUsize, Ordering},
};
use once_cell::sync::Lazy;
use tracing::trace;
use virtio_drivers::{BufferDirection, Hal, PhysAddr, PAGE_SIZE};

extern "C" {
	static dma_region: u8;
}

static DMA_PADDR: Lazy<AtomicUsize> = Lazy::new(|| {
	AtomicUsize::new(unsafe { &dma_region as *const u8 as usize })
});

pub struct HalImpl;

unsafe impl Hal for HalImpl {
	fn dma_alloc(
		pages: usize,
		_direction: BufferDirection,
	) -> (PhysAddr, NonNull<u8>) {
		let paddr = DMA_PADDR.fetch_add(PAGE_SIZE * pages, Ordering::SeqCst);
		trace!("alloc DMA: paddr={:#x}, pages={}", paddr, pages);
		let vaddr = NonNull::new(paddr as _).unwrap();
		(paddr, vaddr)
	}

	unsafe fn dma_dealloc(
		paddr: PhysAddr,
		_vaddr: NonNull<u8>,
		pages: usize,
	) -> i32 {
		trace!("dealloc DMA: paddr={:#x}, pages={}", paddr, pages);
		0
	}

	unsafe fn mmio_phys_to_virt(paddr: PhysAddr, _size: usize) -> NonNull<u8> {
		NonNull::new(paddr as _).unwrap()
	}

	unsafe fn share(
		buffer: NonNull<[u8]>,
		_direction: BufferDirection,
	) -> PhysAddr {
		let vaddr = buffer.as_ptr() as *mut u8 as usize;
		// Nothing to do, as the host already has access to all memory.
		virt_to_phys(vaddr)
	}

	unsafe fn unshare(
		_paddr: PhysAddr,
		_buffer: NonNull<[u8]>,
		_direction: BufferDirection,
	) {
		// Nothing to do, as the host already has access to all memory and we didn't copy the buffer
		// anywhere else.
	}
}

fn virt_to_phys(vaddr: usize) -> PhysAddr {
	vaddr
}

use critical_section::RawRestoreState;
struct MyCriticalSection;
#[no_mangle]
unsafe fn _critical_section_1_0_acquire() -> ::critical_section::RawRestoreState
{
	<MyCriticalSection as ::critical_section::Impl>::acquire()
}
#[no_mangle]
unsafe fn _critical_section_1_0_release(
	restore_state: ::critical_section::RawRestoreState,
) {
	<MyCriticalSection as ::critical_section::Impl>::release(restore_state)
}
unsafe impl critical_section::Impl for MyCriticalSection {
	unsafe fn acquire() -> RawRestoreState {}
	unsafe fn release(_token: RawRestoreState) {}
}
