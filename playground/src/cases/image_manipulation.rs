use super::{wrap_callback, CaseReturn, Utils};

use vulkano::{
    buffer::BufferUsage,
    command_buffer::{ClearColorImageInfo, CommandBufferUsage, CopyImageToBufferInfo},
    format::{ClearColorValue, Format},
    image::ImageUsage,
    memory::allocator::MemoryTypeFilter,
};

use vulkano_kit::{
    buffer::{create_basic_allocator, create_basic_buffer_from_iter, empty_iter},
    command::{create_command_buffer_allocator, create_primary_builder},
    image::create_2d_image,
};

use ::image::{ImageBuffer, Rgba};


pub fn case(utils: &Utils) -> CaseReturn {
    let allocator = create_basic_allocator(utils.device.clone());

    let image = create_2d_image(
        allocator.clone(),
        (1024, 1024),
        Format::R8G8B8A8_UNORM,
        ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC,
    )
    .unwrap();

    let _empty = empty_iter::<u8>(1024 * 1024 * 4);
    let buffer = create_basic_buffer_from_iter(
        _empty,
        allocator.clone(),
        BufferUsage::TRANSFER_DST,
        MemoryTypeFilter::PREFER_HOST | MemoryTypeFilter::HOST_RANDOM_ACCESS,
    )
    .unwrap();

    let command_buffer_allocator = create_command_buffer_allocator(utils.device.clone());

    let mut command_buffer_builder = create_primary_builder(
        command_buffer_allocator.clone(),
        utils.queue_family_index,
        CommandBufferUsage::OneTimeSubmit,
    )
    .expect("Failed to create command buffer builder");

    command_buffer_builder
        .clear_color_image(ClearColorImageInfo {
            clear_value: ClearColorValue::Float([0.0, 0.0, 1.0, 1.0]),
            ..ClearColorImageInfo::image(image.clone())
        })
        .unwrap()
        .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
            image.clone(),
            buffer.clone(),
        ))
        .unwrap();

    let callback = move || {
        let image_content = buffer.read().unwrap();
        let res = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &image_content[..]).unwrap();
        res.save("out/image.png").unwrap();
    };

    (command_buffer_builder, wrap_callback(callback))
}
