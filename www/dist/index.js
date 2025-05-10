/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var rockies__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! rockies */ \"./node_modules/rockies/rockies.js\");\n/* harmony import */ var rockies_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! rockies/rockies_bg.wasm */ \"./node_modules/rockies/rockies_bg.wasm\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([rockies__WEBPACK_IMPORTED_MODULE_0__, rockies_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);\n([rockies__WEBPACK_IMPORTED_MODULE_0__, rockies_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__] = __webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__);\n\n\n\nconst canvas = document.getElementById(\"the-canvas\");\n\nconst SIZE = 64;\nconst CELL_SIZE = Math.min(canvas.clientWidth / SIZE, canvas.clientHeight / SIZE) | 0; // px\n\n\nconst game = rockies__WEBPACK_IMPORTED_MODULE_0__.Game.new(SIZE, SIZE);\nconst width = game.width();\nconst height = game.height();\n\nconst ticks = document.getElementById(\"ticks\");\nconst cells_count = document.getElementById(\"cells-count\");\nconst collisions_count = document.getElementById(\"collisions-count\");\nconst collision_pairs_tested = document.getElementById(\"collision-pairs-tested\");\n\ncanvas.height = (CELL_SIZE) * height + 1;\ncanvas.width = (CELL_SIZE) * width + 1;\n\nconst ctx = canvas.getContext('2d');\n\nconst renderLoop = () => {\n\n    game.tick();\n\n    drawPixels();\n\n    let stats = game.stats();\n\n    ticks.textContent = stats.ticks();\n    cells_count.textContent = stats.cells_count();\n    collisions_count.textContent = (stats.collisions_count() / stats.ticks()) | 0;\n    collision_pairs_tested.textContent = (stats.collision_pairs_tested() / stats.ticks()) | 0;\n\n\n\n    requestAnimationFrame(renderLoop);\n};\n\nconst getIndex = (row, column) => {\n    return row * width + column;\n};\n\nconst drawPixels = () => {\n    const pixelsPtr = game.pixels();\n    const pixels = new Uint32Array(rockies_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__.memory.buffer, pixelsPtr, width * height);\n\n    for (let row = 0; row < height; row++) {\n        for (let col = 0; col < width; col++) {\n            const idx = getIndex(row, col);\n            ctx.beginPath();\n\n            let val = pixels[idx];\n            ctx.fillStyle = \"#\" + val.toString(16).padStart(6, \"0\");\n            //console.log(\"[%d,%d] = %s = %s\", row, col, pixels[idx].toString(16), ctx.fillStyle);\n\n            ctx.fillRect(\n                col * CELL_SIZE + 1,\n                row * CELL_SIZE + 1,\n                CELL_SIZE,\n                CELL_SIZE\n            );\n\n            ctx.stroke();\n        }\n    }\n\n};\n\ndrawPixels();\nrequestAnimationFrame(renderLoop);\n\ncanvas.onmousemove = (e) => {\n    if (e.buttons > 0) {\n        game.click(e.offsetX / (CELL_SIZE + 1), e.offsetY / (CELL_SIZE + 1));\n    }\n};\n\ncanvas.onclick = (e) => {\n    game.click(e.offsetX / (CELL_SIZE + 1), e.offsetY / (CELL_SIZE + 1));\n};\n\ncanvas.ontouchmove = (e) => {\n    e.preventDefault();\n    let x = e.touches[0].clientX - canvas.offsetLeft;\n    let y = e.touches[0].clientY - canvas.offsetTop;\n    game.click(x / (CELL_SIZE + 1), y / (CELL_SIZE + 1));\n};\n\ncanvas.addEventListener('blur', function (event) {\n    game.unfocus();\n});\n\nwindow.addEventListener('blur', function (event) {\n    game.unfocus();\n});\n\ndocument.onkeydown = (e) => {\n    game.key_down(e.key);\n};\n\ndocument.onkeyup = (e) => {\n    game.key_up(e.key);\n};\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://create-wasm-app/./index.js?");

/***/ }),

/***/ "./node_modules/rockies/rockies.js":
/*!*****************************************!*\
  !*** ./node_modules/rockies/rockies.js ***!
  \*****************************************/
