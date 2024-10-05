use vk::{Buffer, DeviceMemory};
use std::ptr::copy_nonoverlapping as memcpy;
use vulkanalia::prelude::v1_3::*;
use anyhow::{anyhow, Ok, Result};

use super::engine_data::EngineData;

#[derive(Clone, Debug, Default)]
pub struct AllocatedBuffer {
    pub buffer: Buffer,
    pub buffer_memory: DeviceMemory,
}

impl AllocatedBuffer {
    pub unsafe fn destroy(&mut self, device: &Device) {
        device.destroy_buffer(self.buffer, None);
        device.free_memory(self.buffer_memory, None);
    }

    pub unsafe fn create<T>(buffer_data: *const T, buffer_len: usize,
        instance: &Instance, device: &Device, data: &EngineData) -> Result<Self> 
    {
        // Create the buffer
        let buffer_info = vk::BufferCreateInfo::builder()
            .size(buffer_len as u64)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let buffer = device.create_buffer(&buffer_info, None)?;

        // Set the memory requirements
        let requirements = device.get_buffer_memory_requirements(buffer);
        let memory_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(requirements.size)
        .memory_type_index(get_memory_type_index(
            instance,
            data,
            vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE,
            requirements,
        )?);

        // Allocate the memory
        let buffer_memory = device.allocate_memory(&memory_info, None)?;

        // Bind the memory
        device.bind_buffer_memory(buffer, buffer_memory, 0)?;
        let memory = device.map_memory(
            buffer_memory,
            0,
            buffer_info.size,
            vk::MemoryMapFlags::empty(),
        )?;

        // Copy and unmap
        memcpy(buffer_data, memory.cast(), buffer_len);
        device.unmap_memory(buffer_memory);
        
        Ok(Self { buffer, buffer_memory} )
    }
}

unsafe fn get_memory_type_index(instance: &Instance, data: &EngineData,
    properties: vk::MemoryPropertyFlags, requirements: vk::MemoryRequirements, ) -> Result<u32> 
{
    let memory = instance.get_physical_device_memory_properties(data.physical_device);
    (0..memory.memory_type_count)
        .find(|i| {
            let suitable = (requirements.memory_type_bits & (1 << i)) != 0;
            let memory_type = memory.memory_types[*i as usize];
            suitable && memory_type.property_flags.contains(properties)
        })
        .ok_or_else(|| anyhow!("Failed to find suitable memory type."))
}