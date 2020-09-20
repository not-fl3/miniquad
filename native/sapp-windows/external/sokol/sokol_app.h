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
extern sapp_desc sokol_main(int argc, char* argv[]);

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

#include <string.h> /* memset */

/* check if the config defines are alright */
#if defined(__APPLE__)
    #if !__has_feature(objc_arc)
        #error "sokol_app.h requires ARC (Automatic Reference Counting) on MacOS and iOS"
    #endif
    #include <TargetConditionals.h>
    #if defined(TARGET_OS_IPHONE) && TARGET_OS_IPHONE
        /* iOS */
        #if !defined(SOKOL_METAL) && !defined(SOKOL_GLES3)
        #error("sokol_app.h: unknown 3D API selected for iOS, must be SOKOL_METAL or SOKOL_GLES3")
        #endif
    #else
        /* MacOS */
        #if !defined(SOKOL_METAL) && !defined(SOKOL_GLCORE33)
        #error("sokol_app.h: unknown 3D API selected for MacOS, must be SOKOL_METAL or SOKOL_GLCORE33")
        #endif
    #endif
#elif defined(__EMSCRIPTEN__)
    /* emscripten (asm.js or wasm) */
    #if !defined(SOKOL_GLES3) && !defined(SOKOL_GLES2)
    #error("sokol_app.h: unknown 3D API selected for emscripten, must be SOKOL_GLES3 or SOKOL_GLES2")
    #endif
#elif defined(_WIN32)
    /* Windows (D3D11 or GL) */
    #if !defined(SOKOL_D3D11) && !defined(SOKOL_GLCORE33)
    #error("sokol_app.h: unknown 3D API selected for Win32, must be SOKOL_D3D11 or SOKOL_GLCORE33")
    #endif
#elif defined(__ANDROID__)
    /* Android */
    #if !defined(SOKOL_GLES3) && !defined(SOKOL_GLES2)
    #error("sokol_app.h: unknown 3D API selected for Android, must be SOKOL_GLES3 or SOKOL_GLES2")
    #endif
    #if defined(SOKOL_NO_ENTRY)
    #error("sokol_app.h: SOKOL_NO_ENTRY is not supported on Android")
    #endif
#elif defined(__linux__) || defined(__unix__)
    /* Linux */
    #if !defined(SOKOL_GLCORE33)
    #error("sokol_app.h: unknown 3D API selected for Linux, must be SOKOL_GLCORE33")
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

/*== WINDOWS ==================================================================*/
#if defined(_WIN32)
#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif
#include <windows.h>
#include <windowsx.h>
#include <shellapi.h>
#pragma comment (lib, "Shell32.lib")

#if defined(SOKOL_D3D11)
#ifndef D3D11_NO_HELPERS
#define D3D11_NO_HELPERS
#endif
#ifndef CINTERFACE
#define CINTERFACE
#endif
#ifndef COBJMACROS
#define COBJMACROS
#endif
#include <windows.h>
#include <d3d11.h>
#include <dxgi.h>
#if (defined(WINAPI_FAMILY_PARTITION) && !WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP))
#pragma comment (lib, "WindowsApp.lib")
#else
#pragma comment (lib, "user32.lib")
#pragma comment (lib, "dxgi.lib")
#pragma comment (lib, "d3d11.lib")
#pragma comment (lib, "dxguid.lib")
#endif
#endif

/* see https://github.com/floooh/sokol/issues/138 */
#ifndef WM_MOUSEHWHEEL
#define WM_MOUSEHWHEEL (0x020E)
#endif

#ifndef DPI_ENUMS_DECLARED
typedef enum PROCESS_DPI_AWARENESS
{
    PROCESS_DPI_UNAWARE = 0,
    PROCESS_SYSTEM_DPI_AWARE = 1,
    PROCESS_PER_MONITOR_DPI_AWARE = 2
} PROCESS_DPI_AWARENESS;
typedef enum MONITOR_DPI_TYPE {
    MDT_EFFECTIVE_DPI = 0,
    MDT_ANGULAR_DPI = 1,
    MDT_RAW_DPI = 2,
    MDT_DEFAULT = MDT_EFFECTIVE_DPI
} MONITOR_DPI_TYPE;
#endif /*DPI_ENUMS_DECLARED*/

static HWND _sapp_win32_hwnd;
static HDC _sapp_win32_dc;
static bool _sapp_win32_in_create_window;
static bool _sapp_win32_dpi_aware;
static float _sapp_win32_content_scale;
static float _sapp_win32_window_scale;
static float _sapp_win32_mouse_scale;
static bool _sapp_win32_iconified;
typedef BOOL(WINAPI * SETPROCESSDPIAWARE_T)(void);
typedef HRESULT(WINAPI * SETPROCESSDPIAWARENESS_T)(PROCESS_DPI_AWARENESS);
typedef HRESULT(WINAPI * GETDPIFORMONITOR_T)(HMONITOR, MONITOR_DPI_TYPE, UINT*, UINT*);
static SETPROCESSDPIAWARE_T _sapp_win32_setprocessdpiaware;
static SETPROCESSDPIAWARENESS_T _sapp_win32_setprocessdpiawareness;
static GETDPIFORMONITOR_T _sapp_win32_getdpiformonitor;
#if defined(SOKOL_D3D11)
static ID3D11Device* _sapp_d3d11_device;
static ID3D11DeviceContext* _sapp_d3d11_device_context;
static DXGI_SWAP_CHAIN_DESC _sapp_dxgi_swap_chain_desc;
static IDXGISwapChain* _sapp_dxgi_swap_chain;
static ID3D11Texture2D* _sapp_d3d11_rt;
static ID3D11RenderTargetView* _sapp_d3d11_rtv;
static ID3D11Texture2D* _sapp_d3d11_ds;
static ID3D11DepthStencilView* _sapp_d3d11_dsv;
#endif
#define WGL_NUMBER_PIXEL_FORMATS_ARB 0x2000
#define WGL_SUPPORT_OPENGL_ARB 0x2010
#define WGL_DRAW_TO_WINDOW_ARB 0x2001
#define WGL_PIXEL_TYPE_ARB 0x2013
#define WGL_TYPE_RGBA_ARB 0x202b
#define WGL_ACCELERATION_ARB 0x2003
#define WGL_NO_ACCELERATION_ARB 0x2025
#define WGL_RED_BITS_ARB 0x2015
#define WGL_RED_SHIFT_ARB 0x2016
#define WGL_GREEN_BITS_ARB 0x2017
#define WGL_GREEN_SHIFT_ARB 0x2018
#define WGL_BLUE_BITS_ARB 0x2019
#define WGL_BLUE_SHIFT_ARB 0x201a
#define WGL_ALPHA_BITS_ARB 0x201b
#define WGL_ALPHA_SHIFT_ARB 0x201c
#define WGL_ACCUM_BITS_ARB 0x201d
#define WGL_ACCUM_RED_BITS_ARB 0x201e
#define WGL_ACCUM_GREEN_BITS_ARB 0x201f
#define WGL_ACCUM_BLUE_BITS_ARB 0x2020
#define WGL_ACCUM_ALPHA_BITS_ARB 0x2021
#define WGL_DEPTH_BITS_ARB 0x2022
#define WGL_STENCIL_BITS_ARB 0x2023
#define WGL_AUX_BUFFERS_ARB 0x2024
#define WGL_STEREO_ARB 0x2012
#define WGL_DOUBLE_BUFFER_ARB 0x2011
#define WGL_SAMPLES_ARB 0x2042
#define WGL_FRAMEBUFFER_SRGB_CAPABLE_ARB 0x20a9
#define WGL_CONTEXT_DEBUG_BIT_ARB 0x00000001
#define WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB 0x00000002
#define WGL_CONTEXT_PROFILE_MASK_ARB 0x9126
#define WGL_CONTEXT_CORE_PROFILE_BIT_ARB 0x00000001
#define WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB 0x00000002
#define WGL_CONTEXT_MAJOR_VERSION_ARB 0x2091
#define WGL_CONTEXT_MINOR_VERSION_ARB 0x2092
#define WGL_CONTEXT_FLAGS_ARB 0x2094
#define WGL_CONTEXT_ROBUST_ACCESS_BIT_ARB 0x00000004
#define WGL_LOSE_CONTEXT_ON_RESET_ARB 0x8252
#define WGL_CONTEXT_RESET_NOTIFICATION_STRATEGY_ARB 0x8256
#define WGL_NO_RESET_NOTIFICATION_ARB 0x8261
#define WGL_CONTEXT_RELEASE_BEHAVIOR_ARB 0x2097
#define WGL_CONTEXT_RELEASE_BEHAVIOR_NONE_ARB 0
#define WGL_CONTEXT_RELEASE_BEHAVIOR_FLUSH_ARB 0x2098
#define WGL_COLORSPACE_EXT 0x309d
#define WGL_COLORSPACE_SRGB_EXT 0x3089
#define ERROR_INVALID_VERSION_ARB 0x2095
#define ERROR_INVALID_PROFILE_ARB 0x2096
#define ERROR_INCOMPATIBLE_DEVICE_CONTEXTS_ARB 0x2054
typedef BOOL (WINAPI * PFNWGLSWAPINTERVALEXTPROC)(int);
typedef BOOL (WINAPI * PFNWGLGETPIXELFORMATATTRIBIVARBPROC)(HDC,int,int,UINT,const int*,int*);
typedef const char* (WINAPI * PFNWGLGETEXTENSIONSSTRINGEXTPROC)(void);
typedef const char* (WINAPI * PFNWGLGETEXTENSIONSSTRINGARBPROC)(HDC);
typedef HGLRC (WINAPI * PFNWGLCREATECONTEXTATTRIBSARBPROC)(HDC,HGLRC,const int*);
typedef HGLRC (WINAPI * PFN_wglCreateContext)(HDC);
typedef BOOL (WINAPI * PFN_wglDeleteContext)(HGLRC);
typedef PROC (WINAPI * PFN_wglGetProcAddress)(LPCSTR);
typedef HDC (WINAPI * PFN_wglGetCurrentDC)(void);
typedef BOOL (WINAPI * PFN_wglMakeCurrent)(HDC,HGLRC);
static HINSTANCE _sapp_opengl32;
static HGLRC _sapp_gl_ctx;
static PFN_wglCreateContext _sapp_wglCreateContext;
static PFN_wglDeleteContext _sapp_wglDeleteContext;
static PFN_wglGetProcAddress _sapp_wglGetProcAddress;
static PFN_wglGetCurrentDC _sapp_wglGetCurrentDC;
static PFN_wglMakeCurrent _sapp_wglMakeCurrent;
static PFNWGLSWAPINTERVALEXTPROC _sapp_SwapIntervalEXT;
static PFNWGLGETPIXELFORMATATTRIBIVARBPROC _sapp_GetPixelFormatAttribivARB;
static PFNWGLGETEXTENSIONSSTRINGEXTPROC _sapp_GetExtensionsStringEXT;
static PFNWGLGETEXTENSIONSSTRINGARBPROC _sapp_GetExtensionsStringARB;
static PFNWGLCREATECONTEXTATTRIBSARBPROC _sapp_CreateContextAttribsARB;
static bool _sapp_ext_swap_control;
static bool _sapp_arb_multisample;
static bool _sapp_arb_pixel_format;
static bool _sapp_arb_create_context;
static bool _sapp_arb_create_context_profile;
static HWND _sapp_win32_msg_hwnd;
static HDC _sapp_win32_msg_dc;

/* NOTE: the optional GL loader only contains the GL constants and functions required for sokol_gfx.h, if you need
more, you'll need to use you own gl header-generator/loader
*/
#if !defined(SOKOL_WIN32_NO_GL_LOADER)
#if defined(SOKOL_GLCORE33)
#define __gl_h_ 1
#define __gl32_h_ 1
#define __gl31_h_ 1
#define __GL_H__ 1
#define __glext_h_ 1
#define __GLEXT_H_ 1
#define __gltypes_h_ 1
#define __glcorearb_h_ 1
#define __gl_glcorearb_h_ 1
#define GL_APIENTRY APIENTRY

