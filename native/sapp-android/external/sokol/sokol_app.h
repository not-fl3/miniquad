#ifndef SOKOL_APP_INCLUDED

/*
    sokol_app.h -- cross-platform application wrapper

    Project URL: https://github.com/floooh/sokol

    Do this:
        #define SOKOL_IMPL
    before you include this file in *one* C or C++ file to create the
    implementation.

    Optionally provide the following defines with your own implementations:

    SOKOL_ASSERT(c)     - your own assert macro (default: assert(c))
    SOKOL_LOG(msg)      - your own logging function (default: puts(msg))
    SOKOL_UNREACHABLE() - a guard macro for unreachable code (default: assert(false))
    SOKOL_ABORT()       - called after an unrecoverable error (default: abort())
    SOKOL_WIN32_FORCE_MAIN  - define this on Win32 to use a main() entry point instead of WinMain
    SOKOL_NO_ENTRY      - define this if sokol_app.h shouldn't "hijack" the main() function
    SOKOL_API_DECL      - public function declaration prefix (default: extern)
    SOKOL_API_IMPL      - public function implementation prefix (default: -)
    SOKOL_CALLOC        - your own calloc function (default: calloc(n, s))
    SOKOL_FREE          - your own free function (default: free(p))

    Optionally define the following to force debug checks and validations
    even in release mode:

    SOKOL_DEBUG         - by default this is defined if _DEBUG is defined

    If sokol_app.h is compiled as a DLL, define the following before
    including the declaration or implementation:

    SOKOL_DLL

    On Windows, SOKOL_DLL will define SOKOL_API_DECL as __declspec(dllexport)
    or __declspec(dllimport) as needed.

    Portions of the Windows and Linux GL initialization and event code have been
    taken from GLFW (http://www.glfw.org/)

    iOS onscreen keyboard support 'inspired' by libgdx.

    If you use sokol_app.h together with sokol_gfx.h, include both headers
    in the implementation source file, and include sokol_app.h before
    sokol_gfx.h since sokol_app.h will also include the required 3D-API
    headers.

    On Windows, a minimal 'GL header' and function loader is integrated which
    contains just enough of GL for sokol_gfx.h. If you want to use your own
    GL header-generator/loader instead, define SOKOL_WIN32_NO_GL_LOADER
    before including the implementation part of sokol_app.h.

    For example code, see https://github.com/floooh/sokol-samples/tree/master/sapp

    FEATURE OVERVIEW
    ================
    sokol_app.h provides a minimalistic cross-platform API which
    implements the 'application-wrapper' parts of a 3D application:

    - a common application entry function
    - creates a window and 3D-API context/device with a 'default framebuffer'
    - makes the rendered frame visible
    - provides keyboard-, mouse- and low-level touch-events
    - platforms: MacOS, iOS, HTML5, Win32, Linux, Android (RaspberryPi)
    - 3D-APIs: Metal, D3D11, GL3.2, GLES2, GLES3, WebGL, WebGL2

    FEATURE/PLATFORM MATRIX
    =======================
                        | Windows | macOS | Linux |  iOS  | Android | Raspi | HTML5
    --------------------+---------+-------+-------+-------+---------+-------+-------
    gl 3.x              | YES     | YES   | YES   | ---   | ---     | ---   | ---
    gles2/webgl         | ---     | ---   | ---   | YES   | YES     | TODO  | YES
    gles3/webgl2        | ---     | ---   | ---   | YES   | YES     | ---   | YES
    metal               | ---     | YES   | ---   | YES   | ---     | ---   | ---
    d3d11               | YES     | ---   | ---   | ---   | ---     | ---   | ---
    KEY_DOWN            | YES     | YES   | YES   | SOME  | TODO    | TODO  | YES
    KEY_UP              | YES     | YES   | YES   | SOME  | TODO    | TODO  | YES
    CHAR                | YES     | YES   | YES   | YES   | TODO    | TODO  | YES
    MOUSE_DOWN          | YES     | YES   | YES   | ---   | ---     | TODO  | YES
    MOUSE_UP            | YES     | YES   | YES   | ---   | ---     | TODO  | YES
    MOUSE_SCROLL        | YES     | YES   | YES   | ---   | ---     | TODO  | YES
    MOUSE_MOVE          | YES     | YES   | YES   | ---   | ---     | TODO  | YES
    MOUSE_ENTER         | YES     | YES   | YES   | ---   | ---     | TODO  | YES
    MOUSE_LEAVE         | YES     | YES   | YES   | ---   | ---     | TODO  | YES
    TOUCHES_BEGAN       | ---     | ---   | ---   | YES   | YES     | ---   | YES
    TOUCHES_MOVED       | ---     | ---   | ---   | YES   | YES     | ---   | YES
    TOUCHES_ENDED       | ---     | ---   | ---   | YES   | YES     | ---   | YES
    TOUCHES_CANCELLED   | ---     | ---   | ---   | YES   | YES     | ---   | YES
    RESIZED             | YES     | YES   | YES   | YES   | YES     | ---   | YES
    ICONIFIED           | YES     | YES   | YES   | ---   | ---     | ---   | ---
    RESTORED            | YES     | YES   | YES   | ---   | ---     | ---   | ---
    SUSPENDED           | ---     | ---   | ---   | YES   | YES     | ---   | TODO
    RESUMED             | ---     | ---   | ---   | YES   | YES     | ---   | TODO
    QUIT_REQUESTED      | YES     | YES   | YES   | ---   | ---     | TODO  | ---
    UPDATE_CURSOR       | YES     | YES   | TODO  | ---   | ---     | ---   | TODO
    IME                 | TODO    | TODO? | TODO  | ???   | TODO    | ???   | ???
    key repeat flag     | YES     | YES   | YES   | ---   | ---     | TODO  | YES
    windowed            | YES     | YES   | YES   | ---   | ---     | TODO  | YES
    fullscreen          | YES     | YES   | TODO  | YES   | YES     | TODO  | ---
    pointer lock        | TODO    | TODO  | TODO  | ---   | ---     | TODO  | TODO
    screen keyboard     | ---     | ---   | ---   | YES   | TODO    | ---   | YES
    swap interval       | YES     | YES   | YES   | YES   | TODO    | TODO  | YES
    high-dpi            | YES     | YES   | TODO  | YES   | YES     | TODO  | YES

    - what about bluetooth keyboard / mouse on mobile platforms?

    STEP BY STEP
    ============
    --- Add a sokol_main() function to your code which returns a sapp_desc structure
        with initialization parameters and callback function pointers. This
        function is called very early, usually at the start of the
        platform's entry function (e.g. main or WinMain). You should do as
        little as possible here, since the rest of your code might be called
        from another thread (this depends on the platform):

            sapp_desc sokol_main(int argc, char* argv[]) {
                return (sapp_desc) {
                    .width = 640,
                    .height = 480,
                    .init_cb = my_init_func,
                    .frame_cb = my_frame_func,
                    .cleanup_cb = my_cleanup_func,
                    .event_cb = my_event_func,
                    ...
                };
            }

        There are many more setup parameters, but these are the most important.
        For a complete list search for the sapp_desc structure declaration
        below.

        DO NOT call any sokol-app function from inside sokol_main(), since
        sokol-app will not be initialized at this point.

        The .width and .height parameters are the preferred size of the 3D
        rendering canvas. The actual size may differ from this depending on
        platform and other circumstances. Also the canvas size may change at
        any time (for instance when the user resizes the application window,
        or rotates the mobile device).

        All provided function callbacks will be called from the same thread,
        but this may be different from the thread where sokol_main() was called.

        .init_cb (void (*)(void))
            This function is called once after the application window,
            3D rendering context and swap chain have been created. The
            function takes no arguments and has no return value.
        .frame_cb (void (*)(void))
            This is the per-frame callback, which is usually called 60
            times per second. This is where your application would update
            most of its state and perform all rendering.
        .cleanup_cb (void (*)(void))
            The cleanup callback is called once right before the application
            quits.
        .event_cb (void (*)(const sapp_event* event))
            The event callback is mainly for input handling, but in the
            future may also be used to communicate other types of events
            to the application. Keep the event_cb struct member zero-initialized
            if your application doesn't require event handling.
        .fail_cb (void (*)(const char* msg))
            The fail callback is called when a fatal error is encountered
            during start which doesn't allow the program to continue.
            Providing a callback here gives you a chance to show an error message
            to the user. The default behaviour is SOKOL_LOG(msg)

        As you can see, those 'standard callbacks' don't have a user_data
        argument, so any data that needs to be preserved between callbacks
        must live in global variables. If you're allergic to global variables
        or cannot use them for other reasons, an alternative set of callbacks
        can be defined in sapp_desc, together with a user_data pointer:

        .user_data (void*)
            The user-data argument for the callbacks below
        .init_userdata_cb (void (*)(void* user_data))
        .frame_userdata_cb (void (*)(void* user_data))
        .cleanup_userdata_cb (void (*)(void* user_data))
        .event_cb (void(*)(const sapp_event* event, void* user_data))
        .fail_cb (void(*)(const char* msg, void* user_data))
            These are the user-data versions of the callback functions. You
            can mix those with the standard callbacks that don't have the
            user_data argument.

        The function sapp_userdata() can be used to query the user_data
        pointer provided in the sapp_desc struct.

        You can call sapp_query_desc() to get a copy of the
        original sapp_desc structure.

        NOTE that there's also an alternative compile mode where sokol_app.h
        doesn't "hijack" the main() function. Search below for SOKOL_NO_ENTRY.

    --- Implement the initialization callback function (init_cb), this is called
        once after the rendering surface, 3D API and swap chain have been
        initialized by sokol_app. All sokol-app functions can be called
        from inside the initialization callback, the most useful functions
        at this point are:

        int sapp_width(void)
            Returns the current width of the default framebuffer, this may change
            from one frame to the next.
        int sapp_height(void)
            Likewise, returns the current height of the default framebuffer.

        bool sapp_gles2(void)
            Returns true if a GLES2 or WebGL context has been created. This
            is useful when a GLES3/WebGL2 context was requested but is not
            available so that sokol_app.h had to fallback to GLES2/WebGL.

        const void* sapp_metal_get_device(void)
        const void* sapp_metal_get_renderpass_descriptor(void)
        const void* sapp_metal_get_drawable(void)
            If the Metal backend has been selected, these functions return pointers
            to various Metal API objects required for rendering, otherwise
            they return a null pointer. These void pointers are actually
            Objective-C ids converted with an ARC __bridge cast so that
            they ids can be tunnel through C code. Also note that the returned
            pointers to the renderpass-descriptor and drawable may change from one
            frame to the next, only the Metal device object is guaranteed to
            stay the same.

        const void* sapp_macos_get_window(void)
            On macOS, get the NSWindow object pointer, otherwise a null pointer.
            Before being used as Objective-C object, the void* must be converted
            back with an ARC __bridge cast.

        const void* sapp_ios_get_window(void)
            On iOS, get the UIWindow object pointer, otherwise a null pointer.
            Before being used as Objective-C object, the void* must be converted
            back with an ARC __bridge cast.

        const void* sapp_win32_get_hwnd(void)
            On Windows, get the window's HWND, otherwise a null pointer. The
            HWND has been cast to a void pointer in order to be tunneled
            through code which doesn't include Windows.h.

        const void* sapp_d3d11_get_device(void);
        const void* sapp_d3d11_get_device_context(void);
        const void* sapp_d3d11_get_render_target_view(void);
        const void* sapp_d3d11_get_depth_stencil_view(void);
            Similar to the sapp_metal_* functions, the sapp_d3d11_* functions
            return pointers to D3D11 API objects required for rendering,
            only if the D3D11 backend has been selected. Otherwise they
            return a null pointer. Note that the returned pointers to the
            render-target-view and depth-stencil-view may change from one
            frame to the next!

        const void* sapp_android_get_native_activity(void);
            On Android, get the native activity ANativeActivity pointer, otherwise
            a null pointer.

    --- Implement the frame-callback function, this function will be called
        on the same thread as the init callback, but might be on a different
        thread than the sokol_main() function. Note that the size of
        the rendering framebuffer might have changed since the frame callback
        was called last. Call the functions sapp_width() and sapp_height()
        each frame to get the current size.

    --- Optionally implement the event-callback to handle input events.
        sokol-app provides the following type of input events:
            - a 'virtual key' was pressed down or released
            - a single text character was entered (provided as UTF-32 code point)
            - a mouse button was pressed down or released (left, right, middle)
            - mouse-wheel or 2D scrolling events
            - the mouse was moved
            - the mouse has entered or left the application window boundaries
            - low-level, portable multi-touch events (began, moved, ended, cancelled)
            - the application window was resized, iconified or restored
            - the application was suspended or restored (on mobile platforms)
            - the user or application code has asked to quit the application

    --- Implement the cleanup-callback function, this is called once
        after the user quits the application (see the section
        "APPLICATION QUIT" for detailed information on quitting
        behaviour, and how to intercept a pending quit (for instance to show a
        "Really Quit?" dialog box). Note that the cleanup-callback isn't
        called on the web and mobile platforms.

    HIGH-DPI RENDERING
    ==================
    You can set the sapp_desc.high_dpi flag during initialization to request
    a full-resolution framebuffer on HighDPI displays. The default behaviour
    is sapp_desc.high_dpi=false, this means that the application will
    render to a lower-resolution framebuffer on HighDPI displays and the
    rendered content will be upscaled by the window system composer.

    In a HighDPI scenario, you still request the same window size during
    sokol_main(), but the framebuffer sizes returned by sapp_width()
    and sapp_height() will be scaled up according to the DPI scaling
    ratio. You can also get a DPI scaling factor with the function
    sapp_dpi_scale().

    Here's an example on a Mac with Retina display:

    sapp_desc sokol_main() {
        return (sapp_desc) {
            .width = 640,
            .height = 480,
            .high_dpi = true,
            ...
        };
    }

    The functions sapp_width(), sapp_height() and sapp_dpi_scale() will
    return the following values:

    sapp_width      -> 1280
    sapp_height     -> 960
    sapp_dpi_scale  -> 2.0

    If the high_dpi flag is false, or you're not running on a Retina display,
    the values would be:

    sapp_width      -> 640
    sapp_height     -> 480
    sapp_dpi_scale  -> 1.0

    APPLICATION QUIT
    ================
    Without special quit handling, a sokol_app.h application will exist
    'gracefully' when the user clicks the window close-button. 'Graceful
    exit' means that the application-provided cleanup callback will be
    called.

    This 'graceful exit' is only supported on native desktop platforms, on
    the web and mobile platforms an application may be terminated at any time
    by the user or browser/OS runtime environment without a chance to run
    custom shutdown code.

    On the web platform, you can call the following function to let the
    browser open a standard popup dialog before the user wants to leave a site:

        sapp_html5_ask_leave_site(bool ask);

    The initial state of the associated internal flag can be provided
    at startup via sapp_desc.html5_ask_leave_site.

    This feature should only be used sparingly in critical situations - for
    instance when the user would loose data - since popping up modal dialog
    boxes is considered quite rude in the web world. Note that there's no way
    to customize the content of this dialog box or run any code as a result
    of the user's decision. Also note that the user must have interacted with
    the site before the dialog box will appear. These are all security measures
    to prevent fishing.

    On native desktop platforms, sokol_app.h provides more control over the
    application-quit-process. It's possible to initiate a 'programmatic quit'
    from the application code, and a quit initiated by the application user
    can be intercepted (for instance to show a custom dialog box).

    This 'programmatic quit protocol' is implemented trough 3 functions
    and 1 event:

        - sapp_quit(): This function simply quits the application without
          giving the user a chance to intervene. Usually this might
          be called when the user clicks the 'Ok' button in a 'Really Quit?'
          dialog box
        - sapp_request_quit(): Calling sapp_request_quit() will send the
          event SAPP_EVENTTYPE_QUIT_REQUESTED to the applications event handler
          callback, giving the user code a chance to intervene and cancel the
          pending quit process (for instance to show a 'Really Quit?' dialog
          box). If the event handler callback does nothing, the application
          will be quit as usual. To prevent this, call the function
          sapp_cancel_quit() from inside the event handler.
        - sapp_cancel_quit(): Cancels a pending quit request, either initiated
          by the user clicking the window close button, or programmatically
          by calling sapp_request_quit(). The only place where calling this
          function makes sense is from inside the event handler callback when
          the SAPP_EVENTTYPE_QUIT_REQUESTED event has been received.
        - SAPP_EVENTTYPE_QUIT_REQUESTED: this event is sent when the user
          clicks the window's close button or application code calls the
          sapp_request_quit() function. The event handler callback code can handle
          this event by calling sapp_cancel_quit() to cancel the quit.
          If the event is ignored, the application will quit as usual.

    The Dear ImGui HighDPI sample contains example code of how to
    implement a 'Really Quit?' dialog box with Dear ImGui (native desktop
    platforms only), and for showing the hardwired "Leave Site?" dialog box
    when running on the web platform:

        https://floooh.github.io/sokol-html5/wasm/imgui-highdpi-sapp.html

    FULLSCREEN
    ==========
    If the sapp_desc.fullscreen flag is true, sokol-app will try to create
    a fullscreen window on platforms with a 'proper' window system
    (mobile devices will always use fullscreen). The implementation details
    depend on the target platform, in general sokol-app will use a
    'soft approach' which doesn't interfere too much with the platform's
    window system (for instance borderless fullscreen window instead of
    a 'real' fullscreen mode). Such details might change over time
    as sokol-app is adapted for different needs.

    The most important effect of fullscreen mode to keep in mind is that
    the requested canvas width and height will be ignored for the initial
    window size, calling sapp_width() and sapp_height() will instead return
    the resolution of the fullscreen canvas (however the provided size
    might still be used for the non-fullscreen window, in case the user can
    switch back from fullscreen- to windowed-mode).

    ONSCREEN KEYBOARD
    =================
    On some platforms which don't provide a physical keyboard, sokol-app
    can display the platform's integrated onscreen keyboard for text
    input. To request that the onscreen keyboard is shown, call

        sapp_show_keyboard(true);

    Likewise, to hide the keyboard call:

        sapp_show_keyboard(false);

    Note that on the web platform, the keyboard can only be shown from
    inside an input handler. On such platforms, sapp_show_keyboard()
    will only work as expected when it is called from inside the
    sokol-app event callback function. When called from other places,
    an internal flag will be set, and the onscreen keyboard will be
    called at the next 'legal' opportunity (when the next input event
    is handled).

    OPTIONAL: DON'T HIJACK main() (#define SOKOL_NO_ENTRY)
    ======================================================
    In its default configuration, sokol_app.h "hijacks" the platform's
    standard main() function. This was done because different platforms
    have different main functions which are not compatible with
    C's main() (for instance WinMain on Windows has completely different
    arguments). However, this "main hijacking" posed a problem for
    usage scenarios like integrating sokol_app.h with other languages than
    C or C++, so an alternative SOKOL_NO_ENTRY mode has been added
    in which the user code provides the platform's main function:

    - define SOKOL_NO_ENTRY before including the sokol_app.h implementation
    - do *not* provide a sokol_main() function
    - instead provide the standard main() function of the platform
    - from the main function, call the function ```sapp_run()``` which
      takes a pointer to an ```sapp_desc``` structure.
    - ```sapp_run()``` takes over control and calls the provided init-, frame-,
      shutdown- and event-callbacks just like in the default model, it
      will only return when the application quits (or not at all on some
      platforms, like emscripten)

    NOTE: SOKOL_NO_ENTRY is currently not supported on Android.

    TEMP NOTE DUMP
    ==============
    - onscreen keyboard support on Android requires Java :(, should we even bother?
    - sapp_desc needs a bool whether to initialize depth-stencil surface
    - GL context initialization needs more control (at least what GL version to initialize)
    - application icon
    - mouse pointer visibility(?)
    - the UPDATE_CURSOR event currently behaves differently between Win32 and OSX
      (Win32 sends the event each frame when the mouse moves and is inside the window
      client area, OSX sends it only once when the mouse enters the client area)
    - the Android implementation calls cleanup_cb() and destroys the egl context in onDestroy
      at the latest but should do it earlier, in onStop, as an app is "killable" after onStop
      on Android Honeycomb and later (it can't be done at the moment as the app may be started
      again after onStop and the sokol lifecycle does not yet handle context teardown/bringup)

    FIXME: ERROR HANDLING (this will need an error callback function)

    zlib/libpng license

    Copyright (c) 2018 Andre Weissflog

    This software is provided 'as-is', without any express or implied warranty.
    In no event will the authors be held liable for any damages arising from the
    use of this software.

    Permission is granted to anyone to use this software for any purpose,
    including commercial applications, and to alter it and redistribute it
    freely, subject to the following restrictions:

        1. The origin of this software must not be misrepresented; you must not
        claim that you wrote the original software. If you use this software in a
        product, an acknowledgment in the product documentation would be
        appreciated but is not required.

        2. Altered source versions must be plainly marked as such, and must not
        be misrepresented as being the original software.

        3. This notice may not be removed or altered from any source
        distribution.
*/
#define SOKOL_APP_INCLUDED (1)
#include <stdint.h>
#include <stdbool.h>