/***/ ((__webpack_module__, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(__webpack_module__, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   Game: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.Game),\n/* harmony export */   Stats: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.Stats),\n/* harmony export */   __wbg_error_f851667af71bcfc6: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_error_f851667af71bcfc6),\n/* harmony export */   __wbg_new_abda76e883ba8a5f: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_new_abda76e883ba8a5f),\n/* harmony export */   __wbg_set_wasm: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm),\n/* harmony export */   __wbg_stack_658279fe44541cf6: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_stack_658279fe44541cf6),\n/* harmony export */   __wbindgen_init_externref_table: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_init_externref_table),\n/* harmony export */   __wbindgen_throw: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbindgen_throw)\n/* harmony export */ });\n/* harmony import */ var _rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./rockies_bg.wasm */ \"./node_modules/rockies/rockies_bg.wasm\");\n/* harmony import */ var _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./rockies_bg.js */ \"./node_modules/rockies/rockies_bg.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);\n_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\n\n(0,_rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm)(_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__);\n_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__.__wbindgen_start();\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://create-wasm-app/./node_modules/rockies/rockies.js?");

/***/ }),

/***/ "./node_modules/rockies/rockies_bg.js":
/*!********************************************!*\
  !*** ./node_modules/rockies/rockies_bg.js ***!
  \********************************************/
