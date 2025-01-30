use std::sync::Arc;

use vulkano::device::physical::PhysicalDevice;
use vulkano::instance::Instance;
use vulkano::VulkanError;


pub fn get_physical_devices(
    instance: Arc<Instance>,
) -> Result<impl ExactSizeIterator<Item = Arc<PhysicalDevice>>, VulkanError> {
    let physical_devices = instance.enumerate_physical_devices();

    if let Err(err) = physical_devices {
        return Err(err);
    }

    Ok(physical_devices.unwrap())
}
