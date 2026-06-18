use ash::vk;
use ash::khr::surface::{self, Instance as SurfaceInstance};
use ash::khr::win32_surface;
use ash::khr::swapchain::{self, Device as SwapchainDevice};
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::Window;
use crate::core::node_graph::NodeGraph;
use crate::vulkan::pipeline::GraphicsPipeline;
use crate::vulkan::renderer::{Renderer, RenderState, Viewport2D};
use crate::vulkan::text::{FontAtlas, TextPipeline};

#[allow(dead_code)]
pub struct VulkanContext {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub surface: vk::SurfaceKHR,
    pub surface_loader: SurfaceInstance,
    pub physical_device: vk::PhysicalDevice,
    pub device: ash::Device,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
    pub graphics_queue_index: u32,

    pub swapchain_loader: SwapchainDevice,
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

    pub pipeline: GraphicsPipeline,
    pub text_pipeline: Option<TextPipeline>,
    pub font_atlas: Option<FontAtlas>,
    pub renderer: Renderer,
    swapchain_dirty: bool,
}

const MAX_FRAMES_IN_FLIGHT: usize = 2;

impl VulkanContext {
    // FIX: Añadimos paréntesis para resolver la ambigüedad del trait
    pub fn new(window: &Window) -> Self {
        let entry = unsafe { ash::Entry::load().expect("Failed to load Vulkan entry") };

        // FIX: ash 0.38 usa c"..." en lugar de CString
        let app_info = vk::ApplicationInfo {
            p_application_name: c"Ultra-Omega".as_ptr(),
            application_version: vk::make_api_version(0, 1, 2, 0),
            p_engine_name: c"Ultra-Omega".as_ptr(),
            engine_version: vk::make_api_version(0, 1, 2, 0),
            api_version: vk::make_api_version(0, 1, 2, 0),
            ..Default::default()
        };

        // FIX: ash 0.38 usa ash::khr::... en lugar de ash::extensions::khr::...
        let instance_extensions = [
            surface::NAME.as_ptr(),
            win32_surface::NAME.as_ptr(),
        ];
        let instance_create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            pp_enabled_extension_names: instance_extensions.as_ptr(),
            enabled_extension_count: instance_extensions.len() as u32,
            ..Default::default()
        };

