/*
 * 基于链表的内存堆分配器
 */

use core::alloc::{GlobalAlloc, Layout};

use linked_list_allocator::Heap;
use spin::Mutex;

static HEAP: Mutex<Option<Heap>> = Mutex::new(None); // 全局堆实例

pub struct Allocator;

impl Allocator {
    pub unsafe fn init(offset: usize, size: usize) {
        *HEAP.lock() = Some(Heap::new(offset as *mut u8, size))
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        while let Some(ref mut heap) = *HEAP.lock() {
            match heap.allocate_first_fit(layout) {
                Err(()) => {
                    let size = heap.size();
                }
            }
        }
    }
}
