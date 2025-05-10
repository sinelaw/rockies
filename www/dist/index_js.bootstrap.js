"use strict";
(self["webpackChunkcreate_wasm_app"] = self["webpackChunkcreate_wasm_app"] || []).push([["index_js"],{

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {
__webpack_require__.r(__webpack_exports__);
/* harmony import */ var rockies__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! rockies */ "./node_modules/rockies/rockies.js");
/* harmony import */ var rockies_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! rockies/rockies_bg.wasm */ "./node_modules/rockies/rockies_bg.wasm");
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

canvas.addEventListener('blur', function (event) {
    game.unfocus();
});

window.addEventListener('blur', function (event) {
    game.unfocus();
});

document.onkeydown = (e) => {
    game.key_down(e.key);
};

document.onkeyup = (e) => {
    game.key_up(e.key);
};

__webpack_async_result__();
} catch(e) { __webpack_async_result__(e); } });

/***/ }),

/***/ "./node_modules/rockies/rockies.js":
/*!*****************************************!*\
  !*** ./node_modules/rockies/rockies.js ***!
  \*****************************************/
/***/ ((__webpack_module__, __webpack_exports__, __webpack_require__) => {

__webpack_require__.a(__webpack_module__, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {
__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   Game: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.Game),
/* harmony export */   Stats: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.Stats),
/* harmony export */   __wbg_set_wasm: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm),
/* harmony export */   __wbindgen_init_externref_table: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_init_externref_table),
/* harmony export */   __wbindgen_throw: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_throw)
/* harmony export */ });
/* harmony import */ var _rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./rockies_bg.wasm */ "./node_modules/rockies/rockies_bg.wasm");
/* harmony import */ var _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./rockies_bg.js */ "./node_modules/rockies/rockies_bg.js");
var __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);
_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];



(0,_rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm)(_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__);
_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__.__wbindgen_start();

__webpack_async_result__();
} catch(e) { __webpack_async_result__(e); } });

/***/ }),

/***/ "./node_modules/rockies/rockies_bg.js":
/*!********************************************!*\
  !*** ./node_modules/rockies/rockies_bg.js ***!
  \********************************************/
/***/ ((__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   Game: () => (/* binding */ Game),
/* harmony export */   Stats: () => (/* binding */ Stats),
/* harmony export */   __wbg_set_wasm: () => (/* binding */ __wbg_set_wasm),
/* harmony export */   __wbindgen_init_externref_table: () => (/* binding */ __wbindgen_init_externref_table),
/* harmony export */   __wbindgen_throw: () => (/* binding */ __wbindgen_throw)
/* harmony export */ });
let wasm;
function __wbg_set_wasm(val) {
    wasm = val;
}


const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

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

const GameFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_game_free(ptr >>> 0, 1));

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
        wasm.__wbg_game_free(ptr, 0);
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
    tick() {
        wasm.game_tick(this.__wbg_ptr);
    }
    render() {
        wasm.game_render(this.__wbg_ptr);
    }
    /**
     * @param {string} key
     */
    key_down(key) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.game_key_down(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {string} key
     */
    key_up(key) {
        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.game_key_up(this.__wbg_ptr, ptr0, len0);
    }
    unfocus() {
        wasm.game_unfocus(this.__wbg_ptr);
    }
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
    : new FinalizationRegistry(ptr => wasm.__wbg_stats_free(ptr >>> 0, 1));

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
        wasm.__wbg_stats_free(ptr, 0);
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

function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_export_0;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};



/***/ }),

/***/ "./node_modules/rockies/rockies_bg.wasm":
/*!**********************************************!*\
  !*** ./node_modules/rockies/rockies_bg.wasm ***!
  \**********************************************/
