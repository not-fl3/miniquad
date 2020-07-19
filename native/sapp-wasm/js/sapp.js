﻿const SAPP_EVENTTYPE_TOUCHES_BEGAN = 10;
const SAPP_EVENTTYPE_TOUCHES_MOVED = 11;
const SAPP_EVENTTYPE_TOUCHES_ENDED = 12;
const SAPP_EVENTTYPE_TOUCHES_CANCELLED = 13;

const SAPP_MODIFIER_SHIFT = 1;
const SAPP_MODIFIER_CTRL = 2;
const SAPP_MODIFIER_ALT = 4;
const SAPP_MODIFIER_SUPER = 8;

window.into_sapp_mousebutton = function (btn) {
    switch (btn) {
        case 0: return 0;
        case 1: return 2;
        case 2: return 1;
        default: return btn;
    }
}

window.into_sapp_keycode = function (key_code) {
    switch (key_code) {
        case "Space": return 32;
        case "Comma": return 44;
        case "Minus": return 45;
        case "Period": return 46;
        case "Digit0": return 48;
        case "Digit1": return 49;
        case "Digit2": return 50;
        case "Digit3": return 51;
        case "Digit4": return 52;
        case "Digit5": return 53;
        case "Digit6": return 54;
        case "Digit7": return 55;
        case "Digit8": return 56;
        case "Digit9": return 57;
        case "Semicolon": return 59;
        case "Equal": return 61;
        case "KeyA": return 65;
        case "KeyB": return 66;
        case "KeyC": return 67;
        case "KeyD": return 68;
        case "KeyE": return 69;
        case "KeyF": return 70;
        case "KeyG": return 71;
        case "KeyH": return 72;
        case "KeyI": return 73;
        case "KeyJ": return 74;
        case "KeyK": return 75;
        case "KeyL": return 76;
        case "KeyM": return 77;
        case "KeyN": return 78;
        case "KeyO": return 79;
        case "KeyP": return 80;
        case "KeyQ": return 81;
        case "KeyR": return 82;
        case "KeyS": return 83;
        case "KeyT": return 84;
        case "KeyU": return 85;
        case "KeyV": return 86;
        case "KeyW": return 87;
        case "KeyX": return 88;
        case "KeyY": return 89;
        case "KeyZ": return 90;
        case "BracketLeft": return 91;
        case "Backslash": return 92;
        case "BracketRight": return 93;
        case "Escape": return 256;
        case "Enter": return 257;
        case "Tab": return 258;
        case "Backspace": return 259;
        case "Insert": return 260;
        case "Delete": return 261;
        case "ArrowRight": return 262;
        case "ArrowLeft": return 263;
        case "ArrowDown": return 264;
        case "ArrowUp": return 265;
        case "PageUp": return 266;
        case "PageDown": return 267;
        case "Home": return 268;
        case "End": return 269;
        case "CapsLock": return 280;
        case "ScrollLock": return 281;
        case "NumLock": return 282;
        case "PrintScreen": return 283;
        case "Pause": return 284;
        case "F1": return 290;
        case "F2": return 291;
        case "F3": return 292;
        case "F4": return 293;
        case "F5": return 294;
        case "F6": return 295;
        case "F7": return 296;
        case "F8": return 297;
        case "F9": return 298;
        case "F10": return 299;
        case "F11": return 300;
        case "F12": return 301;
        case "F13": return 302;
        case "F14": return 303;
        case "F15": return 304;
        case "F16": return 305;
        case "F17": return 306;
        case "F18": return 307;
        case "F19": return 308;
        case "F20": return 309;
        case "F21": return 310;
        case "F22": return 311;
        case "F23": return 312;
        case "F24": return 313;
        case "Numpad0": return 320;
        case "Numpad1": return 321;
        case "Numpad2": return 322;
        case "Numpad3": return 323;
        case "Numpad4": return 324;
        case "Numpad5": return 325;
        case "Numpad6": return 326;
        case "Numpad7": return 327;
        case "Numpad8": return 328;
        case "Numpad9": return 329;
        case "NumpadDecimal": return 330;
        case "NumpadDivide": return 331;
        case "NumpadMultiply": return 332;
        case "NumpadSubstract": return 333;
        case "NumpadAdd": return 334;
        case "NumpadEnter": return 335;
        case "NumpadEqual": return 336;
        case "ShiftLeft": return 340;
        case "ControlLeft": return 341;
        case "AltLeft": return 342;
        case "OSLeft": return 343;
        case "ShiftRight": return 344;
        case "ControlRight": return 345;
        case "AltRight": return 346;
        case "OSRight": return 347;
        case "ContextMenu": return 348;
    }

    console.log("Unsupported keyboard key: ", key_code)
}

window.mouse_relative_position = function (clientX, clientY) {
    var targetRect = canvas.getBoundingClientRect();
    var x = clientX - targetRect.left;
    var y = clientY - targetRect.top;
    return { x, y };
}

window.wasmImports = {
    ...window.wasmImports,
    sapp_set_clipboard: function(ptr, len) {
        clipboard = UTF8ToString(ptr, len);
    },
    sapp_set_cursor_grab: function (grab) {
        if (grab) {
            canvas.requestPointerLock();
        } else {
            document.exitPointerLock();
        }
    }
};