typedef unsigned int  GLenum;
typedef unsigned int  GLuint;
typedef int  GLsizei;
typedef char  GLchar;
typedef ptrdiff_t  GLintptr;
typedef signed   long long int  GLsizeiptr;
typedef double  GLclampd;
typedef unsigned short  GLushort;
typedef unsigned char  GLubyte;
typedef unsigned char  GLboolean;
typedef uint64_t  GLuint64;
typedef double  GLdouble;
typedef unsigned short  GLhalf;
typedef float  GLclampf;
typedef unsigned int  GLbitfield;
typedef signed char  GLbyte;
typedef short  GLshort;
typedef void  GLvoid;
typedef int64_t  GLint64;
typedef float  GLfloat;
typedef struct __GLsync * GLsync;
typedef int  GLint;
#define GL_INT_2_10_10_10_REV 0x8D9F
#define GL_R32F 0x822E
#define GL_PROGRAM_POINT_SIZE 0x8642
#define GL_STENCIL_ATTACHMENT 0x8D20
#define GL_DEPTH_ATTACHMENT 0x8D00
#define GL_COLOR_ATTACHMENT2 0x8CE2
#define GL_COLOR_ATTACHMENT0 0x8CE0
#define GL_R16F 0x822D
#define GL_COLOR_ATTACHMENT22 0x8CF6
#define GL_DRAW_FRAMEBUFFER 0x8CA9
#define GL_FRAMEBUFFER_COMPLETE 0x8CD5
#define GL_NUM_EXTENSIONS 0x821D
#define GL_INFO_LOG_LENGTH 0x8B84
#define GL_VERTEX_SHADER 0x8B31
#define GL_INCR 0x1E02
#define GL_DYNAMIC_DRAW 0x88E8
#define GL_STATIC_DRAW 0x88E4
#define GL_TEXTURE_CUBE_MAP_POSITIVE_Z 0x8519
#define GL_TEXTURE_CUBE_MAP 0x8513
#define GL_FUNC_SUBTRACT 0x800A
#define GL_FUNC_REVERSE_SUBTRACT 0x800B
#define GL_CONSTANT_COLOR 0x8001
#define GL_DECR_WRAP 0x8508
#define GL_R8 0x8229
#define GL_LINEAR_MIPMAP_LINEAR 0x2703
#define GL_ELEMENT_ARRAY_BUFFER 0x8893
#define GL_SHORT 0x1402
#define GL_DEPTH_TEST 0x0B71
#define GL_TEXTURE_CUBE_MAP_NEGATIVE_Y 0x8518
#define GL_LINK_STATUS 0x8B82
#define GL_TEXTURE_CUBE_MAP_POSITIVE_Y 0x8517
#define GL_SAMPLE_ALPHA_TO_COVERAGE 0x809E
#define GL_RGBA16F 0x881A
#define GL_CONSTANT_ALPHA 0x8003
#define GL_READ_FRAMEBUFFER 0x8CA8
#define GL_TEXTURE0 0x84C0
#define GL_TEXTURE_MIN_LOD 0x813A
#define GL_CLAMP_TO_EDGE 0x812F
#define GL_UNSIGNED_SHORT_5_6_5 0x8363
#define GL_TEXTURE_WRAP_R 0x8072
#define GL_UNSIGNED_SHORT_5_5_5_1 0x8034
#define GL_NEAREST_MIPMAP_NEAREST 0x2700
#define GL_UNSIGNED_SHORT_4_4_4_4 0x8033
#define GL_SRC_ALPHA_SATURATE 0x0308
#define GL_STREAM_DRAW 0x88E0
#define GL_ONE 1
#define GL_NEAREST_MIPMAP_LINEAR 0x2702
#define GL_RGB10_A2 0x8059
#define GL_RGBA8 0x8058
#define GL_COLOR_ATTACHMENT1 0x8CE1
#define GL_RGBA4 0x8056
#define GL_RGB8 0x8051
#define GL_ARRAY_BUFFER 0x8892
#define GL_STENCIL 0x1802
#define GL_TEXTURE_2D 0x0DE1
#define GL_DEPTH 0x1801
#define GL_FRONT 0x0404
#define GL_STENCIL_BUFFER_BIT 0x00000400
#define GL_REPEAT 0x2901
#define GL_RGBA 0x1908
#define GL_TEXTURE_CUBE_MAP_POSITIVE_X 0x8515
#define GL_DECR 0x1E03
#define GL_FRAGMENT_SHADER 0x8B30
#define GL_FLOAT 0x1406
#define GL_TEXTURE_MAX_LOD 0x813B
#define GL_DEPTH_COMPONENT 0x1902
#define GL_ONE_MINUS_DST_ALPHA 0x0305
#define GL_COLOR 0x1800
#define GL_TEXTURE_2D_ARRAY 0x8C1A
#define GL_TRIANGLES 0x0004
#define GL_UNSIGNED_BYTE 0x1401
#define GL_TEXTURE_MAG_FILTER 0x2800
#define GL_ONE_MINUS_CONSTANT_ALPHA 0x8004
#define GL_NONE 0
#define GL_SRC_COLOR 0x0300
#define GL_BYTE 0x1400
#define GL_TEXTURE_CUBE_MAP_NEGATIVE_Z 0x851A
#define GL_LINE_STRIP 0x0003
#define GL_TEXTURE_3D 0x806F
#define GL_CW 0x0900
#define GL_LINEAR 0x2601
#define GL_RENDERBUFFER 0x8D41
#define GL_GEQUAL 0x0206
#define GL_COLOR_BUFFER_BIT 0x00004000
#define GL_RGBA32F 0x8814
#define GL_BLEND 0x0BE2
#define GL_ONE_MINUS_SRC_ALPHA 0x0303
#define GL_ONE_MINUS_CONSTANT_COLOR 0x8002
#define GL_TEXTURE_WRAP_T 0x2803
#define GL_TEXTURE_WRAP_S 0x2802
#define GL_TEXTURE_MIN_FILTER 0x2801
#define GL_LINEAR_MIPMAP_NEAREST 0x2701
#define GL_EXTENSIONS 0x1F03
#define GL_NO_ERROR 0
#define GL_REPLACE 0x1E01
#define GL_KEEP 0x1E00
#define GL_CCW 0x0901
#define GL_TEXTURE_CUBE_MAP_NEGATIVE_X 0x8516
#define GL_RGB 0x1907
#define GL_TRIANGLE_STRIP 0x0005
#define GL_FALSE 0
#define GL_ZERO 0
#define GL_CULL_FACE 0x0B44
#define GL_INVERT 0x150A
#define GL_INT 0x1404
#define GL_UNSIGNED_INT 0x1405
#define GL_UNSIGNED_SHORT 0x1403
#define GL_NEAREST 0x2600
#define GL_SCISSOR_TEST 0x0C11
#define GL_LEQUAL 0x0203
#define GL_STENCIL_TEST 0x0B90
#define GL_DITHER 0x0BD0
#define GL_DEPTH_COMPONENT16 0x81A5
#define GL_EQUAL 0x0202
#define GL_FRAMEBUFFER 0x8D40
#define GL_RGB5 0x8050
#define GL_LINES 0x0001
#define GL_DEPTH_BUFFER_BIT 0x00000100
#define GL_SRC_ALPHA 0x0302
#define GL_INCR_WRAP 0x8507
#define GL_LESS 0x0201
#define GL_MULTISAMPLE 0x809D
#define GL_FRAMEBUFFER_BINDING 0x8CA6
#define GL_BACK 0x0405
#define GL_ALWAYS 0x0207
#define GL_FUNC_ADD 0x8006
#define GL_ONE_MINUS_DST_COLOR 0x0307
#define GL_NOTEQUAL 0x0205
#define GL_DST_COLOR 0x0306
#define GL_COMPILE_STATUS 0x8B81
#define GL_RED 0x1903
#define GL_COLOR_ATTACHMENT3 0x8CE3
#define GL_DST_ALPHA 0x0304
#define GL_RGB5_A1 0x8057
#define GL_GREATER 0x0204
#define GL_POLYGON_OFFSET_FILL 0x8037
#define GL_TRUE 1
#define GL_NEVER 0x0200
#define GL_POINTS 0x0000
#define GL_ONE_MINUS_SRC_COLOR 0x0301
#define GL_MIRRORED_REPEAT 0x8370
#define GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS 0x8B4D
#define GL_R11F_G11F_B10F 0x8C3A
#define GL_UNSIGNED_INT_10F_11F_11F_REV 0x8C3B
#define GL_RGBA32UI 0x8D70
#define GL_RGB32UI 0x8D71
#define GL_RGBA16UI 0x8D76
#define GL_RGB16UI 0x8D77
#define GL_RGBA8UI 0x8D7C
#define GL_RGB8UI 0x8D7D
#define GL_RGBA32I 0x8D82
#define GL_RGB32I 0x8D83
#define GL_RGBA16I 0x8D88
#define GL_RGB16I 0x8D89
#define GL_RGBA8I 0x8D8E
#define GL_RGB8I 0x8D8F
#define GL_RED_INTEGER 0x8D94
#define GL_RG 0x8227
#define GL_RG_INTEGER 0x8228
#define GL_R8 0x8229
#define GL_R16 0x822A
#define GL_RG8 0x822B
#define GL_RG16 0x822C
#define GL_R16F 0x822D
#define GL_R32F 0x822E
#define GL_RG16F 0x822F
#define GL_RG32F 0x8230
#define GL_R8I 0x8231
#define GL_R8UI 0x8232
#define GL_R16I 0x8233
#define GL_R16UI 0x8234
#define GL_R32I 0x8235
#define GL_R32UI 0x8236
#define GL_RG8I 0x8237
#define GL_RG8UI 0x8238
#define GL_RG16I 0x8239
#define GL_RG16UI 0x823A
#define GL_RG32I 0x823B
#define GL_RG32UI 0x823C
#define GL_RGBA_INTEGER 0x8D99
#define GL_R8_SNORM 0x8F94
#define GL_RG8_SNORM 0x8F95
#define GL_RGB8_SNORM 0x8F96
#define GL_RGBA8_SNORM 0x8F97
#define GL_R16_SNORM 0x8F98
#define GL_RG16_SNORM 0x8F99
#define GL_RGB16_SNORM 0x8F9A
#define GL_RGBA16_SNORM 0x8F9B
#define GL_RGBA16 0x805B
#define GL_MAX_TEXTURE_SIZE 0x0D33
#define GL_MAX_CUBE_MAP_TEXTURE_SIZE 0x851C
#define GL_MAX_3D_TEXTURE_SIZE 0x8073
#define GL_MAX_ARRAY_TEXTURE_LAYERS 0x88FF
#define GL_MAX_VERTEX_ATTRIBS 0x8869
#define GL_CLAMP_TO_BORDER 0x812D
#define GL_TEXTURE_BORDER_COLOR 0x1004

#define GL_TEXTURE_2D 0x0DE1