#ifndef SOKOL_API_DECL
#if defined(_WIN32) && defined(SOKOL_DLL) && defined(SOKOL_IMPL)
#define SOKOL_API_DECL __declspec(dllexport)
#elif defined(_WIN32) && defined(SOKOL_DLL)
#define SOKOL_API_DECL __declspec(dllimport)
#else
#define SOKOL_API_DECL extern
#endif
#endif

#ifdef __cplusplus
extern "C" {
#endif

enum {
    SAPP_MAX_TOUCHPOINTS = 8,
    SAPP_MAX_MOUSEBUTTONS = 3,
    SAPP_MAX_KEYCODES = 512,
};

typedef enum sapp_event_type {
    SAPP_EVENTTYPE_INVALID,
    SAPP_EVENTTYPE_KEY_DOWN,
    SAPP_EVENTTYPE_KEY_UP,
    SAPP_EVENTTYPE_CHAR,
    SAPP_EVENTTYPE_MOUSE_DOWN,
    SAPP_EVENTTYPE_MOUSE_UP,
    SAPP_EVENTTYPE_MOUSE_SCROLL,
    SAPP_EVENTTYPE_MOUSE_MOVE,
    SAPP_EVENTTYPE_MOUSE_ENTER,
    SAPP_EVENTTYPE_MOUSE_LEAVE,
    SAPP_EVENTTYPE_TOUCHES_BEGAN,
    SAPP_EVENTTYPE_TOUCHES_MOVED,
    SAPP_EVENTTYPE_TOUCHES_ENDED,
    SAPP_EVENTTYPE_TOUCHES_CANCELLED,
    SAPP_EVENTTYPE_RESIZED,
    SAPP_EVENTTYPE_ICONIFIED,
    SAPP_EVENTTYPE_RESTORED,
    SAPP_EVENTTYPE_SUSPENDED,
    SAPP_EVENTTYPE_RESUMED,
    SAPP_EVENTTYPE_UPDATE_CURSOR,
    SAPP_EVENTTYPE_QUIT_REQUESTED,
    SAPP_EVENTTYPE_RAW_DEVICE,
    _SAPP_EVENTTYPE_NUM,
    _SAPP_EVENTTYPE_FORCE_U32 = 0x7FFFFFFF
} sapp_event_type;

/* key codes are the same names and values as GLFW */
typedef enum sapp_keycode {
    SAPP_KEYCODE_INVALID          = 0,
    SAPP_KEYCODE_SPACE            = 32,
    SAPP_KEYCODE_APOSTROPHE       = 39,  /* ' */
    SAPP_KEYCODE_COMMA            = 44,  /* , */
    SAPP_KEYCODE_MINUS            = 45,  /* - */
    SAPP_KEYCODE_PERIOD           = 46,  /* . */
    SAPP_KEYCODE_SLASH            = 47,  /* / */
    SAPP_KEYCODE_0                = 48,
    SAPP_KEYCODE_1                = 49,
    SAPP_KEYCODE_2                = 50,
    SAPP_KEYCODE_3                = 51,
    SAPP_KEYCODE_4                = 52,
    SAPP_KEYCODE_5                = 53,
    SAPP_KEYCODE_6                = 54,
    SAPP_KEYCODE_7                = 55,
    SAPP_KEYCODE_8                = 56,
    SAPP_KEYCODE_9                = 57,
    SAPP_KEYCODE_SEMICOLON        = 59,  /* ; */
    SAPP_KEYCODE_EQUAL            = 61,  /* = */
    SAPP_KEYCODE_A                = 65,
    SAPP_KEYCODE_B                = 66,
    SAPP_KEYCODE_C                = 67,
    SAPP_KEYCODE_D                = 68,
    SAPP_KEYCODE_E                = 69,
    SAPP_KEYCODE_F                = 70,
    SAPP_KEYCODE_G                = 71,
    SAPP_KEYCODE_H                = 72,
    SAPP_KEYCODE_I                = 73,
    SAPP_KEYCODE_J                = 74,
    SAPP_KEYCODE_K                = 75,
    SAPP_KEYCODE_L                = 76,
    SAPP_KEYCODE_M                = 77,
    SAPP_KEYCODE_N                = 78,
    SAPP_KEYCODE_O                = 79,
    SAPP_KEYCODE_P                = 80,
    SAPP_KEYCODE_Q                = 81,
    SAPP_KEYCODE_R                = 82,
    SAPP_KEYCODE_S                = 83,
    SAPP_KEYCODE_T                = 84,
    SAPP_KEYCODE_U                = 85,
    SAPP_KEYCODE_V                = 86,
    SAPP_KEYCODE_W                = 87,
    SAPP_KEYCODE_X                = 88,
    SAPP_KEYCODE_Y                = 89,
    SAPP_KEYCODE_Z                = 90,
    SAPP_KEYCODE_LEFT_BRACKET     = 91,  /* [ */
    SAPP_KEYCODE_BACKSLASH        = 92,  /* \ */
    SAPP_KEYCODE_RIGHT_BRACKET    = 93,  /* ] */
    SAPP_KEYCODE_GRAVE_ACCENT     = 96,  /* ` */
    SAPP_KEYCODE_WORLD_1          = 161, /* non-US #1 */
    SAPP_KEYCODE_WORLD_2          = 162, /* non-US #2 */
    SAPP_KEYCODE_ESCAPE           = 256,
    SAPP_KEYCODE_ENTER            = 257,
    SAPP_KEYCODE_TAB              = 258,
    SAPP_KEYCODE_BACKSPACE        = 259,
    SAPP_KEYCODE_INSERT           = 260,
    SAPP_KEYCODE_DELETE           = 261,
    SAPP_KEYCODE_RIGHT            = 262,
    SAPP_KEYCODE_LEFT             = 263,
    SAPP_KEYCODE_DOWN             = 264,
    SAPP_KEYCODE_UP               = 265,
    SAPP_KEYCODE_PAGE_UP          = 266,
    SAPP_KEYCODE_PAGE_DOWN        = 267,
    SAPP_KEYCODE_HOME             = 268,
    SAPP_KEYCODE_END              = 269,
    SAPP_KEYCODE_CAPS_LOCK        = 280,
    SAPP_KEYCODE_SCROLL_LOCK      = 281,
    SAPP_KEYCODE_NUM_LOCK         = 282,
    SAPP_KEYCODE_PRINT_SCREEN     = 283,
    SAPP_KEYCODE_PAUSE            = 284,
    SAPP_KEYCODE_F1               = 290,
    SAPP_KEYCODE_F2               = 291,
    SAPP_KEYCODE_F3               = 292,
    SAPP_KEYCODE_F4               = 293,
    SAPP_KEYCODE_F5               = 294,
    SAPP_KEYCODE_F6               = 295,
    SAPP_KEYCODE_F7               = 296,
    SAPP_KEYCODE_F8               = 297,
    SAPP_KEYCODE_F9               = 298,
    SAPP_KEYCODE_F10              = 299,
    SAPP_KEYCODE_F11              = 300,
    SAPP_KEYCODE_F12              = 301,
    SAPP_KEYCODE_F13              = 302,
    SAPP_KEYCODE_F14              = 303,
    SAPP_KEYCODE_F15              = 304,
    SAPP_KEYCODE_F16              = 305,
    SAPP_KEYCODE_F17              = 306,
    SAPP_KEYCODE_F18              = 307,
    SAPP_KEYCODE_F19              = 308,
    SAPP_KEYCODE_F20              = 309,
    SAPP_KEYCODE_F21              = 310,
    SAPP_KEYCODE_F22              = 311,
    SAPP_KEYCODE_F23              = 312,
    SAPP_KEYCODE_F24              = 313,
    SAPP_KEYCODE_F25              = 314,
    SAPP_KEYCODE_KP_0             = 320,
    SAPP_KEYCODE_KP_1             = 321,
    SAPP_KEYCODE_KP_2             = 322,
    SAPP_KEYCODE_KP_3             = 323,
    SAPP_KEYCODE_KP_4             = 324,
    SAPP_KEYCODE_KP_5             = 325,
    SAPP_KEYCODE_KP_6             = 326,
    SAPP_KEYCODE_KP_7             = 327,
    SAPP_KEYCODE_KP_8             = 328,
    SAPP_KEYCODE_KP_9             = 329,
    SAPP_KEYCODE_KP_DECIMAL       = 330,
    SAPP_KEYCODE_KP_DIVIDE        = 331,
    SAPP_KEYCODE_KP_MULTIPLY      = 332,
    SAPP_KEYCODE_KP_SUBTRACT      = 333,
    SAPP_KEYCODE_KP_ADD           = 334,
    SAPP_KEYCODE_KP_ENTER         = 335,
    SAPP_KEYCODE_KP_EQUAL         = 336,
    SAPP_KEYCODE_LEFT_SHIFT       = 340,
    SAPP_KEYCODE_LEFT_CONTROL     = 341,
    SAPP_KEYCODE_LEFT_ALT         = 342,
    SAPP_KEYCODE_LEFT_SUPER       = 343,
    SAPP_KEYCODE_RIGHT_SHIFT      = 344,
    SAPP_KEYCODE_RIGHT_CONTROL    = 345,
    SAPP_KEYCODE_RIGHT_ALT        = 346,
    SAPP_KEYCODE_RIGHT_SUPER      = 347,
    SAPP_KEYCODE_MENU             = 348,
} sapp_keycode;

typedef struct sapp_touchpoint {
    uintptr_t identifier;
    float pos_x;
    float pos_y;
    bool changed;
} sapp_touchpoint;

typedef enum sapp_mousebutton {
    SAPP_MOUSEBUTTON_INVALID = -1,
    SAPP_MOUSEBUTTON_LEFT = 0,
    SAPP_MOUSEBUTTON_RIGHT = 1,
    SAPP_MOUSEBUTTON_MIDDLE = 2,
} sapp_mousebutton;

enum {
    SAPP_MODIFIER_SHIFT = (1<<0),
    SAPP_MODIFIER_CTRL = (1<<1),
    SAPP_MODIFIER_ALT = (1<<2),
    SAPP_MODIFIER_SUPER = (1<<3)
};

typedef struct sapp_event {
    uint64_t frame_count;
    sapp_event_type type;
    sapp_keycode key_code;
    uint32_t char_code;
    bool key_repeat;
    uint32_t modifiers;
    sapp_mousebutton mouse_button;
    float mouse_x;
    float mouse_y;
    float mouse_dx;
    float mouse_dy;
    float scroll_x;
    float scroll_y;
    int num_touches;
    sapp_touchpoint touches[SAPP_MAX_TOUCHPOINTS];
    int window_width;
    int window_height;
    int framebuffer_width;
    int framebuffer_height;
} sapp_event;

typedef struct sapp_desc {
    void (*init_cb)(void);                  /* these are the user-provided callbacks without user data */
    void (*frame_cb)(void);
    void (*cleanup_cb)(void);
    void (*event_cb)(const sapp_event*);
    void (*fail_cb)(const char*);

    void* user_data;                        /* these are the user-provided callbacks with user data */
    void (*init_userdata_cb)(void*);
    void (*frame_userdata_cb)(void*);
    void (*cleanup_userdata_cb)(void*);
    void (*event_userdata_cb)(const sapp_event*, void*);
    void (*fail_userdata_cb)(const char*, void*);

    int width;                          /* the preferred width of the window / canvas */
    int height;                         /* the preferred height of the window / canvas */
    int sample_count;                   /* MSAA sample count */
    int swap_interval;                  /* the preferred swap interval (ignored on some platforms) */
    bool high_dpi;                      /* whether the rendering canvas is full-resolution on HighDPI displays */
    bool fullscreen;                    /* whether the window should be created in fullscreen mode */
    bool alpha;                         /* whether the framebuffer should have an alpha channel (ignored on some platforms) */
    const char* window_title;           /* the window title as UTF-8 encoded string */
    bool user_cursor;                   /* if true, user is expected to manage cursor image in SAPP_EVENTTYPE_UPDATE_CURSOR */

    const char* html5_canvas_name;      /* the name (id) of the HTML5 canvas element, default is "canvas" */
    bool html5_canvas_resize;           /* if true, the HTML5 canvas size is set to sapp_desc.width/height, otherwise canvas size is tracked */
    bool html5_preserve_drawing_buffer; /* HTML5 only: whether to preserve default framebuffer content between frames */
    bool html5_premultiplied_alpha;     /* HTML5 only: whether the rendered pixels use premultiplied alpha convention */
    bool html5_ask_leave_site;          /* initial state of the internal html5_ask_leave_site flag (see sapp_html5_ask_leave_site()) */
    bool ios_keyboard_resizes_canvas;   /* if true, showing the iOS keyboard shrinks the canvas */
    bool gl_force_gles2;                /* if true, setup GLES2/WebGL even if GLES3/WebGL2 is available */
} sapp_desc;

/* user-provided functions */
extern void sokol_main(int argc, char* argv[]);

/* returns true after sokol-app has been initialized */
SOKOL_API_DECL bool sapp_isvalid(void);
/* returns the current framebuffer width in pixels */
SOKOL_API_DECL int sapp_width(void);
/* returns the current framebuffer height in pixels */
SOKOL_API_DECL int sapp_height(void);
/* returns true when high_dpi was requested and actually running in a high-dpi scenario */
SOKOL_API_DECL bool sapp_high_dpi(void);
/* returns the dpi scaling factor (window pixels to framebuffer pixels) */
SOKOL_API_DECL float sapp_dpi_scale(void);
/* show or hide the mobile device onscreen keyboard */
SOKOL_API_DECL void sapp_show_keyboard(bool visible);
/* return true if the mobile device onscreen keyboard is currently shown */
SOKOL_API_DECL bool sapp_keyboard_shown(void);

SOKOL_API_DECL void sapp_set_cursor_grab(bool grab);
/* show or hide the mouse cursor */
SOKOL_API_DECL void sapp_show_mouse(bool visible);
/* show or hide the mouse cursor */
SOKOL_API_DECL bool sapp_mouse_shown();
/* return the userdata pointer optionally provided in sapp_desc */
SOKOL_API_DECL void* sapp_userdata(void);
/* return a copy of the sapp_desc structure */
SOKOL_API_DECL sapp_desc sapp_query_desc(void);
/* initiate a "soft quit" (sends SAPP_EVENTTYPE_QUIT_REQUESTED) */
SOKOL_API_DECL void sapp_request_quit(void);
/* cancel a pending quit (when SAPP_EVENTTYPE_QUIT_REQUESTED has been received) */
SOKOL_API_DECL void sapp_cancel_quit(void);
/* intiate a "hard quit" (quit application without sending SAPP_EVENTTYPE_QUIT_REQUSTED) */
SOKOL_API_DECL void sapp_quit(void);
/* get the current frame counter (for comparison with sapp_event.frame_count) */
SOKOL_API_DECL uint64_t sapp_frame_count(void);

/* special run-function for SOKOL_NO_ENTRY (in standard mode this is an empty stub) */
SOKOL_API_DECL int sapp_run(const sapp_desc* desc);

/* GL: return true when GLES2 fallback is active (to detect fallback from GLES3) */
SOKOL_API_DECL bool sapp_gles2(void);

/* HTML5: enable or disable the hardwired "Leave Site?" dialog box */
SOKOL_API_DECL void sapp_html5_ask_leave_site(bool ask);

/* Metal: get ARC-bridged pointer to Metal device object */
SOKOL_API_DECL const void* sapp_metal_get_device(void);
/* Metal: get ARC-bridged pointer to this frame's renderpass descriptor */
SOKOL_API_DECL const void* sapp_metal_get_renderpass_descriptor(void);
/* Metal: get ARC-bridged pointer to current drawable */
SOKOL_API_DECL const void* sapp_metal_get_drawable(void);
/* macOS: get ARC-bridged pointer to macOS NSWindow */
SOKOL_API_DECL const void* sapp_macos_get_window(void);
/* iOS: get ARC-bridged pointer to iOS UIWindow */
SOKOL_API_DECL const void* sapp_ios_get_window(void);

/* D3D11: get pointer to ID3D11Device object */
SOKOL_API_DECL const void* sapp_d3d11_get_device(void);
/* D3D11: get pointer to ID3D11DeviceContext object */
SOKOL_API_DECL const void* sapp_d3d11_get_device_context(void);
/* D3D11: get pointer to ID3D11RenderTargetView object */
SOKOL_API_DECL const void* sapp_d3d11_get_render_target_view(void);
/* D3D11: get pointer to ID3D11DepthStencilView */
SOKOL_API_DECL const void* sapp_d3d11_get_depth_stencil_view(void);
/* Win32: get the HWND window handle */
SOKOL_API_DECL const void* sapp_win32_get_hwnd(void);

/* Android: get native activity handle */
SOKOL_API_DECL const void* sapp_android_get_native_activity(void);

SOKOL_API_DECL void sapp_ANativeActivity_onCreate(void* activity, void* saved_state, int saved_state_size);

SOKOL_API_DECL void sapp_android_log(const char *message);

typedef struct {
    char * content;
    int content_length;
} android_asset;

SOKOL_API_DECL void sapp_load_asset(const char *filepath, android_asset *asset);

#ifdef __cplusplus
} /* extern "C" */
#endif
#endif // SOKOL_APP_INCLUDED

