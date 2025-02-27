mod basic_compute;
mod image_manipulation;

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
type CaseCallback = Option<Box<dyn Fn()>>;
type CaseReturn = (CommandBuilder, CaseCallback);
type CaseFn = fn(&Utils) -> CaseReturn;

static CASES: &[CaseFn] = &[
    basic_compute::case,
    image_manipulation::case,    
];


pub fn process_cases(utils: &Utils) {
    let queue = utils.queues.first().unwrap();

    let mut command_buffers: CommandVec = vec![];
    let mut callbacks: Vec<CaseCallback> = vec![];

    for case in CASES.iter() {
        let (command_buffer_builder, callback) = case(utils);

        let command_buffer = command_buffer_builder
            .build()
            .expect("Failed to build command buffer");

        command_buffers.push(command_buffer);
        callbacks.push(callback);
    }

    execute_commands_sync(utils.device.clone(), queue.clone(), command_buffers).unwrap();

    for callback in callbacks {
        if let Some(func) = callback {
            func();
        }
    }
}

pub fn wrap_callback<F>(f: F) -> CaseCallback
where
    F: Fn() + 'static,
{
    Some(Box::new(f))
}
