use std::sync::Arc;

use vulkano::{
    descriptor_set::layout::DescriptorSetLayout,
    device::Device,
    pipeline::{
        compute::ComputePipelineCreateInfo,
        layout::{PipelineLayoutCreateInfo, PushConstantRange},
        ComputePipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
    Validated, VulkanError,
};

pub fn crate_pipeline_layout<
    T: Into<Vec<Arc<DescriptorSetLayout>>>,
    U: Into<Vec<PushConstantRange>>,
>(
    device: Arc<Device>,
    set_layouts: T,
    push_constant_ranges: U,
) -> Result<Arc<PipelineLayout>, Validated<VulkanError>> {
    PipelineLayout::new(
        device,
        PipelineLayoutCreateInfo {
            set_layouts: set_layouts.into(),
            push_constant_ranges: push_constant_ranges.into(),
            ..Default::default()
        },
    )
}

pub fn create_compute_pipeline(
    device: Arc<Device>,
    stage: PipelineShaderStageCreateInfo,
    layout: Arc<PipelineLayout>,
) -> Result<Arc<ComputePipeline>, Validated<vulkano::VulkanError>> {
    ComputePipeline::new(
        device.clone(),
        None,
        ComputePipelineCreateInfo::stage_layout(stage, layout),
    )
}
