const canvas = document.querySelector("#glcanvas");
const gl = canvas.getContext("webgl");
if (gl === null) {
    alert("Unable to initialize WebGL. Your browser or machine may not support it.");
}

var clipboard = null;

canvas.focus();

canvas.requestPointerLock = canvas.requestPointerLock ||
    canvas.mozRequestPointerLock;
document.exitPointerLock = document.exitPointerLock ||
    document.mozExitPointerLock;

function assert(flag, message) {
    if (flag == false) {
        alert(message)
    }
}

function acquireVertexArrayObjectExtension(ctx) {
    // Extension available in WebGL 1 from Firefox 25 and WebKit 536.28/desktop Safari 6.0.3 onwards. Core feature in WebGL 2.
    var ext = ctx.getExtension('OES_vertex_array_object');
    if (ext) {
        ctx['createVertexArray'] = function () { return ext['createVertexArrayOES'](); };
        ctx['deleteVertexArray'] = function (vao) { ext['deleteVertexArrayOES'](vao); };
        ctx['bindVertexArray'] = function (vao) { ext['bindVertexArrayOES'](vao); };
        ctx['isVertexArray'] = function (vao) { return ext['isVertexArrayOES'](vao); };
    }
    else {
        alert("Unable to get OES_vertex_array_object extension");
    }
}


function acquireInstancedArraysExtension(ctx) {
    // Extension available in WebGL 1 from Firefox 26 and Google Chrome 30 onwards. Core feature in WebGL 2.
    var ext = ctx.getExtension('ANGLE_instanced_arrays');
    if (ext) {
        ctx['vertexAttribDivisor'] = function (index, divisor) { ext['vertexAttribDivisorANGLE'](index, divisor); };
        ctx['drawArraysInstanced'] = function (mode, first, count, primcount) { ext['drawArraysInstancedANGLE'](mode, first, count, primcount); };
        ctx['drawElementsInstanced'] = function (mode, count, type, indices, primcount) { ext['drawElementsInstancedANGLE'](mode, count, type, indices, primcount); };
    }
}

acquireVertexArrayObjectExtension(gl);
acquireInstancedArraysExtension(gl);

// https://developer.mozilla.org/en-US/docs/Web/API/WEBGL_depth_texture
if (gl.getExtension('WEBGL_depth_texture') == null) {
    alert("Cant initialize WEBGL_depth_texture extension");
}

function getArray(ptr, arr, n) {
    return new arr(wasm.memory.buffer, ptr, n);
}

