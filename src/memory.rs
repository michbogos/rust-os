use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::{mapper, page, FrameAllocator, Mapper, Page, PageTable, PhysFrame, Size4KiB};
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::OffsetPageTable;
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static>{
    let level4_table = active_level4_table(physical_memory_offset);
    OffsetPageTable::new(level4_table, physical_memory_offset)
}

pub unsafe fn active_level4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable{
    use x86_64::registers::control::Cr3;
    let (frame, _) = Cr3::read();
    let virt = physical_memory_offset+frame.start_address().as_u64();
    return &mut *(virt.as_mut_ptr());
}

pub fn create_example_mapping(page:Page,  mapper: &mut OffsetPageTable, frame_allocator: &mut impl FrameAllocator<Size4KiB>){
        use x86_64::structures::paging::PageTableFlags as Flags;
    
        let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
        let flags = Flags::PRESENT | Flags::WRITABLE;
    
        unsafe {
            // FIXME: this is not safe, we do it only for testing
            let map_to_result = mapper.map_to(page, frame, flags, frame_allocator);
            map_to_result.expect("failed map_to").flush();
        };
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}


impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame>{
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}


unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        return frame;
    }
}

pub fn init_heap(mapper:&mut impl Mapper<Size4KiB>, frame_allocator: &mut impl FrameAllocator<Size4KiB>, heap_start:u64, heap_size:u64)->Result<(), MapToError<Size4KiB>>{
    let page_range = {
        let heap_start = VirtAddr::new(heap_start as u64);
        let heap_end = heap_start + heap_size - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = x86_64::structures::paging::PageTableFlags::PRESENT | x86_64::structures::paging::PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush()
        };
    }

    unsafe{
        ALLOCATOR.lock().init(heap_start as usize, heap_size as usize);
    }

    Ok(())
}