/*-- IMPLEMENTATION ----------------------------------------------------------*/
#ifdef SOKOL_IMPL
#define SOKOL_APP_IMPL_INCLUDED (1)

#ifdef _MSC_VER
#pragma warning(push)
#pragma warning(disable:4201)   /* nonstandard extension used: nameless struct/union */
#pragma warning(disable:4115)   /* named type definition in parentheses */
#pragma warning(disable:4054)   /* 'type cast': from function pointer */
#pragma warning(disable:4055)   /* 'type cast': from data pointer */
#pragma warning(disable:4505)   /* unreferenced local function has been removed */
#endif

//#include <string.h> /* memset */
void* memset(void*,int,size_t);


/* check if the config defines are alright */
#if defined(__ANDROID__)
    /* Android */
    #if !defined(SOKOL_GLES3) && !defined(SOKOL_GLES2)
    #error("sokol_app.h: unknown 3D API selected for Android, must be SOKOL_GLES3 or SOKOL_GLES2")
    #endif
    #if defined(SOKOL_NO_ENTRY)
    #error("sokol_app.h: SOKOL_NO_ENTRY is not supported on Android")
    #endif
#else
#error "sokol_app.h: Unknown platform"
#endif

#ifndef SOKOL_API_IMPL
    #define SOKOL_API_IMPL
