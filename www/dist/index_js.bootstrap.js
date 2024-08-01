"use strict";
(self["webpackChunkcreate_wasm_app"] = self["webpackChunkcreate_wasm_app"] || []).push([["index_js"],{

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {
__webpack_require__.r(__webpack_exports__);
/* harmony import */ var rockies__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! rockies */ "../pkg/rockies.js");
/* harmony import */ var rockies_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! rockies/rockies_bg.wasm */ "../pkg/rockies_bg.wasm");
var __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([rockies__WEBPACK_IMPORTED_MODULE_0__, rockies_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);
([rockies__WEBPACK_IMPORTED_MODULE_0__, rockies_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__] = __webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__);



const canvas = document.getElementById("the-canvas");

const SIZE = 64;
const CELL_SIZE = Math.min(canvas.clientWidth / SIZE, canvas.clientHeight / SIZE) | 0; // px


const game = rockies__WEBPACK_IMPORTED_MODULE_0__.Game.new(SIZE, SIZE);
const width = game.width();
const height = game.height();

const ticks = document.getElementById("ticks");
const cells_count = document.getElementById("cells-count");
const collisions_count = document.getElementById("collisions-count");
const collision_pairs_tested = document.getElementById("collision-pairs-tested");

canvas.height = (CELL_SIZE) * height + 1;
canvas.width = (CELL_SIZE) * width + 1;

const ctx = canvas.getContext('2d');


const heldKeys = new Set();


document.onkeydown = (e) => {
    game.key_down(e.key);
};
document.onkeyup = (e) => {
    game.key_up(e.key);
};


const renderLoop = () => {

    game.tick();

    drawPixels();

    let stats = game.stats();

    ticks.textContent = stats.ticks();
    cells_count.textContent = stats.cells_count();
    collisions_count.textContent = (stats.collisions_count() / stats.ticks()) | 0;
    collision_pairs_tested.textContent = (stats.collision_pairs_tested() / stats.ticks()) | 0;



    requestAnimationFrame(renderLoop);
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawPixels = () => {
    const pixelsPtr = game.pixels();
    const pixels = new Uint32Array(rockies_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__.memory.buffer, pixelsPtr, width * height);

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            ctx.beginPath();

            let val = pixels[idx];
            ctx.fillStyle = "#" + val.toString(16).padStart(6, "0");
            //console.log("[%d,%d] = %s = %s", row, col, pixels[idx].toString(16), ctx.fillStyle);

            ctx.fillRect(
                col * CELL_SIZE + 1,
                row * CELL_SIZE + 1,
                CELL_SIZE,
                CELL_SIZE
            );

            ctx.stroke();
        }
    }

};

drawPixels();
requestAnimationFrame(renderLoop);

canvas.onmousemove = (e) => {
    if (e.buttons > 0) {
        game.click(e.offsetX / (CELL_SIZE + 1), e.offsetY / (CELL_SIZE + 1));
    }
};

canvas.onclick = (e) => {
    game.click(e.offsetX / (CELL_SIZE + 1), e.offsetY / (CELL_SIZE + 1));
};

canvas.ontouchmove = (e) => {
    e.preventDefault();
    let x = e.touches[0].clientX - canvas.offsetLeft;
    let y = e.touches[0].clientY - canvas.offsetTop;
    game.click(x / (CELL_SIZE + 1), y / (CELL_SIZE + 1));
};

__webpack_async_result__();
} catch(e) { __webpack_async_result__(e); } });

/***/ }),

/***/ "../pkg/rockies.js":
/*!*************************!*\
  !*** ../pkg/rockies.js ***!
  \*************************/
/***/ ((__webpack_module__, __webpack_exports__, __webpack_require__) => {

__webpack_require__.a(__webpack_module__, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {
__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   Game: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.Game),
/* harmony export */   Stats: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.Stats),
/* harmony export */   __wbg_error_f851667af71bcfc6: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_error_f851667af71bcfc6),
/* harmony export */   __wbg_new_abda76e883ba8a5f: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_abda76e883ba8a5f),
/* harmony export */   __wbg_set_wasm: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm),
/* harmony export */   __wbg_stack_658279fe44541cf6: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_stack_658279fe44541cf6),
/* harmony export */   __wbindgen_object_drop_ref: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_object_drop_ref),
/* harmony export */   __wbindgen_throw: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_throw)
/* harmony export */ });
/* harmony import */ var _rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./rockies_bg.wasm */ "../pkg/rockies_bg.wasm");
/* harmony import */ var _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./rockies_bg.js */ "../pkg/rockies_bg.js");
var __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);
_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];


(0,_rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm)(_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__);


__webpack_async_result__();
} catch(e) { __webpack_async_result__(e); } });

/***/ }),

/***/ "../pkg/rockies_bg.js":
/*!****************************!*\
  !*** ../pkg/rockies_bg.js ***!
  \****************************/
/***/ ((__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   Game: () => (/* binding */ Game),
/* harmony export */   Stats: () => (/* binding */ Stats),
/* harmony export */   __wbg_error_f851667af71bcfc6: () => (/* binding */ __wbg_error_f851667af71bcfc6),
/* harmony export */   __wbg_new_abda76e883ba8a5f: () => (/* binding */ __wbg_new_abda76e883ba8a5f),
/* harmony export */   __wbg_set_wasm: () => (/* binding */ __wbg_set_wasm),
/* harmony export */   __wbg_stack_658279fe44541cf6: () => (/* binding */ __wbg_stack_658279fe44541cf6),
/* harmony export */   __wbindgen_object_drop_ref: () => (/* binding */ __wbindgen_object_drop_ref),
/* harmony export */   __wbindgen_throw: () => (/* binding */ __wbindgen_throw)
/* harmony export */ });
let wasm;
function __wbg_set_wasm(val) {
    wasm = val;
}


const heap = new Array(128).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 132) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0 = null;