        let instance = unsafe { entry.create_instance(&instance_create_info, None).unwrap() };
        let surface_loader = SurfaceInstance::new(&entry, &instance);

        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.display_handle().unwrap().as_raw(),
                window.window_handle().unwrap().as_raw(),
                None,
            ).expect("Failed to create Vulkan surface")
        };

        let physical_devices = unsafe { instance.enumerate_physical_devices().unwrap() };
        let physical_device = physical_devices.iter().copied().find(|&device| {
            let properties = unsafe { instance.get_physical_device_properties(device) };
            properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
        }).unwrap_or(physical_devices[0]);

        let queue_families = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        let graphics_queue_index = queue_families.iter().position(|q| q.queue_flags.contains(vk::QueueFlags::GRAPHICS)).unwrap() as u32;
        
        let mut present_queue_index = 0;
        for (i, _) in queue_families.iter().enumerate() {
            let supports_present = unsafe {
                surface_loader.get_physical_device_surface_support(physical_device, i as u32, surface).unwrap()
            };
            if supports_present {
                present_queue_index = i as u32;
                break;
            }
        }

        let queue_priorities = [1.0];
        let queue_create_infos = [
            vk::DeviceQueueCreateInfo {
                queue_family_index: graphics_queue_index,
                p_queue_priorities: queue_priorities.as_ptr(),
                queue_count: 1,
                ..Default::default()
            },
            vk::DeviceQueueCreateInfo {
                queue_family_index: present_queue_index,
                p_queue_priorities: queue_priorities.as_ptr(),
                queue_count: 1,
                ..Default::default()
            },
        ];

        let device_extensions = [swapchain::NAME.as_ptr()];
        let device_features = vk::PhysicalDeviceFeatures::default();
        let device_create_info = vk::DeviceCreateInfo {
            p_queue_create_infos: queue_create_infos.as_ptr(),
            queue_create_info_count: queue_create_infos.len() as u32,
            pp_enabled_extension_names: device_extensions.as_ptr(),
            enabled_extension_count: device_extensions.len() as u32,
            p_enabled_features: &device_features,
            ..Default::default()
        };

        let device = unsafe { instance.create_device(physical_device, &device_create_info, None).unwrap() };
        let graphics_queue = unsafe { device.get_device_queue(graphics_queue_index, 0) };
        let present_queue = unsafe { device.get_device_queue(present_queue_index, 0) };

        let swapchain_loader = SwapchainDevice::new(&instance, &device);
        let surface_capabilities = unsafe { surface_loader.get_physical_device_surface_capabilities(physical_device, surface).unwrap() };
        let surface_formats = unsafe { surface_loader.get_physical_device_surface_formats(physical_device, surface).unwrap() };
        let present_modes = unsafe { surface_loader.get_physical_device_surface_present_modes(physical_device, surface).unwrap() };

        let surface_format = surface_formats.iter().copied().find(|f| f.format == vk::Format::B8G8R8A8_SRGB && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR).unwrap_or(surface_formats[0]);
        let present_mode = present_modes.into_iter().find(|&m| m == vk::PresentModeKHR::MAILBOX).unwrap_or(vk::PresentModeKHR::FIFO);
        
        let mut desired_image_count = surface_capabilities.min_image_count + 1;
        if surface_capabilities.max_image_count > 0 && desired_image_count > surface_capabilities.max_image_count {
            desired_image_count = surface_capabilities.max_image_count;
        }

        let extent = surface_capabilities.current_extent;
        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            surface,
            min_image_count: desired_image_count,
            image_color_space: surface_format.color_space,
            image_format: surface_format.format,
            image_extent: extent,
            image_array_layers: 1,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode: vk::SharingMode::EXCLUSIVE,
            pre_transform: surface_capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode,
            clipped: vk::TRUE,
            ..Default::default()
        };

        let swapchain = unsafe { swapchain_loader.create_swapchain(&swapchain_create_info, None).unwrap() };
        let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain).unwrap() };

        let mut swapchain_image_views = Vec::new();
        for &image in &swapchain_images {
            let create_info = vk::ImageViewCreateInfo {
                image,
                view_type: vk::ImageViewType::TYPE_2D,
                format: surface_format.format,
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                ..Default::default()
            };
            let view = unsafe { device.create_image_view(&create_info, None).unwrap() };
            swapchain_image_views.push(view);
        }

        let color_attachment = vk::AttachmentDescription {
            format: surface_format.format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            ..Default::default()
        };

        let color_attachment_ref = vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            ..Default::default()
        };

        let subpass = vk::SubpassDescription {
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            color_attachment_count: 1,
            p_color_attachments: &color_attachment_ref,
            ..Default::default()
        };

        let render_pass_create_info = vk::RenderPassCreateInfo {
            attachment_count: 1,
            p_attachments: &color_attachment,
            subpass_count: 1,
            p_subpasses: &subpass,
            ..Default::default()
        };

        let render_pass = unsafe { device.create_render_pass(&render_pass_create_info, None).unwrap() };

        let mut framebuffers = Vec::new();
        for &image_view in &swapchain_image_views {
            let framebuffer_create_info = vk::FramebufferCreateInfo {
                render_pass,
                attachment_count: 1,
                p_attachments: &image_view,
                width: extent.width,
                height: extent.height,
                layers: 1,
                ..Default::default()
            };
            let framebuffer = unsafe { device.create_framebuffer(&framebuffer_create_info, None).unwrap() };
            framebuffers.push(framebuffer);
        }

        let pool_create_info = vk::CommandPoolCreateInfo {
            queue_family_index: graphics_queue_index,
            flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
            ..Default::default()
        };
        let command_pool = unsafe { device.create_command_pool(&pool_create_info, None).unwrap() };

        let cmd_alloc_info = vk::CommandBufferAllocateInfo {
            command_pool,
            level: vk::CommandBufferLevel::PRIMARY,
            command_buffer_count: MAX_FRAMES_IN_FLIGHT as u32,
            ..Default::default()
        };
        let command_buffers = unsafe { device.allocate_command_buffers(&cmd_alloc_info).unwrap() };

        let semaphore_create_info = vk::SemaphoreCreateInfo::default();
        let fence_create_info = vk::FenceCreateInfo {
            flags: vk::FenceCreateFlags::SIGNALED,
            ..Default::default()
        };

        let image_available_semaphore = unsafe { device.create_semaphore(&semaphore_create_info, None).unwrap() };
        let render_finished_semaphore = unsafe { device.create_semaphore(&semaphore_create_info, None).unwrap() };
        let in_flight_fence = unsafe { device.create_fence(&fence_create_info, None).unwrap() };

        let pipeline = GraphicsPipeline::new(&device, render_pass, extent);
        let text_pipeline_obj = TextPipeline::new(&device, render_pass, extent);
        let font_atlas = FontAtlas::new(&device, &instance, physical_device, text_pipeline_obj.descriptor_set_layout);

        // Transition font atlas texture layout to SHADER_READ_ONLY_OPTIMAL
        if let Some(ref atlas) = font_atlas {
            let cmd_alloc_info = vk::CommandBufferAllocateInfo {
                command_pool,
                level: vk::CommandBufferLevel::PRIMARY,
                command_buffer_count: 1,
                ..Default::default()
            };
            let cmd_buf = unsafe { device.allocate_command_buffers(&cmd_alloc_info).unwrap() }[0];
            let begin_info = vk::CommandBufferBeginInfo {
                flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
                ..Default::default()
            };
            unsafe {
                device.begin_command_buffer(cmd_buf, &begin_info).unwrap();
                let barrier = vk::ImageMemoryBarrier {
                    old_layout: vk::ImageLayout::UNDEFINED,
                    new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                    src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    image: atlas.texture,
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    src_access_mask: vk::AccessFlags::empty(),
                    dst_access_mask: vk::AccessFlags::SHADER_READ,
                    ..Default::default()
                };
                device.cmd_pipeline_barrier(
                    cmd_buf,
                    vk::PipelineStageFlags::TOP_OF_PIPE,
                    vk::PipelineStageFlags::FRAGMENT_SHADER,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &[barrier],
                );
                device.end_command_buffer(cmd_buf).unwrap();
                let submit_info = vk::SubmitInfo {
                    command_buffer_count: 1,
                    p_command_buffers: &cmd_buf,
                    ..Default::default()
                };
                device.queue_submit(graphics_queue, &[submit_info], vk::Fence::null()).unwrap();
                device.queue_wait_idle(graphics_queue).unwrap();
                device.free_command_buffers(command_pool, &[cmd_buf]);
            }
        }

        // FIX: Pasamos instance para get_physical_device_memory_properties
        let renderer = Renderer::new(&device, &instance, physical_device);

        Self {
            entry, instance, surface, surface_loader, physical_device, device,
            graphics_queue, present_queue, graphics_queue_index,
            swapchain_loader, swapchain, swapchain_images, swapchain_image_views,
            swapchain_format: surface_format.format, swapchain_extent: extent,
            render_pass, framebuffers, command_pool, command_buffers,
            image_available_semaphore, render_finished_semaphore, in_flight_fence, current_frame: 0,
            pipeline, text_pipeline: Some(text_pipeline_obj), font_atlas,
            renderer,
            swapchain_dirty: false,
        }
    }

    pub fn mark_swapchain_dirty(&mut self) {
        self.swapchain_dirty = true;
    }

    pub fn draw_frame(&mut self, window: &Window, graph: &NodeGraph, viewport: Viewport2D, state: RenderState) {
        let size = window.inner_size();
        if size.width == 0 || size.height == 0 {
            self.swapchain_dirty = true;
            return;
        }

        if self.swapchain_dirty {
            self.recreate_swapchain(window);
        }

        unsafe {
            self.device.wait_for_fences(&[self.in_flight_fence], true, u64::MAX).unwrap();
        }

        self.renderer.update_from_graph(&self.device, graph, self.swapchain_extent, viewport, state, self.font_atlas.as_ref());

        let (image_index, suboptimal) = match unsafe {
            self.swapchain_loader.acquire_next_image(
                self.swapchain,
                u64::MAX,
                self.image_available_semaphore,
                vk::Fence::null(),
            )
        } {
            Ok(result) => result,
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                self.recreate_swapchain(window);
                return;
            }
            Err(error) => panic!("Failed to acquire swapchain image: {error:?}"),
        };

        if suboptimal {
            self.swapchain_dirty = true;
        }

        unsafe {
            self.device.reset_fences(&[self.in_flight_fence]).unwrap();
            self.device.reset_command_buffer(self.command_buffers[self.current_frame], vk::CommandBufferResetFlags::empty()).unwrap();
        }

        let cmd_begin_info = vk::CommandBufferBeginInfo {
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            ..Default::default()
        };
        unsafe {
            self.device.begin_command_buffer(self.command_buffers[self.current_frame], &cmd_begin_info).unwrap();
        }

        let clear_values = [vk::ClearValue {
            color: vk::ClearColorValue { float32: [0.086, 0.075, 0.067, 1.0] }, // #161311 warm ink
        }];

        let render_pass_begin_info = vk::RenderPassBeginInfo {
            render_pass: self.render_pass,
            framebuffer: self.framebuffers[image_index as usize],
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.swapchain_extent,
            },
            clear_value_count: 1,
            p_clear_values: clear_values.as_ptr(),
            ..Default::default()
        };

        unsafe {
            self.device.cmd_begin_render_pass(
                self.command_buffers[self.current_frame],
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            );
            
            // Dibujar el canvas Vulkan generado desde NodeGraph.
            self.renderer.record_command_buffer(
                &self.device,
                self.command_buffers[self.current_frame],
                &self.pipeline,
                self.text_pipeline.as_ref(),
                self.font_atlas.as_ref(),
            );

            self.device.cmd_end_render_pass(self.command_buffers[self.current_frame]);
            self.device.end_command_buffer(self.command_buffers[self.current_frame]).unwrap();
        }

        let wait_semaphores = [self.image_available_semaphore];
        let command_buffers_slice = [self.command_buffers[self.current_frame]];
        let signal_semaphores = [self.render_finished_semaphore];
        let wait_dst_stage_mask = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let submit_info = vk::SubmitInfo {
            wait_semaphore_count: 1,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_dst_stage_mask.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: command_buffers_slice.as_ptr(),
            signal_semaphore_count: 1,
            p_signal_semaphores: signal_semaphores.as_ptr(),
            ..Default::default()
        };

        unsafe {
            self.device.queue_submit(
                self.graphics_queue,
                &[submit_info],
                self.in_flight_fence,
            ).unwrap();
        }

        let wait_semaphores_present = [self.render_finished_semaphore];
        let swapchains = [self.swapchain];
        let image_indices = [image_index];

        let present_info = vk::PresentInfoKHR {
            wait_semaphore_count: 1,
            p_wait_semaphores: wait_semaphores_present.as_ptr(),
            swapchain_count: 1,
            p_swapchains: swapchains.as_ptr(),
            p_image_indices: image_indices.as_ptr(),
            ..Default::default()
        };

        let present_result = unsafe {
            self.swapchain_loader.queue_present(self.present_queue, &present_info)
        };

        match present_result {
            Ok(suboptimal) => {
                if suboptimal {
                    self.swapchain_dirty = true;
                }
            }
            Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => self.swapchain_dirty = true,
            Err(error) => panic!("Failed to present swapchain image: {error:?}"),
        }

        self.current_frame = (self.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;
    }

    fn recreate_swapchain(&mut self, window: &Window) {
        let size = window.inner_size();
        if size.width == 0 || size.height == 0 {
            self.swapchain_dirty = true;
            return;
        }

        unsafe {
            self.device.device_wait_idle().unwrap();
        }

        let surface_capabilities = unsafe {
            self.surface_loader
                .get_physical_device_surface_capabilities(self.physical_device, self.surface)
                .unwrap()
        };
        let surface_formats = unsafe {
            self.surface_loader
                .get_physical_device_surface_formats(self.physical_device, self.surface)
                .unwrap()
        };
        let present_modes = unsafe {
            self.surface_loader
                .get_physical_device_surface_present_modes(self.physical_device, self.surface)
                .unwrap()
        };

        let surface_format = surface_formats
            .iter()
            .copied()
            .find(|f| f.format == vk::Format::B8G8R8A8_SRGB && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR)
            .unwrap_or(surface_formats[0]);
        let present_mode = present_modes
            .into_iter()
            .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(vk::PresentModeKHR::FIFO);
        let extent = choose_swapchain_extent(surface_capabilities, window);
        let desired_image_count = desired_swapchain_image_count(surface_capabilities);

        let old_swapchain = self.swapchain;
        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            surface: self.surface,
            min_image_count: desired_image_count,
            image_color_space: surface_format.color_space,
            image_format: surface_format.format,
            image_extent: extent,
            image_array_layers: 1,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode: vk::SharingMode::EXCLUSIVE,
            pre_transform: surface_capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode,
            clipped: vk::TRUE,
            old_swapchain,
            ..Default::default()
        };

        let swapchain = unsafe {
            self.swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .unwrap()
        };
        let swapchain_images = unsafe { self.swapchain_loader.get_swapchain_images(swapchain).unwrap() };
        let swapchain_image_views = create_image_views(&self.device, &swapchain_images, surface_format.format);

        unsafe {
            self.device.destroy_pipeline(self.pipeline.pipeline, None);
            self.device.destroy_pipeline_layout(self.pipeline.pipeline_layout, None);
            for &framebuffer in &self.framebuffers {
                self.device.destroy_framebuffer(framebuffer, None);
            }
            self.device.destroy_render_pass(self.render_pass, None);
            for &view in &self.swapchain_image_views {
                self.device.destroy_image_view(view, None);
            }
            self.swapchain_loader.destroy_swapchain(old_swapchain, None);
        }

        self.swapchain = swapchain;
        self.swapchain_images = swapchain_images;
        self.swapchain_image_views = swapchain_image_views;
        self.swapchain_format = surface_format.format;
        self.swapchain_extent = extent;
        self.render_pass = create_render_pass(&self.device, self.swapchain_format);
        self.framebuffers = create_framebuffers(&self.device, self.render_pass, &self.swapchain_image_views, self.swapchain_extent);
        self.pipeline = GraphicsPipeline::new(&self.device, self.render_pass, self.swapchain_extent);
        if let Some(ref old_tp) = self.text_pipeline {
            old_tp.destroy(&self.device);
        }
        self.text_pipeline = Some(TextPipeline::new(&self.device, self.render_pass, self.swapchain_extent));
        self.swapchain_dirty = false;
    }
}

