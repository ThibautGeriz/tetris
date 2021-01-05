(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _pkg__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./pkg */ \"./pkg/wasm_tetris.js\");\n/* harmony import */ var _pkg_wasm_tetris_bg__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./pkg/wasm_tetris_bg */ \"./pkg/wasm_tetris_bg.wasm\");\n\n\n\nconst colors = {\n  [_pkg__WEBPACK_IMPORTED_MODULE_0__[\"Color\"].None]: '#FFFFFF',\n  [_pkg__WEBPACK_IMPORTED_MODULE_0__[\"Color\"].Cyan]: '#00FFFF',\n  [_pkg__WEBPACK_IMPORTED_MODULE_0__[\"Color\"].Yellow]: '#FFFF00',\n  [_pkg__WEBPACK_IMPORTED_MODULE_0__[\"Color\"].Purple]: '#800080',\n  [_pkg__WEBPACK_IMPORTED_MODULE_0__[\"Color\"].Green]: '#008000',\n  [_pkg__WEBPACK_IMPORTED_MODULE_0__[\"Color\"].Red]: '#FF0000',\n  [_pkg__WEBPACK_IMPORTED_MODULE_0__[\"Color\"].Blue]: '#0000FF',\n  [_pkg__WEBPACK_IMPORTED_MODULE_0__[\"Color\"].Orange]: '#FFA500',\n};\n\n// // Construct the universe, and get its width and height.\nconst game = _pkg__WEBPACK_IMPORTED_MODULE_0__[\"TetrisGame\"].new();\nconst columnCount = game.column_count();\nconst rowCount = game.row_count();\nlet squareSize = getSquareSize();\nconst canvas = document.getElementById('tetris-playground');\nconst context = canvas.getContext('2d');\n\nconst fps = new (class {\n  constructor() {\n    this.fps = document.getElementById('fps');\n    this.frames = [];\n    this.lastFrameTimeStamp = performance.now();\n  }\n\n  render() {\n    // Convert the delta time since the last frame render into a measure\n    // of frames per second.\n    const now = performance.now();\n    const delta = now - this.lastFrameTimeStamp;\n    this.lastFrameTimeStamp = now;\n    const fps = (1 / delta) * 1000;\n\n    // Save only the latest 100 timings.\n    this.frames.push(fps);\n    if (this.frames.length > 100) {\n      this.frames.shift();\n    }\n\n    // Find the max, min, and mean of our 100 latest timings.\n    let min = Infinity;\n    let max = -Infinity;\n    let sum = 0;\n    for (let i = 0; i < this.frames.length; i++) {\n      sum += this.frames[i];\n      min = Math.min(this.frames[i], min);\n      max = Math.max(this.frames[i], max);\n    }\n    let mean = sum / this.frames.length;\n\n    // Render the statistics.\n    this.fps.textContent = `\n         latest = ${Math.round(fps)}\navg of last 100 = ${Math.round(mean)}\nmin of last 100 = ${Math.round(min)}\nmax of last 100 = ${Math.round(max)}\n`.trim();\n  }\n})();\n\n// resize the canvas to fill browser window dynamically\nwindow.addEventListener('resize', resizeCanvas, false);\n\nfunction resizeCanvas() {\n  squareSize = getSquareSize();\n  canvas.width = columnCount * (squareSize + 1);\n  canvas.height = rowCount * (squareSize + 1);\n}\nresizeCanvas();\n\nfunction getSquareSize() {\n  return Math.floor((window.innerHeight - 100) / rowCount);\n}\n\nfunction draw() {\n  const squaresPtr = game.squares();\n  const squares = new Uint8Array(_pkg_wasm_tetris_bg__WEBPACK_IMPORTED_MODULE_1__[\"memory\"].buffer, squaresPtr, rowCount * columnCount);\n  context.beginPath();\n  for (let row = 0; row < rowCount; row++) {\n    for (let col = 0; col < columnCount; col++) {\n      const idx = getIndex(row, col);\n      context.fillStyle = colors[squares[idx]];\n      context.fillRect(\n        col * (squareSize + 1) + 1,\n        row * (squareSize + 1) + 1,\n        squareSize,\n        squareSize,\n      );\n    }\n  }\n\n  context.stroke();\n}\n\nfunction setScore() {\n  const score = game.score();\n  const scoreTag = document.getElementById('tetris-score');\n  scoreTag.textContent = score;\n}\n\nconst renderLoop = () => {\n  fps.render();\n  draw();\n  setScore();\n  if (game.is_game_over()) {\n    alert('game over !');\n    return;\n  }\n  requestAnimationFrame(renderLoop);\n};\nrenderLoop();\n\nsetInterval(() => {\n  game.tick();\n}, 300);\n\ndocument.addEventListener('keydown', ({ key }) => {\n  if (key === 'ArrowLeft' || key == 'a') {\n    game.go_left();\n  }\n  if (key === 'ArrowRight' || key == 'd') {\n    game.go_right();\n  }\n  if (key === 'ArrowDown' || key == 's') {\n    game.go_bottom();\n  }\n  if (key == 'm') {\n    toggleAudio();\n  }\n});\n\nfunction getIndex(row, column) {\n  return row * columnCount + column;\n}\n\nconst url = 'https://ia600504.us.archive.org/33/items/TetrisThemeMusic/Tetris.mp3';\nconst audio = new Audio(url);\nlet isPlaying = false;\n\nfunction toggleAudio() {\n  if (isPlaying) {\n    pause();\n  } else {\n    play();\n  }\n}\n\nfunction play() {\n  audio.addEventListener('ended', function () {\n    if (isPlaying) {\n      audio.play();\n    }\n  });\n  audio.play().then(() => {\n    isPlaying = true;\n  });\n}\n\nfunction pause() {\n  isPlaying = false;\n  audio.pause();\n}\n\nplay();\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ }),

/***/ "./pkg/wasm_tetris.js":
/*!****************************!*\
  !*** ./pkg/wasm_tetris.js ***!
  \****************************/
/*! exports provided: Color, TetrisGame, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbindgen_object_drop_ref, __wbg_randomFillSync_d2ba53160aec6aba, __wbg_getRandomValues_e57c9b75ddead065, __wbg_self_86b4b13392c7af56, __wbg_require_f5521a5b85ad2542, __wbg_crypto_b8c92eaac23d0d80, __wbg_msCrypto_9ad6677321a08dd8, __wbindgen_is_undefined, __wbg_getRandomValues_dd27e6b0652b3236, __wbg_static_accessor_MODULE_452b4680e8614c81, __wbg_buffer_3f12a1c608c6d04e, __wbg_new_c6c0228e6d22a2f9, __wbg_set_b91afac9fd216d99, __wbg_length_c645e7c02233b440, __wbg_newwithlength_a429e08f8a8fe4b3, __wbg_subarray_02e2fcfa6b285cb2, __wbindgen_throw, __wbindgen_memory */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_tetris_bg.wasm */ \"./pkg/wasm_tetris_bg.wasm\");\n/* harmony import */ var _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./wasm_tetris_bg.js */ \"./pkg/wasm_tetris_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"Color\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"Color\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"TetrisGame\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"TetrisGame\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_59cb74e423758ede\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_stack_558ba5917b466edd\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_error_4bb6c2a97407129a\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_d2ba53160aec6aba\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_randomFillSync_d2ba53160aec6aba\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_e57c9b75ddead065\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_getRandomValues_e57c9b75ddead065\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_86b4b13392c7af56\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_self_86b4b13392c7af56\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_f5521a5b85ad2542\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_require_f5521a5b85ad2542\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_b8c92eaac23d0d80\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_crypto_b8c92eaac23d0d80\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_msCrypto_9ad6677321a08dd8\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_msCrypto_9ad6677321a08dd8\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_is_undefined\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_dd27e6b0652b3236\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_getRandomValues_dd27e6b0652b3236\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_static_accessor_MODULE_452b4680e8614c81\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_static_accessor_MODULE_452b4680e8614c81\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_buffer_3f12a1c608c6d04e\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_buffer_3f12a1c608c6d04e\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_c6c0228e6d22a2f9\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_c6c0228e6d22a2f9\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_set_b91afac9fd216d99\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_set_b91afac9fd216d99\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_length_c645e7c02233b440\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_length_c645e7c02233b440\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newwithlength_a429e08f8a8fe4b3\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_newwithlength_a429e08f8a8fe4b3\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_subarray_02e2fcfa6b285cb2\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_subarray_02e2fcfa6b285cb2\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_memory\", function() { return _wasm_tetris_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_memory\"]; });\n\n\n\n\n//# sourceURL=webpack:///./pkg/wasm_tetris.js?");

