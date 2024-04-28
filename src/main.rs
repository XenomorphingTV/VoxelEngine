use std::error::Error;
use std::io;
use std::sync::Arc;
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage};
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::VulkanLibrary;

#[derive(BufferContents)]
#[repr(C)]
struct TestStruct {
    a: u32,
    b: u32,
}

fn initialize_vulkan() -> Result<
    (
        std::sync::Arc<vulkano::device::Device>,
        std::sync::Arc<vulkano::device::Queue>,
    ),
    Box<dyn Error>,
> {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance =
        Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");

    let physical_devices: Vec<_> = instance.enumerate_physical_devices().unwrap().collect();
    let devices_count = physical_devices.len();

    println!("Devices: {}", devices_count);

    println!("Available devices:");
    for (i, physical_device) in physical_devices.iter().enumerate() {
        println!("({}) {}", i + 1, physical_device.properties().device_name);
    }

    println!("Enter the number of the device you want to select:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index: usize = input.trim().parse().expect("Please enter a number");
    if index > 0 && index <= devices_count {
        let selected_device = &physical_devices[index - 1];
        println!(
            "Selected device: {}",
            selected_device.properties().device_name
        );

        let queue_family_index = selected_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_queue_family_index, queue_family_properties)| {
                queue_family_properties
                    .queue_flags
                    .contains(QueueFlags::GRAPHICS)
            })
            .expect("couldn't find a graphical queue family")
            as u32;

        let selected_device = selected_device.clone();
        let (device, mut queues) = Device::new(
            selected_device,
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .expect("failed to create device");

        let queue = queues.next().unwrap();
        Ok((device, queue))
    } else {
        Err("Invalid selection".into())
    }
}

fn create_buffer(
    device: std::sync::Arc<vulkano::device::Device>,
    queue: std::sync::Arc<vulkano::device::Queue>,
) -> vulkano::buffer::Subbuffer<[u8]> {
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    let iter = (0..128).map(|_| 5u8);

    let buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::UNIFORM_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        iter,
    )
    .unwrap();

    buffer
}

fn main() {
    let init_vulkan_outcome = initialize_vulkan();
    let (device, queue) = if let Ok(output) = init_vulkan_outcome {
        (output.0, output.1)
    } else {
        eprintln!(
            "Error initializing Vulkan: {}",
            init_vulkan_outcome.unwrap_err()
        );
        return;
    };

    let create_buff_outcome = create_buffer(device, queue);
}