fn choose_swapchain_extent(capabilities: vk::SurfaceCapabilitiesKHR, window: &Window) -> vk::Extent2D {
    if capabilities.current_extent.width != u32::MAX {
        return capabilities.current_extent;
    }

    let size = window.inner_size();
    vk::Extent2D {
        width: size.width.clamp(capabilities.min_image_extent.width, capabilities.max_image_extent.width),
        height: size.height.clamp(capabilities.min_image_extent.height, capabilities.max_image_extent.height),
    }
}

fn desired_swapchain_image_count(capabilities: vk::SurfaceCapabilitiesKHR) -> u32 {
    let mut desired_image_count = capabilities.min_image_count + 1;
    if capabilities.max_image_count > 0 && desired_image_count > capabilities.max_image_count {
        desired_image_count = capabilities.max_image_count;
    }
    desired_image_count
}

fn create_image_views(device: &ash::Device, images: &[vk::Image], format: vk::Format) -> Vec<vk::ImageView> {
    images
        .iter()
        .map(|&image| {
            let create_info = vk::ImageViewCreateInfo {
                image,
                view_type: vk::ImageViewType::TYPE_2D,
                format,
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                ..Default::default()
            };
            unsafe { device.create_image_view(&create_info, None).unwrap() }
        })
        .collect()
}