typedef void  (GL_APIENTRY *PFN_glBindVertexArray)(GLuint array);
static PFN_glBindVertexArray _sapp_glBindVertexArray;
void glBindVertexArray(GLuint array) {
    _sapp_glBindVertexArray(array);
}
typedef void  (GL_APIENTRY *PFN_glFramebufferTextureLayer)(GLenum target, GLenum attachment, GLuint texture, GLint level, GLint layer);
static PFN_glFramebufferTextureLayer _sapp_glFramebufferTextureLayer;
typedef void  (GL_APIENTRY *PFN_glGenFramebuffers)(GLsizei n, GLuint * framebuffers);
static PFN_glGenFramebuffers _sapp_glGenFramebuffers;
void glGenFramebuffers(GLsizei n, GLuint * framebuffers) {
    _sapp_glGenFramebuffers(n, framebuffers);
}
typedef void  (GL_APIENTRY *PFN_glBindFramebuffer)(GLenum target, GLuint framebuffer);
static PFN_glBindFramebuffer _sapp_glBindFramebuffer;
void glBindFramebuffer(GLenum target, GLuint framebuffer) {
    _sapp_glBindFramebuffer(target, framebuffer);
}
typedef void  (GL_APIENTRY *PFN_glBindRenderbuffer)(GLenum target, GLuint renderbuffer);
static PFN_glBindRenderbuffer _sapp_glBindRenderbuffer;
typedef const GLubyte * (GL_APIENTRY *PFN_glGetStringi)(GLenum name, GLuint index);
static PFN_glGetStringi _sapp_glGetStringi;
typedef void  (GL_APIENTRY *PFN_glClearBufferfi)(GLenum buffer, GLint drawbuffer, GLfloat depth, GLint stencil);
static PFN_glClearBufferfi _sapp_glClearBufferfi;
typedef void  (GL_APIENTRY *PFN_glClearBufferfv)(GLenum buffer, GLint drawbuffer, const GLfloat * value);
static PFN_glClearBufferfv _sapp_glClearBufferfv;
typedef void  (GL_APIENTRY *PFN_glClearBufferuiv)(GLenum buffer, GLint drawbuffer, const GLuint * value);
static PFN_glClearBufferuiv _sapp_glClearBufferuiv;
typedef void  (GL_APIENTRY *PFN_glDeleteRenderbuffers)(GLsizei n, const GLuint * renderbuffers);
static PFN_glDeleteRenderbuffers _sapp_glDeleteRenderbuffers;
typedef void  (GL_APIENTRY *PFN_glUniform4fv)(GLint location, GLsizei count, const GLfloat * value);
static PFN_glUniform4fv _sapp_glUniform4fv;
void glUniform4fv(GLint location, GLsizei count, const GLfloat * value) {
    _sapp_glUniform4fv(location, count, value);
}
typedef void  (GL_APIENTRY *PFN_glUniform2fv)(GLint location, GLsizei count, const GLfloat * value);
static PFN_glUniform2fv _sapp_glUniform2fv;
void glUniform2fv(GLint location, GLsizei count, const GLfloat * value) {
    _sapp_glUniform2fv(location, count, value);
}
typedef void  (GL_APIENTRY *PFN_glUseProgram)(GLuint program);
static PFN_glUseProgram _sapp_glUseProgram;
void  glUseProgram(GLuint program) {
    _sapp_glUseProgram(program);
}
typedef void  (GL_APIENTRY *PFN_glShaderSource)(GLuint shader, GLsizei count, const GLchar *const* string, const GLint * length);
static PFN_glShaderSource _sapp_glShaderSource;
void glShaderSource(GLuint shader, GLsizei count, const GLchar *const* string, const GLint * length) {
    _sapp_glShaderSource(shader, count, string, length);
}
typedef void  (GL_APIENTRY *PFN_glLinkProgram)(GLuint program);
static PFN_glLinkProgram _sapp_glLinkProgram;
void glLinkProgram(GLuint program) {
    _sapp_glLinkProgram(program);
}
typedef void  (GL_APIENTRY *PFN_glPixelStorei)(GLenum pname, GLint param);
static PFN_glPixelStorei _sapp_glPixelStorei;
void glPixelStorei(GLenum pname, GLint param) {
    _sapp_glPixelStorei(pname, param);
}
typedef GLint (GL_APIENTRY *PFN_glGetUniformLocation)(GLuint program, const GLchar * name);
static PFN_glGetUniformLocation _sapp_glGetUniformLocation;
GLint glGetUniformLocation(GLuint program, const GLchar * name) {
    return _sapp_glGetUniformLocation(program, name);
}
typedef void  (GL_APIENTRY *PFN_glGetShaderiv)(GLuint shader, GLenum pname, GLint * params);
static PFN_glGetShaderiv _sapp_glGetShaderiv;
void glGetShaderiv(GLuint shader, GLenum pname, GLint * params) {
    _sapp_glGetShaderiv(shader, pname, params);
}
typedef void  (GL_APIENTRY *PFN_glGetProgramInfoLog)(GLuint program, GLsizei bufSize, GLsizei * length, GLchar * infoLog);
static PFN_glGetProgramInfoLog _sapp_glGetProgramInfoLog;
void glGetProgramInfoLog(GLuint program, GLsizei bufSize, GLsizei * length, GLchar * infoLog) {
    _sapp_glGetProgramInfoLog(program, bufSize, length, infoLog);
}
typedef GLint (GL_APIENTRY *PFN_glGetAttribLocation)(GLuint program, const GLchar * name);
static PFN_glGetAttribLocation _sapp_glGetAttribLocation;
GLint glGetAttribLocation(GLuint program, const GLchar * name) {
    return _sapp_glGetAttribLocation(program, name);
}
typedef void  (GL_APIENTRY *PFN_glDisableVertexAttribArray)(GLuint index);
static PFN_glDisableVertexAttribArray _sapp_glDisableVertexAttribArray;
void glDisableVertexAttribArray(GLuint index) {
    _sapp_glDisableVertexAttribArray(index);
}
typedef void  (GL_APIENTRY *PFN_glDeleteShader)(GLuint shader);
static PFN_glDeleteShader _sapp_glDeleteShader;
void glDeleteShader(GLuint shader) {
    _sapp_glDeleteShader(shader);
}
typedef void  (GL_APIENTRY *PFN_glDeleteProgram)(GLuint program);
static PFN_glDeleteProgram _sapp_glDeleteProgram;
typedef void  (GL_APIENTRY *PFN_glCompileShader)(GLuint shader);
static PFN_glCompileShader _sapp_glCompileShader;
void glCompileShader(GLuint shader) {
    _sapp_glCompileShader(shader);
}
typedef void  (GL_APIENTRY *PFN_glStencilFuncSeparate)(GLenum face, GLenum func, GLint ref, GLuint mask);
static PFN_glStencilFuncSeparate _sapp_glStencilFuncSeparate;
void glStencilFuncSeparate(GLenum face, GLenum func, GLint ref, GLuint mask) {
    _sapp_glStencilFuncSeparate(face, func, ref, mask);
}
typedef void  (GL_APIENTRY *PFN_glStencilOpSeparate)(GLenum face, GLenum sfail, GLenum dpfail, GLenum dppass);
static PFN_glStencilOpSeparate _sapp_glStencilOpSeparate;
void glStencilOpSeparate(GLenum face, GLenum sfail, GLenum dpfail, GLenum dppass) {
    _sapp_glStencilOpSeparate(face, sfail, dpfail, dppass);
}
typedef void  (GL_APIENTRY *PFN_glRenderbufferStorageMultisample)(GLenum target, GLsizei samples, GLenum internalformat, GLsizei width, GLsizei height);
static PFN_glRenderbufferStorageMultisample _sapp_glRenderbufferStorageMultisample;
typedef void  (GL_APIENTRY *PFN_glDrawBuffers)(GLsizei n, const GLenum * bufs);
static PFN_glDrawBuffers _sapp_glDrawBuffers;
typedef void  (GL_APIENTRY *PFN_glVertexAttribDivisor)(GLuint index, GLuint divisor);
static PFN_glVertexAttribDivisor _sapp_glVertexAttribDivisor;
void glVertexAttribDivisor(GLuint index, GLuint divisor) {
    _sapp_glVertexAttribDivisor(index, divisor);
}
typedef void  (GL_APIENTRY *PFN_glBufferSubData)(GLenum target, GLintptr offset, GLsizeiptr size, const void * data);
static PFN_glBufferSubData _sapp_glBufferSubData;
void glBufferSubData(GLenum target, GLintptr offset, GLsizeiptr size, const void * data) {
    _sapp_glBufferSubData(target, offset, size, data);
}
typedef void  (GL_APIENTRY *PFN_glGenBuffers)(GLsizei n, GLuint * buffers);
static PFN_glGenBuffers _sapp_glGenBuffers;
void glGenBuffers(GLsizei n, GLuint * buffers) {
    _sapp_glGenBuffers(n, buffers);
}
typedef GLenum (GL_APIENTRY *PFN_glCheckFramebufferStatus)(GLenum target);
static PFN_glCheckFramebufferStatus _sapp_glCheckFramebufferStatus;
typedef void  (GL_APIENTRY *PFN_glFramebufferRenderbuffer)(GLenum target, GLenum attachment, GLenum renderbuffertarget, GLuint renderbuffer);
static PFN_glFramebufferRenderbuffer _sapp_glFramebufferRenderbuffer;
typedef void  (GL_APIENTRY *PFN_glCompressedTexImage2D)(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLint border, GLsizei imageSize, const void * data);
static PFN_glCompressedTexImage2D _sapp_glCompressedTexImage2D;
typedef void  (GL_APIENTRY *PFN_glCompressedTexImage3D)(GLenum target, GLint level, GLenum internalformat, GLsizei width, GLsizei height, GLsizei depth, GLint border, GLsizei imageSize, const void * data);
static PFN_glCompressedTexImage3D _sapp_glCompressedTexImage3D;
typedef void  (GL_APIENTRY *PFN_glActiveTexture)(GLenum texture);
static PFN_glActiveTexture _sapp_glActiveTexture;
void glActiveTexture(GLenum texture) {
    _sapp_glActiveTexture(texture);
}
typedef void  (GL_APIENTRY *PFN_glTexSubImage3D)(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLint zoffset, GLsizei width, GLsizei height, GLsizei depth, GLenum format, GLenum type, const void * pixels);
static PFN_glTexSubImage3D _sapp_glTexSubImage3D;
typedef void  (GL_APIENTRY *PFN_glUniformMatrix4fv)(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value);
static PFN_glUniformMatrix4fv _sapp_glUniformMatrix4fv;
void glUniformMatrix4fv(GLint location, GLsizei count, GLboolean transpose, const GLfloat * value) {
    _sapp_glUniformMatrix4fv(location, count, transpose, value);
}
typedef void  (GL_APIENTRY *PFN_glRenderbufferStorage)(GLenum target, GLenum internalformat, GLsizei width, GLsizei height);
static PFN_glRenderbufferStorage _sapp_glRenderbufferStorage;
typedef void  (GL_APIENTRY *PFN_glGenTextures)(GLsizei n, GLuint * textures);
static PFN_glGenTextures _sapp_glGenTextures;
void glGenTextures(GLsizei n, GLuint * textures) {
    _sapp_glGenTextures(n, textures);
}
typedef void  (GL_APIENTRY *PFN_glPolygonOffset)(GLfloat factor, GLfloat units);
static PFN_glPolygonOffset _sapp_glPolygonOffset;
typedef void  (GL_APIENTRY *PFN_glDrawElements)(GLenum mode, GLsizei count, GLenum type, const void * indices);
static PFN_glDrawElements _sapp_glDrawElements;
void glDrawElements(GLenum mode, GLsizei count, GLenum type, const void * indices) {
    _sapp_glDrawElements(mode, count, type, indices);
}
typedef void  (GL_APIENTRY *PFN_glDeleteFramebuffers)(GLsizei n, const GLuint * framebuffers);
static PFN_glDeleteFramebuffers _sapp_glDeleteFramebuffers;
void glDeleteFramebuffers(GLsizei n, const GLuint * buffers) {
    _sapp_glDeleteFramebuffers(n, buffers);
}
static PFN_glDeleteFramebuffers _sapp_glDeleteFramebuffers;
typedef void  (GL_APIENTRY *PFN_glBlendEquationSeparate)(GLenum modeRGB, GLenum modeAlpha);
static PFN_glBlendEquationSeparate _sapp_glBlendEquationSeparate;
typedef void  (GL_APIENTRY *PFN_glBlendEquationSeparate)(GLenum modeRGB, GLenum modeAlpha);
void glBlendEquationSeparate(GLenum modeRGB, GLenum modeAlpha) {
    _sapp_glBlendEquationSeparate(modeRGB, modeAlpha);
}
static PFN_glBlendEquationSeparate _sapp_glBlendEquationSeparate;
typedef void  (GL_APIENTRY *PFN_glDeleteTextures)(GLsizei n, const GLuint * textures);
static PFN_glDeleteTextures _sapp_glDeleteTextures;
void glDeleteTextures(GLsizei n, const GLuint * textures) {
    _sapp_glDeleteTextures(n, textures);
}
typedef void  (GL_APIENTRY *PFN_glGetProgramiv)(GLuint program, GLenum pname, GLint * params);
static PFN_glGetProgramiv _sapp_glGetProgramiv;
void glGetProgramiv(GLuint program, GLenum pname, GLint * params) {
    _sapp_glGetProgramiv(program, pname, params);
}
typedef void  (GL_APIENTRY *PFN_glBindTexture)(GLenum target, GLuint texture);
static PFN_glBindTexture _sapp_glBindTexture;
void glBindTexture(GLenum target, GLuint texture) {
    _sapp_glBindTexture(target, texture);
}
typedef void  (GL_APIENTRY *PFN_glTexImage3D)(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLsizei depth, GLint border, GLenum format, GLenum type, const void * pixels);
static PFN_glTexImage3D _sapp_glTexImage3D;
typedef GLuint (GL_APIENTRY *PFN_glCreateShader)(GLenum type);
static PFN_glCreateShader _sapp_glCreateShader;
GLuint glCreateShader(GLenum type) {
    return _sapp_glCreateShader(type);
}
typedef void  (GL_APIENTRY *PFN_glTexSubImage2D)(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const void * pixels);
static PFN_glTexSubImage2D _sapp_glTexSubImage2D;
void glTexSubImage2D(GLenum target, GLint level, GLint xoffset, GLint yoffset, GLsizei width, GLsizei height, GLenum format, GLenum type, const void * pixels) {
    _sapp_glTexSubImage2D(target, level, xoffset, yoffset, width, height, format, type, pixels);
}
typedef void  (GL_APIENTRY *PFN_glClearDepth)(GLdouble depth);
static PFN_glClearDepth _sapp_glClearDepth;
void glClearDepth(GLdouble depth) {
    _sapp_glClearDepth(depth);
}
typedef void  (GL_APIENTRY *PFN_glClearDepthf)(GLfloat depth);
static PFN_glClearDepthf _sapp_glClearDepthf;
void glClearDepthf(GLfloat depth) {
    _sapp_glClearDepthf(depth);
}
typedef void  (GL_APIENTRY *PFN_glFramebufferTexture2D)(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level);
static PFN_glFramebufferTexture2D _sapp_glFramebufferTexture2D;
void glFramebufferTexture2D(GLenum target, GLenum attachment, GLenum textarget, GLuint texture, GLint level) {
    _sapp_glFramebufferTexture2D(target, attachment, textarget, texture, level);
}
typedef GLuint (GL_APIENTRY *PFN_glCreateProgram)();
static PFN_glCreateProgram _sapp_glCreateProgram;
GLuint glCreateProgram() {
    return _sapp_glCreateProgram();
}
typedef void  (GL_APIENTRY *PFN_glViewport)(GLint x, GLint y, GLsizei width, GLsizei height);
static PFN_glViewport _sapp_glViewport;
void glViewport(GLint x, GLint y, GLsizei width, GLsizei height) {
    _sapp_glViewport(x, y, width, height);
}
typedef void  (GL_APIENTRY *PFN_glDeleteBuffers)(GLsizei n, const GLuint * buffers);
static PFN_glDeleteBuffers _sapp_glDeleteBuffers;
void glDeleteBuffers(GLsizei n, const GLuint * buffers) {
    _sapp_glDeleteBuffers(n, buffers);
}
typedef void  (GL_APIENTRY *PFN_glDrawArrays)(GLenum mode, GLint first, GLsizei count);
static PFN_glDrawArrays _sapp_glDrawArrays;
void glDrawArrays(GLenum mode, GLint first, GLsizei count) {
    _sapp_glDrawArrays(mode, first, count);
}

typedef void  (GL_APIENTRY *PFN_glDrawElementsInstanced)(GLenum mode, GLsizei count, GLenum type, const void * indices, GLsizei instancecount);
static PFN_glDrawElementsInstanced _sapp_glDrawElementsInstanced;
void glDrawElementsInstanced(GLenum mode, GLsizei count, GLenum type, const void * indices, GLsizei instancecount) {
    _sapp_glDrawElementsInstanced(mode, count, type, indices, instancecount);
}
typedef void  (GL_APIENTRY *PFN_glVertexAttribPointer)(GLuint index, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const void * pointer);
static PFN_glVertexAttribPointer _sapp_glVertexAttribPointer;
void  glVertexAttribPointer(GLuint index, GLint size, GLenum type, GLboolean normalized, GLsizei stride, const void * pointer) {
    _sapp_glVertexAttribPointer(index, size, type, normalized, stride, pointer);
}
typedef void  (GL_APIENTRY *PFN_glUniform1i)(GLint location, GLint v0);
static PFN_glUniform1i _sapp_glUniform1i;
void  glUniform1i(GLint location, GLint v0) {
    _sapp_glUniform1i(location, v0);
}
typedef void  (GL_APIENTRY *PFN_glUniform1iv)(GLint location, GLsizei count, const GLint * value);
static PFN_glUniform1iv _sapp_glUniform1iv;
void  glUniform1iv(GLint location, GLsizei count, const GLint * value) {
    _sapp_glUniform1iv(location, count, value);
}
typedef void  (GL_APIENTRY *PFN_glUniform2iv)(GLint location, GLsizei count, const GLint * value);
static PFN_glUniform2iv _sapp_glUniform2iv;
void  glUniform2iv(GLint location, GLsizei count, const GLint * value) {
    _sapp_glUniform2iv(location, count, value);
}
typedef void  (GL_APIENTRY *PFN_glUniform3iv)(GLint location, GLsizei count, const GLint * value);
static PFN_glUniform3iv _sapp_glUniform3iv;
void  glUniform3iv(GLint location, GLsizei count, const GLint * value) {
    _sapp_glUniform3iv(location, count, value);
}
typedef void  (GL_APIENTRY *PFN_glUniform4iv)(GLint location, GLsizei count, const GLint * value);
static PFN_glUniform4iv _sapp_glUniform4iv;
void  glUniform4iv(GLint location, GLsizei count, const GLint * value) {
    _sapp_glUniform4iv(location, count, value);
}
typedef void  (GL_APIENTRY *PFN_glDisable)(GLenum cap);
static PFN_glDisable _sapp_glDisable;
void glDisable(GLenum cap) {
    _sapp_glDisable(cap);
}
typedef void  (GL_APIENTRY *PFN_glColorMask)(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha);
static PFN_glColorMask _sapp_glColorMask;
void glColorMask(GLboolean red, GLboolean green, GLboolean blue, GLboolean alpha) {
    _sapp_glColorMask(red, green, blue, alpha);
}
typedef void  (GL_APIENTRY *PFN_glBindBuffer)(GLenum target, GLuint buffer);
static PFN_glBindBuffer _sapp_glBindBuffer;
void glBindBuffer(GLenum target, GLuint buffer) {
    _sapp_glBindBuffer(target, buffer);
}

