use super::PhysicalAddress;

mod buddy;

/// 帧数量，保证与无符号整型内存布局保持一致，提供类型安全
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct FrameCount(usize);

impl FrameCount {
    pub fn new(count: usize) -> Self {
        Self(count)
    }

    pub fn data(&self) -> usize {
        self.0
    }
}

/// 记录内存使用情况，包含已用帧数和总帧数
#[derive(Debug)]
pub struct FrameUsage {
    used: FrameCount,
    total: FrameCount,
}

impl FrameUsage {
    pub fn new(used: FrameCount, total: FrameCount) -> Self {
        Self { used, total }
    }

    pub fn used(&self) -> FrameCount {
        self.used
    }

    // 构造方法及访问器
    pub fn free(&self) -> FrameCount {
        FrameCount(self.total.0 - self.used.0)
    }

    pub fn total(&self) -> FrameCount {
        self.total
    }
}

/// 定义内存帧分配器
pub trait FrameAllocator {
    // 保证 count 必须为非零值
    // 返回地址必须对齐到帧大小
    unsafe fn allocate(&mut self, count: FrameCount) -> Option<PhysicalAddress>;
    unsafe fn free(&mut self, address: PhysicalAddress, count: FrameCount);
    unsafe fn allocate_one(&mut self) -> Option<PhysicalAddress> {
        self.allocate(FrameCount::new(1))
    }
    unsafe fn free_one(&mut self, address: PhysicalAddress) {
        self.free(address, FrameCount::new(1));
    }
    unsafe fn usage(&self) -> FrameUsage;
}

impl<T> FrameAllocator for &mut T
where
    T: FrameAllocator,
{
    unsafe fn allocate(&mut self, count: FrameCount) -> Option<PhysicalAddress> {
        T::allocate(self, count)
    }

    unsafe fn free(&mut self, address: PhysicalAddress, count: FrameCount) {
        T::free(self, address, count);
    }
    unsafe fn allocate_one(&mut self) -> Option<PhysicalAddress> {
        T::allocate_one(self)
    }
    unsafe fn free_one(&mut self, address: PhysicalAddress) {
        T::free_one(self, address);
    }
    unsafe fn usage(&self) -> FrameUsage {
        T::usage(self)
    }
}
