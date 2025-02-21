use std::sync::Arc;

use vulkano::{
    descriptor_set::{
        allocator::{DescriptorSetAllocator, StandardDescriptorSetAllocator},
        layout::{
            DescriptorSetLayout,
            DescriptorSetLayoutBinding,
            DescriptorSetLayoutCreateInfo,
        },
    },
    device::Device,
    Validated,
};


pub fn create_descriptor_set_allocator(device: Arc<Device>) -> Arc<dyn DescriptorSetAllocator> {
    Arc::new(StandardDescriptorSetAllocator::new(device.clone(), Default::default()))
}

type BindingVec = Vec<DescriptorSetLayoutBinding>;

pub fn create_layout<T: Into<BindingVec>>(
    device: Arc<Device>,
    bindings: T,
) -> Result<Arc<DescriptorSetLayout>, Validated<vulkano::VulkanError>> {
    let bindings: BindingVec = bindings.into();

    DescriptorSetLayout::new(
        device,
        DescriptorSetLayoutCreateInfo {
            bindings: bindings
                .into_iter()
                .enumerate()
                .map(|(i, v)| (i as u32, v))
                .collect(),
            ..Default::default()
        },
    )
}
