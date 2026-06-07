use ash::vk;
use ash::{Entry, Instance, Device};
use std::ffi::CString;
use std::os::raw::c_char;
use raw_window_handle::{HasRawWindowHandle, HasRawDisplayHandle};
use ash_window;

/// Estructura principal que contiene todo el contexto de Vulkan.
/// Este es el núcleo del motor de renderizado con control total.
pub struct VulkanContext {
    pub entry: Entry,
    pub instance: Instance,
    pub surface_loader: ash::extensions::khr::Surface,
    pub surface: vk::SurfaceKHR,
    pub physical_device: vk::PhysicalDevice,
    pub device: Device,
    pub graphics_queue: vk::Queue,
    pub present_queue: vk::Queue,
}

impl VulkanContext {
    /// Inicializa el contexto completo de Vulkan: Instance, Surface, Device y Colas.
    pub fn new(window: &(impl HasRawWindowHandle + HasRawDisplayHandle)) -> Self {
        unsafe {
            let entry = Entry::load().expect("Failed to load Vulkan entry");

            // 1. Crear Instancia de Vulkan
            let app_name = CString::new("Ultra-Omega Node Editor").unwrap();
            let engine_name = CString::new("Ultra-Omega Engine v2.0").unwrap();
            
            let app_info = vk::ApplicationInfo::builder()
                .application_name(&app_name)
                .application_version(vk::make_api_version(0, 1, 0, 0))
                .engine_name(&engine_name)
                .engine_version(vk::make_api_version(0, 1, 0, 0))
                .api_version(vk::API_VERSION_1_2);

            // Extensiones requeridas para la ventana (Winit -> Surface)
            let surface_extensions = ash_window::enumerate_required_extensions(
                &window.raw_display_handle(),
            ).expect("Failed to enumerate surface extensions");

            let extension_names: Vec<*const c_char> = surface_extensions
                .iter()
                .map(|ext| *ext)
                .collect();

            // Capas de validación (opcional, útil para debug)
            let validation_layer_name = CString::new("VK_LAYER_KHRONOS_validation").unwrap();
            let layer_names = [validation_layer_name.as_ptr()];

            let create_info = vk::InstanceCreateInfo::builder()
                .application_info(&app_info)
                .enabled_extension_names(&extension_names)
                .enabled_layer_names(&layer_names);

            let instance = entry.create_instance(&create_info, None)
                .expect("Failed to create Vulkan instance");

            // 2. Crear Surface (Ventana)
            let surface_loader = ash::extensions::khr::Surface::new(&entry, &instance);
            let surface = ash_window::create_surface(
                &entry,
                &instance,
                &window.raw_display_handle(),
                &window.raw_window_handle(),
                None,
            ).expect("Failed to create Vulkan surface");

            // 3. Seleccionar Dispositivo Físico (GPU)
            let physical_devices = instance.enumerate_physical_devices()
                .expect("Failed to enumerate physical devices");
            
            let physical_device = physical_devices.into_iter()
                .next()
                .expect("No physical devices found");

            // 4. Crear Dispositivo Lógico y Colas
            let queue_family_properties = instance.get_physical_device_queue_family_properties(physical_device);
            
            // Buscar una cola que soporte gráficos y presentación
            let mut graphics_queue_family = None;
            let mut present_queue_family = None;

            for (index, family) in queue_family_properties.iter().enumerate() {
                if family.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                    graphics_queue_family = Some(index as u32);
                }

                let supports_present = surface_loader.get_physical_device_surface_support(
                    physical_device,
                    index as u32,
                    surface,
                ).expect("Failed to get surface support");

                if supports_present {
                    present_queue_family = Some(index as u32);
                }
            }

            let graphics_queue_family = graphics_queue_family.expect("No graphics queue family found");
            let present_queue_family = present_queue_family.expect("No present queue family found");

            let queue_priorities = [1.0];
            let mut queue_create_infos = vec![];

            // Si las colas son diferentes, necesitamos crear ambas
            if graphics_queue_family != present_queue_family {
                queue_create_infos.push(
                    vk::DeviceQueueCreateInfo::builder()
                        .queue_family_index(graphics_queue_family)
                        .queue_priorities(&queue_priorities)
                        .build()
                );
                queue_create_infos.push(
                    vk::DeviceQueueCreateInfo::builder()
                        .queue_family_index(present_queue_family)
                        .queue_priorities(&queue_priorities)
                        .build()
                );
            } else {
                queue_create_infos.push(
                    vk::DeviceQueueCreateInfo::builder()
                        .queue_family_index(graphics_queue_family)
                        .queue_priorities(&queue_priorities)
                        .build()
                );
            }

            let device_extension_name_raw = vk::KhrSwapchainFn::name().as_ptr();
            let device_extensions = [device_extension_name_raw];

            let device_create_info = vk::DeviceCreateInfo::builder()
                .queue_create_infos(&queue_create_infos)
                .enabled_extension_names(&device_extensions);

            let device = instance.create_device(physical_device, &device_create_info, None)
                .expect("Failed to create logical device");

            let graphics_queue = device.get_device_queue(graphics_queue_family, 0);
            let present_queue = device.get_device_queue(present_queue_family, 0);

            VulkanContext {
                entry,
                instance,
                surface_loader,
                surface,
                physical_device,
                device,
                graphics_queue,
                present_queue,
            }
        }
    }
}

impl Drop for VulkanContext {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }
    }
}
