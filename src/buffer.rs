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