function getUint8Memory0() {
    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

let cachedInt32Memory0 = null;

function getInt32Memory0() {
    if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function _assertChar(c) {
    if (typeof(c) === 'number' && (c >= 0x110000 || (c >= 0xD800 && c < 0xE000))) throw new Error(`expected a valid Unicode scalar value, found ${c}`);
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

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
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8Memory0();

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
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

const GameFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_game_free(ptr >>> 0));
/**
*/
class Game {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Game.prototype);
        obj.__wbg_ptr = ptr;
        GameFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GameFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_game_free(ptr);
    }
    /**
    * @param {number} width
    * @param {number} height
    * @returns {Game}
    */
    static new(width, height) {
        const ret = wasm.game_new(width, height);
        return Game.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    pixels() {
        const ret = wasm.game_pixels(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    */
    tick() {
        wasm.game_tick(this.__wbg_ptr);
    }
    /**
    */
    render() {
        wasm.game_render(this.__wbg_ptr);
    }
    /**
    * @returns {string}
    */
    text_render() {
        let deferred1_0;
        let deferred1_1;
        try {
            const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
            wasm.game_text_render(retptr, this.__wbg_ptr);
            var r0 = getInt32Memory0()[retptr / 4 + 0];
            var r1 = getInt32Memory0()[retptr / 4 + 1];
            deferred1_0 = r0;
            deferred1_1 = r1;
            return getStringFromWasm0(r0, r1);
        } finally {
            wasm.__wbindgen_add_to_stack_pointer(16);
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
    * @param {string} key
    */
    key_down(key) {
        const char0 = key.codePointAt(0);
        _assertChar(char0);
        wasm.game_key_down(this.__wbg_ptr, char0);
    }
    /**
    * @param {string} key
    */
    key_up(key) {
        const char0 = key.codePointAt(0);
        _assertChar(char0);
        wasm.game_key_up(this.__wbg_ptr, char0);
    }
    /**
    */
    process_keys() {
        wasm.game_process_keys(this.__wbg_ptr);
    }
    /**
    * @param {number} x
    * @param {number} y
    */
    click(x, y) {
        wasm.game_click(this.__wbg_ptr, x, y);
    }
    /**
    * @returns {number}
    */
    width() {
        const ret = wasm.game_width(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    height() {
        const ret = wasm.game_height(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {Stats}
    */
    stats() {
        const ret = wasm.game_stats(this.__wbg_ptr);
        return Stats.__wrap(ret);
    }
}

const StatsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_stats_free(ptr >>> 0));
/**
*/
class Stats {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Stats.prototype);
        obj.__wbg_ptr = ptr;
        StatsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StatsFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_stats_free(ptr);
    }
    /**
    * @returns {Stats}
    */
    get_and_reset() {
        const ret = wasm.stats_get_and_reset(this.__wbg_ptr);
        return Stats.__wrap(ret);
    }
    /**
    * @returns {Stats}
    */
    static zero() {
        const ret = wasm.stats_zero();
        return Stats.__wrap(ret);
    }
    /**
    * @returns {number}
    */
    ticks() {
        const ret = wasm.stats_ticks(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    cells_count() {
        const ret = wasm.stats_cells_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    collisions_count() {
        const ret = wasm.stats_collisions_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    collision_pairs_tested() {
        const ret = wasm.stats_collision_pairs_tested(this.__wbg_ptr);
        return ret >>> 0;
    }
}

function __wbg_new_abda76e883ba8a5f() {
    const ret = new Error();
    return addHeapObject(ret);
};

function __wbg_stack_658279fe44541cf6(arg0, arg1) {
    const ret = getObject(arg1).stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};

function __wbg_error_f851667af71bcfc6(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};



/***/ }),

/***/ "../pkg/rockies_bg.wasm":
/*!******************************!*\
  !*** ../pkg/rockies_bg.wasm ***!
  \******************************/
/***/ ((module, exports, __webpack_require__) => {

/* harmony import */ var WEBPACK_IMPORTED_MODULE_0 = __webpack_require__(/*! ./rockies_bg.js */ "../pkg/rockies_bg.js");
module.exports = __webpack_require__.v(exports, module.id, "62d3be03491564699edd", {
	"./rockies_bg.js": {
		"__wbg_new_abda76e883ba8a5f": WEBPACK_IMPORTED_MODULE_0.__wbg_new_abda76e883ba8a5f,
		"__wbg_stack_658279fe44541cf6": WEBPACK_IMPORTED_MODULE_0.__wbg_stack_658279fe44541cf6,
		"__wbg_error_f851667af71bcfc6": WEBPACK_IMPORTED_MODULE_0.__wbg_error_f851667af71bcfc6,
		"__wbindgen_object_drop_ref": WEBPACK_IMPORTED_MODULE_0.__wbindgen_object_drop_ref,
		"__wbindgen_throw": WEBPACK_IMPORTED_MODULE_0.__wbindgen_throw
	}
});

/***/ })

}]);
//# sourceMappingURL=data:application/json;charset=utf-8;base64,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QUNBO0FBQ0EsZ0JBQWdCO0FBQ2hCO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBLGdCQUFnQjtBQUNoQjtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7O0FBRUE7QUFDQSxRQUFRLGtCQUFrQjtBQUMxQjtBQUNBO0FBQ0E7QUFDTzs7QUFFUDtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTs7QUFFQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7O0FBRUE7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBLGdCQUFnQjtBQUNoQjtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQSxnQkFBZ0I7QUFDaEI7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0EsZ0JBQWdCO0FBQ2hCO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBLGdCQUFnQjtBQUNoQjtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQSxnQkFBZ0I7QUFDaEI7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0EsZ0JBQWdCO0FBQ2hCO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTs7QUFFTztBQUNQO0FBQ0E7QUFDQTs7QUFFTztBQUNQO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTs7QUFFTztBQUNQO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBLE1BQU07QUFDTjtBQUNBO0FBQ0E7O0FBRU87QUFDUDtBQUNBOztBQUVPO0FBQ1A7QUFDQSIsInNvdXJjZXMiOlsid2VicGFjazovL2NyZWF0ZS13YXNtLWFwcC8uL2luZGV4LmpzIiwid2VicGFjazovL2NyZWF0ZS13YXNtLWFwcC8uLi9wa2cvcm9ja2llcy5qcyIsIndlYnBhY2s6Ly9jcmVhdGUtd2FzbS1hcHAvLi4vcGtnL3JvY2tpZXNfYmcuanMiXSwic291cmNlc0NvbnRlbnQiOlsiaW1wb3J0IHsgR2FtZSwgQ2VsbCB9IGZyb20gXCJyb2NraWVzXCI7XG5pbXBvcnQgeyBtZW1vcnkgfSBmcm9tIFwicm9ja2llcy9yb2NraWVzX2JnLndhc21cIjtcblxuY29uc3QgY2FudmFzID0gZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoXCJ0aGUtY2FudmFzXCIpO1xuXG5jb25zdCBTSVpFID0gNjQ7XG5jb25zdCBDRUxMX1NJWkUgPSBNYXRoLm1pbihjYW52YXMuY2xpZW50V2lkdGggLyBTSVpFLCBjYW52YXMuY2xpZW50SGVpZ2h0IC8gU0laRSkgfCAwOyAvLyBweFxuXG5cbmNvbnN0IGdhbWUgPSBHYW1lLm5ldyhTSVpFLCBTSVpFKTtcbmNvbnN0IHdpZHRoID0gZ2FtZS53aWR0aCgpO1xuY29uc3QgaGVpZ2h0ID0gZ2FtZS5oZWlnaHQoKTtcblxuY29uc3QgdGlja3MgPSBkb2N1bWVudC5nZXRFbGVtZW50QnlJZChcInRpY2tzXCIpO1xuY29uc3QgY2VsbHNfY291bnQgPSBkb2N1bWVudC5nZXRFbGVtZW50QnlJZChcImNlbGxzLWNvdW50XCIpO1xuY29uc3QgY29sbGlzaW9uc19jb3VudCA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKFwiY29sbGlzaW9ucy1jb3VudFwiKTtcbmNvbnN0IGNvbGxpc2lvbl9wYWlyc190ZXN0ZWQgPSBkb2N1bWVudC5nZXRFbGVtZW50QnlJZChcImNvbGxpc2lvbi1wYWlycy10ZXN0ZWRcIik7XG5cbmNhbnZhcy5oZWlnaHQgPSAoQ0VMTF9TSVpFKSAqIGhlaWdodCArIDE7XG5jYW52YXMud2lkdGggPSAoQ0VMTF9TSVpFKSAqIHdpZHRoICsgMTtcblxuY29uc3QgY3R4ID0gY2FudmFzLmdldENvbnRleHQoJzJkJyk7XG5cblxuY29uc3QgaGVsZEtleXMgPSBuZXcgU2V0KCk7XG5cblxuZG9jdW1lbnQub25rZXlkb3duID0gKGUpID0+IHtcbiAgICBnYW1lLmtleV9kb3duKGUua2V5KTtcbn07XG5kb2N1bWVudC5vbmtleXVwID0gKGUpID0+IHtcbiAgICBnYW1lLmtleV91cChlLmtleSk7XG59O1xuXG5cbmNvbnN0IHJlbmRlckxvb3AgPSAoKSA9PiB7XG5cbiAgICBnYW1lLnRpY2soKTtcblxuICAgIGRyYXdQaXhlbHMoKTtcblxuICAgIGxldCBzdGF0cyA9IGdhbWUuc3RhdHMoKTtcblxuICAgIHRpY2tzLnRleHRDb250ZW50ID0gc3RhdHMudGlja3MoKTtcbiAgICBjZWxsc19jb3VudC50ZXh0Q29udGVudCA9IHN0YXRzLmNlbGxzX2NvdW50KCk7XG4gICAgY29sbGlzaW9uc19jb3VudC50ZXh0Q29udGVudCA9IChzdGF0cy5jb2xsaXNpb25zX2NvdW50KCkgLyBzdGF0cy50aWNrcygpKSB8IDA7XG4gICAgY29sbGlzaW9uX3BhaXJzX3Rlc3RlZC50ZXh0Q29udGVudCA9IChzdGF0cy5jb2xsaXNpb25fcGFpcnNfdGVzdGVkKCkgLyBzdGF0cy50aWNrcygpKSB8IDA7XG5cblxuXG4gICAgcmVxdWVzdEFuaW1hdGlvbkZyYW1lKHJlbmRlckxvb3ApO1xufTtcblxuY29uc3QgZ2V0SW5kZXggPSAocm93LCBjb2x1bW4pID0+IHtcbiAgICByZXR1cm4gcm93ICogd2lkdGggKyBjb2x1bW47XG59O1xuXG5jb25zdCBkcmF3UGl4ZWxzID0gKCkgPT4ge1xuICAgIGNvbnN0IHBpeGVsc1B0ciA9IGdhbWUucGl4ZWxzKCk7XG4gICAgY29uc3QgcGl4ZWxzID0gbmV3IFVpbnQzMkFycmF5KG1lbW9yeS5idWZmZXIsIHBpeGVsc1B0ciwgd2lkdGggKiBoZWlnaHQpO1xuXG4gICAgZm9yIChsZXQgcm93ID0gMDsgcm93IDwgaGVpZ2h0OyByb3crKykge1xuICAgICAgICBmb3IgKGxldCBjb2wgPSAwOyBjb2wgPCB3aWR0aDsgY29sKyspIHtcbiAgICAgICAgICAgIGNvbnN0IGlkeCA9IGdldEluZGV4KHJvdywgY29sKTtcbiAgICAgICAgICAgIGN0eC5iZWdpblBhdGgoKTtcblxuICAgICAgICAgICAgbGV0IHZhbCA9IHBpeGVsc1tpZHhdO1xuICAgICAgICAgICAgY3R4LmZpbGxTdHlsZSA9IFwiI1wiICsgdmFsLnRvU3RyaW5nKDE2KS5wYWRTdGFydCg2LCBcIjBcIik7XG4gICAgICAgICAgICAvL2NvbnNvbGUubG9nKFwiWyVkLCVkXSA9ICVzID0gJXNcIiwgcm93LCBjb2wsIHBpeGVsc1tpZHhdLnRvU3RyaW5nKDE2KSwgY3R4LmZpbGxTdHlsZSk7XG5cbiAgICAgICAgICAgIGN0eC5maWxsUmVjdChcbiAgICAgICAgICAgICAgICBjb2wgKiBDRUxMX1NJWkUgKyAxLFxuICAgICAgICAgICAgICAgIHJvdyAqIENFTExfU0laRSArIDEsXG4gICAgICAgICAgICAgICAgQ0VMTF9TSVpFLFxuICAgICAgICAgICAgICAgIENFTExfU0laRVxuICAgICAgICAgICAgKTtcblxuICAgICAgICAgICAgY3R4LnN0cm9rZSgpO1xuICAgICAgICB9XG4gICAgfVxuXG59O1xuXG5kcmF3UGl4ZWxzKCk7XG5yZXF1ZXN0QW5pbWF0aW9uRnJhbWUocmVuZGVyTG9vcCk7XG5cbmNhbnZhcy5vbm1vdXNlbW92ZSA9IChlKSA9PiB7XG4gICAgaWYgKGUuYnV0dG9ucyA+IDApIHtcbiAgICAgICAgZ2FtZS5jbGljayhlLm9mZnNldFggLyAoQ0VMTF9TSVpFICsgMSksIGUub2Zmc2V0WSAvIChDRUxMX1NJWkUgKyAxKSk7XG4gICAgfVxufTtcblxuY2FudmFzLm9uY2xpY2sgPSAoZSkgPT4ge1xuICAgIGdhbWUuY2xpY2soZS5vZmZzZXRYIC8gKENFTExfU0laRSArIDEpLCBlLm9mZnNldFkgLyAoQ0VMTF9TSVpFICsgMSkpO1xufTtcblxuY2FudmFzLm9udG91Y2htb3ZlID0gKGUpID0+IHtcbiAgICBlLnByZXZlbnREZWZhdWx0KCk7XG4gICAgbGV0IHggPSBlLnRvdWNoZXNbMF0uY2xpZW50WCAtIGNhbnZhcy5vZmZzZXRMZWZ0O1xuICAgIGxldCB5ID0gZS50b3VjaGVzWzBdLmNsaWVudFkgLSBjYW52YXMub2Zmc2V0VG9wO1xuICAgIGdhbWUuY2xpY2soeCAvIChDRUxMX1NJWkUgKyAxKSwgeSAvIChDRUxMX1NJWkUgKyAxKSk7XG59O1xuIiwiaW1wb3J0ICogYXMgd2FzbSBmcm9tIFwiLi9yb2NraWVzX2JnLndhc21cIjtcbmltcG9ydCB7IF9fd2JnX3NldF93YXNtIH0gZnJvbSBcIi4vcm9ja2llc19iZy5qc1wiO1xuX193Ymdfc2V0X3dhc20od2FzbSk7XG5leHBvcnQgKiBmcm9tIFwiLi9yb2NraWVzX2JnLmpzXCI7XG4iLCJsZXQgd2FzbTtcbmV4cG9ydCBmdW5jdGlvbiBfX3diZ19zZXRfd2FzbSh2YWwpIHtcbiAgICB3YXNtID0gdmFsO1xufVxuXG5cbmNvbnN0IGhlYXAgPSBuZXcgQXJyYXkoMTI4KS5maWxsKHVuZGVmaW5lZCk7XG5cbmhlYXAucHVzaCh1bmRlZmluZWQsIG51bGwsIHRydWUsIGZhbHNlKTtcblxuZnVuY3Rpb24gZ2V0T2JqZWN0KGlkeCkgeyByZXR1cm4gaGVhcFtpZHhdOyB9XG5cbmxldCBoZWFwX25leHQgPSBoZWFwLmxlbmd0aDtcblxuZnVuY3Rpb24gZHJvcE9iamVjdChpZHgpIHtcbiAgICBpZiAoaWR4IDwgMTMyKSByZXR1cm47XG4gICAgaGVhcFtpZHhdID0gaGVhcF9uZXh0O1xuICAgIGhlYXBfbmV4dCA9IGlkeDtcbn1cblxuZnVuY3Rpb24gdGFrZU9iamVjdChpZHgpIHtcbiAgICBjb25zdCByZXQgPSBnZXRPYmplY3QoaWR4KTtcbiAgICBkcm9wT2JqZWN0KGlkeCk7XG4gICAgcmV0dXJuIHJldDtcbn1cblxuY29uc3QgbFRleHREZWNvZGVyID0gdHlwZW9mIFRleHREZWNvZGVyID09PSAndW5kZWZpbmVkJyA/ICgwLCBtb2R1bGUucmVxdWlyZSkoJ3V0aWwnKS5UZXh0RGVjb2RlciA6IFRleHREZWNvZGVyO1xuXG5sZXQgY2FjaGVkVGV4dERlY29kZXIgPSBuZXcgbFRleHREZWNvZGVyKCd1dGYtOCcsIHsgaWdub3JlQk9NOiB0cnVlLCBmYXRhbDogdHJ1ZSB9KTtcblxuY2FjaGVkVGV4dERlY29kZXIuZGVjb2RlKCk7XG5cbmxldCBjYWNoZWRVaW50OE1lbW9yeTAgPSBudWxsO1xuXG5mdW5jdGlvbiBnZXRVaW50OE1lbW9yeTAoKSB7XG4gICAgaWYgKGNhY2hlZFVpbnQ4TWVtb3J5MCA9PT0gbnVsbCB8fCBjYWNoZWRVaW50OE1lbW9yeTAuYnl0ZUxlbmd0aCA9PT0gMCkge1xuICAgICAgICBjYWNoZWRVaW50OE1lbW9yeTAgPSBuZXcgVWludDhBcnJheSh3YXNtLm1lbW9yeS5idWZmZXIpO1xuICAgIH1cbiAgICByZXR1cm4gY2FjaGVkVWludDhNZW1vcnkwO1xufVxuXG5mdW5jdGlvbiBnZXRTdHJpbmdGcm9tV2FzbTAocHRyLCBsZW4pIHtcbiAgICBwdHIgPSBwdHIgPj4+IDA7XG4gICAgcmV0dXJuIGNhY2hlZFRleHREZWNvZGVyLmRlY29kZShnZXRVaW50OE1lbW9yeTAoKS5zdWJhcnJheShwdHIsIHB0ciArIGxlbikpO1xufVxuXG5sZXQgY2FjaGVkSW50MzJNZW1vcnkwID0gbnVsbDtcblxuZnVuY3Rpb24gZ2V0SW50MzJNZW1vcnkwKCkge1xuICAgIGlmIChjYWNoZWRJbnQzMk1lbW9yeTAgPT09IG51bGwgfHwgY2FjaGVkSW50MzJNZW1vcnkwLmJ5dGVMZW5ndGggPT09IDApIHtcbiAgICAgICAgY2FjaGVkSW50MzJNZW1vcnkwID0gbmV3IEludDMyQXJyYXkod2FzbS5tZW1vcnkuYnVmZmVyKTtcbiAgICB9XG4gICAgcmV0dXJuIGNhY2hlZEludDMyTWVtb3J5MDtcbn1cblxuZnVuY3Rpb24gX2Fzc2VydENoYXIoYykge1xuICAgIGlmICh0eXBlb2YoYykgPT09ICdudW1iZXInICYmIChjID49IDB4MTEwMDAwIHx8IChjID49IDB4RDgwMCAmJiBjIDwgMHhFMDAwKSkpIHRocm93IG5ldyBFcnJvcihgZXhwZWN0ZWQgYSB2YWxpZCBVbmljb2RlIHNjYWxhciB2YWx1ZSwgZm91bmQgJHtjfWApO1xufVxuXG5mdW5jdGlvbiBhZGRIZWFwT2JqZWN0KG9iaikge1xuICAgIGlmIChoZWFwX25leHQgPT09IGhlYXAubGVuZ3RoKSBoZWFwLnB1c2goaGVhcC5sZW5ndGggKyAxKTtcbiAgICBjb25zdCBpZHggPSBoZWFwX25leHQ7XG4gICAgaGVhcF9uZXh0ID0gaGVhcFtpZHhdO1xuXG4gICAgaGVhcFtpZHhdID0gb2JqO1xuICAgIHJldHVybiBpZHg7XG59XG5cbmxldCBXQVNNX1ZFQ1RPUl9MRU4gPSAwO1xuXG5jb25zdCBsVGV4dEVuY29kZXIgPSB0eXBlb2YgVGV4dEVuY29kZXIgPT09ICd1bmRlZmluZWQnID8gKDAsIG1vZHVsZS5yZXF1aXJlKSgndXRpbCcpLlRleHRFbmNvZGVyIDogVGV4dEVuY29kZXI7XG5cbmxldCBjYWNoZWRUZXh0RW5jb2RlciA9IG5ldyBsVGV4dEVuY29kZXIoJ3V0Zi04Jyk7XG5cbmNvbnN0IGVuY29kZVN0cmluZyA9ICh0eXBlb2YgY2FjaGVkVGV4dEVuY29kZXIuZW5jb2RlSW50byA9PT0gJ2Z1bmN0aW9uJ1xuICAgID8gZnVuY3Rpb24gKGFyZywgdmlldykge1xuICAgIHJldHVybiBjYWNoZWRUZXh0RW5jb2Rlci5lbmNvZGVJbnRvKGFyZywgdmlldyk7XG59XG4gICAgOiBmdW5jdGlvbiAoYXJnLCB2aWV3KSB7XG4gICAgY29uc3QgYnVmID0gY2FjaGVkVGV4dEVuY29kZXIuZW5jb2RlKGFyZyk7XG4gICAgdmlldy5zZXQoYnVmKTtcbiAgICByZXR1cm4ge1xuICAgICAgICByZWFkOiBhcmcubGVuZ3RoLFxuICAgICAgICB3cml0dGVuOiBidWYubGVuZ3RoXG4gICAgfTtcbn0pO1xuXG5mdW5jdGlvbiBwYXNzU3RyaW5nVG9XYXNtMChhcmcsIG1hbGxvYywgcmVhbGxvYykge1xuXG4gICAgaWYgKHJlYWxsb2MgPT09IHVuZGVmaW5lZCkge1xuICAgICAgICBjb25zdCBidWYgPSBjYWNoZWRUZXh0RW5jb2Rlci5lbmNvZGUoYXJnKTtcbiAgICAgICAgY29uc3QgcHRyID0gbWFsbG9jKGJ1Zi5sZW5ndGgsIDEpID4+PiAwO1xuICAgICAgICBnZXRVaW50OE1lbW9yeTAoKS5zdWJhcnJheShwdHIsIHB0ciArIGJ1Zi5sZW5ndGgpLnNldChidWYpO1xuICAgICAgICBXQVNNX1ZFQ1RPUl9MRU4gPSBidWYubGVuZ3RoO1xuICAgICAgICByZXR1cm4gcHRyO1xuICAgIH1cblxuICAgIGxldCBsZW4gPSBhcmcubGVuZ3RoO1xuICAgIGxldCBwdHIgPSBtYWxsb2MobGVuLCAxKSA+Pj4gMDtcblxuICAgIGNvbnN0IG1lbSA9IGdldFVpbnQ4TWVtb3J5MCgpO1xuXG4gICAgbGV0IG9mZnNldCA9IDA7XG5cbiAgICBmb3IgKDsgb2Zmc2V0IDwgbGVuOyBvZmZzZXQrKykge1xuICAgICAgICBjb25zdCBjb2RlID0gYXJnLmNoYXJDb2RlQXQob2Zmc2V0KTtcbiAgICAgICAgaWYgKGNvZGUgPiAweDdGKSBicmVhaztcbiAgICAgICAgbWVtW3B0ciArIG9mZnNldF0gPSBjb2RlO1xuICAgIH1cblxuICAgIGlmIChvZmZzZXQgIT09IGxlbikge1xuICAgICAgICBpZiAob2Zmc2V0ICE9PSAwKSB7XG4gICAgICAgICAgICBhcmcgPSBhcmcuc2xpY2Uob2Zmc2V0KTtcbiAgICAgICAgfVxuICAgICAgICBwdHIgPSByZWFsbG9jKHB0ciwgbGVuLCBsZW4gPSBvZmZzZXQgKyBhcmcubGVuZ3RoICogMywgMSkgPj4+IDA7XG4gICAgICAgIGNvbnN0IHZpZXcgPSBnZXRVaW50OE1lbW9yeTAoKS5zdWJhcnJheShwdHIgKyBvZmZzZXQsIHB0ciArIGxlbik7XG4gICAgICAgIGNvbnN0IHJldCA9IGVuY29kZVN0cmluZyhhcmcsIHZpZXcpO1xuXG4gICAgICAgIG9mZnNldCArPSByZXQud3JpdHRlbjtcbiAgICAgICAgcHRyID0gcmVhbGxvYyhwdHIsIGxlbiwgb2Zmc2V0LCAxKSA+Pj4gMDtcbiAgICB9XG5cbiAgICBXQVNNX1ZFQ1RPUl9MRU4gPSBvZmZzZXQ7XG4gICAgcmV0dXJuIHB0cjtcbn1cblxuY29uc3QgR2FtZUZpbmFsaXphdGlvbiA9ICh0eXBlb2YgRmluYWxpemF0aW9uUmVnaXN0cnkgPT09ICd1bmRlZmluZWQnKVxuICAgID8geyByZWdpc3RlcjogKCkgPT4ge30sIHVucmVnaXN0ZXI6ICgpID0+IHt9IH1cbiAgICA6IG5ldyBGaW5hbGl6YXRpb25SZWdpc3RyeShwdHIgPT4gd2FzbS5fX3diZ19nYW1lX2ZyZWUocHRyID4+PiAwKSk7XG4vKipcbiovXG5leHBvcnQgY2xhc3MgR2FtZSB7XG5cbiAgICBzdGF0aWMgX193cmFwKHB0cikge1xuICAgICAgICBwdHIgPSBwdHIgPj4+IDA7XG4gICAgICAgIGNvbnN0IG9iaiA9IE9iamVjdC5jcmVhdGUoR2FtZS5wcm90b3R5cGUpO1xuICAgICAgICBvYmouX193YmdfcHRyID0gcHRyO1xuICAgICAgICBHYW1lRmluYWxpemF0aW9uLnJlZ2lzdGVyKG9iaiwgb2JqLl9fd2JnX3B0ciwgb2JqKTtcbiAgICAgICAgcmV0dXJuIG9iajtcbiAgICB9XG5cbiAgICBfX2Rlc3Ryb3lfaW50b19yYXcoKSB7XG4gICAgICAgIGNvbnN0IHB0ciA9IHRoaXMuX193YmdfcHRyO1xuICAgICAgICB0aGlzLl9fd2JnX3B0ciA9IDA7XG4gICAgICAgIEdhbWVGaW5hbGl6YXRpb24udW5yZWdpc3Rlcih0aGlzKTtcbiAgICAgICAgcmV0dXJuIHB0cjtcbiAgICB9XG5cbiAgICBmcmVlKCkge1xuICAgICAgICBjb25zdCBwdHIgPSB0aGlzLl9fZGVzdHJveV9pbnRvX3JhdygpO1xuICAgICAgICB3YXNtLl9fd2JnX2dhbWVfZnJlZShwdHIpO1xuICAgIH1cbiAgICAvKipcbiAgICAqIEBwYXJhbSB7bnVtYmVyfSB3aWR0aFxuICAgICogQHBhcmFtIHtudW1iZXJ9IGhlaWdodFxuICAgICogQHJldHVybnMge0dhbWV9XG4gICAgKi9cbiAgICBzdGF0aWMgbmV3KHdpZHRoLCBoZWlnaHQpIHtcbiAgICAgICAgY29uc3QgcmV0ID0gd2FzbS5nYW1lX25ldyh3aWR0aCwgaGVpZ2h0KTtcbiAgICAgICAgcmV0dXJuIEdhbWUuX193cmFwKHJldCk7XG4gICAgfVxuICAgIC8qKlxuICAgICogQHJldHVybnMge251bWJlcn1cbiAgICAqL1xuICAgIHBpeGVscygpIHtcbiAgICAgICAgY29uc3QgcmV0ID0gd2FzbS5nYW1lX3BpeGVscyh0aGlzLl9fd2JnX3B0cik7XG4gICAgICAgIHJldHVybiByZXQgPj4+IDA7XG4gICAgfVxuICAgIC8qKlxuICAgICovXG4gICAgdGljaygpIHtcbiAgICAgICAgd2FzbS5nYW1lX3RpY2sodGhpcy5fX3diZ19wdHIpO1xuICAgIH1cbiAgICAvKipcbiAgICAqL1xuICAgIHJlbmRlcigpIHtcbiAgICAgICAgd2FzbS5nYW1lX3JlbmRlcih0aGlzLl9fd2JnX3B0cik7XG4gICAgfVxuICAgIC8qKlxuICAgICogQHJldHVybnMge3N0cmluZ31cbiAgICAqL1xuICAgIHRleHRfcmVuZGVyKCkge1xuICAgICAgICBsZXQgZGVmZXJyZWQxXzA7XG4gICAgICAgIGxldCBkZWZlcnJlZDFfMTtcbiAgICAgICAgdHJ5IHtcbiAgICAgICAgICAgIGNvbnN0IHJldHB0ciA9IHdhc20uX193YmluZGdlbl9hZGRfdG9fc3RhY2tfcG9pbnRlcigtMTYpO1xuICAgICAgICAgICAgd2FzbS5nYW1lX3RleHRfcmVuZGVyKHJldHB0ciwgdGhpcy5fX3diZ19wdHIpO1xuICAgICAgICAgICAgdmFyIHIwID0gZ2V0SW50MzJNZW1vcnkwKClbcmV0cHRyIC8gNCArIDBdO1xuICAgICAgICAgICAgdmFyIHIxID0gZ2V0SW50MzJNZW1vcnkwKClbcmV0cHRyIC8gNCArIDFdO1xuICAgICAgICAgICAgZGVmZXJyZWQxXzAgPSByMDtcbiAgICAgICAgICAgIGRlZmVycmVkMV8xID0gcjE7XG4gICAgICAgICAgICByZXR1cm4gZ2V0U3RyaW5nRnJvbVdhc20wKHIwLCByMSk7XG4gICAgICAgIH0gZmluYWxseSB7XG4gICAgICAgICAgICB3YXNtLl9fd2JpbmRnZW5fYWRkX3RvX3N0YWNrX3BvaW50ZXIoMTYpO1xuICAgICAgICAgICAgd2FzbS5fX3diaW5kZ2VuX2ZyZWUoZGVmZXJyZWQxXzAsIGRlZmVycmVkMV8xLCAxKTtcbiAgICAgICAgfVxuICAgIH1cbiAgICAvKipcbiAgICAqIEBwYXJhbSB7c3RyaW5nfSBrZXlcbiAgICAqL1xuICAgIGtleV9kb3duKGtleSkge1xuICAgICAgICBjb25zdCBjaGFyMCA9IGtleS5jb2RlUG9pbnRBdCgwKTtcbiAgICAgICAgX2Fzc2VydENoYXIoY2hhcjApO1xuICAgICAgICB3YXNtLmdhbWVfa2V5X2Rvd24odGhpcy5fX3diZ19wdHIsIGNoYXIwKTtcbiAgICB9XG4gICAgLyoqXG4gICAgKiBAcGFyYW0ge3N0cmluZ30ga2V5XG4gICAgKi9cbiAgICBrZXlfdXAoa2V5KSB7XG4gICAgICAgIGNvbnN0IGNoYXIwID0ga2V5LmNvZGVQb2ludEF0KDApO1xuICAgICAgICBfYXNzZXJ0Q2hhcihjaGFyMCk7XG4gICAgICAgIHdhc20uZ2FtZV9rZXlfdXAodGhpcy5fX3diZ19wdHIsIGNoYXIwKTtcbiAgICB9XG4gICAgLyoqXG4gICAgKi9cbiAgICBwcm9jZXNzX2tleXMoKSB7XG4gICAgICAgIHdhc20uZ2FtZV9wcm9jZXNzX2tleXModGhpcy5fX3diZ19wdHIpO1xuICAgIH1cbiAgICAvKipcbiAgICAqIEBwYXJhbSB7bnVtYmVyfSB4XG4gICAgKiBAcGFyYW0ge251bWJlcn0geVxuICAgICovXG4gICAgY2xpY2soeCwgeSkge1xuICAgICAgICB3YXNtLmdhbWVfY2xpY2sodGhpcy5fX3diZ19wdHIsIHgsIHkpO1xuICAgIH1cbiAgICAvKipcbiAgICAqIEByZXR1cm5zIHtudW1iZXJ9XG4gICAgKi9cbiAgICB3aWR0aCgpIHtcbiAgICAgICAgY29uc3QgcmV0ID0gd2FzbS5nYW1lX3dpZHRoKHRoaXMuX193YmdfcHRyKTtcbiAgICAgICAgcmV0dXJuIHJldCA+Pj4gMDtcbiAgICB9XG4gICAgLyoqXG4gICAgKiBAcmV0dXJucyB7bnVtYmVyfVxuICAgICovXG4gICAgaGVpZ2h0KCkge1xuICAgICAgICBjb25zdCByZXQgPSB3YXNtLmdhbWVfaGVpZ2h0KHRoaXMuX193YmdfcHRyKTtcbiAgICAgICAgcmV0dXJuIHJldCA+Pj4gMDtcbiAgICB9XG4gICAgLyoqXG4gICAgKiBAcmV0dXJucyB7U3RhdHN9XG4gICAgKi9cbiAgICBzdGF0cygpIHtcbiAgICAgICAgY29uc3QgcmV0ID0gd2FzbS5nYW1lX3N0YXRzKHRoaXMuX193YmdfcHRyKTtcbiAgICAgICAgcmV0dXJuIFN0YXRzLl9fd3JhcChyZXQpO1xuICAgIH1cbn1cblxuY29uc3QgU3RhdHNGaW5hbGl6YXRpb24gPSAodHlwZW9mIEZpbmFsaXphdGlvblJlZ2lzdHJ5ID09PSAndW5kZWZpbmVkJylcbiAgICA/IHsgcmVnaXN0ZXI6ICgpID0+IHt9LCB1bnJlZ2lzdGVyOiAoKSA9PiB7fSB9XG4gICAgOiBuZXcgRmluYWxpemF0aW9uUmVnaXN0cnkocHRyID0+IHdhc20uX193Ymdfc3RhdHNfZnJlZShwdHIgPj4+IDApKTtcbi8qKlxuKi9cbmV4cG9ydCBjbGFzcyBTdGF0cyB7XG5cbiAgICBzdGF0aWMgX193cmFwKHB0cikge1xuICAgICAgICBwdHIgPSBwdHIgPj4+IDA7XG4gICAgICAgIGNvbnN0IG9iaiA9IE9iamVjdC5jcmVhdGUoU3RhdHMucHJvdG90eXBlKTtcbiAgICAgICAgb2JqLl9fd2JnX3B0ciA9IHB0cjtcbiAgICAgICAgU3RhdHNGaW5hbGl6YXRpb24ucmVnaXN0ZXIob2JqLCBvYmouX193YmdfcHRyLCBvYmopO1xuICAgICAgICByZXR1cm4gb2JqO1xuICAgIH1cblxuICAgIF9fZGVzdHJveV9pbnRvX3JhdygpIHtcbiAgICAgICAgY29uc3QgcHRyID0gdGhpcy5fX3diZ19wdHI7XG4gICAgICAgIHRoaXMuX193YmdfcHRyID0gMDtcbiAgICAgICAgU3RhdHNGaW5hbGl6YXRpb24udW5yZWdpc3Rlcih0aGlzKTtcbiAgICAgICAgcmV0dXJuIHB0cjtcbiAgICB9XG5cbiAgICBmcmVlKCkge1xuICAgICAgICBjb25zdCBwdHIgPSB0aGlzLl9fZGVzdHJveV9pbnRvX3JhdygpO1xuICAgICAgICB3YXNtLl9fd2JnX3N0YXRzX2ZyZWUocHRyKTtcbiAgICB9XG4gICAgLyoqXG4gICAgKiBAcmV0dXJucyB7U3RhdHN9XG4gICAgKi9cbiAgICBnZXRfYW5kX3Jlc2V0KCkge1xuICAgICAgICBjb25zdCByZXQgPSB3YXNtLnN0YXRzX2dldF9hbmRfcmVzZXQodGhpcy5fX3diZ19wdHIpO1xuICAgICAgICByZXR1cm4gU3RhdHMuX193cmFwKHJldCk7XG4gICAgfVxuICAgIC8qKlxuICAgICogQHJldHVybnMge1N0YXRzfVxuICAgICovXG4gICAgc3RhdGljIHplcm8oKSB7XG4gICAgICAgIGNvbnN0IHJldCA9IHdhc20uc3RhdHNfemVybygpO1xuICAgICAgICByZXR1cm4gU3RhdHMuX193cmFwKHJldCk7XG4gICAgfVxuICAgIC8qKlxuICAgICogQHJldHVybnMge251bWJlcn1cbiAgICAqL1xuICAgIHRpY2tzKCkge1xuICAgICAgICBjb25zdCByZXQgPSB3YXNtLnN0YXRzX3RpY2tzKHRoaXMuX193YmdfcHRyKTtcbiAgICAgICAgcmV0dXJuIHJldCA+Pj4gMDtcbiAgICB9XG4gICAgLyoqXG4gICAgKiBAcmV0dXJucyB7bnVtYmVyfVxuICAgICovXG4gICAgY2VsbHNfY291bnQoKSB7XG4gICAgICAgIGNvbnN0IHJldCA9IHdhc20uc3RhdHNfY2VsbHNfY291bnQodGhpcy5fX3diZ19wdHIpO1xuICAgICAgICByZXR1cm4gcmV0ID4+PiAwO1xuICAgIH1cbiAgICAvKipcbiAgICAqIEByZXR1cm5zIHtudW1iZXJ9XG4gICAgKi9cbiAgICBjb2xsaXNpb25zX2NvdW50KCkge1xuICAgICAgICBjb25zdCByZXQgPSB3YXNtLnN0YXRzX2NvbGxpc2lvbnNfY291bnQodGhpcy5fX3diZ19wdHIpO1xuICAgICAgICByZXR1cm4gcmV0ID4+PiAwO1xuICAgIH1cbiAgICAvKipcbiAgICAqIEByZXR1cm5zIHtudW1iZXJ9XG4gICAgKi9cbiAgICBjb2xsaXNpb25fcGFpcnNfdGVzdGVkKCkge1xuICAgICAgICBjb25zdCByZXQgPSB3YXNtLnN0YXRzX2NvbGxpc2lvbl9wYWlyc190ZXN0ZWQodGhpcy5fX3diZ19wdHIpO1xuICAgICAgICByZXR1cm4gcmV0ID4+PiAwO1xuICAgIH1cbn1cblxuZXhwb3J0IGZ1bmN0aW9uIF9fd2JnX25ld19hYmRhNzZlODgzYmE4YTVmKCkge1xuICAgIGNvbnN0IHJldCA9IG5ldyBFcnJvcigpO1xuICAgIHJldHVybiBhZGRIZWFwT2JqZWN0KHJldCk7XG59O1xuXG5leHBvcnQgZnVuY3Rpb24gX193Ymdfc3RhY2tfNjU4Mjc5ZmU0NDU0MWNmNihhcmcwLCBhcmcxKSB7XG4gICAgY29uc3QgcmV0ID0gZ2V0T2JqZWN0KGFyZzEpLnN0YWNrO1xuICAgIGNvbnN0IHB0cjEgPSBwYXNzU3RyaW5nVG9XYXNtMChyZXQsIHdhc20uX193YmluZGdlbl9tYWxsb2MsIHdhc20uX193YmluZGdlbl9yZWFsbG9jKTtcbiAgICBjb25zdCBsZW4xID0gV0FTTV9WRUNUT1JfTEVOO1xuICAgIGdldEludDMyTWVtb3J5MCgpW2FyZzAgLyA0ICsgMV0gPSBsZW4xO1xuICAgIGdldEludDMyTWVtb3J5MCgpW2FyZzAgLyA0ICsgMF0gPSBwdHIxO1xufTtcblxuZXhwb3J0IGZ1bmN0aW9uIF9fd2JnX2Vycm9yX2Y4NTE2NjdhZjcxYmNmYzYoYXJnMCwgYXJnMSkge1xuICAgIGxldCBkZWZlcnJlZDBfMDtcbiAgICBsZXQgZGVmZXJyZWQwXzE7XG4gICAgdHJ5IHtcbiAgICAgICAgZGVmZXJyZWQwXzAgPSBhcmcwO1xuICAgICAgICBkZWZlcnJlZDBfMSA9IGFyZzE7XG4gICAgICAgIGNvbnNvbGUuZXJyb3IoZ2V0U3RyaW5nRnJvbVdhc20wKGFyZzAsIGFyZzEpKTtcbiAgICB9IGZpbmFsbHkge1xuICAgICAgICB3YXNtLl9fd2JpbmRnZW5fZnJlZShkZWZlcnJlZDBfMCwgZGVmZXJyZWQwXzEsIDEpO1xuICAgIH1cbn07XG5cbmV4cG9ydCBmdW5jdGlvbiBfX3diaW5kZ2VuX29iamVjdF9kcm9wX3JlZihhcmcwKSB7XG4gICAgdGFrZU9iamVjdChhcmcwKTtcbn07XG5cbmV4cG9ydCBmdW5jdGlvbiBfX3diaW5kZ2VuX3Rocm93KGFyZzAsIGFyZzEpIHtcbiAgICB0aHJvdyBuZXcgRXJyb3IoZ2V0U3RyaW5nRnJvbVdhc20wKGFyZzAsIGFyZzEpKTtcbn07XG5cbiJdLCJuYW1lcyI6W10sInNvdXJjZVJvb3QiOiIifQ==