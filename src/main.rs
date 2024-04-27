use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::VulkanLibrary;

fn increment(n: &mut isize) {
    *n += 1;
}

fn main() {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance =
        Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");

    let mut i: isize = 0;
    loop {
        let physical_device = instance
            .enumerate_physical_devices()
            .expect("Could not enumerate physical device")
            .next()
            .expect({
                "No more devices";
                break;
            });
        increment(&mut i);
    }

    println!("Devices: {i}");

    let mut length: f32 = 1.0;
    let mut width: f32 = 1.0;
    let mut height: f32 = 1.0;

    println!("Hello, world!");
}
