//! Window decorations(borders, titlebards, close/minimize buttons) are optional
//! Most importantly they are not implemented on Gnome.
//! Gnome suggest linking with GTK and do lots of gnome-specific things to
//! get decorations working...
//!
//! So this module is drawing some sort of a window border, just for GNOME
//! looks horrible, doesn't fit OS theme at all, but better than nothing

use crate::{
    native::linux_wayland::{
        extensions::viewporter::{wp_viewport, wp_viewport_interface, wp_viewporter},
        libwayland_client::*,
        shm, WaylandPayload,
    },
    wl_request, wl_request_constructor,
};

pub struct Decoration {
    pub surface: *mut wl_surface,
    pub subsurface: *mut wl_subsurface,
    pub viewport: *mut wp_viewport,
}

pub(crate) struct Decorations {
    buffer: *mut wl_buffer,
    pub top_decoration: Decoration,
    pub bottom_decoration: Decoration,
    pub left_decoration: Decoration,
    pub right_decoration: Decoration,
}

unsafe fn create_decoration(
    display: &mut WaylandPayload,
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
        display.client,
        compositor,
        WL_COMPOSITOR_CREATE_SURFACE,
        display.client.wl_surface_interface,
    );

    let subsurface = wl_request_constructor!(
        display.client,
        subcompositor,
        WL_SUBCOMPOSITOR_GET_SUBSURFACE,
        display.client.wl_subsurface_interface,
        surface,
        parent
    );

    wl_request!(display.client, subsurface, WL_SUBSURFACE_SET_POSITION, x, y);

    let viewport = wl_request_constructor!(
        display.client,
        display.viewporter,
        wp_viewporter::get_viewport,
        &wp_viewport_interface,
        surface
    );

    wl_request!(display.client, viewport, wp_viewport::set_destination, w, h);
    wl_request!(display.client, surface, WL_SURFACE_ATTACH, buffer, 0, 0);
    wl_request!(display.client, surface, WL_SURFACE_COMMIT);

    Decoration {
        surface,
        subsurface,
        viewport,
    }
}

impl Decorations {
    pub const WIDTH: i32 = 2;
    pub const BAR_HEIGHT: i32 = 15;

    pub(super) unsafe fn new(display: &mut WaylandPayload, width: i32, height: i32) -> Decorations {
        let buffer = shm::create_shm_buffer(
            &mut display.client,
            display.shm,
            1,
            1,
            &[200, 200, 200, 255],
        );

        Decorations {
            buffer,
            top_decoration: create_decoration(
                display,
                display.compositor,
                display.subcompositor,
                display.surface,
                buffer,
                -Self::WIDTH,
                -Self::BAR_HEIGHT,
                width + Self::WIDTH * Self::WIDTH,
                Self::BAR_HEIGHT,
            ),
            left_decoration: create_decoration(
                display,
                display.compositor,
                display.subcompositor,
                display.surface,
                buffer,
                -Self::WIDTH,
                -Self::BAR_HEIGHT,
                Self::WIDTH,
                height + Self::BAR_HEIGHT,
            ),
            right_decoration: create_decoration(
                display,
                display.compositor,
                display.subcompositor,
                display.surface,
                buffer,
                width,
                -Self::BAR_HEIGHT,
                Self::WIDTH,
                height + Self::BAR_HEIGHT,
            ),
            bottom_decoration: create_decoration(
                display,
                display.compositor,
                display.subcompositor,
                display.surface,
                buffer,
                -Self::WIDTH,
                height,
                width + Self::WIDTH,
                Self::WIDTH,
            ),
        }
    }

    pub unsafe fn resize(&self, client: &mut LibWaylandClient, width: i32, height: i32) {
        wl_request!(
            client,
            self.top_decoration.viewport,
            wp_viewport::set_destination,
            width,
            Self::BAR_HEIGHT
        );
        wl_request!(client, self.top_decoration.surface, WL_SURFACE_COMMIT);

        wl_request!(
            client,
            self.left_decoration.viewport,
            wp_viewport::set_destination,
            Self::WIDTH,
            height
        );
        wl_request!(client, self.left_decoration.surface, WL_SURFACE_COMMIT);

        wl_request!(
            client,
            self.right_decoration.subsurface,
            WL_SUBSURFACE_SET_POSITION,
            width - Self::WIDTH * 2,
            -Self::BAR_HEIGHT
        );
        wl_request!(
            client,
            self.right_decoration.viewport,
            wp_viewport::set_destination,
            Self::WIDTH,
            height
        );
        wl_request!(client, self.right_decoration.surface, WL_SURFACE_COMMIT);

        wl_request!(
            client,
            self.bottom_decoration.subsurface,
            WL_SUBSURFACE_SET_POSITION,
            0,
            height - Self::BAR_HEIGHT - Self::WIDTH
        );
        wl_request!(
            client,
            self.bottom_decoration.viewport,
            wp_viewport::set_destination,
            width - Self::WIDTH * 2,
            Self::WIDTH
        );
        wl_request!(client, self.bottom_decoration.surface, WL_SURFACE_COMMIT);
    }
}
