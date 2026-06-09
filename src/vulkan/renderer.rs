use ash::vk;
use crate::vulkan::pipeline::{GraphicsPipeline, Vertex};

pub struct Renderer {
    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
}

impl Renderer {
    // FIX: Añadimos instance para get_physical_device_memory_properties
    pub fn new(device: &ash::Device, instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> Self {
        // Definimos un rectángulo simple (2 triángulos)
        let vertices: [Vertex; 6] = [
            Vertex { pos: [-0.5, -0.5], color: [1.0, 0.0, 0.0] }, // Rojo
            Vertex { pos: [ 0.5, -0.5], color: [0.0, 1.0, 0.0] }, // Verde
            Vertex { pos: [ 0.5,  0.5], color: [0.0, 0.0, 1.0] }, // Azul
            Vertex { pos: [-0.5, -0.5], color: [1.0, 0.0, 0.0] }, // Rojo
            Vertex { pos: [ 0.5,  0.5], color: [0.0, 0.0, 1.0] }, // Azul
            Vertex { pos: [-0.5,  0.5], color: [1.0, 1.0, 0.0] }, // Amarillo
        ];

        let buffer_size = (std::mem::size_of::<Vertex>() * vertices.len()) as vk::DeviceSize;

        // Crear buffer (simplificado: en producción usarías staging buffers)
        let buffer_info = vk::BufferCreateInfo {
            size: buffer_size,
            usage: vk::BufferUsageFlags::VERTEX_BUFFER,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };

        let vertex_buffer = unsafe { device.create_buffer(&buffer_info, None).unwrap() };

        let mem_requirements = unsafe { device.get_buffer_memory_requirements(vertex_buffer) };

        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: mem_requirements.size,
            memory_type_index: find_memory_type(
                instance,
                physical_device,
                mem_requirements.memory_type_bits,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            ),
            ..Default::default()
        };

        let vertex_buffer_memory = unsafe { device.allocate_memory(&alloc_info, None).unwrap() };
        unsafe { device.bind_buffer_memory(vertex_buffer, vertex_buffer_memory, 0).unwrap() };

        // Mapear y copiar datos
        unsafe {
            let data_ptr = device.map_memory(vertex_buffer_memory, 0, buffer_size, vk::MemoryMapFlags::empty()).unwrap() as *mut Vertex;
            data_ptr.copy_from_nonoverlapping(vertices.as_ptr(), vertices.len());
            device.unmap_memory(vertex_buffer_memory);
        }

        Self {
            vertex_buffer,
            vertex_buffer_memory,
        }
    }

    pub fn record_command_buffer(
        &self,
        device: &ash::Device,
        command_buffer: vk::CommandBuffer,
        pipeline: &GraphicsPipeline,
    ) {
        unsafe {
            device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline.pipeline);
            
            let vertex_buffers = [self.vertex_buffer];
            let offsets = [0];
            device.cmd_bind_vertex_buffers(command_buffer, 0, &vertex_buffers, &offsets);
            
            device.cmd_draw(command_buffer, 6, 1, 0, 0); // ¡Dibuja el rectángulo!
        }
    }

    pub fn destroy(&self, device: &ash::Device) {
        unsafe {
            device.destroy_buffer(self.vertex_buffer, None);
            device.free_memory(self.vertex_buffer_memory, None);
        }
    }
}

fn find_memory_type(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    type_filter: u32,
    properties: vk::MemoryPropertyFlags,
) -> u32 {
    let mem_properties = unsafe { instance.get_physical_device_memory_properties(physical_device) };
    for i in 0..mem_properties.memory_type_count {
        if (type_filter & (1 << i)) != 0
            && mem_properties.memory_types[i as usize].property_flags & properties == properties
        {
            return i;
        }
    }
    panic!("Failed to find suitable memory type!");
}
