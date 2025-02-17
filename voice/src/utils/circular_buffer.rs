/*
    循环缓冲区实现，用于在固定大小的缓冲区中存储数据，并在缓冲区满时覆盖最旧的数据。
    缓冲区的实现需要一个固定大小的数组和两个指针（读指针和写指针）
    缓冲区需要保证线程安全
    循环缓冲区支持以下操作：
    1. 写入数据（push或者push_slice）
    2. 读取数据（可能读取最近N个样本）
    3. 清空缓冲区
    4. 获取当前缓冲区的数据长度（len）
    5. 获取缓冲区的容量（capacity）或者是否已满（is_full）
*/

use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
};

// 无锁循环缓冲区（SPSC单生产者单消费者模型）
pub(crate) struct CircularBuffer<T> {
    buffer: UnsafeCell<Vec<T>>,
    capacity: usize,
    read_index: AtomicUsize,  // 原子读指针（消费者维护）
    write_index: AtomicUsize, // 原子写指针（生产者维护）
    is_full: bool,
}

// 明确标记为线程安全（确保单生产者单消费者模型）
unsafe impl<T: Send> Sync for CircularBuffer<T> {}

impl<T> CircularBuffer<T>
where
    T: Default + Copy,
{
    /// 创建指定容量的循环缓冲区
    pub fn new(capacity: usize) -> Self {
        let mut buffer = Vec::with_capacity(capacity);
        buffer.resize_with(capacity, T::default);

        Self {
            buffer: UnsafeCell::new(buffer),
            capacity,
            read_index: AtomicUsize::new(0),
            write_index: AtomicUsize::new(0),
            is_full: false,
        }
    }

    /// 写入数据切片（生产者调用）
    pub fn push_slice(&self, data: &[T]) {
        let current_write = self.write_index.load(Ordering::Relaxed);
        let current_read = self.read_index.load(Ordering::Acquire);

        // 可用空间计算（考虑环形覆盖）
        let available = self.capacity - (current_write - current_read);
        if data.len() > available {
            // 覆盖就数据：移动读指针
            let overflow = data.len() - available;
            self.read_index
                .store(current_read + overflow, Ordering::Release);
        }

        // 获取底层缓冲区的可变引用
        let buffer = unsafe { &mut *self.buffer.get() };

        // 分段写入数据（尾部 + 头部）
        let write_pos = current_write % self.capacity;
        let first_len = (self.capacity - write_pos).min(data.len());
        let second_len = data.len() - first_len;

        // 写入第一段（尾部剩余空间）
        buffer[write_pos..write_pos + first_len].copy_from_slice(&data[..first_len]);

        // 写入第二段（头部环绕）
        if second_len > 0 {
            buffer[..second_len].copy_from_slice(&data[first_len..]);
        }

        // 更新写指针（Release 保证之前的写入操作对其他线程可见）
        self.write_index
            .store(current_write + data.len(), Ordering::Release);
    }

    /// 读取当前缓冲区的数据（消费者调用）
    /// 返回两个连续切片（可能环绕缓冲区尾部）
    pub fn slices(&self) -> (&[T], &[T]) {
        let current_write = self.write_index.load(Ordering::Acquire);
        let current_read = self.read_index.load(Ordering::Relaxed);
        let len = current_write - current_read;

        if len == 0 {
            return (&[], &[]);
        }

        let buffer = unsafe { &*self.buffer.get() };
        let read_pos = current_read % self.capacity;
        let first_len = (self.capacity - read_pos).min(len);
        let second_len = len - first_len;

        (
            &buffer[read_pos..read_pos + first_len],
            &buffer[..second_len],
        )
    }

    /// 清空缓冲区（重置指针）
    pub fn clear(&self) {
        self.read_index
            .store(self.write_index.load(Ordering::Relaxed), Ordering::Relaxed);
    }

    /// 获取当前缓冲区的数据长度
    pub fn len(&self) -> usize {
        self.write_index.load(Ordering::Acquire) - self.read_index.load(Ordering::Acquire)
    }

    /// 获取缓冲区的容量
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
