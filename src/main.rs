use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::VulkanLibrary;
use std::io;

fn main() {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = Instance::new(library, InstanceCreateInfo::default())
        .expect("failed to create instance");

    let physical_devices: Vec<_> = instance.enumerate_physical_devices().unwrap().collect();
    let devices_count = physical_devices.len();

    println!("Devices: {}", devices_count);

    println!("Available devices:");
    for (i, physical_device) in physical_devices.iter().enumerate() {
        println!("({}) {}", i+1, physical_device.properties().device_name);
    }

    println!("Enter the number of the device you want to select:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let index: usize = input.trim().parse().expect("Please enter a number");
    if index > 0 && index <= devices_count {
        let selected_device = &physical_devices[index - 1];
        println!("Selected device: {}", selected_device.properties().device_name);
    } else {
        println!("Invalid selection");
    }
}
