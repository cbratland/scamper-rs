import { createEditor, createDiagnostic, createSplit } from './snippets/web-67aca56aae793faf/build/index.js';
import * as __wbg_star0 from './snippets/web-67aca56aae793faf/build/index.js';
import * as __wbg_star1 from './snippets/web-67aca56aae793faf/public/js/highlight.js';

let wasm;

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function debugString(val) {
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
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
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
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let cachedFloat64ArrayMemory0 = null;

function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

function getArrayF64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => {
    wasm.__wbindgen_export_4.get(state.dtor)(state.a, state.b)
});

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_4.get(state.dtor)(a, state.b);
                CLOSURE_DTORS.unregister(state);
            } else {
                state.a = a;
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}
function __wbg_adapter_42(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hdd5bc4b4eeb64ab6(arg0, arg1);
}

function __wbg_adapter_45(arg0, arg1, arg2) {
    wasm.closure263_externref_shim(arg0, arg1, arg2);
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_4.get(state.dtor)(state.a, state.b);
                state.a = 0;
                CLOSURE_DTORS.unregister(state);
            }
        }
    };
    real.original = state;
    CLOSURE_DTORS.register(real, state, state);
    return real;
}
function __wbg_adapter_48(arg0, arg1, arg2) {
    wasm.closure267_externref_shim(arg0, arg1, arg2);
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_export_3.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}
function __wbg_adapter_51(arg0, arg1, arg2) {
    const ret = wasm.closure270_externref_shim_multivalue_shim(arg0, arg1, arg2);
    var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v1;
}

function __wbg_adapter_58(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h3139a25345eb112b(arg0, arg1);
}

function __wbg_adapter_61(arg0, arg1, arg2) {
    wasm.closure813_externref_shim(arg0, arg1, arg2);
}

function __wbg_adapter_64(arg0, arg1) {
    wasm._dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9d24d50675a459c4(arg0, arg1);
}

function __wbg_adapter_67(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hc12e9961a5f712a1(arg0, arg1);
}

function __wbg_adapter_70(arg0, arg1, arg2) {
    wasm.closure877_externref_shim(arg0, arg1, arg2);
}

function getFromExternrefTable0(idx) { return wasm.__wbindgen_export_3.get(idx); }

function getCachedStringFromWasm0(ptr, len) {
    if (ptr === 0) {
        return getFromExternrefTable0(len);
    } else {
        return getStringFromWasm0(ptr, len);
    }
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_3.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }
function __wbg_adapter_441(arg0, arg1, arg2, arg3) {
    wasm.closure898_externref_shim(arg0, arg1, arg2, arg3);
}

const __wbindgen_enum_BinaryType = ["blob", "arraybuffer"];

const __wbindgen_enum_ReadableStreamReaderMode = ["byob"];

const __wbindgen_enum_ReadableStreamType = ["bytes"];

const __wbindgen_enum_ReferrerPolicy = ["", "no-referrer", "no-referrer-when-downgrade", "origin", "origin-when-cross-origin", "unsafe-url", "same-origin", "strict-origin", "strict-origin-when-cross-origin"];

const __wbindgen_enum_RequestCache = ["default", "no-store", "reload", "no-cache", "force-cache", "only-if-cached"];

const __wbindgen_enum_RequestCredentials = ["omit", "same-origin", "include"];

const __wbindgen_enum_RequestMode = ["same-origin", "no-cors", "cors", "navigate"];

const __wbindgen_enum_RequestRedirect = ["follow", "error", "manual"];

const __wbindgen_enum_ResponseType = ["basic", "cors", "default", "error", "opaque", "opaqueredirect"];

const __wbindgen_enum_ShadowRootMode = ["open", "closed"];

const IntoUnderlyingByteSourceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_intounderlyingbytesource_free(ptr >>> 0, 1));

export class IntoUnderlyingByteSource {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IntoUnderlyingByteSourceFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_intounderlyingbytesource_free(ptr, 0);
    }
    /**
     * @returns {string}
     */
    get type() {
        const ret = wasm.intounderlyingbytesource_type(this.__wbg_ptr);
        var v1 = getCachedStringFromWasm0(ret[0], ret[1]);
    if (ret[0] !== 0) { wasm.__wbindgen_free(ret[0], ret[1], 1); }
    return v1;
}
/**
 * @returns {number}
 */
get autoAllocateChunkSize() {
    const ret = wasm.intounderlyingbytesource_autoAllocateChunkSize(this.__wbg_ptr);
    return ret >>> 0;
}
/**
 * @param {ReadableByteStreamController} controller
 */
start(controller) {
    wasm.intounderlyingbytesource_start(this.__wbg_ptr, controller);
}
/**
 * @param {ReadableByteStreamController} controller
 * @returns {Promise<any>}
 */
pull(controller) {
    const ret = wasm.intounderlyingbytesource_pull(this.__wbg_ptr, controller);
    return ret;
}
cancel() {
    const ptr = this.__destroy_into_raw();
    wasm.intounderlyingbytesource_cancel(ptr);
}
}

const IntoUnderlyingSinkFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_intounderlyingsink_free(ptr >>> 0, 1));

