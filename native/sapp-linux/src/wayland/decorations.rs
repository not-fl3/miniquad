use crate::{
    wayland::{
        extensions::viewporter::{wp_viewport, wp_viewport_interface, wp_viewporter},
        *,
    },
    wl_request, wl_request_constructor,
};

pub struct Decoration {
    pub surface: *mut wl_surface,
    pub subsurface: *mut wl_subsurface,
    pub viewport: *mut wp_viewport,
}

pub struct Decorations {
    buffer: *mut wl_buffer,
    pub top_decoration: Decoration,
    pub bottom_decoration: Decoration,
    pub left_decoration: Decoration,
    pub right_decoration: Decoration,
}

unsafe fn create_decoration(
    compositor: *mut wl_compositor,
    subcompositor: *mut wl_subcompositor,
    parent: *mut wl_surface,
    buffer: *mut wl_buffer,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
) -> Decoration {
    let surface = wl_request_constructor!(
        compositor,
        WL_COMPOSITOR_CREATE_SURFACE,
        &wl_surface_interface,
    );

    let subsurface = wl_request_constructor!(
        subcompositor,
        WL_SUBCOMPOSITOR_GET_SUBSURFACE,
        &wl_subsurface_interface,
        surface,
        parent
    );

    wl_request!(subsurface, WL_SUBSURFACE_SET_POSITION, x, y);

    let viewport = wl_request_constructor!(
        GLOBALS.viewporter,
        wp_viewporter::get_viewport,
        &wp_viewport_interface,
        surface
    );

    wl_request!(viewport, wp_viewport::set_destination, w, h);
    wl_request!(surface, WL_SURFACE_ATTACH, buffer, 0, 0);
    wl_request!(surface, WL_SURFACE_COMMIT);

    Decoration {
        surface,
        subsurface,
        viewport,
    }
}

impl Decorations {
    pub const WIDTH: i32 = 2;
    pub const BAR_HEIGHT: i32 = 15;

    pub unsafe fn new(
        compositor: *mut wl_compositor,
        subcompositor: *mut wl_subcompositor,
        shm: *mut wl_shm,
        parent: *mut wl_surface,
        width: i32,
        height: i32,
    ) -> Decorations {
        let buffer = shm::create_shm_buffer(shm, 1, 1, &[200, 200, 200, 255]);

        Decorations {
            buffer,
            top_decoration: create_decoration(
                compositor,
                subcompositor,
                parent,
                buffer,
                -Self::WIDTH,
                -Self::BAR_HEIGHT,
                width + Self::WIDTH * Self::WIDTH,
                Self::BAR_HEIGHT,
            ),
            left_decoration: create_decoration(
                compositor,
                subcompositor,
                parent,
                buffer,
                -Self::WIDTH,
                -Self::BAR_HEIGHT,
                Self::WIDTH,
                height + Self::BAR_HEIGHT,
            ),
            right_decoration: create_decoration(
                compositor,
                subcompositor,
                parent,
                buffer,
                width,
                -Self::BAR_HEIGHT,
                Self::WIDTH,
                height + Self::BAR_HEIGHT,
            ),
            bottom_decoration: create_decoration(
                compositor,
                subcompositor,
                parent,
                buffer,
                -Self::WIDTH,
                height,
                width + Self::WIDTH,
                Self::WIDTH,
            ),
        }
    }

    pub unsafe fn resize(&self, width: i32, height: i32) {
        wl_request!(
            self.top_decoration.viewport,
            wp_viewport::set_destination,
            width,
            Self::BAR_HEIGHT
        );
        wl_request!(self.top_decoration.surface, WL_SURFACE_COMMIT);

        wl_request!(
            self.left_decoration.viewport,
            wp_viewport::set_destination,
            Self::WIDTH,
            height
        );
        wl_request!(self.left_decoration.surface, WL_SURFACE_COMMIT);

        wl_request!(
            self.right_decoration.subsurface,
            WL_SUBSURFACE_SET_POSITION,
            width - Self::WIDTH * 2,
            -Self::BAR_HEIGHT
        );
        wl_request!(
            self.right_decoration.viewport,
            wp_viewport::set_destination,
            Self::WIDTH,
            height
        );
        wl_request!(self.right_decoration.surface, WL_SURFACE_COMMIT);

        wl_request!(
            self.bottom_decoration.subsurface,
            WL_SUBSURFACE_SET_POSITION,
            0,
            height - Self::BAR_HEIGHT - Self::WIDTH
        );
        wl_request!(
            self.bottom_decoration.viewport,
            wp_viewport::set_destination,
            width - Self::WIDTH * 2,
            Self::WIDTH
        );
        wl_request!(self.bottom_decoration.surface, WL_SURFACE_COMMIT);
    }
}
