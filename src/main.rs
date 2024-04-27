use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::VulkanLibrary;

fn main() {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = Instance::new(library, InstanceCreateInfo::default())
        .expect("failed to create instance");

    let mut devices_count = 0;
    for _ in instance.enumerate_physical_devices() {
        devices_count += 1;
    }

    println!("Devices: {}", devices_count);
}
