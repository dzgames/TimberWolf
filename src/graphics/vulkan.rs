//! Vulkan graphics backend

use gfx_hal::Surface;
use gfx_backend_vulkan as gfx_backend;

#[cfg(not(target_arch = "wasm32"))]
use winit::Window;

#[cfg(all(feature = "vulkan", not(target_arch = "wasm32")))]
pub struct Backend {
    instance: gfx_backend::Instance,
}
impl Backend {

    /// create a new instance of the Vulkan graphics backend
    pub fn new (name: &str, version: u32) -> Self {
        Self {
            instance: gfx_backend::Instance::create(name, version),
        }
    }

    /// create a new window surface from a window
    pub fn create_window_surface (&self, window: &Window) -> Surface {
        self.instance.create_surface(window)
    }

}