/***/ ((__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   Game: () => (/* binding */ Game),\n/* harmony export */   Stats: () => (/* binding */ Stats),\n/* harmony export */   __wbg_error_f851667af71bcfc6: () => (/* binding */ __wbg_error_f851667af71bcfc6),\n/* harmony export */   __wbg_new_abda76e883ba8a5f: () => (/* binding */ __wbg_new_abda76e883ba8a5f),\n/* harmony export */   __wbg_set_wasm: () => (/* binding */ __wbg_set_wasm),\n/* harmony export */   __wbg_stack_658279fe44541cf6: () => (/* binding */ __wbg_stack_658279fe44541cf6),\n/* harmony export */   __wbindgen_init_externref_table: () => (/* binding */ __wbindgen_init_externref_table),\n/* harmony export */   __wbindgen_throw: () => (/* binding */ __wbindgen_throw)\n/* harmony export */ });\nlet wasm;\nfunction __wbg_set_wasm(val) {\n    wasm = val;\n}\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachedUint8ArrayMemory0 = null;\n\nfunction getUint8ArrayMemory0() {\n    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {\n        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);\n    }\n    return cachedUint8ArrayMemory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    ptr = ptr >>> 0;\n    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length, 1) >>> 0;\n        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len, 1) >>> 0;\n\n    const mem = getUint8ArrayMemory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;\n        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n        ptr = realloc(ptr, len, offset, 1) >>> 0;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachedDataViewMemory0 = null;\n\nfunction getDataViewMemory0() {\n    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {\n        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);\n    }\n    return cachedDataViewMemory0;\n}\n\nconst GameFinalization = (typeof FinalizationRegistry === 'undefined')\n    ? { register: () => {}, unregister: () => {} }\n    : new FinalizationRegistry(ptr => wasm.__wbg_game_free(ptr >>> 0, 1));\n\nclass Game {\n\n    static __wrap(ptr) {\n        ptr = ptr >>> 0;\n        const obj = Object.create(Game.prototype);\n        obj.__wbg_ptr = ptr;\n        GameFinalization.register(obj, obj.__wbg_ptr, obj);\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.__wbg_ptr;\n        this.__wbg_ptr = 0;\n        GameFinalization.unregister(this);\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        wasm.__wbg_game_free(ptr, 0);\n    }\n    /**\n     * @param {number} width\n     * @param {number} height\n     * @returns {Game}\n     */\n    static new(width, height) {\n        const ret = wasm.game_new(width, height);\n        return Game.__wrap(ret);\n    }\n    /**\n     * @returns {number}\n     */\n    pixels() {\n        const ret = wasm.game_pixels(this.__wbg_ptr);\n        return ret >>> 0;\n    }\n    tick() {\n        wasm.game_tick(this.__wbg_ptr);\n    }\n    render() {\n        wasm.game_render(this.__wbg_ptr);\n    }\n    /**\n     * @param {string} key\n     */\n    key_down(key) {\n        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n        const len0 = WASM_VECTOR_LEN;\n        wasm.game_key_down(this.__wbg_ptr, ptr0, len0);\n    }\n    /**\n     * @param {string} key\n     */\n    key_up(key) {\n        const ptr0 = passStringToWasm0(key, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n        const len0 = WASM_VECTOR_LEN;\n        wasm.game_key_up(this.__wbg_ptr, ptr0, len0);\n    }\n    unfocus() {\n        wasm.game_unfocus(this.__wbg_ptr);\n    }\n    process_keys() {\n        wasm.game_process_keys(this.__wbg_ptr);\n    }\n    /**\n     * @param {number} x\n     * @param {number} y\n     */\n    click(x, y) {\n        wasm.game_click(this.__wbg_ptr, x, y);\n    }\n    /**\n     * @returns {number}\n     */\n    width() {\n        const ret = wasm.game_width(this.__wbg_ptr);\n        return ret >>> 0;\n    }\n    /**\n     * @returns {number}\n     */\n    height() {\n        const ret = wasm.game_height(this.__wbg_ptr);\n        return ret >>> 0;\n    }\n    /**\n     * @returns {Stats}\n     */\n    stats() {\n        const ret = wasm.game_stats(this.__wbg_ptr);\n        return Stats.__wrap(ret);\n    }\n}\n\nconst StatsFinalization = (typeof FinalizationRegistry === 'undefined')\n    ? { register: () => {}, unregister: () => {} }\n    : new FinalizationRegistry(ptr => wasm.__wbg_stats_free(ptr >>> 0, 1));\n\nclass Stats {\n\n    static __wrap(ptr) {\n        ptr = ptr >>> 0;\n        const obj = Object.create(Stats.prototype);\n        obj.__wbg_ptr = ptr;\n        StatsFinalization.register(obj, obj.__wbg_ptr, obj);\n        return obj;\n    }\n\n    __destroy_into_raw() {\n        const ptr = this.__wbg_ptr;\n        this.__wbg_ptr = 0;\n        StatsFinalization.unregister(this);\n        return ptr;\n    }\n\n    free() {\n        const ptr = this.__destroy_into_raw();\n        wasm.__wbg_stats_free(ptr, 0);\n    }\n    /**\n     * @returns {Stats}\n     */\n    get_and_reset() {\n        const ret = wasm.stats_get_and_reset(this.__wbg_ptr);\n        return Stats.__wrap(ret);\n    }\n    /**\n     * @returns {Stats}\n     */\n    static zero() {\n        const ret = wasm.stats_zero();\n        return Stats.__wrap(ret);\n    }\n    /**\n     * @returns {number}\n     */\n    ticks() {\n        const ret = wasm.stats_ticks(this.__wbg_ptr);\n        return ret >>> 0;\n    }\n    /**\n     * @returns {number}\n     */\n    cells_count() {\n        const ret = wasm.stats_cells_count(this.__wbg_ptr);\n        return ret >>> 0;\n    }\n    /**\n     * @returns {number}\n     */\n    collisions_count() {\n        const ret = wasm.stats_collisions_count(this.__wbg_ptr);\n        return ret >>> 0;\n    }\n    /**\n     * @returns {number}\n     */\n    collision_pairs_tested() {\n        const ret = wasm.stats_collision_pairs_tested(this.__wbg_ptr);\n        return ret >>> 0;\n    }\n}\n\nfunction __wbg_new_abda76e883ba8a5f() {\n    const ret = new Error();\n    return ret;\n};\n\nfunction __wbg_stack_658279fe44541cf6(arg0, arg1) {\n    const ret = arg1.stack;\n    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);\n    const len1 = WASM_VECTOR_LEN;\n    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);\n    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);\n};\n\nfunction __wbg_error_f851667af71bcfc6(arg0, arg1) {\n    let deferred0_0;\n    let deferred0_1;\n    try {\n        deferred0_0 = arg0;\n        deferred0_1 = arg1;\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);\n    }\n};\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\nfunction __wbindgen_init_externref_table() {\n    const table = wasm.__wbindgen_export_0;\n    const offset = table.grow(4);\n    table.set(0, undefined);\n    table.set(offset + 0, undefined);\n    table.set(offset + 1, null);\n    table.set(offset + 2, true);\n    table.set(offset + 3, false);\n    ;\n};\n\n\n\n//# sourceURL=webpack://create-wasm-app/./node_modules/rockies/rockies_bg.js?");

