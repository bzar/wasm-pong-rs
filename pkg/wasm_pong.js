
const lAudioContext = (typeof AudioContext == 'undefined' ? webkitAudioContext : AudioContext);
const __exports = {};
let wasm;

/**
* @returns {void}
*/
export function start() {
    return wasm.start();
}
__exports.start = start

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let cachegetFloat32Memory = null;
function getFloat32Memory() {
    if (cachegetFloat32Memory === null || cachegetFloat32Memory.buffer !== wasm.memory.buffer) {
        cachegetFloat32Memory = new Float32Array(wasm.memory.buffer);
    }
    return cachegetFloat32Memory;
}

function getArrayF32FromWasm(ptr, len) {
    return getFloat32Memory().subarray(ptr / 4, ptr / 4 + len);
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function handleError(exnptr, e) {
    const view = getUint32Memory();
    view[exnptr / 4] = 1;
    view[exnptr / 4 + 1] = addHeapObject(e);
}

function __widl_f_copy_to_channel_AudioBuffer(arg0, arg1, arg2, arg3, exnptr) {
    let varg1 = getArrayF32FromWasm(arg1, arg2);
    try {
        getObject(arg0).copyToChannel(varg1, arg3);
    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__widl_f_copy_to_channel_AudioBuffer = __widl_f_copy_to_channel_AudioBuffer

function __widl_f_start_AudioBufferSourceNode(arg0, exnptr) {
    try {
        getObject(arg0).start();
    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__widl_f_start_AudioBufferSourceNode = __widl_f_start_AudioBufferSourceNode

function __widl_f_set_buffer_AudioBufferSourceNode(arg0, arg1) {
    getObject(arg0).buffer = getObject(arg1);
}
__exports.__widl_f_set_buffer_AudioBufferSourceNode = __widl_f_set_buffer_AudioBufferSourceNode

function __widl_f_new_AudioContext(exnptr) {
    try {
        return addHeapObject(new lAudioContext());
    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__widl_f_new_AudioContext = __widl_f_new_AudioContext

function __widl_f_create_buffer_AudioContext(arg0, arg1, arg2, arg3, exnptr) {
    try {
        return addHeapObject(getObject(arg0).createBuffer(arg1 >>> 0, arg2 >>> 0, arg3));
    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__widl_f_create_buffer_AudioContext = __widl_f_create_buffer_AudioContext

function __widl_f_create_buffer_source_AudioContext(arg0, exnptr) {
    try {
        return addHeapObject(getObject(arg0).createBufferSource());
    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__widl_f_create_buffer_source_AudioContext = __widl_f_create_buffer_source_AudioContext

function __widl_f_resume_AudioContext(arg0, exnptr) {
    try {
        return addHeapObject(getObject(arg0).resume());
    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__widl_f_resume_AudioContext = __widl_f_resume_AudioContext

function __widl_f_destination_AudioContext(arg0) {
    return addHeapObject(getObject(arg0).destination);
}
__exports.__widl_f_destination_AudioContext = __widl_f_destination_AudioContext

function __widl_f_sample_rate_AudioContext(arg0) {
    return getObject(arg0).sampleRate;
}
__exports.__widl_f_sample_rate_AudioContext = __widl_f_sample_rate_AudioContext

function __widl_f_connect_with_audio_node_AudioNode(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).connect(getObject(arg1)));
    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__widl_f_connect_with_audio_node_AudioNode = __widl_f_connect_with_audio_node_AudioNode

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function __widl_f_get_element_by_id_Document(arg0, arg1, arg2) {
    let varg1 = getStringFromWasm(arg1, arg2);

    const val = getObject(arg0).getElementById(varg1);
    return isLikeNone(val) ? 0 : addHeapObject(val);

}
__exports.__widl_f_get_element_by_id_Document = __widl_f_get_element_by_id_Document

function __widl_instanceof_HTMLCanvasElement(idx) { return getObject(idx) instanceof HTMLCanvasElement ? 1 : 0; }
__exports.__widl_instanceof_HTMLCanvasElement = __widl_instanceof_HTMLCanvasElement

function __widl_f_get_context_with_context_options_HTMLCanvasElement(arg0, arg1, arg2, arg3, exnptr) {
    let varg1 = getStringFromWasm(arg1, arg2);
    try {

        const val = getObject(arg0).getContext(varg1, getObject(arg3));
        return isLikeNone(val) ? 0 : addHeapObject(val);

    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__widl_f_get_context_with_context_options_HTMLCanvasElement = __widl_f_get_context_with_context_options_HTMLCanvasElement

function __widl_f_key_code_KeyboardEvent(arg0) {
    return getObject(arg0).keyCode;
}
__exports.__widl_f_key_code_KeyboardEvent = __widl_f_key_code_KeyboardEvent

function __widl_f_set_text_content_Node(arg0, arg1, arg2) {
    let varg1 = arg1 == 0 ? undefined : getStringFromWasm(arg1, arg2);
    getObject(arg0).textContent = varg1;
}
__exports.__widl_f_set_text_content_Node = __widl_f_set_text_content_Node

function __widl_instanceof_WebGLRenderingContext(idx) { return getObject(idx) instanceof WebGLRenderingContext ? 1 : 0; }
__exports.__widl_instanceof_WebGLRenderingContext = __widl_instanceof_WebGLRenderingContext

function __widl_f_buffer_data_with_array_buffer_view_WebGLRenderingContext(arg0, arg1, arg2, arg3) {
    getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
}
__exports.__widl_f_buffer_data_with_array_buffer_view_WebGLRenderingContext = __widl_f_buffer_data_with_array_buffer_view_WebGLRenderingContext

function getArrayU8FromWasm(ptr, len) {
    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);
}

function __widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array_WebGLRenderingContext(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, exnptr) {
    let varg9 = arg9 == 0 ? undefined : getArrayU8FromWasm(arg9, arg10);
    try {
        getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, varg9);
    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array_WebGLRenderingContext = __widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array_WebGLRenderingContext

function __widl_f_active_texture_WebGLRenderingContext(arg0, arg1) {
    getObject(arg0).activeTexture(arg1 >>> 0);
}
__exports.__widl_f_active_texture_WebGLRenderingContext = __widl_f_active_texture_WebGLRenderingContext

function __widl_f_attach_shader_WebGLRenderingContext(arg0, arg1, arg2) {
    getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
}
__exports.__widl_f_attach_shader_WebGLRenderingContext = __widl_f_attach_shader_WebGLRenderingContext

function __widl_f_bind_buffer_WebGLRenderingContext(arg0, arg1, arg2) {
    getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
}
__exports.__widl_f_bind_buffer_WebGLRenderingContext = __widl_f_bind_buffer_WebGLRenderingContext

function __widl_f_bind_texture_WebGLRenderingContext(arg0, arg1, arg2) {
    getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
}
__exports.__widl_f_bind_texture_WebGLRenderingContext = __widl_f_bind_texture_WebGLRenderingContext

function __widl_f_blend_func_WebGLRenderingContext(arg0, arg1, arg2) {
    getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);
}
__exports.__widl_f_blend_func_WebGLRenderingContext = __widl_f_blend_func_WebGLRenderingContext

function __widl_f_clear_WebGLRenderingContext(arg0, arg1) {
    getObject(arg0).clear(arg1 >>> 0);
}
__exports.__widl_f_clear_WebGLRenderingContext = __widl_f_clear_WebGLRenderingContext

function __widl_f_clear_color_WebGLRenderingContext(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
}
__exports.__widl_f_clear_color_WebGLRenderingContext = __widl_f_clear_color_WebGLRenderingContext

function __widl_f_compile_shader_WebGLRenderingContext(arg0, arg1) {
    getObject(arg0).compileShader(getObject(arg1));
}
__exports.__widl_f_compile_shader_WebGLRenderingContext = __widl_f_compile_shader_WebGLRenderingContext

function __widl_f_create_buffer_WebGLRenderingContext(arg0) {

    const val = getObject(arg0).createBuffer();
    return isLikeNone(val) ? 0 : addHeapObject(val);

}
__exports.__widl_f_create_buffer_WebGLRenderingContext = __widl_f_create_buffer_WebGLRenderingContext

function __widl_f_create_program_WebGLRenderingContext(arg0) {

    const val = getObject(arg0).createProgram();
    return isLikeNone(val) ? 0 : addHeapObject(val);

}
__exports.__widl_f_create_program_WebGLRenderingContext = __widl_f_create_program_WebGLRenderingContext

function __widl_f_create_shader_WebGLRenderingContext(arg0, arg1) {

    const val = getObject(arg0).createShader(arg1 >>> 0);
    return isLikeNone(val) ? 0 : addHeapObject(val);

}
__exports.__widl_f_create_shader_WebGLRenderingContext = __widl_f_create_shader_WebGLRenderingContext

function __widl_f_create_texture_WebGLRenderingContext(arg0) {

    const val = getObject(arg0).createTexture();
    return isLikeNone(val) ? 0 : addHeapObject(val);

}
__exports.__widl_f_create_texture_WebGLRenderingContext = __widl_f_create_texture_WebGLRenderingContext

function __widl_f_depth_func_WebGLRenderingContext(arg0, arg1) {
    getObject(arg0).depthFunc(arg1 >>> 0);
}
__exports.__widl_f_depth_func_WebGLRenderingContext = __widl_f_depth_func_WebGLRenderingContext

function __widl_f_draw_arrays_WebGLRenderingContext(arg0, arg1, arg2, arg3) {
    getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
}
__exports.__widl_f_draw_arrays_WebGLRenderingContext = __widl_f_draw_arrays_WebGLRenderingContext

function __widl_f_enable_WebGLRenderingContext(arg0, arg1) {
    getObject(arg0).enable(arg1 >>> 0);
}
__exports.__widl_f_enable_WebGLRenderingContext = __widl_f_enable_WebGLRenderingContext

function __widl_f_enable_vertex_attrib_array_WebGLRenderingContext(arg0, arg1) {
    getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
}
__exports.__widl_f_enable_vertex_attrib_array_WebGLRenderingContext = __widl_f_enable_vertex_attrib_array_WebGLRenderingContext

function __widl_f_get_attrib_location_WebGLRenderingContext(arg0, arg1, arg2, arg3) {
    let varg2 = getStringFromWasm(arg2, arg3);
    return getObject(arg0).getAttribLocation(getObject(arg1), varg2);
}
__exports.__widl_f_get_attrib_location_WebGLRenderingContext = __widl_f_get_attrib_location_WebGLRenderingContext

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

let passStringToWasm;
if (typeof cachedTextEncoder.encodeInto === 'function') {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            arg = arg.slice(offset);
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + arg.length * 3);
            const view = getUint8Memory().subarray(ptr + offset, ptr + size);
            const ret = cachedTextEncoder.encodeInto(arg, view);

            offset += ret.written;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
} else {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            const buf = cachedTextEncoder.encode(arg.slice(offset));
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + buf.length);
            getUint8Memory().set(buf, ptr + offset);
            offset += buf.length;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
}

function __widl_f_get_program_info_log_WebGLRenderingContext(ret, arg0, arg1) {
    const val = getObject(arg0).getProgramInfoLog(getObject(arg1));
    const retptr = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}
__exports.__widl_f_get_program_info_log_WebGLRenderingContext = __widl_f_get_program_info_log_WebGLRenderingContext

function __widl_f_get_program_parameter_WebGLRenderingContext(arg0, arg1, arg2) {
    return addHeapObject(getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0));
}
__exports.__widl_f_get_program_parameter_WebGLRenderingContext = __widl_f_get_program_parameter_WebGLRenderingContext

function __widl_f_get_shader_info_log_WebGLRenderingContext(ret, arg0, arg1) {
    const val = getObject(arg0).getShaderInfoLog(getObject(arg1));
    const retptr = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
    const retlen = WASM_VECTOR_LEN;
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

}
__exports.__widl_f_get_shader_info_log_WebGLRenderingContext = __widl_f_get_shader_info_log_WebGLRenderingContext

function __widl_f_get_shader_parameter_WebGLRenderingContext(arg0, arg1, arg2) {
    return addHeapObject(getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0));
}
__exports.__widl_f_get_shader_parameter_WebGLRenderingContext = __widl_f_get_shader_parameter_WebGLRenderingContext

function __widl_f_get_uniform_location_WebGLRenderingContext(arg0, arg1, arg2, arg3) {
    let varg2 = getStringFromWasm(arg2, arg3);

    const val = getObject(arg0).getUniformLocation(getObject(arg1), varg2);
    return isLikeNone(val) ? 0 : addHeapObject(val);

}
__exports.__widl_f_get_uniform_location_WebGLRenderingContext = __widl_f_get_uniform_location_WebGLRenderingContext

function __widl_f_link_program_WebGLRenderingContext(arg0, arg1) {
    getObject(arg0).linkProgram(getObject(arg1));
}
__exports.__widl_f_link_program_WebGLRenderingContext = __widl_f_link_program_WebGLRenderingContext

function __widl_f_shader_source_WebGLRenderingContext(arg0, arg1, arg2, arg3) {
    let varg2 = getStringFromWasm(arg2, arg3);
    getObject(arg0).shaderSource(getObject(arg1), varg2);
}
__exports.__widl_f_shader_source_WebGLRenderingContext = __widl_f_shader_source_WebGLRenderingContext

function __widl_f_tex_parameteri_WebGLRenderingContext(arg0, arg1, arg2, arg3) {
    getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
}
__exports.__widl_f_tex_parameteri_WebGLRenderingContext = __widl_f_tex_parameteri_WebGLRenderingContext

function __widl_f_uniform1f_WebGLRenderingContext(arg0, arg1, arg2) {
    getObject(arg0).uniform1f(getObject(arg1), arg2);
}
__exports.__widl_f_uniform1f_WebGLRenderingContext = __widl_f_uniform1f_WebGLRenderingContext

function __widl_f_uniform1i_WebGLRenderingContext(arg0, arg1, arg2) {
    getObject(arg0).uniform1i(getObject(arg1), arg2);
}
__exports.__widl_f_uniform1i_WebGLRenderingContext = __widl_f_uniform1i_WebGLRenderingContext

function __widl_f_uniform4f_WebGLRenderingContext(arg0, arg1, arg2, arg3, arg4, arg5) {
    getObject(arg0).uniform4f(getObject(arg1), arg2, arg3, arg4, arg5);
}
__exports.__widl_f_uniform4f_WebGLRenderingContext = __widl_f_uniform4f_WebGLRenderingContext

function __widl_f_use_program_WebGLRenderingContext(arg0, arg1) {
    getObject(arg0).useProgram(getObject(arg1));
}
__exports.__widl_f_use_program_WebGLRenderingContext = __widl_f_use_program_WebGLRenderingContext

function __widl_f_vertex_attrib_pointer_with_i32_WebGLRenderingContext(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
}
__exports.__widl_f_vertex_attrib_pointer_with_i32_WebGLRenderingContext = __widl_f_vertex_attrib_pointer_with_i32_WebGLRenderingContext

function __widl_instanceof_Window(idx) { return getObject(idx) instanceof Window ? 1 : 0; }
__exports.__widl_instanceof_Window = __widl_instanceof_Window

function __widl_f_request_animation_frame_Window(arg0, arg1, exnptr) {
    try {
        return getObject(arg0).requestAnimationFrame(getObject(arg1));
    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__widl_f_request_animation_frame_Window = __widl_f_request_animation_frame_Window

function __widl_f_document_Window(arg0) {

    const val = getObject(arg0).document;
    return isLikeNone(val) ? 0 : addHeapObject(val);

}
__exports.__widl_f_document_Window = __widl_f_document_Window

function __widl_f_set_onkeydown_Window(arg0, arg1) {
    getObject(arg0).onkeydown = getObject(arg1);
}
__exports.__widl_f_set_onkeydown_Window = __widl_f_set_onkeydown_Window

function __widl_f_set_onkeyup_Window(arg0, arg1) {
    getObject(arg0).onkeyup = getObject(arg1);
}
__exports.__widl_f_set_onkeyup_Window = __widl_f_set_onkeyup_Window

function __wbg_newnoargs_a172f39151049128(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
}
__exports.__wbg_newnoargs_a172f39151049128 = __wbg_newnoargs_a172f39151049128

function __wbg_call_8a9c8b0a32a202ff(arg0, arg1, exnptr) {
    try {
        return addHeapObject(getObject(arg0).call(getObject(arg1)));
    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__wbg_call_8a9c8b0a32a202ff = __wbg_call_8a9c8b0a32a202ff

function __wbg_new_68180085d411e1be() {
    return addHeapObject(new Object());
}
__exports.__wbg_new_68180085d411e1be = __wbg_new_68180085d411e1be

function __wbg_new_0c9fa29a7eadebb7(arg0) {
    return addHeapObject(new Float32Array(getObject(arg0)));
}
__exports.__wbg_new_0c9fa29a7eadebb7 = __wbg_new_0c9fa29a7eadebb7

function __wbg_subarray_e11b44a1654f24d3(arg0, arg1, arg2) {
    return addHeapObject(getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0));
}
__exports.__wbg_subarray_e11b44a1654f24d3 = __wbg_subarray_e11b44a1654f24d3

function __wbg_set_8866dbb36cf947cb(arg0, arg1, arg2, exnptr) {
    try {
        return Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
    } catch (e) {
        handleError(exnptr, e);
    }
}
__exports.__wbg_set_8866dbb36cf947cb = __wbg_set_8866dbb36cf947cb

function __wbg_instanceof_Memory_2dd1115524c7b73a(idx) { return getObject(idx) instanceof WebAssembly.Memory ? 1 : 0; }
__exports.__wbg_instanceof_Memory_2dd1115524c7b73a = __wbg_instanceof_Memory_2dd1115524c7b73a

function __wbg_buffer_0b401f8e593a961e(arg0) {
    return addHeapObject(getObject(arg0).buffer);
}
__exports.__wbg_buffer_0b401f8e593a961e = __wbg_buffer_0b401f8e593a961e

function __wbindgen_string_new(p, l) { return addHeapObject(getStringFromWasm(p, l)); }
__exports.__wbindgen_string_new = __wbindgen_string_new

function __wbindgen_boolean_get(i) {
    let v = getObject(i);
    return typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
}
__exports.__wbindgen_boolean_get = __wbindgen_boolean_get

function __wbindgen_debug_string(i, len_ptr) {
    const debug_str =
    val => {
        // primitive types
        const type = typeof val;
        if (type == 'number' || type == 'boolean' || val == null) {
            return  `${val}`;
        }
        if (type == 'string') {
            return `"${val}"`;
        }
        if (type == 'symbol') {
            const description = val.description;
            if (description == null) {
                return 'Symbol';
            } else {
                return `Symbol(${description})`;
            }
        }
        if (type == 'function') {
            const name = val.name;
            if (typeof name == 'string' && name.length > 0) {
                return `Function(${name})`;
            } else {
                return 'Function';
            }
        }
        // objects
        if (Array.isArray(val)) {
            const length = val.length;
            let debug = '[';
            if (length > 0) {
                debug += debug_str(val[0]);
            }
            for(let i = 1; i < length; i++) {
                debug += ', ' + debug_str(val[i]);
            }
            debug += ']';
            return debug;
        }
        // Test for built-in
        const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
        let className;
        if (builtInMatches.length > 1) {
            className = builtInMatches[1];
        } else {
            // Failed to match the standard '[object ClassName]'
            return toString.call(val);
        }
        if (className == 'Object') {
            // we're a user defined class or Object
            // JSON.stringify avoids problems with cycles, and is generally much
            // easier than looping through ownProperties of `val`.
            try {
                return 'Object(' + JSON.stringify(val) + ')';
            } catch (_) {
                return 'Object';
            }
        }
        // errors
        if (val instanceof Error) {
        return `${val.name}: ${val.message}
        ${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}
;
const toString = Object.prototype.toString;
const val = getObject(i);
const debug = debug_str(val);
const ptr = passStringToWasm(debug);
getUint32Memory()[len_ptr / 4] = WASM_VECTOR_LEN;
return ptr;
}
__exports.__wbindgen_debug_string = __wbindgen_debug_string

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

function __wbindgen_cb_drop(i) {
    const obj = takeObject(i).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return 1;
    }
    return 0;
}
__exports.__wbindgen_cb_drop = __wbindgen_cb_drop

const __wbindgen_cb_forget = dropObject;
__exports.__wbindgen_cb_forget = __wbindgen_cb_forget;

function __wbindgen_memory() { return addHeapObject(wasm.memory); }
__exports.__wbindgen_memory = __wbindgen_memory

function __wbindgen_rethrow(idx) { throw takeObject(idx); }
__exports.__wbindgen_rethrow = __wbindgen_rethrow

function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}
__exports.__wbindgen_throw = __wbindgen_throw

function __wbindgen_closure_wrapper156(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(32);
    const d = wasm.__wbg_function_table.get(33);
    const cb = function(arg0) {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b, arg0);

        } finally {
            if (--this.cnt === 0) d(a, b);
            else this.a = a;

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
}
__exports.__wbindgen_closure_wrapper156 = __wbindgen_closure_wrapper156

function __wbindgen_closure_wrapper158(a, b, _ignored) {
    const f = wasm.__wbg_function_table.get(29);
    const d = wasm.__wbg_function_table.get(30);
    const cb = function(arg0) {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b, addHeapObject(arg0));

        } finally {
            if (--this.cnt === 0) d(a, b);
            else this.a = a;

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
}
__exports.__wbindgen_closure_wrapper158 = __wbindgen_closure_wrapper158

function __wbindgen_object_clone_ref(idx) {
    return addHeapObject(getObject(idx));
}
__exports.__wbindgen_object_clone_ref = __wbindgen_object_clone_ref

function __wbindgen_object_drop_ref(i) { dropObject(i); }
__exports.__wbindgen_object_drop_ref = __wbindgen_object_drop_ref

function init(module) {
    let result;
    const imports = { './wasm_pong': __exports };

    if (module instanceof URL || typeof module === 'string' || module instanceof Request) {

        const response = fetch(module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module, imports)
        .then(result => {
            if (result instanceof WebAssembly.Instance) {
                return { instance: result, module };
            } else {
                return result;
            }
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;
        wasm.__wbindgen_start();
        return wasm;
    });
}

export default init;

