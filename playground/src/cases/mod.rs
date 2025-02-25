mod basic_compute;

use std::sync::Arc;
use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
    device::{Device, Queue},
};
use vulkano_kit::command::{execute_commands_sync, CommandVec};

pub struct Utils {
    pub device: Arc<Device>,
    pub queues: Vec<Arc<Queue>>,
    pub queue_family_index: u32,
}

type CommandBuilder = AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>;
type CaseFn = fn(&Utils) -> CommandBuilder;

static CASES: &[CaseFn] = &[
    basic_compute::case,
];


pub fn process_cases(utils: &Utils) {
    let queue = utils.queues.first().unwrap();

    let mut command_buffers: CommandVec = vec![];

    for case in CASES.iter() {
        let command_buffer_builder = case(utils);

        let command_buffer = command_buffer_builder
            .build()
            .expect("Failed to build command buffer");

        command_buffers.push(command_buffer);
    }

    execute_commands_sync(utils.device.clone(), queue.clone(), command_buffers).unwrap();
}
