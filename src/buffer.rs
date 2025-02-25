use std::sync::Arc;

use vulkano::{
    Validated,
    device::Device,
    buffer::{
        Buffer,
        Subbuffer,
        BufferUsage,
        BufferContents,
        BufferCreateInfo,
        AllocateBufferError,
    },
    memory::allocator::{
        AllocationCreateInfo,
        MemoryTypeFilter,
        FreeListAllocator,
        GenericMemoryAllocator,
        StandardMemoryAllocator,
    },
};


type BasicAllocator = Arc<GenericMemoryAllocator<FreeListAllocator>>;

pub fn create_basic_allocator(device: Arc<Device>) -> BasicAllocator {
    Arc::new(StandardMemoryAllocator::new_default(device.clone()))
}

pub fn create_basic_buffer<T>(
    data: T,
    allocator: BasicAllocator,
    usage: BufferUsage,
    memory_type_filter: MemoryTypeFilter,
) -> Result<Subbuffer<T>, Validated<AllocateBufferError>>
where
    T: BufferContents,
{
    Buffer::from_data(
        allocator,
        BufferCreateInfo {
            usage,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter,
            ..Default::default()
        },
        data,
    )
}

pub fn create_basic_buffer_from_iter<T, I>(
    iter: I,
    allocator: BasicAllocator,
    usage: BufferUsage,
    memory_type_filter: MemoryTypeFilter,
) -> Result<Subbuffer<[T]>, Validated<AllocateBufferError>>
where 
    T: BufferContents,
    I: IntoIterator<Item = T>,
    I::IntoIter: ExactSizeIterator,
{
    Buffer::from_iter(
        allocator,
        BufferCreateInfo {
            usage,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter,
            ..Default::default()
        },
        iter,
    )
}

pub fn empty_iter<T: From<u8>>(
    size: u32,
) -> impl IntoIterator<Item = T, IntoIter: ExactSizeIterator> {
    (0..size).map(|_| T::from(0))
}