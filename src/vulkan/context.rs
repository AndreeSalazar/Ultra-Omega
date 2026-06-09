use ash::vk;
use ash::Entry;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use std::ffi::CString;

pub struct VulkanContext {
    pub entry: Entry,
    pub instance: ash::Instance,
    pub surface: vk::SurfaceKHR,
    pub surface_loader: ash::extensions::khr::Surface,
    pub physical_device: vk::PhysicalDevice,
    pub device: ash::Device,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
    pub graphics_queue_index: u32,

    // Swapchain y Renderizado
    pub swapchain_loader: ash::extensions::khr::Swapchain,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_image_views: Vec<vk::ImageView>,
    pub swapchain_format: vk::Format,
    pub swapchain_extent: vk::Extent2D,
    
    pub render_pass: vk::RenderPass,
    pub framebuffers: Vec<vk::Framebuffer>,
    
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,
    
    pub image_available_semaphore: vk::Semaphore,
    pub render_finished_semaphore: vk::Semaphore,
    pub in_flight_fence: vk::Fence,
    pub current_frame: usize,
}

const MAX_FRAMES_IN_FLIGHT: usize = 2;

impl VulkanContext {
    pub fn new(window: &(impl HasRawWindowHandle + HasRawDisplayHandle)) -> Self {
        env_logger::init();
        let entry = unsafe { Entry::load().expect("Failed to load Vulkan entry") };

        // 1. Instance
        let app_name = CString::new("Ultra-Omega").unwrap();
        let app_info = vk::ApplicationInfo::builder()
            .application_name(&app_name)
            .application_version(vk::make_api_version(0, 1, 2, 0))
            .engine_name(&app_name)
            .engine_version(vk::make_api_version(0, 1, 2, 0))
            .api_version(vk::make_api_version(0, 1, 2, 0));

        let instance_extensions = [
            ash::extensions::khr::Surface::name().as_ptr(),
            ash::extensions::khr::Win32Surface::name().as_ptr(), // Asumimos Windows por tu ruta
        ];
        let instance_create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&instance_extensions);