typedef void  (GL_APIENTRY *PFN_glDeleteVertexArrays)(GLsizei n, const GLuint * arrays);
static PFN_glDeleteVertexArrays _sapp_glDeleteVertexArrays;
typedef void  (GL_APIENTRY *PFN_glDepthMask)(GLboolean flag);
static PFN_glDepthMask _sapp_glDepthMask;
typedef void  (GL_APIENTRY *PFN_glDrawArraysInstanced)(GLenum mode, GLint first, GLsizei count, GLsizei instancecount);
static PFN_glDrawArraysInstanced _sapp_glDrawArraysInstanced;
typedef void  (GL_APIENTRY *PFN_glClearStencil)(GLint s);
static PFN_glClearStencil _sapp_glClearStencil;
void glClearStencil(GLint s) {
    _sapp_glClearStencil(s);
}
typedef void  (GL_APIENTRY *PFN_glScissor)(GLint x, GLint y, GLsizei width, GLsizei height);
static PFN_glScissor _sapp_glScissor;
void glScissor(GLint x, GLint y, GLsizei width, GLsizei height) {
    _sapp_glScissor(x, y, width, height);
}
typedef void  (GL_APIENTRY *PFN_glUniform3fv)(GLint location, GLsizei count, const GLfloat * value);
static PFN_glUniform3fv _sapp_glUniform3fv;
void glUniform3fv(GLint location, GLsizei count, const GLfloat * value) {
    _sapp_glUniform3fv(location, count, value);
}
typedef void  (GL_APIENTRY *PFN_glGenRenderbuffers)(GLsizei n, GLuint * renderbuffers);
static PFN_glGenRenderbuffers _sapp_glGenRenderbuffers;
typedef void  (GL_APIENTRY *PFN_glBufferData)(GLenum target, GLsizeiptr size, const void * data, GLenum usage);
static PFN_glBufferData _sapp_glBufferData;
void glBufferData(GLenum target, GLsizeiptr size, const void * data, GLenum usage) {
    _sapp_glBufferData(target, size, data, usage);
}

typedef void  (GL_APIENTRY *PFN_glBlendFuncSeparate)(GLenum sfactorRGB, GLenum dfactorRGB, GLenum sfactorAlpha, GLenum dfactorAlpha);
static PFN_glBlendFuncSeparate _sapp_glBlendFuncSeparate;
void  glBlendFuncSeparate(GLenum sfactorRGB, GLenum dfactorRGB, GLenum sfactorAlpha, GLenum dfactorAlpha) {
    _sapp_glBlendFuncSeparate(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha);
}
typedef void  (GL_APIENTRY *PFN_glTexParameteri)(GLenum target, GLenum pname, GLint param);
static PFN_glTexParameteri _sapp_glTexParameteri;
void glTexParameteri(GLenum target, GLenum pname, GLint param) {
    _sapp_glTexParameteri(target, pname, param);
}
typedef void  (GL_APIENTRY *PFN_glGetIntegerv)(GLenum pname, GLint * data);
static PFN_glGetIntegerv _sapp_glGetIntegerv;
void glGetIntegerv(GLenum pname, GLint * data) {
    _sapp_glGetIntegerv(pname, data);
}
typedef void  (GL_APIENTRY *PFN_glEnable)(GLenum cap);
static PFN_glEnable _sapp_glEnable;
void  glEnable(GLenum cap) {
    _sapp_glEnable(cap);
}
typedef void  (GL_APIENTRY *PFN_glBlitFramebuffer)(GLint srcX0, GLint srcY0, GLint srcX1, GLint srcY1, GLint dstX0, GLint dstY0, GLint dstX1, GLint dstY1, GLbitfield mask, GLenum filter);
static PFN_glBlitFramebuffer _sapp_glBlitFramebuffer;
typedef void  (GL_APIENTRY *PFN_glStencilMask)(GLuint mask);
static PFN_glStencilMask _sapp_glStencilMask;
typedef void  (GL_APIENTRY *PFN_glStencilMaskSeparate)(GLenum face, GLuint mask);
static PFN_glStencilMaskSeparate _sapp_glStencilMaskSeparate;
void glStencilMaskSeparate(GLenum face, GLuint mask) {
    _sapp_glStencilMaskSeparate(face, mask);
}
typedef void  (GL_APIENTRY *PFN_glAttachShader)(GLuint program, GLuint shader);
static PFN_glAttachShader _sapp_glAttachShader;
void glAttachShader(GLuint program, GLuint shader) {
    _sapp_glAttachShader(program, shader);
}
typedef GLenum (GL_APIENTRY *PFN_glGetError)();
static PFN_glGetError _sapp_glGetError;
typedef void  (GL_APIENTRY *PFN_glClearColor)(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha);
static PFN_glClearColor _sapp_glClearColor;
void glClearColor(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha) {
    _sapp_glClearColor(red, green, blue, alpha);
}
typedef void  (GL_APIENTRY *PFN_glBlendColor)(GLfloat red, GLfloat green, GLfloat blue, GLfloat alpha);
static PFN_glBlendColor _sapp_glBlendColor;
typedef void  (GL_APIENTRY *PFN_glTexParameterf)(GLenum target, GLenum pname, GLfloat param);
static PFN_glTexParameterf _sapp_glTexParameterf;
typedef void  (GL_APIENTRY *PFN_glTexParameterfv)(GLenum target, GLenum pname, GLfloat* params);
static PFN_glTexParameterfv _sapp_glTexParameterfv;
typedef void  (GL_APIENTRY *PFN_glGetShaderInfoLog)(GLuint shader, GLsizei bufSize, GLsizei * length, GLchar * infoLog);
static PFN_glGetShaderInfoLog _sapp_glGetShaderInfoLog;
void glGetShaderInfoLog(GLuint shader, GLsizei bufSize, GLsizei * length, GLchar * infoLog) {
    _sapp_glGetShaderInfoLog(shader, bufSize, length, infoLog);
}
typedef void  (GL_APIENTRY *PFN_glDepthFunc)(GLenum func);
static PFN_glDepthFunc _sapp_glDepthFunc;
void glDepthFunc(GLenum func) {
    _sapp_glDepthFunc(func);
}
typedef void  (GL_APIENTRY *PFN_glStencilOp)(GLenum fail, GLenum zfail, GLenum zpass);
static PFN_glStencilOp _sapp_glStencilOp;
typedef void  (GL_APIENTRY *PFN_glStencilFunc)(GLenum func, GLint ref, GLuint mask);
static PFN_glStencilFunc _sapp_glStencilFunc;
typedef void  (GL_APIENTRY *PFN_glEnableVertexAttribArray)(GLuint index);
static PFN_glEnableVertexAttribArray _sapp_glEnableVertexAttribArray;
void glEnableVertexAttribArray(GLuint index) {
    _sapp_glEnableVertexAttribArray(index);
}
typedef void  (GL_APIENTRY *PFN_glBlendFunc)(GLenum sfactor, GLenum dfactor);
static PFN_glBlendFunc _sapp_glBlendFunc;
void glBlendFunc(GLenum sfactor, GLenum dfactor) {
    _sapp_glBlendFunc(sfactor, dfactor);
}
typedef void  (GL_APIENTRY *PFN_glUniform1fv)(GLint location, GLsizei count, const GLfloat * value);
static PFN_glUniform1fv _sapp_glUniform1fv;
void glUniform1fv(GLint location, GLsizei count, const GLfloat * value) {
    _sapp_glUniform1fv(location, count, value);
}
typedef void  (GL_APIENTRY *PFN_glUniform1f)(GLint location, GLfloat value);
static PFN_glUniform1f _sapp_glUniform1f;
void glUniform1f(GLint location, GLfloat value) {
    _sapp_glUniform1f(location, value);
}
typedef void  (GL_APIENTRY *PFN_glReadBuffer)(GLenum src);
static PFN_glReadBuffer _sapp_glReadBuffer;
typedef void  (GL_APIENTRY *PFN_glClear)(GLbitfield mask);
static PFN_glClear _sapp_glClear;
void glClear(GLbitfield mask) {
    _sapp_glClear(mask);
}
typedef void  (GL_APIENTRY *PFN_glTexImage2D)(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const void * pixels);
static PFN_glTexImage2D _sapp_glTexImage2D;
void glTexImage2D(GLenum target, GLint level, GLint internalformat, GLsizei width, GLsizei height, GLint border, GLenum format, GLenum type, const void * pixels) {
    _sapp_glTexImage2D(target, level, internalformat, width, height, border, format, type, pixels);
}
typedef void  (GL_APIENTRY *PFN_glGenVertexArrays)(GLsizei n, GLuint * arrays);
static PFN_glGenVertexArrays _sapp_glGenVertexArrays;
void glGenVertexArrays(GLsizei n, GLuint * arrays) {
    _sapp_glGenVertexArrays(n, arrays);
}
typedef void  (GL_APIENTRY *PFN_glFrontFace)(GLenum mode);
static PFN_glFrontFace _sapp_glFrontFace;
void glFrontFace(GLenum mode) {
    _sapp_glFrontFace(mode);
}
typedef void  (GL_APIENTRY *PFN_glCullFace)(GLenum mode);
static PFN_glCullFace _sapp_glCullFace;
void glCullFace(GLenum mode) {
    _sapp_glCullFace(mode);
}

_SOKOL_PRIVATE void* _sapp_win32_glgetprocaddr(const char* name) {
    void* proc_addr = (void*) _sapp_wglGetProcAddress(name);
    if (0 == proc_addr) {
        proc_addr = (void*) GetProcAddress(_sapp_opengl32, name);
    }
    SOKOL_ASSERT(proc_addr);
    return proc_addr;
}

#define _SAPP_GLPROC(name) _sapp_ ## name = (PFN_ ## name) _sapp_win32_glgetprocaddr(#name)

