use std::sync::Arc;

use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::{LoadingError, VulkanLibrary};
use vulkano::{Validated, VulkanError};


// OnceLock or LazyLock ?
static VULKAN_LIBRARY: Option<Arc<VulkanLibrary>> = None;

pub fn get_vulkan_library() -> Result<Arc<VulkanLibrary>, LoadingError> {
    match &VULKAN_LIBRARY {
        Some(library) => Ok(library.clone()),
        None => VulkanLibrary::new(),
    }
}

#[derive(Debug)]
pub enum InstanceError {
    LoadingError,
    Validated(Validated<VulkanError>),
}
#[derive(Default)]
pub struct InstanceOptions {
    pub info: Option<InstanceCreateInfo>,
    pub library: Option<Arc<VulkanLibrary>>,
}

pub fn create_instance(options: InstanceOptions) -> Result<Arc<Instance>, InstanceError> {
    let library = options
        .library
        .or_else(|| get_vulkan_library().ok())
        .ok_or(InstanceError::LoadingError)?;

    let create_info = InstanceCreateInfo {
        flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
        ..options.info.unwrap_or_default()
    };

    let instance = Instance::new(library.clone(), create_info);

    match &instance {
        Ok(instance) => Ok(instance.clone()),
        Err(err) => Err(InstanceError::Validated(err.clone())),
    }
}
