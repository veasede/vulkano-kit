mod common;

use vulkano_kit::instance;
use vulkano_kit::instance::InstanceOptions;


#[test]
fn get_vulkan_library() {
    match instance::get_vulkan_library() {
        Ok(library) => {
            println!("Vulkan Library: {library:#?}");
            common::set_library(library);
        }
        err => {
            err.unwrap();
        }
    }
}

#[test]
fn create_instance() {
    let library = common::get_library()
        .unwrap_or(instance::get_vulkan_library().unwrap());

    let instance = instance::create_instance(
        InstanceOptions {
            library: Some(library),
            ..Default::default()
        }
    ).unwrap();

    println!("Vulkan Library Instance: {instance:#?}");
    common::set_instance(instance);
}