_SOKOL_PRIVATE  void _sapp_win32_gl_loadfuncs(void) {
    SOKOL_ASSERT(_sapp_wglGetProcAddress);
    SOKOL_ASSERT(_sapp_opengl32);
    
    _SAPP_GLPROC(glGetStringi);
    _SAPP_GLPROC(glBindVertexArray);
    _SAPP_GLPROC(glFramebufferTextureLayer);
    _SAPP_GLPROC(glGenFramebuffers);
    _SAPP_GLPROC(glBindFramebuffer);
    _SAPP_GLPROC(glBindRenderbuffer);
    _SAPP_GLPROC(glClearBufferfi);
    _SAPP_GLPROC(glClearBufferfv);
    _SAPP_GLPROC(glClearBufferuiv);
    _SAPP_GLPROC(glDeleteRenderbuffers);
    _SAPP_GLPROC(glUniform4fv);
    _SAPP_GLPROC(glUniform2fv);
    _SAPP_GLPROC(glUseProgram);
    _SAPP_GLPROC(glShaderSource);
    _SAPP_GLPROC(glLinkProgram);
    _SAPP_GLPROC(glPixelStorei);
    _SAPP_GLPROC(glGetUniformLocation);
    _SAPP_GLPROC(glGetShaderiv);
    _SAPP_GLPROC(glGetProgramInfoLog);
    _SAPP_GLPROC(glGetAttribLocation);
    _SAPP_GLPROC(glDisableVertexAttribArray);
    _SAPP_GLPROC(glDeleteShader);
    _SAPP_GLPROC(glDeleteProgram);
    _SAPP_GLPROC(glCompileShader);
    _SAPP_GLPROC(glStencilFuncSeparate);
    _SAPP_GLPROC(glStencilOpSeparate);
    _SAPP_GLPROC(glRenderbufferStorageMultisample);
    _SAPP_GLPROC(glDrawBuffers);
    _SAPP_GLPROC(glVertexAttribDivisor);
    _SAPP_GLPROC(glBufferSubData);
    _SAPP_GLPROC(glGenBuffers);
    _SAPP_GLPROC(glCheckFramebufferStatus);
    _SAPP_GLPROC(glFramebufferRenderbuffer);
    _SAPP_GLPROC(glCompressedTexImage2D);
    _SAPP_GLPROC(glCompressedTexImage3D);
    _SAPP_GLPROC(glActiveTexture);
    _SAPP_GLPROC(glTexSubImage3D);
    _SAPP_GLPROC(glUniformMatrix4fv);
    _SAPP_GLPROC(glRenderbufferStorage);
    _SAPP_GLPROC(glGenTextures);
    _SAPP_GLPROC(glPolygonOffset);
    _SAPP_GLPROC(glDrawElements);
    _SAPP_GLPROC(glDeleteFramebuffers);
    _SAPP_GLPROC(glBlendEquationSeparate);
    _SAPP_GLPROC(glDeleteTextures);
    _SAPP_GLPROC(glGetProgramiv);
    _SAPP_GLPROC(glBindTexture);
    _SAPP_GLPROC(glTexImage3D);
    _SAPP_GLPROC(glCreateShader);
    _SAPP_GLPROC(glTexSubImage2D);
    _SAPP_GLPROC(glClearDepth);
    _SAPP_GLPROC(glClearDepthf);
    _SAPP_GLPROC(glFramebufferTexture2D);
    _SAPP_GLPROC(glCreateProgram);
    _SAPP_GLPROC(glViewport);
    _SAPP_GLPROC(glDeleteBuffers);
    _SAPP_GLPROC(glDrawArrays);
    _SAPP_GLPROC(glDrawElementsInstanced);
    _SAPP_GLPROC(glVertexAttribPointer);
    _SAPP_GLPROC(glUniform1i);
    _SAPP_GLPROC(glUniform1iv);
    _SAPP_GLPROC(glUniform2iv);
    _SAPP_GLPROC(glUniform3iv);
    _SAPP_GLPROC(glUniform4iv);
    _SAPP_GLPROC(glDisable);
    _SAPP_GLPROC(glColorMask);
    _SAPP_GLPROC(glBindBuffer);
    _SAPP_GLPROC(glDeleteVertexArrays);
    _SAPP_GLPROC(glDepthMask);
    _SAPP_GLPROC(glDrawArraysInstanced);
    _SAPP_GLPROC(glClearStencil);
    _SAPP_GLPROC(glScissor);
    _SAPP_GLPROC(glUniform3fv);
    _SAPP_GLPROC(glGenRenderbuffers);
    _SAPP_GLPROC(glBufferData);
    _SAPP_GLPROC(glBlendFuncSeparate);
    _SAPP_GLPROC(glTexParameteri);
    _SAPP_GLPROC(glGetIntegerv);
    _SAPP_GLPROC(glEnable);
    _SAPP_GLPROC(glBlitFramebuffer);
    _SAPP_GLPROC(glStencilMask);
    _SAPP_GLPROC(glStencilMaskSeparate);
    _SAPP_GLPROC(glAttachShader);
    _SAPP_GLPROC(glGetError);
    _SAPP_GLPROC(glClearColor);
    _SAPP_GLPROC(glBlendColor);
    _SAPP_GLPROC(glTexParameterf);
    _SAPP_GLPROC(glTexParameterfv);
    _SAPP_GLPROC(glGetShaderInfoLog);
    _SAPP_GLPROC(glDepthFunc);
    _SAPP_GLPROC(glStencilOp);
    _SAPP_GLPROC(glStencilFunc);
    _SAPP_GLPROC(glEnableVertexAttribArray);
    _SAPP_GLPROC(glBlendFunc);
    _SAPP_GLPROC(glUniform1f);
    _SAPP_GLPROC(glUniform1fv);
    _SAPP_GLPROC(glReadBuffer);
    _SAPP_GLPROC(glClear);
    _SAPP_GLPROC(glTexImage2D);
    _SAPP_GLPROC(glGenVertexArrays);
    _SAPP_GLPROC(glFrontFace);
    _SAPP_GLPROC(glCullFace);
}
#define glBindVertexArray _sapp_glBindVertexArray
#define glFramebufferTextureLayer _sapp_glFramebufferTextureLayer
#define glGenFramebuffers _sapp_glGenFramebuffers
#define glBindFramebuffer _sapp_glBindFramebuffer
#define glBindRenderbuffer _sapp_glBindRenderbuffer
#define glGetStringi _sapp_glGetStringi
#define glClearBufferfi _sapp_glClearBufferfi
#define glClearBufferfv _sapp_glClearBufferfv
#define glClearBufferuiv _sapp_glClearBufferuiv
#define glDeleteRenderbuffers _sapp_glDeleteRenderbuffers
#define glUniform4fv _sapp_glUniform4fv
#define glUniform2fv _sapp_glUniform2fv
#define glUseProgram _sapp_glUseProgram
#define glShaderSource _sapp_glShaderSource
#define glLinkProgram _sapp_glLinkProgram
#define glPixelStorei _sapp_glPixelStorei
#define glGetUniformLocation _sapp_glGetUniformLocation
#define glGetShaderiv _sapp_glGetShaderiv
#define glGetProgramInfoLog _sapp_glGetProgramInfoLog
#define glGetAttribLocation _sapp_glGetAttribLocation
#define glDisableVertexAttribArray _sapp_glDisableVertexAttribArray
#define glDeleteShader _sapp_glDeleteShader
#define glDeleteProgram _sapp_glDeleteProgram
#define glCompileShader _sapp_glCompileShader
#define glStencilFuncSeparate _sapp_glStencilFuncSeparate
#define glStencilOpSeparate _sapp_glStencilOpSeparate
#define glRenderbufferStorageMultisample _sapp_glRenderbufferStorageMultisample
#define glDrawBuffers _sapp_glDrawBuffers
#define glVertexAttribDivisor _sapp_glVertexAttribDivisor
#define glBufferSubData _sapp_glBufferSubData
#define glGenBuffers _sapp_glGenBuffers
#define glCheckFramebufferStatus _sapp_glCheckFramebufferStatus
#define glFramebufferRenderbuffer _sapp_glFramebufferRenderbuffer
#define glCompressedTexImage2D _sapp_glCompressedTexImage2D
#define glCompressedTexImage3D _sapp_glCompressedTexImage3D
#define glActiveTexture _sapp_glActiveTexture
#define glTexSubImage3D _sapp_glTexSubImage3D
#define glUniformMatrix4fv _sapp_glUniformMatrix4fv
#define glRenderbufferStorage _sapp_glRenderbufferStorage
#define glGenTextures _sapp_glGenTextures
#define glPolygonOffset _sapp_glPolygonOffset
#define glDrawElements _sapp_glDrawElements
#define glDeleteFramebuffers _sapp_glDeleteFramebuffers
#define glBlendEquationSeparate _sapp_glBlendEquationSeparate
#define glDeleteTextures _sapp_glDeleteTextures
#define glGetProgramiv _sapp_glGetProgramiv
#define glBindTexture _sapp_glBindTexture
#define glTexImage3D _sapp_glTexImage3D
#define glCreateShader _sapp_glCreateShader
#define glTexSubImage2D _sapp_glTexSubImage2D
#define glClearDepth _sapp_glClearDepth
#define glFramebufferTexture2D _sapp_glFramebufferTexture2D
#define glCreateProgram _sapp_glCreateProgram
#define glViewport _sapp_glViewport
#define glDeleteBuffers _sapp_glDeleteBuffers
#define glDrawArrays _sapp_glDrawArrays
#define glDrawElementsInstanced _sapp_glDrawElementsInstanced
#define glVertexAttribPointer _sapp_glVertexAttribPointer
#define glUniform1i _sapp_glUniform1i
#define glDisable _sapp_glDisable
#define glColorMask _sapp_glColorMask
#define glBindBuffer _sapp_glBindBuffer
#define glDeleteVertexArrays _sapp_glDeleteVertexArrays
#define glDepthMask _sapp_glDepthMask
#define glDrawArraysInstanced _sapp_glDrawArraysInstanced
#define glClearStencil _sapp_glClearStencil
#define glScissor _sapp_glScissor
#define glUniform3fv _sapp_glUniform3fv
#define glGenRenderbuffers _sapp_glGenRenderbuffers
#define glBufferData _sapp_glBufferData
#define glBlendFuncSeparate _sapp_glBlendFuncSeparate
#define glTexParameteri _sapp_glTexParameteri
#define glGetIntegerv _sapp_glGetIntegerv
#define glEnable _sapp_glEnable
#define glBlitFramebuffer _sapp_glBlitFramebuffer
#define glStencilMask _sapp_glStencilMask
#define glStencilMaskSeparate _sapp_glStencilMaskSeparate
#define glAttachShader _sapp_glAttachShader
#define glGetError _sapp_glGetError
#define glClearColor _sapp_glClearColor
#define glBlendColor _sapp_glBlendColor
#define glTexParameterf _sapp_glTexParameterf
#define glTexParameterfv _sapp_glTexParameterfv
#define glGetShaderInfoLog _sapp_glGetShaderInfoLog
#define glDepthFunc _sapp_glDepthFunc
#define glStencilOp _sapp_glStencilOp
#define glStencilFunc _sapp_glStencilFunc
#define glEnableVertexAttribArray _sapp_glEnableVertexAttribArray
#define glBlendFunc _sapp_glBlendFunc
#define glUniform1fv _sapp_glUniform1fv
#define glReadBuffer _sapp_glReadBuffer
#define glClear _sapp_glClear
#define glTexImage2D _sapp_glTexImage2D
#define glGenVertexArrays _sapp_glGenVertexArrays
#define glFrontFace _sapp_glFrontFace
#define glCullFace _sapp_glCullFace

#endif /* SOKOL_WIN32_NO_GL_LOADER */

#endif /* SOKOL_GLCORE33 */

#if defined(SOKOL_D3D11)
#define _SAPP_SAFE_RELEASE(class, obj) if (obj) { class##_Release(obj); obj=0; }
_SOKOL_PRIVATE void _sapp_d3d11_create_device_and_swapchain(void) {
    DXGI_SWAP_CHAIN_DESC* sc_desc = &_sapp_dxgi_swap_chain_desc;
    sc_desc->BufferDesc.Width = _sapp.framebuffer_width;
    sc_desc->BufferDesc.Height = _sapp.framebuffer_height;
    sc_desc->BufferDesc.Format = DXGI_FORMAT_B8G8R8A8_UNORM;
    sc_desc->BufferDesc.RefreshRate.Numerator = 60;
    sc_desc->BufferDesc.RefreshRate.Denominator = 1;
    sc_desc->OutputWindow = _sapp_win32_hwnd;
    sc_desc->Windowed = true;
    sc_desc->SwapEffect = DXGI_SWAP_EFFECT_DISCARD;
    sc_desc->BufferCount = 1;
    sc_desc->SampleDesc.Count = _sapp.sample_count;
    sc_desc->SampleDesc.Quality = _sapp.sample_count > 1 ? D3D11_STANDARD_MULTISAMPLE_PATTERN : 0;
    sc_desc->BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
    int create_flags = D3D11_CREATE_DEVICE_SINGLETHREADED | D3D11_CREATE_DEVICE_BGRA_SUPPORT;
    #if defined(SOKOL_DEBUG)
        create_flags |= D3D11_CREATE_DEVICE_DEBUG;
    #endif
    D3D_FEATURE_LEVEL feature_level;
    HRESULT hr = D3D11CreateDeviceAndSwapChain(
        NULL,                           /* pAdapter (use default) */
        D3D_DRIVER_TYPE_HARDWARE,       /* DriverType */
        NULL,                           /* Software */
        create_flags,                   /* Flags */
        NULL,                           /* pFeatureLevels */
        0,                              /* FeatureLevels */
        D3D11_SDK_VERSION,              /* SDKVersion */
        sc_desc,                        /* pSwapChainDesc */
        &_sapp_dxgi_swap_chain,         /* ppSwapChain */
        &_sapp_d3d11_device,            /* ppDevice */
        &feature_level,                 /* pFeatureLevel */
        &_sapp_d3d11_device_context);   /* ppImmediateContext */
    _SOKOL_UNUSED(hr);
    SOKOL_ASSERT(SUCCEEDED(hr) && _sapp_dxgi_swap_chain && _sapp_d3d11_device && _sapp_d3d11_device_context);
}

_SOKOL_PRIVATE void _sapp_d3d11_destroy_device_and_swapchain(void) {
    _SAPP_SAFE_RELEASE(IDXGISwapChain, _sapp_dxgi_swap_chain);
    _SAPP_SAFE_RELEASE(ID3D11DeviceContext, _sapp_d3d11_device_context);
    _SAPP_SAFE_RELEASE(ID3D11Device, _sapp_d3d11_device);
}

_SOKOL_PRIVATE void _sapp_d3d11_create_default_render_target(void) {
    HRESULT hr;
    #ifdef __cplusplus
    hr = IDXGISwapChain_GetBuffer(_sapp_dxgi_swap_chain, 0, IID_ID3D11Texture2D, (void**)&_sapp_d3d11_rt);
    #else
    hr = IDXGISwapChain_GetBuffer(_sapp_dxgi_swap_chain, 0, &IID_ID3D11Texture2D, (void**)&_sapp_d3d11_rt);
    #endif
    SOKOL_ASSERT(SUCCEEDED(hr) && _sapp_d3d11_rt);
    hr = ID3D11Device_CreateRenderTargetView(_sapp_d3d11_device, (ID3D11Resource*)_sapp_d3d11_rt, NULL, &_sapp_d3d11_rtv);
    SOKOL_ASSERT(SUCCEEDED(hr) && _sapp_d3d11_rtv);
    D3D11_TEXTURE2D_DESC ds_desc;
    memset(&ds_desc, 0, sizeof(ds_desc));
    ds_desc.Width = _sapp.framebuffer_width;
    ds_desc.Height = _sapp.framebuffer_height;
    ds_desc.MipLevels = 1;
    ds_desc.ArraySize = 1;
    ds_desc.Format = DXGI_FORMAT_D24_UNORM_S8_UINT;
    ds_desc.SampleDesc = _sapp_dxgi_swap_chain_desc.SampleDesc;
    ds_desc.Usage = D3D11_USAGE_DEFAULT;
    ds_desc.BindFlags = D3D11_BIND_DEPTH_STENCIL;
    hr = ID3D11Device_CreateTexture2D(_sapp_d3d11_device, &ds_desc, NULL, &_sapp_d3d11_ds);
    SOKOL_ASSERT(SUCCEEDED(hr) && _sapp_d3d11_ds);
    D3D11_DEPTH_STENCIL_VIEW_DESC dsv_desc;
    memset(&dsv_desc, 0, sizeof(dsv_desc));
    dsv_desc.Format = ds_desc.Format;
    dsv_desc.ViewDimension = _sapp.sample_count > 1 ? D3D11_DSV_DIMENSION_TEXTURE2DMS : D3D11_DSV_DIMENSION_TEXTURE2D;
    hr = ID3D11Device_CreateDepthStencilView(_sapp_d3d11_device, (ID3D11Resource*)_sapp_d3d11_ds, &dsv_desc, &_sapp_d3d11_dsv);
    SOKOL_ASSERT(SUCCEEDED(hr) && _sapp_d3d11_dsv);
}

_SOKOL_PRIVATE void _sapp_d3d11_destroy_default_render_target(void) {
    _SAPP_SAFE_RELEASE(ID3D11Texture2D, _sapp_d3d11_rt);
    _SAPP_SAFE_RELEASE(ID3D11RenderTargetView, _sapp_d3d11_rtv);
    _SAPP_SAFE_RELEASE(ID3D11Texture2D, _sapp_d3d11_ds);
    _SAPP_SAFE_RELEASE(ID3D11DepthStencilView, _sapp_d3d11_dsv);
}

_SOKOL_PRIVATE void _sapp_d3d11_resize_default_render_target(void) {
    if (_sapp_dxgi_swap_chain) {
        _sapp_d3d11_destroy_default_render_target();
        IDXGISwapChain_ResizeBuffers(_sapp_dxgi_swap_chain, 1, _sapp.framebuffer_width, _sapp.framebuffer_height, DXGI_FORMAT_B8G8R8A8_UNORM, 0);
        _sapp_d3d11_create_default_render_target();
    }
}
#endif