export class IntoUnderlyingSink {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IntoUnderlyingSinkFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_intounderlyingsink_free(ptr, 0);
    }
    /**
     * @param {any} chunk
     * @returns {Promise<any>}
     */
    write(chunk) {
        const ret = wasm.intounderlyingsink_write(this.__wbg_ptr, chunk);
        return ret;
    }
    /**
     * @returns {Promise<any>}
     */
    close() {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.intounderlyingsink_close(ptr);
        return ret;
    }
    /**
     * @param {any} reason
     * @returns {Promise<any>}
     */
    abort(reason) {
        const ptr = this.__destroy_into_raw();
        const ret = wasm.intounderlyingsink_abort(ptr, reason);
        return ret;
    }
}

const IntoUnderlyingSourceFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_intounderlyingsource_free(ptr >>> 0, 1));

export class IntoUnderlyingSource {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        IntoUnderlyingSourceFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_intounderlyingsource_free(ptr, 0);
    }
    /**
     * @param {ReadableStreamDefaultController} controller
     * @returns {Promise<any>}
     */
    pull(controller) {
        const ret = wasm.intounderlyingsource_pull(this.__wbg_ptr, controller);
        return ret;
    }
    cancel() {
        const ptr = this.__destroy_into_raw();
        wasm.intounderlyingsource_cancel(ptr);
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_new_abda76e883ba8a5f = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_stack_658279fe44541cf6 = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_error_f851667af71bcfc6 = function(arg0, arg1) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
    if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1, 1); }
    console.error(v0);
};
imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
};
imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbindgen_cb_drop = function(arg0) {
    const obj = arg0.original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    const ret = false;
    return ret;
};
imports.wbg.__wbg_createEditor_6f42b05454c58762 = function(arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = createEditor(v0, arg2, arg3, arg4);
    return ret;
};
imports.wbg.__wbg_getdoc_f843d9d1ea01e836 = function(arg0, arg1) {
    const ret = arg1.get_doc();
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbindgen_boolean_get = function(arg0) {
    const v = arg0;
    const ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
    return ret;
};
imports.wbg.__wbg_stop_4cc53d725601dbef = function(arg0) {
    arg0.stop();
};
imports.wbg.__wbg_createDiagnostic_88e51a2f00d29e56 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
if (arg2 !== 0) { wasm.__wbindgen_free(arg2, arg3, 1); }
var v1 = getCachedStringFromWasm0(arg4, arg5);
if (arg4 !== 0) { wasm.__wbindgen_free(arg4, arg5, 1); }
const ret = createDiagnostic(arg0 >>> 0, arg1 >>> 0, v0, v1);
return ret;
};
imports.wbg.__wbg_createSplit_0e8daef7adba1bde = function(arg0, arg1, arg2, arg3) {
    var v0 = getArrayJsValueFromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 4, 4);
    var v1 = getArrayF64FromWasm0(arg2, arg3).slice();
    wasm.__wbindgen_free(arg2, arg3 * 8, 8);
    createSplit(v0, v1);
};
imports.wbg.__wbg_playNote_9a4c46c17d90cad5 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.playNote(arg1, arg2, arg3, arg4, arg5);
};
imports.wbg.__wbg_getTime_f91315f7e371df41 = function(arg0) {
    const ret = arg0.getTime();
    return ret;
};
imports.wbg.__wbg_crypto_1d1f22824a6a080c = function(arg0) {
    const ret = arg0.crypto;
    return ret;
};
imports.wbg.__wbindgen_is_object = function(arg0) {
    const val = arg0;
    const ret = typeof(val) === 'object' && val !== null;
    return ret;
};
imports.wbg.__wbg_process_4a72847cc503995b = function(arg0) {
    const ret = arg0.process;
    return ret;
};
imports.wbg.__wbg_versions_f686565e586dd935 = function(arg0) {
    const ret = arg0.versions;
    return ret;
};
imports.wbg.__wbg_node_104a2ff8d6ea03a2 = function(arg0) {
    const ret = arg0.node;
    return ret;
};
imports.wbg.__wbindgen_is_string = function(arg0) {
    const ret = typeof(arg0) === 'string';
    return ret;
};
imports.wbg.__wbg_require_cca90b1a94a0255b = function() { return handleError(function () {
    const ret = module.require;
    return ret;
}, arguments) };
imports.wbg.__wbindgen_is_function = function(arg0) {
    const ret = typeof(arg0) === 'function';
    return ret;
};
imports.wbg.__wbg_msCrypto_eb05e62b530a1508 = function(arg0) {
    const ret = arg0.msCrypto;
    return ret;
};
imports.wbg.__wbg_randomFillSync_5c9c955aa56b6049 = function() { return handleError(function (arg0, arg1) {
    arg0.randomFillSync(arg1);
}, arguments) };
imports.wbg.__wbg_getRandomValues_3aa56aa6edec874c = function() { return handleError(function (arg0, arg1) {
    arg0.getRandomValues(arg1);
}, arguments) };
imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
    const ret = arg0 === arg1;
    return ret;
};
imports.wbg.__wbindgen_is_null = function(arg0) {
    const ret = arg0 === null;
    return ret;
};
imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
    const obj = arg1;
    const ret = typeof(obj) === 'number' ? obj : undefined;
    getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
};
imports.wbg.__wbindgen_is_undefined = function(arg0) {
    const ret = arg0 === undefined;
    return ret;
};
imports.wbg.__wbindgen_is_falsy = function(arg0) {
    const ret = !arg0;
    return ret;
};
imports.wbg.__wbg_classList_865deb8c9db0f67a = function(arg0) {
    const ret = arg0.classList;
    return ret;
};
imports.wbg.__wbg_setinnerHTML_559d45055154f1d8 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.innerHTML = v0;
};
imports.wbg.__wbg_getAttribute_8ac49f4186f4cefd = function(arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    const ret = arg1.getAttribute(v0);
    var ptr2 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len2, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr2, true);
};
imports.wbg.__wbg_hasAttribute_df717f416620be1e = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = arg0.hasAttribute(v0);
    return ret;
};
imports.wbg.__wbg_removeAttribute_2dc68056b5e6ea3d = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.removeAttribute(v0);
}, arguments) };
imports.wbg.__wbg_scrollIntoView_dce310a559f74fe7 = function(arg0) {
    arg0.scrollIntoView();
};
imports.wbg.__wbg_setAttribute_2a8f647a8d92c712 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    arg0.setAttribute(v0, v1);
}, arguments) };
imports.wbg.__wbg_before_3350442e02ed9f52 = function() { return handleError(function (arg0, arg1) {
    arg0.before(arg1);
}, arguments) };
imports.wbg.__wbg_remove_d7a18d9f46bc50fd = function(arg0) {
    arg0.remove();
};
imports.wbg.__wbg_append_95ebd1cfe732a3e6 = function() { return handleError(function (arg0, arg1) {
    arg0.append(arg1);
}, arguments) };
imports.wbg.__wbg_body_8e909b791b1745d3 = function(arg0) {
    const ret = arg0.body;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_head_01a058f7b7d3cd52 = function(arg0) {
    const ret = arg0.head;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_createComment_91ba91f80deb16bd = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = arg0.createComment(v0);
    return ret;
};
imports.wbg.__wbg_createDocumentFragment_f0be9d8f1abfac54 = function(arg0) {
    const ret = arg0.createDocumentFragment();
    return ret;
};
imports.wbg.__wbg_createElement_e4523490bd0ae51d = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = arg0.createElement(v0);
    return ret;
}, arguments) };
imports.wbg.__wbg_createTextNode_3b33c97f8ef3e999 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = arg0.createTextNode(v0);
    return ret;
};
imports.wbg.__wbg_getElementById_734c4eac4fec5911 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = arg0.getElementById(v0);
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_instanceof_Window_6575cd7f1322f82f = function(arg0) {
    let result;
    try {
        result = arg0 instanceof Window;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_document_d7fa2c739c2b191a = function(arg0) {
    const ret = arg0.document;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_location_72721055fbff81f2 = function(arg0) {
    const ret = arg0.location;
    return ret;
};
imports.wbg.__wbg_history_95935eecf7ecc279 = function() { return handleError(function (arg0) {
    const ret = arg0.history;
    return ret;
}, arguments) };
imports.wbg.__wbg_navigator_3d3836196a5d8e62 = function(arg0) {
    const ret = arg0.navigator;
    return ret;
};
imports.wbg.__wbg_prompt_4782eebe173b4d54 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    const ret = arg1.prompt(v0);
    var ptr2 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len2 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len2, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr2, true);
}, arguments) };
imports.wbg.__wbg_scrollTo_348ad9b3fa67341f = function(arg0, arg1, arg2) {
    arg0.scrollTo(arg1, arg2);
};
imports.wbg.__wbg_requestAnimationFrame_8c3436f4ac89bc48 = function() { return handleError(function (arg0, arg1) {
    const ret = arg0.requestAnimationFrame(arg1);
    return ret;
}, arguments) };
imports.wbg.__wbg_clearInterval_538773bf49791f6f = function(arg0, arg1) {
    arg0.clearInterval(arg1);
};
imports.wbg.__wbg_clearTimeout_8567b0ecb6ec5d60 = function(arg0, arg1) {
    arg0.clearTimeout(arg1);
};
imports.wbg.__wbg_setInterval_4dcf8a1b846034db = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.setInterval(arg1, arg2);
    return ret;
}, arguments) };
imports.wbg.__wbg_setTimeout_e5d5b865335ce177 = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.setTimeout(arg1, arg2);
    return ret;
}, arguments) };
imports.wbg.__wbg_addEventListener_4357f9b7b3826784 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.addEventListener(v0, arg3);
}, arguments) };
imports.wbg.__wbg_addEventListener_0ac72681badaf1aa = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.addEventListener(v0, arg3, arg4);
}, arguments) };
imports.wbg.__wbg_removeEventListener_4c13d11156153514 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.removeEventListener(v0, arg3);
}, arguments) };
imports.wbg.__wbg_instanceof_HtmlElement_aab18e065dc9207d = function(arg0) {
    let result;
    try {
        result = arg0 instanceof HTMLElement;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_style_04eb1488bc2ceffc = function(arg0) {
    const ret = arg0.style;
    return ret;
};
imports.wbg.__wbg_click_f078705a1e3d47a8 = function(arg0) {
    arg0.click();
};
imports.wbg.__wbg_newwithstrsequenceandoptions_3d581ce16ca52c44 = function() { return handleError(function (arg0, arg1) {
    const ret = new Blob(arg0, arg1);
    return ret;
}, arguments) };
imports.wbg.__wbg_text_770c62e631da845e = function(arg0) {
    const ret = arg0.text();
    return ret;
};
imports.wbg.__wbg_append_2d052bdf2a44d6e4 = function() { return handleError(function (arg0, arg1) {
    arg0.append(arg1);
}, arguments) };
imports.wbg.__wbg_instanceof_FileSystemDirectoryHandle_da8256f5fca2a69c = function(arg0) {
    let result;
    try {
        result = arg0 instanceof FileSystemDirectoryHandle;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_entries_50f4d9eca1f3cfa9 = function(arg0) {
    const ret = arg0.entries();
    return ret;
};
imports.wbg.__wbg_getFileHandle_956d073fb81b0418 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = arg0.getFileHandle(v0);
    return ret;
};
imports.wbg.__wbg_getFileHandle_05cfec70df42dae6 = function(arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = arg0.getFileHandle(v0, arg3);
    return ret;
};
imports.wbg.__wbg_removeEntry_941245b7dfb6f679 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = arg0.removeEntry(v0);
    return ret;
};
imports.wbg.__wbg_getFile_8d1525a22e6ea015 = function(arg0) {
    const ret = arg0.getFile();
    return ret;
};
imports.wbg.__wbg_setonmessage_7e6ff33e920fdb07 = function(arg0, arg1) {
    arg0.onmessage = arg1;
};
imports.wbg.__wbg_new_00d033f8a8736a28 = function() { return handleError(function (arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Worker(v0);
    return ret;
}, arguments) };
imports.wbg.__wbg_postMessage_49334e5d7d9cc421 = function() { return handleError(function (arg0, arg1) {
    arg0.postMessage(arg1);
}, arguments) };
imports.wbg.__wbg_removeProperty_5acbca68235d0706 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    const ret = arg1.removeProperty(v0);
    const ptr2 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len2 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len2, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr2, true);
}, arguments) };
imports.wbg.__wbg_setProperty_b9a2384cbfb499fb = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    arg0.setProperty(v0, v1);
}, arguments) };
imports.wbg.__wbg_new_316590394dc4853b = function() { return handleError(function () {
    const ret = new Range();
    return ret;
}, arguments) };
imports.wbg.__wbg_deleteContents_30ea3b34e71dad6f = function() { return handleError(function (arg0) {
    arg0.deleteContents();
}, arguments) };
imports.wbg.__wbg_setEndBefore_27caf1489575dde7 = function() { return handleError(function (arg0, arg1) {
    arg0.setEndBefore(arg1);
}, arguments) };
imports.wbg.__wbg_setStartBefore_aac408e4ac84af7f = function() { return handleError(function (arg0, arg1) {
    arg0.setStartBefore(arg1);
}, arguments) };
imports.wbg.__wbg_setdata_ccbac292cd5e0fc0 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.data = v0;
};
imports.wbg.__wbg_href_a78089b3b726e0af = function() { return handleError(function (arg0, arg1) {
    const ret = arg1.href;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };
imports.wbg.__wbg_sethref_54265015953e6e04 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.href = v0;
}, arguments) };
imports.wbg.__wbg_origin_1830c25dfb01148b = function() { return handleError(function (arg0, arg1) {
    const ret = arg1.origin;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };
imports.wbg.__wbg_pathname_f807053b46d955a7 = function() { return handleError(function (arg0, arg1) {
    const ret = arg1.pathname;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };
imports.wbg.__wbg_search_aaeccdb8c45f3efa = function() { return handleError(function (arg0, arg1) {
    const ret = arg1.search;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };
imports.wbg.__wbg_hash_acef7ae4422b13b0 = function() { return handleError(function (arg0, arg1) {
    const ret = arg1.hash;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };
imports.wbg.__wbg_close_cfd08d9cf9f36856 = function() { return handleError(function (arg0) {
    arg0.close();
}, arguments) };
imports.wbg.__wbg_enqueue_e693a6fb4f3261c1 = function() { return handleError(function (arg0, arg1) {
    arg0.enqueue(arg1);
}, arguments) };
imports.wbg.__wbg_target_b0499015ea29563d = function(arg0) {
    const ret = arg0.target;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_defaultPrevented_29b1516330ff9603 = function(arg0) {
    const ret = arg0.defaultPrevented;
    return ret;
};
imports.wbg.__wbg_cancelBubble_1fc3632e2ba513ed = function(arg0) {
    const ret = arg0.cancelBubble;
    return ret;
};
imports.wbg.__wbg_composedPath_d27a772830ab5dd0 = function(arg0) {
    const ret = arg0.composedPath();
    return ret;
};
imports.wbg.__wbg_preventDefault_eecc4a63e64c4526 = function(arg0) {
    arg0.preventDefault();
};
imports.wbg.__wbg_stopPropagation_8a8fc87824cc6f0b = function(arg0) {
    arg0.stopPropagation();
};
imports.wbg.__wbg_ctrlKey_4015247a39aa9410 = function(arg0) {
    const ret = arg0.ctrlKey;
    return ret;
};
imports.wbg.__wbg_shiftKey_6d843f3032fd0366 = function(arg0) {
    const ret = arg0.shiftKey;
    return ret;
};
imports.wbg.__wbg_altKey_c9401b041949ea90 = function(arg0) {
    const ret = arg0.altKey;
    return ret;
};
imports.wbg.__wbg_metaKey_5d680933661ea1ea = function(arg0) {
    const ret = arg0.metaKey;
    return ret;
};
imports.wbg.__wbg_button_d8226b772c8cf494 = function(arg0) {
    const ret = arg0.button;
    return ret;
};
imports.wbg.__wbg_byobRequest_86ac467c94924d3c = function(arg0) {
    const ret = arg0.byobRequest;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_close_7cda9dd901230214 = function() { return handleError(function (arg0) {
    arg0.close();
}, arguments) };
imports.wbg.__wbg_error_53abcd6a461f73d8 = typeof console.error == 'function' ? console.error : notDefined('console.error');
imports.wbg.__wbg_warn_41503a1c2194de89 = typeof console.warn == 'function' ? console.warn : notDefined('console.warn');
imports.wbg.__wbg_instanceof_HtmlAnchorElement_e47c33c680406d32 = function(arg0) {
    let result;
    try {
        result = arg0 instanceof HTMLAnchorElement;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_target_92543a86c7612559 = function(arg0, arg1) {
    const ret = arg1.target;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_href_3bcf489ff26fb1d0 = function(arg0, arg1) {
    const ret = arg1.href;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_width_e4c18791794a7c38 = function(arg0) {
    const ret = arg0.width;
    return ret;
};
imports.wbg.__wbg_actualBoundingBoxAscent_e4665126f079f915 = function(arg0) {
    const ret = arg0.actualBoundingBoxAscent;
    return ret;
};
imports.wbg.__wbg_actualBoundingBoxDescent_a5bc578c1efd04bb = function(arg0) {
    const ret = arg0.actualBoundingBoxDescent;
    return ret;
};
imports.wbg.__wbg_instanceof_CanvasRenderingContext2d_775df7bd32f07559 = function(arg0) {
    let result;
    try {
        result = arg0 instanceof CanvasRenderingContext2D;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_setstrokeStyle_3c29a4e85b6087f5 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.strokeStyle = v0;
};
imports.wbg.__wbg_setfillStyle_2cc2c748b938a95e = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.fillStyle = v0;
};
imports.wbg.__wbg_setfont_669f9943743a4efe = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.font = v0;
};
imports.wbg.__wbg_settextAlign_6f0a2b262f5f8bf0 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.textAlign = v0;
};
imports.wbg.__wbg_beginPath_03b82752a91dba4b = function(arg0) {
    arg0.beginPath();
};
imports.wbg.__wbg_fill_b7e7fd440fcd53b1 = function(arg0) {
    arg0.fill();
};
imports.wbg.__wbg_stroke_8b530d51b796d0df = function(arg0) {
    arg0.stroke();
};
imports.wbg.__wbg_ellipse_ad0bddf007c5e28c = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    arg0.ellipse(arg1, arg2, arg3, arg4, arg5, arg6, arg7);
}, arguments) };
imports.wbg.__wbg_lineTo_1da60c4e058c338e = function(arg0, arg1, arg2) {
    arg0.lineTo(arg1, arg2);
};
imports.wbg.__wbg_moveTo_8756b579ffc530b4 = function(arg0, arg1, arg2) {
    arg0.moveTo(arg1, arg2);
};
imports.wbg.__wbg_setLineDash_a27082dd66f071a9 = function() { return handleError(function (arg0, arg1) {
    arg0.setLineDash(arg1);
}, arguments) };
imports.wbg.__wbg_fillRect_6784ab0aab9eebd5 = function(arg0, arg1, arg2, arg3, arg4) {
    arg0.fillRect(arg1, arg2, arg3, arg4);
};
imports.wbg.__wbg_strokeRect_6cca2fd41979dbb5 = function(arg0, arg1, arg2, arg3, arg4) {
    arg0.strokeRect(arg1, arg2, arg3, arg4);
};
imports.wbg.__wbg_fillText_285687ced8ee535f = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.fillText(v0, arg3, arg4);
}, arguments) };
imports.wbg.__wbg_measureText_349ff768850cea82 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = arg0.measureText(v0);
    return ret;
}, arguments) };
imports.wbg.__wbg_rotate_a47ef597ad8ee4c5 = function() { return handleError(function (arg0, arg1) {
    arg0.rotate(arg1);
}, arguments) };
imports.wbg.__wbg_translate_d170860b2494cc19 = function() { return handleError(function (arg0, arg1, arg2) {
    arg0.translate(arg1, arg2);
}, arguments) };
imports.wbg.__wbg_data_134d3a704b9fca32 = function(arg0) {
    const ret = arg0.data;
    return ret;
};
imports.wbg.__wbg_view_de0e81c5c00d2129 = function(arg0) {
    const ret = arg0.view;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_respond_ffb6928cd9b79c32 = function() { return handleError(function (arg0, arg1) {
    arg0.respond(arg1 >>> 0);
}, arguments) };
imports.wbg.__wbg_state_ea7aeeadc8019f77 = function() { return handleError(function (arg0) {
    const ret = arg0.state;
    return ret;
}, arguments) };
imports.wbg.__wbg_pushState_fd9ad18c3fdad921 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    var v1 = getCachedStringFromWasm0(arg4, arg5);
    arg0.pushState(arg1, v0, v1);
}, arguments) };
imports.wbg.__wbg_replaceState_590d6294219f655e = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    var v1 = getCachedStringFromWasm0(arg4, arg5);
    arg0.replaceState(arg1, v0, v1);
}, arguments) };
imports.wbg.__wbg_instanceof_HtmlCanvasElement_022ad88c76df9031 = function(arg0) {
    let result;
    try {
        result = arg0 instanceof HTMLCanvasElement;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_setwidth_23bf2deedd907275 = function(arg0, arg1) {
    arg0.width = arg1 >>> 0;
};
imports.wbg.__wbg_setheight_239dc283bbe50da4 = function(arg0, arg1) {
    arg0.height = arg1 >>> 0;
};
imports.wbg.__wbg_getContext_bf8985355a4d22ca = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = arg0.getContext(v0);
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
}, arguments) };
imports.wbg.__wbg_instanceof_ShadowRoot_6d00cedbc919c9a6 = function(arg0) {
    let result;
    try {
        result = arg0 instanceof ShadowRoot;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};
imports.wbg.__wbg_host_4a0b95cc36a45cb6 = function(arg0) {
    const ret = arg0.host;
    return ret;
};
imports.wbg.__wbg_settype_623d2ee701e6310a = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.type = v0;
};
imports.wbg.__wbg_setcreate_487c8857ba35bbda = function(arg0, arg1) {
    arg0.create = arg1 !== 0;
};
imports.wbg.__wbg_storage_91bb06ceeb7e37bf = function(arg0) {
    const ret = arg0.storage;
    return ret;
};
imports.wbg.__wbg_href_07ab8fba72e97d85 = function(arg0, arg1) {
    const ret = arg1.href;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_origin_e03d684beeb7ffe4 = function(arg0, arg1) {
    const ret = arg1.origin;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_pathname_e2ffbf8ec6773a59 = function(arg0, arg1) {
    const ret = arg1.pathname;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_search_b46ea3b7e7b1866c = function(arg0, arg1) {
    const ret = arg1.search;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_searchParams_a766192bfbbda6e5 = function(arg0) {
    const ret = arg0.searchParams;
    return ret;
};
imports.wbg.__wbg_hash_cabc4c43a4d7e941 = function(arg0, arg1) {
    const ret = arg1.hash;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_newwithbase_ba00450eb5df91c3 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    var v1 = getCachedStringFromWasm0(arg2, arg3);
    const ret = new URL(v0, v1);
    return ret;
}, arguments) };
imports.wbg.__wbg_createObjectURL_11804d71ac214694 = function() { return handleError(function (arg0, arg1) {
    const ret = URL.createObjectURL(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments) };
imports.wbg.__wbg_revokeObjectURL_8e72bad4541bdca0 = function() { return handleError(function (arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    URL.revokeObjectURL(v0);
}, arguments) };
imports.wbg.__wbg_add_dfb70ffb1d8bc2a5 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.add(v0);
}, arguments) };
imports.wbg.__wbg_remove_dc3dc335e5308e36 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.remove(v0);
}, arguments) };
imports.wbg.__wbg_name_e9303afdc96872b2 = function(arg0, arg1) {
    const ret = arg1.name;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbg_parentNode_7e7d8adc9b41ce58 = function(arg0) {
    const ret = arg0.parentNode;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_childNodes_87c5e311593a6192 = function(arg0) {
    const ret = arg0.childNodes;
    return ret;
};
imports.wbg.__wbg_previousSibling_5fbe2684a49cc571 = function(arg0) {
    const ret = arg0.previousSibling;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_nextSibling_46da01c3a2ce3774 = function(arg0) {
    const ret = arg0.nextSibling;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_settextContent_f9c4b60e6c009ea2 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    arg0.textContent = v0;
};
imports.wbg.__wbg_appendChild_bc4a0deae90a5164 = function() { return handleError(function (arg0, arg1) {
    const ret = arg0.appendChild(arg1);
    return ret;
}, arguments) };
imports.wbg.__wbg_cloneNode_bd4b7e47afe3ce9f = function() { return handleError(function (arg0) {
    const ret = arg0.cloneNode();
    return ret;
}, arguments) };
imports.wbg.__wbg_contains_a28a8f7c01e4c130 = function(arg0, arg1) {
    const ret = arg0.contains(arg1);
    return ret;
};
imports.wbg.__wbg_removeChild_aa85e67649730769 = function() { return handleError(function (arg0, arg1) {
    const ret = arg0.removeChild(arg1);
    return ret;
}, arguments) };
imports.wbg.__wbg_length_9a6b70327f5f86e1 = function(arg0) {
    const ret = arg0.length;
    return ret;
};
imports.wbg.__wbg_getDirectory_af54cd258fcb9cff = function(arg0) {
    const ret = arg0.getDirectory();
    return ret;
};
imports.wbg.__wbg_queueMicrotask_848aa4969108a57e = function(arg0) {
    const ret = arg0.queueMicrotask;
    return ret;
};
imports.wbg.__wbg_queueMicrotask_c5419c06eab41e73 = typeof queueMicrotask == 'function' ? queueMicrotask : notDefined('queueMicrotask');
imports.wbg.__wbg_get_5419cf6b954aa11d = function(arg0, arg1) {
    const ret = arg0[arg1 >>> 0];
    return ret;
};
imports.wbg.__wbg_length_f217bbbf7e8e4df4 = function(arg0) {
    const ret = arg0.length;
    return ret;
};
imports.wbg.__wbg_new_034f913e7636e987 = function() {
    const ret = new Array();
    return ret;
};
imports.wbg.__wbg_newnoargs_1ede4bf2ebbaaf43 = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Function(v0);
    return ret;
};
imports.wbg.__wbg_next_13b477da1eaa3897 = function(arg0) {
    const ret = arg0.next;
    return ret;
};
imports.wbg.__wbg_next_b06e115d1b01e10b = function() { return handleError(function (arg0) {
    const ret = arg0.next();
    return ret;
}, arguments) };
imports.wbg.__wbg_done_983b5ffcaec8c583 = function(arg0) {
    const ret = arg0.done;
    return ret;
};
imports.wbg.__wbg_value_2ab8a198c834c26a = function(arg0) {
    const ret = arg0.value;
    return ret;
};
imports.wbg.__wbg_iterator_695d699a44d6234c = function() {
    const ret = Symbol.iterator;
    return ret;
};
imports.wbg.__wbg_get_ef828680c64da212 = function() { return handleError(function (arg0, arg1) {
    const ret = Reflect.get(arg0, arg1);
    return ret;
}, arguments) };
imports.wbg.__wbg_call_a9ef466721e824f2 = function() { return handleError(function (arg0, arg1) {
    const ret = arg0.call(arg1);
    return ret;
}, arguments) };
imports.wbg.__wbg_new_e69b5f66fda8f13c = function() {
    const ret = new Object();
    return ret;
};
imports.wbg.__wbg_self_bf91bf94d9e04084 = function() { return handleError(function () {
    const ret = self.self;
    return ret;
}, arguments) };
imports.wbg.__wbg_window_52dd9f07d03fd5f8 = function() { return handleError(function () {
    const ret = window.window;
    return ret;
}, arguments) };
imports.wbg.__wbg_globalThis_05c129bf37fcf1be = function() { return handleError(function () {
    const ret = globalThis.globalThis;
    return ret;
}, arguments) };
imports.wbg.__wbg_global_3eca19bb09e9c484 = function() { return handleError(function () {
    const ret = global.global;
    return ret;
}, arguments) };
imports.wbg.__wbg_decodeURI_135fe2e8f0684ba4 = function() { return handleError(function (arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = decodeURI(v0);
    return ret;
}, arguments) };
imports.wbg.__wbg_decodeURIComponent_b5b4c94b85a4ec75 = function() { return handleError(function (arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = decodeURIComponent(v0);
    return ret;
}, arguments) };
imports.wbg.__wbg_isArray_6f3b47f09adb61b5 = function(arg0) {
    const ret = Array.isArray(arg0);
    return ret;
};
imports.wbg.__wbg_push_36cf4d81d7da33d1 = function(arg0, arg1) {
    const ret = arg0.push(arg1);
    return ret;
};
imports.wbg.__wbg_new_70a2f23d1565c04c = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    const ret = new Error(v0);
    return ret;
};
imports.wbg.__wbg_call_3bfa248576352471 = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.call(arg1, arg2);
    return ret;
}, arguments) };
imports.wbg.__wbg_next_3903305faa61ec71 = function() { return handleError(function (arg0) {
    const ret = arg0.next();
    return ret;
}, arguments) };
imports.wbg.__wbg_is_4b64bc96710d6a0f = function(arg0, arg1) {
    const ret = Object.is(arg0, arg1);
    return ret;
};
imports.wbg.__wbg_toString_aea130fe68d19e1a = function(arg0) {
    const ret = arg0.toString();
    return ret;
};
imports.wbg.__wbg_exec_c872ad5c15e456ad = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    const ret = arg0.exec(v0);
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};
imports.wbg.__wbg_new_800498ec872f75d4 = function(arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    var v1 = getCachedStringFromWasm0(arg2, arg3);
    const ret = new RegExp(v0, v1);
    return ret;
};
imports.wbg.__wbg_replace_378f99a02c0d0d21 = function(arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    const ret = arg0.replace(v0, v1);
    return ret;
};
imports.wbg.__wbg_new_1073970097e5a420 = function(arg0, arg1) {
    try {
        var state0 = {a: arg0, b: arg1};
        var cb0 = (arg0, arg1) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return __wbg_adapter_441(a, state0.b, arg0, arg1);
            } finally {
                state0.a = a;
            }
        };
        const ret = new Promise(cb0);
        return ret;
    } finally {
        state0.a = state0.b = 0;
    }
};
imports.wbg.__wbg_resolve_0aad7c1484731c99 = function(arg0) {
    const ret = Promise.resolve(arg0);
    return ret;
};
imports.wbg.__wbg_then_748f75edfb032440 = function(arg0, arg1) {
    const ret = arg0.then(arg1);
    return ret;
};
imports.wbg.__wbg_then_4866a7d9f55d8f3e = function(arg0, arg1, arg2) {
    const ret = arg0.then(arg1, arg2);
    return ret;
};
imports.wbg.__wbg_buffer_ccaed51a635d8a2d = function(arg0) {
    const ret = arg0.buffer;
    return ret;
};
imports.wbg.__wbg_newwithbyteoffsetandlength_7e3eb787208af730 = function(arg0, arg1, arg2) {
    const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
    return ret;
};
imports.wbg.__wbg_new_fec2611eb9180f95 = function(arg0) {
    const ret = new Uint8Array(arg0);
    return ret;
};
imports.wbg.__wbg_set_ec2fcf81bc573fd9 = function(arg0, arg1, arg2) {
    arg0.set(arg1, arg2 >>> 0);
};
imports.wbg.__wbg_length_9254c4bd3b9f23c4 = function(arg0) {
    const ret = arg0.length;
    return ret;
};
imports.wbg.__wbg_newwithlength_76462a666eca145f = function(arg0) {
    const ret = new Uint8Array(arg0 >>> 0);
    return ret;
};
imports.wbg.__wbg_buffer_95102df5554646dc = function(arg0) {
    const ret = arg0.buffer;
    return ret;
};
imports.wbg.__wbg_subarray_975a06f9dbd16995 = function(arg0, arg1, arg2) {
    const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
    return ret;
};
imports.wbg.__wbg_byteLength_5d623ba3d92a3a9c = function(arg0) {
    const ret = arg0.byteLength;
    return ret;
};
imports.wbg.__wbg_byteOffset_ec0928143c619cd7 = function(arg0) {
    const ret = arg0.byteOffset;
    return ret;
};
imports.wbg.__wbg_set_e864d25d9b399c9f = function() { return handleError(function (arg0, arg1, arg2) {
    const ret = Reflect.set(arg0, arg1, arg2);
    return ret;
}, arguments) };
imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};
imports.wbg.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};
imports.wbg.__wbindgen_rethrow = function(arg0) {
    throw arg0;
};
imports.wbg.__wbindgen_memory = function() {
    const ret = wasm.memory;
    return ret;
};
imports.wbg.__wbindgen_float64_array_new = function(arg0, arg1) {
    var v0 = getArrayF64FromWasm0(arg0, arg1).slice();
    wasm.__wbindgen_free(arg0, arg1 * 8, 8);
    const ret = v0;
    return ret;
};
imports.wbg.__wbindgen_closure_wrapper765 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 264, __wbg_adapter_42);
    return ret;
};
imports.wbg.__wbindgen_closure_wrapper766 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 264, __wbg_adapter_45);
    return ret;
};
imports.wbg.__wbindgen_closure_wrapper768 = function(arg0, arg1, arg2) {
    const ret = makeClosure(arg0, arg1, 264, __wbg_adapter_48);
    return ret;
};
imports.wbg.__wbindgen_closure_wrapper770 = function(arg0, arg1, arg2) {
    const ret = makeClosure(arg0, arg1, 264, __wbg_adapter_51);
    return ret;
};
imports.wbg.__wbindgen_closure_wrapper773 = function(arg0, arg1, arg2) {
    const ret = makeClosure(arg0, arg1, 264, __wbg_adapter_48);
    return ret;
};
imports.wbg.__wbindgen_closure_wrapper775 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 264, __wbg_adapter_45);
    return ret;
};
imports.wbg.__wbindgen_closure_wrapper1793 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 762, __wbg_adapter_58);
    return ret;
};
imports.wbg.__wbindgen_closure_wrapper2015 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 814, __wbg_adapter_61);
    return ret;
};
imports.wbg.__wbindgen_closure_wrapper2017 = function(arg0, arg1, arg2) {
    const ret = makeClosure(arg0, arg1, 814, __wbg_adapter_64);
    return ret;
};
imports.wbg.__wbindgen_closure_wrapper2163 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 850, __wbg_adapter_67);
    return ret;
};
imports.wbg.__wbindgen_closure_wrapper3842 = function(arg0, arg1, arg2) {
    const ret = makeMutClosure(arg0, arg1, 878, __wbg_adapter_70);
    return ret;
};
imports.wbg.__wbindgen_init_externref_table = function() {
    const table = wasm.__wbindgen_export_3;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};
imports['./snippets/web-67aca56aae793faf/build/index.js'] = __wbg_star0;
imports['./snippets/web-67aca56aae793faf/public/js/highlight.js'] = __wbg_star1;

return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedFloat64ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('app_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