window.sappOpenGlInitHook = function() {
    canvas.onmousemove = function (event) {
        var relative_position = mouse_relative_position(event.clientX, event.clientY);
        var x = relative_position.x;
        var y = relative_position.y;

        // TODO: do not send mouse_move when cursor is captured
        wasm.exports.mouse_move(Math.floor(x), Math.floor(y));

        // TODO: check that mouse is captured?
        if (event.movementX != 0 || event.movementY != 0) {
            wasm.exports.raw_mouse_move(Math.floor(event.movementX), Math.floor(event.movementY));
        }
    };
    
    canvas.onmousedown = function (event) {
        var relative_position = mouse_relative_position(event.clientX, event.clientY);
        var x = relative_position.x;
        var y = relative_position.y;

        var btn = into_sapp_mousebutton(event.button);
        wasm.exports.mouse_down(x, y, btn);
    };
    
    // SO WEB SO CONSISTENT
    canvas.addEventListener('wheel', function (event) {
        event.preventDefault();
        wasm.exports.mouse_wheel(-event.deltaX, -event.deltaY);
    });
    
    canvas.onmouseup = function (event) {
        var relative_position = mouse_relative_position(event.clientX, event.clientY);
        var x = relative_position.x;
        var y = relative_position.y;

        var btn = into_sapp_mousebutton(event.button);
        wasm.exports.mouse_up(x, y, btn);
    };
    
    canvas.onkeydown = function (event) {
        var sapp_key_code = into_sapp_keycode(event.code);
        switch (sapp_key_code) {
            //  space, arrows - prevent scrolling of the page
            case 32: case 262: case 263: case 264: case 265:
            // F1-F10
            case 290: case 291: case 292: case 293: case 294: case 295: case 296: case 297: case 298: case 299:
            // backspace is Back on Firefox/Windows
            case 259:
                event.preventDefault();
                break;
        }

        var modifiers = 0;
        if (event.ctrlKey) {
            modifiers |= SAPP_MODIFIER_CTRL;
        }
        if (event.shiftKey) {
            modifiers |= SAPP_MODIFIER_SHIFT;
        }
        if (event.altKey) {
            modifiers |= SAPP_MODIFIER_ALT;
        }
        wasm.exports.key_down(sapp_key_code, modifiers, event.repeat);
        // for "space" preventDefault will prevent
        // key_press event, so send it here instead
        if (sapp_key_code == 32) {
            wasm.exports.key_press(sapp_key_code);
        }
    };
    
    canvas.onkeyup = function (event) {
        var sapp_key_code = into_sapp_keycode(event.code);
        wasm.exports.key_up(sapp_key_code);
    };
    
    canvas.onkeypress = function (event) {
        var sapp_key_code = into_sapp_keycode(event.code);

        // firefox do not send onkeypress events for ctrl+keys and delete key while chrome do
        // workaround to make this behavior consistent
        let chrome_only = sapp_key_code == 261 || event.ctrlKey;
        if (chrome_only == false) {
            wasm.exports.key_press(event.charCode);
        }
    };
    
    canvas.addEventListener("touchstart", function (event) {
        event.preventDefault();

        for (touch of event.changedTouches) {
            wasm.exports.touch(SAPP_EVENTTYPE_TOUCHES_BEGAN, touch.identifier, Math.floor(touch.clientX), Math.floor(touch.clientY));
        }
    });
    
    canvas.addEventListener("touchend", function (event) {
        event.preventDefault();

        for (touch of event.changedTouches) {
            wasm.exports.touch(SAPP_EVENTTYPE_TOUCHES_ENDED, touch.identifier, Math.floor(touch.clientX), Math.floor(touch.clientY));
        }
    });
    
    canvas.addEventListener("touchcancel", function (event) {
        event.preventDefault();

        for (touch of event.changedTouches) {
            wasm.exports.touch(SAPP_EVENTTYPE_TOUCHES_CANCELED, touch.identifier, Math.floor(touch.clientX), Math.floor(touch.clientY));
        }
    });
    
    canvas.addEventListener("touchmove", function (event) {
        event.preventDefault();

        for (touch of event.changedTouches) {
            wasm.exports.touch(SAPP_EVENTTYPE_TOUCHES_MOVED, touch.identifier, Math.floor(touch.clientX), Math.floor(touch.clientY));
        }
    });

    window.onresize = function () {
        resize(canvas, wasm.exports.resize);
    };
    
    window.addEventListener("copy", function(e) {
        if (clipboard != null) {
            event.clipboardData.setData('text/plain', clipboard);
            event.preventDefault();
        }
    });
    
    window.addEventListener("cut", function(e) {
        if (clipboard != null) {
            event.clipboardData.setData('text/plain', clipboard);
            event.preventDefault();
        }
    });

    window.addEventListener("paste", function(e) {
        e.stopPropagation();
        e.preventDefault();
        clipboardData = e.clipboardData || window.clipboardData;
        pastedData = clipboardData.getData('Text');

        if (pastedData != undefined && pastedData != null && pastedData.length != 0) {
            var len = pastedData.length;
            var msg = wasm.exports.allocate_vec_u8(len);
            var heap = new Uint8Array(wasm.memory.buffer, msg, len);
            stringToUTF8(pastedData, heap, 0, len);
            wasm.exports.on_clipboard_paste(msg, len);
        }
    });
}