#if defined(SOKOL_GLCORE33)
_SOKOL_PRIVATE void _sapp_wgl_init(void) {
    _sapp_opengl32 = LoadLibraryA("opengl32.dll");
    if (!_sapp_opengl32) {
        _sapp_fail("Failed to load opengl32.dll\n");
    }
    SOKOL_ASSERT(_sapp_opengl32);
    _sapp_wglCreateContext = (PFN_wglCreateContext) GetProcAddress(_sapp_opengl32, "wglCreateContext");
    SOKOL_ASSERT(_sapp_wglCreateContext);
    _sapp_wglDeleteContext = (PFN_wglDeleteContext) GetProcAddress(_sapp_opengl32, "wglDeleteContext");
    SOKOL_ASSERT(_sapp_wglDeleteContext);
    _sapp_wglGetProcAddress = (PFN_wglGetProcAddress) GetProcAddress(_sapp_opengl32, "wglGetProcAddress");
    SOKOL_ASSERT(_sapp_wglGetProcAddress);
    _sapp_wglGetCurrentDC = (PFN_wglGetCurrentDC) GetProcAddress(_sapp_opengl32, "wglGetCurrentDC");
    SOKOL_ASSERT(_sapp_wglGetCurrentDC);
    _sapp_wglMakeCurrent = (PFN_wglMakeCurrent) GetProcAddress(_sapp_opengl32, "wglMakeCurrent");
    SOKOL_ASSERT(_sapp_wglMakeCurrent);

    _sapp_win32_msg_hwnd = CreateWindowExW(WS_EX_OVERLAPPEDWINDOW,
        L"SOKOLAPP",
        L"sokol-app message window",
        WS_CLIPSIBLINGS|WS_CLIPCHILDREN,
        0, 0, 1, 1,
        NULL, NULL,
        GetModuleHandleW(NULL),
        NULL);
    if (!_sapp_win32_msg_hwnd) {
        _sapp_fail("Win32: failed to create helper window!\n");
    }
    ShowWindow(_sapp_win32_msg_hwnd, SW_HIDE);
    MSG msg;
    while (PeekMessageW(&msg, _sapp_win32_msg_hwnd, 0, 0, PM_REMOVE)) {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
    _sapp_win32_msg_dc = GetDC(_sapp_win32_msg_hwnd);
    if (!_sapp_win32_msg_dc) {
        _sapp_fail("Win32: failed to obtain helper window DC!\n");
    }
}

_SOKOL_PRIVATE void _sapp_wgl_shutdown(void) {
    SOKOL_ASSERT(_sapp_opengl32 && _sapp_win32_msg_hwnd);
    DestroyWindow(_sapp_win32_msg_hwnd); _sapp_win32_msg_hwnd = 0;
    FreeLibrary(_sapp_opengl32); _sapp_opengl32 = 0;
}

_SOKOL_PRIVATE bool _sapp_wgl_has_ext(const char* ext, const char* extensions) {
    SOKOL_ASSERT(ext && extensions);
    const char* start = extensions;
    while (true) {
        const char* where = strstr(start, ext);
        if (!where) {
            return false;
        }
        const char* terminator = where + strlen(ext);
        if ((where == start) || (*(where - 1) == ' ')) {
            if (*terminator == ' ' || *terminator == '\0') {
                break;
            }
        }
        start = terminator;
    }
    return true;
}

_SOKOL_PRIVATE bool _sapp_wgl_ext_supported(const char* ext) {
    SOKOL_ASSERT(ext);
    if (_sapp_GetExtensionsStringEXT) {
        const char* extensions = _sapp_GetExtensionsStringEXT();
        if (extensions) {
            if (_sapp_wgl_has_ext(ext, extensions)) {
                return true;
            }
        }
    }
    if (_sapp_GetExtensionsStringARB) {
        const char* extensions = _sapp_GetExtensionsStringARB(_sapp_wglGetCurrentDC());
        if (extensions) {
            if (_sapp_wgl_has_ext(ext, extensions)) {
                return true;
            }
        }
    }
    return false;
}

_SOKOL_PRIVATE void _sapp_wgl_load_extensions(void) {
    SOKOL_ASSERT(_sapp_win32_msg_dc);
    PIXELFORMATDESCRIPTOR pfd;
    memset(&pfd, 0, sizeof(pfd));
    pfd.nSize = sizeof(pfd);
    pfd.nVersion = 1;
    pfd.dwFlags = PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
    pfd.iPixelType = PFD_TYPE_RGBA;
    pfd.cColorBits = 24;
    if (!SetPixelFormat(_sapp_win32_msg_dc, ChoosePixelFormat(_sapp_win32_msg_dc, &pfd), &pfd)) {
        _sapp_fail("WGL: failed to set pixel format for dummy context\n");
    }
    HGLRC rc = _sapp_wglCreateContext(_sapp_win32_msg_dc);
    if (!rc) {
        _sapp_fail("WGL: Failed to create dummy context\n");
    }
    if (!_sapp_wglMakeCurrent(_sapp_win32_msg_dc, rc)) {
        _sapp_fail("WGL: Failed to make context current\n");
    }
    _sapp_GetExtensionsStringEXT = (PFNWGLGETEXTENSIONSSTRINGEXTPROC) _sapp_wglGetProcAddress("wglGetExtensionsStringEXT");
    _sapp_GetExtensionsStringARB = (PFNWGLGETEXTENSIONSSTRINGARBPROC) _sapp_wglGetProcAddress("wglGetExtensionsStringARB");
    _sapp_CreateContextAttribsARB = (PFNWGLCREATECONTEXTATTRIBSARBPROC) _sapp_wglGetProcAddress("wglCreateContextAttribsARB");
    _sapp_SwapIntervalEXT = (PFNWGLSWAPINTERVALEXTPROC) _sapp_wglGetProcAddress("wglSwapIntervalEXT");
    _sapp_GetPixelFormatAttribivARB = (PFNWGLGETPIXELFORMATATTRIBIVARBPROC) _sapp_wglGetProcAddress("wglGetPixelFormatAttribivARB");
    _sapp_arb_multisample = _sapp_wgl_ext_supported("WGL_ARB_multisample");
    _sapp_arb_create_context = _sapp_wgl_ext_supported("WGL_ARB_create_context");
    _sapp_arb_create_context_profile = _sapp_wgl_ext_supported("WGL_ARB_create_context_profile");
    _sapp_ext_swap_control = _sapp_wgl_ext_supported("WGL_EXT_swap_control");
    _sapp_arb_pixel_format = _sapp_wgl_ext_supported("WGL_ARB_pixel_format");
    _sapp_wglMakeCurrent(_sapp_win32_msg_dc, 0);
    _sapp_wglDeleteContext(rc);
}

_SOKOL_PRIVATE int _sapp_wgl_attrib(int pixel_format, int attrib) {
    SOKOL_ASSERT(_sapp_arb_pixel_format);
    int value = 0;
    if (!_sapp_GetPixelFormatAttribivARB(_sapp_win32_dc, pixel_format, 0, 1, &attrib, &value)) {
        _sapp_fail("WGL: Failed to retrieve pixel format attribute\n");
    }
    return value;
}

_SOKOL_PRIVATE int _sapp_wgl_find_pixel_format(void) {
    SOKOL_ASSERT(_sapp_win32_dc);
    SOKOL_ASSERT(_sapp_arb_pixel_format);
    const _sapp_gl_fbconfig* closest;

    int native_count = _sapp_wgl_attrib(1, WGL_NUMBER_PIXEL_FORMATS_ARB);
    _sapp_gl_fbconfig* usable_configs = (_sapp_gl_fbconfig*) SOKOL_CALLOC(native_count, sizeof(_sapp_gl_fbconfig));
    int usable_count = 0;
    for (int i = 0; i < native_count; i++) {
        const int n = i + 1;
        _sapp_gl_fbconfig* u = usable_configs + usable_count;
        _sapp_gl_init_fbconfig(u);
        if (!_sapp_wgl_attrib(n, WGL_SUPPORT_OPENGL_ARB) || !_sapp_wgl_attrib(n, WGL_DRAW_TO_WINDOW_ARB)) {
            continue;
        }
        if (_sapp_wgl_attrib(n, WGL_PIXEL_TYPE_ARB) != WGL_TYPE_RGBA_ARB) {
            continue;
        }
        if (_sapp_wgl_attrib(n, WGL_ACCELERATION_ARB) == WGL_NO_ACCELERATION_ARB) {
            continue;
        }
        u->red_bits     = _sapp_wgl_attrib(n, WGL_RED_BITS_ARB);
        u->green_bits   = _sapp_wgl_attrib(n, WGL_GREEN_BITS_ARB);
        u->blue_bits    = _sapp_wgl_attrib(n, WGL_BLUE_BITS_ARB);
        u->alpha_bits   = _sapp_wgl_attrib(n, WGL_ALPHA_BITS_ARB);
        u->depth_bits   = _sapp_wgl_attrib(n, WGL_DEPTH_BITS_ARB);
        u->stencil_bits = _sapp_wgl_attrib(n, WGL_STENCIL_BITS_ARB);
        if (_sapp_wgl_attrib(n, WGL_DOUBLE_BUFFER_ARB)) {
            u->doublebuffer = true;
        }
        if (_sapp_arb_multisample) {
            u->samples = _sapp_wgl_attrib(n, WGL_SAMPLES_ARB);
        }
        u->handle = n;
        usable_count++;
    }
    SOKOL_ASSERT(usable_count > 0);
    _sapp_gl_fbconfig desired;
    _sapp_gl_init_fbconfig(&desired);
    desired.red_bits = 8;
    desired.green_bits = 8;
    desired.blue_bits = 8;
    desired.alpha_bits = 8;
    desired.depth_bits = 24;
    desired.stencil_bits = 8;
    desired.doublebuffer = true;
    desired.samples = _sapp.sample_count > 1 ? _sapp.sample_count : 0;
    closest = _sapp_gl_choose_fbconfig(&desired, usable_configs, usable_count);
    int pixel_format = 0;
    if (closest) {
        pixel_format = (int) closest->handle;
    }
    SOKOL_FREE(usable_configs);
    return pixel_format;
}

_SOKOL_PRIVATE void _sapp_wgl_create_context(void) {
    int pixel_format = _sapp_wgl_find_pixel_format();
    if (0 == pixel_format) {
        _sapp_fail("WGL: Didn't find matching pixel format.\n");
    }
    PIXELFORMATDESCRIPTOR pfd;
    if (!DescribePixelFormat(_sapp_win32_dc, pixel_format, sizeof(pfd), &pfd)) {
        _sapp_fail("WGL: Failed to retrieve PFD for selected pixel format!\n");
    }
    if (!SetPixelFormat(_sapp_win32_dc, pixel_format, &pfd)) {
        _sapp_fail("WGL: Failed to set selected pixel format!\n");
    }
    if (!_sapp_arb_create_context) {
        _sapp_fail("WGL: ARB_create_context required!\n");
    }
    if (!_sapp_arb_create_context_profile) {
        _sapp_fail("WGL: ARB_create_context_profile required!\n");
    }
    const int attrs[] = {
        WGL_CONTEXT_MAJOR_VERSION_ARB, 3,
        WGL_CONTEXT_MINOR_VERSION_ARB, 3,
        WGL_CONTEXT_FLAGS_ARB, WGL_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB,
        WGL_CONTEXT_PROFILE_MASK_ARB, WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
        0, 0
    };
    _sapp_gl_ctx = _sapp_CreateContextAttribsARB(_sapp_win32_dc, 0, attrs);
    if (!_sapp_gl_ctx) {
        const DWORD err = GetLastError();
        if (err == (0xc0070000 | ERROR_INVALID_VERSION_ARB)) {
            _sapp_fail("WGL: Driver does not support OpenGL version 3.3\n");
        }
        else if (err == (0xc0070000 | ERROR_INVALID_PROFILE_ARB)) {
            _sapp_fail("WGL: Driver does not support the requested OpenGL profile");
        }
        else if (err == (0xc0070000 | ERROR_INCOMPATIBLE_DEVICE_CONTEXTS_ARB)) {
            _sapp_fail("WGL: The share context is not compatible with the requested context");
        }
        else {
            _sapp_fail("WGL: Failed to create OpenGL context");
        }
    }
    _sapp_wglMakeCurrent(_sapp_win32_dc, _sapp_gl_ctx);
    if (_sapp_ext_swap_control) {
        /* FIXME: DwmIsCompositionEnabled() (see GLFW) */
        _sapp_SwapIntervalEXT(_sapp.swap_interval);
    }
}

_SOKOL_PRIVATE void _sapp_wgl_destroy_context(void) {
    SOKOL_ASSERT(_sapp_gl_ctx);
    _sapp_wglDeleteContext(_sapp_gl_ctx);
    _sapp_gl_ctx = 0;
}

_SOKOL_PRIVATE void _sapp_wgl_swap_buffers(void) {
    SOKOL_ASSERT(_sapp_win32_dc);
    /* FIXME: DwmIsCompositionEnabled? (see GLFW) */
    SwapBuffers(_sapp_win32_dc);
}
#endif

_SOKOL_PRIVATE bool _sapp_win32_utf8_to_wide(const char* src, wchar_t* dst, int dst_num_bytes) {
    SOKOL_ASSERT(src && dst && (dst_num_bytes > 1));
    memset(dst, 0, dst_num_bytes);
    const int dst_chars = dst_num_bytes / sizeof(wchar_t);
    const int dst_needed = MultiByteToWideChar(CP_UTF8, 0, src, -1, 0, 0);
    if ((dst_needed > 0) && (dst_needed < dst_chars)) {
        MultiByteToWideChar(CP_UTF8, 0, src, -1, dst, dst_chars);
        return true;
    }
    else {
        /* input string doesn't fit into destination buffer */
        return false;
    }
}

_SOKOL_PRIVATE void _sapp_win32_show_mouse(bool shown) {
    ShowCursor((BOOL)shown);
}

_SOKOL_PRIVATE bool _sapp_win32_mouse_shown(void) {
    CURSORINFO cursor_info;
    memset(&cursor_info, 0, sizeof(CURSORINFO));
    cursor_info.cbSize = sizeof(CURSORINFO);
    GetCursorInfo(&cursor_info);
    return (cursor_info.flags & CURSOR_SHOWING) != 0;
}

_SOKOL_PRIVATE void _sapp_win32_init_keytable(void) {
    /* same as GLFW */
    _sapp.keycodes[0x00B] = SAPP_KEYCODE_0;
    _sapp.keycodes[0x002] = SAPP_KEYCODE_1;
    _sapp.keycodes[0x003] = SAPP_KEYCODE_2;
    _sapp.keycodes[0x004] = SAPP_KEYCODE_3;
    _sapp.keycodes[0x005] = SAPP_KEYCODE_4;
    _sapp.keycodes[0x006] = SAPP_KEYCODE_5;
    _sapp.keycodes[0x007] = SAPP_KEYCODE_6;
    _sapp.keycodes[0x008] = SAPP_KEYCODE_7;
    _sapp.keycodes[0x009] = SAPP_KEYCODE_8;
    _sapp.keycodes[0x00A] = SAPP_KEYCODE_9;
    _sapp.keycodes[0x01E] = SAPP_KEYCODE_A;
    _sapp.keycodes[0x030] = SAPP_KEYCODE_B;
    _sapp.keycodes[0x02E] = SAPP_KEYCODE_C;
    _sapp.keycodes[0x020] = SAPP_KEYCODE_D;
    _sapp.keycodes[0x012] = SAPP_KEYCODE_E;
    _sapp.keycodes[0x021] = SAPP_KEYCODE_F;
    _sapp.keycodes[0x022] = SAPP_KEYCODE_G;
    _sapp.keycodes[0x023] = SAPP_KEYCODE_H;
    _sapp.keycodes[0x017] = SAPP_KEYCODE_I;
    _sapp.keycodes[0x024] = SAPP_KEYCODE_J;
    _sapp.keycodes[0x025] = SAPP_KEYCODE_K;
    _sapp.keycodes[0x026] = SAPP_KEYCODE_L;
    _sapp.keycodes[0x032] = SAPP_KEYCODE_M;
    _sapp.keycodes[0x031] = SAPP_KEYCODE_N;
    _sapp.keycodes[0x018] = SAPP_KEYCODE_O;
    _sapp.keycodes[0x019] = SAPP_KEYCODE_P;
    _sapp.keycodes[0x010] = SAPP_KEYCODE_Q;
    _sapp.keycodes[0x013] = SAPP_KEYCODE_R;
    _sapp.keycodes[0x01F] = SAPP_KEYCODE_S;
    _sapp.keycodes[0x014] = SAPP_KEYCODE_T;
    _sapp.keycodes[0x016] = SAPP_KEYCODE_U;
    _sapp.keycodes[0x02F] = SAPP_KEYCODE_V;
    _sapp.keycodes[0x011] = SAPP_KEYCODE_W;
    _sapp.keycodes[0x02D] = SAPP_KEYCODE_X;
    _sapp.keycodes[0x015] = SAPP_KEYCODE_Y;
    _sapp.keycodes[0x02C] = SAPP_KEYCODE_Z;
    _sapp.keycodes[0x028] = SAPP_KEYCODE_APOSTROPHE;
    _sapp.keycodes[0x02B] = SAPP_KEYCODE_BACKSLASH;
    _sapp.keycodes[0x033] = SAPP_KEYCODE_COMMA;
    _sapp.keycodes[0x00D] = SAPP_KEYCODE_EQUAL;
    _sapp.keycodes[0x029] = SAPP_KEYCODE_GRAVE_ACCENT;
    _sapp.keycodes[0x01A] = SAPP_KEYCODE_LEFT_BRACKET;
    _sapp.keycodes[0x00C] = SAPP_KEYCODE_MINUS;
    _sapp.keycodes[0x034] = SAPP_KEYCODE_PERIOD;
    _sapp.keycodes[0x01B] = SAPP_KEYCODE_RIGHT_BRACKET;
    _sapp.keycodes[0x027] = SAPP_KEYCODE_SEMICOLON;
    _sapp.keycodes[0x035] = SAPP_KEYCODE_SLASH;
    _sapp.keycodes[0x056] = SAPP_KEYCODE_WORLD_2;
    _sapp.keycodes[0x00E] = SAPP_KEYCODE_BACKSPACE;
    _sapp.keycodes[0x153] = SAPP_KEYCODE_DELETE;
    _sapp.keycodes[0x14F] = SAPP_KEYCODE_END;
    _sapp.keycodes[0x01C] = SAPP_KEYCODE_ENTER;
    _sapp.keycodes[0x001] = SAPP_KEYCODE_ESCAPE;
    _sapp.keycodes[0x147] = SAPP_KEYCODE_HOME;
    _sapp.keycodes[0x152] = SAPP_KEYCODE_INSERT;
    _sapp.keycodes[0x15D] = SAPP_KEYCODE_MENU;
    _sapp.keycodes[0x151] = SAPP_KEYCODE_PAGE_DOWN;
    _sapp.keycodes[0x149] = SAPP_KEYCODE_PAGE_UP;
    _sapp.keycodes[0x045] = SAPP_KEYCODE_PAUSE;
    _sapp.keycodes[0x146] = SAPP_KEYCODE_PAUSE;
    _sapp.keycodes[0x039] = SAPP_KEYCODE_SPACE;
    _sapp.keycodes[0x00F] = SAPP_KEYCODE_TAB;
    _sapp.keycodes[0x03A] = SAPP_KEYCODE_CAPS_LOCK;
    _sapp.keycodes[0x145] = SAPP_KEYCODE_NUM_LOCK;
    _sapp.keycodes[0x046] = SAPP_KEYCODE_SCROLL_LOCK;
    _sapp.keycodes[0x03B] = SAPP_KEYCODE_F1;
    _sapp.keycodes[0x03C] = SAPP_KEYCODE_F2;
    _sapp.keycodes[0x03D] = SAPP_KEYCODE_F3;
    _sapp.keycodes[0x03E] = SAPP_KEYCODE_F4;
    _sapp.keycodes[0x03F] = SAPP_KEYCODE_F5;
    _sapp.keycodes[0x040] = SAPP_KEYCODE_F6;
    _sapp.keycodes[0x041] = SAPP_KEYCODE_F7;
    _sapp.keycodes[0x042] = SAPP_KEYCODE_F8;
    _sapp.keycodes[0x043] = SAPP_KEYCODE_F9;
    _sapp.keycodes[0x044] = SAPP_KEYCODE_F10;
    _sapp.keycodes[0x057] = SAPP_KEYCODE_F11;
    _sapp.keycodes[0x058] = SAPP_KEYCODE_F12;
    _sapp.keycodes[0x064] = SAPP_KEYCODE_F13;
    _sapp.keycodes[0x065] = SAPP_KEYCODE_F14;
    _sapp.keycodes[0x066] = SAPP_KEYCODE_F15;
    _sapp.keycodes[0x067] = SAPP_KEYCODE_F16;
    _sapp.keycodes[0x068] = SAPP_KEYCODE_F17;
    _sapp.keycodes[0x069] = SAPP_KEYCODE_F18;
    _sapp.keycodes[0x06A] = SAPP_KEYCODE_F19;
    _sapp.keycodes[0x06B] = SAPP_KEYCODE_F20;
    _sapp.keycodes[0x06C] = SAPP_KEYCODE_F21;
    _sapp.keycodes[0x06D] = SAPP_KEYCODE_F22;
    _sapp.keycodes[0x06E] = SAPP_KEYCODE_F23;
    _sapp.keycodes[0x076] = SAPP_KEYCODE_F24;
    _sapp.keycodes[0x038] = SAPP_KEYCODE_LEFT_ALT;
    _sapp.keycodes[0x01D] = SAPP_KEYCODE_LEFT_CONTROL;
    _sapp.keycodes[0x02A] = SAPP_KEYCODE_LEFT_SHIFT;
    _sapp.keycodes[0x15B] = SAPP_KEYCODE_LEFT_SUPER;
    _sapp.keycodes[0x137] = SAPP_KEYCODE_PRINT_SCREEN;
    _sapp.keycodes[0x138] = SAPP_KEYCODE_RIGHT_ALT;
    _sapp.keycodes[0x11D] = SAPP_KEYCODE_RIGHT_CONTROL;
    _sapp.keycodes[0x036] = SAPP_KEYCODE_RIGHT_SHIFT;
    _sapp.keycodes[0x15C] = SAPP_KEYCODE_RIGHT_SUPER;
    _sapp.keycodes[0x150] = SAPP_KEYCODE_DOWN;
    _sapp.keycodes[0x14B] = SAPP_KEYCODE_LEFT;
    _sapp.keycodes[0x14D] = SAPP_KEYCODE_RIGHT;
    _sapp.keycodes[0x148] = SAPP_KEYCODE_UP;
    _sapp.keycodes[0x052] = SAPP_KEYCODE_KP_0;
    _sapp.keycodes[0x04F] = SAPP_KEYCODE_KP_1;
    _sapp.keycodes[0x050] = SAPP_KEYCODE_KP_2;
    _sapp.keycodes[0x051] = SAPP_KEYCODE_KP_3;
    _sapp.keycodes[0x04B] = SAPP_KEYCODE_KP_4;
    _sapp.keycodes[0x04C] = SAPP_KEYCODE_KP_5;
    _sapp.keycodes[0x04D] = SAPP_KEYCODE_KP_6;
    _sapp.keycodes[0x047] = SAPP_KEYCODE_KP_7;
    _sapp.keycodes[0x048] = SAPP_KEYCODE_KP_8;
    _sapp.keycodes[0x049] = SAPP_KEYCODE_KP_9;
    _sapp.keycodes[0x04E] = SAPP_KEYCODE_KP_ADD;
    _sapp.keycodes[0x053] = SAPP_KEYCODE_KP_DECIMAL;
    _sapp.keycodes[0x135] = SAPP_KEYCODE_KP_DIVIDE;
    _sapp.keycodes[0x11C] = SAPP_KEYCODE_KP_ENTER;
    _sapp.keycodes[0x037] = SAPP_KEYCODE_KP_MULTIPLY;
    _sapp.keycodes[0x04A] = SAPP_KEYCODE_KP_SUBTRACT;
}

/* updates current window and framebuffer size from the window's client rect, returns true if size has changed */
_SOKOL_PRIVATE bool _sapp_win32_update_dimensions(void) {
    RECT rect;
    if (GetClientRect(_sapp_win32_hwnd, &rect)) {
        _sapp.window_width = (int)((float)(rect.right - rect.left) / _sapp_win32_window_scale);
        _sapp.window_height = (int)((float)(rect.bottom - rect.top) / _sapp_win32_window_scale);
        const int fb_width = (int)((float)_sapp.window_width * _sapp_win32_content_scale);
        const int fb_height = (int)((float)_sapp.window_height * _sapp_win32_content_scale);
        if ((fb_width != _sapp.framebuffer_width) || (fb_height != _sapp.framebuffer_height)) {
            _sapp.framebuffer_width = (int)((float)_sapp.window_width * _sapp_win32_content_scale);
            _sapp.framebuffer_height = (int)((float)_sapp.window_height * _sapp_win32_content_scale);
            /* prevent a framebuffer size of 0 when window is minimized */
            if (_sapp.framebuffer_width == 0) {
                _sapp.framebuffer_width = 1;
            }
            if (_sapp.framebuffer_height == 0) {
                _sapp.framebuffer_height = 1;
            }
            return true;
        }
    }
    else {
        _sapp.window_width = _sapp.window_height = 1;
        _sapp.framebuffer_width = _sapp.framebuffer_height = 1;
    }
    return false;
}

_SOKOL_PRIVATE uint32_t _sapp_win32_mods(void) {
    uint32_t mods = 0;
    if (GetKeyState(VK_SHIFT) & (1<<31)) {
        mods |= SAPP_MODIFIER_SHIFT;
    }
    if (GetKeyState(VK_CONTROL) & (1<<31)) {
        mods |= SAPP_MODIFIER_CTRL;
    }
    if (GetKeyState(VK_MENU) & (1<<31)) {
        mods |= SAPP_MODIFIER_ALT;
    }
    if ((GetKeyState(VK_LWIN) | GetKeyState(VK_RWIN)) & (1<<31)) {
        mods |= SAPP_MODIFIER_SUPER;
    }
    return mods;
}

_SOKOL_PRIVATE void _sapp_win32_mouse_event(sapp_event_type type, sapp_mousebutton btn) {
    if (_sapp_events_enabled()) {
        _sapp_init_event(type);
        _sapp.event.modifiers = _sapp_win32_mods();
        _sapp.event.mouse_button = btn;
        _sapp.event.mouse_x = _sapp.mouse_x;
        _sapp.event.mouse_y = _sapp.mouse_y;
        _sapp_call_event(&_sapp.event);
    }
}

_SOKOL_PRIVATE void _sapp_win32_scroll_event(float x, float y) {
    if (_sapp_events_enabled()) {
        _sapp_init_event(SAPP_EVENTTYPE_MOUSE_SCROLL);
        _sapp.event.modifiers = _sapp_win32_mods();
        _sapp.event.scroll_x = -x / 30.0f;
        _sapp.event.scroll_y = y / 30.0f;
        _sapp_call_event(&_sapp.event);
    }
}

_SOKOL_PRIVATE void _sapp_win32_key_event(sapp_event_type type, int vk, bool repeat) {
    if (_sapp_events_enabled() && (vk < SAPP_MAX_KEYCODES)) {
        _sapp_init_event(type);
        _sapp.event.modifiers = _sapp_win32_mods();
        _sapp.event.key_code = _sapp.keycodes[vk];
        _sapp.event.key_repeat = repeat;
        _sapp_call_event(&_sapp.event);
    }
}

_SOKOL_PRIVATE void _sapp_win32_char_event(uint32_t c, bool repeat) {
    if (_sapp_events_enabled() && (c >= 32)) {
        _sapp_init_event(SAPP_EVENTTYPE_CHAR);
        _sapp.event.modifiers = _sapp_win32_mods();
        _sapp.event.char_code = c;
        _sapp.event.key_repeat = repeat;
        _sapp_call_event(&_sapp.event);
    }
}

_SOKOL_PRIVATE void _sapp_win32_app_event(sapp_event_type type) {
    if (_sapp_events_enabled()) {
        _sapp_init_event(type);
        _sapp_call_event(&_sapp.event);
    }
}

_SOKOL_PRIVATE LRESULT CALLBACK _sapp_win32_wndproc(HWND hWnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
    /* FIXME: refresh rendering during resize with a WM_TIMER event */
    if (!_sapp_win32_in_create_window) {
        switch (uMsg) {
            case WM_CLOSE:
                /* only give user a chance to intervene when sapp_quit() wasn't already called */
                if (!_sapp.quit_ordered) {
                    /* if window should be closed and event handling is enabled, give user code
                        a change to intervene via sapp_cancel_quit()
                    */
                    _sapp.quit_requested = true;
                    _sapp_win32_app_event(SAPP_EVENTTYPE_QUIT_REQUESTED);
                    /* if user code hasn't intervened, quit the app */
                    if (_sapp.quit_requested) {
                        _sapp.quit_ordered = true;
                    }
                }
                if (_sapp.quit_ordered) {
                    PostQuitMessage(0);
                }
                return 0;
            case WM_SYSCOMMAND:
                switch (wParam & 0xFFF0) {
                    case SC_SCREENSAVE:
                    case SC_MONITORPOWER:
                        if (_sapp.desc.fullscreen) {
                            /* disable screen saver and blanking in fullscreen mode */
                            return 0;
                        }
                        break;
                    case SC_KEYMENU:
                        /* user trying to access menu via ALT */
                        return 0;
                }
                break;
            case WM_ERASEBKGND:
                return 1;
            case WM_SIZE:
                {
                    const bool iconified = wParam == SIZE_MINIMIZED;
                    if (iconified != _sapp_win32_iconified) {
                        _sapp_win32_iconified = iconified;
                        if (iconified) {
                            _sapp_win32_app_event(SAPP_EVENTTYPE_ICONIFIED);
                        }
                        else {
                            _sapp_win32_app_event(SAPP_EVENTTYPE_RESTORED);
                        }
                    }
                }
                break;
            case WM_SETCURSOR:
                if (_sapp.desc.user_cursor) {
                    if (LOWORD(lParam) == HTCLIENT) {
                        _sapp_win32_app_event(SAPP_EVENTTYPE_UPDATE_CURSOR);
                        return 1;
                    }
                }
                break;
            case WM_LBUTTONDOWN:
                _sapp_win32_mouse_event(SAPP_EVENTTYPE_MOUSE_DOWN, SAPP_MOUSEBUTTON_LEFT);
                break;
            case WM_RBUTTONDOWN:
                _sapp_win32_mouse_event(SAPP_EVENTTYPE_MOUSE_DOWN, SAPP_MOUSEBUTTON_RIGHT);
                break;
            case WM_MBUTTONDOWN:
                _sapp_win32_mouse_event(SAPP_EVENTTYPE_MOUSE_DOWN, SAPP_MOUSEBUTTON_MIDDLE);
                break;
            case WM_LBUTTONUP:
                _sapp_win32_mouse_event(SAPP_EVENTTYPE_MOUSE_UP, SAPP_MOUSEBUTTON_LEFT);
                break;
            case WM_RBUTTONUP:
                _sapp_win32_mouse_event(SAPP_EVENTTYPE_MOUSE_UP, SAPP_MOUSEBUTTON_RIGHT);
                break;
            case WM_MBUTTONUP:
                _sapp_win32_mouse_event(SAPP_EVENTTYPE_MOUSE_UP, SAPP_MOUSEBUTTON_MIDDLE);
                break;
            case WM_MOUSEMOVE:
                _sapp.mouse_x = (float)GET_X_LPARAM(lParam) * _sapp_win32_mouse_scale;
                _sapp.mouse_y = (float)GET_Y_LPARAM(lParam) * _sapp_win32_mouse_scale;
                if (!_sapp.win32_mouse_tracked) {
                    _sapp.win32_mouse_tracked = true;
                    TRACKMOUSEEVENT tme;
                    memset(&tme, 0, sizeof(tme));
                    tme.cbSize = sizeof(tme);
                    tme.dwFlags = TME_LEAVE;
                    tme.hwndTrack = _sapp_win32_hwnd;
                    TrackMouseEvent(&tme);
                    _sapp_win32_mouse_event(SAPP_EVENTTYPE_MOUSE_ENTER, SAPP_MOUSEBUTTON_INVALID);
                }
                _sapp_win32_mouse_event(SAPP_EVENTTYPE_MOUSE_MOVE,  SAPP_MOUSEBUTTON_INVALID);
                break;
            case WM_MOUSELEAVE:
                _sapp.win32_mouse_tracked = false;
                _sapp_win32_mouse_event(SAPP_EVENTTYPE_MOUSE_LEAVE, SAPP_MOUSEBUTTON_INVALID);
                break;
            case WM_MOUSEWHEEL:
                _sapp_win32_scroll_event(0.0f, (float)((SHORT)HIWORD(wParam)));
                break;
            case WM_MOUSEHWHEEL:
                _sapp_win32_scroll_event((float)((SHORT)HIWORD(wParam)), 0.0f);
                break;
            case WM_CHAR:
                _sapp_win32_char_event((uint32_t)wParam, !!(lParam&0x40000000));
                break;
            case WM_KEYDOWN:
            case WM_SYSKEYDOWN:
                _sapp_win32_key_event(SAPP_EVENTTYPE_KEY_DOWN, (int)(HIWORD(lParam)&0x1FF), !!(lParam&0x40000000));
                break;
            case WM_KEYUP:
            case WM_SYSKEYUP:
                _sapp_win32_key_event(SAPP_EVENTTYPE_KEY_UP, (int)(HIWORD(lParam)&0x1FF), false);
                break;
            default:
                break;
        }
    }
    return DefWindowProcW(hWnd, uMsg, wParam, lParam);
}

_SOKOL_PRIVATE void _sapp_win32_create_window(void) {
    WNDCLASSW wndclassw;
    memset(&wndclassw, 0, sizeof(wndclassw));
    wndclassw.style = CS_HREDRAW | CS_VREDRAW | CS_OWNDC;
    wndclassw.lpfnWndProc = (WNDPROC) _sapp_win32_wndproc;
    wndclassw.hInstance = GetModuleHandleW(NULL);
    wndclassw.hCursor = LoadCursor(NULL, IDC_ARROW);
    wndclassw.hIcon = LoadIcon(NULL, IDI_WINLOGO);
    wndclassw.lpszClassName = L"SOKOLAPP";
    RegisterClassW(&wndclassw);

    DWORD win_style;
    const DWORD win_ex_style = WS_EX_APPWINDOW | WS_EX_WINDOWEDGE;
    RECT rect = { 0, 0, 0, 0 };
    if (_sapp.desc.fullscreen) {
        win_style = WS_POPUP | WS_SYSMENU | WS_VISIBLE;
        rect.right = GetSystemMetrics(SM_CXSCREEN);
        rect.bottom = GetSystemMetrics(SM_CYSCREEN);
    }
    else {
        win_style = WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_SIZEBOX;
        rect.right = (int) ((float)_sapp.window_width * _sapp_win32_window_scale);
        rect.bottom = (int) ((float)_sapp.window_height * _sapp_win32_window_scale);
    }
    AdjustWindowRectEx(&rect, win_style, FALSE, win_ex_style);
    const int win_width = rect.right - rect.left;
    const int win_height = rect.bottom - rect.top;
    _sapp_win32_in_create_window = true;
    _sapp_win32_hwnd = CreateWindowExW(
        win_ex_style,               /* dwExStyle */
        L"SOKOLAPP",                /* lpClassName */
        _sapp.window_title_wide,    /* lpWindowName */
        win_style,                  /* dwStyle */
        CW_USEDEFAULT,              /* X */
        CW_USEDEFAULT,              /* Y */
        win_width,                  /* nWidth */
        win_height,                 /* nHeight */
        NULL,                       /* hWndParent */
        NULL,                       /* hMenu */
        GetModuleHandle(NULL),      /* hInstance */
        NULL);                      /* lParam */
    ShowWindow(_sapp_win32_hwnd, SW_SHOW);
    _sapp_win32_in_create_window = false;
    _sapp_win32_dc = GetDC(_sapp_win32_hwnd);
    SOKOL_ASSERT(_sapp_win32_dc);
    _sapp_win32_update_dimensions();
}

_SOKOL_PRIVATE void _sapp_win32_destroy_window(void) {
    DestroyWindow(_sapp_win32_hwnd); _sapp_win32_hwnd = 0;
    UnregisterClassW(L"SOKOLAPP", GetModuleHandleW(NULL));
}

_SOKOL_PRIVATE void _sapp_win32_init_dpi(void) {
    SOKOL_ASSERT(0 == _sapp_win32_setprocessdpiaware);
    SOKOL_ASSERT(0 == _sapp_win32_setprocessdpiawareness);
    SOKOL_ASSERT(0 == _sapp_win32_getdpiformonitor);
    HINSTANCE user32 = LoadLibraryA("user32.dll");
    if (user32) {
        _sapp_win32_setprocessdpiaware = (SETPROCESSDPIAWARE_T) GetProcAddress(user32, "SetProcessDPIAware");
    }
    HINSTANCE shcore = LoadLibraryA("shcore.dll");
    if (shcore) {
        _sapp_win32_setprocessdpiawareness = (SETPROCESSDPIAWARENESS_T) GetProcAddress(shcore, "SetProcessDpiAwareness");
        _sapp_win32_getdpiformonitor = (GETDPIFORMONITOR_T) GetProcAddress(shcore, "GetDpiForMonitor");
    }
    if (_sapp_win32_setprocessdpiawareness) {
        /* if the app didn't request HighDPI rendering, let Windows do the upscaling */
        PROCESS_DPI_AWARENESS process_dpi_awareness = PROCESS_SYSTEM_DPI_AWARE;
        _sapp_win32_dpi_aware = true;
        if (!_sapp.desc.high_dpi) {
            process_dpi_awareness = PROCESS_DPI_UNAWARE;
            _sapp_win32_dpi_aware = false;
        }
        _sapp_win32_setprocessdpiawareness(process_dpi_awareness);
    }
    else if (_sapp_win32_setprocessdpiaware) {
        _sapp_win32_setprocessdpiaware();
        _sapp_win32_dpi_aware = true;
    }
    /* get dpi scale factor for main monitor */
    if (_sapp_win32_getdpiformonitor && _sapp_win32_dpi_aware) {
        POINT pt = { 1, 1 };
        HMONITOR hm = MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST);
        UINT dpix, dpiy;
        HRESULT hr = _sapp_win32_getdpiformonitor(hm, MDT_EFFECTIVE_DPI, &dpix, &dpiy);
        _SOKOL_UNUSED(hr);
        SOKOL_ASSERT(SUCCEEDED(hr));
        /* clamp window scale to an integer factor */
        _sapp_win32_window_scale = (float)dpix / 96.0f;
    }
    else {
        _sapp_win32_window_scale = 1.0f;
    }
    if (_sapp.desc.high_dpi) {
        _sapp_win32_content_scale = _sapp_win32_window_scale;
        _sapp_win32_mouse_scale = 1.0f;
    }
    else {
        _sapp_win32_content_scale = 1.0f;
        _sapp_win32_mouse_scale = 1.0f / _sapp_win32_window_scale;
    }
    _sapp.dpi_scale = _sapp_win32_content_scale;
    if (user32) {
        FreeLibrary(user32);
    }
    if (shcore) {
        FreeLibrary(shcore);
    }
}

