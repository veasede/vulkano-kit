use std::sync::Arc;

use vulkano::device::physical::PhysicalDevice;
use vulkano::device::QueueFlags;


pub fn get_family_indices_by_flags<T: Into<Vec<QueueFlags>>>(
    physical_device: Arc<PhysicalDevice>,
    flags: T,
) -> impl Iterator<Item = u32> {
    let flags: Vec<QueueFlags> = flags.into();

    physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .filter(|(_, family)| flags.iter().all(|flag| family.queue_flags.contains(*flag)))
        .map(|(i, _)| i as u32)
        .collect::<Vec<_>>()
        .into_iter()
}
