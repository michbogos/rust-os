use x86_64::structures::paging::{mapper, page, FrameAllocator, Mapper, Page, PageTable, PhysFrame, Size4KiB};
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::OffsetPageTable;

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
