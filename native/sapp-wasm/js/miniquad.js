const canvas = document.querySelector("#glcanvas");
const gl = canvas.getContext("webgl");
if (gl === null) {
    alert("Unable to initialize WebGL. Your browser or machine may not support it.");
}

var plugins = [];
var wasm_exports;
var wasm_memory;

var Module;

canvas.focus();

canvas.requestPointerLock = canvas.requestPointerLock ||
    canvas.mozRequestPointerLock;
document.exitPointerLock = document.exitPointerLock ||
    document.mozExitPointerLock;

function UTF8ToString(ptr, maxBytesToRead) {
    let u8Array = new Uint8Array(wasm_memory.buffer, ptr);

    var idx = 0;
    var endIdx = idx + maxBytesToRead;

    var str = '';
    while (!(idx >= endIdx)) {
        // For UTF8 byte structure, see:
        // http://en.wikipedia.org/wiki/UTF-8#Description
        // https://www.ietf.org/rfc/rfc2279.txt
        // https://tools.ietf.org/html/rfc3629
        var u0 = u8Array[idx++];

        // If not building with TextDecoder enabled, we don't know the string length, so scan for \0 byte.
        // If building with TextDecoder, we know exactly at what byte index the string ends, so checking for nulls here would be redundant.
        if (!u0) return str;

        if (!(u0 & 0x80)) { str += String.fromCharCode(u0); continue; }
        var u1 = u8Array[idx++] & 63;
        if ((u0 & 0xE0) == 0xC0) { str += String.fromCharCode(((u0 & 31) << 6) | u1); continue; }
        var u2 = u8Array[idx++] & 63;
        if ((u0 & 0xF0) == 0xE0) {
            u0 = ((u0 & 15) << 12) | (u1 << 6) | u2;
        } else {

            if ((u0 & 0xF8) != 0xF0) console.warn('Invalid UTF-8 leading byte 0x' + u0.toString(16) + ' encountered when deserializing a UTF-8 string on the asm.js/wasm heap to a JS string!');

            u0 = ((u0 & 7) << 18) | (u1 << 12) | (u2 << 6) | (u8Array[idx++] & 63);
        }

        if (u0 < 0x10000) {
            str += String.fromCharCode(u0);
        } else {
            var ch = u0 - 0x10000;
            str += String.fromCharCode(0xD800 | (ch >> 10), 0xDC00 | (ch & 0x3FF));
        }
    }

    return str;
}

function load_js(value) {
	window.eval(UTF8ToString(value));
}

function register_plugins(plugins) {
    if (plugins == undefined)
        return;

    for (var i = 0; i < plugins.length; i++) {
        if (plugins[i].register_plugin != undefined && plugins[i].register_plugin != null) {
            plugins[i].register_plugin(importObject);
        }
    }
}

function init_plugins(plugins) {
    if (plugins == undefined)
        return;

    for (var i = 0; i < plugins.length; i++) {
        if (plugins[i].on_init != undefined && plugins[i].on_init != null) {
            plugins[i].on_init();
        }
    }
}

function miniquad_add_plugin(plugin) {
    plugins.push(plugin);
}

function load(wasm_path) {
	// TODO: this will fail if any plugin is added because importObject isn't known here yet
    register_plugins(plugins);

	// Use a custom streaming function for older browser versions
	if (!WebAssembly.compileStreaming) {
		WebAssembly.compileStreaming = async(resp, importObject) => {
			const source = await(await resp).arrayBuffer();
			return await WebAssembly.compile(source, importObject);
		};
	}

	var localImportObject = { env: {} };

	WebAssembly.compileStreaming(fetch(wasm_path))
		// First compile the module to get the list of imports and inject load_js
		.then(mod => {
			// Load a list of expected imports
			var imports = WebAssembly.Module.imports(mod);

			// Add all requested imports with an empty function
			imports.forEach(func => localImportObject.env[func.name] = function() {});

			// Implement the 'load_js' function
			localImportObject.env.load_js = load_js

			return WebAssembly.instantiate(mod, localImportObject);
		})
		// Then invoke load_js and override the functions for the imports
		.then(obj => {
			wasm_memory = obj.exports.memory;
			wasm_exports = obj.exports;

			// Load the gl.js data included in the wasm
			wasm_exports.load_gl_js();

			// Override the import object functions with the one from gl.js
			localImportObject = importObject;
			localImportObject.env.load_js = load_js;

			// Start miniquad
			init_plugins(plugins);
			wasm_exports.main();
		})
		.catch(err => console.error(err));
}

// Resize
canvas.width = canvas.clientWidth;
canvas.height = canvas.clientHeight;
