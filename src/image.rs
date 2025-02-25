use std::sync::Arc;

use vulkano::format::Format;
use vulkano::image::{Image, ImageCreateInfo, ImageType, ImageUsage};
use vulkano::{
    image::AllocateImageError,
    memory::allocator::{AllocationCreateInfo, MemoryAllocator, MemoryTypeFilter},
    Validated,
};

pub fn create_2d_image(
    allocator: Arc<dyn MemoryAllocator>,
    (width, height): (u32, u32),
    format: Format,
    usage: ImageUsage,
) -> Result<Arc<Image>, Validated<AllocateImageError>> {
    Image::new(
        allocator,
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            extent: [width, height, 1],
            format,
            usage,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    )
}