        let instance = unsafe { entry.create_instance(&instance_create_info, None).unwrap() };
        let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);

        // 2. Surface
        let surface = unsafe {
            ash_window::create_surface(&entry, &instance, window.raw_display_handle(), window.raw_window_handle(), None).unwrap()
        };

        // 3. Physical Device
        let physical_devices = unsafe { instance.enumerate_physical_devices().unwrap() };
        let physical_device = physical_devices.into_iter().find(|&device| {
            let properties = unsafe { instance.get_physical_device_properties(device) };
            properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
        }).unwrap_or(physical_devices[0]);

        // 4. Queue Families
        let queue_families = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        let graphics_queue_index = queue_families.iter().position(|q| q.queue_flags.contains(vk::QueueFlags::GRAPHICS)).unwrap() as u32;
        let present_queue_index = queue_families.iter().position(|q| {
            unsafe { surface_loader.get_physical_device_surface_support(physical_device, q.queue_index as u32, surface).unwrap() }
        }).unwrap() as u32;

        let queue_priorities = [1.0];
        let queue_create_infos = [
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(graphics_queue_index)
                .queue_priorities(&queue_priorities)
                .build(),
            vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(present_queue_index)
                .queue_priorities(&queue_priorities)
                .build(),
        ];

        // 5. Logical Device
        let device_extensions = [ash::extensions::khr::Swapchain::name().as_ptr()];
        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queue_create_infos)
            .enabled_extension_names(&device_extensions);

        let device = unsafe { instance.create_device(physical_device, &device_create_info, None).unwrap() };
        let graphics_queue = unsafe { device.get_device_queue(graphics_queue_index, 0) };
        let present_queue = unsafe { device.get_device_queue(present_queue_index, 0) };

        // 6. Swapchain
        let swapchain_loader = ash::extensions::khr::Swapchain::new(&instance, &device);
        let surface_capabilities = unsafe { instance.get_physical_device_surface_capabilities_khr(physical_device, surface).unwrap() };
        let surface_formats = unsafe { instance.get_physical_device_surface_formats_khr(physical_device, surface).unwrap() };
        let present_modes = unsafe { instance.get_physical_device_surface_present_modes_khr(physical_device, surface).unwrap() };

        let surface_format = surface_formats.into_iter().find(|f| f.format == vk::Format::B8G8R8A8_SRGB && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR).unwrap_or(surface_formats[0]);
        let present_mode = present_modes.into_iter().find(|&m| m == vk::PresentModeKHR::MAILBOX).unwrap_or(vk::PresentModeKHR::FIFO);
        
        let mut desired_image_count = surface_capabilities.min_image_count + 1;
        if surface_capabilities.max_image_count > 0 && desired_image_count > surface_capabilities.max_image_count {
            desired_image_count = surface_capabilities.max_image_count;
        }

        let extent = surface_capabilities.current_extent;
        let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(surface)
            .min_image_count(desired_image_count)
            .image_color_space(surface_format.color_space)
            .image_format(surface_format.format)
            .image_extent(extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(surface_capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(present_mode)
            .clipped(true);

        let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None).unwrap() };
        let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain).unwrap() };

        let mut swapchain_image_views = Vec::new();
        for &image in &swapchain_images {
            let create_info = vk::ImageViewCreateInfo::builder()
                .image(image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(surface_format.format)
                .subresource_range(vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                });
            let view = unsafe { device.create_image_view(&create_info, None).unwrap() };
            swapchain_image_views.push(view);
        }

        // 7. Render Pass
        let color_attachment = vk::AttachmentDescription::builder()
            .format(surface_format.format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

        let color_attachment_ref = vk::AttachmentReference::builder()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

        let subpass = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(std::slice::from_ref(&color_attachment_ref));

        let render_pass_create_info = vk::RenderPassCreateInfo::builder()
            .attachments(std::slice::from_ref(&color_attachment))
            .subpasses(std::slice::from_ref(&subpass));

        let render_pass = unsafe { device.create_render_pass(&render_pass_create_info, None).unwrap() };

        // 8. Framebuffers
        let mut framebuffers = Vec::new();
        for &image_view in &swapchain_image_views {
            let framebuffer_create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(render_pass)
                .attachments(std::slice::from_ref(&image_view))
                .width(extent.width)
                .height(extent.height)
                .layers(1);
            let framebuffer = unsafe { device.create_framebuffer(&framebuffer_create_info, None).unwrap() };
            framebuffers.push(framebuffer);
        }

        // 9. Command Pool & Buffers
        let pool_create_info = vk::CommandPoolCreateInfo::builder()
            .queue_family_index(graphics_queue_index)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);
        let command_pool = unsafe { device.create_command_pool(&pool_create_info, None).unwrap() };

        let cmd_alloc_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(MAX_FRAMES_IN_FLIGHT as u32);
        let command_buffers = unsafe { device.allocate_command_buffers(&cmd_alloc_info).unwrap() };

        // 10. Sync Objects
        let semaphore_create_info = vk::SemaphoreCreateInfo::builder();
        let fence_create_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

        let image_available_semaphore = unsafe { device.create_semaphore(&semaphore_create_info, None).unwrap() };
        let render_finished_semaphore = unsafe { device.create_semaphore(&semaphore_create_info, None).unwrap() };
        let in_flight_fence = unsafe { device.create_fence(&fence_create_info, None).unwrap() };

        Self {
            entry, instance, surface, surface_loader, physical_device, device,
            graphics_queue, present_queue, graphics_queue_index,
            swapchain_loader, swapchain, swapchain_images, swapchain_image_views,
            swapchain_format: surface_format.format, swapchain_extent: extent,
            render_pass, framebuffers, command_pool, command_buffers,
            image_available_semaphore, render_finished_semaphore, in_flight_fence, current_frame: 0,
        }
    }

    pub fn draw_frame(&mut self) {
        unsafe {
            self.device.wait_for_fences(&[self.in_flight_fence], true, u64::MAX).unwrap();
        }

        let (image_index, _) = unsafe {
            self.swapchain_loader.acquire_next_image(
                self.swapchain,
                u64::MAX,
                self.image_available_semaphore,
                vk::Fence::null(),
            ).unwrap()
        } as (u32, _);

        unsafe {
            self.device.reset_fences(&[self.in_flight_fence]).unwrap();
            self.device.reset_command_buffer(self.command_buffers[self.current_frame], vk::CommandBufferResetFlags::empty()).unwrap();
        }

        let cmd_begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe {
            self.device.begin_command_buffer(self.command_buffers[self.current_frame], &cmd_begin_info).unwrap();
        }

        let clear_values = [vk::ClearValue {
            color: vk::ClearColorValue { float32: [0.117, 0.117, 0.117, 1.0] }, // #1E1E1E (VS Code Dark)
        }];

        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.render_pass)
            .framebuffer(self.framebuffers[image_index as usize])
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.swapchain_extent,
            })
            .clear_values(&clear_values);

        unsafe {
            self.device.cmd_begin_render_pass(
                self.command_buffers[self.current_frame],
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            );
            self.device.cmd_end_render_pass(self.command_buffers[self.current_frame]);
            self.device.end_command_buffer(self.command_buffers[self.current_frame]).unwrap();
        }

        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&[self.image_available_semaphore])
            .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
            .command_buffers(&[self.command_buffers[self.current_frame]])
            .signal_semaphores(&[self.render_finished_semaphore]);

        unsafe {
            self.device.queue_submit(
                self.graphics_queue,
                &[submit_info.build()],
                self.in_flight_fence,
            ).unwrap();
        }

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&[self.render_finished_semaphore])
            .swapchains(&[self.swapchain])
            .image_indices(&[image_index]);

        unsafe {
            let _ = self.swapchain_loader.queue_present(self.present_queue, &present_info);
        }

        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;
    }
}

impl Drop for VulkanContext {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
            self.device.destroy_semaphore(self.image_available_semaphore, None);
            self.device.destroy_semaphore(self.render_finished_semaphore, None);
            self.device.destroy_fence(self.in_flight_fence, None);
            self.device.destroy_command_pool(self.command_pool, None);
            for &fb in &self.framebuffers {
                self.device.destroy_framebuffer(fb, None);
            }
            self.device.destroy_render_pass(self.render_pass, None);
            for &view in &self.swapchain_image_views {
                self.device.destroy_image_view(view, None);
            }
            self.swapchain_loader.destroy_swapchain(self.swapchain, None);
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }
    }
}