#endif
#ifndef SOKOL_DEBUG
    #ifndef NDEBUG
        #define SOKOL_DEBUG (1)
    #endif
#endif
#ifndef SOKOL_ASSERT
    #include <assert.h>
    #define SOKOL_ASSERT(c) assert(c)
#endif
#if !defined(SOKOL_CALLOC) || !defined(SOKOL_FREE)
    #include <stdlib.h>
#endif
#if !defined(SOKOL_CALLOC)
    #define SOKOL_CALLOC(n,s) calloc(n,s)
#endif
#if !defined(SOKOL_FREE)
    #define SOKOL_FREE(p) free(p)
#endif
#ifndef SOKOL_LOG
    #ifdef SOKOL_DEBUG
        #if defined(__ANDROID__)
            #include <android/log.h>
            #define SOKOL_LOG(s) { SOKOL_ASSERT(s); __android_log_write(ANDROID_LOG_INFO, "SOKOL_APP", s); }
        #else
            #include <stdio.h>
            #define SOKOL_LOG(s) { SOKOL_ASSERT(s); puts(s); }
        #endif
    #else
        #define SOKOL_LOG(s)
    #endif
#endif
#ifndef SOKOL_ABORT
    #include <stdlib.h>
    #define SOKOL_ABORT() abort()
#endif
#ifndef _SOKOL_PRIVATE
    #if defined(__GNUC__)
        #define _SOKOL_PRIVATE __attribute__((unused)) static
    #else
        #define _SOKOL_PRIVATE static
    #endif
#endif
#ifndef _SOKOL_UNUSED
    #define _SOKOL_UNUSED(x) (void)(x)
#endif

/* helper macros */
#define _sapp_def(val, def) (((val) == 0) ? (def) : (val))
#define _sapp_absf(a) (((a)<0.0f)?-(a):(a))

enum {
    _SAPP_MAX_TITLE_LENGTH = 128,
};

typedef struct {
    bool valid;
    int window_width;
    int window_height;
    int framebuffer_width;
    int framebuffer_height;
    int sample_count;
    int swap_interval;
    float dpi_scale;
    bool gles2_fallback;
    bool first_frame;
    bool init_called;
    bool cleanup_called;
    bool quit_requested;
    bool quit_ordered;
    const char* html5_canvas_name;
    bool html5_ask_leave_site;
    char window_title[_SAPP_MAX_TITLE_LENGTH];      /* UTF-8 */
    wchar_t window_title_wide[_SAPP_MAX_TITLE_LENGTH];   /* UTF-32 or UCS-2 */
    uint64_t frame_count;
    float mouse_x;
    float mouse_y;
    bool win32_mouse_tracked;
    bool onscreen_keyboard_shown;
    sapp_event event;
    sapp_desc desc;
    sapp_keycode keycodes[SAPP_MAX_KEYCODES];
} _sapp_state;
static _sapp_state _sapp;

_SOKOL_PRIVATE void _sapp_fail(const char* msg) {
    if (_sapp.desc.fail_cb) {
        _sapp.desc.fail_cb(msg);
    }
    else if (_sapp.desc.fail_userdata_cb) {
        _sapp.desc.fail_userdata_cb(msg, _sapp.desc.user_data);
    }
    else {
        SOKOL_LOG(msg);
    }
    SOKOL_ABORT();
}

_SOKOL_PRIVATE void _sapp_call_init(void) {
    if (_sapp.desc.init_cb) {
        _sapp.desc.init_cb();
    }
    else if (_sapp.desc.init_userdata_cb) {
        _sapp.desc.init_userdata_cb(_sapp.desc.user_data);
    }
    _sapp.init_called = true;
}

_SOKOL_PRIVATE void _sapp_call_frame(void) {
    if (_sapp.init_called && !_sapp.cleanup_called) {
        if (_sapp.desc.frame_cb) {
            _sapp.desc.frame_cb();
        }
        else if (_sapp.desc.frame_userdata_cb) {
            _sapp.desc.frame_userdata_cb(_sapp.desc.user_data);
        }
    }
}

_SOKOL_PRIVATE void _sapp_call_cleanup(void) {
    if (!_sapp.cleanup_called) {
        if (_sapp.desc.cleanup_cb) {
            _sapp.desc.cleanup_cb();
        }
        else if (_sapp.desc.cleanup_userdata_cb) {
            _sapp.desc.cleanup_userdata_cb(_sapp.desc.user_data);
        }
        _sapp.cleanup_called = true;
    }
}

_SOKOL_PRIVATE void _sapp_call_event(const sapp_event* e) {
    if (!_sapp.cleanup_called) {
        if (_sapp.desc.event_cb) {
            _sapp.desc.event_cb(e);
        }
        else if (_sapp.desc.event_userdata_cb) {
            _sapp.desc.event_userdata_cb(e, _sapp.desc.user_data);
        }
    }
}

_SOKOL_PRIVATE void _sapp_strcpy(const char* src, char* dst, int max_len) {
    SOKOL_ASSERT(src && dst && (max_len > 0));
    char* const end = &(dst[max_len-1]);
    char c = 0;
    for (int i = 0; i < max_len; i++) {
        c = *src;
        if (c != 0) {
            src++;
        }
        *dst++ = c;
    }
    /* truncated? */
    if (c != 0) {
        *end = 0;
    }
}

