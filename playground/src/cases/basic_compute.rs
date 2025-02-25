use crate::shaders;
use super::{wrap_callback, CaseReturn, Utils};

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

use vulkano_kit::{buffer::create_basic_buffer_from_iter, *};

use buffer::create_basic_allocator;
use command::{create_command_buffer_allocator, create_primary_builder};
use descriptor::{create_descriptor_set_allocator, create_descriptor_set_layout};
use pipeline::{crate_pipeline_layout, create_compute_pipeline};


pub fn case(utils: &Utils) -> CaseReturn {
    let memory_allocator = create_basic_allocator(utils.device.clone());

    let data = 0..65536u32;
    let buffer = create_basic_buffer_from_iter(
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

    let callback = move || {
        let content = buffer.read().unwrap();

        for (n, val) in content.iter().enumerate() {
            assert_eq!(*val, n as u32 * 12);
        }
    };

    (command_buffer_builder, wrap_callback(callback))
}