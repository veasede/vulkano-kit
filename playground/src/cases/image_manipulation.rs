use super::{CaseReturn, Utils};

use vulkano::command_buffer::CommandBufferUsage;

use vulkano_kit::*;

use command::{create_command_buffer_allocator, create_primary_builder};


pub fn case(utils: &Utils) -> CaseReturn {
    let command_buffer_allocator = create_command_buffer_allocator(utils.device.clone());

    let command_buffer_builder = create_primary_builder(
        command_buffer_allocator.clone(),
        utils.queue_family_index,
        CommandBufferUsage::OneTimeSubmit,
    )
    .expect("Failed to create command buffer builder");

    (command_buffer_builder, None)
}
