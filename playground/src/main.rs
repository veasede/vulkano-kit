mod cases;
mod shaders;

use vulkano::{
    self,
    device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags},
};

use vulkano_kit::*;

use device::get_physical_devices;
use instance::{create_instance, get_vulkan_library};
use queue::get_family_indices_by_flags;

fn main() {
    let _library =
        get_vulkan_library().unwrap_or_else(|err| panic!("Couldn't create instance: {:?}", err));

    let instance = create_instance(Default::default())
        .unwrap_or_else(|err| panic!("Couldn't create instance: {:?}", err));

    let physical_devices = get_physical_devices(instance.clone())
        .expect("Couldn't enumerate devices")
        .collect::<Vec<_>>();

    let physical_device = physical_devices
        .get(0)
        .expect("Couldn't get first physical device")
        .clone();

    let queue_family_index =
        get_family_indices_by_flags(physical_device.clone(), [QueueFlags::GRAPHICS])
            .next()
            .expect("Failed to get next queue family index");

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("Failed to create device");

    println!("Everything succeeded!");
}
