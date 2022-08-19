// viewporter.xml

use super::super::libwayland_client::{wl_interface, wl_message};
use crate::wayland_interface;

wayland_interface!(
    wp_viewporter_interface,
    wp_viewporter,
    3,
    [
        (destroy, "", ()),
        (get_viewport, "no", (wp_viewport_interface))
    ],
    []
);

wayland_interface!(
    wp_viewport_interface,
    wp_viewport,
    3,
    [
        (destroy, "", ()),
        (set_source, "ffff", ()),
        (set_destination, "ii", ())
    ],
    []
);