fn create_render_pass(device: &ash::Device, format: vk::Format) -> vk::RenderPass {
    let color_attachment = vk::AttachmentDescription {
        format,
        samples: vk::SampleCountFlags::TYPE_1,
        load_op: vk::AttachmentLoadOp::CLEAR,
        store_op: vk::AttachmentStoreOp::STORE,
        stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
        stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
        initial_layout: vk::ImageLayout::UNDEFINED,
        final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        ..Default::default()
    };
    let color_attachment_ref = vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        ..Default::default()
    };
    let subpass = vk::SubpassDescription {
        pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
        color_attachment_count: 1,
        p_color_attachments: &color_attachment_ref,
        ..Default::default()
    };
    let render_pass_create_info = vk::RenderPassCreateInfo {
        attachment_count: 1,
        p_attachments: &color_attachment,
        subpass_count: 1,
        p_subpasses: &subpass,
        ..Default::default()
    };

    unsafe { device.create_render_pass(&render_pass_create_info, None).unwrap() }
}

fn create_framebuffers(
    device: &ash::Device,
    render_pass: vk::RenderPass,
    image_views: &[vk::ImageView],
    extent: vk::Extent2D,
) -> Vec<vk::Framebuffer> {
    image_views
        .iter()
        .map(|&image_view| {
            let framebuffer_create_info = vk::FramebufferCreateInfo {
                render_pass,
                attachment_count: 1,
                p_attachments: &image_view,
                width: extent.width,
                height: extent.height,
                layers: 1,
                ..Default::default()
            };
            unsafe { device.create_framebuffer(&framebuffer_create_info, None).unwrap() }
        })
        .collect()
}

impl Drop for VulkanContext {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();
            
            // Destruir pipeline y renderer
            self.device.destroy_pipeline(self.pipeline.pipeline, None);
            self.device.destroy_pipeline_layout(self.pipeline.pipeline_layout, None);
            if let Some(ref tp) = self.text_pipeline {
                tp.destroy(&self.device);
            }
            if let Some(ref atlas) = self.font_atlas {
                atlas.destroy(&self.device);
            }
            self.renderer.destroy(&self.device);

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