_SOKOL_PRIVATE void _sapp_init_state(const sapp_desc* desc) {
    memset(&_sapp, 0, sizeof(_sapp));
    _sapp.desc = *desc;
    _sapp.first_frame = true;
    _sapp.window_width = _sapp_def(_sapp.desc.width, 640);
    _sapp.window_height = _sapp_def(_sapp.desc.height, 480);
    _sapp.framebuffer_width = _sapp.window_width;
    _sapp.framebuffer_height = _sapp.window_height;
    _sapp.sample_count = _sapp_def(_sapp.desc.sample_count, 1);
    _sapp.swap_interval = _sapp_def(_sapp.desc.swap_interval, 1);
    _sapp.html5_canvas_name = _sapp_def(_sapp.desc.html5_canvas_name, "canvas");
    _sapp.html5_ask_leave_site = _sapp.desc.html5_ask_leave_site;
    if (_sapp.desc.window_title) {
        _sapp_strcpy(_sapp.desc.window_title, _sapp.window_title, sizeof(_sapp.window_title));
    }
    else {
        _sapp_strcpy("sokol_app", _sapp.window_title, sizeof(_sapp.window_title));
    }
    _sapp.dpi_scale = 1.0f;
}

_SOKOL_PRIVATE void _sapp_init_event(sapp_event_type type) {
    memset(&_sapp.event, 0, sizeof(_sapp.event));
    _sapp.event.type = type;
    _sapp.event.frame_count = _sapp.frame_count;
    _sapp.event.mouse_button = SAPP_MOUSEBUTTON_INVALID;
    _sapp.event.window_width = _sapp.window_width;
    _sapp.event.window_height = _sapp.window_height;
    _sapp.event.framebuffer_width = _sapp.framebuffer_width;
    _sapp.event.framebuffer_height = _sapp.framebuffer_height;
}

_SOKOL_PRIVATE bool _sapp_events_enabled(void) {
    /* only send events when an event callback is set, and the init function was called */
    return (_sapp.desc.event_cb || _sapp.desc.event_userdata_cb) && _sapp.init_called;
}

_SOKOL_PRIVATE sapp_keycode _sapp_translate_key(int scan_code) {
    if ((scan_code >= 0) && (scan_code < SAPP_MAX_KEYCODES)) {
        return _sapp.keycodes[scan_code];
    }
    else {
        return SAPP_KEYCODE_INVALID;
    }
}

_SOKOL_PRIVATE void _sapp_frame(void) {
    if (_sapp.first_frame) {
        _sapp.first_frame = false;
        _sapp_call_init();
    }
    _sapp_call_frame();
    _sapp.frame_count++;
}

/*== MISC GL SUPPORT FUNCTIONS ================================================*/
#if defined(SOKOL_GLCORE33)
typedef struct {
    int         red_bits;
    int         green_bits;
    int         blue_bits;
    int         alpha_bits;
    int         depth_bits;
    int         stencil_bits;
    int         samples;
    bool        doublebuffer;
    uintptr_t   handle;
} _sapp_gl_fbconfig;

_SOKOL_PRIVATE void _sapp_gl_init_fbconfig(_sapp_gl_fbconfig* fbconfig) {
    memset(fbconfig, 0, sizeof(_sapp_gl_fbconfig));
    /* -1 means "don't care" */
    fbconfig->red_bits = -1;
    fbconfig->green_bits = -1;
    fbconfig->blue_bits = -1;
    fbconfig->alpha_bits = -1;
    fbconfig->depth_bits = -1;
    fbconfig->stencil_bits = -1;
    fbconfig->samples = -1;
}

_SOKOL_PRIVATE const _sapp_gl_fbconfig* _sapp_gl_choose_fbconfig(const _sapp_gl_fbconfig* desired, const _sapp_gl_fbconfig* alternatives, unsigned int count) {
    unsigned int i;
    unsigned int missing, least_missing = 1000000;
    unsigned int color_diff, least_color_diff = 10000000;
    unsigned int extra_diff, least_extra_diff = 10000000;
    const _sapp_gl_fbconfig* current;
    const _sapp_gl_fbconfig* closest = NULL;
    for (i = 0;  i < count;  i++) {
        current = alternatives + i;
        if (desired->doublebuffer != current->doublebuffer) {
            continue;
        }
        missing = 0;
        if (desired->alpha_bits > 0 && current->alpha_bits == 0) {
            missing++;
        }
        if (desired->depth_bits > 0 && current->depth_bits == 0) {
            missing++;
        }
        if (desired->stencil_bits > 0 && current->stencil_bits == 0) {
            missing++;
        }
        if (desired->samples > 0 && current->samples == 0) {
            /* Technically, several multisampling buffers could be
                involved, but that's a lower level implementation detail and
                not important to us here, so we count them as one
            */
            missing++;
        }

        /* These polynomials make many small channel size differences matter
            less than one large channel size difference
            Calculate color channel size difference value
        */
        color_diff = 0;
        if (desired->red_bits != -1) {
            color_diff += (desired->red_bits - current->red_bits) * (desired->red_bits - current->red_bits);
        }
        if (desired->green_bits != -1) {
            color_diff += (desired->green_bits - current->green_bits) * (desired->green_bits - current->green_bits);
        }
        if (desired->blue_bits != -1) {
            color_diff += (desired->blue_bits - current->blue_bits) * (desired->blue_bits - current->blue_bits);
        }

        /* Calculate non-color channel size difference value */
        extra_diff = 0;
        if (desired->alpha_bits != -1) {
            extra_diff += (desired->alpha_bits - current->alpha_bits) * (desired->alpha_bits - current->alpha_bits);
        }
        if (desired->depth_bits != -1) {
            extra_diff += (desired->depth_bits - current->depth_bits) * (desired->depth_bits - current->depth_bits);
        }
        if (desired->stencil_bits != -1) {
            extra_diff += (desired->stencil_bits - current->stencil_bits) * (desired->stencil_bits - current->stencil_bits);
        }
        if (desired->samples != -1) {
            extra_diff += (desired->samples - current->samples) * (desired->samples - current->samples);
        }

        /* Figure out if the current one is better than the best one found so far
            Least number of missing buffers is the most important heuristic,
            then color buffer size match and lastly size match for other buffers
        */
        if (missing < least_missing) {
            closest = current;
        }
        else if (missing == least_missing) {
            if ((color_diff < least_color_diff) ||
                (color_diff == least_color_diff && extra_diff < least_extra_diff))
            {
                closest = current;
            }
        }
        if (current == closest) {
            least_missing = missing;
            least_color_diff = color_diff;
            least_extra_diff = extra_diff;
        }
    }
    return closest;
}
#endif

/*== Android ================================================================*/
#if defined(__ANDROID__)
#include <pthread.h>
#include <unistd.h>
#include <android/native_activity.h>
#include <android/looper.h>

#include <EGL/egl.h>
#if defined(SOKOL_GLES3)
    #include <GLES3/gl3.h>
#else
    #ifndef GL_EXT_PROTOTYPES
        #define GL_GLEXT_PROTOTYPES
    #endif
    #include <GLES2/gl2.h>
    #include <GLES2/gl2ext.h>
#endif

typedef struct {
    pthread_t thread;
    pthread_mutex_t mutex;
    pthread_cond_t cond;
    int read_from_main_fd;
    int write_from_main_fd;
} _sapp_android_pt_t;

typedef struct {
    ANativeWindow* window;
    AInputQueue* input;
} _sapp_android_resources_t;

typedef enum {
    _SOKOL_ANDROID_MSG_CREATE,
    _SOKOL_ANDROID_MSG_RESUME,
    _SOKOL_ANDROID_MSG_PAUSE,
    _SOKOL_ANDROID_MSG_FOCUS,
    _SOKOL_ANDROID_MSG_NO_FOCUS,
    _SOKOL_ANDROID_MSG_SET_NATIVE_WINDOW,
    _SOKOL_ANDROID_MSG_SET_INPUT_QUEUE,
    _SOKOL_ANDROID_MSG_DESTROY,
} _sapp_android_msg_t;

typedef struct {
    ANativeActivity* activity;
    _sapp_android_pt_t pt;
    _sapp_android_resources_t pending;
    _sapp_android_resources_t current;
    ALooper* looper;
    bool is_thread_started;
    bool is_thread_stopping;
    bool is_thread_stopped;
    bool has_created;
    bool has_resumed;
    bool has_focus;
    EGLConfig config;
    EGLDisplay display;
    EGLContext context;
    EGLSurface surface;
} _sapp_android_state_t;

static _sapp_android_state_t _sapp_android_state;

/* android loop thread */
_SOKOL_PRIVATE bool _sapp_android_init_egl(void) {
    _sapp_android_state_t* state = &_sapp_android_state;
    SOKOL_ASSERT(state->display == EGL_NO_DISPLAY);
    SOKOL_ASSERT(state->context == EGL_NO_CONTEXT);

    EGLDisplay display = eglGetDisplay(EGL_DEFAULT_DISPLAY);
    if (display == EGL_NO_DISPLAY) {
        return false;
    }
    if (eglInitialize(display, NULL, NULL) == EGL_FALSE) {
        return false;
    }

    EGLint alpha_size = _sapp.desc.alpha ? 8 : 0;
    const EGLint cfg_attributes[] = {
        EGL_SURFACE_TYPE, EGL_WINDOW_BIT,
        EGL_RED_SIZE, 8,
        EGL_GREEN_SIZE, 8,
        EGL_BLUE_SIZE, 8,
        EGL_ALPHA_SIZE, alpha_size,
        EGL_DEPTH_SIZE, 16,
        EGL_STENCIL_SIZE, 0,
        EGL_NONE,
    };
    EGLConfig available_cfgs[32];
    EGLint cfg_count;
    eglChooseConfig(display, cfg_attributes, available_cfgs, 32, &cfg_count);
    SOKOL_ASSERT(cfg_count > 0);
    SOKOL_ASSERT(cfg_count <= 32);

    /* find config with 8-bit rgb buffer if available, ndk sample does not trust egl spec */
    EGLConfig config;
    bool exact_cfg_found = false;
    for (int i = 0; i < cfg_count; ++i) {
        EGLConfig c = available_cfgs[i];
        EGLint r, g, b, a, d;
        if (eglGetConfigAttrib(display, c, EGL_RED_SIZE, &r) == EGL_TRUE &&
            eglGetConfigAttrib(display, c, EGL_GREEN_SIZE, &g) == EGL_TRUE &&
            eglGetConfigAttrib(display, c, EGL_BLUE_SIZE, &b) == EGL_TRUE &&
            eglGetConfigAttrib(display, c, EGL_ALPHA_SIZE, &a) == EGL_TRUE &&
            eglGetConfigAttrib(display, c, EGL_DEPTH_SIZE, &d) == EGL_TRUE &&
            r == 8 && g == 8 && b == 8 && (alpha_size == 0 || a == alpha_size) && d == 16) {
            exact_cfg_found = true;
            config = c;
            break;
        }
    }
    if (!exact_cfg_found) {
        config = available_cfgs[0];
    }

    EGLint ctx_attributes[] = {
        #if defined(SOKOL_GLES3)
            EGL_CONTEXT_CLIENT_VERSION, _sapp.desc.gl_force_gles2 ? 2 : 3,
        #else
            EGL_CONTEXT_CLIENT_VERSION, 2,
        #endif
        EGL_NONE,
    };
    EGLContext context = eglCreateContext(display, config, EGL_NO_CONTEXT, ctx_attributes);
    if (context == EGL_NO_CONTEXT) {
        return false;
    }

    state->config = config;
    state->display = display;
    state->context = context;
    return true;
}