_SOKOL_PRIVATE void _sapp_run(const sapp_desc* desc) {
    _sapp_init_state(desc);
    _sapp_win32_init_keytable();
    _sapp_win32_utf8_to_wide(_sapp.window_title, _sapp.window_title_wide, sizeof(_sapp.window_title_wide));
    _sapp_win32_init_dpi();
    _sapp_win32_create_window();
    #if defined(SOKOL_D3D11)
        _sapp_d3d11_create_device_and_swapchain();
        _sapp_d3d11_create_default_render_target();
    #endif
    #if defined(SOKOL_GLCORE33)
        _sapp_wgl_init();
        _sapp_wgl_load_extensions();
        _sapp_wgl_create_context();
        #if !defined(SOKOL_WIN32_NO_GL_LOADER)
            _sapp_win32_gl_loadfuncs();
        #endif
    #endif
    _sapp.valid = true;

    bool done = false;
    while (!(done || _sapp.quit_ordered)) {
        MSG msg;
        while (PeekMessageW(&msg, NULL, 0, 0, PM_REMOVE)) {
            if (WM_QUIT == msg.message) {
                done = true;
                continue;
            }
            else {
                TranslateMessage(&msg);
                DispatchMessage(&msg);
            }
        }
        _sapp_frame();
        #if defined(SOKOL_D3D11)
            IDXGISwapChain_Present(_sapp_dxgi_swap_chain, _sapp.swap_interval, 0);
            if (IsIconic(_sapp_win32_hwnd)) {
                Sleep(16 * _sapp.swap_interval);
            }
        #endif
        #if defined(SOKOL_GLCORE33)
            _sapp_wgl_swap_buffers();
        #endif
        /* check for window resized, this cannot happen in WM_SIZE as it explodes memory usage */
        if (_sapp_win32_update_dimensions()) {
            #if defined(SOKOL_D3D11)
            _sapp_d3d11_resize_default_render_target();
            #endif
            _sapp_win32_app_event(SAPP_EVENTTYPE_RESIZED);
        }
        if (_sapp.quit_requested) {
            PostMessage(_sapp_win32_hwnd, WM_CLOSE, 0, 0);
        }
    }
    _sapp_call_cleanup();

    #if defined(SOKOL_D3D11)
        _sapp_d3d11_destroy_default_render_target();
        _sapp_d3d11_destroy_device_and_swapchain();
    #else
        _sapp_wgl_destroy_context();
        _sapp_wgl_shutdown();
    #endif
    _sapp_win32_destroy_window();
}

