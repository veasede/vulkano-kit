pub mod logger;
mod playground;

use std::sync::{Arc, LazyLock, Mutex, MutexGuard};

use vulkano::instance::Instance;
use vulkano::VulkanLibrary;

#[derive(Debug, Default)]
pub struct Common {
    pub library: Option<Arc<VulkanLibrary>>,
    pub instance: Option<Arc<Instance>>,
}

static _COMMON: LazyLock<Mutex<Common>> = LazyLock::new(|| {
    Mutex::new(Common {
        ..Default::default()
    })
});

fn common<'a>() -> MutexGuard<'a, Common> {
    _COMMON.lock().unwrap()
}

pub fn set_library<'a>(lib: Arc<VulkanLibrary>) {
    common::<'a>().library = Some(lib);
}
pub fn set_instance<'a>(ins: Arc<Instance>) {
    common::<'a>().instance = Some(ins);
}

pub fn get_library<'a>() -> Option<Arc<VulkanLibrary>> {
    common().library.clone()
}
pub fn get_instance<'a>() -> Option<Arc<Instance>> {
    common().instance.clone()
}