_SOKOL_PRIVATE void _sapp_android_cleanup_egl(void) {
    _sapp_android_state_t* state = &_sapp_android_state;
    if (state->display != EGL_NO_DISPLAY) {
        eglMakeCurrent(state->display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
        if (state->surface != EGL_NO_SURFACE) {
            SOKOL_LOG("Destroying egl surface");
            eglDestroySurface(state->display, state->surface);
            state->surface = EGL_NO_SURFACE;
        }
        if (state->context != EGL_NO_CONTEXT) {
            SOKOL_LOG("Destroying egl context");
            eglDestroyContext(state->display, state->context);
            state->context = EGL_NO_CONTEXT;
        }
        SOKOL_LOG("Terminating egl display");
        eglTerminate(state->display);
        state->display = EGL_NO_DISPLAY;
    }
}

_SOKOL_PRIVATE bool _sapp_android_init_egl_surface(ANativeWindow* window) {
    _sapp_android_state_t* state = &_sapp_android_state;
    SOKOL_ASSERT(state->display != EGL_NO_DISPLAY);
    SOKOL_ASSERT(state->context != EGL_NO_CONTEXT);
    SOKOL_ASSERT(state->surface == EGL_NO_SURFACE);
    SOKOL_ASSERT(window);

    /* TODO: set window flags */
    /* ANativeActivity_setWindowFlags(activity, AWINDOW_FLAG_KEEP_SCREEN_ON, 0); */

    /* create egl surface and make it current */
    EGLSurface surface = eglCreateWindowSurface(state->display, state->config, window, NULL);
    if (surface == EGL_NO_SURFACE) {
        return false;
    }
    if (eglMakeCurrent(state->display, surface, surface, state->context) == EGL_FALSE) {
        return false;
    }
    state->surface = surface;
    return true;
}

_SOKOL_PRIVATE void _sapp_android_cleanup_egl_surface(void) {
    _sapp_android_state_t* state = &_sapp_android_state;
    if (state->display == EGL_NO_DISPLAY) {
        return;
    }
    eglMakeCurrent(state->display, EGL_NO_SURFACE, EGL_NO_SURFACE, EGL_NO_CONTEXT);
    if (state->surface != EGL_NO_SURFACE) {
        eglDestroySurface(state->display, state->surface);
        state->surface = EGL_NO_SURFACE;
    }
}

_SOKOL_PRIVATE void _sapp_android_app_event(sapp_event_type type) {
    if (_sapp_events_enabled()) {
        _sapp_init_event(type);
        SOKOL_LOG("event_cb()");
        _sapp_call_event(&_sapp.event);
    }
}

_SOKOL_PRIVATE void _sapp_android_update_dimensions(ANativeWindow* window, bool force_update) {
    _sapp_android_state_t* state = &_sapp_android_state;
    SOKOL_ASSERT(state->display != EGL_NO_DISPLAY);
    SOKOL_ASSERT(state->context != EGL_NO_CONTEXT);
    SOKOL_ASSERT(state->surface != EGL_NO_SURFACE);
    SOKOL_ASSERT(window);

    const int32_t win_w = ANativeWindow_getWidth(window);
    const int32_t win_h = ANativeWindow_getHeight(window);
    SOKOL_ASSERT(win_w >= 0 && win_h >= 0);
    const bool win_changed = (win_w != _sapp.window_width) || (win_h != _sapp.window_height);
    _sapp.window_width = win_w;
    _sapp.window_height = win_h;
    if (win_changed || force_update) {
        if (!_sapp.desc.high_dpi) {
            const int32_t buf_w = win_w / 2;
            const int32_t buf_h = win_h / 2;
            EGLint format;
            EGLBoolean egl_result = eglGetConfigAttrib(state->display, state->config, EGL_NATIVE_VISUAL_ID, &format);
            SOKOL_ASSERT(egl_result == EGL_TRUE);
            /* NOTE: calling ANativeWindow_setBuffersGeometry() with the same dimensions
                as the ANativeWindow size results in weird display artefacts, that's
                why it's only called when the buffer geometry is different from
                the window size
            */
            int32_t result = ANativeWindow_setBuffersGeometry(window, buf_w, buf_h, format);
            SOKOL_ASSERT(result == 0);
        }
    }

    /* query surface size */
    EGLint fb_w, fb_h;
    EGLBoolean egl_result_w = eglQuerySurface(state->display, state->surface, EGL_WIDTH, &fb_w);
    EGLBoolean egl_result_h = eglQuerySurface(state->display, state->surface, EGL_HEIGHT, &fb_h);
    SOKOL_ASSERT(egl_result_w == EGL_TRUE);
    SOKOL_ASSERT(egl_result_h == EGL_TRUE);
    const bool fb_changed = (fb_w != _sapp.framebuffer_width) || (fb_h != _sapp.framebuffer_height);
    _sapp.framebuffer_width = fb_w;
    _sapp.framebuffer_height = fb_h;
    _sapp.dpi_scale = (float)_sapp.framebuffer_width / (float)_sapp.window_width;
    if (win_changed || fb_changed || force_update) {
        if (!_sapp.first_frame) {
            SOKOL_LOG("SAPP_EVENTTYPE_RESIZED");
            _sapp_android_app_event(SAPP_EVENTTYPE_RESIZED);
        }
    }
}

_SOKOL_PRIVATE void _sapp_android_cleanup(void) {
    _sapp_android_state_t* state = &_sapp_android_state;
    SOKOL_LOG("Cleaning up");
    if (state->surface != EGL_NO_SURFACE) {
        /* egl context is bound, cleanup gracefully */
        if (_sapp.init_called && !_sapp.cleanup_called) {
            SOKOL_LOG("cleanup_cb()");
            _sapp_call_cleanup();
        }
    }
    /* always try to cleanup by destroying egl context */
    _sapp_android_cleanup_egl();
}

_SOKOL_PRIVATE void _sapp_android_shutdown(void) {
    /* try to cleanup while we still have a surface and can call cleanup_cb() */
    _sapp_android_cleanup();
    /* request exit */
    ANativeActivity_finish(_sapp_android_state.activity);
}

_SOKOL_PRIVATE void _sapp_android_frame(void) {
    _sapp_android_state_t* state = &_sapp_android_state;
    SOKOL_ASSERT(state->display != EGL_NO_DISPLAY);
    SOKOL_ASSERT(state->context != EGL_NO_CONTEXT);
    SOKOL_ASSERT(state->surface != EGL_NO_SURFACE);
    _sapp_android_update_dimensions(state->current.window, false);
    _sapp_frame();
    eglSwapBuffers(state->display, _sapp_android_state.surface);
}

_SOKOL_PRIVATE bool _sapp_android_touch_event(const AInputEvent* e) {
    if (AInputEvent_getType(e) != AINPUT_EVENT_TYPE_MOTION) {
        return false;
    }
    if (!_sapp_events_enabled()) {
        return false;
    }
    int32_t action_idx = AMotionEvent_getAction(e);
    int32_t action = action_idx & AMOTION_EVENT_ACTION_MASK;
    sapp_event_type type = SAPP_EVENTTYPE_INVALID;
    switch (action) {
        case AMOTION_EVENT_ACTION_DOWN:
            SOKOL_LOG("Touch: down");
        case AMOTION_EVENT_ACTION_POINTER_DOWN:
            SOKOL_LOG("Touch: ptr down");
            type = SAPP_EVENTTYPE_TOUCHES_BEGAN;
            break;
        case AMOTION_EVENT_ACTION_MOVE:
            type = SAPP_EVENTTYPE_TOUCHES_MOVED;
            break;
        case AMOTION_EVENT_ACTION_UP:
            SOKOL_LOG("Touch: up");
        case AMOTION_EVENT_ACTION_POINTER_UP:
            SOKOL_LOG("Touch: ptr up");
            type = SAPP_EVENTTYPE_TOUCHES_ENDED;
            break;
        case AMOTION_EVENT_ACTION_CANCEL:
            SOKOL_LOG("Touch: cancel");
            type = SAPP_EVENTTYPE_TOUCHES_CANCELLED;
            break;
        default:
            break;
    }
    if (type == SAPP_EVENTTYPE_INVALID) {
        return false;
    }
    int32_t idx = action_idx >> AMOTION_EVENT_ACTION_POINTER_INDEX_SHIFT;
    _sapp_init_event(type);
    _sapp.event.num_touches = AMotionEvent_getPointerCount(e);
    if (_sapp.event.num_touches > SAPP_MAX_TOUCHPOINTS) {
        _sapp.event.num_touches = SAPP_MAX_TOUCHPOINTS;
    }
    for (int32_t i = 0; i < _sapp.event.num_touches; i++) {
        sapp_touchpoint* dst = &_sapp.event.touches[i];
        dst->identifier = AMotionEvent_getPointerId(e, i);
        dst->pos_x = (AMotionEvent_getRawX(e, i) / _sapp.window_width) * _sapp.framebuffer_width;
        dst->pos_y = (AMotionEvent_getRawY(e, i) / _sapp.window_height) * _sapp.framebuffer_height;

        if (action == AMOTION_EVENT_ACTION_POINTER_DOWN ||
            action == AMOTION_EVENT_ACTION_POINTER_UP) {
            dst->changed = i == idx;
        } else {
            dst->changed = true;
        }
    }
    _sapp_call_event(&_sapp.event);
    return true;
}

_SOKOL_PRIVATE bool _sapp_android_key_event(const AInputEvent* e) {
    if (AInputEvent_getType(e) != AINPUT_EVENT_TYPE_KEY) {
        return false;
    }
    if (AKeyEvent_getKeyCode(e) == AKEYCODE_BACK) {
        /* FIXME: this should be hooked into a "really quit?" mechanism
           so the app can ask the user for confirmation, this is currently
           generally missing in sokol_app.h
        */
        _sapp_android_shutdown();
        return true;
    }
    return false;
}

_SOKOL_PRIVATE int _sapp_android_input_cb(int fd, int events, void* data) {
    if ((events & ALOOPER_EVENT_INPUT) == 0) {
        SOKOL_LOG("_sapp_android_input_cb() encountered unsupported event");
        return 1;
    }
    _sapp_android_state_t* state = &_sapp_android_state;;
    SOKOL_ASSERT(state->current.input);
    AInputEvent* event = NULL;
    while (AInputQueue_getEvent(state->current.input, &event) >= 0) {
        if (AInputQueue_preDispatchEvent(state->current.input, event) != 0) {
            continue;
        }
        int32_t handled = 0;
        if (_sapp_android_touch_event(event) || _sapp_android_key_event(event)) {
            handled = 1;
        }
        AInputQueue_finishEvent(state->current.input, event, handled);
    }
    return 1;
}

_SOKOL_PRIVATE int _sapp_android_main_cb(int fd, int events, void* data) {
    if ((events & ALOOPER_EVENT_INPUT) == 0) {
        SOKOL_LOG("_sapp_android_main_cb() encountered unsupported event");
        return 1;
    }
    _sapp_android_state_t* state = &_sapp_android_state;

    _sapp_android_msg_t msg;
    if (read(fd, &msg, sizeof(msg)) != sizeof(msg)) {
        SOKOL_LOG("Could not write to read_from_main_fd");
        return 1;
    }

    pthread_mutex_lock(&state->pt.mutex);
    switch (msg) {
        case _SOKOL_ANDROID_MSG_CREATE:
            {
                SOKOL_LOG("MSG_CREATE");
                SOKOL_ASSERT(!_sapp.valid);
                bool result = _sapp_android_init_egl();
                SOKOL_ASSERT(result);
                _sapp.valid = true;
                state->has_created = true;
            }
            break;
        case _SOKOL_ANDROID_MSG_RESUME:
            SOKOL_LOG("MSG_RESUME");
            state->has_resumed = true;
            _sapp_android_app_event(SAPP_EVENTTYPE_RESUMED);
            break;
        case _SOKOL_ANDROID_MSG_PAUSE:
            SOKOL_LOG("MSG_PAUSE");
            state->has_resumed = false;
            _sapp_android_app_event(SAPP_EVENTTYPE_SUSPENDED);
            break;
        case _SOKOL_ANDROID_MSG_FOCUS:
            SOKOL_LOG("MSG_FOCUS");
            state->has_focus = true;
            break;
        case _SOKOL_ANDROID_MSG_NO_FOCUS:
            SOKOL_LOG("MSG_NO_FOCUS");
            state->has_focus = false;
            break;
        case _SOKOL_ANDROID_MSG_SET_NATIVE_WINDOW:
            SOKOL_LOG("MSG_SET_NATIVE_WINDOW");
            if (state->current.window != state->pending.window) {
                if (state->current.window != NULL) {
                    _sapp_android_cleanup_egl_surface();
                }
                if (state->pending.window != NULL) {
                    SOKOL_LOG("Creating egl surface ...");
                    if (_sapp_android_init_egl_surface(state->pending.window)) {
                        SOKOL_LOG("... ok!");
                        _sapp_android_update_dimensions(state->pending.window, true);
                    } else {
                        SOKOL_LOG("... failed!");
                        _sapp_android_shutdown();
                    }
                }
            }
            state->current.window = state->pending.window;
            break;
        case _SOKOL_ANDROID_MSG_SET_INPUT_QUEUE:
            SOKOL_LOG("MSG_SET_INPUT_QUEUE");
            if (state->current.input != state->pending.input) {
                if (state->current.input != NULL) {
                    AInputQueue_detachLooper(state->current.input);
                }
                if (state->pending.input != NULL) {
                    AInputQueue_attachLooper(
                        state->pending.input,
                        state->looper,
                        ALOOPER_POLL_CALLBACK,
                        _sapp_android_input_cb,
                        NULL); /* data */
                }
            }
            state->current.input = state->pending.input;
            break;
        case _SOKOL_ANDROID_MSG_DESTROY:
            SOKOL_LOG("MSG_DESTROY");
            _sapp_android_cleanup();
            _sapp.valid = false;
            state->is_thread_stopping = true;
            break;
        default:
            SOKOL_LOG("Unknown msg type received");
            break;
    }
    pthread_cond_broadcast(&state->pt.cond); /* signal "received" */
    pthread_mutex_unlock(&state->pt.mutex);
    return 1;
}

_SOKOL_PRIVATE bool _sapp_android_should_update(void) {
    bool is_in_front = _sapp_android_state.has_resumed && _sapp_android_state.has_focus;
    bool has_surface = _sapp_android_state.surface != EGL_NO_SURFACE;
    return is_in_front && has_surface;
}

_SOKOL_PRIVATE void _sapp_android_show_keyboard(bool shown) {
    SOKOL_ASSERT(_sapp.valid);
    /* This seems to be broken in the NDK, but there is (a very cumbersome) workaround... */
    if (shown) {
        SOKOL_LOG("Showing keyboard");
        ANativeActivity_showSoftInput(_sapp_android_state.activity, ANATIVEACTIVITY_SHOW_SOFT_INPUT_FORCED);
    } else {
        SOKOL_LOG("Hiding keyboard");
        ANativeActivity_hideSoftInput(_sapp_android_state.activity, ANATIVEACTIVITY_HIDE_SOFT_INPUT_NOT_ALWAYS);
    }
}

_SOKOL_PRIVATE void* _sapp_android_loop(void* obj) {
    SOKOL_LOG("Loop thread started");
    _sapp_android_state_t* state = (_sapp_android_state_t*)obj;

    state->looper = ALooper_prepare(0 /* or ALOOPER_PREPARE_ALLOW_NON_CALLBACKS*/);
    ALooper_addFd(state->looper,
        state->pt.read_from_main_fd,
        ALOOPER_POLL_CALLBACK,
        ALOOPER_EVENT_INPUT,
        _sapp_android_main_cb,
        NULL); /* data */

    /* signal start to main thread */
    pthread_mutex_lock(&state->pt.mutex);
    state->is_thread_started = true;
    pthread_cond_broadcast(&state->pt.cond);
    pthread_mutex_unlock(&state->pt.mutex);

    /* main loop */
    while (!state->is_thread_stopping) {
        /* sokol frame */
        if (_sapp_android_should_update()) {
            _sapp_android_frame();
        }

        /* process all events (or stop early if app is requested to quit) */
        bool process_events = true;
        while (process_events && !state->is_thread_stopping) {
            bool block_until_event = !state->is_thread_stopping && !_sapp_android_should_update();
            process_events = ALooper_pollOnce(block_until_event ? -1 : 0, NULL, NULL, NULL) == ALOOPER_POLL_CALLBACK;
        }
    }

    /* cleanup thread */
    if (state->current.input != NULL) {
        AInputQueue_detachLooper(state->current.input);
    }

    /* the following causes heap corruption on exit, why??
    ALooper_removeFd(state->looper, state->pt.read_from_main_fd);
    ALooper_release(state->looper);*/

    /* signal "destroyed" */
    pthread_mutex_lock(&state->pt.mutex);
    state->is_thread_stopped = true;
    pthread_cond_broadcast(&state->pt.cond);
    pthread_mutex_unlock(&state->pt.mutex);
    SOKOL_LOG("Loop thread done");
    return NULL;
}

/* android main/ui thread */
_SOKOL_PRIVATE void _sapp_android_msg(_sapp_android_state_t* state, _sapp_android_msg_t msg) {
    if (write(state->pt.write_from_main_fd, &msg, sizeof(msg)) != sizeof(msg)) {
        SOKOL_LOG("Could not write to write_from_main_fd");
    }
}

_SOKOL_PRIVATE void _sapp_android_on_start(ANativeActivity* activity) {
    SOKOL_LOG("NativeActivity onStart()");
}

_SOKOL_PRIVATE void _sapp_android_on_resume(ANativeActivity* activity) {
    SOKOL_LOG("NativeActivity onResume()");
    _sapp_android_msg(&_sapp_android_state, _SOKOL_ANDROID_MSG_RESUME);
}

_SOKOL_PRIVATE void* _sapp_android_on_save_instance_state(ANativeActivity* activity, size_t* out_size) {
    SOKOL_LOG("NativeActivity onSaveInstanceState()");
    *out_size = 0;
    return NULL;
}

_SOKOL_PRIVATE void _sapp_android_on_window_focus_changed(ANativeActivity* activity, int has_focus) {
    SOKOL_LOG("NativeActivity onWindowFocusChanged()");
    if (has_focus) {
        _sapp_android_msg(&_sapp_android_state, _SOKOL_ANDROID_MSG_FOCUS);
    } else {
        _sapp_android_msg(&_sapp_android_state, _SOKOL_ANDROID_MSG_NO_FOCUS);
    }
}

_SOKOL_PRIVATE void _sapp_android_on_pause(ANativeActivity* activity) {
    SOKOL_LOG("NativeActivity onPause()");
    _sapp_android_msg(&_sapp_android_state, _SOKOL_ANDROID_MSG_PAUSE);
}

_SOKOL_PRIVATE void _sapp_android_on_stop(ANativeActivity* activity) {
    SOKOL_LOG("NativeActivity onStop()");
}

_SOKOL_PRIVATE void _sapp_android_msg_set_native_window(_sapp_android_state_t* state, ANativeWindow* window) {
    pthread_mutex_lock(&state->pt.mutex);
    state->pending.window = window;
    _sapp_android_msg(state, _SOKOL_ANDROID_MSG_SET_NATIVE_WINDOW);
    while (state->current.window != window) {
        pthread_cond_wait(&state->pt.cond, &state->pt.mutex);
    }
    pthread_mutex_unlock(&state->pt.mutex);
}

_SOKOL_PRIVATE void _sapp_android_on_native_window_created(ANativeActivity* activity, ANativeWindow* window) {
    SOKOL_LOG("NativeActivity onNativeWindowCreated()");
    _sapp_android_msg_set_native_window(&_sapp_android_state, window);
}

_SOKOL_PRIVATE void _sapp_android_on_native_window_destroyed(ANativeActivity* activity, ANativeWindow* window) {
    SOKOL_LOG("NativeActivity onNativeWindowDestroyed()");
    _sapp_android_msg_set_native_window(&_sapp_android_state, NULL);
}

_SOKOL_PRIVATE void _sapp_android_msg_set_input_queue(_sapp_android_state_t* state, AInputQueue* input) {
    pthread_mutex_lock(&state->pt.mutex);
    state->pending.input = input;
    _sapp_android_msg(state, _SOKOL_ANDROID_MSG_SET_INPUT_QUEUE);
    while (state->current.input != input) {
        pthread_cond_wait(&state->pt.cond, &state->pt.mutex);
    }
    pthread_mutex_unlock(&state->pt.mutex);
}

_SOKOL_PRIVATE void _sapp_android_on_input_queue_created(ANativeActivity* activity, AInputQueue* queue) {
    SOKOL_LOG("NativeActivity onInputQueueCreated()");
    _sapp_android_msg_set_input_queue(&_sapp_android_state, queue);
}

_SOKOL_PRIVATE void _sapp_android_on_input_queue_destroyed(ANativeActivity* activity, AInputQueue* queue) {
    SOKOL_LOG("NativeActivity onInputQueueDestroyed()");
    _sapp_android_msg_set_input_queue(&_sapp_android_state, NULL);
}

_SOKOL_PRIVATE void _sapp_android_on_config_changed(ANativeActivity* activity) {
    SOKOL_LOG("NativeActivity onConfigurationChanged()");
    /* see android:configChanges in manifest */
}

_SOKOL_PRIVATE void _sapp_android_on_low_memory(ANativeActivity* activity) {
    SOKOL_LOG("NativeActivity onLowMemory()");
}

_SOKOL_PRIVATE void _sapp_android_on_destroy(ANativeActivity* activity) {
    /*
     * For some reason even an empty app using nativeactivity.h will crash (WIN DEATH)
     * on my device (Moto X 2nd gen) when the app is removed from the task view
     * (TaskStackView: onTaskViewDismissed).
     *
     * However, if ANativeActivity_finish() is explicitly called from for example
     * _sapp_android_on_stop(), the crash disappears. Is this a bug in NativeActivity?
     */
    SOKOL_LOG("NativeActivity onDestroy()");
    _sapp_android_state_t* state = &_sapp_android_state;

    /* send destroy msg */
    pthread_mutex_lock(&state->pt.mutex);
    _sapp_android_msg(state, _SOKOL_ANDROID_MSG_DESTROY);
    while (!_sapp_android_state.is_thread_stopped) {
        pthread_cond_wait(&state->pt.cond, &state->pt.mutex);
    }
    pthread_mutex_unlock(&state->pt.mutex);

    /* clean up main thread */
    pthread_cond_destroy(&state->pt.cond);
    pthread_mutex_destroy(&state->pt.mutex);

    close(state->pt.read_from_main_fd);
    close(state->pt.write_from_main_fd);

    SOKOL_LOG("NativeActivity done");

    /* this is a bit naughty, but causes a clean restart of the app (static globals are reset) */
    exit(0);
}

// will be filled by rust sapp_run call
sapp_desc _sapp_desc;

JNIEXPORT
void sapp_ANativeActivity_onCreate(void* activity0, void* saved_state, int saved_state_size) {
    ANativeActivity* activity = activity0;

    SOKOL_LOG("NativeActivity onCreate()");

    sokol_main(0, NULL);
    _sapp_init_state(&_sapp_desc);

    /* start loop thread */
    _sapp_android_state = (_sapp_android_state_t){0};
    _sapp_android_state_t* state = &_sapp_android_state;

    state->activity = activity;

    int pipe_fd[2];
    if (pipe(pipe_fd) != 0) {
        SOKOL_LOG("Could not create thread pipe");
        return;
    }
    state->pt.read_from_main_fd = pipe_fd[0];
    state->pt.write_from_main_fd = pipe_fd[1];

    pthread_mutex_init(&state->pt.mutex, NULL);
    pthread_cond_init(&state->pt.cond, NULL);

    pthread_attr_t attr;
    pthread_attr_init(&attr);
    pthread_attr_setdetachstate(&attr, PTHREAD_CREATE_DETACHED);
    pthread_create(&state->pt.thread, &attr, _sapp_android_loop, state);
    pthread_attr_destroy(&attr);

    /* wait until main loop has started */
    pthread_mutex_lock(&state->pt.mutex);
    while (!state->is_thread_started) {
        pthread_cond_wait(&state->pt.cond, &state->pt.mutex);
    }
    pthread_mutex_unlock(&state->pt.mutex);

    /* send create msg */
    pthread_mutex_lock(&state->pt.mutex);
    _sapp_android_msg(state, _SOKOL_ANDROID_MSG_CREATE);
    while (!state->has_created) {
        pthread_cond_wait(&state->pt.cond, &state->pt.mutex);
    }
    pthread_mutex_unlock(&state->pt.mutex);

    /* register for callbacks */
    activity->instance = state;
    activity->callbacks->onStart = _sapp_android_on_start;
    activity->callbacks->onResume = _sapp_android_on_resume;
    activity->callbacks->onSaveInstanceState = _sapp_android_on_save_instance_state;
    activity->callbacks->onWindowFocusChanged = _sapp_android_on_window_focus_changed;
    activity->callbacks->onPause = _sapp_android_on_pause;
    activity->callbacks->onStop = _sapp_android_on_stop;
    activity->callbacks->onDestroy = _sapp_android_on_destroy;
    activity->callbacks->onNativeWindowCreated = _sapp_android_on_native_window_created;
    /* activity->callbacks->onNativeWindowResized = _sapp_android_on_native_window_resized; */
    /* activity->callbacks->onNativeWindowRedrawNeeded = _sapp_android_on_native_window_redraw_needed; */
    activity->callbacks->onNativeWindowDestroyed = _sapp_android_on_native_window_destroyed;
    activity->callbacks->onInputQueueCreated = _sapp_android_on_input_queue_created;
    activity->callbacks->onInputQueueDestroyed = _sapp_android_on_input_queue_destroyed;
    /* activity->callbacks->onContentRectChanged = _sapp_android_on_content_rect_changed; */
    activity->callbacks->onConfigurationChanged = _sapp_android_on_config_changed;
    activity->callbacks->onLowMemory = _sapp_android_on_low_memory;

    SOKOL_LOG("NativeActivity successfully created");
}

#endif /* Android */

/* likewise, in normal mode, sapp_run() is just an empty stub */
SOKOL_API_IMPL int sapp_run(const sapp_desc* desc) {
    _sapp_desc = *desc;
    return 0;
}

SOKOL_API_IMPL bool sapp_isvalid(void) {
    return _sapp.valid;
}

SOKOL_API_IMPL void* sapp_userdata(void) {
    return _sapp.desc.user_data;
}

SOKOL_API_IMPL sapp_desc sapp_query_desc(void) {
    return _sapp.desc;
}

SOKOL_API_IMPL uint64_t sapp_frame_count(void) {
    return _sapp.frame_count;
}

SOKOL_API_IMPL int sapp_width(void) {
    return (_sapp.framebuffer_width > 0) ? _sapp.framebuffer_width : 1;
}

SOKOL_API_IMPL int sapp_height(void) {
    return (_sapp.framebuffer_height > 0) ? _sapp.framebuffer_height : 1;
}

SOKOL_API_IMPL bool sapp_high_dpi(void) {
    return _sapp.desc.high_dpi && (_sapp.dpi_scale > 1.5f);
}

SOKOL_API_IMPL float sapp_dpi_scale(void) {
    return _sapp.dpi_scale;
}

SOKOL_API_IMPL bool sapp_gles2(void) {
    return _sapp.gles2_fallback;
}

SOKOL_API_IMPL void sapp_show_keyboard(bool shown) {
    #if defined(TARGET_OS_IPHONE) && TARGET_OS_IPHONE
    _sapp_ios_show_keyboard(shown);
    #elif defined(__EMSCRIPTEN__)
    _sapp_emsc_show_keyboard(shown);
    #elif defined(__ANDROID__)
    _sapp_android_show_keyboard(shown);
    #else
    _SOKOL_UNUSED(shown);
    #endif
}

SOKOL_API_IMPL bool sapp_keyboard_shown(void) {
    return _sapp.onscreen_keyboard_shown;
}

SOKOL_API_IMPL void sapp_set_cursor_grab(bool grab) {
    _SOKOL_UNUSED(grab);
}

SOKOL_API_IMPL void sapp_show_mouse(bool shown) {
    #if defined(_WIN32)
    _sapp_win32_show_mouse(shown);
    #else
    _SOKOL_UNUSED(shown);
    #endif
}

SOKOL_API_IMPL bool sapp_mouse_shown(void) {
    #if defined(_WIN32)
    return _sapp_win32_mouse_shown();
    #else
    return false;
    #endif
}

SOKOL_API_IMPL void sapp_request_quit(void) {
    _sapp.quit_requested = true;
}

SOKOL_API_IMPL void sapp_cancel_quit(void) {
    _sapp.quit_requested = false;
}

SOKOL_API_IMPL void sapp_quit(void) {
    _sapp.quit_ordered = true;
}

SOKOL_API_IMPL const void* sapp_metal_get_device(void) {
    SOKOL_ASSERT(_sapp.valid);
    #if defined(SOKOL_METAL)
        const void* obj = (__bridge const void*) _sapp_mtl_device_obj;
        SOKOL_ASSERT(obj);
        return obj;
    #else
        return 0;
    #endif
}

SOKOL_API_IMPL const void* sapp_metal_get_renderpass_descriptor(void) {
    SOKOL_ASSERT(_sapp.valid);
    #if defined(SOKOL_METAL)
        const void* obj =  (__bridge const void*) [_sapp_view_obj currentRenderPassDescriptor];
        SOKOL_ASSERT(obj);
        return obj;
    #else
        return 0;
    #endif
}

SOKOL_API_IMPL const void* sapp_metal_get_drawable(void) {
    SOKOL_ASSERT(_sapp.valid);
    #if defined(SOKOL_METAL)
        const void* obj = (__bridge const void*) [_sapp_view_obj currentDrawable];
        SOKOL_ASSERT(obj);
        return obj;
    #else
        return 0;
    #endif
}

SOKOL_API_IMPL const void* sapp_macos_get_window(void) {
    #if defined(__APPLE__) && !TARGET_OS_IPHONE
        const void* obj = (__bridge const void*) _sapp_macos_window_obj;
        SOKOL_ASSERT(obj);
        return obj;
    #else
        return 0;
    #endif
}

SOKOL_API_IMPL const void* sapp_ios_get_window(void) {
    #if defined(__APPLE__) && TARGET_OS_IPHONE
        const void* obj = (__bridge const void*) _sapp_ios_window_obj;
        SOKOL_ASSERT(obj);
        return obj;
    #else
        return 0;
    #endif

}

SOKOL_API_IMPL const void* sapp_d3d11_get_device(void) {
    SOKOL_ASSERT(_sapp.valid);
    #if defined(SOKOL_D3D11)
        return _sapp_d3d11_device;
    #else
        return 0;
    #endif
}

SOKOL_API_IMPL const void* sapp_d3d11_get_device_context(void) {
    SOKOL_ASSERT(_sapp.valid);
    #if defined(SOKOL_D3D11)
        return _sapp_d3d11_device_context;
    #else
        return 0;
    #endif
}

SOKOL_API_IMPL const void* sapp_d3d11_get_render_target_view(void) {
    SOKOL_ASSERT(_sapp.valid);
    #if defined(SOKOL_D3D11)
        return _sapp_d3d11_rtv;
    #else
        return 0;
    #endif
}

SOKOL_API_IMPL const void* sapp_d3d11_get_depth_stencil_view(void) {
    SOKOL_ASSERT(_sapp.valid);
    #if defined(SOKOL_D3D11)
        return _sapp_d3d11_dsv;
    #else
        return 0;
    #endif
}

SOKOL_API_IMPL const void* sapp_win32_get_hwnd(void) {
    SOKOL_ASSERT(_sapp.valid);
    #if defined(_WIN32)
        return _sapp_win32_hwnd;
    #else
        return 0;
    #endif
}

SOKOL_API_IMPL const void* sapp_android_get_native_activity(void) {
    SOKOL_ASSERT(_sapp.valid);
    #if defined(__ANDROID__)
        return (void*)_sapp_android_state.activity;
    #else
        return 0;
    #endif
}

SOKOL_API_IMPL void sapp_html5_ask_leave_site(bool ask) {
    _sapp.html5_ask_leave_site = ask;
}

SOKOL_API_IMPL void sapp_android_log(const char *message) {
    #include <android/log.h>
    __android_log_write(ANDROID_LOG_INFO, "SOKOL_APP", message);
}

SOKOL_API_IMPL void sapp_load_asset(const char* filepath, android_asset *out) {
    #include <android/asset_manager.h>
    #include <android/asset_manager_jni.h>

    AAssetManager * mgr = _sapp_android_state.activity->assetManager;

    AAsset *asset = AAssetManager_open(mgr, filepath, AASSET_MODE_BUFFER);

    if (asset == NULL) {
        return;
    }

    off64_t length = AAsset_getLength64(asset);
    char *buffer = malloc(length);
    if (AAsset_read(asset, buffer, length) > 0) {
         AAsset_close(asset);

         out->content_length = length;
         out->content = buffer;
    }
}
#undef _sapp_def

#ifdef _MSC_VER
#pragma warning(pop)
#endif

#endif /* SOKOL_IMPL */