function UTF8ToString(ptr, maxBytesToRead) {
    let u8Array = new Uint8Array(wasm.memory.buffer, ptr);

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

function stringToUTF8(str, heap, outIdx, maxBytesToWrite) {
    var startIdx = outIdx;
    var endIdx = outIdx + maxBytesToWrite;
    for (var i = 0; i < str.length; ++i) {
        // Gotcha: charCodeAt returns a 16-bit word that is a UTF-16 encoded code unit, not a Unicode code point of the character! So decode UTF16->UTF32->UTF8.
        // See http://unicode.org/faq/utf_bom.html#utf16-3
        // For UTF8 byte structure, see http://en.wikipedia.org/wiki/UTF-8#Description and https://www.ietf.org/rfc/rfc2279.txt and https://tools.ietf.org/html/rfc3629
        var u = str.charCodeAt(i); // possibly a lead surrogate
        if (u >= 0xD800 && u <= 0xDFFF) {
            var u1 = str.charCodeAt(++i);
            u = 0x10000 + ((u & 0x3FF) << 10) | (u1 & 0x3FF);
        }
        if (u <= 0x7F) {
            if (outIdx >= endIdx) break;
            heap[outIdx++] = u;
        } else if (u <= 0x7FF) {
            if (outIdx + 1 >= endIdx) break;
            heap[outIdx++] = 0xC0 | (u >> 6);
            heap[outIdx++] = 0x80 | (u & 63);
        } else if (u <= 0xFFFF) {
            if (outIdx + 2 >= endIdx) break;
            heap[outIdx++] = 0xE0 | (u >> 12);
            heap[outIdx++] = 0x80 | ((u >> 6) & 63);
            heap[outIdx++] = 0x80 | (u & 63);
        } else {
            if (outIdx + 3 >= endIdx) break;

            if (u >= 0x200000) console.warn('Invalid Unicode code point 0x' + u.toString(16) + ' encountered when serializing a JS string to an UTF-8 string on the asm.js/wasm heap! (Valid unicode code points should be in range 0-0x1FFFFF).');

            heap[outIdx++] = 0xF0 | (u >> 18);
            heap[outIdx++] = 0x80 | ((u >> 12) & 63);
            heap[outIdx++] = 0x80 | ((u >> 6) & 63);
            heap[outIdx++] = 0x80 | (u & 63);
        }
    }
    return outIdx - startIdx;
}

var FS = {
    loaded_files: [],
    unique_id: 0
};

var GL = {
    counter: 1,
    buffers: [],
    mappedBuffers: {},
    programs: [],
    framebuffers: [],
    renderbuffers: [],
    textures: [],
    uniforms: [],
    shaders: [],
    vaos: [],
    contexts: {},
    programInfos: {},

    getNewId: function (table) {
        var ret = GL.counter++;
        for (var i = table.length; i < ret; i++) {
            table[i] = null;
        }
        return ret;
    },

    validateGLObjectID: function (objectHandleArray, objectID, callerFunctionName, objectReadableType) {
        if (objectID != 0) {
            if (objectHandleArray[objectID] === null) {
                console.error(callerFunctionName + ' called with an already deleted ' + objectReadableType + ' ID ' + objectID + '!');
            } else if (!objectHandleArray[objectID]) {
                console.error(callerFunctionName + ' called with an invalid ' + objectReadableType + ' ID ' + objectID + '!');
            }
        }
    },
    getSource: function (shader, count, string, length) {
        var source = '';
        for (var i = 0; i < count; ++i) {
            var len = length == 0 ? undefined : getArray(length + i * 4, Uint32Array, 1)[0];
            source += UTF8ToString(getArray(string + i * 4, Uint32Array, 1)[0], len);
        }
        return source;
    },
    populateUniformTable: function (program) {
        GL.validateGLObjectID(GL.programs, program, 'populateUniformTable', 'program');
        var p = GL.programs[program];
        var ptable = GL.programInfos[program] = {
            uniforms: {},
            maxUniformLength: 0, // This is eagerly computed below, since we already enumerate all uniforms anyway.
            maxAttributeLength: -1, // This is lazily computed and cached, computed when/if first asked, "-1" meaning not computed yet.
            maxUniformBlockNameLength: -1 // Lazily computed as well
        };

        var utable = ptable.uniforms;
        // A program's uniform table maps the string name of an uniform to an integer location of that uniform.
        // The global GL.uniforms map maps integer locations to WebGLUniformLocations.
        var numUniforms = gl.getProgramParameter(p, 0x8B86/*GL_ACTIVE_UNIFORMS*/);
        for (var i = 0; i < numUniforms; ++i) {
            var u = gl.getActiveUniform(p, i);

            var name = u.name;
            ptable.maxUniformLength = Math.max(ptable.maxUniformLength, name.length + 1);

            // If we are dealing with an array, e.g. vec4 foo[3], strip off the array index part to canonicalize that "foo", "foo[]",
            // and "foo[0]" will mean the same. Loop below will populate foo[1] and foo[2].
            if (name.slice(-1) == ']') {
                name = name.slice(0, name.lastIndexOf('['));
            }

            // Optimize memory usage slightly: If we have an array of uniforms, e.g. 'vec3 colors[3];', then
            // only store the string 'colors' in utable, and 'colors[0]', 'colors[1]' and 'colors[2]' will be parsed as 'colors'+i.
            // Note that for the GL.uniforms table, we still need to fetch the all WebGLUniformLocations for all the indices.
            var loc = gl.getUniformLocation(p, name);
            if (loc) {
                var id = GL.getNewId(GL.uniforms);
                utable[name] = [u.size, id];
                GL.uniforms[id] = loc;

                for (var j = 1; j < u.size; ++j) {
                    var n = name + '[' + j + ']';
                    loc = gl.getUniformLocation(p, n);
                    id = GL.getNewId(GL.uniforms);

                    GL.uniforms[id] = loc;
                }
            }
        }
    }
}

window._glGenObject = function (n, buffers, createFunction, objectTable, functionName) {
    for (var i = 0; i < n; i++) {
        var buffer = gl[createFunction]();
        var id = buffer && GL.getNewId(objectTable);
        if (buffer) {
            buffer.name = id;
            objectTable[id] = buffer;
        } else {
            console.error("GL_INVALID_OPERATION");
            GL.recordError(0x0502 /* GL_INVALID_OPERATION */);

            alert('GL_INVALID_OPERATION in ' + functionName + ': GLctx.' + createFunction + ' returned null - most likely GL context is lost!');
        }
        getArray(buffers + i * 4, Int32Array, 1)[0] = id;
    }
}

window._webglGet = function (name_, p, type) {
    // Guard against user passing a null pointer.
    // Note that GLES2 spec does not say anything about how passing a null pointer should be treated.
    // Testing on desktop core GL 3, the application crashes on glGetIntegerv to a null pointer, but
    // better to report an error instead of doing anything random.
    if (!p) {
        console.error('GL_INVALID_VALUE in glGet' + type + 'v(name=' + name_ + ': Function called with null out pointer!');
        GL.recordError(0x501 /* GL_INVALID_VALUE */);
        return;
    }
    var ret = undefined;
    switch (name_) { // Handle a few trivial GLES values
        case 0x8DFA: // GL_SHADER_COMPILER
            ret = 1;
            break;
        case 0x8DF8: // GL_SHADER_BINARY_FORMATS
            if (type != 'EM_FUNC_SIG_PARAM_I' && type != 'EM_FUNC_SIG_PARAM_I64') {
                GL.recordError(0x500); // GL_INVALID_ENUM

                err('GL_INVALID_ENUM in glGet' + type + 'v(GL_SHADER_BINARY_FORMATS): Invalid parameter type!');
            }
            return; // Do not write anything to the out pointer, since no binary formats are supported.
        case 0x87FE: // GL_NUM_PROGRAM_BINARY_FORMATS
        case 0x8DF9: // GL_NUM_SHADER_BINARY_FORMATS
            ret = 0;
            break;
        case 0x86A2: // GL_NUM_COMPRESSED_TEXTURE_FORMATS
            // WebGL doesn't have GL_NUM_COMPRESSED_TEXTURE_FORMATS (it's obsolete since GL_COMPRESSED_TEXTURE_FORMATS returns a JS array that can be queried for length),
            // so implement it ourselves to allow C++ GLES2 code get the length.
            var formats = gl.getParameter(0x86A3 /*GL_COMPRESSED_TEXTURE_FORMATS*/);
            ret = formats ? formats.length : 0;
            break;
        case 0x821D: // GL_NUM_EXTENSIONS
            assert(false, "unimplemented");
            break;
        case 0x821B: // GL_MAJOR_VERSION
        case 0x821C: // GL_MINOR_VERSION
            assert(false, "unimplemented");
            break;
    }

    if (ret === undefined) {
        var result = gl.getParameter(name_);
        switch (typeof (result)) {
            case "number":
                ret = result;
                break;
            case "boolean":
                ret = result ? 1 : 0;
                break;
            case "string":
                GL.recordError(0x500); // GL_INVALID_ENUM
                console.error('GL_INVALID_ENUM in glGet' + type + 'v(' + name_ + ') on a name which returns a string!');
                return;
            case "object":
                if (result === null) {
                    // null is a valid result for some (e.g., which buffer is bound - perhaps nothing is bound), but otherwise
                    // can mean an invalid name_, which we need to report as an error
                    switch (name_) {
                        case 0x8894: // ARRAY_BUFFER_BINDING
                        case 0x8B8D: // CURRENT_PROGRAM
                        case 0x8895: // ELEMENT_ARRAY_BUFFER_BINDING
                        case 0x8CA6: // FRAMEBUFFER_BINDING
                        case 0x8CA7: // RENDERBUFFER_BINDING
                        case 0x8069: // TEXTURE_BINDING_2D
                        case 0x85B5: // WebGL 2 GL_VERTEX_ARRAY_BINDING, or WebGL 1 extension OES_vertex_array_object GL_VERTEX_ARRAY_BINDING_OES
                        case 0x8919: // GL_SAMPLER_BINDING
                        case 0x8E25: // GL_TRANSFORM_FEEDBACK_BINDING
                        case 0x8514: { // TEXTURE_BINDING_CUBE_MAP
                            ret = 0;
                            break;
                        }
                        default: {
                            GL.recordError(0x500); // GL_INVALID_ENUM
                            console.error('GL_INVALID_ENUM in glGet' + type + 'v(' + name_ + ') and it returns null!');
                            return;
                        }
                    }
                } else if (result instanceof Float32Array ||
                    result instanceof Uint32Array ||
                    result instanceof Int32Array ||
                    result instanceof Array) {
                    for (var i = 0; i < result.length; ++i) {
                        assert(false, "unimplemented")
                    }
                    return;
                } else {
                    try {
                        ret = result.name | 0;
                    } catch (e) {
                        GL.recordError(0x500); // GL_INVALID_ENUM
                        console.error('GL_INVALID_ENUM in glGet' + type + 'v: Unknown object returned from WebGL getParameter(' + name_ + ')! (error: ' + e + ')');
                        return;
                    }
                }
                break;
            default:
                GL.recordError(0x500); // GL_INVALID_ENUM
                console.error('GL_INVALID_ENUM in glGet' + type + 'v: Native code calling glGet' + type + 'v(' + name_ + ') and it returns ' + result + ' of type ' + typeof (result) + '!');
                return;
        }
    }

    switch (type) {
        case 'EM_FUNC_SIG_PARAM_I64': getArray(p, Int32Array, 1)[0] = ret;
        case 'EM_FUNC_SIG_PARAM_I': getArray(p, Int32Array, 1)[0] = ret; break;
        case 'EM_FUNC_SIG_PARAM_F': getArray(p, Float32Array, 1)[0] = ret; break;
        case 'EM_FUNC_SIG_PARAM_B': getArray(p, Int8Array, 1)[0] = ret ? 1 : 0; break;
        default: throw 'internal glGet error, bad type: ' + type;
    }
}

function resize(canvas, on_resize) {
    var displayWidth = canvas.clientWidth;
    var displayHeight = canvas.clientHeight;

    if (canvas.width != displayWidth ||
        canvas.height != displayHeight) {
        canvas.width = displayWidth;
        canvas.height = displayHeight;
        if (on_resize != undefined)
            on_resize(Math.floor(displayWidth), Math.floor(displayHeight))
    }
}

window.animation = function () {
    wasm.exports.frame();
    window.requestAnimationFrame(animation);
}

window.texture_size = function (internalFormat, width, height) {
    if (internalFormat == gl.ALPHA) {
        return width * height;
    }
    else if (internalFormat == gl.RGB) {
        return width * height * 3;
    } else if (internalFormat == gl.RGBA) {
        return width * height * 4;
    } else { // TextureFormat::RGB565 | TextureFormat::RGBA4 | TextureFormat::RGBA5551
        return width * height * 3;
    }
}

var emscripten_shaders_hack = false;

window.wasmImports = {
    ...window.wasmImports,
    console_debug: function (ptr) {
        console.debug(UTF8ToString(ptr));
    },
    console_log: function (ptr) {
        console.log(UTF8ToString(ptr));
    },
    console_info: function (ptr) {
        console.info(UTF8ToString(ptr));
    },
    console_warn: function (ptr) {
        console.warn(UTF8ToString(ptr));
    },
    console_error: function (ptr) {
        console.error(UTF8ToString(ptr));
    },
    set_emscripten_shader_hack: function (flag) {
        emscripten_shaders_hack = flag;
    },
    rand: function () {
        return Math.floor(Math.random() * 2147483647);
    },
    now: function () {
        return Date.now() / 1000.0;
    },
    canvas_width: function () {
        return Math.floor(canvas.clientWidth);
    },
    canvas_height: function () {
        return Math.floor(canvas.clientHeight);
    },
    glClearDepthf: function (depth) {
        gl.clearDepth(depth);
    },
    glClearColor: function (r, g, b, a) {
        gl.clearColor(r, g, b, a);
    },
    glClearStencil: function (s) {
        gl.clearColorStencil(s);
    },
    glColorMask: function (red, green, blue, alpha) {
        gl.colorMask(red, green, blue, alpha);
    },
    glScissor: function (x, y, w, h) {
        gl.scissor(x, y, w, h);
    },
    glClear: function (mask) {
        gl.clear(mask);
    },
    glGenTextures: function (n, textures) {
        _glGenObject(n, textures, "createTexture", GL.textures, "glGenTextures")
    },
    glActiveTexture: function (texture) {
        gl.activeTexture(texture)
    },
    glBindTexture: function (target, texture) {
        GL.validateGLObjectID(GL.textures, texture, 'glBindTexture', 'texture');
        gl.bindTexture(target, GL.textures[texture]);
    },
    glTexImage2D: function (target, level, internalFormat, width, height, border, format, type, pixels) {
        gl.texImage2D(target, level, internalFormat, width, height, border, format, type,
            pixels ? getArray(pixels, Uint8Array, texture_size(internalFormat, width, height)) : null);
    },
    glTexSubImage2D: function (target, level, xoffset, yoffset, width, height, format, type, pixels) {
        gl.texSubImage2D(target, level, xoffset, yoffset, width, height, format, type,
            pixels ? getArray(pixels, Uint8Array, texture_size(format, width, height)) : null);
    },
    glTexParameteri: function (target, pname, param) {
        gl.texParameteri(target, pname, param);
    },
    glUniform1fv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform1fv', 'location');
        assert((value & 3) == 0, 'Pointer to float data passed to glUniform1fv must be aligned to four bytes!');
        var view = getArray(value, Float32Array, 1 * count);
        gl.uniform1fv(GL.uniforms[location], view);
    },
    glUniform2fv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform2fv', 'location');
        assert((value & 3) == 0, 'Pointer to float data passed to glUniform2fv must be aligned to four bytes!');
        var view = getArray(value, Float32Array, 2 * count);
        gl.uniform2fv(GL.uniforms[location], view);
    },
    glUniform3fv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform3fv', 'location');
        assert((value & 3) == 0, 'Pointer to float data passed to glUniform3fv must be aligned to four bytes!');
        var view = getArray(value, Float32Array, 4 * count);
        gl.uniform3fv(GL.uniforms[location], view);
    },
    glUniform4fv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform4fv', 'location');
        assert((value & 3) == 0, 'Pointer to float data passed to glUniform4fv must be aligned to four bytes!');
        var view = getArray(value, Float32Array, 4 * count);
        gl.uniform4fv(GL.uniforms[location], view);
    },
    glUniform1iv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform1fv', 'location');
        assert((value & 3) == 0, 'Pointer to i32 data passed to glUniform1iv must be aligned to four bytes!');
        var view = getArray(value, Int32Array, 1 * count);
        gl.uniform1iv(GL.uniforms[location], view);
    },
    glUniform2iv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform2fv', 'location');
        assert((value & 3) == 0, 'Pointer to i32 data passed to glUniform2iv must be aligned to four bytes!');
        var view = getArray(value, Int32Array, 2 * count);
        gl.uniform2iv(GL.uniforms[location], view);
    },
    glUniform3iv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform3fv', 'location');
        assert((value & 3) == 0, 'Pointer to i32 data passed to glUniform3iv must be aligned to four bytes!');
        var view = getArray(value, Int32Array, 3 * count);
        gl.uniform3iv(GL.uniforms[location], view);
    },
    glUniform4iv: function (location, count, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform4fv', 'location');
        assert((value & 3) == 0, 'Pointer to i32 data passed to glUniform4iv must be aligned to four bytes!');
        var view = getArray(value, Int32Array, 4 * count);
        gl.uniform4iv(GL.uniforms[location], view);
    },
    glBlendFunc: function (sfactor, dfactor) {
        gl.blendFunc(sfactor, dfactor);
    },
    glBlendEquationSeparate: function (modeRGB, modeAlpha) {
        gl.blendEquationSeparate(modeRGB, modeAlpha);
    },
    glDisable: function (cap) {
        gl.disable(cap);
    },
    glDrawElements: function (mode, count, type, indices) {
        gl.drawElements(mode, count, type, indices);
    },
    glGetIntegerv: function (name_, p) {
        _webglGet(name_, p, 'EM_FUNC_SIG_PARAM_I');
    },
    glUniform1f: function (location, v0) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform1f', 'location');
        gl.uniform1f(GL.uniforms[location], v0);
    },
    glUniform1i: function (location, v0) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniform1i', 'location');
        gl.uniform1i(GL.uniforms[location], v0);
    },
    glGetAttribLocation: function (program, name) {
        return gl.getAttribLocation(GL.programs[program], UTF8ToString(name));
    },
    glEnableVertexAttribArray: function (index) {
        gl.enableVertexAttribArray(index);
    },
    glDisableVertexAttribArray: function (index) {
        gl.disableVertexAttribArray(index);
    },
    glVertexAttribPointer: function (index, size, type, normalized, stride, ptr) {
        gl.vertexAttribPointer(index, size, type, !!normalized, stride, ptr);
    },
    glGetUniformLocation: function (program, name) {
        GL.validateGLObjectID(GL.programs, program, 'glGetUniformLocation', 'program');
        name = UTF8ToString(name);
        var arrayIndex = 0;
        // If user passed an array accessor "[index]", parse the array index off the accessor.
        if (name[name.length - 1] == ']') {
            var leftBrace = name.lastIndexOf('[');
            arrayIndex = name[leftBrace + 1] != ']' ? parseInt(name.slice(leftBrace + 1)) : 0; // "index]", parseInt will ignore the ']' at the end; but treat "foo[]" as "foo[0]"
            name = name.slice(0, leftBrace);
        }

        var uniformInfo = GL.programInfos[program] && GL.programInfos[program].uniforms[name]; // returns pair [ dimension_of_uniform_array, uniform_location ]
        if (uniformInfo && arrayIndex >= 0 && arrayIndex < uniformInfo[0]) { // Check if user asked for an out-of-bounds element, i.e. for 'vec4 colors[3];' user could ask for 'colors[10]' which should return -1.
            return uniformInfo[1] + arrayIndex;
        } else {
            return -1;
        }
    },
    glUniformMatrix4fv: function (location, count, transpose, value) {
        GL.validateGLObjectID(GL.uniforms, location, 'glUniformMatrix4fv', 'location');
        assert((value & 3) == 0, 'Pointer to float data passed to glUniformMatrix4fv must be aligned to four bytes!');
        var view = getArray(value, Float32Array, 16);
        gl.uniformMatrix4fv(GL.uniforms[location], !!transpose, view);
    },
    glUseProgram: function (program) {
        GL.validateGLObjectID(GL.programs, program, 'glUseProgram', 'program');
        gl.useProgram(GL.programs[program]);
    },
    glGenVertexArrays: function (n, arrays) {
        _glGenObject(n, arrays, 'createVertexArray', GL.vaos, 'glGenVertexArrays');
    },
    glGenFramebuffers: function (n, ids) {
        _glGenObject(n, ids, 'createFramebuffer', GL.framebuffers, 'glGenFramebuffers');
    },
    glBindVertexArray: function (vao) {
        gl.bindVertexArray(GL.vaos[vao]);
    },
    glBindFramebuffer: function (target, framebuffer) {
        GL.validateGLObjectID(GL.framebuffers, framebuffer, 'glBindFramebuffer', 'framebuffer');

        gl.bindFramebuffer(target, GL.framebuffers[framebuffer]);
    },

    glGenBuffers: function (n, buffers) {
        _glGenObject(n, buffers, 'createBuffer', GL.buffers, 'glGenBuffers');
    },
    glBindBuffer: function (target, buffer) {
        GL.validateGLObjectID(GL.buffers, buffer, 'glBindBuffer', 'buffer');
        gl.bindBuffer(target, GL.buffers[buffer]);
    },
    glBufferData: function (target, size, data, usage) {
        gl.bufferData(target, data ? getArray(data, Uint8Array, size) : size, usage);
    },
    glBufferSubData: function (target, offset, size, data) {
        gl.bufferSubData(target, offset, data ? getArray(data, Uint8Array, size) : size);
    },
    glEnable: function (cap) {
        gl.enable(cap);
    },
    glDepthFunc: function (func) {
        gl.depthFunc(func);
    },
    glBlendFuncSeparate: function (sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha) {
        gl.blendFuncSeparate(sfactorRGB, dfactorRGB, sfactorAlpha, dfactorAlpha);
    },
    glViewport: function (x, y, width, height) {
        gl.viewport(x, y, width, height);
    },
    glDrawArrays: function (mode, first, count) {
        gl.drawArrays(mode, first, count);
    },
    glCreateProgram: function () {
        var id = GL.getNewId(GL.programs);
        var program = gl.createProgram();
        program.name = id;
        GL.programs[id] = program;
        return id;
    },
    glAttachShader: function (program, shader) {
        GL.validateGLObjectID(GL.programs, program, 'glAttachShader', 'program');
        GL.validateGLObjectID(GL.shaders, shader, 'glAttachShader', 'shader');
        gl.attachShader(GL.programs[program], GL.shaders[shader]);
    },
    glLinkProgram: function (program) {
        GL.validateGLObjectID(GL.programs, program, 'glLinkProgram', 'program');
        gl.linkProgram(GL.programs[program]);
        GL.populateUniformTable(program);
    },
    glPixelStorei: function (pname, param) {
        gl.pixelStorei(pname, param);
    },
    glFramebufferTexture2D: function (target, attachment, textarget, texture, level) {
        GL.validateGLObjectID(GL.textures, texture, 'glFramebufferTexture2D', 'texture');
        gl.framebufferTexture2D(target, attachment, textarget, GL.textures[texture], level);
    },
    glGetProgramiv: function (program, pname, p) {
        assert(p);
        GL.validateGLObjectID(GL.programs, program, 'glGetProgramiv', 'program');
        if (program >= GL.counter) {
            console.error("GL_INVALID_VALUE in glGetProgramiv");
            return;
        }
        var ptable = GL.programInfos[program];
        if (!ptable) {
            console.error('GL_INVALID_OPERATION in glGetProgramiv(program=' + program + ', pname=' + pname + ', p=0x' + p.toString(16) + '): The specified GL object name does not refer to a program object!');
            return;
        }
        if (pname == 0x8B84) { // GL_INFO_LOG_LENGTH
            var log = gl.getProgramInfoLog(GL.programs[program]);
            assert(log !== null);

            getArray(p, Int32Array, 1)[0] = log.length + 1;
        } else if (pname == 0x8B87 /* GL_ACTIVE_UNIFORM_MAX_LENGTH */) {
            console.error("unsupported operation");
            return;
        } else if (pname == 0x8B8A /* GL_ACTIVE_ATTRIBUTE_MAX_LENGTH */) {
            console.error("unsupported operation");
            return;
        } else if (pname == 0x8A35 /* GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH */) {
            console.error("unsupported operation");
            return;
        } else {
            getArray(p, Int32Array, 1)[0] = gl.getProgramParameter(GL.programs[program], pname);
        }
    },
    glCreateShader: function (shaderType) {
        var id = GL.getNewId(GL.shaders);
        GL.shaders[id] = gl.createShader(shaderType);
        return id;
    },
    glStencilFuncSeparate: function (face, func, ref_, mask) {
        gl.stencilFuncSeparate(face, func, ref_, mask);
    },
    glStencilMaskSeparate: function (face, mask) {
        gl.stencilMaskSeparate(face, mask);
    },
    glStencilOpSeparate: function (face, fail, zfail, zpass) {
        gl.stencilOpSeparate(face, fail, zfail, zpass);
    },
    glFrontFace: function (mode) {
        gl.frontFace(mode);
    },
    glCullFace: function (mode) {
        gl.cullFace(mode);
    },
    glShaderSource: function (shader, count, string, length) {
        GL.validateGLObjectID(GL.shaders, shader, 'glShaderSource', 'shader');
        var source = GL.getSource(shader, count, string, length);

        // https://github.com/emscripten-core/emscripten/blob/incoming/src/library_webgl.js#L2708
        if (emscripten_shaders_hack) {
            source = source.replace(/#extension GL_OES_standard_derivatives : enable/g, "");
            source = source.replace(/#extension GL_EXT_shader_texture_lod : enable/g, '');
            var prelude = '';
            if (source.indexOf('gl_FragColor') != -1) {
                prelude += 'out mediump vec4 GL_FragColor;\n';
                source = source.replace(/gl_FragColor/g, 'GL_FragColor');
            }
            if (source.indexOf('attribute') != -1) {
                source = source.replace(/attribute/g, 'in');
                source = source.replace(/varying/g, 'out');
            } else {
                source = source.replace(/varying/g, 'in');
            }

            source = source.replace(/textureCubeLodEXT/g, 'textureCubeLod');
            source = source.replace(/texture2DLodEXT/g, 'texture2DLod');
            source = source.replace(/texture2DProjLodEXT/g, 'texture2DProjLod');
            source = source.replace(/texture2DGradEXT/g, 'texture2DGrad');
            source = source.replace(/texture2DProjGradEXT/g, 'texture2DProjGrad');
            source = source.replace(/textureCubeGradEXT/g, 'textureCubeGrad');

            source = source.replace(/textureCube/g, 'texture');
            source = source.replace(/texture1D/g, 'texture');
            source = source.replace(/texture2D/g, 'texture');
            source = source.replace(/texture3D/g, 'texture');
            source = source.replace(/#version 100/g, '#version 300 es\n' + prelude);
        }

        gl.shaderSource(GL.shaders[shader], source);
    },
    glGetProgramInfoLog: function (program, maxLength, length, infoLog) {
        GL.validateGLObjectID(GL.programs, program, 'glGetProgramInfoLog', 'program');
        var log = gl.getProgramInfoLog(GL.programs[program]);
        assert(log !== null);
        let array = getArray(infoLog, Uint8Array, maxLength);
        for (var i = 0; i < maxLength; i++) {
            array[i] = log.charCodeAt(i);
        }
    },
    glCompileShader: function (shader, count, string, length) {
        GL.validateGLObjectID(GL.shaders, shader, 'glCompileShader', 'shader');
        gl.compileShader(GL.shaders[shader]);
    },
    glGetShaderiv: function (shader, pname, p) {
        assert(p);
        GL.validateGLObjectID(GL.shaders, shader, 'glGetShaderiv', 'shader');
        if (pname == 0x8B84) { // GL_INFO_LOG_LENGTH
            var log = gl.getShaderInfoLog(GL.shaders[shader]);
            assert(log !== null);

            getArray(p, Int32Array, 1)[0] = log.length + 1;

        } else if (pname == 0x8B88) { // GL_SHADER_SOURCE_LENGTH
            var source = gl.getShaderSource(GL.shaders[shader]);
            var sourceLength = (source === null || source.length == 0) ? 0 : source.length + 1;
            getArray(p, Int32Array, 1)[0] = sourceLength;
        } else {
            getArray(p, Int32Array, 1)[0] = gl.getShaderParameter(GL.shaders[shader], pname);
        }
    },
    glGetShaderInfoLog: function (shader, maxLength, length, infoLog) {
        GL.validateGLObjectID(GL.shaders, shader, 'glGetShaderInfoLog', 'shader');
        var log = gl.getShaderInfoLog(GL.shaders[shader]);
        assert(log !== null);
        let array = getArray(infoLog, Uint8Array, maxLength);
        for (var i = 0; i < maxLength; i++) {
            array[i] = log.charCodeAt(i);
        }
    },
    glVertexAttribDivisor: function (index, divisor) {
        gl.vertexAttribDivisor(index, divisor);
    },
    glDrawArraysInstanced: function (mode, first, count, primcount) {
        gl.drawArraysInstanced(mode, first, count, primcount);
    },
    glDrawElementsInstanced: function (mode, count, type, indices, primcount) {
        gl.drawElementsInstanced(mode, count, type, indices, primcount);
    },
    glDeleteShader: function (shader) { gl.deleteShader(shader) },
    glDeleteBuffers: function (n, buffers) {
        for (var i = 0; i < n; i++) {
            var id = getArray(buffers + i * 4, Uint32Array, 1)[0];
            var buffer = GL.buffers[id];

            // From spec: "glDeleteBuffers silently ignores 0's and names that do not
            // correspond to existing buffer objects."
            if (!buffer) continue;

            gl.deleteBuffer(buffer);
            buffer.name = 0;
            GL.buffers[id] = null;
        }
    },
    glDeleteFramebuffers: function (n, buffers) {
        for (var i = 0; i < n; i++) {
            var id = getArray(buffers + i * 4, Uint32Array, 1)[0];
            var buffer = GL.framebuffers[id];

            // From spec: "glDeleteFrameBuffers silently ignores 0's and names that do not
            // correspond to existing buffer objects."
            if (!buffer) continue;

            gl.deleteFramebuffer(buffer);
            buffer.name = 0;
            GL.framebuffers[id] = null;
        }
    },
    glDeleteTextures: function (n, textures) {
        for (var i = 0; i < n; i++) {
            var id = getArray(textures + i * 4, Uint32Array, 1)[0];
            var texture = GL.textures[id];
            if (!texture) continue; // GL spec: "glDeleteTextures silently ignores 0s and names that do not correspond to existing textures".
            gl.deleteTexture(texture);
            texture.name = 0;
            GL.textures[id] = null;
        }
    },
    init_opengl: function (ptr) {
        window.sappOpenGlInitHook();
        window.requestAnimationFrame(animation);
    },
    fs_load_file: function (ptr, len) {
        var url = UTF8ToString(ptr, len);
        var file_id = FS.unique_id;
        FS.unique_id += 1;
        var xhr = new XMLHttpRequest();
        xhr.open('GET', url, true);
        xhr.responseType = 'arraybuffer';
        xhr.onload = function (e) {
            if (this.status == 200) {
                var uInt8Array = new Uint8Array(this.response);

                FS.loaded_files[file_id] = uInt8Array;
                wasm.exports.file_loaded(file_id);
            }
        }
        xhr.onerror = function (e) {
            FS.loaded_files[file_id] = null;
            wasm.exports.file_loaded(file_id);
        };

        xhr.send();

        return file_id;
    },
    fs_get_buffer_size: function (file_id) {
        if (FS.loaded_files[file_id] == null) {
            return -1;
        } else {
            return FS.loaded_files[file_id].length;
        }
    },
    fs_take_buffer: function (file_id, ptr, max_length) {
        var file = FS.loaded_files[file_id];
        console.assert(file.length <= max_length);
        var dest = new Uint8Array(wasm.memory.buffer, ptr, max_length);
        for (var i = 0; i < file.length; i++) {
            dest[i] = file[i];
        }
        delete FS.loaded_files[file_id];
    },
};

resize(canvas);