/***/ }),

/***/ "./node_modules/rockies/rockies_bg.wasm":
/*!**********************************************!*\
  !*** ./node_modules/rockies/rockies_bg.wasm ***!
  \**********************************************/
/***/ ((module, exports, __webpack_require__) => {

eval("/* harmony import */ var WEBPACK_IMPORTED_MODULE_0 = __webpack_require__(/*! ./rockies_bg.js */ \"./node_modules/rockies/rockies_bg.js\");\nmodule.exports = __webpack_require__.v(exports, module.id, \"0716e0476e7106cf0621\", {\n\t\"./rockies_bg.js\": {\n\t\t\"__wbg_new_abda76e883ba8a5f\": WEBPACK_IMPORTED_MODULE_0.__wbg_new_abda76e883ba8a5f,\n\t\t\"__wbg_stack_658279fe44541cf6\": WEBPACK_IMPORTED_MODULE_0.__wbg_stack_658279fe44541cf6,\n\t\t\"__wbg_error_f851667af71bcfc6\": WEBPACK_IMPORTED_MODULE_0.__wbg_error_f851667af71bcfc6,\n\t\t\"__wbindgen_throw\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_throw,\n\t\t\"__wbindgen_init_externref_table\": WEBPACK_IMPORTED_MODULE_0.__wbindgen_init_externref_table\n\t}\n});\n\n//# sourceURL=webpack://create-wasm-app/./node_modules/rockies/rockies_bg.wasm?");

/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			id: moduleId,
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/async module */
/******/ 	(() => {
/******/ 		var webpackQueues = typeof Symbol === "function" ? Symbol("webpack queues") : "__webpack_queues__";
/******/ 		var webpackExports = typeof Symbol === "function" ? Symbol("webpack exports") : "__webpack_exports__";
/******/ 		var webpackError = typeof Symbol === "function" ? Symbol("webpack error") : "__webpack_error__";
/******/ 		var resolveQueue = (queue) => {
/******/ 			if(queue && queue.d < 1) {
/******/ 				queue.d = 1;
/******/ 				queue.forEach((fn) => (fn.r--));
/******/ 				queue.forEach((fn) => (fn.r-- ? fn.r++ : fn()));
/******/ 			}
/******/ 		}
/******/ 		var wrapDeps = (deps) => (deps.map((dep) => {
/******/ 			if(dep !== null && typeof dep === "object") {
/******/ 				if(dep[webpackQueues]) return dep;
/******/ 				if(dep.then) {
/******/ 					var queue = [];
/******/ 					queue.d = 0;
/******/ 					dep.then((r) => {
/******/ 						obj[webpackExports] = r;
/******/ 						resolveQueue(queue);
/******/ 					}, (e) => {
/******/ 						obj[webpackError] = e;
/******/ 						resolveQueue(queue);
/******/ 					});
/******/ 					var obj = {};
/******/ 					obj[webpackQueues] = (fn) => (fn(queue));
/******/ 					return obj;
/******/ 				}
/******/ 			}
/******/ 			var ret = {};
/******/ 			ret[webpackQueues] = x => {};
/******/ 			ret[webpackExports] = dep;
/******/ 			return ret;
/******/ 		}));
/******/ 		__webpack_require__.a = (module, body, hasAwait) => {
/******/ 			var queue;
/******/ 			hasAwait && ((queue = []).d = -1);
/******/ 			var depQueues = new Set();
/******/ 			var exports = module.exports;
/******/ 			var currentDeps;
/******/ 			var outerResolve;
/******/ 			var reject;
/******/ 			var promise = new Promise((resolve, rej) => {
/******/ 				reject = rej;
/******/ 				outerResolve = resolve;
/******/ 			});
/******/ 			promise[webpackExports] = exports;
/******/ 			promise[webpackQueues] = (fn) => (queue && fn(queue), depQueues.forEach(fn), promise["catch"](x => {}));
/******/ 			module.exports = promise;
/******/ 			body((deps) => {
/******/ 				currentDeps = wrapDeps(deps);
/******/ 				var fn;
/******/ 				var getResult = () => (currentDeps.map((d) => {
/******/ 					if(d[webpackError]) throw d[webpackError];
/******/ 					return d[webpackExports];
/******/ 				}))
/******/ 				var promise = new Promise((resolve) => {
/******/ 					fn = () => (resolve(getResult));
/******/ 					fn.r = 0;
/******/ 					var fnQueue = (q) => (q !== queue && !depQueues.has(q) && (depQueues.add(q), q && !q.d && (fn.r++, q.push(fn))));
/******/ 					currentDeps.map((dep) => (dep[webpackQueues](fnQueue)));
/******/ 				});
/******/ 				return fn.r ? promise : getResult();
/******/ 			}, (err) => ((err ? reject(promise[webpackError] = err) : outerResolve(exports)), resolveQueue(queue)));
/******/ 			queue && queue.d < 0 && (queue.d = 0);
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/global */
/******/ 	(() => {
/******/ 		__webpack_require__.g = (function() {
/******/ 			if (typeof globalThis === 'object') return globalThis;
/******/ 			try {
/******/ 				return this || new Function('return this')();
/******/ 			} catch (e) {
/******/ 				if (typeof window === 'object') return window;
/******/ 			}
/******/ 		})();
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/wasm loading */
/******/ 	(() => {
/******/ 		__webpack_require__.v = (exports, wasmModuleId, wasmModuleHash, importsObj) => {
/******/ 		
/******/ 			var req = fetch(__webpack_require__.p + "" + wasmModuleHash + ".module.wasm");
/******/ 			var fallback = () => (req
/******/ 				.then((x) => (x.arrayBuffer()))
/******/ 				.then((bytes) => (WebAssembly.instantiate(bytes, importsObj)))
/******/ 				.then((res) => (Object.assign(exports, res.instance.exports))));
/******/ 			return req.then((res) => {
/******/ 				if (typeof WebAssembly.instantiateStreaming === "function") {
/******/ 		
/******/ 					return WebAssembly.instantiateStreaming(res, importsObj)
/******/ 						.then(
/******/ 							(res) => (Object.assign(exports, res.instance.exports)),
/******/ 							(e) => {
/******/ 								if(res.headers.get("Content-Type") !== "application/wasm") {
/******/ 									console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
/******/ 									return fallback();
/******/ 								}
/******/ 								throw e;
/******/ 							}
/******/ 						);
/******/ 				}
/******/ 				return fallback();
/******/ 			});
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/publicPath */
/******/ 	(() => {
/******/ 		var scriptUrl;
/******/ 		if (__webpack_require__.g.importScripts) scriptUrl = __webpack_require__.g.location + "";
/******/ 		var document = __webpack_require__.g.document;
/******/ 		if (!scriptUrl && document) {
/******/ 			if (document.currentScript && document.currentScript.tagName.toUpperCase() === 'SCRIPT')
/******/ 				scriptUrl = document.currentScript.src;
/******/ 			if (!scriptUrl) {
/******/ 				var scripts = document.getElementsByTagName("script");
/******/ 				if(scripts.length) {
/******/ 					var i = scripts.length - 1;
/******/ 					while (i > -1 && (!scriptUrl || !/^http(s?):/.test(scriptUrl))) scriptUrl = scripts[i--].src;
/******/ 				}
/******/ 			}
/******/ 		}
/******/ 		// When supporting browsers where an automatic publicPath is not supported you must specify an output.publicPath manually via configuration
/******/ 		// or pass an empty string ("") and set the __webpack_public_path__ variable from your code to use your own logic.
/******/ 		if (!scriptUrl) throw new Error("Automatic publicPath is not supported in this browser");
/******/ 		scriptUrl = scriptUrl.replace(/^blob:/, "").replace(/#.*$/, "").replace(/\?.*$/, "").replace(/\/[^\/]+$/, "/");
/******/ 		__webpack_require__.p = scriptUrl;
/******/ 	})();
/******/ 	
/************************************************************************/
/******/ 	
/******/ 	// startup
/******/ 	// Load entry module and return exports
/******/ 	// This entry module can't be inlined because the eval devtool is used.
/******/ 	var __webpack_exports__ = __webpack_require__("./index.js");
/******/ 	
/******/ })()
;