/***/ ((module, exports, __webpack_require__) => {

/* harmony import */ var WEBPACK_IMPORTED_MODULE_0 = __webpack_require__(/*! ./rockies_bg.js */ "./node_modules/rockies/rockies_bg.js");
module.exports = __webpack_require__.v(exports, module.id, "e1a8d93d1ba5a32f6fb9", {
	"./rockies_bg.js": {
		"__wbindgen_throw": WEBPACK_IMPORTED_MODULE_0.__wbindgen_throw,
		"__wbindgen_init_externref_table": WEBPACK_IMPORTED_MODULE_0.__wbindgen_init_externref_table
	}
});

/***/ })

}]);
//# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiaW5kZXhfanMuYm9vdHN0cmFwLmpzIiwibWFwcGluZ3MiOiI7Ozs7Ozs7Ozs7Ozs7OztBQUFxQztBQUNZOztBQUVqRDs7QUFFQTtBQUNBLHVGQUF1Rjs7O0FBR3ZGLGFBQWEseUNBQUk7QUFDakI7QUFDQTs7QUFFQTtBQUNBO0FBQ0E7QUFDQTs7QUFFQTtBQUNBOztBQUVBOztBQUVBOztBQUVBOztBQUVBOztBQUVBOztBQUVBO0FBQ0E7QUFDQTtBQUNBOzs7O0FBSUE7QUFDQTs7QUFFQTtBQUNBO0FBQ0E7O0FBRUE7QUFDQTtBQUNBLG1DQUFtQywyREFBTTs7QUFFekMsc0JBQXNCLGNBQWM7QUFDcEMsMEJBQTBCLGFBQWE7QUFDdkM7QUFDQTs7QUFFQTtBQUNBO0FBQ0E7O0FBRUE7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBOztBQUVBO0FBQ0E7QUFDQTs7QUFFQTs7QUFFQTtBQUNBOztBQUVBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7O0FBRUE7QUFDQTtBQUNBOztBQUVBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTs7QUFFQTtBQUNBO0FBQ0EsQ0FBQzs7QUFFRDtBQUNBO0FBQ0EsQ0FBQzs7QUFFRDtBQUNBO0FBQ0E7O0FBRUE7QUFDQTtBQUNBOzs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7OztBQ3pHMEM7QUFDVjtBQUNpQjtBQUNqRCw4REFBYyxDQUFDLDZDQUFJO0FBQ25CLDhEQUFxQjs7Ozs7Ozs7Ozs7Ozs7Ozs7Ozs7O0FDSnJCO0FBQ087QUFDUDtBQUNBOzs7QUFHQTs7QUFFQSxvREFBb0QsOEJBQThCOztBQUVsRjs7QUFFQTs7QUFFQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7O0FBRUE7QUFDQTtBQUNBO0FBQ0E7O0FBRUE7O0FBRUE7O0FBRUE7O0FBRUE7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBLENBQUM7O0FBRUQ7O0FBRUE7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7O0FBRUE7QUFDQTs7QUFFQTs7QUFFQTs7QUFFQSxXQUFXLGNBQWM7QUFDekI7QUFDQTtBQUNBO0FBQ0E7O0FBRUE7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7O0FBRUE7QUFDQTtBQUNBOztBQUVBO0FBQ0E7QUFDQTs7QUFFQTtBQUNBLFFBQVEsa0JBQWtCO0FBQzFCOztBQUVPOztBQUVQO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBOztBQUVBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTs7QUFFQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0EsZUFBZSxRQUFRO0FBQ3ZCLGVBQWUsUUFBUTtBQUN2QixpQkFBaUI7QUFDakI7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0EsaUJBQWlCO0FBQ2pCO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBLGVBQWUsUUFBUTtBQUN2QjtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBLGVBQWUsUUFBUTtBQUN2QjtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBLGVBQWUsUUFBUTtBQUN2QixlQUFlLFFBQVE7QUFDdkI7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBLGlCQUFpQjtBQUNqQjtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQSxpQkFBaUI7QUFDakI7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0EsaUJBQWlCO0FBQ2pCO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTs7QUFFQTtBQUNBLFFBQVEsa0JBQWtCO0FBQzFCOztBQUVPOztBQUVQO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBOztBQUVBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTs7QUFFQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0EsaUJBQWlCO0FBQ2pCO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBLGlCQUFpQjtBQUNqQjtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQSxpQkFBaUI7QUFDakI7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0EsaUJBQWlCO0FBQ2pCO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBLGlCQUFpQjtBQUNqQjtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQSxpQkFBaUI7QUFDakI7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBOztBQUVPO0FBQ1A7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBO0FBQ0E7QUFDQTtBQUNBOztBQUVPO0FBQ1A7QUFDQSIsInNvdXJjZXMiOlsid2VicGFjazovL2NyZWF0ZS13YXNtLWFwcC8uL2luZGV4LmpzIiwid2VicGFjazovL2NyZWF0ZS13YXNtLWFwcC8uL25vZGVfbW9kdWxlcy9yb2NraWVzL3JvY2tpZXMuanMiLCJ3ZWJwYWNrOi8vY3JlYXRlLXdhc20tYXBwLy4vbm9kZV9tb2R1bGVzL3JvY2tpZXMvcm9ja2llc19iZy5qcyJdLCJzb3VyY2VzQ29udGVudCI6WyJpbXBvcnQgeyBHYW1lLCBDZWxsIH0gZnJvbSBcInJvY2tpZXNcIjtcbmltcG9ydCB7IG1lbW9yeSB9IGZyb20gXCJyb2NraWVzL3JvY2tpZXNfYmcud2FzbVwiO1xuXG5jb25zdCBjYW52YXMgPSBkb2N1bWVudC5nZXRFbGVtZW50QnlJZChcInRoZS1jYW52YXNcIik7XG5cbmNvbnN0IFNJWkUgPSA2NDtcbmNvbnN0IENFTExfU0laRSA9IE1hdGgubWluKGNhbnZhcy5jbGllbnRXaWR0aCAvIFNJWkUsIGNhbnZhcy5jbGllbnRIZWlnaHQgLyBTSVpFKSB8IDA7IC8vIHB4XG5cblxuY29uc3QgZ2FtZSA9IEdhbWUubmV3KFNJWkUsIFNJWkUpO1xuY29uc3Qgd2lkdGggPSBnYW1lLndpZHRoKCk7XG5jb25zdCBoZWlnaHQgPSBnYW1lLmhlaWdodCgpO1xuXG5jb25zdCB0aWNrcyA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKFwidGlja3NcIik7XG5jb25zdCBjZWxsc19jb3VudCA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKFwiY2VsbHMtY291bnRcIik7XG5jb25zdCBjb2xsaXNpb25zX2NvdW50ID0gZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoXCJjb2xsaXNpb25zLWNvdW50XCIpO1xuY29uc3QgY29sbGlzaW9uX3BhaXJzX3Rlc3RlZCA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKFwiY29sbGlzaW9uLXBhaXJzLXRlc3RlZFwiKTtcblxuY2FudmFzLmhlaWdodCA9IChDRUxMX1NJWkUpICogaGVpZ2h0ICsgMTtcbmNhbnZhcy53aWR0aCA9IChDRUxMX1NJWkUpICogd2lkdGggKyAxO1xuXG5jb25zdCBjdHggPSBjYW52YXMuZ2V0Q29udGV4dCgnMmQnKTtcblxuY29uc3QgcmVuZGVyTG9vcCA9ICgpID0+IHtcblxuICAgIGdhbWUudGljaygpO1xuXG4gICAgZHJhd1BpeGVscygpO1xuXG4gICAgbGV0IHN0YXRzID0gZ2FtZS5zdGF0cygpO1xuXG4gICAgdGlja3MudGV4dENvbnRlbnQgPSBzdGF0cy50aWNrcygpO1xuICAgIGNlbGxzX2NvdW50LnRleHRDb250ZW50ID0gc3RhdHMuY2VsbHNfY291bnQoKTtcbiAgICBjb2xsaXNpb25zX2NvdW50LnRleHRDb250ZW50ID0gKHN0YXRzLmNvbGxpc2lvbnNfY291bnQoKSAvIHN0YXRzLnRpY2tzKCkpIHwgMDtcbiAgICBjb2xsaXNpb25fcGFpcnNfdGVzdGVkLnRleHRDb250ZW50ID0gKHN0YXRzLmNvbGxpc2lvbl9wYWlyc190ZXN0ZWQoKSAvIHN0YXRzLnRpY2tzKCkpIHwgMDtcblxuXG5cbiAgICByZXF1ZXN0QW5pbWF0aW9uRnJhbWUocmVuZGVyTG9vcCk7XG59O1xuXG5jb25zdCBnZXRJbmRleCA9IChyb3csIGNvbHVtbikgPT4ge1xuICAgIHJldHVybiByb3cgKiB3aWR0aCArIGNvbHVtbjtcbn07XG5cbmNvbnN0IGRyYXdQaXhlbHMgPSAoKSA9PiB7XG4gICAgY29uc3QgcGl4ZWxzUHRyID0gZ2FtZS5waXhlbHMoKTtcbiAgICBjb25zdCBwaXhlbHMgPSBuZXcgVWludDMyQXJyYXkobWVtb3J5LmJ1ZmZlciwgcGl4ZWxzUHRyLCB3aWR0aCAqIGhlaWdodCk7XG5cbiAgICBmb3IgKGxldCByb3cgPSAwOyByb3cgPCBoZWlnaHQ7IHJvdysrKSB7XG4gICAgICAgIGZvciAobGV0IGNvbCA9IDA7IGNvbCA8IHdpZHRoOyBjb2wrKykge1xuICAgICAgICAgICAgY29uc3QgaWR4ID0gZ2V0SW5kZXgocm93LCBjb2wpO1xuICAgICAgICAgICAgY3R4LmJlZ2luUGF0aCgpO1xuXG4gICAgICAgICAgICBsZXQgdmFsID0gcGl4ZWxzW2lkeF07XG4gICAgICAgICAgICBjdHguZmlsbFN0eWxlID0gXCIjXCIgKyB2YWwudG9TdHJpbmcoMTYpLnBhZFN0YXJ0KDYsIFwiMFwiKTtcbiAgICAgICAgICAgIC8vY29uc29sZS5sb2coXCJbJWQsJWRdID0gJXMgPSAlc1wiLCByb3csIGNvbCwgcGl4ZWxzW2lkeF0udG9TdHJpbmcoMTYpLCBjdHguZmlsbFN0eWxlKTtcblxuICAgICAgICAgICAgY3R4LmZpbGxSZWN0KFxuICAgICAgICAgICAgICAgIGNvbCAqIENFTExfU0laRSArIDEsXG4gICAgICAgICAgICAgICAgcm93ICogQ0VMTF9TSVpFICsgMSxcbiAgICAgICAgICAgICAgICBDRUxMX1NJWkUsXG4gICAgICAgICAgICAgICAgQ0VMTF9TSVpFXG4gICAgICAgICAgICApO1xuXG4gICAgICAgICAgICBjdHguc3Ryb2tlKCk7XG4gICAgICAgIH1cbiAgICB9XG5cbn07XG5cbmRyYXdQaXhlbHMoKTtcbnJlcXVlc3RBbmltYXRpb25GcmFtZShyZW5kZXJMb29wKTtcblxuY2FudmFzLm9ubW91c2Vtb3ZlID0gKGUpID0+IHtcbiAgICBpZiAoZS5idXR0b25zID4gMCkge1xuICAgICAgICBnYW1lLmNsaWNrKGUub2Zmc2V0WCAvIChDRUxMX1NJWkUgKyAxKSwgZS5vZmZzZXRZIC8gKENFTExfU0laRSArIDEpKTtcbiAgICB9XG59O1xuXG5jYW52YXMub25jbGljayA9IChlKSA9PiB7XG4gICAgZ2FtZS5jbGljayhlLm9mZnNldFggLyAoQ0VMTF9TSVpFICsgMSksIGUub2Zmc2V0WSAvIChDRUxMX1NJWkUgKyAxKSk7XG59O1xuXG5jYW52YXMub250b3VjaG1vdmUgPSAoZSkgPT4ge1xuICAgIGUucHJldmVudERlZmF1bHQoKTtcbiAgICBsZXQgeCA9IGUudG91Y2hlc1swXS5jbGllbnRYIC0gY2FudmFzLm9mZnNldExlZnQ7XG4gICAgbGV0IHkgPSBlLnRvdWNoZXNbMF0uY2xpZW50WSAtIGNhbnZhcy5vZmZzZXRUb3A7XG4gICAgZ2FtZS5jbGljayh4IC8gKENFTExfU0laRSArIDEpLCB5IC8gKENFTExfU0laRSArIDEpKTtcbn07XG5cbmNhbnZhcy5hZGRFdmVudExpc3RlbmVyKCdibHVyJywgZnVuY3Rpb24gKGV2ZW50KSB7XG4gICAgZ2FtZS51bmZvY3VzKCk7XG59KTtcblxud2luZG93LmFkZEV2ZW50TGlzdGVuZXIoJ2JsdXInLCBmdW5jdGlvbiAoZXZlbnQpIHtcbiAgICBnYW1lLnVuZm9jdXMoKTtcbn0pO1xuXG5kb2N1bWVudC5vbmtleWRvd24gPSAoZSkgPT4ge1xuICAgIGdhbWUua2V5X2Rvd24oZS5rZXkpO1xufTtcblxuZG9jdW1lbnQub25rZXl1cCA9IChlKSA9PiB7XG4gICAgZ2FtZS5rZXlfdXAoZS5rZXkpO1xufTtcbiIsImltcG9ydCAqIGFzIHdhc20gZnJvbSBcIi4vcm9ja2llc19iZy53YXNtXCI7XG5leHBvcnQgKiBmcm9tIFwiLi9yb2NraWVzX2JnLmpzXCI7XG5pbXBvcnQgeyBfX3diZ19zZXRfd2FzbSB9IGZyb20gXCIuL3JvY2tpZXNfYmcuanNcIjtcbl9fd2JnX3NldF93YXNtKHdhc20pO1xud2FzbS5fX3diaW5kZ2VuX3N0YXJ0KCk7XG4iLCJsZXQgd2FzbTtcbmV4cG9ydCBmdW5jdGlvbiBfX3diZ19zZXRfd2FzbSh2YWwpIHtcbiAgICB3YXNtID0gdmFsO1xufVxuXG5cbmNvbnN0IGxUZXh0RGVjb2RlciA9IHR5cGVvZiBUZXh0RGVjb2RlciA9PT0gJ3VuZGVmaW5lZCcgPyAoMCwgbW9kdWxlLnJlcXVpcmUpKCd1dGlsJykuVGV4dERlY29kZXIgOiBUZXh0RGVjb2RlcjtcblxubGV0IGNhY2hlZFRleHREZWNvZGVyID0gbmV3IGxUZXh0RGVjb2RlcigndXRmLTgnLCB7IGlnbm9yZUJPTTogdHJ1ZSwgZmF0YWw6IHRydWUgfSk7XG5cbmNhY2hlZFRleHREZWNvZGVyLmRlY29kZSgpO1xuXG5sZXQgY2FjaGVkVWludDhBcnJheU1lbW9yeTAgPSBudWxsO1xuXG5mdW5jdGlvbiBnZXRVaW50OEFycmF5TWVtb3J5MCgpIHtcbiAgICBpZiAoY2FjaGVkVWludDhBcnJheU1lbW9yeTAgPT09IG51bGwgfHwgY2FjaGVkVWludDhBcnJheU1lbW9yeTAuYnl0ZUxlbmd0aCA9PT0gMCkge1xuICAgICAgICBjYWNoZWRVaW50OEFycmF5TWVtb3J5MCA9IG5ldyBVaW50OEFycmF5KHdhc20ubWVtb3J5LmJ1ZmZlcik7XG4gICAgfVxuICAgIHJldHVybiBjYWNoZWRVaW50OEFycmF5TWVtb3J5MDtcbn1cblxuZnVuY3Rpb24gZ2V0U3RyaW5nRnJvbVdhc20wKHB0ciwgbGVuKSB7XG4gICAgcHRyID0gcHRyID4+PiAwO1xuICAgIHJldHVybiBjYWNoZWRUZXh0RGVjb2Rlci5kZWNvZGUoZ2V0VWludDhBcnJheU1lbW9yeTAoKS5zdWJhcnJheShwdHIsIHB0ciArIGxlbikpO1xufVxuXG5sZXQgV0FTTV9WRUNUT1JfTEVOID0gMDtcblxuY29uc3QgbFRleHRFbmNvZGVyID0gdHlwZW9mIFRleHRFbmNvZGVyID09PSAndW5kZWZpbmVkJyA/ICgwLCBtb2R1bGUucmVxdWlyZSkoJ3V0aWwnKS5UZXh0RW5jb2RlciA6IFRleHRFbmNvZGVyO1xuXG5sZXQgY2FjaGVkVGV4dEVuY29kZXIgPSBuZXcgbFRleHRFbmNvZGVyKCd1dGYtOCcpO1xuXG5jb25zdCBlbmNvZGVTdHJpbmcgPSAodHlwZW9mIGNhY2hlZFRleHRFbmNvZGVyLmVuY29kZUludG8gPT09ICdmdW5jdGlvbidcbiAgICA/IGZ1bmN0aW9uIChhcmcsIHZpZXcpIHtcbiAgICByZXR1cm4gY2FjaGVkVGV4dEVuY29kZXIuZW5jb2RlSW50byhhcmcsIHZpZXcpO1xufVxuICAgIDogZnVuY3Rpb24gKGFyZywgdmlldykge1xuICAgIGNvbnN0IGJ1ZiA9IGNhY2hlZFRleHRFbmNvZGVyLmVuY29kZShhcmcpO1xuICAgIHZpZXcuc2V0KGJ1Zik7XG4gICAgcmV0dXJuIHtcbiAgICAgICAgcmVhZDogYXJnLmxlbmd0aCxcbiAgICAgICAgd3JpdHRlbjogYnVmLmxlbmd0aFxuICAgIH07XG59KTtcblxuZnVuY3Rpb24gcGFzc1N0cmluZ1RvV2FzbTAoYXJnLCBtYWxsb2MsIHJlYWxsb2MpIHtcblxuICAgIGlmIChyZWFsbG9jID09PSB1bmRlZmluZWQpIHtcbiAgICAgICAgY29uc3QgYnVmID0gY2FjaGVkVGV4dEVuY29kZXIuZW5jb2RlKGFyZyk7XG4gICAgICAgIGNvbnN0IHB0ciA9IG1hbGxvYyhidWYubGVuZ3RoLCAxKSA+Pj4gMDtcbiAgICAgICAgZ2V0VWludDhBcnJheU1lbW9yeTAoKS5zdWJhcnJheShwdHIsIHB0ciArIGJ1Zi5sZW5ndGgpLnNldChidWYpO1xuICAgICAgICBXQVNNX1ZFQ1RPUl9MRU4gPSBidWYubGVuZ3RoO1xuICAgICAgICByZXR1cm4gcHRyO1xuICAgIH1cblxuICAgIGxldCBsZW4gPSBhcmcubGVuZ3RoO1xuICAgIGxldCBwdHIgPSBtYWxsb2MobGVuLCAxKSA+Pj4gMDtcblxuICAgIGNvbnN0IG1lbSA9IGdldFVpbnQ4QXJyYXlNZW1vcnkwKCk7XG5cbiAgICBsZXQgb2Zmc2V0ID0gMDtcblxuICAgIGZvciAoOyBvZmZzZXQgPCBsZW47IG9mZnNldCsrKSB7XG4gICAgICAgIGNvbnN0IGNvZGUgPSBhcmcuY2hhckNvZGVBdChvZmZzZXQpO1xuICAgICAgICBpZiAoY29kZSA+IDB4N0YpIGJyZWFrO1xuICAgICAgICBtZW1bcHRyICsgb2Zmc2V0XSA9IGNvZGU7XG4gICAgfVxuXG4gICAgaWYgKG9mZnNldCAhPT0gbGVuKSB7XG4gICAgICAgIGlmIChvZmZzZXQgIT09IDApIHtcbiAgICAgICAgICAgIGFyZyA9IGFyZy5zbGljZShvZmZzZXQpO1xuICAgICAgICB9XG4gICAgICAgIHB0ciA9IHJlYWxsb2MocHRyLCBsZW4sIGxlbiA9IG9mZnNldCArIGFyZy5sZW5ndGggKiAzLCAxKSA+Pj4gMDtcbiAgICAgICAgY29uc3QgdmlldyA9IGdldFVpbnQ4QXJyYXlNZW1vcnkwKCkuc3ViYXJyYXkocHRyICsgb2Zmc2V0LCBwdHIgKyBsZW4pO1xuICAgICAgICBjb25zdCByZXQgPSBlbmNvZGVTdHJpbmcoYXJnLCB2aWV3KTtcblxuICAgICAgICBvZmZzZXQgKz0gcmV0LndyaXR0ZW47XG4gICAgICAgIHB0ciA9IHJlYWxsb2MocHRyLCBsZW4sIG9mZnNldCwgMSkgPj4+IDA7XG4gICAgfVxuXG4gICAgV0FTTV9WRUNUT1JfTEVOID0gb2Zmc2V0O1xuICAgIHJldHVybiBwdHI7XG59XG5cbmNvbnN0IEdhbWVGaW5hbGl6YXRpb24gPSAodHlwZW9mIEZpbmFsaXphdGlvblJlZ2lzdHJ5ID09PSAndW5kZWZpbmVkJylcbiAgICA/IHsgcmVnaXN0ZXI6ICgpID0+IHt9LCB1bnJlZ2lzdGVyOiAoKSA9PiB7fSB9XG4gICAgOiBuZXcgRmluYWxpemF0aW9uUmVnaXN0cnkocHRyID0+IHdhc20uX193YmdfZ2FtZV9mcmVlKHB0ciA+Pj4gMCwgMSkpO1xuXG5leHBvcnQgY2xhc3MgR2FtZSB7XG5cbiAgICBzdGF0aWMgX193cmFwKHB0cikge1xuICAgICAgICBwdHIgPSBwdHIgPj4+IDA7XG4gICAgICAgIGNvbnN0IG9iaiA9IE9iamVjdC5jcmVhdGUoR2FtZS5wcm90b3R5cGUpO1xuICAgICAgICBvYmouX193YmdfcHRyID0gcHRyO1xuICAgICAgICBHYW1lRmluYWxpemF0aW9uLnJlZ2lzdGVyKG9iaiwgb2JqLl9fd2JnX3B0ciwgb2JqKTtcbiAgICAgICAgcmV0dXJuIG9iajtcbiAgICB9XG5cbiAgICBfX2Rlc3Ryb3lfaW50b19yYXcoKSB7XG4gICAgICAgIGNvbnN0IHB0ciA9IHRoaXMuX193YmdfcHRyO1xuICAgICAgICB0aGlzLl9fd2JnX3B0ciA9IDA7XG4gICAgICAgIEdhbWVGaW5hbGl6YXRpb24udW5yZWdpc3Rlcih0aGlzKTtcbiAgICAgICAgcmV0dXJuIHB0cjtcbiAgICB9XG5cbiAgICBmcmVlKCkge1xuICAgICAgICBjb25zdCBwdHIgPSB0aGlzLl9fZGVzdHJveV9pbnRvX3JhdygpO1xuICAgICAgICB3YXNtLl9fd2JnX2dhbWVfZnJlZShwdHIsIDApO1xuICAgIH1cbiAgICAvKipcbiAgICAgKiBAcGFyYW0ge251bWJlcn0gd2lkdGhcbiAgICAgKiBAcGFyYW0ge251bWJlcn0gaGVpZ2h0XG4gICAgICogQHJldHVybnMge0dhbWV9XG4gICAgICovXG4gICAgc3RhdGljIG5ldyh3aWR0aCwgaGVpZ2h0KSB7XG4gICAgICAgIGNvbnN0IHJldCA9IHdhc20uZ2FtZV9uZXcod2lkdGgsIGhlaWdodCk7XG4gICAgICAgIHJldHVybiBHYW1lLl9fd3JhcChyZXQpO1xuICAgIH1cbiAgICAvKipcbiAgICAgKiBAcmV0dXJucyB7bnVtYmVyfVxuICAgICAqL1xuICAgIHBpeGVscygpIHtcbiAgICAgICAgY29uc3QgcmV0ID0gd2FzbS5nYW1lX3BpeGVscyh0aGlzLl9fd2JnX3B0cik7XG4gICAgICAgIHJldHVybiByZXQgPj4+IDA7XG4gICAgfVxuICAgIHRpY2soKSB7XG4gICAgICAgIHdhc20uZ2FtZV90aWNrKHRoaXMuX193YmdfcHRyKTtcbiAgICB9XG4gICAgcmVuZGVyKCkge1xuICAgICAgICB3YXNtLmdhbWVfcmVuZGVyKHRoaXMuX193YmdfcHRyKTtcbiAgICB9XG4gICAgLyoqXG4gICAgICogQHBhcmFtIHtzdHJpbmd9IGtleVxuICAgICAqL1xuICAgIGtleV9kb3duKGtleSkge1xuICAgICAgICBjb25zdCBwdHIwID0gcGFzc1N0cmluZ1RvV2FzbTAoa2V5LCB3YXNtLl9fd2JpbmRnZW5fbWFsbG9jLCB3YXNtLl9fd2JpbmRnZW5fcmVhbGxvYyk7XG4gICAgICAgIGNvbnN0IGxlbjAgPSBXQVNNX1ZFQ1RPUl9MRU47XG4gICAgICAgIHdhc20uZ2FtZV9rZXlfZG93bih0aGlzLl9fd2JnX3B0ciwgcHRyMCwgbGVuMCk7XG4gICAgfVxuICAgIC8qKlxuICAgICAqIEBwYXJhbSB7c3RyaW5nfSBrZXlcbiAgICAgKi9cbiAgICBrZXlfdXAoa2V5KSB7XG4gICAgICAgIGNvbnN0IHB0cjAgPSBwYXNzU3RyaW5nVG9XYXNtMChrZXksIHdhc20uX193YmluZGdlbl9tYWxsb2MsIHdhc20uX193YmluZGdlbl9yZWFsbG9jKTtcbiAgICAgICAgY29uc3QgbGVuMCA9IFdBU01fVkVDVE9SX0xFTjtcbiAgICAgICAgd2FzbS5nYW1lX2tleV91cCh0aGlzLl9fd2JnX3B0ciwgcHRyMCwgbGVuMCk7XG4gICAgfVxuICAgIHVuZm9jdXMoKSB7XG4gICAgICAgIHdhc20uZ2FtZV91bmZvY3VzKHRoaXMuX193YmdfcHRyKTtcbiAgICB9XG4gICAgcHJvY2Vzc19rZXlzKCkge1xuICAgICAgICB3YXNtLmdhbWVfcHJvY2Vzc19rZXlzKHRoaXMuX193YmdfcHRyKTtcbiAgICB9XG4gICAgLyoqXG4gICAgICogQHBhcmFtIHtudW1iZXJ9IHhcbiAgICAgKiBAcGFyYW0ge251bWJlcn0geVxuICAgICAqL1xuICAgIGNsaWNrKHgsIHkpIHtcbiAgICAgICAgd2FzbS5nYW1lX2NsaWNrKHRoaXMuX193YmdfcHRyLCB4LCB5KTtcbiAgICB9XG4gICAgLyoqXG4gICAgICogQHJldHVybnMge251bWJlcn1cbiAgICAgKi9cbiAgICB3aWR0aCgpIHtcbiAgICAgICAgY29uc3QgcmV0ID0gd2FzbS5nYW1lX3dpZHRoKHRoaXMuX193YmdfcHRyKTtcbiAgICAgICAgcmV0dXJuIHJldCA+Pj4gMDtcbiAgICB9XG4gICAgLyoqXG4gICAgICogQHJldHVybnMge251bWJlcn1cbiAgICAgKi9cbiAgICBoZWlnaHQoKSB7XG4gICAgICAgIGNvbnN0IHJldCA9IHdhc20uZ2FtZV9oZWlnaHQodGhpcy5fX3diZ19wdHIpO1xuICAgICAgICByZXR1cm4gcmV0ID4+PiAwO1xuICAgIH1cbiAgICAvKipcbiAgICAgKiBAcmV0dXJucyB7U3RhdHN9XG4gICAgICovXG4gICAgc3RhdHMoKSB7XG4gICAgICAgIGNvbnN0IHJldCA9IHdhc20uZ2FtZV9zdGF0cyh0aGlzLl9fd2JnX3B0cik7XG4gICAgICAgIHJldHVybiBTdGF0cy5fX3dyYXAocmV0KTtcbiAgICB9XG59XG5cbmNvbnN0IFN0YXRzRmluYWxpemF0aW9uID0gKHR5cGVvZiBGaW5hbGl6YXRpb25SZWdpc3RyeSA9PT0gJ3VuZGVmaW5lZCcpXG4gICAgPyB7IHJlZ2lzdGVyOiAoKSA9PiB7fSwgdW5yZWdpc3RlcjogKCkgPT4ge30gfVxuICAgIDogbmV3IEZpbmFsaXphdGlvblJlZ2lzdHJ5KHB0ciA9PiB3YXNtLl9fd2JnX3N0YXRzX2ZyZWUocHRyID4+PiAwLCAxKSk7XG5cbmV4cG9ydCBjbGFzcyBTdGF0cyB7XG5cbiAgICBzdGF0aWMgX193cmFwKHB0cikge1xuICAgICAgICBwdHIgPSBwdHIgPj4+IDA7XG4gICAgICAgIGNvbnN0IG9iaiA9IE9iamVjdC5jcmVhdGUoU3RhdHMucHJvdG90eXBlKTtcbiAgICAgICAgb2JqLl9fd2JnX3B0ciA9IHB0cjtcbiAgICAgICAgU3RhdHNGaW5hbGl6YXRpb24ucmVnaXN0ZXIob2JqLCBvYmouX193YmdfcHRyLCBvYmopO1xuICAgICAgICByZXR1cm4gb2JqO1xuICAgIH1cblxuICAgIF9fZGVzdHJveV9pbnRvX3JhdygpIHtcbiAgICAgICAgY29uc3QgcHRyID0gdGhpcy5fX3diZ19wdHI7XG4gICAgICAgIHRoaXMuX193YmdfcHRyID0gMDtcbiAgICAgICAgU3RhdHNGaW5hbGl6YXRpb24udW5yZWdpc3Rlcih0aGlzKTtcbiAgICAgICAgcmV0dXJuIHB0cjtcbiAgICB9XG5cbiAgICBmcmVlKCkge1xuICAgICAgICBjb25zdCBwdHIgPSB0aGlzLl9fZGVzdHJveV9pbnRvX3JhdygpO1xuICAgICAgICB3YXNtLl9fd2JnX3N0YXRzX2ZyZWUocHRyLCAwKTtcbiAgICB9XG4gICAgLyoqXG4gICAgICogQHJldHVybnMge1N0YXRzfVxuICAgICAqL1xuICAgIGdldF9hbmRfcmVzZXQoKSB7XG4gICAgICAgIGNvbnN0IHJldCA9IHdhc20uc3RhdHNfZ2V0X2FuZF9yZXNldCh0aGlzLl9fd2JnX3B0cik7XG4gICAgICAgIHJldHVybiBTdGF0cy5fX3dyYXAocmV0KTtcbiAgICB9XG4gICAgLyoqXG4gICAgICogQHJldHVybnMge1N0YXRzfVxuICAgICAqL1xuICAgIHN0YXRpYyB6ZXJvKCkge1xuICAgICAgICBjb25zdCByZXQgPSB3YXNtLnN0YXRzX3plcm8oKTtcbiAgICAgICAgcmV0dXJuIFN0YXRzLl9fd3JhcChyZXQpO1xuICAgIH1cbiAgICAvKipcbiAgICAgKiBAcmV0dXJucyB7bnVtYmVyfVxuICAgICAqL1xuICAgIHRpY2tzKCkge1xuICAgICAgICBjb25zdCByZXQgPSB3YXNtLnN0YXRzX3RpY2tzKHRoaXMuX193YmdfcHRyKTtcbiAgICAgICAgcmV0dXJuIHJldCA+Pj4gMDtcbiAgICB9XG4gICAgLyoqXG4gICAgICogQHJldHVybnMge251bWJlcn1cbiAgICAgKi9cbiAgICBjZWxsc19jb3VudCgpIHtcbiAgICAgICAgY29uc3QgcmV0ID0gd2FzbS5zdGF0c19jZWxsc19jb3VudCh0aGlzLl9fd2JnX3B0cik7XG4gICAgICAgIHJldHVybiByZXQgPj4+IDA7XG4gICAgfVxuICAgIC8qKlxuICAgICAqIEByZXR1cm5zIHtudW1iZXJ9XG4gICAgICovXG4gICAgY29sbGlzaW9uc19jb3VudCgpIHtcbiAgICAgICAgY29uc3QgcmV0ID0gd2FzbS5zdGF0c19jb2xsaXNpb25zX2NvdW50KHRoaXMuX193YmdfcHRyKTtcbiAgICAgICAgcmV0dXJuIHJldCA+Pj4gMDtcbiAgICB9XG4gICAgLyoqXG4gICAgICogQHJldHVybnMge251bWJlcn1cbiAgICAgKi9cbiAgICBjb2xsaXNpb25fcGFpcnNfdGVzdGVkKCkge1xuICAgICAgICBjb25zdCByZXQgPSB3YXNtLnN0YXRzX2NvbGxpc2lvbl9wYWlyc190ZXN0ZWQodGhpcy5fX3diZ19wdHIpO1xuICAgICAgICByZXR1cm4gcmV0ID4+PiAwO1xuICAgIH1cbn1cblxuZXhwb3J0IGZ1bmN0aW9uIF9fd2JpbmRnZW5faW5pdF9leHRlcm5yZWZfdGFibGUoKSB7XG4gICAgY29uc3QgdGFibGUgPSB3YXNtLl9fd2JpbmRnZW5fZXhwb3J0XzA7XG4gICAgY29uc3Qgb2Zmc2V0ID0gdGFibGUuZ3Jvdyg0KTtcbiAgICB0YWJsZS5zZXQoMCwgdW5kZWZpbmVkKTtcbiAgICB0YWJsZS5zZXQob2Zmc2V0ICsgMCwgdW5kZWZpbmVkKTtcbiAgICB0YWJsZS5zZXQob2Zmc2V0ICsgMSwgbnVsbCk7XG4gICAgdGFibGUuc2V0KG9mZnNldCArIDIsIHRydWUpO1xuICAgIHRhYmxlLnNldChvZmZzZXQgKyAzLCBmYWxzZSk7XG4gICAgO1xufTtcblxuZXhwb3J0IGZ1bmN0aW9uIF9fd2JpbmRnZW5fdGhyb3coYXJnMCwgYXJnMSkge1xuICAgIHRocm93IG5ldyBFcnJvcihnZXRTdHJpbmdGcm9tV2FzbTAoYXJnMCwgYXJnMSkpO1xufTtcblxuIl0sIm5hbWVzIjpbXSwic291cmNlUm9vdCI6IiJ9