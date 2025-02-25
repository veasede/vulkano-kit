use crate::shaders;
use super::{CommandBuilder, Utils};

use vulkano::{
    self,
    buffer::BufferUsage,
    command_buffer::CommandBufferUsage,
    descriptor_set::{
        layout::{DescriptorSetLayoutBinding, DescriptorType},
        DescriptorSet, WriteDescriptorSet,
    },
    memory::allocator::MemoryTypeFilter,
    pipeline::{PipelineBindPoint, PipelineShaderStageCreateInfo},
    shader::ShaderStages,
};

use vulkano_kit::*;

use buffer::{create_basic_allocator, create_basic_buffer};
use command::{create_command_buffer_allocator, create_primary_builder};
use descriptor::{create_descriptor_set_allocator, create_descriptor_set_layout};
use pipeline::{crate_pipeline_layout, create_compute_pipeline};


pub fn case(utils: &Utils) -> CommandBuilder {
    let memory_allocator = create_basic_allocator(utils.device.clone());

    let mut data: [u32; 65536] = [0; 65536];
    data.iter_mut().enumerate().for_each(|(i, v)| {
        *v = i as u32;
    });

    let buffer = create_basic_buffer(
        data,
        memory_allocator,
        BufferUsage::STORAGE_BUFFER,
        MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
    )
    .expect("Failed to create buffer");

    let shader = shaders::cs::load(utils.device.clone()).expect("Failed to create shader module");
    let entry_point = shader.entry_point("main").expect("Failed on entry point");
    let stage = PipelineShaderStageCreateInfo::new(entry_point);

    let descriptor_set_allocator = create_descriptor_set_allocator(utils.device.clone());

    let mut buffer_binding =
        DescriptorSetLayoutBinding::descriptor_type(DescriptorType::StorageBuffer);
    buffer_binding.stages = ShaderStages::COMPUTE;

    let bindings = [buffer_binding];

    let descriptor_set_layout = create_descriptor_set_layout(utils.device.clone(), bindings)
        .expect("Failed to create descriptor set layout");

    let pipeline_layout =
        crate_pipeline_layout(utils.device.clone(), [descriptor_set_layout.clone()], [])
            .expect("Failed to create pipeline layout");

    let compute_pipeline = create_compute_pipeline(utils.device.clone(), stage, pipeline_layout.clone())
        .expect("Failed to create compute pipeline");

    let descriptor_set = DescriptorSet::new(
        descriptor_set_allocator,
        descriptor_set_layout.clone(),
        [WriteDescriptorSet::buffer(0, buffer.clone())],
        [],
    )
    .expect("Failed to create descriptor set");

    let command_buffer_allocator = create_command_buffer_allocator(utils.device.clone());
    let mut command_buffer_builder = create_primary_builder(
        command_buffer_allocator.clone(),
        utils.queue_family_index,
        CommandBufferUsage::OneTimeSubmit,
    )
    .expect("Failed to create command buffer builder");

    let work_group_counts = [1024, 1, 1];

    unsafe {
        command_buffer_builder
            .bind_pipeline_compute(compute_pipeline.clone())
            .unwrap()
            .bind_descriptor_sets(
                PipelineBindPoint::Compute,
                pipeline_layout.clone(),
                0 as u32,
                descriptor_set,
            )
            .unwrap()
            .dispatch(work_group_counts)
            .unwrap();
    }

    command_buffer_builder
}