/***/ }),

/***/ "./pkg/wasm_tetris_bg.js":
/*!*******************************!*\
  !*** ./pkg/wasm_tetris_bg.js ***!
  \*******************************/
/*! exports provided: Color, TetrisGame, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbindgen_object_drop_ref, __wbg_randomFillSync_d2ba53160aec6aba, __wbg_getRandomValues_e57c9b75ddead065, __wbg_self_86b4b13392c7af56, __wbg_require_f5521a5b85ad2542, __wbg_crypto_b8c92eaac23d0d80, __wbg_msCrypto_9ad6677321a08dd8, __wbindgen_is_undefined, __wbg_getRandomValues_dd27e6b0652b3236, __wbg_static_accessor_MODULE_452b4680e8614c81, __wbg_buffer_3f12a1c608c6d04e, __wbg_new_c6c0228e6d22a2f9, __wbg_set_b91afac9fd216d99, __wbg_length_c645e7c02233b440, __wbg_newwithlength_a429e08f8a8fe4b3, __wbg_subarray_02e2fcfa6b285cb2, __wbindgen_throw, __wbindgen_memory */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"Color\", function() { return Color; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"TetrisGame\", function() { return TetrisGame; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return __wbg_new_59cb74e423758ede; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return __wbg_stack_558ba5917b466edd; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return __wbg_error_4bb6c2a97407129a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_randomFillSync_d2ba53160aec6aba\", function() { return __wbg_randomFillSync_d2ba53160aec6aba; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_e57c9b75ddead065\", function() { return __wbg_getRandomValues_e57c9b75ddead065; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_86b4b13392c7af56\", function() { return __wbg_self_86b4b13392c7af56; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_require_f5521a5b85ad2542\", function() { return __wbg_require_f5521a5b85ad2542; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_crypto_b8c92eaac23d0d80\", function() { return __wbg_crypto_b8c92eaac23d0d80; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_msCrypto_9ad6677321a08dd8\", function() { return __wbg_msCrypto_9ad6677321a08dd8; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_getRandomValues_dd27e6b0652b3236\", function() { return __wbg_getRandomValues_dd27e6b0652b3236; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_static_accessor_MODULE_452b4680e8614c81\", function() { return __wbg_static_accessor_MODULE_452b4680e8614c81; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_buffer_3f12a1c608c6d04e\", function() { return __wbg_buffer_3f12a1c608c6d04e; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_c6c0228e6d22a2f9\", function() { return __wbg_new_c6c0228e6d22a2f9; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_set_b91afac9fd216d99\", function() { return __wbg_set_b91afac9fd216d99; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_length_c645e7c02233b440\", function() { return __wbg_length_c645e7c02233b440; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newwithlength_a429e08f8a8fe4b3\", function() { return __wbg_newwithlength_a429e08f8a8fe4b3; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_subarray_02e2fcfa6b285cb2\", function() { return __wbg_subarray_02e2fcfa6b285cb2; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_memory\", function() { return __wbindgen_memory; });\n/* harmony import */ var _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./wasm_tetris_bg.wasm */ \"./pkg/wasm_tetris_bg.wasm\");\n\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nfunction handleError(f) {\n    return function () {\n        try {\n            return f.apply(this, arguments);\n\n        } catch (e) {\n            _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_exn_store\"](addHeapObject(e));\n        }\n    };\n}\n\nfunction getArrayU8FromWasm0(ptr, len) {\n    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);\n}\n/**\n*/\nconst Color = Object.freeze({ None:0,\"0\":\"None\",Cyan:1,\"1\":\"Cyan\",Yellow:2,\"2\":\"Yellow\",Purple:3,\"3\":\"Purple\",Green:4,\"4\":\"Green\",Red:5,\"5\":\"Red\",Blue:6,\"6\":\"Blue\",Orange:7,\"7\":\"Orange\", });\n/**\n*/\nclass TetrisGame {\n\n    static __wrap(ptr) {\n        const obj = Object.create(TetrisGame.prototype);\n        obj.ptr = ptr;\n\n        return obj;\n    }\n\n    free() {\n        const ptr = this.ptr;\n        this.ptr = 0;\n\n        _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbg_tetrisgame_free\"](ptr);\n    }\n    /**\n    */\n    tick() {\n        _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"tetrisgame_tick\"](this.ptr);\n    }\n    /**\n    * @returns {TetrisGame}\n    */\n    static new() {\n        var ret = _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"tetrisgame_new\"]();\n        return TetrisGame.__wrap(ret);\n    }\n    /**\n    * @returns {number}\n    */\n    squares() {\n        var ret = _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"tetrisgame_squares\"](this.ptr);\n        return ret;\n    }\n    /**\n    * @returns {number}\n    */\n    column_count() {\n        var ret = _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"tetrisgame_column_count\"](this.ptr);\n        return ret >>> 0;\n    }\n    /**\n    * @returns {number}\n    */\n    row_count() {\n        var ret = _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"tetrisgame_row_count\"](this.ptr);\n        return ret >>> 0;\n    }\n    /**\n    * @returns {number}\n    */\n    score() {\n        var ret = _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"tetrisgame_score\"](this.ptr);\n        return ret >>> 0;\n    }\n    /**\n    * @returns {boolean}\n    */\n    is_game_over() {\n        var ret = _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"tetrisgame_is_game_over\"](this.ptr);\n        return ret !== 0;\n    }\n    /**\n    * @returns {number}\n    */\n    level() {\n        var ret = _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"tetrisgame_level\"](this.ptr);\n        return ret >>> 0;\n    }\n    /**\n    */\n    go_right() {\n        _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"tetrisgame_go_right\"](this.ptr);\n    }\n    /**\n    */\n    go_left() {\n        _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"tetrisgame_go_left\"](this.ptr);\n    }\n    /**\n    */\n    go_bottom() {\n        _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"tetrisgame_go_bottom\"](this.ptr);\n    }\n}\n\nconst __wbg_new_59cb74e423758ede = function() {\n    var ret = new Error();\n    return addHeapObject(ret);\n};\n\nconst __wbg_stack_558ba5917b466edd = function(arg0, arg1) {\n    var ret = getObject(arg1).stack;\n    var ptr0 = passStringToWasm0(ret, _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nconst __wbg_error_4bb6c2a97407129a = function(arg0, arg1) {\n    try {\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](arg0, arg1);\n    }\n};\n\nconst __wbindgen_object_drop_ref = function(arg0) {\n    takeObject(arg0);\n};\n\nconst __wbg_randomFillSync_d2ba53160aec6aba = function(arg0, arg1, arg2) {\n    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));\n};\n\nconst __wbg_getRandomValues_e57c9b75ddead065 = function(arg0, arg1) {\n    getObject(arg0).getRandomValues(getObject(arg1));\n};\n\nconst __wbg_self_86b4b13392c7af56 = handleError(function() {\n    var ret = self.self;\n    return addHeapObject(ret);\n});\n\nconst __wbg_require_f5521a5b85ad2542 = function(arg0, arg1, arg2) {\n    var ret = getObject(arg0).require(getStringFromWasm0(arg1, arg2));\n    return addHeapObject(ret);\n};\n\nconst __wbg_crypto_b8c92eaac23d0d80 = function(arg0) {\n    var ret = getObject(arg0).crypto;\n    return addHeapObject(ret);\n};\n\nconst __wbg_msCrypto_9ad6677321a08dd8 = function(arg0) {\n    var ret = getObject(arg0).msCrypto;\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_is_undefined = function(arg0) {\n    var ret = getObject(arg0) === undefined;\n    return ret;\n};\n\nconst __wbg_getRandomValues_dd27e6b0652b3236 = function(arg0) {\n    var ret = getObject(arg0).getRandomValues;\n    return addHeapObject(ret);\n};\n\nconst __wbg_static_accessor_MODULE_452b4680e8614c81 = function() {\n    var ret = module;\n    return addHeapObject(ret);\n};\n\nconst __wbg_buffer_3f12a1c608c6d04e = function(arg0) {\n    var ret = getObject(arg0).buffer;\n    return addHeapObject(ret);\n};\n\nconst __wbg_new_c6c0228e6d22a2f9 = function(arg0) {\n    var ret = new Uint8Array(getObject(arg0));\n    return addHeapObject(ret);\n};\n\nconst __wbg_set_b91afac9fd216d99 = function(arg0, arg1, arg2) {\n    getObject(arg0).set(getObject(arg1), arg2 >>> 0);\n};\n\nconst __wbg_length_c645e7c02233b440 = function(arg0) {\n    var ret = getObject(arg0).length;\n    return ret;\n};\n\nconst __wbg_newwithlength_a429e08f8a8fe4b3 = function(arg0) {\n    var ret = new Uint8Array(arg0 >>> 0);\n    return addHeapObject(ret);\n};\n\nconst __wbg_subarray_02e2fcfa6b285cb2 = function(arg0, arg1, arg2) {\n    var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);\n    return addHeapObject(ret);\n};\n\nconst __wbindgen_throw = function(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\nconst __wbindgen_memory = function() {\n    var ret = _wasm_tetris_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"];\n    return addHeapObject(ret);\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///./pkg/wasm_tetris_bg.js?");

/***/ }),

/***/ "./pkg/wasm_tetris_bg.wasm":
/*!*********************************!*\
  !*** ./pkg/wasm_tetris_bg.wasm ***!
  \*********************************/
/*! exports provided: memory, __wbg_tetrisgame_free, tetrisgame_tick, tetrisgame_new, tetrisgame_squares, tetrisgame_column_count, tetrisgame_row_count, tetrisgame_score, tetrisgame_is_game_over, tetrisgame_level, tetrisgame_go_right, tetrisgame_go_left, tetrisgame_go_bottom, __wbindgen_free, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_exn_store */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./wasm_tetris_bg.js */ \"./pkg/wasm_tetris_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///./pkg/wasm_tetris_bg.wasm?");

/***/ })

}]);