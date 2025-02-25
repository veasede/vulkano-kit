use vulkano::command_buffer::CommandBufferUsage;
use vulkano_kit::command::{create_command_buffer_allocator, create_primary_builder};

use super::{CommandBuilder, Utils};

pub fn case(utils: &Utils) -> CommandBuilder {
    let command_buffer_allocator = create_command_buffer_allocator(utils.device.clone());

    create_primary_builder(
        command_buffer_allocator.clone(),
        utils.queue_family_index,
        CommandBufferUsage::OneTimeSubmit,
    )
    .expect("Failed to create command buffer builder")
}