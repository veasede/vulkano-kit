use std::sync::Arc;

use thiserror::Error;

use vulkano::{
    command_buffer::{
        allocator::{
            StandardCommandBufferAllocator,
            StandardCommandBufferAllocatorCreateInfo
        },
        AutoCommandBufferBuilder,
        CommandBufferExecError,
        CommandBufferUsage,
        PrimaryAutoCommandBuffer,
    },
    device::{Device, Queue},
    sync::{self, GpuFuture},
    Validated,
    VulkanError,
};


pub fn create_standard_allocator(device: Arc<Device>) -> StandardCommandBufferAllocator {
    StandardCommandBufferAllocator::new(device, StandardCommandBufferAllocatorCreateInfo::default())
}

pub fn create_primary_builder(
    device: Arc<Device>,
    queue_family_index: u32,
    usage: CommandBufferUsage,
) -> Result<
    AutoCommandBufferBuilder<
        PrimaryAutoCommandBuffer<StandardCommandBufferAllocator>,
        StandardCommandBufferAllocator,
    >,
    Validated<VulkanError>,
> {
    AutoCommandBufferBuilder::primary(
        &create_standard_allocator(device),
        queue_family_index,
        usage,
    )
}

#[derive(Debug, Error)]
pub enum CommandExecErr {
    #[error(transparent)]
    CommandBufferExecError(#[from] CommandBufferExecError),

    #[error(transparent)]
    Validated(#[from] Validated<VulkanError>),
}

type CommandVec = Vec<Arc<PrimaryAutoCommandBuffer>>;

pub fn execute_commands_sync<T: Into<CommandVec>>(
    device: Arc<Device>,
    queue: Arc<Queue>,
    command_buffers: T,
) -> Result<(), CommandExecErr> {
    let mut command_buffers: CommandVec = command_buffers.into();

    if let Some(command_buffer) = command_buffers.pop() {
        let mut future: Box<dyn GpuFuture> =
            Box::new(sync::now(device).then_execute(queue.clone(), command_buffer)?);

        for command_buffer in command_buffers {
            future = Box::new(future.then_execute_same_queue(command_buffer)?);
        }

        future.then_signal_fence_and_flush()?.wait(None)?;
    }
    Ok(())
}
