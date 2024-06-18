"use strict";
/*
 * ATTENTION: The "eval" devtool has been used (maybe by default in mode: "development").
 * This devtool is neither made for production nor for readable output files.
 * It uses "eval()" calls to create a separate source file in the browser devtools.
 * If you are trying to read the output file, select a different devtool (https://webpack.js.org/configuration/devtool/)
 * or disable the default devtool with "devtool: false".
 * If you are looking for production-ready output files, see mode: "production" (https://webpack.js.org/configuration/mode/).
 */
(self["webpackChunkcreate_wasm_app"] = self["webpackChunkcreate_wasm_app"] || []).push([["index_js"],{

/***/ "../pkg/rockies.js":
/*!*************************!*\
  !*** ../pkg/rockies.js ***!
  \*************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   __wbg_alert_2ee703420fead299: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_alert_2ee703420fead299),\n/* harmony export */   __wbg_set_wasm: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm),\n/* harmony export */   greet: () => (/* reexport safe */ _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.greet)\n/* harmony export */ });\n/* harmony import */ var _rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./rockies_bg.wasm */ \"../pkg/rockies_bg.wasm\");\n/* harmony import */ var _rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./rockies_bg.js */ \"../pkg/rockies_bg.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__]);\n_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\n(0,_rockies_bg_js__WEBPACK_IMPORTED_MODULE_0__.__wbg_set_wasm)(_rockies_bg_wasm__WEBPACK_IMPORTED_MODULE_1__);\n\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://create-wasm-app/../pkg/rockies.js?");

/***/ }),

/***/ "../pkg/rockies_bg.js":
/*!****************************!*\
  !*** ../pkg/rockies_bg.js ***!
  \****************************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export */ __webpack_require__.d(__webpack_exports__, {\n/* harmony export */   __wbg_alert_2ee703420fead299: () => (/* binding */ __wbg_alert_2ee703420fead299),\n/* harmony export */   __wbg_set_wasm: () => (/* binding */ __wbg_set_wasm),\n/* harmony export */   greet: () => (/* binding */ greet)\n/* harmony export */ });\n/* module decorator */ module = __webpack_require__.hmd(module);\nlet wasm;\nfunction __wbg_set_wasm(val) {\n    wasm = val;\n}\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachedUint8Memory0 = null;\n\nfunction getUint8Memory0() {\n    if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {\n        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);\n    }\n    return cachedUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    ptr = ptr >>> 0;\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n*/\nfunction greet() {\n    wasm.greet();\n}\n\nfunction __wbg_alert_2ee703420fead299(arg0, arg1) {\n    alert(getStringFromWasm0(arg0, arg1));\n};\n\n\n\n//# sourceURL=webpack://create-wasm-app/../pkg/rockies_bg.js?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/***/ ((module, __webpack_exports__, __webpack_require__) => {

eval("__webpack_require__.a(module, async (__webpack_handle_async_dependencies__, __webpack_async_result__) => { try {\n__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var rockies__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! rockies */ \"../pkg/rockies.js\");\nvar __webpack_async_dependencies__ = __webpack_handle_async_dependencies__([rockies__WEBPACK_IMPORTED_MODULE_0__]);\nrockies__WEBPACK_IMPORTED_MODULE_0__ = (__webpack_async_dependencies__.then ? (await __webpack_async_dependencies__)() : __webpack_async_dependencies__)[0];\n\n\nrockies__WEBPACK_IMPORTED_MODULE_0__.greet();\n\n__webpack_async_result__();\n} catch(e) { __webpack_async_result__(e); } });\n\n//# sourceURL=webpack://create-wasm-app/./index.js?");

/***/ }),

/***/ "../pkg/rockies_bg.wasm":
/*!******************************!*\
  !*** ../pkg/rockies_bg.wasm ***!
  \******************************/
/***/ ((module, exports, __webpack_require__) => {

eval("/* harmony import */ var WEBPACK_IMPORTED_MODULE_0 = __webpack_require__(/*! ./rockies_bg.js */ \"../pkg/rockies_bg.js\");\nmodule.exports = __webpack_require__.v(exports, module.id, \"96e735cfd55b7efab554\", {\n\t\"./rockies_bg.js\": {\n\t\t\"__wbg_alert_2ee703420fead299\": WEBPACK_IMPORTED_MODULE_0.__wbg_alert_2ee703420fead299\n\t}\n});\n\n//# sourceURL=webpack://create-wasm-app/../pkg/rockies_bg.wasm?");

/***/ })

}]);