static char** _sapp_win32_command_line_to_utf8_argv(LPWSTR w_command_line, int* o_argc) {
    int argc = 0;
    char** argv;
    char* args;

    LPWSTR* w_argv = CommandLineToArgvW(w_command_line, &argc);
    if (w_argv == NULL) {
        _sapp_fail("Win32: failed to parse command line");
    } else {
        size_t size = wcslen(w_command_line) * 4;
        argv = (char**) SOKOL_CALLOC(1, (argc + 1) * sizeof(char*) + size);
        args = (char*)&argv[argc + 1];
        int n;
        for (int i = 0; i < argc; ++i) {
            n = WideCharToMultiByte(CP_UTF8, 0, w_argv[i], -1, args, (int)size, NULL, NULL);
            if (n == 0) {
                _sapp_fail("Win32: failed to convert all arguments to utf8");
                break;
            }
            argv[i] = args;
            size -= n;
            args += n;
        }
        LocalFree(w_argv);
    }
    *o_argc = argc;
    return argv;
}

#if !defined(SOKOL_NO_ENTRY)
#if defined(SOKOL_WIN32_FORCE_MAIN)
int main(int argc, char* argv[]) {
    sapp_desc desc = sokol_main(argc, argv);
    _sapp_run(&desc);
    return 0;
}
#else
int WINAPI WinMain(_In_ HINSTANCE hInstance, _In_opt_ HINSTANCE hPrevInstance, _In_ LPSTR lpCmdLine, _In_ int nCmdShow) {
    _SOKOL_UNUSED(hInstance);
    _SOKOL_UNUSED(hPrevInstance);
    _SOKOL_UNUSED(lpCmdLine);
    _SOKOL_UNUSED(nCmdShow);
    int argc_utf8 = 0;
    char** argv_utf8 = _sapp_win32_command_line_to_utf8_argv(GetCommandLineW(), &argc_utf8);
    sapp_desc desc = sokol_main(argc_utf8, argv_utf8);
    _sapp_run(&desc);
    SOKOL_FREE(argv_utf8);
    return 0;
}
#endif /* SOKOL_WIN32_FORCE_MAIN */
#endif /* SOKOL_NO_ENTRY */
#undef _SAPP_SAFE_RELEASE
#endif /* WINDOWS */

/*== PUBLIC API FUNCTIONS ====================================================*/
#if defined(SOKOL_NO_ENTRY)

SOKOL_API_IMPL int sapp_run(const sapp_desc* desc) {
    SOKOL_ASSERT(desc);
    _sapp_run(desc);
    return 0;
}

/* this is just a stub so the linker doesn't complain */
sapp_desc sokol_main(int argc, char* argv[]) {
    _SOKOL_UNUSED(argc);
    _SOKOL_UNUSED(argv);
    sapp_desc desc;
    memset(&desc, 0, sizeof(desc));
    return desc;
}
#else
/* likewise, in normal mode, sapp_run() is just an empty stub */
SOKOL_API_IMPL int sapp_run(const sapp_desc* desc) {
    _SOKOL_UNUSED(desc);
    return 0;
}
#endif

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

#undef _sapp_def

#ifdef _MSC_VER
#pragma warning(pop)
#endif

#endif /* SOKOL_IMPL */
