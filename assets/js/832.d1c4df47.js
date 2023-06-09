"use strict";
exports.id = 832;
exports.ids = [832];
exports.modules = {

/***/ 4832:
/***/ ((__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (/* binding */ ua),
/* harmony export */   uriTransformer: () => (/* binding */ Nr)
/* harmony export */ });
/* harmony import */ var react__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(7294);
/* harmony import */ var _index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(3220);
/* harmony import */ var react_dom__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(2834);



const Wn = ["http", "https", "mailto", "tel"];
function Nr(e) {
  const n = (e || "").trim(), t = n.charAt(0);
  if (t === "#" || t === "/")
    return n;
  const r = n.indexOf(":");
  if (r === -1)
    return n;
  let i = -1;
  for (; ++i < Wn.length; ) {
    const l = Wn[i];
    if (r === l.length && n.slice(0, l.length).toLowerCase() === l)
      return n;
  }
  return i = n.indexOf("?"), i !== -1 && r > i || (i = n.indexOf("#"), i !== -1 && r > i) ? n : "javascript:void(0)";
}
/*!
 * Determine if an object is a Buffer
 *
 * @author   Feross Aboukhadijeh <https://feross.org>
 * @license  MIT
 */
var jr = function(n) {
  return n != null && n.constructor != null && typeof n.constructor.isBuffer == "function" && n.constructor.isBuffer(n);
};
const Xt = /* @__PURE__ */ (0,_index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.g)(jr);
function $r(e) {
  return !e || typeof e != "object" ? "" : "position" in e || "type" in e ? Xn(e.position) : "start" in e || "end" in e ? Xn(e) : "line" in e || "column" in e ? En(e) : "";
}
function En(e) {
  return Qn(e && e.line) + ":" + Qn(e && e.column);
}
function Xn(e) {
  return En(e && e.start) + "-" + En(e && e.end);
}
function Qn(e) {
  return e && typeof e == "number" ? e : 1;
}
class we extends Error {
  /**
   * Create a message for `reason` at `place` from `origin`.
   *
   * When an error is passed in as `reason`, the `stack` is copied.
   *
   * @param {string | Error | VFileMessage} reason
   *   Reason for message, uses the stack and message of the error if given.
   *
   *   > 👉 **Note**: you should use markdown.
   * @param {Node | NodeLike | Position | Point | null | undefined} [place]
   *   Place in file where the message occurred.
   * @param {string | null | undefined} [origin]
   *   Place in code where the message originates (example:
   *   `'my-package:my-rule'` or `'my-rule'`).
   * @returns
   *   Instance of `VFileMessage`.
   */
  // To do: next major: expose `undefined` everywhere instead of `null`.
  constructor(n, t, r) {
    const i = [null, null];
    let l = {
      // @ts-expect-error: we always follows the structure of `position`.
      start: { line: null, column: null },
      // @ts-expect-error: "
      end: { line: null, column: null }
    };
    if (super(), typeof t == "string" && (r = t, t = void 0), typeof r == "string") {
      const o = r.indexOf(":");
      o === -1 ? i[1] = r : (i[0] = r.slice(0, o), i[1] = r.slice(o + 1));
    }
    t && ("type" in t || "position" in t ? t.position && (l = t.position) : "start" in t || "end" in t ? l = t : ("line" in t || "column" in t) && (l.start = t)), this.name = $r(t) || "1:1", this.message = typeof n == "object" ? n.message : n, this.stack = "", typeof n == "object" && n.stack && (this.stack = n.stack), this.reason = this.message, this.fatal, this.line = l.start.line, this.column = l.start.column, this.position = l, this.source = i[0], this.ruleId = i[1], this.file, this.actual, this.expected, this.url, this.note;
  }
}
we.prototype.file = "";
we.prototype.name = "";
we.prototype.reason = "";
we.prototype.message = "";
we.prototype.stack = "";
we.prototype.fatal = null;
we.prototype.column = null;
we.prototype.line = null;
we.prototype.source = null;
we.prototype.ruleId = null;
we.prototype.position = null;
const Ce = { basename: Ur, dirname: qr, extname: Hr, join: Vr, sep: "/" };
function Ur(e, n) {
  if (n !== void 0 && typeof n != "string")
    throw new TypeError('"ext" argument must be a string');
  We(e);
  let t = 0, r = -1, i = e.length, l;
  if (n === void 0 || n.length === 0 || n.length > e.length) {
    for (; i--; )
      if (e.charCodeAt(i) === 47) {
        if (l) {
          t = i + 1;
          break;
        }
      } else
        r < 0 && (l = !0, r = i + 1);
    return r < 0 ? "" : e.slice(t, r);
  }
  if (n === e)
    return "";
  let o = -1, u = n.length - 1;
  for (; i--; )
    if (e.charCodeAt(i) === 47) {
      if (l) {
        t = i + 1;
        break;
      }
    } else
      o < 0 && (l = !0, o = i + 1), u > -1 && (e.charCodeAt(i) === n.charCodeAt(u--) ? u < 0 && (r = i) : (u = -1, r = o));
  return t === r ? r = o : r < 0 && (r = e.length), e.slice(t, r);
}
function qr(e) {
  if (We(e), e.length === 0)
    return ".";
  let n = -1, t = e.length, r;
  for (; --t; )
    if (e.charCodeAt(t) === 47) {
      if (r) {
        n = t;
        break;
      }
    } else
      r || (r = !0);
  return n < 0 ? e.charCodeAt(0) === 47 ? "/" : "." : n === 1 && e.charCodeAt(0) === 47 ? "//" : e.slice(0, n);
}
function Hr(e) {
  We(e);
  let n = e.length, t = -1, r = 0, i = -1, l = 0, o;
  for (; n--; ) {
    const u = e.charCodeAt(n);
    if (u === 47) {
      if (o) {
        r = n + 1;
        break;
      }
      continue;
    }
    t < 0 && (o = !0, t = n + 1), u === 46 ? i < 0 ? i = n : l !== 1 && (l = 1) : i > -1 && (l = -1);
  }
  return i < 0 || t < 0 || // We saw a non-dot character immediately before the dot.
  l === 0 || // The (right-most) trimmed path component is exactly `..`.
  l === 1 && i === t - 1 && i === r + 1 ? "" : e.slice(i, t);
}
function Vr(...e) {
  let n = -1, t;
  for (; ++n < e.length; )
    We(e[n]), e[n] && (t = t === void 0 ? e[n] : t + "/" + e[n]);
  return t === void 0 ? "." : Yr(t);
}
function Yr(e) {
  We(e);
  const n = e.charCodeAt(0) === 47;
  let t = Wr(e, !n);
  return t.length === 0 && !n && (t = "."), t.length > 0 && e.charCodeAt(e.length - 1) === 47 && (t += "/"), n ? "/" + t : t;
}
function Wr(e, n) {
  let t = "", r = 0, i = -1, l = 0, o = -1, u, a;
  for (; ++o <= e.length; ) {
    if (o < e.length)
      u = e.charCodeAt(o);
    else {
      if (u === 47)
        break;
      u = 47;
    }
    if (u === 47) {
      if (!(i === o - 1 || l === 1))
        if (i !== o - 1 && l === 2) {
          if (t.length < 2 || r !== 2 || t.charCodeAt(t.length - 1) !== 46 || t.charCodeAt(t.length - 2) !== 46) {
            if (t.length > 2) {
              if (a = t.lastIndexOf("/"), a !== t.length - 1) {
                a < 0 ? (t = "", r = 0) : (t = t.slice(0, a), r = t.length - 1 - t.lastIndexOf("/")), i = o, l = 0;
                continue;
              }
            } else if (t.length > 0) {
              t = "", r = 0, i = o, l = 0;
              continue;
            }
          }
          n && (t = t.length > 0 ? t + "/.." : "..", r = 2);
        } else
          t.length > 0 ? t += "/" + e.slice(i + 1, o) : t = e.slice(i + 1, o), r = o - i - 1;
      i = o, l = 0;
    } else
      u === 46 && l > -1 ? l++ : l = -1;
  }
  return t;
}
function We(e) {
  if (typeof e != "string")
    throw new TypeError(
      "Path must be a string. Received " + JSON.stringify(e)
    );
}
const Xr = { cwd: Qr };
function Qr() {
  return "/";
}
function Cn(e) {
  return e !== null && typeof e == "object" && // @ts-expect-error: indexable.
  e.href && // @ts-expect-error: indexable.
  e.origin;
}
function Kr(e) {
  if (typeof e == "string")
    e = new URL(e);
  else if (!Cn(e)) {
    const n = new TypeError(
      'The "path" argument must be of type string or an instance of URL. Received `' + e + "`"
    );
    throw n.code = "ERR_INVALID_ARG_TYPE", n;
  }
  if (e.protocol !== "file:") {
    const n = new TypeError("The URL must be of scheme file");
    throw n.code = "ERR_INVALID_URL_SCHEME", n;
  }
  return Gr(e);
}
function Gr(e) {
  if (e.hostname !== "") {
    const r = new TypeError(
      'File URL host must be "localhost" or empty on darwin'
    );
    throw r.code = "ERR_INVALID_FILE_URL_HOST", r;
  }
  const n = e.pathname;
  let t = -1;
  for (; ++t < n.length; )
    if (n.charCodeAt(t) === 37 && n.charCodeAt(t + 1) === 50) {
      const r = n.charCodeAt(t + 2);
      if (r === 70 || r === 102) {
        const i = new TypeError(
          "File URL path must not include encoded / characters"
        );
        throw i.code = "ERR_INVALID_FILE_URL_PATH", i;
      }
    }
  return decodeURIComponent(n);
}
const un = ["history", "path", "basename", "stem", "extname", "dirname"];
class Qt {
  /**
   * Create a new virtual file.
   *
   * `options` is treated as:
   *
   * *   `string` or `Buffer` — `{value: options}`
   * *   `URL` — `{path: options}`
   * *   `VFile` — shallow copies its data over to the new file
   * *   `object` — all fields are shallow copied over to the new file
   *
   * Path related fields are set in the following order (least specific to
   * most specific): `history`, `path`, `basename`, `stem`, `extname`,
   * `dirname`.
   *
   * You cannot set `dirname` or `extname` without setting either `history`,
   * `path`, `basename`, or `stem` too.
   *
   * @param {Compatible | null | undefined} [value]
   *   File value.
   * @returns
   *   New instance.
   */
  constructor(n) {
    let t;
    n ? typeof n == "string" || Zr(n) ? t = { value: n } : Cn(n) ? t = { path: n } : t = n : t = {}, this.data = {}, this.messages = [], this.history = [], this.cwd = Xr.cwd(), this.value, this.stored, this.result, this.map;
    let r = -1;
    for (; ++r < un.length; ) {
      const l = un[r];
      l in t && t[l] !== void 0 && t[l] !== null && (this[l] = l === "history" ? [...t[l]] : t[l]);
    }
    let i;
    for (i in t)
      un.includes(i) || (this[i] = t[i]);
  }
  /**
   * Get the full path (example: `'~/index.min.js'`).
   *
   * @returns {string}
   */
  get path() {
    return this.history[this.history.length - 1];
  }
  /**
   * Set the full path (example: `'~/index.min.js'`).
   *
   * Cannot be nullified.
   * You can set a file URL (a `URL` object with a `file:` protocol) which will
   * be turned into a path with `url.fileURLToPath`.
   *
   * @param {string | URL} path
   */
  set path(n) {
    Cn(n) && (n = Kr(n)), sn(n, "path"), this.path !== n && this.history.push(n);
  }
  /**
   * Get the parent path (example: `'~'`).
   */
  get dirname() {
    return typeof this.path == "string" ? Ce.dirname(this.path) : void 0;
  }
  /**
   * Set the parent path (example: `'~'`).
   *
   * Cannot be set if there’s no `path` yet.
   */
  set dirname(n) {
    Kn(this.basename, "dirname"), this.path = Ce.join(n || "", this.basename);
  }
  /**
   * Get the basename (including extname) (example: `'index.min.js'`).
   */
  get basename() {
    return typeof this.path == "string" ? Ce.basename(this.path) : void 0;
  }
  /**
   * Set basename (including extname) (`'index.min.js'`).
   *
   * Cannot contain path separators (`'/'` on unix, macOS, and browsers, `'\'`
   * on windows).
   * Cannot be nullified (use `file.path = file.dirname` instead).
   */
  set basename(n) {
    sn(n, "basename"), an(n, "basename"), this.path = Ce.join(this.dirname || "", n);
  }
  /**
   * Get the extname (including dot) (example: `'.js'`).
   */
  get extname() {
    return typeof this.path == "string" ? Ce.extname(this.path) : void 0;
  }
  /**
   * Set the extname (including dot) (example: `'.js'`).
   *
   * Cannot contain path separators (`'/'` on unix, macOS, and browsers, `'\'`
   * on windows).
   * Cannot be set if there’s no `path` yet.
   */
  set extname(n) {
    if (an(n, "extname"), Kn(this.dirname, "extname"), n) {
      if (n.charCodeAt(0) !== 46)
        throw new Error("`extname` must start with `.`");
      if (n.includes(".", 1))
        throw new Error("`extname` cannot contain multiple dots");
    }
    this.path = Ce.join(this.dirname, this.stem + (n || ""));
  }
  /**
   * Get the stem (basename w/o extname) (example: `'index.min'`).
   */
  get stem() {
    return typeof this.path == "string" ? Ce.basename(this.path, this.extname) : void 0;
  }
  /**
   * Set the stem (basename w/o extname) (example: `'index.min'`).
   *
   * Cannot contain path separators (`'/'` on unix, macOS, and browsers, `'\'`
   * on windows).
   * Cannot be nullified (use `file.path = file.dirname` instead).
   */
  set stem(n) {
    sn(n, "stem"), an(n, "stem"), this.path = Ce.join(this.dirname || "", n + (this.extname || ""));
  }
  /**
   * Serialize the file.
   *
   * @param {BufferEncoding | null | undefined} [encoding='utf8']
   *   Character encoding to understand `value` as when it’s a `Buffer`
   *   (default: `'utf8'`).
   * @returns {string}
   *   Serialized file.
   */
  toString(n) {
    return (this.value || "").toString(n || void 0);
  }
  /**
   * Create a warning message associated with the file.
   *
   * Its `fatal` is set to `false` and `file` is set to the current file path.
   * Its added to `file.messages`.
   *
   * @param {string | Error | VFileMessage} reason
   *   Reason for message, uses the stack and message of the error if given.
   * @param {Node | NodeLike | Position | Point | null | undefined} [place]
   *   Place in file where the message occurred.
   * @param {string | null | undefined} [origin]
   *   Place in code where the message originates (example:
   *   `'my-package:my-rule'` or `'my-rule'`).
   * @returns {VFileMessage}
   *   Message.
   */
  message(n, t, r) {
    const i = new we(n, t, r);
    return this.path && (i.name = this.path + ":" + i.name, i.file = this.path), i.fatal = !1, this.messages.push(i), i;
  }
  /**
   * Create an info message associated with the file.
   *
   * Its `fatal` is set to `null` and `file` is set to the current file path.
   * Its added to `file.messages`.
   *
   * @param {string | Error | VFileMessage} reason
   *   Reason for message, uses the stack and message of the error if given.
   * @param {Node | NodeLike | Position | Point | null | undefined} [place]
   *   Place in file where the message occurred.
   * @param {string | null | undefined} [origin]
   *   Place in code where the message originates (example:
   *   `'my-package:my-rule'` or `'my-rule'`).
   * @returns {VFileMessage}
   *   Message.
   */
  info(n, t, r) {
    const i = this.message(n, t, r);
    return i.fatal = null, i;
  }
  /**
   * Create a fatal error associated with the file.
   *
   * Its `fatal` is set to `true` and `file` is set to the current file path.
   * Its added to `file.messages`.
   *
   * > 👉 **Note**: a fatal error means that a file is no longer processable.
   *
   * @param {string | Error | VFileMessage} reason
   *   Reason for message, uses the stack and message of the error if given.
   * @param {Node | NodeLike | Position | Point | null | undefined} [place]
   *   Place in file where the message occurred.
   * @param {string | null | undefined} [origin]
   *   Place in code where the message originates (example:
   *   `'my-package:my-rule'` or `'my-rule'`).
   * @returns {never}
   *   Message.
   * @throws {VFileMessage}
   *   Message.
   */
  fail(n, t, r) {
    const i = this.message(n, t, r);
    throw i.fatal = !0, i;
  }
}
function an(e, n) {
  if (e && e.includes(Ce.sep))
    throw new Error(
      "`" + n + "` cannot be a path: did not expect `" + Ce.sep + "`"
    );
}
function sn(e, n) {
  if (!e)
    throw new Error("`" + n + "` cannot be empty");
}
function Kn(e, n) {
  if (!e)
    throw new Error("Setting `" + n + "` requires `path` to be set too");
}
function Zr(e) {
  return Xt(e);
}
function Gn(e) {
  if (e)
    throw e;
}
var Ze = Object.prototype.hasOwnProperty, Kt = Object.prototype.toString, Zn = Object.defineProperty, Jn = Object.getOwnPropertyDescriptor, et = function(n) {
  return typeof Array.isArray == "function" ? Array.isArray(n) : Kt.call(n) === "[object Array]";
}, nt = function(n) {
  if (!n || Kt.call(n) !== "[object Object]")
    return !1;
  var t = Ze.call(n, "constructor"), r = n.constructor && n.constructor.prototype && Ze.call(n.constructor.prototype, "isPrototypeOf");
  if (n.constructor && !t && !r)
    return !1;
  var i;
  for (i in n)
    ;
  return typeof i > "u" || Ze.call(n, i);
}, tt = function(n, t) {
  Zn && t.name === "__proto__" ? Zn(n, t.name, {
    enumerable: !0,
    configurable: !0,
    value: t.newValue,
    writable: !0
  }) : n[t.name] = t.newValue;
}, rt = function(n, t) {
  if (t === "__proto__")
    if (Ze.call(n, t)) {
      if (Jn)
        return Jn(n, t).value;
    } else
      return;
  return n[t];
}, Jr = function e() {
  var n, t, r, i, l, o, u = arguments[0], a = 1, f = arguments.length, c = !1;
  for (typeof u == "boolean" && (c = u, u = arguments[1] || {}, a = 2), (u == null || typeof u != "object" && typeof u != "function") && (u = {}); a < f; ++a)
    if (n = arguments[a], n != null)
      for (t in n)
        r = rt(u, t), i = rt(n, t), u !== i && (c && i && (nt(i) || (l = et(i))) ? (l ? (l = !1, o = r && et(r) ? r : []) : o = r && nt(r) ? r : {}, tt(u, { name: t, newValue: e(c, o, i) })) : typeof i < "u" && tt(u, { name: t, newValue: i }));
  return u;
};
const it = /* @__PURE__ */ (0,_index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.g)(Jr);
function vn(e) {
  if (typeof e != "object" || e === null)
    return !1;
  const n = Object.getPrototypeOf(e);
  return (n === null || n === Object.prototype || Object.getPrototypeOf(n) === null) && !(Symbol.toStringTag in e) && !(Symbol.iterator in e);
}
function ei() {
  const e = [], n = { run: t, use: r };
  return n;
  function t(...i) {
    let l = -1;
    const o = i.pop();
    if (typeof o != "function")
      throw new TypeError("Expected function as last argument, not " + o);
    u(null, ...i);
    function u(a, ...f) {
      const c = e[++l];
      let h = -1;
      if (a) {
        o(a);
        return;
      }
      for (; ++h < i.length; )
        (f[h] === null || f[h] === void 0) && (f[h] = i[h]);
      i = f, c ? ni(c, u)(...f) : o(null, ...f);
    }
  }
  function r(i) {
    if (typeof i != "function")
      throw new TypeError(
        "Expected `middelware` to be a function, not " + i
      );
    return e.push(i), n;
  }
}
function ni(e, n) {
  let t;
  return r;
  function r(...o) {
    const u = e.length > o.length;
    let a;
    u && o.push(i);
    try {
      a = e.apply(this, o);
    } catch (f) {
      const c = (
        /** @type {Error} */
        f
      );
      if (u && t)
        throw c;
      return i(c);
    }
    u || (a instanceof Promise ? a.then(l, i) : a instanceof Error ? i(a) : l(a));
  }
  function i(o, ...u) {
    t || (t = !0, n(o, ...u));
  }
  function l(o) {
    i(null, o);
  }
}
const ti = Zt().freeze(), Gt = {}.hasOwnProperty;
function Zt() {
  const e = ei(), n = [];
  let t = {}, r, i = -1;
  return l.data = o, l.Parser = void 0, l.Compiler = void 0, l.freeze = u, l.attachers = n, l.use = a, l.parse = f, l.stringify = c, l.run = h, l.runSync = g, l.process = m, l.processSync = d, l;
  function l() {
    const y = Zt();
    let x = -1;
    for (; ++x < n.length; )
      y.use(...n[x]);
    return y.data(it(!0, {}, t)), y;
  }
  function o(y, x) {
    return typeof y == "string" ? arguments.length === 2 ? (pn("data", r), t[y] = x, l) : Gt.call(t, y) && t[y] || null : y ? (pn("data", r), t = y, l) : t;
  }
  function u() {
    if (r)
      return l;
    for (; ++i < n.length; ) {
      const [y, ...x] = n[i];
      if (x[0] === !1)
        continue;
      x[0] === !0 && (x[0] = void 0);
      const b = y.call(l, ...x);
      typeof b == "function" && e.use(b);
    }
    return r = !0, i = Number.POSITIVE_INFINITY, l;
  }
  function a(y, ...x) {
    let b;
    if (pn("use", r), y != null)
      if (typeof y == "function")
        S(y, ...x);
      else if (typeof y == "object")
        Array.isArray(y) ? F(y) : v(y);
      else
        throw new TypeError("Expected usable value, not `" + y + "`");
    return b && (t.settings = Object.assign(t.settings || {}, b)), l;
    function z(k) {
      if (typeof k == "function")
        S(k);
      else if (typeof k == "object")
        if (Array.isArray(k)) {
          const [_, ...U] = k;
          S(_, ...U);
        } else
          v(k);
      else
        throw new TypeError("Expected usable value, not `" + k + "`");
    }
    function v(k) {
      F(k.plugins), k.settings && (b = Object.assign(b || {}, k.settings));
    }
    function F(k) {
      let _ = -1;
      if (k != null)
        if (Array.isArray(k))
          for (; ++_ < k.length; ) {
            const U = k[_];
            z(U);
          }
        else
          throw new TypeError("Expected a list of plugins, not `" + k + "`");
    }
    function S(k, _) {
      let U = -1, Y;
      for (; ++U < n.length; )
        if (n[U][0] === k) {
          Y = n[U];
          break;
        }
      Y ? (vn(Y[1]) && vn(_) && (_ = it(!0, Y[1], _)), Y[1] = _) : n.push([...arguments]);
    }
  }
  function f(y) {
    l.freeze();
    const x = He(y), b = l.Parser;
    return cn("parse", b), lt(b, "parse") ? new b(String(x), x).parse() : b(String(x), x);
  }
  function c(y, x) {
    l.freeze();
    const b = He(x), z = l.Compiler;
    return fn("stringify", z), ot(y), lt(z, "compile") ? new z(y, b).compile() : z(y, b);
  }
  function h(y, x, b) {
    if (ot(y), l.freeze(), !b && typeof x == "function" && (b = x, x = void 0), !b)
      return new Promise(z);
    z(null, b);
    function z(v, F) {
      e.run(y, He(x), S);
      function S(k, _, U) {
        _ = _ || y, k ? F(k) : v ? v(_) : b(null, _, U);
      }
    }
  }
  function g(y, x) {
    let b, z;
    return l.run(y, x, v), ut("runSync", "run", z), b;
    function v(F, S) {
      Gn(F), b = S, z = !0;
    }
  }
  function m(y, x) {
    if (l.freeze(), cn("process", l.Parser), fn("process", l.Compiler), !x)
      return new Promise(b);
    b(null, x);
    function b(z, v) {
      const F = He(y);
      l.run(l.parse(F), F, (k, _, U) => {
        if (k || !_ || !U)
          S(k);
        else {
          const Y = l.stringify(_, U);
          Y == null || (li(Y) ? U.value = Y : U.result = Y), S(k, U);
        }
      });
      function S(k, _) {
        k || !_ ? v(k) : z ? z(_) : x(null, _);
      }
    }
  }
  function d(y) {
    let x;
    l.freeze(), cn("processSync", l.Parser), fn("processSync", l.Compiler);
    const b = He(y);
    return l.process(b, z), ut("processSync", "process", x), b;
    function z(v) {
      x = !0, Gn(v);
    }
  }
}
function lt(e, n) {
  return typeof e == "function" && // Prototypes do exist.
  // type-coverage:ignore-next-line
  e.prototype && // A function with keys in its prototype is probably a constructor.
  // Classes’ prototype methods are not enumerable, so we check if some value
  // exists in the prototype.
  // type-coverage:ignore-next-line
  (ri(e.prototype) || n in e.prototype);
}
function ri(e) {
  let n;
  for (n in e)
    if (Gt.call(e, n))
      return !0;
  return !1;
}
function cn(e, n) {
  if (typeof n != "function")
    throw new TypeError("Cannot `" + e + "` without `Parser`");
}
function fn(e, n) {
  if (typeof n != "function")
    throw new TypeError("Cannot `" + e + "` without `Compiler`");
}
function pn(e, n) {
  if (n)
    throw new Error(
      "Cannot call `" + e + "` on a frozen processor.\nCreate a new processor first, by calling it: use `processor()` instead of `processor`."
    );
}
function ot(e) {
  if (!vn(e) || typeof e.type != "string")
    throw new TypeError("Expected node, got `" + e + "`");
}
function ut(e, n, t) {
  if (!t)
    throw new Error(
      "`" + e + "` finished async. Use `" + n + "` instead"
    );
}
function He(e) {
  return ii(e) ? e : new Qt(e);
}
function ii(e) {
  return !!(e && typeof e == "object" && "message" in e && "messages" in e);
}
function li(e) {
  return typeof e == "string" || Xt(e);
}
const oi = {};
function ui(e, n) {
  const t = n || oi, r = typeof t.includeImageAlt == "boolean" ? t.includeImageAlt : !0, i = typeof t.includeHtml == "boolean" ? t.includeHtml : !0;
  return Jt(e, r, i);
}
function Jt(e, n, t) {
  if (ai(e)) {
    if ("value" in e)
      return e.type === "html" && !t ? "" : e.value;
    if (n && "alt" in e && e.alt)
      return e.alt;
    if ("children" in e)
      return at(e.children, n, t);
  }
  return Array.isArray(e) ? at(e, n, t) : "";
}
function at(e, n, t) {
  const r = [];
  let i = -1;
  for (; ++i < e.length; )
    r[i] = Jt(e[i], n, t);
  return r.join("");
}
function ai(e) {
  return !!(e && typeof e == "object");
}
function Te(e, n, t, r) {
  const i = e.length;
  let l = 0, o;
  if (n < 0 ? n = -n > i ? 0 : i + n : n = n > i ? i : n, t = t > 0 ? t : 0, r.length < 1e4)
    o = Array.from(r), o.unshift(n, t), [].splice.apply(e, o);
  else
    for (t && [].splice.apply(e, [n, t]); l < r.length; )
      o = r.slice(l, l + 1e4), o.unshift(n, 0), [].splice.apply(e, o), l += 1e4, n += 1e4;
}
function be(e, n) {
  return e.length > 0 ? (Te(e, e.length, 0, n), e) : n;
}
const st = {}.hasOwnProperty;
function si(e) {
  const n = {};
  let t = -1;
  for (; ++t < e.length; )
    ci(n, e[t]);
  return n;
}
function ci(e, n) {
  let t;
  for (t in n) {
    const i = (st.call(e, t) ? e[t] : void 0) || (e[t] = {}), l = n[t];
    let o;
    for (o in l) {
      st.call(i, o) || (i[o] = []);
      const u = l[o];
      fi(
        // @ts-expect-error Looks like a list.
        i[o],
        Array.isArray(u) ? u : u ? [u] : []
      );
    }
  }
}
function fi(e, n) {
  let t = -1;
  const r = [];
  for (; ++t < n.length; )
    (n[t].add === "after" ? e : r).push(n[t]);
  Te(e, 0, 0, r);
}
const pi = /[!-/:-@[-`{-~\u00A1\u00A7\u00AB\u00B6\u00B7\u00BB\u00BF\u037E\u0387\u055A-\u055F\u0589\u058A\u05BE\u05C0\u05C3\u05C6\u05F3\u05F4\u0609\u060A\u060C\u060D\u061B\u061E\u061F\u066A-\u066D\u06D4\u0700-\u070D\u07F7-\u07F9\u0830-\u083E\u085E\u0964\u0965\u0970\u09FD\u0A76\u0AF0\u0C77\u0C84\u0DF4\u0E4F\u0E5A\u0E5B\u0F04-\u0F12\u0F14\u0F3A-\u0F3D\u0F85\u0FD0-\u0FD4\u0FD9\u0FDA\u104A-\u104F\u10FB\u1360-\u1368\u1400\u166E\u169B\u169C\u16EB-\u16ED\u1735\u1736\u17D4-\u17D6\u17D8-\u17DA\u1800-\u180A\u1944\u1945\u1A1E\u1A1F\u1AA0-\u1AA6\u1AA8-\u1AAD\u1B5A-\u1B60\u1BFC-\u1BFF\u1C3B-\u1C3F\u1C7E\u1C7F\u1CC0-\u1CC7\u1CD3\u2010-\u2027\u2030-\u2043\u2045-\u2051\u2053-\u205E\u207D\u207E\u208D\u208E\u2308-\u230B\u2329\u232A\u2768-\u2775\u27C5\u27C6\u27E6-\u27EF\u2983-\u2998\u29D8-\u29DB\u29FC\u29FD\u2CF9-\u2CFC\u2CFE\u2CFF\u2D70\u2E00-\u2E2E\u2E30-\u2E4F\u2E52\u3001-\u3003\u3008-\u3011\u3014-\u301F\u3030\u303D\u30A0\u30FB\uA4FE\uA4FF\uA60D-\uA60F\uA673\uA67E\uA6F2-\uA6F7\uA874-\uA877\uA8CE\uA8CF\uA8F8-\uA8FA\uA8FC\uA92E\uA92F\uA95F\uA9C1-\uA9CD\uA9DE\uA9DF\uAA5C-\uAA5F\uAADE\uAADF\uAAF0\uAAF1\uABEB\uFD3E\uFD3F\uFE10-\uFE19\uFE30-\uFE52\uFE54-\uFE61\uFE63\uFE68\uFE6A\uFE6B\uFF01-\uFF03\uFF05-\uFF0A\uFF0C-\uFF0F\uFF1A\uFF1B\uFF1F\uFF20\uFF3B-\uFF3D\uFF3F\uFF5B\uFF5D\uFF5F-\uFF65]/, ve = Re(/[A-Za-z]/), Tn = Re(/\d/), hi = Re(/[\dA-Fa-f]/), xe = Re(/[\dA-Za-z]/), di = Re(/[!-/:-@[-`{-~]/), ct = Re(/[#-'*+\--9=?A-Z^-~]/);
function Pn(e) {
  return (
    // Special whitespace codes (which have negative values), C0 and Control
    // character DEL
    e !== null && (e < 32 || e === 127)
  );
}
function ke(e) {
  return e !== null && (e < 0 || e === 32);
}
function N(e) {
  return e !== null && e < -2;
}
function pe(e) {
  return e === -2 || e === -1 || e === 32;
}
const mi = Re(/\s/), gi = Re(pi);
function Re(e) {
  return n;
  function n(t) {
    return t !== null && e.test(String.fromCharCode(t));
  }
}
function ne(e, n, t, r) {
  const i = r ? r - 1 : Number.POSITIVE_INFINITY;
  let l = 0;
  return o;
  function o(a) {
    return pe(a) ? (e.enter(t), u(a)) : n(a);
  }
  function u(a) {
    return pe(a) && l++ < i ? (e.consume(a), u) : (e.exit(t), n(a));
  }
}
const yi = {
  tokenize: xi
};
function xi(e) {
  const n = e.attempt(
    this.parser.constructs.contentInitial,
    r,
    i
  );
  let t;
  return n;
  function r(u) {
    if (u === null) {
      e.consume(u);
      return;
    }
    return e.enter("lineEnding"), e.consume(u), e.exit("lineEnding"), ne(e, n, "linePrefix");
  }
  function i(u) {
    return e.enter("paragraph"), l(u);
  }
  function l(u) {
    const a = e.enter("chunkText", {
      contentType: "text",
      previous: t
    });
    return t && (t.next = a), t = a, o(u);
  }
  function o(u) {
    if (u === null) {
      e.exit("chunkText"), e.exit("paragraph"), e.consume(u);
      return;
    }
    return N(u) ? (e.consume(u), e.exit("chunkText"), l) : (e.consume(u), o);
  }
}
const bi = {
  tokenize: ki
}, ft = {
  tokenize: wi
};
function ki(e) {
  const n = this, t = [];
  let r = 0, i, l, o;
  return u;
  function u(v) {
    if (r < t.length) {
      const F = t[r];
      return n.containerState = F[1], e.attempt(
        F[0].continuation,
        a,
        f
      )(v);
    }
    return f(v);
  }
  function a(v) {
    if (r++, n.containerState._closeFlow) {
      n.containerState._closeFlow = void 0, i && z();
      const F = n.events.length;
      let S = F, k;
      for (; S--; )
        if (n.events[S][0] === "exit" && n.events[S][1].type === "chunkFlow") {
          k = n.events[S][1].end;
          break;
        }
      b(r);
      let _ = F;
      for (; _ < n.events.length; )
        n.events[_][1].end = Object.assign({}, k), _++;
      return Te(
        n.events,
        S + 1,
        0,
        n.events.slice(F)
      ), n.events.length = _, f(v);
    }
    return u(v);
  }
  function f(v) {
    if (r === t.length) {
      if (!i)
        return g(v);
      if (i.currentConstruct && i.currentConstruct.concrete)
        return d(v);
      n.interrupt = !!(i.currentConstruct && !i._gfmTableDynamicInterruptHack);
    }
    return n.containerState = {}, e.check(
      ft,
      c,
      h
    )(v);
  }
  function c(v) {
    return i && z(), b(r), g(v);
  }
  function h(v) {
    return n.parser.lazy[n.now().line] = r !== t.length, o = n.now().offset, d(v);
  }
  function g(v) {
    return n.containerState = {}, e.attempt(
      ft,
      m,
      d
    )(v);
  }
  function m(v) {
    return r++, t.push([n.currentConstruct, n.containerState]), g(v);
  }
  function d(v) {
    if (v === null) {
      i && z(), b(0), e.consume(v);
      return;
    }
    return i = i || n.parser.flow(n.now()), e.enter("chunkFlow", {
      contentType: "flow",
      previous: l,
      _tokenizer: i
    }), y(v);
  }
  function y(v) {
    if (v === null) {
      x(e.exit("chunkFlow"), !0), b(0), e.consume(v);
      return;
    }
    return N(v) ? (e.consume(v), x(e.exit("chunkFlow")), r = 0, n.interrupt = void 0, u) : (e.consume(v), y);
  }
  function x(v, F) {
    const S = n.sliceStream(v);
    if (F && S.push(null), v.previous = l, l && (l.next = v), l = v, i.defineSkip(v.start), i.write(S), n.parser.lazy[v.start.line]) {
      let k = i.events.length;
      for (; k--; )
        if (
          // The token starts before the line ending…
          i.events[k][1].start.offset < o && // …and either is not ended yet…
          (!i.events[k][1].end || // …or ends after it.
          i.events[k][1].end.offset > o)
        )
          return;
      const _ = n.events.length;
      let U = _, Y, le;
      for (; U--; )
        if (n.events[U][0] === "exit" && n.events[U][1].type === "chunkFlow") {
          if (Y) {
            le = n.events[U][1].end;
            break;
          }
          Y = !0;
        }
      for (b(r), k = _; k < n.events.length; )
        n.events[k][1].end = Object.assign({}, le), k++;
      Te(
        n.events,
        U + 1,
        0,
        n.events.slice(_)
      ), n.events.length = k;
    }
  }
  function b(v) {
    let F = t.length;
    for (; F-- > v; ) {
      const S = t[F];
      n.containerState = S[1], S[0].exit.call(n, e);
    }
    t.length = v;
  }
  function z() {
    i.write([null]), l = void 0, i = void 0, n.containerState._closeFlow = void 0;
  }
}
function wi(e, n, t) {
  return ne(
    e,
    e.attempt(this.parser.constructs.document, n, t),
    "linePrefix",
    this.parser.constructs.disable.null.includes("codeIndented") ? void 0 : 4
  );
}
function pt(e) {
  if (e === null || ke(e) || mi(e))
    return 1;
  if (gi(e))
    return 2;
}
function Dn(e, n, t) {
  const r = [];
  let i = -1;
  for (; ++i < e.length; ) {
    const l = e[i].resolveAll;
    l && !r.includes(l) && (n = l(n, t), r.push(l));
  }
  return n;
}
const An = {
  name: "attention",
  tokenize: Ei,
  resolveAll: Si
};
function Si(e, n) {
  let t = -1, r, i, l, o, u, a, f, c;
  for (; ++t < e.length; )
    if (e[t][0] === "enter" && e[t][1].type === "attentionSequence" && e[t][1]._close) {
      for (r = t; r--; )
        if (e[r][0] === "exit" && e[r][1].type === "attentionSequence" && e[r][1]._open && // If the markers are the same:
        n.sliceSerialize(e[r][1]).charCodeAt(0) === n.sliceSerialize(e[t][1]).charCodeAt(0)) {
          if ((e[r][1]._close || e[t][1]._open) && (e[t][1].end.offset - e[t][1].start.offset) % 3 && !((e[r][1].end.offset - e[r][1].start.offset + e[t][1].end.offset - e[t][1].start.offset) % 3))
            continue;
          a = e[r][1].end.offset - e[r][1].start.offset > 1 && e[t][1].end.offset - e[t][1].start.offset > 1 ? 2 : 1;
          const h = Object.assign({}, e[r][1].end), g = Object.assign({}, e[t][1].start);
          ht(h, -a), ht(g, a), o = {
            type: a > 1 ? "strongSequence" : "emphasisSequence",
            start: h,
            end: Object.assign({}, e[r][1].end)
          }, u = {
            type: a > 1 ? "strongSequence" : "emphasisSequence",
            start: Object.assign({}, e[t][1].start),
            end: g
          }, l = {
            type: a > 1 ? "strongText" : "emphasisText",
            start: Object.assign({}, e[r][1].end),
            end: Object.assign({}, e[t][1].start)
          }, i = {
            type: a > 1 ? "strong" : "emphasis",
            start: Object.assign({}, o.start),
            end: Object.assign({}, u.end)
          }, e[r][1].end = Object.assign({}, o.start), e[t][1].start = Object.assign({}, u.end), f = [], e[r][1].end.offset - e[r][1].start.offset && (f = be(f, [
            ["enter", e[r][1], n],
            ["exit", e[r][1], n]
          ])), f = be(f, [
            ["enter", i, n],
            ["enter", o, n],
            ["exit", o, n],
            ["enter", l, n]
          ]), f = be(
            f,
            Dn(
              n.parser.constructs.insideSpan.null,
              e.slice(r + 1, t),
              n
            )
          ), f = be(f, [
            ["exit", l, n],
            ["enter", u, n],
            ["exit", u, n],
            ["exit", i, n]
          ]), e[t][1].end.offset - e[t][1].start.offset ? (c = 2, f = be(f, [
            ["enter", e[t][1], n],
            ["exit", e[t][1], n]
          ])) : c = 0, Te(e, r - 1, t - r + 3, f), t = r + f.length - c - 2;
          break;
        }
    }
  for (t = -1; ++t < e.length; )
    e[t][1].type === "attentionSequence" && (e[t][1].type = "data");
  return e;
}
function Ei(e, n) {
  const t = this.parser.constructs.attentionMarkers.null, r = this.previous, i = pt(r);
  let l;
  return o;
  function o(a) {
    return e.enter("attentionSequence"), l = a, u(a);
  }
  function u(a) {
    if (a === l)
      return e.consume(a), u;
    const f = e.exit("attentionSequence"), c = pt(a), h = !c || c === 2 && i || t.includes(a), g = !i || i === 2 && c || t.includes(r);
    return f._open = !!(l === 42 ? h : h && (i || !g)), f._close = !!(l === 42 ? g : g && (c || !h)), n(a);
  }
}
function ht(e, n) {
  e.column += n, e.offset += n, e._bufferIndex += n;
}
const Ci = {
  name: "autolink",
  tokenize: vi
};
function vi(e, n, t) {
  let r = 1;
  return i;
  function i(d) {
    return e.enter("autolink"), e.enter("autolinkMarker"), e.consume(d), e.exit("autolinkMarker"), e.enter("autolinkProtocol"), l;
  }
  function l(d) {
    return ve(d) ? (e.consume(d), o) : ct(d) ? f(d) : t(d);
  }
  function o(d) {
    return d === 43 || d === 45 || d === 46 || xe(d) ? u(d) : f(d);
  }
  function u(d) {
    return d === 58 ? (e.consume(d), a) : (d === 43 || d === 45 || d === 46 || xe(d)) && r++ < 32 ? (e.consume(d), u) : f(d);
  }
  function a(d) {
    return d === 62 ? (e.exit("autolinkProtocol"), m(d)) : d === null || d === 32 || d === 60 || Pn(d) ? t(d) : (e.consume(d), a);
  }
  function f(d) {
    return d === 64 ? (e.consume(d), r = 0, c) : ct(d) ? (e.consume(d), f) : t(d);
  }
  function c(d) {
    return xe(d) ? h(d) : t(d);
  }
  function h(d) {
    return d === 46 ? (e.consume(d), r = 0, c) : d === 62 ? (e.exit("autolinkProtocol").type = "autolinkEmail", m(d)) : g(d);
  }
  function g(d) {
    return (d === 45 || xe(d)) && r++ < 63 ? (e.consume(d), d === 45 ? g : h) : t(d);
  }
  function m(d) {
    return e.enter("autolinkMarker"), e.consume(d), e.exit("autolinkMarker"), e.exit("autolink"), n;
  }
}
const tn = {
  tokenize: Ti,
  partial: !0
};
function Ti(e, n, t) {
  return ne(e, r, "linePrefix");
  function r(i) {
    return i === null || N(i) ? n(i) : t(i);
  }
}
const er = {
  name: "blockQuote",
  tokenize: Pi,
  continuation: {
    tokenize: Ai
  },
  exit: Oi
};
function Pi(e, n, t) {
  const r = this;
  return i;
  function i(o) {
    if (o === 62) {
      const u = r.containerState;
      return u.open || (e.enter("blockQuote", {
        _container: !0
      }), u.open = !0), e.enter("blockQuotePrefix"), e.enter("blockQuoteMarker"), e.consume(o), e.exit("blockQuoteMarker"), l;
    }
    return t(o);
  }
  function l(o) {
    return pe(o) ? (e.enter("blockQuotePrefixWhitespace"), e.consume(o), e.exit("blockQuotePrefixWhitespace"), e.exit("blockQuotePrefix"), n) : (e.exit("blockQuotePrefix"), n(o));
  }
}
function Ai(e, n, t) {
  return ne(
    e,
    e.attempt(er, n, t),
    "linePrefix",
    this.parser.constructs.disable.null.includes("codeIndented") ? void 0 : 4
  );
}
function Oi(e) {
  e.exit("blockQuote");
}
const nr = {
  name: "characterEscape",
  tokenize: Ii
};
function Ii(e, n, t) {
  return r;
  function r(l) {
    return e.enter("characterEscape"), e.enter("escapeMarker"), e.consume(l), e.exit("escapeMarker"), i;
  }
  function i(l) {
    return di(l) ? (e.enter("characterEscapeValue"), e.consume(l), e.exit("characterEscapeValue"), e.exit("characterEscape"), n) : t(l);
  }
}
const dt = document.createElement("i");
function zn(e) {
  const n = "&" + e + ";";
  dt.innerHTML = n;
  const t = dt.textContent;
  return t.charCodeAt(t.length - 1) === 59 && e !== "semi" || t === n ? !1 : t;
}
const tr = {
  name: "characterReference",
  tokenize: Fi
};
function Fi(e, n, t) {
  const r = this;
  let i = 0, l, o;
  return u;
  function u(h) {
    return e.enter("characterReference"), e.enter("characterReferenceMarker"), e.consume(h), e.exit("characterReferenceMarker"), a;
  }
  function a(h) {
    return h === 35 ? (e.enter("characterReferenceMarkerNumeric"), e.consume(h), e.exit("characterReferenceMarkerNumeric"), f) : (e.enter("characterReferenceValue"), l = 31, o = xe, c(h));
  }
  function f(h) {
    return h === 88 || h === 120 ? (e.enter("characterReferenceMarkerHexadecimal"), e.consume(h), e.exit("characterReferenceMarkerHexadecimal"), e.enter("characterReferenceValue"), l = 6, o = hi, c) : (e.enter("characterReferenceValue"), l = 7, o = Tn, c(h));
  }
  function c(h) {
    let g;
    return h === 59 && i ? (g = e.exit("characterReferenceValue"), o === xe && !zn(r.sliceSerialize(g)) ? t(h) : (e.enter("characterReferenceMarker"), e.consume(h), e.exit("characterReferenceMarker"), e.exit("characterReference"), n)) : o(h) && i++ < l ? (e.consume(h), c) : t(h);
  }
}
const mt = {
  name: "codeFenced",
  tokenize: Ri,
  concrete: !0
};
function Ri(e, n, t) {
  const r = this, i = {
    tokenize: S,
    partial: !0
  }, l = {
    tokenize: F,
    partial: !0
  }, o = this.events[this.events.length - 1], u = o && o[1].type === "linePrefix" ? o[2].sliceSerialize(o[1], !0).length : 0;
  let a = 0, f;
  return c;
  function c(k) {
    return e.enter("codeFenced"), e.enter("codeFencedFence"), e.enter("codeFencedFenceSequence"), f = k, h(k);
  }
  function h(k) {
    return k === f ? (e.consume(k), a++, h) : (e.exit("codeFencedFenceSequence"), a < 3 ? t(k) : ne(e, g, "whitespace")(k));
  }
  function g(k) {
    return k === null || N(k) ? x(k) : (e.enter("codeFencedFenceInfo"), e.enter("chunkString", {
      contentType: "string"
    }), m(k));
  }
  function m(k) {
    return k === null || ke(k) ? (e.exit("chunkString"), e.exit("codeFencedFenceInfo"), ne(e, d, "whitespace")(k)) : k === 96 && k === f ? t(k) : (e.consume(k), m);
  }
  function d(k) {
    return k === null || N(k) ? x(k) : (e.enter("codeFencedFenceMeta"), e.enter("chunkString", {
      contentType: "string"
    }), y(k));
  }
  function y(k) {
    return k === null || N(k) ? (e.exit("chunkString"), e.exit("codeFencedFenceMeta"), x(k)) : k === 96 && k === f ? t(k) : (e.consume(k), y);
  }
  function x(k) {
    return e.exit("codeFencedFence"), r.interrupt ? n(k) : b(k);
  }
  function b(k) {
    return k === null ? v(k) : N(k) ? e.attempt(
      l,
      e.attempt(
        i,
        v,
        u ? ne(
          e,
          b,
          "linePrefix",
          u + 1
        ) : b
      ),
      v
    )(k) : (e.enter("codeFlowValue"), z(k));
  }
  function z(k) {
    return k === null || N(k) ? (e.exit("codeFlowValue"), b(k)) : (e.consume(k), z);
  }
  function v(k) {
    return e.exit("codeFenced"), n(k);
  }
  function F(k, _, U) {
    const Y = this;
    return le;
    function le(R) {
      return k.enter("lineEnding"), k.consume(R), k.exit("lineEnding"), P;
    }
    function P(R) {
      return Y.parser.lazy[Y.now().line] ? U(R) : _(R);
    }
  }
  function S(k, _, U) {
    let Y = 0;
    return ne(
      k,
      le,
      "linePrefix",
      this.parser.constructs.disable.null.includes("codeIndented") ? void 0 : 4
    );
    function le(X) {
      return k.enter("codeFencedFence"), k.enter("codeFencedFenceSequence"), P(X);
    }
    function P(X) {
      return X === f ? (k.consume(X), Y++, P) : Y < a ? U(X) : (k.exit("codeFencedFenceSequence"), ne(k, R, "whitespace")(X));
    }
    function R(X) {
      return X === null || N(X) ? (k.exit("codeFencedFence"), _(X)) : U(X);
    }
  }
}
const hn = {
  name: "codeIndented",
  tokenize: Li
}, _i = {
  tokenize: Di,
  partial: !0
};
function Li(e, n, t) {
  const r = this;
  return i;
  function i(f) {
    return e.enter("codeIndented"), ne(e, l, "linePrefix", 4 + 1)(f);
  }
  function l(f) {
    const c = r.events[r.events.length - 1];
    return c && c[1].type === "linePrefix" && c[2].sliceSerialize(c[1], !0).length >= 4 ? o(f) : t(f);
  }
  function o(f) {
    return f === null ? a(f) : N(f) ? e.attempt(_i, o, a)(f) : (e.enter("codeFlowValue"), u(f));
  }
  function u(f) {
    return f === null || N(f) ? (e.exit("codeFlowValue"), o(f)) : (e.consume(f), u);
  }
  function a(f) {
    return e.exit("codeIndented"), n(f);
  }
}
function Di(e, n, t) {
  const r = this;
  return i;
  function i(o) {
    return r.parser.lazy[r.now().line] ? t(o) : N(o) ? (e.enter("lineEnding"), e.consume(o), e.exit("lineEnding"), i) : ne(e, l, "linePrefix", 4 + 1)(o);
  }
  function l(o) {
    const u = r.events[r.events.length - 1];
    return u && u[1].type === "linePrefix" && u[2].sliceSerialize(u[1], !0).length >= 4 ? n(o) : N(o) ? i(o) : t(o);
  }
}
const zi = {
  name: "codeText",
  tokenize: Ni,
  resolve: Mi,
  previous: Bi
};
function Mi(e) {
  let n = e.length - 4, t = 3, r, i;
  if ((e[t][1].type === "lineEnding" || e[t][1].type === "space") && (e[n][1].type === "lineEnding" || e[n][1].type === "space")) {
    for (r = t; ++r < n; )
      if (e[r][1].type === "codeTextData") {
        e[t][1].type = "codeTextPadding", e[n][1].type = "codeTextPadding", t += 2, n -= 2;
        break;
      }
  }
  for (r = t - 1, n++; ++r <= n; )
    i === void 0 ? r !== n && e[r][1].type !== "lineEnding" && (i = r) : (r === n || e[r][1].type === "lineEnding") && (e[i][1].type = "codeTextData", r !== i + 2 && (e[i][1].end = e[r - 1][1].end, e.splice(i + 2, r - i - 2), n -= r - i - 2, r = i + 2), i = void 0);
  return e;
}
function Bi(e) {
  return e !== 96 || this.events[this.events.length - 1][1].type === "characterEscape";
}
function Ni(e, n, t) {
  let r = 0, i, l;
  return o;
  function o(h) {
    return e.enter("codeText"), e.enter("codeTextSequence"), u(h);
  }
  function u(h) {
    return h === 96 ? (e.consume(h), r++, u) : (e.exit("codeTextSequence"), a(h));
  }
  function a(h) {
    return h === null ? t(h) : h === 96 ? (l = e.enter("codeTextSequence"), i = 0, c(h)) : h === 32 ? (e.enter("space"), e.consume(h), e.exit("space"), a) : N(h) ? (e.enter("lineEnding"), e.consume(h), e.exit("lineEnding"), a) : (e.enter("codeTextData"), f(h));
  }
  function f(h) {
    return h === null || h === 32 || h === 96 || N(h) ? (e.exit("codeTextData"), a(h)) : (e.consume(h), f);
  }
  function c(h) {
    return h === 96 ? (e.consume(h), i++, c) : i === r ? (e.exit("codeTextSequence"), e.exit("codeText"), n(h)) : (l.type = "codeTextData", f(h));
  }
}
function rr(e) {
  const n = {};
  let t = -1, r, i, l, o, u, a, f;
  for (; ++t < e.length; ) {
    for (; t in n; )
      t = n[t];
    if (r = e[t], t && r[1].type === "chunkFlow" && e[t - 1][1].type === "listItemPrefix" && (a = r[1]._tokenizer.events, l = 0, l < a.length && a[l][1].type === "lineEndingBlank" && (l += 2), l < a.length && a[l][1].type === "content"))
      for (; ++l < a.length && a[l][1].type !== "content"; )
        a[l][1].type === "chunkText" && (a[l][1]._isInFirstContentOfListItem = !0, l++);
    if (r[0] === "enter")
      r[1].contentType && (Object.assign(n, ji(e, t)), t = n[t], f = !0);
    else if (r[1]._container) {
      for (l = t, i = void 0; l-- && (o = e[l], o[1].type === "lineEnding" || o[1].type === "lineEndingBlank"); )
        o[0] === "enter" && (i && (e[i][1].type = "lineEndingBlank"), o[1].type = "lineEnding", i = l);
      i && (r[1].end = Object.assign({}, e[i][1].start), u = e.slice(i, t), u.unshift(r), Te(e, i, t - i + 1, u));
    }
  }
  return !f;
}
function ji(e, n) {
  const t = e[n][1], r = e[n][2];
  let i = n - 1;
  const l = [], o = t._tokenizer || r.parser[t.contentType](t.start), u = o.events, a = [], f = {};
  let c, h, g = -1, m = t, d = 0, y = 0;
  const x = [y];
  for (; m; ) {
    for (; e[++i][1] !== m; )
      ;
    l.push(i), m._tokenizer || (c = r.sliceStream(m), m.next || c.push(null), h && o.defineSkip(m.start), m._isInFirstContentOfListItem && (o._gfmTasklistFirstContentOfListItem = !0), o.write(c), m._isInFirstContentOfListItem && (o._gfmTasklistFirstContentOfListItem = void 0)), h = m, m = m.next;
  }
  for (m = t; ++g < u.length; )
    // Find a void token that includes a break.
    u[g][0] === "exit" && u[g - 1][0] === "enter" && u[g][1].type === u[g - 1][1].type && u[g][1].start.line !== u[g][1].end.line && (y = g + 1, x.push(y), m._tokenizer = void 0, m.previous = void 0, m = m.next);
  for (o.events = [], m ? (m._tokenizer = void 0, m.previous = void 0) : x.pop(), g = x.length; g--; ) {
    const b = u.slice(x[g], x[g + 1]), z = l.pop();
    a.unshift([z, z + b.length - 1]), Te(e, z, 2, b);
  }
  for (g = -1; ++g < a.length; )
    f[d + a[g][0]] = d + a[g][1], d += a[g][1] - a[g][0] - 1;
  return f;
}
const $i = {
  tokenize: Hi,
  resolve: qi
}, Ui = {
  tokenize: Vi,
  partial: !0
};
function qi(e) {
  return rr(e), e;
}
function Hi(e, n) {
  let t;
  return r;
  function r(u) {
    return e.enter("content"), t = e.enter("chunkContent", {
      contentType: "content"
    }), i(u);
  }
  function i(u) {
    return u === null ? l(u) : N(u) ? e.check(
      Ui,
      o,
      l
    )(u) : (e.consume(u), i);
  }
  function l(u) {
    return e.exit("chunkContent"), e.exit("content"), n(u);
  }
  function o(u) {
    return e.consume(u), e.exit("chunkContent"), t.next = e.enter("chunkContent", {
      contentType: "content",
      previous: t
    }), t = t.next, i;
  }
}
function Vi(e, n, t) {
  const r = this;
  return i;
  function i(o) {
    return e.exit("chunkContent"), e.enter("lineEnding"), e.consume(o), e.exit("lineEnding"), ne(e, l, "linePrefix");
  }
  function l(o) {
    if (o === null || N(o))
      return t(o);
    const u = r.events[r.events.length - 1];
    return !r.parser.constructs.disable.null.includes("codeIndented") && u && u[1].type === "linePrefix" && u[2].sliceSerialize(u[1], !0).length >= 4 ? n(o) : e.interrupt(r.parser.constructs.flow, t, n)(o);
  }
}
function ir(e, n, t, r, i, l, o, u, a) {
  const f = a || Number.POSITIVE_INFINITY;
  let c = 0;
  return h;
  function h(b) {
    return b === 60 ? (e.enter(r), e.enter(i), e.enter(l), e.consume(b), e.exit(l), g) : b === null || b === 41 || Pn(b) ? t(b) : (e.enter(r), e.enter(o), e.enter(u), e.enter("chunkString", {
      contentType: "string"
    }), y(b));
  }
  function g(b) {
    return b === 62 ? (e.enter(l), e.consume(b), e.exit(l), e.exit(i), e.exit(r), n) : (e.enter(u), e.enter("chunkString", {
      contentType: "string"
    }), m(b));
  }
  function m(b) {
    return b === 62 ? (e.exit("chunkString"), e.exit(u), g(b)) : b === null || b === 60 || N(b) ? t(b) : (e.consume(b), b === 92 ? d : m);
  }
  function d(b) {
    return b === 60 || b === 62 || b === 92 ? (e.consume(b), m) : m(b);
  }
  function y(b) {
    return b === 40 ? ++c > f ? t(b) : (e.consume(b), y) : b === 41 ? c-- ? (e.consume(b), y) : (e.exit("chunkString"), e.exit(u), e.exit(o), e.exit(r), n(b)) : b === null || ke(b) ? c ? t(b) : (e.exit("chunkString"), e.exit(u), e.exit(o), e.exit(r), n(b)) : Pn(b) ? t(b) : (e.consume(b), b === 92 ? x : y);
  }
  function x(b) {
    return b === 40 || b === 41 || b === 92 ? (e.consume(b), y) : y(b);
  }
}
function lr(e, n, t, r, i, l) {
  const o = this;
  let u = 0, a;
  return f;
  function f(m) {
    return e.enter(r), e.enter(i), e.consume(m), e.exit(i), e.enter(l), c;
  }
  function c(m) {
    return m === null || m === 91 || m === 93 && !a || /* To do: remove in the future once we’ve switched from
     * `micromark-extension-footnote` to `micromark-extension-gfm-footnote`,
     * which doesn’t need this */
    /* Hidden footnotes hook */
    /* c8 ignore next 3 */
    m === 94 && !u && "_hiddenFootnoteSupport" in o.parser.constructs || u > 999 ? t(m) : m === 93 ? (e.exit(l), e.enter(i), e.consume(m), e.exit(i), e.exit(r), n) : N(m) ? (e.enter("lineEnding"), e.consume(m), e.exit("lineEnding"), c) : (e.enter("chunkString", {
      contentType: "string"
    }), h(m));
  }
  function h(m) {
    return m === null || m === 91 || m === 93 || N(m) || u++ > 999 ? (e.exit("chunkString"), c(m)) : (e.consume(m), a = a || !pe(m), m === 92 ? g : h);
  }
  function g(m) {
    return m === 91 || m === 92 || m === 93 ? (e.consume(m), u++, h) : h(m);
  }
}
function or(e, n, t, r, i, l) {
  let o;
  return u;
  function u(g) {
    return e.enter(r), e.enter(i), e.consume(g), e.exit(i), o = g === 40 ? 41 : g, a;
  }
  function a(g) {
    return g === o ? (e.enter(i), e.consume(g), e.exit(i), e.exit(r), n) : (e.enter(l), f(g));
  }
  function f(g) {
    return g === o ? (e.exit(l), a(o)) : g === null ? t(g) : N(g) ? (e.enter("lineEnding"), e.consume(g), e.exit("lineEnding"), ne(e, f, "linePrefix")) : (e.enter("chunkString", {
      contentType: "string"
    }), c(g));
  }
  function c(g) {
    return g === o || g === null || N(g) ? (e.exit("chunkString"), f(g)) : (e.consume(g), g === 92 ? h : c);
  }
  function h(g) {
    return g === o || g === 92 ? (e.consume(g), c) : c(g);
  }
}
function Ve(e, n) {
  let t;
  return r;
  function r(i) {
    return N(i) ? (e.enter("lineEnding"), e.consume(i), e.exit("lineEnding"), t = !0, r) : pe(i) ? ne(
      e,
      r,
      t ? "linePrefix" : "lineSuffix"
    )(i) : n(i);
  }
}
function Me(e) {
  return e.replace(/[\t\n\r ]+/g, " ").replace(/^ | $/g, "").toLowerCase().toUpperCase();
}
const Yi = {
  name: "definition",
  tokenize: Xi
}, Wi = {
  tokenize: Qi,
  partial: !0
};
function Xi(e, n, t) {
  const r = this;
  let i;
  return l;
  function l(a) {
    return e.enter("definition"), lr.call(
      r,
      e,
      o,
      t,
      "definitionLabel",
      "definitionLabelMarker",
      "definitionLabelString"
    )(a);
  }
  function o(a) {
    return i = Me(
      r.sliceSerialize(r.events[r.events.length - 1][1]).slice(1, -1)
    ), a === 58 ? (e.enter("definitionMarker"), e.consume(a), e.exit("definitionMarker"), Ve(
      e,
      ir(
        e,
        e.attempt(
          Wi,
          ne(e, u, "whitespace"),
          ne(e, u, "whitespace")
        ),
        t,
        "definitionDestination",
        "definitionDestinationLiteral",
        "definitionDestinationLiteralMarker",
        "definitionDestinationRaw",
        "definitionDestinationString"
      )
    )) : t(a);
  }
  function u(a) {
    return a === null || N(a) ? (e.exit("definition"), r.parser.defined.includes(i) || r.parser.defined.push(i), n(a)) : t(a);
  }
}
function Qi(e, n, t) {
  return r;
  function r(o) {
    return ke(o) ? Ve(e, i)(o) : t(o);
  }
  function i(o) {
    return o === 34 || o === 39 || o === 40 ? or(
      e,
      ne(e, l, "whitespace"),
      t,
      "definitionTitle",
      "definitionTitleMarker",
      "definitionTitleString"
    )(o) : t(o);
  }
  function l(o) {
    return o === null || N(o) ? n(o) : t(o);
  }
}
const Ki = {
  name: "hardBreakEscape",
  tokenize: Gi
};
function Gi(e, n, t) {
  return r;
  function r(l) {
    return e.enter("hardBreakEscape"), e.enter("escapeMarker"), e.consume(l), i;
  }
  function i(l) {
    return N(l) ? (e.exit("escapeMarker"), e.exit("hardBreakEscape"), n(l)) : t(l);
  }
}
const Zi = {
  name: "headingAtx",
  tokenize: el,
  resolve: Ji
};
function Ji(e, n) {
  let t = e.length - 2, r = 3, i, l;
  return e[r][1].type === "whitespace" && (r += 2), t - 2 > r && e[t][1].type === "whitespace" && (t -= 2), e[t][1].type === "atxHeadingSequence" && (r === t - 1 || t - 4 > r && e[t - 2][1].type === "whitespace") && (t -= r + 1 === t ? 2 : 4), t > r && (i = {
    type: "atxHeadingText",
    start: e[r][1].start,
    end: e[t][1].end
  }, l = {
    type: "chunkText",
    start: e[r][1].start,
    end: e[t][1].end,
    // @ts-expect-error Constants are fine to assign.
    contentType: "text"
  }, Te(e, r, t - r + 1, [
    ["enter", i, n],
    ["enter", l, n],
    ["exit", l, n],
    ["exit", i, n]
  ])), e;
}
function el(e, n, t) {
  const r = this;
  let i = 0;
  return l;
  function l(c) {
    return e.enter("atxHeading"), e.enter("atxHeadingSequence"), o(c);
  }
  function o(c) {
    return c === 35 && i++ < 6 ? (e.consume(c), o) : c === null || ke(c) ? (e.exit("atxHeadingSequence"), r.interrupt ? n(c) : u(c)) : t(c);
  }
  function u(c) {
    return c === 35 ? (e.enter("atxHeadingSequence"), a(c)) : c === null || N(c) ? (e.exit("atxHeading"), n(c)) : pe(c) ? ne(e, u, "whitespace")(c) : (e.enter("atxHeadingText"), f(c));
  }
  function a(c) {
    return c === 35 ? (e.consume(c), a) : (e.exit("atxHeadingSequence"), u(c));
  }
  function f(c) {
    return c === null || c === 35 || ke(c) ? (e.exit("atxHeadingText"), u(c)) : (e.consume(c), f);
  }
}
const nl = [
  "address",
  "article",
  "aside",
  "base",
  "basefont",
  "blockquote",
  "body",
  "caption",
  "center",
  "col",
  "colgroup",
  "dd",
  "details",
  "dialog",
  "dir",
  "div",
  "dl",
  "dt",
  "fieldset",
  "figcaption",
  "figure",
  "footer",
  "form",
  "frame",
  "frameset",
  "h1",
  "h2",
  "h3",
  "h4",
  "h5",
  "h6",
  "head",
  "header",
  "hr",
  "html",
  "iframe",
  "legend",
  "li",
  "link",
  "main",
  "menu",
  "menuitem",
  "nav",
  "noframes",
  "ol",
  "optgroup",
  "option",
  "p",
  "param",
  "section",
  "summary",
  "table",
  "tbody",
  "td",
  "tfoot",
  "th",
  "thead",
  "title",
  "tr",
  "track",
  "ul"
], gt = ["pre", "script", "style", "textarea"], tl = {
  name: "htmlFlow",
  tokenize: ll,
  resolveTo: il,
  concrete: !0
}, rl = {
  tokenize: ol,
  partial: !0
};
function il(e) {
  let n = e.length;
  for (; n-- && !(e[n][0] === "enter" && e[n][1].type === "htmlFlow"); )
    ;
  return n > 1 && e[n - 2][1].type === "linePrefix" && (e[n][1].start = e[n - 2][1].start, e[n + 1][1].start = e[n - 2][1].start, e.splice(n - 2, 2)), e;
}
function ll(e, n, t) {
  const r = this;
  let i, l, o, u, a;
  return f;
  function f(p) {
    return e.enter("htmlFlow"), e.enter("htmlFlowData"), e.consume(p), c;
  }
  function c(p) {
    return p === 33 ? (e.consume(p), h) : p === 47 ? (e.consume(p), d) : p === 63 ? (e.consume(p), i = 3, r.interrupt ? n : ae) : ve(p) ? (e.consume(p), o = String.fromCharCode(p), l = !0, y) : t(p);
  }
  function h(p) {
    return p === 45 ? (e.consume(p), i = 2, g) : p === 91 ? (e.consume(p), i = 5, o = "CDATA[", u = 0, m) : ve(p) ? (e.consume(p), i = 4, r.interrupt ? n : ae) : t(p);
  }
  function g(p) {
    return p === 45 ? (e.consume(p), r.interrupt ? n : ae) : t(p);
  }
  function m(p) {
    return p === o.charCodeAt(u++) ? (e.consume(p), u === o.length ? r.interrupt ? n : P : m) : t(p);
  }
  function d(p) {
    return ve(p) ? (e.consume(p), o = String.fromCharCode(p), y) : t(p);
  }
  function y(p) {
    return p === null || p === 47 || p === 62 || ke(p) ? p !== 47 && l && gt.includes(o.toLowerCase()) ? (i = 1, r.interrupt ? n(p) : P(p)) : nl.includes(o.toLowerCase()) ? (i = 6, p === 47 ? (e.consume(p), x) : r.interrupt ? n(p) : P(p)) : (i = 7, r.interrupt && !r.parser.lazy[r.now().line] ? t(p) : l ? z(p) : b(p)) : p === 45 || xe(p) ? (e.consume(p), o += String.fromCharCode(p), y) : t(p);
  }
  function x(p) {
    return p === 62 ? (e.consume(p), r.interrupt ? n : P) : t(p);
  }
  function b(p) {
    return pe(p) ? (e.consume(p), b) : Y(p);
  }
  function z(p) {
    return p === 47 ? (e.consume(p), Y) : p === 58 || p === 95 || ve(p) ? (e.consume(p), v) : pe(p) ? (e.consume(p), z) : Y(p);
  }
  function v(p) {
    return p === 45 || p === 46 || p === 58 || p === 95 || xe(p) ? (e.consume(p), v) : F(p);
  }
  function F(p) {
    return p === 61 ? (e.consume(p), S) : pe(p) ? (e.consume(p), F) : z(p);
  }
  function S(p) {
    return p === null || p === 60 || p === 61 || p === 62 || p === 96 ? t(p) : p === 34 || p === 39 ? (e.consume(p), a = p, k) : pe(p) ? (e.consume(p), S) : (a = null, _(p));
  }
  function k(p) {
    return p === null || N(p) ? t(p) : p === a ? (e.consume(p), U) : (e.consume(p), k);
  }
  function _(p) {
    return p === null || p === 34 || p === 39 || p === 60 || p === 61 || p === 62 || p === 96 || ke(p) ? F(p) : (e.consume(p), _);
  }
  function U(p) {
    return p === 47 || p === 62 || pe(p) ? z(p) : t(p);
  }
  function Y(p) {
    return p === 62 ? (e.consume(p), le) : t(p);
  }
  function le(p) {
    return pe(p) ? (e.consume(p), le) : p === null || N(p) ? P(p) : t(p);
  }
  function P(p) {
    return p === 45 && i === 2 ? (e.consume(p), he) : p === 60 && i === 1 ? (e.consume(p), oe) : p === 62 && i === 4 ? (e.consume(p), E) : p === 63 && i === 3 ? (e.consume(p), ae) : p === 93 && i === 5 ? (e.consume(p), re) : N(p) && (i === 6 || i === 7) ? e.check(
      rl,
      E,
      R
    )(p) : p === null || N(p) ? R(p) : (e.consume(p), P);
  }
  function R(p) {
    return e.exit("htmlFlowData"), X(p);
  }
  function X(p) {
    return p === null ? s(p) : N(p) ? e.attempt(
      {
        tokenize: te,
        partial: !0
      },
      X,
      s
    )(p) : (e.enter("htmlFlowData"), P(p));
  }
  function te(p, B, D) {
    return q;
    function q(L) {
      return p.enter("lineEnding"), p.consume(L), p.exit("lineEnding"), O;
    }
    function O(L) {
      return r.parser.lazy[r.now().line] ? D(L) : B(L);
    }
  }
  function he(p) {
    return p === 45 ? (e.consume(p), ae) : P(p);
  }
  function oe(p) {
    return p === 47 ? (e.consume(p), o = "", ue) : P(p);
  }
  function ue(p) {
    return p === 62 && gt.includes(o.toLowerCase()) ? (e.consume(p), E) : ve(p) && o.length < 8 ? (e.consume(p), o += String.fromCharCode(p), ue) : P(p);
  }
  function re(p) {
    return p === 93 ? (e.consume(p), ae) : P(p);
  }
  function ae(p) {
    return p === 62 ? (e.consume(p), E) : p === 45 && i === 2 ? (e.consume(p), ae) : P(p);
  }
  function E(p) {
    return p === null || N(p) ? (e.exit("htmlFlowData"), s(p)) : (e.consume(p), E);
  }
  function s(p) {
    return e.exit("htmlFlow"), n(p);
  }
}
function ol(e, n, t) {
  return r;
  function r(i) {
    return e.exit("htmlFlowData"), e.enter("lineEndingBlank"), e.consume(i), e.exit("lineEndingBlank"), e.attempt(tn, n, t);
  }
}
const ul = {
  name: "htmlText",
  tokenize: al
};
function al(e, n, t) {
  const r = this;
  let i, l, o, u;
  return a;
  function a(s) {
    return e.enter("htmlText"), e.enter("htmlTextData"), e.consume(s), f;
  }
  function f(s) {
    return s === 33 ? (e.consume(s), c) : s === 47 ? (e.consume(s), _) : s === 63 ? (e.consume(s), S) : ve(s) ? (e.consume(s), le) : t(s);
  }
  function c(s) {
    return s === 45 ? (e.consume(s), h) : s === 91 ? (e.consume(s), l = "CDATA[", o = 0, x) : ve(s) ? (e.consume(s), F) : t(s);
  }
  function h(s) {
    return s === 45 ? (e.consume(s), g) : t(s);
  }
  function g(s) {
    return s === null || s === 62 ? t(s) : s === 45 ? (e.consume(s), m) : d(s);
  }
  function m(s) {
    return s === null || s === 62 ? t(s) : d(s);
  }
  function d(s) {
    return s === null ? t(s) : s === 45 ? (e.consume(s), y) : N(s) ? (u = d, re(s)) : (e.consume(s), d);
  }
  function y(s) {
    return s === 45 ? (e.consume(s), E) : d(s);
  }
  function x(s) {
    return s === l.charCodeAt(o++) ? (e.consume(s), o === l.length ? b : x) : t(s);
  }
  function b(s) {
    return s === null ? t(s) : s === 93 ? (e.consume(s), z) : N(s) ? (u = b, re(s)) : (e.consume(s), b);
  }
  function z(s) {
    return s === 93 ? (e.consume(s), v) : b(s);
  }
  function v(s) {
    return s === 62 ? E(s) : s === 93 ? (e.consume(s), v) : b(s);
  }
  function F(s) {
    return s === null || s === 62 ? E(s) : N(s) ? (u = F, re(s)) : (e.consume(s), F);
  }
  function S(s) {
    return s === null ? t(s) : s === 63 ? (e.consume(s), k) : N(s) ? (u = S, re(s)) : (e.consume(s), S);
  }
  function k(s) {
    return s === 62 ? E(s) : S(s);
  }
  function _(s) {
    return ve(s) ? (e.consume(s), U) : t(s);
  }
  function U(s) {
    return s === 45 || xe(s) ? (e.consume(s), U) : Y(s);
  }
  function Y(s) {
    return N(s) ? (u = Y, re(s)) : pe(s) ? (e.consume(s), Y) : E(s);
  }
  function le(s) {
    return s === 45 || xe(s) ? (e.consume(s), le) : s === 47 || s === 62 || ke(s) ? P(s) : t(s);
  }
  function P(s) {
    return s === 47 ? (e.consume(s), E) : s === 58 || s === 95 || ve(s) ? (e.consume(s), R) : N(s) ? (u = P, re(s)) : pe(s) ? (e.consume(s), P) : E(s);
  }
  function R(s) {
    return s === 45 || s === 46 || s === 58 || s === 95 || xe(s) ? (e.consume(s), R) : X(s);
  }
  function X(s) {
    return s === 61 ? (e.consume(s), te) : N(s) ? (u = X, re(s)) : pe(s) ? (e.consume(s), X) : P(s);
  }
  function te(s) {
    return s === null || s === 60 || s === 61 || s === 62 || s === 96 ? t(s) : s === 34 || s === 39 ? (e.consume(s), i = s, he) : N(s) ? (u = te, re(s)) : pe(s) ? (e.consume(s), te) : (e.consume(s), i = void 0, ue);
  }
  function he(s) {
    return s === i ? (e.consume(s), oe) : s === null ? t(s) : N(s) ? (u = he, re(s)) : (e.consume(s), he);
  }
  function oe(s) {
    return s === 62 || s === 47 || ke(s) ? P(s) : t(s);
  }
  function ue(s) {
    return s === null || s === 34 || s === 39 || s === 60 || s === 61 || s === 96 ? t(s) : s === 62 || ke(s) ? P(s) : (e.consume(s), ue);
  }
  function re(s) {
    return e.exit("htmlTextData"), e.enter("lineEnding"), e.consume(s), e.exit("lineEnding"), ne(
      e,
      ae,
      "linePrefix",
      r.parser.constructs.disable.null.includes("codeIndented") ? void 0 : 4
    );
  }
  function ae(s) {
    return e.enter("htmlTextData"), u(s);
  }
  function E(s) {
    return s === 62 ? (e.consume(s), e.exit("htmlTextData"), e.exit("htmlText"), n) : t(s);
  }
}
const Mn = {
  name: "labelEnd",
  tokenize: dl,
  resolveTo: hl,
  resolveAll: pl
}, sl = {
  tokenize: ml
}, cl = {
  tokenize: gl
}, fl = {
  tokenize: yl
};
function pl(e) {
  let n = -1, t;
  for (; ++n < e.length; )
    t = e[n][1], (t.type === "labelImage" || t.type === "labelLink" || t.type === "labelEnd") && (e.splice(n + 1, t.type === "labelImage" ? 4 : 2), t.type = "data", n++);
  return e;
}
function hl(e, n) {
  let t = e.length, r = 0, i, l, o, u;
  for (; t--; )
    if (i = e[t][1], l) {
      if (i.type === "link" || i.type === "labelLink" && i._inactive)
        break;
      e[t][0] === "enter" && i.type === "labelLink" && (i._inactive = !0);
    } else if (o) {
      if (e[t][0] === "enter" && (i.type === "labelImage" || i.type === "labelLink") && !i._balanced && (l = t, i.type !== "labelLink")) {
        r = 2;
        break;
      }
    } else
      i.type === "labelEnd" && (o = t);
  const a = {
    type: e[l][1].type === "labelLink" ? "link" : "image",
    start: Object.assign({}, e[l][1].start),
    end: Object.assign({}, e[e.length - 1][1].end)
  }, f = {
    type: "label",
    start: Object.assign({}, e[l][1].start),
    end: Object.assign({}, e[o][1].end)
  }, c = {
    type: "labelText",
    start: Object.assign({}, e[l + r + 2][1].end),
    end: Object.assign({}, e[o - 2][1].start)
  };
  return u = [
    ["enter", a, n],
    ["enter", f, n]
  ], u = be(u, e.slice(l + 1, l + r + 3)), u = be(u, [["enter", c, n]]), u = be(
    u,
    Dn(
      n.parser.constructs.insideSpan.null,
      e.slice(l + r + 4, o - 3),
      n
    )
  ), u = be(u, [
    ["exit", c, n],
    e[o - 2],
    e[o - 1],
    ["exit", f, n]
  ]), u = be(u, e.slice(o + 1)), u = be(u, [["exit", a, n]]), Te(e, l, e.length, u), e;
}
function dl(e, n, t) {
  const r = this;
  let i = r.events.length, l, o;
  for (; i--; )
    if ((r.events[i][1].type === "labelImage" || r.events[i][1].type === "labelLink") && !r.events[i][1]._balanced) {
      l = r.events[i][1];
      break;
    }
  return u;
  function u(c) {
    return l ? l._inactive ? f(c) : (o = r.parser.defined.includes(
      Me(
        r.sliceSerialize({
          start: l.end,
          end: r.now()
        })
      )
    ), e.enter("labelEnd"), e.enter("labelMarker"), e.consume(c), e.exit("labelMarker"), e.exit("labelEnd"), a) : t(c);
  }
  function a(c) {
    return c === 40 ? e.attempt(
      sl,
      n,
      o ? n : f
    )(c) : c === 91 ? e.attempt(
      cl,
      n,
      o ? e.attempt(fl, n, f) : f
    )(c) : o ? n(c) : f(c);
  }
  function f(c) {
    return l._balanced = !0, t(c);
  }
}
function ml(e, n, t) {
  return r;
  function r(a) {
    return e.enter("resource"), e.enter("resourceMarker"), e.consume(a), e.exit("resourceMarker"), Ve(e, i);
  }
  function i(a) {
    return a === 41 ? u(a) : ir(
      e,
      l,
      t,
      "resourceDestination",
      "resourceDestinationLiteral",
      "resourceDestinationLiteralMarker",
      "resourceDestinationRaw",
      "resourceDestinationString",
      32
    )(a);
  }
  function l(a) {
    return ke(a) ? Ve(e, o)(a) : u(a);
  }
  function o(a) {
    return a === 34 || a === 39 || a === 40 ? or(
      e,
      Ve(e, u),
      t,
      "resourceTitle",
      "resourceTitleMarker",
      "resourceTitleString"
    )(a) : u(a);
  }
  function u(a) {
    return a === 41 ? (e.enter("resourceMarker"), e.consume(a), e.exit("resourceMarker"), e.exit("resource"), n) : t(a);
  }
}
function gl(e, n, t) {
  const r = this;
  return i;
  function i(o) {
    return lr.call(
      r,
      e,
      l,
      t,
      "reference",
      "referenceMarker",
      "referenceString"
    )(o);
  }
  function l(o) {
    return r.parser.defined.includes(
      Me(
        r.sliceSerialize(r.events[r.events.length - 1][1]).slice(1, -1)
      )
    ) ? n(o) : t(o);
  }
}
function yl(e, n, t) {
  return r;
  function r(l) {
    return e.enter("reference"), e.enter("referenceMarker"), e.consume(l), e.exit("referenceMarker"), i;
  }
  function i(l) {
    return l === 93 ? (e.enter("referenceMarker"), e.consume(l), e.exit("referenceMarker"), e.exit("reference"), n) : t(l);
  }
}
const xl = {
  name: "labelStartImage",
  tokenize: bl,
  resolveAll: Mn.resolveAll
};
function bl(e, n, t) {
  const r = this;
  return i;
  function i(u) {
    return e.enter("labelImage"), e.enter("labelImageMarker"), e.consume(u), e.exit("labelImageMarker"), l;
  }
  function l(u) {
    return u === 91 ? (e.enter("labelMarker"), e.consume(u), e.exit("labelMarker"), e.exit("labelImage"), o) : t(u);
  }
  function o(u) {
    return u === 94 && "_hiddenFootnoteSupport" in r.parser.constructs ? t(u) : n(u);
  }
}
const kl = {
  name: "labelStartLink",
  tokenize: wl,
  resolveAll: Mn.resolveAll
};
function wl(e, n, t) {
  const r = this;
  return i;
  function i(o) {
    return e.enter("labelLink"), e.enter("labelMarker"), e.consume(o), e.exit("labelMarker"), e.exit("labelLink"), l;
  }
  function l(o) {
    return o === 94 && "_hiddenFootnoteSupport" in r.parser.constructs ? t(o) : n(o);
  }
}
const dn = {
  name: "lineEnding",
  tokenize: Sl
};
function Sl(e, n) {
  return t;
  function t(r) {
    return e.enter("lineEnding"), e.consume(r), e.exit("lineEnding"), ne(e, n, "linePrefix");
  }
}
const Je = {
  name: "thematicBreak",
  tokenize: El
};
function El(e, n, t) {
  let r = 0, i;
  return l;
  function l(a) {
    return e.enter("thematicBreak"), i = a, o(a);
  }
  function o(a) {
    return a === i ? (e.enter("thematicBreakSequence"), u(a)) : pe(a) ? ne(e, o, "whitespace")(a) : r < 3 || a !== null && !N(a) ? t(a) : (e.exit("thematicBreak"), n(a));
  }
  function u(a) {
    return a === i ? (e.consume(a), r++, u) : (e.exit("thematicBreakSequence"), o(a));
  }
}
const me = {
  name: "list",
  tokenize: Tl,
  continuation: {
    tokenize: Pl
  },
  exit: Ol
}, Cl = {
  tokenize: Il,
  partial: !0
}, vl = {
  tokenize: Al,
  partial: !0
};
function Tl(e, n, t) {
  const r = this, i = r.events[r.events.length - 1];
  let l = i && i[1].type === "linePrefix" ? i[2].sliceSerialize(i[1], !0).length : 0, o = 0;
  return u;
  function u(m) {
    const d = r.containerState.type || (m === 42 || m === 43 || m === 45 ? "listUnordered" : "listOrdered");
    if (d === "listUnordered" ? !r.containerState.marker || m === r.containerState.marker : Tn(m)) {
      if (r.containerState.type || (r.containerState.type = d, e.enter(d, {
        _container: !0
      })), d === "listUnordered")
        return e.enter("listItemPrefix"), m === 42 || m === 45 ? e.check(Je, t, f)(m) : f(m);
      if (!r.interrupt || m === 49)
        return e.enter("listItemPrefix"), e.enter("listItemValue"), a(m);
    }
    return t(m);
  }
  function a(m) {
    return Tn(m) && ++o < 10 ? (e.consume(m), a) : (!r.interrupt || o < 2) && (r.containerState.marker ? m === r.containerState.marker : m === 41 || m === 46) ? (e.exit("listItemValue"), f(m)) : t(m);
  }
  function f(m) {
    return e.enter("listItemMarker"), e.consume(m), e.exit("listItemMarker"), r.containerState.marker = r.containerState.marker || m, e.check(
      tn,
      // Can’t be empty when interrupting.
      r.interrupt ? t : c,
      e.attempt(
        Cl,
        g,
        h
      )
    );
  }
  function c(m) {
    return r.containerState.initialBlankLine = !0, l++, g(m);
  }
  function h(m) {
    return pe(m) ? (e.enter("listItemPrefixWhitespace"), e.consume(m), e.exit("listItemPrefixWhitespace"), g) : t(m);
  }
  function g(m) {
    return r.containerState.size = l + r.sliceSerialize(e.exit("listItemPrefix"), !0).length, n(m);
  }
}
function Pl(e, n, t) {
  const r = this;
  return r.containerState._closeFlow = void 0, e.check(tn, i, l);
  function i(u) {
    return r.containerState.furtherBlankLines = r.containerState.furtherBlankLines || r.containerState.initialBlankLine, ne(
      e,
      n,
      "listItemIndent",
      r.containerState.size + 1
    )(u);
  }
  function l(u) {
    return r.containerState.furtherBlankLines || !pe(u) ? (r.containerState.furtherBlankLines = void 0, r.containerState.initialBlankLine = void 0, o(u)) : (r.containerState.furtherBlankLines = void 0, r.containerState.initialBlankLine = void 0, e.attempt(vl, n, o)(u));
  }
  function o(u) {
    return r.containerState._closeFlow = !0, r.interrupt = void 0, ne(
      e,
      e.attempt(me, n, t),
      "linePrefix",
      r.parser.constructs.disable.null.includes("codeIndented") ? void 0 : 4
    )(u);
  }
}
function Al(e, n, t) {
  const r = this;
  return ne(
    e,
    i,
    "listItemIndent",
    r.containerState.size + 1
  );
  function i(l) {
    const o = r.events[r.events.length - 1];
    return o && o[1].type === "listItemIndent" && o[2].sliceSerialize(o[1], !0).length === r.containerState.size ? n(l) : t(l);
  }
}
function Ol(e) {
  e.exit(this.containerState.type);
}
function Il(e, n, t) {
  const r = this;
  return ne(
    e,
    i,
    "listItemPrefixWhitespace",
    r.parser.constructs.disable.null.includes("codeIndented") ? void 0 : 4 + 1
  );
  function i(l) {
    const o = r.events[r.events.length - 1];
    return !pe(l) && o && o[1].type === "listItemPrefixWhitespace" ? n(l) : t(l);
  }
}
const yt = {
  name: "setextUnderline",
  tokenize: Rl,
  resolveTo: Fl
};
function Fl(e, n) {
  let t = e.length, r, i, l;
  for (; t--; )
    if (e[t][0] === "enter") {
      if (e[t][1].type === "content") {
        r = t;
        break;
      }
      e[t][1].type === "paragraph" && (i = t);
    } else
      e[t][1].type === "content" && e.splice(t, 1), !l && e[t][1].type === "definition" && (l = t);
  const o = {
    type: "setextHeading",
    start: Object.assign({}, e[i][1].start),
    end: Object.assign({}, e[e.length - 1][1].end)
  };
  return e[i][1].type = "setextHeadingText", l ? (e.splice(i, 0, ["enter", o, n]), e.splice(l + 1, 0, ["exit", e[r][1], n]), e[r][1].end = Object.assign({}, e[l][1].end)) : e[r][1] = o, e.push(["exit", o, n]), e;
}
function Rl(e, n, t) {
  const r = this;
  let i = r.events.length, l, o;
  for (; i--; )
    if (r.events[i][1].type !== "lineEnding" && r.events[i][1].type !== "linePrefix" && r.events[i][1].type !== "content") {
      o = r.events[i][1].type === "paragraph";
      break;
    }
  return u;
  function u(c) {
    return !r.parser.lazy[r.now().line] && (r.interrupt || o) ? (e.enter("setextHeadingLine"), e.enter("setextHeadingLineSequence"), l = c, a(c)) : t(c);
  }
  function a(c) {
    return c === l ? (e.consume(c), a) : (e.exit("setextHeadingLineSequence"), ne(e, f, "lineSuffix")(c));
  }
  function f(c) {
    return c === null || N(c) ? (e.exit("setextHeadingLine"), n(c)) : t(c);
  }
}
const _l = {
  tokenize: Ll
};
function Ll(e) {
  const n = this, t = e.attempt(
    // Try to parse a blank line.
    tn,
    r,
    // Try to parse initial flow (essentially, only code).
    e.attempt(
      this.parser.constructs.flowInitial,
      i,
      ne(
        e,
        e.attempt(
          this.parser.constructs.flow,
          i,
          e.attempt($i, i)
        ),
        "linePrefix"
      )
    )
  );
  return t;
  function r(l) {
    if (l === null) {
      e.consume(l);
      return;
    }
    return e.enter("lineEndingBlank"), e.consume(l), e.exit("lineEndingBlank"), n.currentConstruct = void 0, t;
  }
  function i(l) {
    if (l === null) {
      e.consume(l);
      return;
    }
    return e.enter("lineEnding"), e.consume(l), e.exit("lineEnding"), n.currentConstruct = void 0, t;
  }
}
const Dl = {
  resolveAll: ar()
}, zl = ur("string"), Ml = ur("text");
function ur(e) {
  return {
    tokenize: n,
    resolveAll: ar(
      e === "text" ? Bl : void 0
    )
  };
  function n(t) {
    const r = this, i = this.parser.constructs[e], l = t.attempt(i, o, u);
    return o;
    function o(c) {
      return f(c) ? l(c) : u(c);
    }
    function u(c) {
      if (c === null) {
        t.consume(c);
        return;
      }
      return t.enter("data"), t.consume(c), a;
    }
    function a(c) {
      return f(c) ? (t.exit("data"), l(c)) : (t.consume(c), a);
    }
    function f(c) {
      if (c === null)
        return !0;
      const h = i[c];
      let g = -1;
      if (h)
        for (; ++g < h.length; ) {
          const m = h[g];
          if (!m.previous || m.previous.call(r, r.previous))
            return !0;
        }
      return !1;
    }
  }
}
function ar(e) {
  return n;
  function n(t, r) {
    let i = -1, l;
    for (; ++i <= t.length; )
      l === void 0 ? t[i] && t[i][1].type === "data" && (l = i, i++) : (!t[i] || t[i][1].type !== "data") && (i !== l + 2 && (t[l][1].end = t[i - 1][1].end, t.splice(l + 2, i - l - 2), i = l + 2), l = void 0);
    return e ? e(t, r) : t;
  }
}
function Bl(e, n) {
  let t = 0;
  for (; ++t <= e.length; )
    if ((t === e.length || e[t][1].type === "lineEnding") && e[t - 1][1].type === "data") {
      const r = e[t - 1][1], i = n.sliceStream(r);
      let l = i.length, o = -1, u = 0, a;
      for (; l--; ) {
        const f = i[l];
        if (typeof f == "string") {
          for (o = f.length; f.charCodeAt(o - 1) === 32; )
            u++, o--;
          if (o)
            break;
          o = -1;
        } else if (f === -2)
          a = !0, u++;
        else if (f !== -1) {
          l++;
          break;
        }
      }
      if (u) {
        const f = {
          type: t === e.length || a || u < 2 ? "lineSuffix" : "hardBreakTrailing",
          start: {
            line: r.end.line,
            column: r.end.column - u,
            offset: r.end.offset - u,
            _index: r.start._index + l,
            _bufferIndex: l ? o : r.start._bufferIndex + o
          },
          end: Object.assign({}, r.end)
        };
        r.end = Object.assign({}, f.start), r.start.offset === r.end.offset ? Object.assign(r, f) : (e.splice(
          t,
          0,
          ["enter", f, n],
          ["exit", f, n]
        ), t += 2);
      }
      t++;
    }
  return e;
}
function Nl(e, n, t) {
  let r = Object.assign(
    t ? Object.assign({}, t) : {
      line: 1,
      column: 1,
      offset: 0
    },
    {
      _index: 0,
      _bufferIndex: -1
    }
  );
  const i = {}, l = [];
  let o = [], u = [];
  const a = {
    consume: z,
    enter: v,
    exit: F,
    attempt: _(S),
    check: _(k),
    interrupt: _(k, {
      interrupt: !0
    })
  }, f = {
    previous: null,
    code: null,
    containerState: {},
    events: [],
    parser: e,
    sliceStream: m,
    sliceSerialize: g,
    now: d,
    defineSkip: y,
    write: h
  };
  let c = n.tokenize.call(f, a);
  return n.resolveAll && l.push(n), f;
  function h(P) {
    return o = be(o, P), x(), o[o.length - 1] !== null ? [] : (U(n, 0), f.events = Dn(l, f.events, f), f.events);
  }
  function g(P, R) {
    return $l(m(P), R);
  }
  function m(P) {
    return jl(o, P);
  }
  function d() {
    return Object.assign({}, r);
  }
  function y(P) {
    i[P.line] = P.column, le();
  }
  function x() {
    let P;
    for (; r._index < o.length; ) {
      const R = o[r._index];
      if (typeof R == "string")
        for (P = r._index, r._bufferIndex < 0 && (r._bufferIndex = 0); r._index === P && r._bufferIndex < R.length; )
          b(R.charCodeAt(r._bufferIndex));
      else
        b(R);
    }
  }
  function b(P) {
    c = c(P);
  }
  function z(P) {
    N(P) ? (r.line++, r.column = 1, r.offset += P === -3 ? 2 : 1, le()) : P !== -1 && (r.column++, r.offset++), r._bufferIndex < 0 ? r._index++ : (r._bufferIndex++, r._bufferIndex === o[r._index].length && (r._bufferIndex = -1, r._index++)), f.previous = P;
  }
  function v(P, R) {
    const X = R || {};
    return X.type = P, X.start = d(), f.events.push(["enter", X, f]), u.push(X), X;
  }
  function F(P) {
    const R = u.pop();
    return R.end = d(), f.events.push(["exit", R, f]), R;
  }
  function S(P, R) {
    U(P, R.from);
  }
  function k(P, R) {
    R.restore();
  }
  function _(P, R) {
    return X;
    function X(te, he, oe) {
      let ue, re, ae, E;
      return Array.isArray(te) ? (
        /* c8 ignore next 1 */
        p(te)
      ) : "tokenize" in te ? p([te]) : s(te);
      function s(O) {
        return L;
        function L($) {
          const W = $ !== null && O[$], H = $ !== null && O.null, ce = [
            // To do: add more extension tests.
            /* c8 ignore next 2 */
            ...Array.isArray(W) ? W : W ? [W] : [],
            ...Array.isArray(H) ? H : H ? [H] : []
          ];
          return p(ce)($);
        }
      }
      function p(O) {
        return ue = O, re = 0, O.length === 0 ? oe : B(O[re]);
      }
      function B(O) {
        return L;
        function L($) {
          return E = Y(), ae = O, O.partial || (f.currentConstruct = O), O.name && f.parser.constructs.disable.null.includes(O.name) ? q() : O.tokenize.call(
            // If we do have fields, create an object w/ `context` as its
            // prototype.
            // This allows a “live binding”, which is needed for `interrupt`.
            R ? Object.assign(Object.create(f), R) : f,
            a,
            D,
            q
          )($);
        }
      }
      function D(O) {
        return P(ae, E), he;
      }
      function q(O) {
        return E.restore(), ++re < ue.length ? B(ue[re]) : oe;
      }
    }
  }
  function U(P, R) {
    P.resolveAll && !l.includes(P) && l.push(P), P.resolve && Te(
      f.events,
      R,
      f.events.length - R,
      P.resolve(f.events.slice(R), f)
    ), P.resolveTo && (f.events = P.resolveTo(f.events, f));
  }
  function Y() {
    const P = d(), R = f.previous, X = f.currentConstruct, te = f.events.length, he = Array.from(u);
    return {
      restore: oe,
      from: te
    };
    function oe() {
      r = P, f.previous = R, f.currentConstruct = X, f.events.length = te, u = he, le();
    }
  }
  function le() {
    r.line in i && r.column < 2 && (r.column = i[r.line], r.offset += i[r.line] - 1);
  }
}
function jl(e, n) {
  const t = n.start._index, r = n.start._bufferIndex, i = n.end._index, l = n.end._bufferIndex;
  let o;
  return t === i ? o = [e[t].slice(r, l)] : (o = e.slice(t, i), r > -1 && (o[0] = o[0].slice(r)), l > 0 && o.push(e[i].slice(0, l))), o;
}
function $l(e, n) {
  let t = -1;
  const r = [];
  let i;
  for (; ++t < e.length; ) {
    const l = e[t];
    let o;
    if (typeof l == "string")
      o = l;
    else
      switch (l) {
        case -5: {
          o = "\r";
          break;
        }
        case -4: {
          o = `
`;
          break;
        }
        case -3: {
          o = `\r
`;
          break;
        }
        case -2: {
          o = n ? " " : "	";
          break;
        }
        case -1: {
          if (!n && i)
            continue;
          o = " ";
          break;
        }
        default:
          o = String.fromCharCode(l);
      }
    i = l === -2, r.push(o);
  }
  return r.join("");
}
const Ul = {
  [42]: me,
  [43]: me,
  [45]: me,
  [48]: me,
  [49]: me,
  [50]: me,
  [51]: me,
  [52]: me,
  [53]: me,
  [54]: me,
  [55]: me,
  [56]: me,
  [57]: me,
  [62]: er
}, ql = {
  [91]: Yi
}, Hl = {
  [-2]: hn,
  [-1]: hn,
  [32]: hn
}, Vl = {
  [35]: Zi,
  [42]: Je,
  [45]: [yt, Je],
  [60]: tl,
  [61]: yt,
  [95]: Je,
  [96]: mt,
  [126]: mt
}, Yl = {
  [38]: tr,
  [92]: nr
}, Wl = {
  [-5]: dn,
  [-4]: dn,
  [-3]: dn,
  [33]: xl,
  [38]: tr,
  [42]: An,
  [60]: [Ci, ul],
  [91]: kl,
  [92]: [Ki, nr],
  [93]: Mn,
  [95]: An,
  [96]: zi
}, Xl = {
  null: [An, Dl]
}, Ql = {
  null: [42, 95]
}, Kl = {
  null: []
}, Gl = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  attentionMarkers: Ql,
  contentInitial: ql,
  disable: Kl,
  document: Ul,
  flow: Vl,
  flowInitial: Hl,
  insideSpan: Xl,
  string: Yl,
  text: Wl
}, Symbol.toStringTag, { value: "Module" }));
function Zl(e = {}) {
  const n = si(
    // @ts-expect-error Same as above.
    [Gl].concat(e.extensions || [])
  ), t = {
    defined: [],
    lazy: {},
    constructs: n,
    content: r(yi),
    document: r(bi),
    flow: r(_l),
    string: r(zl),
    text: r(Ml)
  };
  return t;
  function r(i) {
    return l;
    function l(o) {
      return Nl(t, i, o);
    }
  }
}
const xt = /[\0\t\n\r]/g;
function Jl() {
  let e = 1, n = "", t = !0, r;
  return i;
  function i(l, o, u) {
    const a = [];
    let f, c, h, g, m;
    for (l = n + l.toString(o), h = 0, n = "", t && (l.charCodeAt(0) === 65279 && h++, t = void 0); h < l.length; ) {
      if (xt.lastIndex = h, f = xt.exec(l), g = f && f.index !== void 0 ? f.index : l.length, m = l.charCodeAt(g), !f) {
        n = l.slice(h);
        break;
      }
      if (m === 10 && h === g && r)
        a.push(-3), r = void 0;
      else
        switch (r && (a.push(-5), r = void 0), h < g && (a.push(l.slice(h, g)), e += g - h), m) {
          case 0: {
            a.push(65533), e++;
            break;
          }
          case 9: {
            for (c = Math.ceil(e / 4) * 4, a.push(-2); e++ < c; )
              a.push(-1);
            break;
          }
          case 10: {
            a.push(-4), e = 1;
            break;
          }
          default:
            r = !0, e = 1;
        }
      h = g + 1;
    }
    return u && (r && a.push(-5), n && a.push(n), a.push(null)), a;
  }
}
function eo(e) {
  for (; !rr(e); )
    ;
  return e;
}
function sr(e, n) {
  const t = Number.parseInt(e, n);
  return (
    // C0 except for HT, LF, FF, CR, space
    t < 9 || t === 11 || t > 13 && t < 32 || // Control character (DEL) of the basic block and C1 controls.
    t > 126 && t < 160 || // Lone high surrogates and low surrogates.
    t > 55295 && t < 57344 || // Noncharacters.
    t > 64975 && t < 65008 || (t & 65535) === 65535 || (t & 65535) === 65534 || // Out of range
    t > 1114111 ? "�" : String.fromCharCode(t)
  );
}
const no = /\\([!-/:-@[-`{-~])|&(#(?:\d{1,7}|x[\da-f]{1,6})|[\da-z]{1,31});/gi;
function to(e) {
  return e.replace(no, ro);
}
function ro(e, n, t) {
  if (n)
    return n;
  if (t.charCodeAt(0) === 35) {
    const i = t.charCodeAt(1), l = i === 120 || i === 88;
    return sr(t.slice(l ? 2 : 1), l ? 16 : 10);
  }
  return zn(t) || e;
}
function en(e) {
  return !e || typeof e != "object" ? "" : "position" in e || "type" in e ? bt(e.position) : "start" in e || "end" in e ? bt(e) : "line" in e || "column" in e ? On(e) : "";
}
function On(e) {
  return kt(e && e.line) + ":" + kt(e && e.column);
}
function bt(e) {
  return On(e && e.start) + "-" + On(e && e.end);
}
function kt(e) {
  return e && typeof e == "number" ? e : 1;
}
const cr = {}.hasOwnProperty, io = (
  /**
   * @type {(
   *   ((value: Value, encoding: Encoding, options?: Options | null | undefined) => Root) &
   *   ((value: Value, options?: Options | null | undefined) => Root)
   * )}
   */
  /**
   * @param {Value} value
   * @param {Encoding | Options | null | undefined} [encoding]
   * @param {Options | null | undefined} [options]
   * @returns {Root}
   */
  function(e, n, t) {
    return typeof n != "string" && (t = n, n = void 0), lo(t)(
      eo(
        // @ts-expect-error: micromark types need to accept `null`.
        Zl(t).document().write(Jl()(e, n, !0))
      )
    );
  }
);
function lo(e) {
  const n = {
    transforms: [],
    canContainEols: ["emphasis", "fragment", "heading", "paragraph", "strong"],
    enter: {
      autolink: u(Vn),
      autolinkProtocol: P,
      autolinkEmail: P,
      atxHeading: u(_e),
      blockQuote: u(ce),
      characterEscape: P,
      characterReference: P,
      codeFenced: u(A),
      codeFencedFenceInfo: a,
      codeFencedFenceMeta: a,
      codeIndented: u(A, a),
      codeText: u(de, a),
      codeTextData: P,
      data: P,
      codeFlowValue: P,
      definition: u(I),
      definitionDestinationString: a,
      definitionLabelString: a,
      definitionTitleString: a,
      emphasis: u(Oe),
      hardBreakEscape: u($e),
      hardBreakTrailing: u($e),
      htmlFlow: u(Hn, a),
      htmlFlowData: P,
      htmlText: u(Hn, a),
      htmlTextData: P,
      image: u(_r),
      label: a,
      link: u(Vn),
      listItem: u(Lr),
      listItemValue: d,
      listOrdered: u(Yn, m),
      listUnordered: u(Yn),
      paragraph: u(Dr),
      reference: q,
      referenceString: a,
      resourceDestinationString: a,
      resourceTitleString: a,
      setextHeading: u(_e),
      strong: u(zr),
      thematicBreak: u(Br)
    },
    exit: {
      atxHeading: c(),
      atxHeadingSequence: _,
      autolink: c(),
      autolinkEmail: H,
      autolinkProtocol: W,
      blockQuote: c(),
      characterEscapeValue: R,
      characterReferenceMarkerHexadecimal: L,
      characterReferenceMarkerNumeric: L,
      characterReferenceValue: $,
      codeFenced: c(z),
      codeFencedFence: b,
      codeFencedFenceInfo: y,
      codeFencedFenceMeta: x,
      codeFlowValue: R,
      codeIndented: c(v),
      codeText: c(ue),
      codeTextData: R,
      data: R,
      definition: c(),
      definitionDestinationString: k,
      definitionLabelString: F,
      definitionTitleString: S,
      emphasis: c(),
      hardBreakEscape: c(te),
      hardBreakTrailing: c(te),
      htmlFlow: c(he),
      htmlFlowData: R,
      htmlText: c(oe),
      htmlTextData: R,
      image: c(ae),
      label: s,
      labelText: E,
      lineEnding: X,
      link: c(re),
      listItem: c(),
      listOrdered: c(),
      listUnordered: c(),
      paragraph: c(),
      referenceString: O,
      resourceDestinationString: p,
      resourceTitleString: B,
      resource: D,
      setextHeading: c(le),
      setextHeadingLineSequence: Y,
      setextHeadingText: U,
      strong: c(),
      thematicBreak: c()
    }
  };
  fr(n, (e || {}).mdastExtensions || []);
  const t = {};
  return r;
  function r(w) {
    let T = {
      type: "root",
      children: []
    };
    const M = {
      stack: [T],
      tokenStack: [],
      config: n,
      enter: f,
      exit: h,
      buffer: a,
      resume: g,
      setData: l,
      getData: o
    }, J = [];
    let ee = -1;
    for (; ++ee < w.length; )
      if (w[ee][1].type === "listOrdered" || w[ee][1].type === "listUnordered")
        if (w[ee][0] === "enter")
          J.push(ee);
        else {
          const Ee = J.pop();
          ee = i(w, Ee, ee);
        }
    for (ee = -1; ++ee < w.length; ) {
      const Ee = n[w[ee][0]];
      cr.call(Ee, w[ee][1].type) && Ee[w[ee][1].type].call(
        Object.assign(
          {
            sliceSerialize: w[ee][2].sliceSerialize
          },
          M
        ),
        w[ee][1]
      );
    }
    if (M.tokenStack.length > 0) {
      const Ee = M.tokenStack[M.tokenStack.length - 1];
      (Ee[1] || wt).call(M, void 0, Ee[0]);
    }
    for (T.position = {
      start: Fe(
        w.length > 0 ? w[0][1].start : {
          line: 1,
          column: 1,
          offset: 0
        }
      ),
      end: Fe(
        w.length > 0 ? w[w.length - 2][1].end : {
          line: 1,
          column: 1,
          offset: 0
        }
      )
    }, ee = -1; ++ee < n.transforms.length; )
      T = n.transforms[ee](T) || T;
    return T;
  }
  function i(w, T, M) {
    let J = T - 1, ee = -1, Ee = !1, Ie, Pe, Ue, qe;
    for (; ++J <= M; ) {
      const se = w[J];
      if (se[1].type === "listUnordered" || se[1].type === "listOrdered" || se[1].type === "blockQuote" ? (se[0] === "enter" ? ee++ : ee--, qe = void 0) : se[1].type === "lineEndingBlank" ? se[0] === "enter" && (Ie && !qe && !ee && !Ue && (Ue = J), qe = void 0) : se[1].type === "linePrefix" || se[1].type === "listItemValue" || se[1].type === "listItemMarker" || se[1].type === "listItemPrefix" || se[1].type === "listItemPrefixWhitespace" || (qe = void 0), !ee && se[0] === "enter" && se[1].type === "listItemPrefix" || ee === -1 && se[0] === "exit" && (se[1].type === "listUnordered" || se[1].type === "listOrdered")) {
        if (Ie) {
          let on = J;
          for (Pe = void 0; on--; ) {
            const Ae = w[on];
            if (Ae[1].type === "lineEnding" || Ae[1].type === "lineEndingBlank") {
              if (Ae[0] === "exit")
                continue;
              Pe && (w[Pe][1].type = "lineEndingBlank", Ee = !0), Ae[1].type = "lineEnding", Pe = on;
            } else if (!(Ae[1].type === "linePrefix" || Ae[1].type === "blockQuotePrefix" || Ae[1].type === "blockQuotePrefixWhitespace" || Ae[1].type === "blockQuoteMarker" || Ae[1].type === "listItemIndent"))
              break;
          }
          Ue && (!Pe || Ue < Pe) && (Ie._spread = !0), Ie.end = Object.assign(
            {},
            Pe ? w[Pe][1].start : se[1].end
          ), w.splice(Pe || J, 0, ["exit", Ie, se[2]]), J++, M++;
        }
        se[1].type === "listItemPrefix" && (Ie = {
          type: "listItem",
          // @ts-expect-error Patched
          _spread: !1,
          start: Object.assign({}, se[1].start)
        }, w.splice(J, 0, ["enter", Ie, se[2]]), J++, M++, Ue = void 0, qe = !0);
      }
    }
    return w[T][1]._spread = Ee, M;
  }
  function l(w, T) {
    t[w] = T;
  }
  function o(w) {
    return t[w];
  }
  function u(w, T) {
    return M;
    function M(J) {
      f.call(this, w(J), J), T && T.call(this, J);
    }
  }
  function a() {
    this.stack.push({
      type: "fragment",
      children: []
    });
  }
  function f(w, T, M) {
    return this.stack[this.stack.length - 1].children.push(w), this.stack.push(w), this.tokenStack.push([T, M]), w.position = {
      start: Fe(T.start)
    }, w;
  }
  function c(w) {
    return T;
    function T(M) {
      w && w.call(this, M), h.call(this, M);
    }
  }
  function h(w, T) {
    const M = this.stack.pop(), J = this.tokenStack.pop();
    if (J)
      J[0].type !== w.type && (T ? T.call(this, w, J[0]) : (J[1] || wt).call(this, w, J[0]));
    else
      throw new Error(
        "Cannot close `" + w.type + "` (" + en({
          start: w.start,
          end: w.end
        }) + "): it’s not open"
      );
    return M.position.end = Fe(w.end), M;
  }
  function g() {
    return ui(this.stack.pop());
  }
  function m() {
    l("expectingFirstListItemValue", !0);
  }
  function d(w) {
    if (o("expectingFirstListItemValue")) {
      const T = this.stack[this.stack.length - 2];
      T.start = Number.parseInt(this.sliceSerialize(w), 10), l("expectingFirstListItemValue");
    }
  }
  function y() {
    const w = this.resume(), T = this.stack[this.stack.length - 1];
    T.lang = w;
  }
  function x() {
    const w = this.resume(), T = this.stack[this.stack.length - 1];
    T.meta = w;
  }
  function b() {
    o("flowCodeInside") || (this.buffer(), l("flowCodeInside", !0));
  }
  function z() {
    const w = this.resume(), T = this.stack[this.stack.length - 1];
    T.value = w.replace(/^(\r?\n|\r)|(\r?\n|\r)$/g, ""), l("flowCodeInside");
  }
  function v() {
    const w = this.resume(), T = this.stack[this.stack.length - 1];
    T.value = w.replace(/(\r?\n|\r)$/g, "");
  }
  function F(w) {
    const T = this.resume(), M = this.stack[this.stack.length - 1];
    M.label = T, M.identifier = Me(
      this.sliceSerialize(w)
    ).toLowerCase();
  }
  function S() {
    const w = this.resume(), T = this.stack[this.stack.length - 1];
    T.title = w;
  }
  function k() {
    const w = this.resume(), T = this.stack[this.stack.length - 1];
    T.url = w;
  }
  function _(w) {
    const T = this.stack[this.stack.length - 1];
    if (!T.depth) {
      const M = this.sliceSerialize(w).length;
      T.depth = M;
    }
  }
  function U() {
    l("setextHeadingSlurpLineEnding", !0);
  }
  function Y(w) {
    const T = this.stack[this.stack.length - 1];
    T.depth = this.sliceSerialize(w).charCodeAt(0) === 61 ? 1 : 2;
  }
  function le() {
    l("setextHeadingSlurpLineEnding");
  }
  function P(w) {
    const T = this.stack[this.stack.length - 1];
    let M = T.children[T.children.length - 1];
    (!M || M.type !== "text") && (M = Mr(), M.position = {
      start: Fe(w.start)
    }, T.children.push(M)), this.stack.push(M);
  }
  function R(w) {
    const T = this.stack.pop();
    T.value += this.sliceSerialize(w), T.position.end = Fe(w.end);
  }
  function X(w) {
    const T = this.stack[this.stack.length - 1];
    if (o("atHardBreak")) {
      const M = T.children[T.children.length - 1];
      M.position.end = Fe(w.end), l("atHardBreak");
      return;
    }
    !o("setextHeadingSlurpLineEnding") && n.canContainEols.includes(T.type) && (P.call(this, w), R.call(this, w));
  }
  function te() {
    l("atHardBreak", !0);
  }
  function he() {
    const w = this.resume(), T = this.stack[this.stack.length - 1];
    T.value = w;
  }
  function oe() {
    const w = this.resume(), T = this.stack[this.stack.length - 1];
    T.value = w;
  }
  function ue() {
    const w = this.resume(), T = this.stack[this.stack.length - 1];
    T.value = w;
  }
  function re() {
    const w = this.stack[this.stack.length - 1];
    if (o("inReference")) {
      const T = o("referenceType") || "shortcut";
      w.type += "Reference", w.referenceType = T, delete w.url, delete w.title;
    } else
      delete w.identifier, delete w.label;
    l("referenceType");
  }
  function ae() {
    const w = this.stack[this.stack.length - 1];
    if (o("inReference")) {
      const T = o("referenceType") || "shortcut";
      w.type += "Reference", w.referenceType = T, delete w.url, delete w.title;
    } else
      delete w.identifier, delete w.label;
    l("referenceType");
  }
  function E(w) {
    const T = this.sliceSerialize(w), M = this.stack[this.stack.length - 2];
    M.label = to(T), M.identifier = Me(T).toLowerCase();
  }
  function s() {
    const w = this.stack[this.stack.length - 1], T = this.resume(), M = this.stack[this.stack.length - 1];
    if (l("inReference", !0), M.type === "link") {
      const J = w.children;
      M.children = J;
    } else
      M.alt = T;
  }
  function p() {
    const w = this.resume(), T = this.stack[this.stack.length - 1];
    T.url = w;
  }
  function B() {
    const w = this.resume(), T = this.stack[this.stack.length - 1];
    T.title = w;
  }
  function D() {
    l("inReference");
  }
  function q() {
    l("referenceType", "collapsed");
  }
  function O(w) {
    const T = this.resume(), M = this.stack[this.stack.length - 1];
    M.label = T, M.identifier = Me(
      this.sliceSerialize(w)
    ).toLowerCase(), l("referenceType", "full");
  }
  function L(w) {
    l("characterReferenceType", w.type);
  }
  function $(w) {
    const T = this.sliceSerialize(w), M = o("characterReferenceType");
    let J;
    M ? (J = sr(
      T,
      M === "characterReferenceMarkerNumeric" ? 10 : 16
    ), l("characterReferenceType")) : J = zn(T);
    const ee = this.stack.pop();
    ee.value += J, ee.position.end = Fe(w.end);
  }
  function W(w) {
    R.call(this, w);
    const T = this.stack[this.stack.length - 1];
    T.url = this.sliceSerialize(w);
  }
  function H(w) {
    R.call(this, w);
    const T = this.stack[this.stack.length - 1];
    T.url = "mailto:" + this.sliceSerialize(w);
  }
  function ce() {
    return {
      type: "blockquote",
      children: []
    };
  }
  function A() {
    return {
      type: "code",
      lang: null,
      meta: null,
      value: ""
    };
  }
  function de() {
    return {
      type: "inlineCode",
      value: ""
    };
  }
  function I() {
    return {
      type: "definition",
      identifier: "",
      label: null,
      title: null,
      url: ""
    };
  }
  function Oe() {
    return {
      type: "emphasis",
      children: []
    };
  }
  function _e() {
    return {
      type: "heading",
      depth: void 0,
      children: []
    };
  }
  function $e() {
    return {
      type: "break"
    };
  }
  function Hn() {
    return {
      type: "html",
      value: ""
    };
  }
  function _r() {
    return {
      type: "image",
      title: null,
      url: "",
      alt: null
    };
  }
  function Vn() {
    return {
      type: "link",
      title: null,
      url: "",
      children: []
    };
  }
  function Yn(w) {
    return {
      type: "list",
      ordered: w.type === "listOrdered",
      start: null,
      // @ts-expect-error Patched.
      spread: w._spread,
      children: []
    };
  }
  function Lr(w) {
    return {
      type: "listItem",
      // @ts-expect-error Patched.
      spread: w._spread,
      checked: null,
      children: []
    };
  }
  function Dr() {
    return {
      type: "paragraph",
      children: []
    };
  }
  function zr() {
    return {
      type: "strong",
      children: []
    };
  }
  function Mr() {
    return {
      type: "text",
      value: ""
    };
  }
  function Br() {
    return {
      type: "thematicBreak"
    };
  }
}
function Fe(e) {
  return {
    line: e.line,
    column: e.column,
    offset: e.offset
  };
}
function fr(e, n) {
  let t = -1;
  for (; ++t < n.length; ) {
    const r = n[t];
    Array.isArray(r) ? fr(e, r) : oo(e, r);
  }
}
function oo(e, n) {
  let t;
  for (t in n)
    if (cr.call(n, t)) {
      if (t === "canContainEols") {
        const r = n[t];
        r && e[t].push(...r);
      } else if (t === "transforms") {
        const r = n[t];
        r && e[t].push(...r);
      } else if (t === "enter" || t === "exit") {
        const r = n[t];
        r && Object.assign(e[t], r);
      }
    }
}
function wt(e, n) {
  throw e ? new Error(
    "Cannot close `" + e.type + "` (" + en({
      start: e.start,
      end: e.end
    }) + "): a different token (`" + n.type + "`, " + en({
      start: n.start,
      end: n.end
    }) + ") is open"
  ) : new Error(
    "Cannot close document, a token (`" + n.type + "`, " + en({
      start: n.start,
      end: n.end
    }) + ") is still open"
  );
}
function uo(e) {
  Object.assign(this, { Parser: (t) => {
    const r = (
      /** @type {Options} */
      this.data("settings")
    );
    return io(
      t,
      Object.assign({}, r, e, {
        // Note: these options are not in the readme.
        // The goal is for them to be set by plugins on `data` instead of being
        // passed by users.
        extensions: this.data("micromarkExtensions") || [],
        mdastExtensions: this.data("fromMarkdownExtensions") || []
      })
    );
  } });
}
function ao(e, n) {
  const t = {
    type: "element",
    tagName: "blockquote",
    properties: {},
    children: e.wrap(e.all(n), !0)
  };
  return e.patch(n, t), e.applyData(n, t);
}
function so(e, n) {
  const t = { type: "element", tagName: "br", properties: {}, children: [] };
  return e.patch(n, t), [e.applyData(n, t), { type: "text", value: `
` }];
}
function co(e, n) {
  const t = n.value ? n.value + `
` : "", r = n.lang ? n.lang.match(/^[^ \t]+(?=[ \t]|$)/) : null, i = {};
  r && (i.className = ["language-" + r]);
  let l = {
    type: "element",
    tagName: "code",
    properties: i,
    children: [{ type: "text", value: t }]
  };
  return n.meta && (l.data = { meta: n.meta }), e.patch(n, l), l = e.applyData(n, l), l = { type: "element", tagName: "pre", properties: {}, children: [l] }, e.patch(n, l), l;
}
function fo(e, n) {
  const t = {
    type: "element",
    tagName: "del",
    properties: {},
    children: e.all(n)
  };
  return e.patch(n, t), e.applyData(n, t);
}
function po(e, n) {
  const t = {
    type: "element",
    tagName: "em",
    properties: {},
    children: e.all(n)
  };
  return e.patch(n, t), e.applyData(n, t);
}
function Ne(e) {
  const n = [];
  let t = -1, r = 0, i = 0;
  for (; ++t < e.length; ) {
    const l = e.charCodeAt(t);
    let o = "";
    if (l === 37 && xe(e.charCodeAt(t + 1)) && xe(e.charCodeAt(t + 2)))
      i = 2;
    else if (l < 128)
      /[!#$&-;=?-Z_a-z~]/.test(String.fromCharCode(l)) || (o = String.fromCharCode(l));
    else if (l > 55295 && l < 57344) {
      const u = e.charCodeAt(t + 1);
      l < 56320 && u > 56319 && u < 57344 ? (o = String.fromCharCode(l, u), i = 1) : o = "�";
    } else
      o = String.fromCharCode(l);
    o && (n.push(e.slice(r, t), encodeURIComponent(o)), r = t + i + 1, o = ""), i && (t += i, i = 0);
  }
  return n.join("") + e.slice(r);
}
function pr(e, n) {
  const t = String(n.identifier).toUpperCase(), r = Ne(t.toLowerCase()), i = e.footnoteOrder.indexOf(t);
  let l;
  i === -1 ? (e.footnoteOrder.push(t), e.footnoteCounts[t] = 1, l = e.footnoteOrder.length) : (e.footnoteCounts[t]++, l = i + 1);
  const o = e.footnoteCounts[t], u = {
    type: "element",
    tagName: "a",
    properties: {
      href: "#" + e.clobberPrefix + "fn-" + r,
      id: e.clobberPrefix + "fnref-" + r + (o > 1 ? "-" + o : ""),
      dataFootnoteRef: !0,
      ariaDescribedBy: ["footnote-label"]
    },
    children: [{ type: "text", value: String(l) }]
  };
  e.patch(n, u);
  const a = {
    type: "element",
    tagName: "sup",
    properties: {},
    children: [u]
  };
  return e.patch(n, a), e.applyData(n, a);
}
function ho(e, n) {
  const t = e.footnoteById;
  let r = 1;
  for (; r in t; )
    r++;
  const i = String(r);
  return t[i] = {
    type: "footnoteDefinition",
    identifier: i,
    children: [{ type: "paragraph", children: n.children }],
    position: n.position
  }, pr(e, {
    type: "footnoteReference",
    identifier: i,
    position: n.position
  });
}
function mo(e, n) {
  const t = {
    type: "element",
    tagName: "h" + n.depth,
    properties: {},
    children: e.all(n)
  };
  return e.patch(n, t), e.applyData(n, t);
}
function go(e, n) {
  if (e.dangerous) {
    const t = { type: "raw", value: n.value };
    return e.patch(n, t), e.applyData(n, t);
  }
  return null;
}
function hr(e, n) {
  const t = n.referenceType;
  let r = "]";
  if (t === "collapsed" ? r += "[]" : t === "full" && (r += "[" + (n.label || n.identifier) + "]"), n.type === "imageReference")
    return { type: "text", value: "![" + n.alt + r };
  const i = e.all(n), l = i[0];
  l && l.type === "text" ? l.value = "[" + l.value : i.unshift({ type: "text", value: "[" });
  const o = i[i.length - 1];
  return o && o.type === "text" ? o.value += r : i.push({ type: "text", value: r }), i;
}
function yo(e, n) {
  const t = e.definition(n.identifier);
  if (!t)
    return hr(e, n);
  const r = { src: Ne(t.url || ""), alt: n.alt };
  t.title !== null && t.title !== void 0 && (r.title = t.title);
  const i = { type: "element", tagName: "img", properties: r, children: [] };
  return e.patch(n, i), e.applyData(n, i);
}
function xo(e, n) {
  const t = { src: Ne(n.url) };
  n.alt !== null && n.alt !== void 0 && (t.alt = n.alt), n.title !== null && n.title !== void 0 && (t.title = n.title);
  const r = { type: "element", tagName: "img", properties: t, children: [] };
  return e.patch(n, r), e.applyData(n, r);
}
function bo(e, n) {
  const t = { type: "text", value: n.value.replace(/\r?\n|\r/g, " ") };
  e.patch(n, t);
  const r = {
    type: "element",
    tagName: "code",
    properties: {},
    children: [t]
  };
  return e.patch(n, r), e.applyData(n, r);
}
function ko(e, n) {
  const t = e.definition(n.identifier);
  if (!t)
    return hr(e, n);
  const r = { href: Ne(t.url || "") };
  t.title !== null && t.title !== void 0 && (r.title = t.title);
  const i = {
    type: "element",
    tagName: "a",
    properties: r,
    children: e.all(n)
  };
  return e.patch(n, i), e.applyData(n, i);
}
function wo(e, n) {
  const t = { href: Ne(n.url) };
  n.title !== null && n.title !== void 0 && (t.title = n.title);
  const r = {
    type: "element",
    tagName: "a",
    properties: t,
    children: e.all(n)
  };
  return e.patch(n, r), e.applyData(n, r);
}
function So(e, n, t) {
  const r = e.all(n), i = t ? Eo(t) : dr(n), l = {}, o = [];
  if (typeof n.checked == "boolean") {
    const c = r[0];
    let h;
    c && c.type === "element" && c.tagName === "p" ? h = c : (h = { type: "element", tagName: "p", properties: {}, children: [] }, r.unshift(h)), h.children.length > 0 && h.children.unshift({ type: "text", value: " " }), h.children.unshift({
      type: "element",
      tagName: "input",
      properties: { type: "checkbox", checked: n.checked, disabled: !0 },
      children: []
    }), l.className = ["task-list-item"];
  }
  let u = -1;
  for (; ++u < r.length; ) {
    const c = r[u];
    (i || u !== 0 || c.type !== "element" || c.tagName !== "p") && o.push({ type: "text", value: `
` }), c.type === "element" && c.tagName === "p" && !i ? o.push(...c.children) : o.push(c);
  }
  const a = r[r.length - 1];
  a && (i || a.type !== "element" || a.tagName !== "p") && o.push({ type: "text", value: `
` });
  const f = { type: "element", tagName: "li", properties: l, children: o };
  return e.patch(n, f), e.applyData(n, f);
}
function Eo(e) {
  let n = !1;
  if (e.type === "list") {
    n = e.spread || !1;
    const t = e.children;
    let r = -1;
    for (; !n && ++r < t.length; )
      n = dr(t[r]);
  }
  return n;
}
function dr(e) {
  const n = e.spread;
  return n ?? e.children.length > 1;
}
function Co(e, n) {
  const t = {}, r = e.all(n);
  let i = -1;
  for (typeof n.start == "number" && n.start !== 1 && (t.start = n.start); ++i < r.length; ) {
    const o = r[i];
    if (o.type === "element" && o.tagName === "li" && o.properties && Array.isArray(o.properties.className) && o.properties.className.includes("task-list-item")) {
      t.className = ["contains-task-list"];
      break;
    }
  }
  const l = {
    type: "element",
    tagName: n.ordered ? "ol" : "ul",
    properties: t,
    children: e.wrap(r, !0)
  };
  return e.patch(n, l), e.applyData(n, l);
}
function vo(e, n) {
  const t = {
    type: "element",
    tagName: "p",
    properties: {},
    children: e.all(n)
  };
  return e.patch(n, t), e.applyData(n, t);
}
function To(e, n) {
  const t = { type: "root", children: e.wrap(e.all(n)) };
  return e.patch(n, t), e.applyData(n, t);
}
function Po(e, n) {
  const t = {
    type: "element",
    tagName: "strong",
    properties: {},
    children: e.all(n)
  };
  return e.patch(n, t), e.applyData(n, t);
}
const Bn = mr("start"), Nn = mr("end");
function Ao(e) {
  return { start: Bn(e), end: Nn(e) };
}
function mr(e) {
  return n;
  function n(t) {
    const r = t && t.position && t.position[e] || {};
    return {
      // @ts-expect-error: in practice, null is allowed.
      line: r.line || null,
      // @ts-expect-error: in practice, null is allowed.
      column: r.column || null,
      // @ts-expect-error: in practice, null is allowed.
      offset: r.offset > -1 ? r.offset : null
    };
  }
}
function Oo(e, n) {
  const t = e.all(n), r = t.shift(), i = [];
  if (r) {
    const o = {
      type: "element",
      tagName: "thead",
      properties: {},
      children: e.wrap([r], !0)
    };
    e.patch(n.children[0], o), i.push(o);
  }
  if (t.length > 0) {
    const o = {
      type: "element",
      tagName: "tbody",
      properties: {},
      children: e.wrap(t, !0)
    }, u = Bn(n.children[1]), a = Nn(n.children[n.children.length - 1]);
    u.line && a.line && (o.position = { start: u, end: a }), i.push(o);
  }
  const l = {
    type: "element",
    tagName: "table",
    properties: {},
    children: e.wrap(i, !0)
  };
  return e.patch(n, l), e.applyData(n, l);
}
function Io(e, n, t) {
  const r = t ? t.children : void 0, l = (r ? r.indexOf(n) : 1) === 0 ? "th" : "td", o = t && t.type === "table" ? t.align : void 0, u = o ? o.length : n.children.length;
  let a = -1;
  const f = [];
  for (; ++a < u; ) {
    const h = n.children[a], g = {}, m = o ? o[a] : void 0;
    m && (g.align = m);
    let d = { type: "element", tagName: l, properties: g, children: [] };
    h && (d.children = e.all(h), e.patch(h, d), d = e.applyData(n, d)), f.push(d);
  }
  const c = {
    type: "element",
    tagName: "tr",
    properties: {},
    children: e.wrap(f, !0)
  };
  return e.patch(n, c), e.applyData(n, c);
}
function Fo(e, n) {
  const t = {
    type: "element",
    tagName: "td",
    // Assume body cell.
    properties: {},
    children: e.all(n)
  };
  return e.patch(n, t), e.applyData(n, t);
}
const St = 9, Et = 32;
function Ro(e) {
  const n = String(e), t = /\r?\n|\r/g;
  let r = t.exec(n), i = 0;
  const l = [];
  for (; r; )
    l.push(
      Ct(n.slice(i, r.index), i > 0, !0),
      r[0]
    ), i = r.index + r[0].length, r = t.exec(n);
  return l.push(Ct(n.slice(i), i > 0, !1)), l.join("");
}
function Ct(e, n, t) {
  let r = 0, i = e.length;
  if (n) {
    let l = e.codePointAt(r);
    for (; l === St || l === Et; )
      r++, l = e.codePointAt(r);
  }
  if (t) {
    let l = e.codePointAt(i - 1);
    for (; l === St || l === Et; )
      i--, l = e.codePointAt(i - 1);
  }
  return i > r ? e.slice(r, i) : "";
}
function _o(e, n) {
  const t = { type: "text", value: Ro(String(n.value)) };
  return e.patch(n, t), e.applyData(n, t);
}
function Lo(e, n) {
  const t = {
    type: "element",
    tagName: "hr",
    properties: {},
    children: []
  };
  return e.patch(n, t), e.applyData(n, t);
}
const Do = {
  blockquote: ao,
  break: so,
  code: co,
  delete: fo,
  emphasis: po,
  footnoteReference: pr,
  footnote: ho,
  heading: mo,
  html: go,
  imageReference: yo,
  image: xo,
  inlineCode: bo,
  linkReference: ko,
  link: wo,
  listItem: So,
  list: Co,
  paragraph: vo,
  root: To,
  strong: Po,
  table: Oo,
  tableCell: Fo,
  tableRow: Io,
  text: _o,
  thematicBreak: Lo,
  toml: Qe,
  yaml: Qe,
  definition: Qe,
  footnoteDefinition: Qe
};
function Qe() {
  return null;
}
const gr = (
  /**
   * @type {(
   *   (<Kind extends Node>(test: PredicateTest<Kind>) => AssertPredicate<Kind>) &
   *   ((test?: Test) => AssertAnything)
   * )}
   */
  /**
   * @param {Test} [test]
   * @returns {AssertAnything}
   */
  function(e) {
    if (e == null)
      return No;
    if (typeof e == "string")
      return Bo(e);
    if (typeof e == "object")
      return Array.isArray(e) ? zo(e) : Mo(e);
    if (typeof e == "function")
      return rn(e);
    throw new Error("Expected function, string, or object as test");
  }
);
function zo(e) {
  const n = [];
  let t = -1;
  for (; ++t < e.length; )
    n[t] = gr(e[t]);
  return rn(r);
  function r(...i) {
    let l = -1;
    for (; ++l < n.length; )
      if (n[l].call(this, ...i))
        return !0;
    return !1;
  }
}
function Mo(e) {
  return rn(n);
  function n(t) {
    let r;
    for (r in e)
      if (t[r] !== e[r])
        return !1;
    return !0;
  }
}
function Bo(e) {
  return rn(n);
  function n(t) {
    return t && t.type === e;
  }
}
function rn(e) {
  return n;
  function n(t, ...r) {
    return !!(t && typeof t == "object" && "type" in t && e.call(this, t, ...r));
  }
}
function No() {
  return !0;
}
const jo = !0, vt = !1, $o = "skip", Uo = (
  /**
   * @type {(
   *   (<Tree extends Node, Check extends Test>(tree: Tree, test: Check, visitor: BuildVisitor<Tree, Check>, reverse?: boolean | null | undefined) => void) &
   *   (<Tree extends Node>(tree: Tree, visitor: BuildVisitor<Tree>, reverse?: boolean | null | undefined) => void)
   * )}
   */
  /**
   * @param {Node} tree
   * @param {Test} test
   * @param {Visitor<Node>} visitor
   * @param {boolean | null | undefined} [reverse]
   * @returns {void}
   */
  function(e, n, t, r) {
    typeof n == "function" && typeof t != "function" && (r = t, t = n, n = null);
    const i = gr(n), l = r ? -1 : 1;
    o(e, void 0, [])();
    function o(u, a, f) {
      const c = u && typeof u == "object" ? u : {};
      if (typeof c.type == "string") {
        const g = (
          // `hast`
          typeof c.tagName == "string" ? c.tagName : (
            // `xast`
            typeof c.name == "string" ? c.name : void 0
          )
        );
        Object.defineProperty(h, "name", {
          value: "node (" + (u.type + (g ? "<" + g + ">" : "")) + ")"
        });
      }
      return h;
      function h() {
        let g = [], m, d, y;
        if ((!n || i(u, a, f[f.length - 1] || null)) && (g = qo(t(u, f)), g[0] === vt))
          return g;
        if (u.children && g[0] !== $o)
          for (d = (r ? u.children.length : -1) + l, y = f.concat(u); d > -1 && d < u.children.length; ) {
            if (m = o(u.children[d], d, y)(), m[0] === vt)
              return m;
            d = typeof m[1] == "number" ? m[1] : d + l;
          }
        return g;
      }
    }
  }
);
function qo(e) {
  return Array.isArray(e) ? e : typeof e == "number" ? [jo, e] : [e];
}
const yr = (
  /**
   * @type {(
   *   (<Tree extends Node, Check extends Test>(tree: Tree, test: Check, visitor: BuildVisitor<Tree, Check>, reverse?: boolean | null | undefined) => void) &
   *   (<Tree extends Node>(tree: Tree, visitor: BuildVisitor<Tree>, reverse?: boolean | null | undefined) => void)
   * )}
   */
  /**
   * @param {Node} tree
   * @param {Test} test
   * @param {Visitor} visitor
   * @param {boolean | null | undefined} [reverse]
   * @returns {void}
   */
  function(e, n, t, r) {
    typeof n == "function" && typeof t != "function" && (r = t, t = n, n = null), Uo(e, n, i, r);
    function i(l, o) {
      const u = o[o.length - 1];
      return t(
        l,
        u ? u.children.indexOf(l) : null,
        u
      );
    }
  }
);
function Ho(e) {
  return !e || !e.position || !e.position.start || !e.position.start.line || !e.position.start.column || !e.position.end || !e.position.end.line || !e.position.end.column;
}
const Tt = {}.hasOwnProperty;
function Vo(e) {
  const n = /* @__PURE__ */ Object.create(null);
  if (!e || !e.type)
    throw new Error("mdast-util-definitions expected node");
  return yr(e, "definition", (r) => {
    const i = Pt(r.identifier);
    i && !Tt.call(n, i) && (n[i] = r);
  }), t;
  function t(r) {
    const i = Pt(r);
    return i && Tt.call(n, i) ? n[i] : null;
  }
}
function Pt(e) {
  return String(e || "").toUpperCase();
}
const nn = {}.hasOwnProperty;
function Yo(e, n) {
  const t = n || {}, r = t.allowDangerousHtml || !1, i = {};
  return o.dangerous = r, o.clobberPrefix = t.clobberPrefix === void 0 || t.clobberPrefix === null ? "user-content-" : t.clobberPrefix, o.footnoteLabel = t.footnoteLabel || "Footnotes", o.footnoteLabelTagName = t.footnoteLabelTagName || "h2", o.footnoteLabelProperties = t.footnoteLabelProperties || {
    className: ["sr-only"]
  }, o.footnoteBackLabel = t.footnoteBackLabel || "Back to content", o.unknownHandler = t.unknownHandler, o.passThrough = t.passThrough, o.handlers = { ...Do, ...t.handlers }, o.definition = Vo(e), o.footnoteById = i, o.footnoteOrder = [], o.footnoteCounts = {}, o.patch = Wo, o.applyData = Xo, o.one = u, o.all = a, o.wrap = Ko, o.augment = l, yr(e, "footnoteDefinition", (f) => {
    const c = String(f.identifier).toUpperCase();
    nn.call(i, c) || (i[c] = f);
  }), o;
  function l(f, c) {
    if (f && "data" in f && f.data) {
      const h = f.data;
      h.hName && (c.type !== "element" && (c = {
        type: "element",
        tagName: "",
        properties: {},
        children: []
      }), c.tagName = h.hName), c.type === "element" && h.hProperties && (c.properties = { ...c.properties, ...h.hProperties }), "children" in c && c.children && h.hChildren && (c.children = h.hChildren);
    }
    if (f) {
      const h = "type" in f ? f : { position: f };
      Ho(h) || (c.position = { start: Bn(h), end: Nn(h) });
    }
    return c;
  }
  function o(f, c, h, g) {
    return Array.isArray(h) && (g = h, h = {}), l(f, {
      type: "element",
      tagName: c,
      properties: h || {},
      children: g || []
    });
  }
  function u(f, c) {
    return xr(o, f, c);
  }
  function a(f) {
    return jn(o, f);
  }
}
function Wo(e, n) {
  e.position && (n.position = Ao(e));
}
function Xo(e, n) {
  let t = n;
  if (e && e.data) {
    const r = e.data.hName, i = e.data.hChildren, l = e.data.hProperties;
    typeof r == "string" && (t.type === "element" ? t.tagName = r : t = {
      type: "element",
      tagName: r,
      properties: {},
      children: []
    }), t.type === "element" && l && (t.properties = { ...t.properties, ...l }), "children" in t && t.children && i !== null && i !== void 0 && (t.children = i);
  }
  return t;
}
function xr(e, n, t) {
  const r = n && n.type;
  if (!r)
    throw new Error("Expected node, got `" + n + "`");
  return nn.call(e.handlers, r) ? e.handlers[r](e, n, t) : e.passThrough && e.passThrough.includes(r) ? "children" in n ? { ...n, children: jn(e, n) } : n : e.unknownHandler ? e.unknownHandler(e, n, t) : Qo(e, n);
}
function jn(e, n) {
  const t = [];
  if ("children" in n) {
    const r = n.children;
    let i = -1;
    for (; ++i < r.length; ) {
      const l = xr(e, r[i], n);
      if (l) {
        if (i && r[i - 1].type === "break" && (!Array.isArray(l) && l.type === "text" && (l.value = l.value.replace(/^\s+/, "")), !Array.isArray(l) && l.type === "element")) {
          const o = l.children[0];
          o && o.type === "text" && (o.value = o.value.replace(/^\s+/, ""));
        }
        Array.isArray(l) ? t.push(...l) : t.push(l);
      }
    }
  }
  return t;
}
function Qo(e, n) {
  const t = n.data || {}, r = "value" in n && !(nn.call(t, "hProperties") || nn.call(t, "hChildren")) ? { type: "text", value: n.value } : {
    type: "element",
    tagName: "div",
    properties: {},
    children: jn(e, n)
  };
  return e.patch(n, r), e.applyData(n, r);
}
function Ko(e, n) {
  const t = [];
  let r = -1;
  for (n && t.push({ type: "text", value: `
` }); ++r < e.length; )
    r && t.push({ type: "text", value: `
` }), t.push(e[r]);
  return n && e.length > 0 && t.push({ type: "text", value: `
` }), t;
}
function Go(e) {
  const n = [];
  let t = -1;
  for (; ++t < e.footnoteOrder.length; ) {
    const r = e.footnoteById[e.footnoteOrder[t]];
    if (!r)
      continue;
    const i = e.all(r), l = String(r.identifier).toUpperCase(), o = Ne(l.toLowerCase());
    let u = 0;
    const a = [];
    for (; ++u <= e.footnoteCounts[l]; ) {
      const h = {
        type: "element",
        tagName: "a",
        properties: {
          href: "#" + e.clobberPrefix + "fnref-" + o + (u > 1 ? "-" + u : ""),
          dataFootnoteBackref: !0,
          className: ["data-footnote-backref"],
          ariaLabel: e.footnoteBackLabel
        },
        children: [{ type: "text", value: "↩" }]
      };
      u > 1 && h.children.push({
        type: "element",
        tagName: "sup",
        children: [{ type: "text", value: String(u) }]
      }), a.length > 0 && a.push({ type: "text", value: " " }), a.push(h);
    }
    const f = i[i.length - 1];
    if (f && f.type === "element" && f.tagName === "p") {
      const h = f.children[f.children.length - 1];
      h && h.type === "text" ? h.value += " " : f.children.push({ type: "text", value: " " }), f.children.push(...a);
    } else
      i.push(...a);
    const c = {
      type: "element",
      tagName: "li",
      properties: { id: e.clobberPrefix + "fn-" + o },
      children: e.wrap(i, !0)
    };
    e.patch(r, c), n.push(c);
  }
  if (n.length !== 0)
    return {
      type: "element",
      tagName: "section",
      properties: { dataFootnotes: !0, className: ["footnotes"] },
      children: [
        {
          type: "element",
          tagName: e.footnoteLabelTagName,
          properties: {
            // To do: use structured clone.
            ...JSON.parse(JSON.stringify(e.footnoteLabelProperties)),
            id: "footnote-label"
          },
          children: [{ type: "text", value: e.footnoteLabel }]
        },
        { type: "text", value: `
` },
        {
          type: "element",
          tagName: "ol",
          properties: {},
          children: e.wrap(n, !0)
        },
        { type: "text", value: `
` }
      ]
    };
}
function br(e, n) {
  const t = Yo(e, n), r = t.one(e, null), i = Go(t);
  return i && r.children.push({ type: "text", value: `
` }, i), Array.isArray(r) ? { type: "root", children: r } : r;
}
const Zo = (
  /** @type {(import('unified').Plugin<[Processor, Options?]|[null|undefined, Options?]|[Options]|[], MdastRoot>)} */
  function(e, n) {
    return e && "run" in e ? eu(e, n) : nu(e || n);
  }
), Jo = Zo;
function eu(e, n) {
  return (t, r, i) => {
    e.run(br(t, n), r, (l) => {
      i(l);
    });
  };
}
function nu(e) {
  return (n) => br(n, e);
}
var In = { exports: {} }, Ke = { exports: {} }, Q = {};
/** @license React v16.13.1
 * react-is.production.min.js
 *
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
var At;
function tu() {
  if (At)
    return Q;
  At = 1;
  var e = typeof Symbol == "function" && Symbol.for, n = e ? Symbol.for("react.element") : 60103, t = e ? Symbol.for("react.portal") : 60106, r = e ? Symbol.for("react.fragment") : 60107, i = e ? Symbol.for("react.strict_mode") : 60108, l = e ? Symbol.for("react.profiler") : 60114, o = e ? Symbol.for("react.provider") : 60109, u = e ? Symbol.for("react.context") : 60110, a = e ? Symbol.for("react.async_mode") : 60111, f = e ? Symbol.for("react.concurrent_mode") : 60111, c = e ? Symbol.for("react.forward_ref") : 60112, h = e ? Symbol.for("react.suspense") : 60113, g = e ? Symbol.for("react.suspense_list") : 60120, m = e ? Symbol.for("react.memo") : 60115, d = e ? Symbol.for("react.lazy") : 60116, y = e ? Symbol.for("react.block") : 60121, x = e ? Symbol.for("react.fundamental") : 60117, b = e ? Symbol.for("react.responder") : 60118, z = e ? Symbol.for("react.scope") : 60119;
  function v(S) {
    if (typeof S == "object" && S !== null) {
      var k = S.$$typeof;
      switch (k) {
        case n:
          switch (S = S.type, S) {
            case a:
            case f:
            case r:
            case l:
            case i:
            case h:
              return S;
            default:
              switch (S = S && S.$$typeof, S) {
                case u:
                case c:
                case d:
                case m:
                case o:
                  return S;
                default:
                  return k;
              }
          }
        case t:
          return k;
      }
    }
  }
  function F(S) {
    return v(S) === f;
  }
  return Q.AsyncMode = a, Q.ConcurrentMode = f, Q.ContextConsumer = u, Q.ContextProvider = o, Q.Element = n, Q.ForwardRef = c, Q.Fragment = r, Q.Lazy = d, Q.Memo = m, Q.Portal = t, Q.Profiler = l, Q.StrictMode = i, Q.Suspense = h, Q.isAsyncMode = function(S) {
    return F(S) || v(S) === a;
  }, Q.isConcurrentMode = F, Q.isContextConsumer = function(S) {
    return v(S) === u;
  }, Q.isContextProvider = function(S) {
    return v(S) === o;
  }, Q.isElement = function(S) {
    return typeof S == "object" && S !== null && S.$$typeof === n;
  }, Q.isForwardRef = function(S) {
    return v(S) === c;
  }, Q.isFragment = function(S) {
    return v(S) === r;
  }, Q.isLazy = function(S) {
    return v(S) === d;
  }, Q.isMemo = function(S) {
    return v(S) === m;
  }, Q.isPortal = function(S) {
    return v(S) === t;
  }, Q.isProfiler = function(S) {
    return v(S) === l;
  }, Q.isStrictMode = function(S) {
    return v(S) === i;
  }, Q.isSuspense = function(S) {
    return v(S) === h;
  }, Q.isValidElementType = function(S) {
    return typeof S == "string" || typeof S == "function" || S === r || S === f || S === l || S === i || S === h || S === g || typeof S == "object" && S !== null && (S.$$typeof === d || S.$$typeof === m || S.$$typeof === o || S.$$typeof === u || S.$$typeof === c || S.$$typeof === x || S.$$typeof === b || S.$$typeof === z || S.$$typeof === y);
  }, Q.typeOf = v, Q;
}
var K = {}, Ot;
function ru() {
  return Ot || (Ot = 1, _index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.p.env.NODE_ENV !== "production" && function() {
    var e = typeof Symbol == "function" && Symbol.for, n = e ? Symbol.for("react.element") : 60103, t = e ? Symbol.for("react.portal") : 60106, r = e ? Symbol.for("react.fragment") : 60107, i = e ? Symbol.for("react.strict_mode") : 60108, l = e ? Symbol.for("react.profiler") : 60114, o = e ? Symbol.for("react.provider") : 60109, u = e ? Symbol.for("react.context") : 60110, a = e ? Symbol.for("react.async_mode") : 60111, f = e ? Symbol.for("react.concurrent_mode") : 60111, c = e ? Symbol.for("react.forward_ref") : 60112, h = e ? Symbol.for("react.suspense") : 60113, g = e ? Symbol.for("react.suspense_list") : 60120, m = e ? Symbol.for("react.memo") : 60115, d = e ? Symbol.for("react.lazy") : 60116, y = e ? Symbol.for("react.block") : 60121, x = e ? Symbol.for("react.fundamental") : 60117, b = e ? Symbol.for("react.responder") : 60118, z = e ? Symbol.for("react.scope") : 60119;
    function v(A) {
      return typeof A == "string" || typeof A == "function" || // Note: its typeof might be other than 'symbol' or 'number' if it's a polyfill.
      A === r || A === f || A === l || A === i || A === h || A === g || typeof A == "object" && A !== null && (A.$$typeof === d || A.$$typeof === m || A.$$typeof === o || A.$$typeof === u || A.$$typeof === c || A.$$typeof === x || A.$$typeof === b || A.$$typeof === z || A.$$typeof === y);
    }
    function F(A) {
      if (typeof A == "object" && A !== null) {
        var de = A.$$typeof;
        switch (de) {
          case n:
            var I = A.type;
            switch (I) {
              case a:
              case f:
              case r:
              case l:
              case i:
              case h:
                return I;
              default:
                var Oe = I && I.$$typeof;
                switch (Oe) {
                  case u:
                  case c:
                  case d:
                  case m:
                  case o:
                    return Oe;
                  default:
                    return de;
                }
            }
          case t:
            return de;
        }
      }
    }
    var S = a, k = f, _ = u, U = o, Y = n, le = c, P = r, R = d, X = m, te = t, he = l, oe = i, ue = h, re = !1;
    function ae(A) {
      return re || (re = !0, console.warn("The ReactIs.isAsyncMode() alias has been deprecated, and will be removed in React 17+. Update your code to use ReactIs.isConcurrentMode() instead. It has the exact same API.")), E(A) || F(A) === a;
    }
    function E(A) {
      return F(A) === f;
    }
    function s(A) {
      return F(A) === u;
    }
    function p(A) {
      return F(A) === o;
    }
    function B(A) {
      return typeof A == "object" && A !== null && A.$$typeof === n;
    }
    function D(A) {
      return F(A) === c;
    }
    function q(A) {
      return F(A) === r;
    }
    function O(A) {
      return F(A) === d;
    }
    function L(A) {
      return F(A) === m;
    }
    function $(A) {
      return F(A) === t;
    }
    function W(A) {
      return F(A) === l;
    }
    function H(A) {
      return F(A) === i;
    }
    function ce(A) {
      return F(A) === h;
    }
    K.AsyncMode = S, K.ConcurrentMode = k, K.ContextConsumer = _, K.ContextProvider = U, K.Element = Y, K.ForwardRef = le, K.Fragment = P, K.Lazy = R, K.Memo = X, K.Portal = te, K.Profiler = he, K.StrictMode = oe, K.Suspense = ue, K.isAsyncMode = ae, K.isConcurrentMode = E, K.isContextConsumer = s, K.isContextProvider = p, K.isElement = B, K.isForwardRef = D, K.isFragment = q, K.isLazy = O, K.isMemo = L, K.isPortal = $, K.isProfiler = W, K.isStrictMode = H, K.isSuspense = ce, K.isValidElementType = v, K.typeOf = F;
  }()), K;
}
var It;
function kr() {
  return It || (It = 1, _index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.p.env.NODE_ENV === "production" ? Ke.exports = tu() : Ke.exports = ru()), Ke.exports;
}
/*
object-assign
(c) Sindre Sorhus
@license MIT
*/
var mn, Ft;
function iu() {
  if (Ft)
    return mn;
  Ft = 1;
  var e = Object.getOwnPropertySymbols, n = Object.prototype.hasOwnProperty, t = Object.prototype.propertyIsEnumerable;
  function r(l) {
    if (l == null)
      throw new TypeError("Object.assign cannot be called with null or undefined");
    return Object(l);
  }
  function i() {
    try {
      if (!Object.assign)
        return !1;
      var l = new String("abc");
      if (l[5] = "de", Object.getOwnPropertyNames(l)[0] === "5")
        return !1;
      for (var o = {}, u = 0; u < 10; u++)
        o["_" + String.fromCharCode(u)] = u;
      var a = Object.getOwnPropertyNames(o).map(function(c) {
        return o[c];
      });
      if (a.join("") !== "0123456789")
        return !1;
      var f = {};
      return "abcdefghijklmnopqrst".split("").forEach(function(c) {
        f[c] = c;
      }), Object.keys(Object.assign({}, f)).join("") === "abcdefghijklmnopqrst";
    } catch {
      return !1;
    }
  }
  return mn = i() ? Object.assign : function(l, o) {
    for (var u, a = r(l), f, c = 1; c < arguments.length; c++) {
      u = Object(arguments[c]);
      for (var h in u)
        n.call(u, h) && (a[h] = u[h]);
      if (e) {
        f = e(u);
        for (var g = 0; g < f.length; g++)
          t.call(u, f[g]) && (a[f[g]] = u[f[g]]);
      }
    }
    return a;
  }, mn;
}
var gn, Rt;
function $n() {
  if (Rt)
    return gn;
  Rt = 1;
  var e = "SECRET_DO_NOT_PASS_THIS_OR_YOU_WILL_BE_FIRED";
  return gn = e, gn;
}
var yn, _t;
function wr() {
  return _t || (_t = 1, yn = Function.call.bind(Object.prototype.hasOwnProperty)), yn;
}
var xn, Lt;
function lu() {
  if (Lt)
    return xn;
  Lt = 1;
  var e = function() {
  };
  if (_index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.p.env.NODE_ENV !== "production") {
    var n = $n(), t = {}, r = wr();
    e = function(l) {
      var o = "Warning: " + l;
      typeof console < "u" && console.error(o);
      try {
        throw new Error(o);
      } catch {
      }
    };
  }
  function i(l, o, u, a, f) {
    if (_index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.p.env.NODE_ENV !== "production") {
      for (var c in l)
        if (r(l, c)) {
          var h;
          try {
            if (typeof l[c] != "function") {
              var g = Error(
                (a || "React class") + ": " + u + " type `" + c + "` is invalid; it must be a function, usually from the `prop-types` package, but received `" + typeof l[c] + "`.This often happens because of typos such as `PropTypes.function` instead of `PropTypes.func`."
              );
              throw g.name = "Invariant Violation", g;
            }
            h = l[c](o, c, a, u, null, n);
          } catch (d) {
            h = d;
          }
          if (h && !(h instanceof Error) && e(
            (a || "React class") + ": type specification of " + u + " `" + c + "` is invalid; the type checker function must return `null` or an `Error` but returned a " + typeof h + ". You may have forgotten to pass an argument to the type checker creator (arrayOf, instanceOf, objectOf, oneOf, oneOfType, and shape all require an argument)."
          ), h instanceof Error && !(h.message in t)) {
            t[h.message] = !0;
            var m = f ? f() : "";
            e(
              "Failed " + u + " type: " + h.message + (m ?? "")
            );
          }
        }
    }
  }
  return i.resetWarningCache = function() {
    _index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.p.env.NODE_ENV !== "production" && (t = {});
  }, xn = i, xn;
}
var bn, Dt;
function ou() {
  if (Dt)
    return bn;
  Dt = 1;
  var e = kr(), n = iu(), t = $n(), r = wr(), i = lu(), l = function() {
  };
  _index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.p.env.NODE_ENV !== "production" && (l = function(u) {
    var a = "Warning: " + u;
    typeof console < "u" && console.error(a);
    try {
      throw new Error(a);
    } catch {
    }
  });
  function o() {
    return null;
  }
  return bn = function(u, a) {
    var f = typeof Symbol == "function" && Symbol.iterator, c = "@@iterator";
    function h(E) {
      var s = E && (f && E[f] || E[c]);
      if (typeof s == "function")
        return s;
    }
    var g = "<<anonymous>>", m = {
      array: b("array"),
      bigint: b("bigint"),
      bool: b("boolean"),
      func: b("function"),
      number: b("number"),
      object: b("object"),
      string: b("string"),
      symbol: b("symbol"),
      any: z(),
      arrayOf: v,
      element: F(),
      elementType: S(),
      instanceOf: k,
      node: le(),
      objectOf: U,
      oneOf: _,
      oneOfType: Y,
      shape: R,
      exact: X
    };
    function d(E, s) {
      return E === s ? E !== 0 || 1 / E === 1 / s : E !== E && s !== s;
    }
    function y(E, s) {
      this.message = E, this.data = s && typeof s == "object" ? s : {}, this.stack = "";
    }
    y.prototype = Error.prototype;
    function x(E) {
      if (_index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.p.env.NODE_ENV !== "production")
        var s = {}, p = 0;
      function B(q, O, L, $, W, H, ce) {
        if ($ = $ || g, H = H || L, ce !== t) {
          if (a) {
            var A = new Error(
              "Calling PropTypes validators directly is not supported by the `prop-types` package. Use `PropTypes.checkPropTypes()` to call them. Read more at http://fb.me/use-check-prop-types"
            );
            throw A.name = "Invariant Violation", A;
          } else if (_index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.p.env.NODE_ENV !== "production" && typeof console < "u") {
            var de = $ + ":" + L;
            !s[de] && // Avoid spamming the console because they are often not actionable except for lib authors
            p < 3 && (l(
              "You are manually calling a React.PropTypes validation function for the `" + H + "` prop on `" + $ + "`. This is deprecated and will throw in the standalone `prop-types` package. You may be seeing this warning due to a third-party PropTypes library. See https://fb.me/react-warning-dont-call-proptypes for details."
            ), s[de] = !0, p++);
          }
        }
        return O[L] == null ? q ? O[L] === null ? new y("The " + W + " `" + H + "` is marked as required " + ("in `" + $ + "`, but its value is `null`.")) : new y("The " + W + " `" + H + "` is marked as required in " + ("`" + $ + "`, but its value is `undefined`.")) : null : E(O, L, $, W, H);
      }
      var D = B.bind(null, !1);
      return D.isRequired = B.bind(null, !0), D;
    }
    function b(E) {
      function s(p, B, D, q, O, L) {
        var $ = p[B], W = oe($);
        if (W !== E) {
          var H = ue($);
          return new y(
            "Invalid " + q + " `" + O + "` of type " + ("`" + H + "` supplied to `" + D + "`, expected ") + ("`" + E + "`."),
            { expectedType: E }
          );
        }
        return null;
      }
      return x(s);
    }
    function z() {
      return x(o);
    }
    function v(E) {
      function s(p, B, D, q, O) {
        if (typeof E != "function")
          return new y("Property `" + O + "` of component `" + D + "` has invalid PropType notation inside arrayOf.");
        var L = p[B];
        if (!Array.isArray(L)) {
          var $ = oe(L);
          return new y("Invalid " + q + " `" + O + "` of type " + ("`" + $ + "` supplied to `" + D + "`, expected an array."));
        }
        for (var W = 0; W < L.length; W++) {
          var H = E(L, W, D, q, O + "[" + W + "]", t);
          if (H instanceof Error)
            return H;
        }
        return null;
      }
      return x(s);
    }
    function F() {
      function E(s, p, B, D, q) {
        var O = s[p];
        if (!u(O)) {
          var L = oe(O);
          return new y("Invalid " + D + " `" + q + "` of type " + ("`" + L + "` supplied to `" + B + "`, expected a single ReactElement."));
        }
        return null;
      }
      return x(E);
    }
    function S() {
      function E(s, p, B, D, q) {
        var O = s[p];
        if (!e.isValidElementType(O)) {
          var L = oe(O);
          return new y("Invalid " + D + " `" + q + "` of type " + ("`" + L + "` supplied to `" + B + "`, expected a single ReactElement type."));
        }
        return null;
      }
      return x(E);
    }
    function k(E) {
      function s(p, B, D, q, O) {
        if (!(p[B] instanceof E)) {
          var L = E.name || g, $ = ae(p[B]);
          return new y("Invalid " + q + " `" + O + "` of type " + ("`" + $ + "` supplied to `" + D + "`, expected ") + ("instance of `" + L + "`."));
        }
        return null;
      }
      return x(s);
    }
    function _(E) {
      if (!Array.isArray(E))
        return _index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.p.env.NODE_ENV !== "production" && (arguments.length > 1 ? l(
          "Invalid arguments supplied to oneOf, expected an array, got " + arguments.length + " arguments. A common mistake is to write oneOf(x, y, z) instead of oneOf([x, y, z])."
        ) : l("Invalid argument supplied to oneOf, expected an array.")), o;
      function s(p, B, D, q, O) {
        for (var L = p[B], $ = 0; $ < E.length; $++)
          if (d(L, E[$]))
            return null;
        var W = JSON.stringify(E, function(ce, A) {
          var de = ue(A);
          return de === "symbol" ? String(A) : A;
        });
        return new y("Invalid " + q + " `" + O + "` of value `" + String(L) + "` " + ("supplied to `" + D + "`, expected one of " + W + "."));
      }
      return x(s);
    }
    function U(E) {
      function s(p, B, D, q, O) {
        if (typeof E != "function")
          return new y("Property `" + O + "` of component `" + D + "` has invalid PropType notation inside objectOf.");
        var L = p[B], $ = oe(L);
        if ($ !== "object")
          return new y("Invalid " + q + " `" + O + "` of type " + ("`" + $ + "` supplied to `" + D + "`, expected an object."));
        for (var W in L)
          if (r(L, W)) {
            var H = E(L, W, D, q, O + "." + W, t);
            if (H instanceof Error)
              return H;
          }
        return null;
      }
      return x(s);
    }
    function Y(E) {
      if (!Array.isArray(E))
        return _index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.p.env.NODE_ENV !== "production" && l("Invalid argument supplied to oneOfType, expected an instance of array."), o;
      for (var s = 0; s < E.length; s++) {
        var p = E[s];
        if (typeof p != "function")
          return l(
            "Invalid argument supplied to oneOfType. Expected an array of check functions, but received " + re(p) + " at index " + s + "."
          ), o;
      }
      function B(D, q, O, L, $) {
        for (var W = [], H = 0; H < E.length; H++) {
          var ce = E[H], A = ce(D, q, O, L, $, t);
          if (A == null)
            return null;
          A.data && r(A.data, "expectedType") && W.push(A.data.expectedType);
        }
        var de = W.length > 0 ? ", expected one of type [" + W.join(", ") + "]" : "";
        return new y("Invalid " + L + " `" + $ + "` supplied to " + ("`" + O + "`" + de + "."));
      }
      return x(B);
    }
    function le() {
      function E(s, p, B, D, q) {
        return te(s[p]) ? null : new y("Invalid " + D + " `" + q + "` supplied to " + ("`" + B + "`, expected a ReactNode."));
      }
      return x(E);
    }
    function P(E, s, p, B, D) {
      return new y(
        (E || "React class") + ": " + s + " type `" + p + "." + B + "` is invalid; it must be a function, usually from the `prop-types` package, but received `" + D + "`."
      );
    }
    function R(E) {
      function s(p, B, D, q, O) {
        var L = p[B], $ = oe(L);
        if ($ !== "object")
          return new y("Invalid " + q + " `" + O + "` of type `" + $ + "` " + ("supplied to `" + D + "`, expected `object`."));
        for (var W in E) {
          var H = E[W];
          if (typeof H != "function")
            return P(D, q, O, W, ue(H));
          var ce = H(L, W, D, q, O + "." + W, t);
          if (ce)
            return ce;
        }
        return null;
      }
      return x(s);
    }
    function X(E) {
      function s(p, B, D, q, O) {
        var L = p[B], $ = oe(L);
        if ($ !== "object")
          return new y("Invalid " + q + " `" + O + "` of type `" + $ + "` " + ("supplied to `" + D + "`, expected `object`."));
        var W = n({}, p[B], E);
        for (var H in W) {
          var ce = E[H];
          if (r(E, H) && typeof ce != "function")
            return P(D, q, O, H, ue(ce));
          if (!ce)
            return new y(
              "Invalid " + q + " `" + O + "` key `" + H + "` supplied to `" + D + "`.\nBad object: " + JSON.stringify(p[B], null, "  ") + `
Valid keys: ` + JSON.stringify(Object.keys(E), null, "  ")
            );
          var A = ce(L, H, D, q, O + "." + H, t);
          if (A)
            return A;
        }
        return null;
      }
      return x(s);
    }
    function te(E) {
      switch (typeof E) {
        case "number":
        case "string":
        case "undefined":
          return !0;
        case "boolean":
          return !E;
        case "object":
          if (Array.isArray(E))
            return E.every(te);
          if (E === null || u(E))
            return !0;
          var s = h(E);
          if (s) {
            var p = s.call(E), B;
            if (s !== E.entries) {
              for (; !(B = p.next()).done; )
                if (!te(B.value))
                  return !1;
            } else
              for (; !(B = p.next()).done; ) {
                var D = B.value;
                if (D && !te(D[1]))
                  return !1;
              }
          } else
            return !1;
          return !0;
        default:
          return !1;
      }
    }
    function he(E, s) {
      return E === "symbol" ? !0 : s ? s["@@toStringTag"] === "Symbol" || typeof Symbol == "function" && s instanceof Symbol : !1;
    }
    function oe(E) {
      var s = typeof E;
      return Array.isArray(E) ? "array" : E instanceof RegExp ? "object" : he(s, E) ? "symbol" : s;
    }
    function ue(E) {
      if (typeof E > "u" || E === null)
        return "" + E;
      var s = oe(E);
      if (s === "object") {
        if (E instanceof Date)
          return "date";
        if (E instanceof RegExp)
          return "regexp";
      }
      return s;
    }
    function re(E) {
      var s = ue(E);
      switch (s) {
        case "array":
        case "object":
          return "an " + s;
        case "boolean":
        case "date":
        case "regexp":
          return "a " + s;
        default:
          return s;
      }
    }
    function ae(E) {
      return !E.constructor || !E.constructor.name ? g : E.constructor.name;
    }
    return m.checkPropTypes = i, m.resetWarningCache = i.resetWarningCache, m.PropTypes = m, m;
  }, bn;
}
var kn, zt;
function uu() {
  if (zt)
    return kn;
  zt = 1;
  var e = $n();
  function n() {
  }
  function t() {
  }
  return t.resetWarningCache = n, kn = function() {
    function r(o, u, a, f, c, h) {
      if (h !== e) {
        var g = new Error(
          "Calling PropTypes validators directly is not supported by the `prop-types` package. Use PropTypes.checkPropTypes() to call them. Read more at http://fb.me/use-check-prop-types"
        );
        throw g.name = "Invariant Violation", g;
      }
    }
    r.isRequired = r;
    function i() {
      return r;
    }
    var l = {
      array: r,
      bigint: r,
      bool: r,
      func: r,
      number: r,
      object: r,
      string: r,
      symbol: r,
      any: r,
      arrayOf: i,
      element: r,
      elementType: r,
      instanceOf: i,
      node: r,
      objectOf: i,
      oneOf: i,
      oneOfType: i,
      shape: i,
      exact: i,
      checkPropTypes: t,
      resetWarningCache: n
    };
    return l.PropTypes = l, l;
  }, kn;
}
if (_index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.p.env.NODE_ENV !== "production") {
  var au = kr(), su = !0;
  In.exports = ou()(au.isElement, su);
} else
  In.exports = uu()();
var cu = In.exports;
const j = /* @__PURE__ */ (0,_index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.g)(cu);
class Xe {
  /**
   * @constructor
   * @param {Properties} property
   * @param {Normal} normal
   * @param {string} [space]
   */
  constructor(n, t, r) {
    this.property = n, this.normal = t, r && (this.space = r);
  }
}
Xe.prototype.property = {};
Xe.prototype.normal = {};
Xe.prototype.space = null;
function Sr(e, n) {
  const t = {}, r = {};
  let i = -1;
  for (; ++i < e.length; )
    Object.assign(t, e[i].property), Object.assign(r, e[i].normal);
  return new Xe(t, r, n);
}
function Fn(e) {
  return e.toLowerCase();
}
class Se {
  /**
   * @constructor
   * @param {string} property
   * @param {string} attribute
   */
  constructor(n, t) {
    this.property = n, this.attribute = t;
  }
}
Se.prototype.space = null;
Se.prototype.boolean = !1;
Se.prototype.booleanish = !1;
Se.prototype.overloadedBoolean = !1;
Se.prototype.number = !1;
Se.prototype.commaSeparated = !1;
Se.prototype.spaceSeparated = !1;
Se.prototype.commaOrSpaceSeparated = !1;
Se.prototype.mustUseProperty = !1;
Se.prototype.defined = !1;
let fu = 0;
const V = De(), fe = De(), Er = De(), C = De(), ie = De(), Be = De(), ge = De();
function De() {
  return 2 ** ++fu;
}
const Rn = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  boolean: V,
  booleanish: fe,
  commaOrSpaceSeparated: ge,
  commaSeparated: Be,
  number: C,
  overloadedBoolean: Er,
  spaceSeparated: ie
}, Symbol.toStringTag, { value: "Module" })), wn = Object.keys(Rn);
class Un extends Se {
  /**
   * @constructor
   * @param {string} property
   * @param {string} attribute
   * @param {number|null} [mask]
   * @param {string} [space]
   */
  constructor(n, t, r, i) {
    let l = -1;
    if (super(n, t), Mt(this, "space", i), typeof r == "number")
      for (; ++l < wn.length; ) {
        const o = wn[l];
        Mt(this, wn[l], (r & Rn[o]) === Rn[o]);
      }
  }
}
Un.prototype.defined = !0;
function Mt(e, n, t) {
  t && (e[n] = t);
}
const pu = {}.hasOwnProperty;
function je(e) {
  const n = {}, t = {};
  let r;
  for (r in e.properties)
    if (pu.call(e.properties, r)) {
      const i = e.properties[r], l = new Un(
        r,
        e.transform(e.attributes || {}, r),
        i,
        e.space
      );
      e.mustUseProperty && e.mustUseProperty.includes(r) && (l.mustUseProperty = !0), n[r] = l, t[Fn(r)] = r, t[Fn(l.attribute)] = r;
    }
  return new Xe(n, t, e.space);
}
const Cr = je({
  space: "xlink",
  transform(e, n) {
    return "xlink:" + n.slice(5).toLowerCase();
  },
  properties: {
    xLinkActuate: null,
    xLinkArcRole: null,
    xLinkHref: null,
    xLinkRole: null,
    xLinkShow: null,
    xLinkTitle: null,
    xLinkType: null
  }
}), vr = je({
  space: "xml",
  transform(e, n) {
    return "xml:" + n.slice(3).toLowerCase();
  },
  properties: { xmlLang: null, xmlBase: null, xmlSpace: null }
});
function Tr(e, n) {
  return n in e ? e[n] : n;
}
function Pr(e, n) {
  return Tr(e, n.toLowerCase());
}
const Ar = je({
  space: "xmlns",
  attributes: { xmlnsxlink: "xmlns:xlink" },
  transform: Pr,
  properties: { xmlns: null, xmlnsXLink: null }
}), Or = je({
  transform(e, n) {
    return n === "role" ? n : "aria-" + n.slice(4).toLowerCase();
  },
  properties: {
    ariaActiveDescendant: null,
    ariaAtomic: fe,
    ariaAutoComplete: null,
    ariaBusy: fe,
    ariaChecked: fe,
    ariaColCount: C,
    ariaColIndex: C,
    ariaColSpan: C,
    ariaControls: ie,
    ariaCurrent: null,
    ariaDescribedBy: ie,
    ariaDetails: null,
    ariaDisabled: fe,
    ariaDropEffect: ie,
    ariaErrorMessage: null,
    ariaExpanded: fe,
    ariaFlowTo: ie,
    ariaGrabbed: fe,
    ariaHasPopup: null,
    ariaHidden: fe,
    ariaInvalid: null,
    ariaKeyShortcuts: null,
    ariaLabel: null,
    ariaLabelledBy: ie,
    ariaLevel: C,
    ariaLive: null,
    ariaModal: fe,
    ariaMultiLine: fe,
    ariaMultiSelectable: fe,
    ariaOrientation: null,
    ariaOwns: ie,
    ariaPlaceholder: null,
    ariaPosInSet: C,
    ariaPressed: fe,
    ariaReadOnly: fe,
    ariaRelevant: null,
    ariaRequired: fe,
    ariaRoleDescription: ie,
    ariaRowCount: C,
    ariaRowIndex: C,
    ariaRowSpan: C,
    ariaSelected: fe,
    ariaSetSize: C,
    ariaSort: null,
    ariaValueMax: C,
    ariaValueMin: C,
    ariaValueNow: C,
    ariaValueText: null,
    role: null
  }
}), hu = je({
  space: "html",
  attributes: {
    acceptcharset: "accept-charset",
    classname: "class",
    htmlfor: "for",
    httpequiv: "http-equiv"
  },
  transform: Pr,
  mustUseProperty: ["checked", "multiple", "muted", "selected"],
  properties: {
    // Standard Properties.
    abbr: null,
    accept: Be,
    acceptCharset: ie,
    accessKey: ie,
    action: null,
    allow: null,
    allowFullScreen: V,
    allowPaymentRequest: V,
    allowUserMedia: V,
    alt: null,
    as: null,
    async: V,
    autoCapitalize: null,
    autoComplete: ie,
    autoFocus: V,
    autoPlay: V,
    capture: V,
    charSet: null,
    checked: V,
    cite: null,
    className: ie,
    cols: C,
    colSpan: null,
    content: null,
    contentEditable: fe,
    controls: V,
    controlsList: ie,
    coords: C | Be,
    crossOrigin: null,
    data: null,
    dateTime: null,
    decoding: null,
    default: V,
    defer: V,
    dir: null,
    dirName: null,
    disabled: V,
    download: Er,
    draggable: fe,
    encType: null,
    enterKeyHint: null,
    form: null,
    formAction: null,
    formEncType: null,
    formMethod: null,
    formNoValidate: V,
    formTarget: null,
    headers: ie,
    height: C,
    hidden: V,
    high: C,
    href: null,
    hrefLang: null,
    htmlFor: ie,
    httpEquiv: ie,
    id: null,
    imageSizes: null,
    imageSrcSet: null,
    inputMode: null,
    integrity: null,
    is: null,
    isMap: V,
    itemId: null,
    itemProp: ie,
    itemRef: ie,
    itemScope: V,
    itemType: ie,
    kind: null,
    label: null,
    lang: null,
    language: null,
    list: null,
    loading: null,
    loop: V,
    low: C,
    manifest: null,
    max: null,
    maxLength: C,
    media: null,
    method: null,
    min: null,
    minLength: C,
    multiple: V,
    muted: V,
    name: null,
    nonce: null,
    noModule: V,
    noValidate: V,
    onAbort: null,
    onAfterPrint: null,
    onAuxClick: null,
    onBeforeMatch: null,
    onBeforePrint: null,
    onBeforeUnload: null,
    onBlur: null,
    onCancel: null,
    onCanPlay: null,
    onCanPlayThrough: null,
    onChange: null,
    onClick: null,
    onClose: null,
    onContextLost: null,
    onContextMenu: null,
    onContextRestored: null,
    onCopy: null,
    onCueChange: null,
    onCut: null,
    onDblClick: null,
    onDrag: null,
    onDragEnd: null,
    onDragEnter: null,
    onDragExit: null,
    onDragLeave: null,
    onDragOver: null,
    onDragStart: null,
    onDrop: null,
    onDurationChange: null,
    onEmptied: null,
    onEnded: null,
    onError: null,
    onFocus: null,
    onFormData: null,
    onHashChange: null,
    onInput: null,
    onInvalid: null,
    onKeyDown: null,
    onKeyPress: null,
    onKeyUp: null,
    onLanguageChange: null,
    onLoad: null,
    onLoadedData: null,
    onLoadedMetadata: null,
    onLoadEnd: null,
    onLoadStart: null,
    onMessage: null,
    onMessageError: null,
    onMouseDown: null,
    onMouseEnter: null,
    onMouseLeave: null,
    onMouseMove: null,
    onMouseOut: null,
    onMouseOver: null,
    onMouseUp: null,
    onOffline: null,
    onOnline: null,
    onPageHide: null,
    onPageShow: null,
    onPaste: null,
    onPause: null,
    onPlay: null,
    onPlaying: null,
    onPopState: null,
    onProgress: null,
    onRateChange: null,
    onRejectionHandled: null,
    onReset: null,
    onResize: null,
    onScroll: null,
    onScrollEnd: null,
    onSecurityPolicyViolation: null,
    onSeeked: null,
    onSeeking: null,
    onSelect: null,
    onSlotChange: null,
    onStalled: null,
    onStorage: null,
    onSubmit: null,
    onSuspend: null,
    onTimeUpdate: null,
    onToggle: null,
    onUnhandledRejection: null,
    onUnload: null,
    onVolumeChange: null,
    onWaiting: null,
    onWheel: null,
    open: V,
    optimum: C,
    pattern: null,
    ping: ie,
    placeholder: null,
    playsInline: V,
    poster: null,
    preload: null,
    readOnly: V,
    referrerPolicy: null,
    rel: ie,
    required: V,
    reversed: V,
    rows: C,
    rowSpan: C,
    sandbox: ie,
    scope: null,
    scoped: V,
    seamless: V,
    selected: V,
    shape: null,
    size: C,
    sizes: null,
    slot: null,
    span: C,
    spellCheck: fe,
    src: null,
    srcDoc: null,
    srcLang: null,
    srcSet: null,
    start: C,
    step: null,
    style: null,
    tabIndex: C,
    target: null,
    title: null,
    translate: null,
    type: null,
    typeMustMatch: V,
    useMap: null,
    value: fe,
    width: C,
    wrap: null,
    // Legacy.
    // See: https://html.spec.whatwg.org/#other-elements,-attributes-and-apis
    align: null,
    // Several. Use CSS `text-align` instead,
    aLink: null,
    // `<body>`. Use CSS `a:active {color}` instead
    archive: ie,
    // `<object>`. List of URIs to archives
    axis: null,
    // `<td>` and `<th>`. Use `scope` on `<th>`
    background: null,
    // `<body>`. Use CSS `background-image` instead
    bgColor: null,
    // `<body>` and table elements. Use CSS `background-color` instead
    border: C,
    // `<table>`. Use CSS `border-width` instead,
    borderColor: null,
    // `<table>`. Use CSS `border-color` instead,
    bottomMargin: C,
    // `<body>`
    cellPadding: null,
    // `<table>`
    cellSpacing: null,
    // `<table>`
    char: null,
    // Several table elements. When `align=char`, sets the character to align on
    charOff: null,
    // Several table elements. When `char`, offsets the alignment
    classId: null,
    // `<object>`
    clear: null,
    // `<br>`. Use CSS `clear` instead
    code: null,
    // `<object>`
    codeBase: null,
    // `<object>`
    codeType: null,
    // `<object>`
    color: null,
    // `<font>` and `<hr>`. Use CSS instead
    compact: V,
    // Lists. Use CSS to reduce space between items instead
    declare: V,
    // `<object>`
    event: null,
    // `<script>`
    face: null,
    // `<font>`. Use CSS instead
    frame: null,
    // `<table>`
    frameBorder: null,
    // `<iframe>`. Use CSS `border` instead
    hSpace: C,
    // `<img>` and `<object>`
    leftMargin: C,
    // `<body>`
    link: null,
    // `<body>`. Use CSS `a:link {color: *}` instead
    longDesc: null,
    // `<frame>`, `<iframe>`, and `<img>`. Use an `<a>`
    lowSrc: null,
    // `<img>`. Use a `<picture>`
    marginHeight: C,
    // `<body>`
    marginWidth: C,
    // `<body>`
    noResize: V,
    // `<frame>`
    noHref: V,
    // `<area>`. Use no href instead of an explicit `nohref`
    noShade: V,
    // `<hr>`. Use background-color and height instead of borders
    noWrap: V,
    // `<td>` and `<th>`
    object: null,
    // `<applet>`
    profile: null,
    // `<head>`
    prompt: null,
    // `<isindex>`
    rev: null,
    // `<link>`
    rightMargin: C,
    // `<body>`
    rules: null,
    // `<table>`
    scheme: null,
    // `<meta>`
    scrolling: fe,
    // `<frame>`. Use overflow in the child context
    standby: null,
    // `<object>`
    summary: null,
    // `<table>`
    text: null,
    // `<body>`. Use CSS `color` instead
    topMargin: C,
    // `<body>`
    valueType: null,
    // `<param>`
    version: null,
    // `<html>`. Use a doctype.
    vAlign: null,
    // Several. Use CSS `vertical-align` instead
    vLink: null,
    // `<body>`. Use CSS `a:visited {color}` instead
    vSpace: C,
    // `<img>` and `<object>`
    // Non-standard Properties.
    allowTransparency: null,
    autoCorrect: null,
    autoSave: null,
    disablePictureInPicture: V,
    disableRemotePlayback: V,
    prefix: null,
    property: null,
    results: C,
    security: null,
    unselectable: null
  }
}), du = je({
  space: "svg",
  attributes: {
    accentHeight: "accent-height",
    alignmentBaseline: "alignment-baseline",
    arabicForm: "arabic-form",
    baselineShift: "baseline-shift",
    capHeight: "cap-height",
    className: "class",
    clipPath: "clip-path",
    clipRule: "clip-rule",
    colorInterpolation: "color-interpolation",
    colorInterpolationFilters: "color-interpolation-filters",
    colorProfile: "color-profile",
    colorRendering: "color-rendering",
    crossOrigin: "crossorigin",
    dataType: "datatype",
    dominantBaseline: "dominant-baseline",
    enableBackground: "enable-background",
    fillOpacity: "fill-opacity",
    fillRule: "fill-rule",
    floodColor: "flood-color",
    floodOpacity: "flood-opacity",
    fontFamily: "font-family",
    fontSize: "font-size",
    fontSizeAdjust: "font-size-adjust",
    fontStretch: "font-stretch",
    fontStyle: "font-style",
    fontVariant: "font-variant",
    fontWeight: "font-weight",
    glyphName: "glyph-name",
    glyphOrientationHorizontal: "glyph-orientation-horizontal",
    glyphOrientationVertical: "glyph-orientation-vertical",
    hrefLang: "hreflang",
    horizAdvX: "horiz-adv-x",
    horizOriginX: "horiz-origin-x",
    horizOriginY: "horiz-origin-y",
    imageRendering: "image-rendering",
    letterSpacing: "letter-spacing",
    lightingColor: "lighting-color",
    markerEnd: "marker-end",
    markerMid: "marker-mid",
    markerStart: "marker-start",
    navDown: "nav-down",
    navDownLeft: "nav-down-left",
    navDownRight: "nav-down-right",
    navLeft: "nav-left",
    navNext: "nav-next",
    navPrev: "nav-prev",
    navRight: "nav-right",
    navUp: "nav-up",
    navUpLeft: "nav-up-left",
    navUpRight: "nav-up-right",
    onAbort: "onabort",
    onActivate: "onactivate",
    onAfterPrint: "onafterprint",
    onBeforePrint: "onbeforeprint",
    onBegin: "onbegin",
    onCancel: "oncancel",
    onCanPlay: "oncanplay",
    onCanPlayThrough: "oncanplaythrough",
    onChange: "onchange",
    onClick: "onclick",
    onClose: "onclose",
    onCopy: "oncopy",
    onCueChange: "oncuechange",
    onCut: "oncut",
    onDblClick: "ondblclick",
    onDrag: "ondrag",
    onDragEnd: "ondragend",
    onDragEnter: "ondragenter",
    onDragExit: "ondragexit",
    onDragLeave: "ondragleave",
    onDragOver: "ondragover",
    onDragStart: "ondragstart",
    onDrop: "ondrop",
    onDurationChange: "ondurationchange",
    onEmptied: "onemptied",
    onEnd: "onend",
    onEnded: "onended",
    onError: "onerror",
    onFocus: "onfocus",
    onFocusIn: "onfocusin",
    onFocusOut: "onfocusout",
    onHashChange: "onhashchange",
    onInput: "oninput",
    onInvalid: "oninvalid",
    onKeyDown: "onkeydown",
    onKeyPress: "onkeypress",
    onKeyUp: "onkeyup",
    onLoad: "onload",
    onLoadedData: "onloadeddata",
    onLoadedMetadata: "onloadedmetadata",
    onLoadStart: "onloadstart",
    onMessage: "onmessage",
    onMouseDown: "onmousedown",
    onMouseEnter: "onmouseenter",
    onMouseLeave: "onmouseleave",
    onMouseMove: "onmousemove",
    onMouseOut: "onmouseout",
    onMouseOver: "onmouseover",
    onMouseUp: "onmouseup",
    onMouseWheel: "onmousewheel",
    onOffline: "onoffline",
    onOnline: "ononline",
    onPageHide: "onpagehide",
    onPageShow: "onpageshow",
    onPaste: "onpaste",
    onPause: "onpause",
    onPlay: "onplay",
    onPlaying: "onplaying",
    onPopState: "onpopstate",
    onProgress: "onprogress",
    onRateChange: "onratechange",
    onRepeat: "onrepeat",
    onReset: "onreset",
    onResize: "onresize",
    onScroll: "onscroll",
    onSeeked: "onseeked",
    onSeeking: "onseeking",
    onSelect: "onselect",
    onShow: "onshow",
    onStalled: "onstalled",
    onStorage: "onstorage",
    onSubmit: "onsubmit",
    onSuspend: "onsuspend",
    onTimeUpdate: "ontimeupdate",
    onToggle: "ontoggle",
    onUnload: "onunload",
    onVolumeChange: "onvolumechange",
    onWaiting: "onwaiting",
    onZoom: "onzoom",
    overlinePosition: "overline-position",
    overlineThickness: "overline-thickness",
    paintOrder: "paint-order",
    panose1: "panose-1",
    pointerEvents: "pointer-events",
    referrerPolicy: "referrerpolicy",
    renderingIntent: "rendering-intent",
    shapeRendering: "shape-rendering",
    stopColor: "stop-color",
    stopOpacity: "stop-opacity",
    strikethroughPosition: "strikethrough-position",
    strikethroughThickness: "strikethrough-thickness",
    strokeDashArray: "stroke-dasharray",
    strokeDashOffset: "stroke-dashoffset",
    strokeLineCap: "stroke-linecap",
    strokeLineJoin: "stroke-linejoin",
    strokeMiterLimit: "stroke-miterlimit",
    strokeOpacity: "stroke-opacity",
    strokeWidth: "stroke-width",
    tabIndex: "tabindex",
    textAnchor: "text-anchor",
    textDecoration: "text-decoration",
    textRendering: "text-rendering",
    typeOf: "typeof",
    underlinePosition: "underline-position",
    underlineThickness: "underline-thickness",
    unicodeBidi: "unicode-bidi",
    unicodeRange: "unicode-range",
    unitsPerEm: "units-per-em",
    vAlphabetic: "v-alphabetic",
    vHanging: "v-hanging",
    vIdeographic: "v-ideographic",
    vMathematical: "v-mathematical",
    vectorEffect: "vector-effect",
    vertAdvY: "vert-adv-y",
    vertOriginX: "vert-origin-x",
    vertOriginY: "vert-origin-y",
    wordSpacing: "word-spacing",
    writingMode: "writing-mode",
    xHeight: "x-height",
    // These were camelcased in Tiny. Now lowercased in SVG 2
    playbackOrder: "playbackorder",
    timelineBegin: "timelinebegin"
  },
  transform: Tr,
  properties: {
    about: ge,
    accentHeight: C,
    accumulate: null,
    additive: null,
    alignmentBaseline: null,
    alphabetic: C,
    amplitude: C,
    arabicForm: null,
    ascent: C,
    attributeName: null,
    attributeType: null,
    azimuth: C,
    bandwidth: null,
    baselineShift: null,
    baseFrequency: null,
    baseProfile: null,
    bbox: null,
    begin: null,
    bias: C,
    by: null,
    calcMode: null,
    capHeight: C,
    className: ie,
    clip: null,
    clipPath: null,
    clipPathUnits: null,
    clipRule: null,
    color: null,
    colorInterpolation: null,
    colorInterpolationFilters: null,
    colorProfile: null,
    colorRendering: null,
    content: null,
    contentScriptType: null,
    contentStyleType: null,
    crossOrigin: null,
    cursor: null,
    cx: null,
    cy: null,
    d: null,
    dataType: null,
    defaultAction: null,
    descent: C,
    diffuseConstant: C,
    direction: null,
    display: null,
    dur: null,
    divisor: C,
    dominantBaseline: null,
    download: V,
    dx: null,
    dy: null,
    edgeMode: null,
    editable: null,
    elevation: C,
    enableBackground: null,
    end: null,
    event: null,
    exponent: C,
    externalResourcesRequired: null,
    fill: null,
    fillOpacity: C,
    fillRule: null,
    filter: null,
    filterRes: null,
    filterUnits: null,
    floodColor: null,
    floodOpacity: null,
    focusable: null,
    focusHighlight: null,
    fontFamily: null,
    fontSize: null,
    fontSizeAdjust: null,
    fontStretch: null,
    fontStyle: null,
    fontVariant: null,
    fontWeight: null,
    format: null,
    fr: null,
    from: null,
    fx: null,
    fy: null,
    g1: Be,
    g2: Be,
    glyphName: Be,
    glyphOrientationHorizontal: null,
    glyphOrientationVertical: null,
    glyphRef: null,
    gradientTransform: null,
    gradientUnits: null,
    handler: null,
    hanging: C,
    hatchContentUnits: null,
    hatchUnits: null,
    height: null,
    href: null,
    hrefLang: null,
    horizAdvX: C,
    horizOriginX: C,
    horizOriginY: C,
    id: null,
    ideographic: C,
    imageRendering: null,
    initialVisibility: null,
    in: null,
    in2: null,
    intercept: C,
    k: C,
    k1: C,
    k2: C,
    k3: C,
    k4: C,
    kernelMatrix: ge,
    kernelUnitLength: null,
    keyPoints: null,
    // SEMI_COLON_SEPARATED
    keySplines: null,
    // SEMI_COLON_SEPARATED
    keyTimes: null,
    // SEMI_COLON_SEPARATED
    kerning: null,
    lang: null,
    lengthAdjust: null,
    letterSpacing: null,
    lightingColor: null,
    limitingConeAngle: C,
    local: null,
    markerEnd: null,
    markerMid: null,
    markerStart: null,
    markerHeight: null,
    markerUnits: null,
    markerWidth: null,
    mask: null,
    maskContentUnits: null,
    maskUnits: null,
    mathematical: null,
    max: null,
    media: null,
    mediaCharacterEncoding: null,
    mediaContentEncodings: null,
    mediaSize: C,
    mediaTime: null,
    method: null,
    min: null,
    mode: null,
    name: null,
    navDown: null,
    navDownLeft: null,
    navDownRight: null,
    navLeft: null,
    navNext: null,
    navPrev: null,
    navRight: null,
    navUp: null,
    navUpLeft: null,
    navUpRight: null,
    numOctaves: null,
    observer: null,
    offset: null,
    onAbort: null,
    onActivate: null,
    onAfterPrint: null,
    onBeforePrint: null,
    onBegin: null,
    onCancel: null,
    onCanPlay: null,
    onCanPlayThrough: null,
    onChange: null,
    onClick: null,
    onClose: null,
    onCopy: null,
    onCueChange: null,
    onCut: null,
    onDblClick: null,
    onDrag: null,
    onDragEnd: null,
    onDragEnter: null,
    onDragExit: null,
    onDragLeave: null,
    onDragOver: null,
    onDragStart: null,
    onDrop: null,
    onDurationChange: null,
    onEmptied: null,
    onEnd: null,
    onEnded: null,
    onError: null,
    onFocus: null,
    onFocusIn: null,
    onFocusOut: null,
    onHashChange: null,
    onInput: null,
    onInvalid: null,
    onKeyDown: null,
    onKeyPress: null,
    onKeyUp: null,
    onLoad: null,
    onLoadedData: null,
    onLoadedMetadata: null,
    onLoadStart: null,
    onMessage: null,
    onMouseDown: null,
    onMouseEnter: null,
    onMouseLeave: null,
    onMouseMove: null,
    onMouseOut: null,
    onMouseOver: null,
    onMouseUp: null,
    onMouseWheel: null,
    onOffline: null,
    onOnline: null,
    onPageHide: null,
    onPageShow: null,
    onPaste: null,
    onPause: null,
    onPlay: null,
    onPlaying: null,
    onPopState: null,
    onProgress: null,
    onRateChange: null,
    onRepeat: null,
    onReset: null,
    onResize: null,
    onScroll: null,
    onSeeked: null,
    onSeeking: null,
    onSelect: null,
    onShow: null,
    onStalled: null,
    onStorage: null,
    onSubmit: null,
    onSuspend: null,
    onTimeUpdate: null,
    onToggle: null,
    onUnload: null,
    onVolumeChange: null,
    onWaiting: null,
    onZoom: null,
    opacity: null,
    operator: null,
    order: null,
    orient: null,
    orientation: null,
    origin: null,
    overflow: null,
    overlay: null,
    overlinePosition: C,
    overlineThickness: C,
    paintOrder: null,
    panose1: null,
    path: null,
    pathLength: C,
    patternContentUnits: null,
    patternTransform: null,
    patternUnits: null,
    phase: null,
    ping: ie,
    pitch: null,
    playbackOrder: null,
    pointerEvents: null,
    points: null,
    pointsAtX: C,
    pointsAtY: C,
    pointsAtZ: C,
    preserveAlpha: null,
    preserveAspectRatio: null,
    primitiveUnits: null,
    propagate: null,
    property: ge,
    r: null,
    radius: null,
    referrerPolicy: null,
    refX: null,
    refY: null,
    rel: ge,
    rev: ge,
    renderingIntent: null,
    repeatCount: null,
    repeatDur: null,
    requiredExtensions: ge,
    requiredFeatures: ge,
    requiredFonts: ge,
    requiredFormats: ge,
    resource: null,
    restart: null,
    result: null,
    rotate: null,
    rx: null,
    ry: null,
    scale: null,
    seed: null,
    shapeRendering: null,
    side: null,
    slope: null,
    snapshotTime: null,
    specularConstant: C,
    specularExponent: C,
    spreadMethod: null,
    spacing: null,
    startOffset: null,
    stdDeviation: null,
    stemh: null,
    stemv: null,
    stitchTiles: null,
    stopColor: null,
    stopOpacity: null,
    strikethroughPosition: C,
    strikethroughThickness: C,
    string: null,
    stroke: null,
    strokeDashArray: ge,
    strokeDashOffset: null,
    strokeLineCap: null,
    strokeLineJoin: null,
    strokeMiterLimit: C,
    strokeOpacity: C,
    strokeWidth: null,
    style: null,
    surfaceScale: C,
    syncBehavior: null,
    syncBehaviorDefault: null,
    syncMaster: null,
    syncTolerance: null,
    syncToleranceDefault: null,
    systemLanguage: ge,
    tabIndex: C,
    tableValues: null,
    target: null,
    targetX: C,
    targetY: C,
    textAnchor: null,
    textDecoration: null,
    textRendering: null,
    textLength: null,
    timelineBegin: null,
    title: null,
    transformBehavior: null,
    type: null,
    typeOf: ge,
    to: null,
    transform: null,
    u1: null,
    u2: null,
    underlinePosition: C,
    underlineThickness: C,
    unicode: null,
    unicodeBidi: null,
    unicodeRange: null,
    unitsPerEm: C,
    values: null,
    vAlphabetic: C,
    vMathematical: C,
    vectorEffect: null,
    vHanging: C,
    vIdeographic: C,
    version: null,
    vertAdvY: C,
    vertOriginX: C,
    vertOriginY: C,
    viewBox: null,
    viewTarget: null,
    visibility: null,
    width: null,
    widths: null,
    wordSpacing: null,
    writingMode: null,
    x: null,
    x1: null,
    x2: null,
    xChannelSelector: null,
    xHeight: C,
    y: null,
    y1: null,
    y2: null,
    yChannelSelector: null,
    z: null,
    zoomAndPan: null
  }
}), mu = /^data[-\w.:]+$/i, Bt = /-[a-z]/g, gu = /[A-Z]/g;
function yu(e, n) {
  const t = Fn(n);
  let r = n, i = Se;
  if (t in e.normal)
    return e.property[e.normal[t]];
  if (t.length > 4 && t.slice(0, 4) === "data" && mu.test(n)) {
    if (n.charAt(4) === "-") {
      const l = n.slice(5).replace(Bt, bu);
      r = "data" + l.charAt(0).toUpperCase() + l.slice(1);
    } else {
      const l = n.slice(4);
      if (!Bt.test(l)) {
        let o = l.replace(gu, xu);
        o.charAt(0) !== "-" && (o = "-" + o), n = "data" + o;
      }
    }
    i = Un;
  }
  return new i(r, n);
}
function xu(e) {
  return "-" + e.toLowerCase();
}
function bu(e) {
  return e.charAt(1).toUpperCase();
}
const Nt = {
  classId: "classID",
  dataType: "datatype",
  itemId: "itemID",
  strokeDashArray: "strokeDasharray",
  strokeDashOffset: "strokeDashoffset",
  strokeLineCap: "strokeLinecap",
  strokeLineJoin: "strokeLinejoin",
  strokeMiterLimit: "strokeMiterlimit",
  typeOf: "typeof",
  xLinkActuate: "xlinkActuate",
  xLinkArcRole: "xlinkArcrole",
  xLinkHref: "xlinkHref",
  xLinkRole: "xlinkRole",
  xLinkShow: "xlinkShow",
  xLinkTitle: "xlinkTitle",
  xLinkType: "xlinkType",
  xmlnsXLink: "xmlnsXlink"
}, ku = Sr([vr, Cr, Ar, Or, hu], "html"), wu = Sr([vr, Cr, Ar, Or, du], "svg"), Ir = (
  /**
   * @type {(
   *   (<Kind extends Node>(test: PredicateTest<Kind>) => AssertPredicate<Kind>) &
   *   ((test?: Test) => AssertAnything)
   * )}
   */
  /**
   * @param {Test} [test]
   * @returns {AssertAnything}
   */
  function(e) {
    if (e == null)
      return vu;
    if (typeof e == "string")
      return Cu(e);
    if (typeof e == "object")
      return Array.isArray(e) ? Su(e) : Eu(e);
    if (typeof e == "function")
      return ln(e);
    throw new Error("Expected function, string, or object as test");
  }
);
function Su(e) {
  const n = [];
  let t = -1;
  for (; ++t < e.length; )
    n[t] = Ir(e[t]);
  return ln(r);
  function r(...i) {
    let l = -1;
    for (; ++l < n.length; )
      if (n[l].call(this, ...i))
        return !0;
    return !1;
  }
}
function Eu(e) {
  return ln(n);
  function n(t) {
    let r;
    for (r in e)
      if (t[r] !== e[r])
        return !1;
    return !0;
  }
}
function Cu(e) {
  return ln(n);
  function n(t) {
    return t && t.type === e;
  }
}
function ln(e) {
  return n;
  function n(t, ...r) {
    return !!(t && typeof t == "object" && "type" in t && e.call(this, t, ...r));
  }
}
function vu() {
  return !0;
}
const Tu = !0, jt = !1, Pu = "skip", Au = (
  /**
   * @type {(
   *   (<Tree extends Node, Check extends Test>(tree: Tree, test: Check, visitor: BuildVisitor<Tree, Check>, reverse?: boolean | null | undefined) => void) &
   *   (<Tree extends Node>(tree: Tree, visitor: BuildVisitor<Tree>, reverse?: boolean | null | undefined) => void)
   * )}
   */
  /**
   * @param {Node} tree
   * @param {Test} test
   * @param {Visitor<Node>} visitor
   * @param {boolean | null | undefined} [reverse]
   * @returns {void}
   */
  function(e, n, t, r) {
    typeof n == "function" && typeof t != "function" && (r = t, t = n, n = null);
    const i = Ir(n), l = r ? -1 : 1;
    o(e, void 0, [])();
    function o(u, a, f) {
      const c = u && typeof u == "object" ? u : {};
      if (typeof c.type == "string") {
        const g = (
          // `hast`
          typeof c.tagName == "string" ? c.tagName : (
            // `xast`
            typeof c.name == "string" ? c.name : void 0
          )
        );
        Object.defineProperty(h, "name", {
          value: "node (" + (u.type + (g ? "<" + g + ">" : "")) + ")"
        });
      }
      return h;
      function h() {
        let g = [], m, d, y;
        if ((!n || i(u, a, f[f.length - 1] || null)) && (g = Ou(t(u, f)), g[0] === jt))
          return g;
        if (u.children && g[0] !== Pu)
          for (d = (r ? u.children.length : -1) + l, y = f.concat(u); d > -1 && d < u.children.length; ) {
            if (m = o(u.children[d], d, y)(), m[0] === jt)
              return m;
            d = typeof m[1] == "number" ? m[1] : d + l;
          }
        return g;
      }
    }
  }
);
function Ou(e) {
  return Array.isArray(e) ? e : typeof e == "number" ? [Tu, e] : [e];
}
const Iu = (
  /**
   * @type {(
   *   (<Tree extends Node, Check extends Test>(tree: Tree, test: Check, visitor: BuildVisitor<Tree, Check>, reverse?: boolean | null | undefined) => void) &
   *   (<Tree extends Node>(tree: Tree, visitor: BuildVisitor<Tree>, reverse?: boolean | null | undefined) => void)
   * )}
   */
  /**
   * @param {Node} tree
   * @param {Test} test
   * @param {Visitor} visitor
   * @param {boolean | null | undefined} [reverse]
   * @returns {void}
   */
  function(e, n, t, r) {
    typeof n == "function" && typeof t != "function" && (r = t, t = n, n = null), Au(e, n, i, r);
    function i(l, o) {
      const u = o[o.length - 1];
      return t(
        l,
        u ? u.children.indexOf(l) : null,
        u
      );
    }
  }
);
function Fu(e) {
  if (e.allowedElements && e.disallowedElements)
    throw new TypeError(
      "Only one of `allowedElements` and `disallowedElements` should be defined"
    );
  if (e.allowedElements || e.disallowedElements || e.allowElement)
    return (n) => {
      Iu(n, "element", (t, r, i) => {
        const l = (
          /** @type {Element|Root} */
          i
        );
        let o;
        if (e.allowedElements ? o = !e.allowedElements.includes(t.tagName) : e.disallowedElements && (o = e.disallowedElements.includes(t.tagName)), !o && e.allowElement && typeof r == "number" && (o = !e.allowElement(t, r, l)), o && typeof r == "number")
          return e.unwrapDisallowed && t.children ? l.children.splice(r, 1, ...t.children) : l.children.splice(r, 1), r;
      });
    };
}
var _n = { exports: {} }, G = {};
/**
 * @license React
 * react-is.production.min.js
 *
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
var $t;
function Ru() {
  if ($t)
    return G;
  $t = 1;
  var e = Symbol.for("react.element"), n = Symbol.for("react.portal"), t = Symbol.for("react.fragment"), r = Symbol.for("react.strict_mode"), i = Symbol.for("react.profiler"), l = Symbol.for("react.provider"), o = Symbol.for("react.context"), u = Symbol.for("react.server_context"), a = Symbol.for("react.forward_ref"), f = Symbol.for("react.suspense"), c = Symbol.for("react.suspense_list"), h = Symbol.for("react.memo"), g = Symbol.for("react.lazy"), m = Symbol.for("react.offscreen"), d;
  d = Symbol.for("react.module.reference");
  function y(x) {
    if (typeof x == "object" && x !== null) {
      var b = x.$$typeof;
      switch (b) {
        case e:
          switch (x = x.type, x) {
            case t:
            case i:
            case r:
            case f:
            case c:
              return x;
            default:
              switch (x = x && x.$$typeof, x) {
                case u:
                case o:
                case a:
                case g:
                case h:
                case l:
                  return x;
                default:
                  return b;
              }
          }
        case n:
          return b;
      }
    }
  }
  return G.ContextConsumer = o, G.ContextProvider = l, G.Element = e, G.ForwardRef = a, G.Fragment = t, G.Lazy = g, G.Memo = h, G.Portal = n, G.Profiler = i, G.StrictMode = r, G.Suspense = f, G.SuspenseList = c, G.isAsyncMode = function() {
    return !1;
  }, G.isConcurrentMode = function() {
    return !1;
  }, G.isContextConsumer = function(x) {
    return y(x) === o;
  }, G.isContextProvider = function(x) {
    return y(x) === l;
  }, G.isElement = function(x) {
    return typeof x == "object" && x !== null && x.$$typeof === e;
  }, G.isForwardRef = function(x) {
    return y(x) === a;
  }, G.isFragment = function(x) {
    return y(x) === t;
  }, G.isLazy = function(x) {
    return y(x) === g;
  }, G.isMemo = function(x) {
    return y(x) === h;
  }, G.isPortal = function(x) {
    return y(x) === n;
  }, G.isProfiler = function(x) {
    return y(x) === i;
  }, G.isStrictMode = function(x) {
    return y(x) === r;
  }, G.isSuspense = function(x) {
    return y(x) === f;
  }, G.isSuspenseList = function(x) {
    return y(x) === c;
  }, G.isValidElementType = function(x) {
    return typeof x == "string" || typeof x == "function" || x === t || x === i || x === r || x === f || x === c || x === m || typeof x == "object" && x !== null && (x.$$typeof === g || x.$$typeof === h || x.$$typeof === l || x.$$typeof === o || x.$$typeof === a || x.$$typeof === d || x.getModuleId !== void 0);
  }, G.typeOf = y, G;
}
var Z = {}, Ut;
function _u() {
  return Ut || (Ut = 1, _index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.p.env.NODE_ENV !== "production" && function() {
    var e = Symbol.for("react.element"), n = Symbol.for("react.portal"), t = Symbol.for("react.fragment"), r = Symbol.for("react.strict_mode"), i = Symbol.for("react.profiler"), l = Symbol.for("react.provider"), o = Symbol.for("react.context"), u = Symbol.for("react.server_context"), a = Symbol.for("react.forward_ref"), f = Symbol.for("react.suspense"), c = Symbol.for("react.suspense_list"), h = Symbol.for("react.memo"), g = Symbol.for("react.lazy"), m = Symbol.for("react.offscreen"), d = !1, y = !1, x = !1, b = !1, z = !1, v;
    v = Symbol.for("react.module.reference");
    function F(I) {
      return !!(typeof I == "string" || typeof I == "function" || I === t || I === i || z || I === r || I === f || I === c || b || I === m || d || y || x || typeof I == "object" && I !== null && (I.$$typeof === g || I.$$typeof === h || I.$$typeof === l || I.$$typeof === o || I.$$typeof === a || // This needs to include all possible module reference object
      // types supported by any Flight configuration anywhere since
      // we don't know which Flight build this will end up being used
      // with.
      I.$$typeof === v || I.getModuleId !== void 0));
    }
    function S(I) {
      if (typeof I == "object" && I !== null) {
        var Oe = I.$$typeof;
        switch (Oe) {
          case e:
            var _e = I.type;
            switch (_e) {
              case t:
              case i:
              case r:
              case f:
              case c:
                return _e;
              default:
                var $e = _e && _e.$$typeof;
                switch ($e) {
                  case u:
                  case o:
                  case a:
                  case g:
                  case h:
                  case l:
                    return $e;
                  default:
                    return Oe;
                }
            }
          case n:
            return Oe;
        }
      }
    }
    var k = o, _ = l, U = e, Y = a, le = t, P = g, R = h, X = n, te = i, he = r, oe = f, ue = c, re = !1, ae = !1;
    function E(I) {
      return re || (re = !0, console.warn("The ReactIs.isAsyncMode() alias has been deprecated, and will be removed in React 18+.")), !1;
    }
    function s(I) {
      return ae || (ae = !0, console.warn("The ReactIs.isConcurrentMode() alias has been deprecated, and will be removed in React 18+.")), !1;
    }
    function p(I) {
      return S(I) === o;
    }
    function B(I) {
      return S(I) === l;
    }
    function D(I) {
      return typeof I == "object" && I !== null && I.$$typeof === e;
    }
    function q(I) {
      return S(I) === a;
    }
    function O(I) {
      return S(I) === t;
    }
    function L(I) {
      return S(I) === g;
    }
    function $(I) {
      return S(I) === h;
    }
    function W(I) {
      return S(I) === n;
    }
    function H(I) {
      return S(I) === i;
    }
    function ce(I) {
      return S(I) === r;
    }
    function A(I) {
      return S(I) === f;
    }
    function de(I) {
      return S(I) === c;
    }
    Z.ContextConsumer = k, Z.ContextProvider = _, Z.Element = U, Z.ForwardRef = Y, Z.Fragment = le, Z.Lazy = P, Z.Memo = R, Z.Portal = X, Z.Profiler = te, Z.StrictMode = he, Z.Suspense = oe, Z.SuspenseList = ue, Z.isAsyncMode = E, Z.isConcurrentMode = s, Z.isContextConsumer = p, Z.isContextProvider = B, Z.isElement = D, Z.isForwardRef = q, Z.isFragment = O, Z.isLazy = L, Z.isMemo = $, Z.isPortal = W, Z.isProfiler = H, Z.isStrictMode = ce, Z.isSuspense = A, Z.isSuspenseList = de, Z.isValidElementType = F, Z.typeOf = S;
  }()), Z;
}
_index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.p.env.NODE_ENV === "production" ? _n.exports = Ru() : _n.exports = _u();
var Lu = _n.exports;
const Du = /* @__PURE__ */ (0,_index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.g)(Lu);
function zu(e) {
  const n = (
    // @ts-expect-error looks like a node.
    e && typeof e == "object" && e.type === "text" ? (
      // @ts-expect-error looks like a text.
      e.value || ""
    ) : e
  );
  return typeof n == "string" && n.replace(/[ \t\n\f\r]/g, "") === "";
}
function Mu(e) {
  return e.join(" ").trim();
}
function Bu(e, n) {
  const t = n || {};
  return (e[e.length - 1] === "" ? [...e, ""] : e).join(
    (t.padRight ? " " : "") + "," + (t.padLeft === !1 ? "" : " ")
  ).trim();
}
var qn = { exports: {} }, qt = /\/\*[^*]*\*+([^/*][^*]*\*+)*\//g, Nu = /\n/g, ju = /^\s*/, $u = /^(\*?[-#/*\\\w]+(\[[0-9a-z_-]+\])?)\s*/, Uu = /^:\s*/, qu = /^((?:'(?:\\'|.)*?'|"(?:\\"|.)*?"|\([^)]*?\)|[^};])+)/, Hu = /^[;\s]*/, Vu = /^\s+|\s+$/g, Yu = `
`, Ht = "/", Vt = "*", Le = "", Wu = "comment", Xu = "declaration", Qu = function(e, n) {
  if (typeof e != "string")
    throw new TypeError("First argument must be a string");
  if (!e)
    return [];
  n = n || {};
  var t = 1, r = 1;
  function i(d) {
    var y = d.match(Nu);
    y && (t += y.length);
    var x = d.lastIndexOf(Yu);
    r = ~x ? d.length - x : r + d.length;
  }
  function l() {
    var d = { line: t, column: r };
    return function(y) {
      return y.position = new o(d), f(), y;
    };
  }
  function o(d) {
    this.start = d, this.end = { line: t, column: r }, this.source = n.source;
  }
  o.prototype.content = e;
  function u(d) {
    var y = new Error(
      n.source + ":" + t + ":" + r + ": " + d
    );
    if (y.reason = d, y.filename = n.source, y.line = t, y.column = r, y.source = e, !n.silent)
      throw y;
  }
  function a(d) {
    var y = d.exec(e);
    if (y) {
      var x = y[0];
      return i(x), e = e.slice(x.length), y;
    }
  }
  function f() {
    a(ju);
  }
  function c(d) {
    var y;
    for (d = d || []; y = h(); )
      y !== !1 && d.push(y);
    return d;
  }
  function h() {
    var d = l();
    if (!(Ht != e.charAt(0) || Vt != e.charAt(1))) {
      for (var y = 2; Le != e.charAt(y) && (Vt != e.charAt(y) || Ht != e.charAt(y + 1)); )
        ++y;
      if (y += 2, Le === e.charAt(y - 1))
        return u("End of comment missing");
      var x = e.slice(2, y - 2);
      return r += 2, i(x), e = e.slice(y), r += 2, d({
        type: Wu,
        comment: x
      });
    }
  }
  function g() {
    var d = l(), y = a($u);
    if (y) {
      if (h(), !a(Uu))
        return u("property missing ':'");
      var x = a(qu), b = d({
        type: Xu,
        property: Yt(y[0].replace(qt, Le)),
        value: x ? Yt(x[0].replace(qt, Le)) : Le
      });
      return a(Hu), b;
    }
  }
  function m() {
    var d = [];
    c(d);
    for (var y; y = g(); )
      y !== !1 && (d.push(y), c(d));
    return d;
  }
  return f(), m();
};
function Yt(e) {
  return e ? e.replace(Vu, Le) : Le;
}
var Ku = Qu;
function Fr(e, n) {
  var t = null;
  if (!e || typeof e != "string")
    return t;
  for (var r, i = Ku(e), l = typeof n == "function", o, u, a = 0, f = i.length; a < f; a++)
    r = i[a], o = r.property, u = r.value, l ? n(o, u, r) : u && (t || (t = {}), t[o] = u);
  return t;
}
qn.exports = Fr;
qn.exports.default = Fr;
var Gu = qn.exports;
const Zu = /* @__PURE__ */ (0,_index_1993541e_js__WEBPACK_IMPORTED_MODULE_1__.g)(Gu), Ln = {}.hasOwnProperty, Ju = /* @__PURE__ */ new Set(["table", "thead", "tbody", "tfoot", "tr"]);
function Rr(e, n) {
  const t = [];
  let r = -1, i;
  for (; ++r < n.children.length; )
    i = n.children[r], i.type === "element" ? t.push(ea(e, i, r, n)) : i.type === "text" ? (n.type !== "element" || !Ju.has(n.tagName) || !zu(i)) && t.push(i.value) : i.type === "raw" && !e.options.skipHtml && t.push(i.value);
  return t;
}
function ea(e, n, t, r) {
  const i = e.options, l = i.transformLinkUri === void 0 ? Nr : i.transformLinkUri, o = e.schema, u = n.tagName, a = {};
  let f = o, c;
  if (o.space === "html" && u === "svg" && (f = wu, e.schema = f), n.properties)
    for (c in n.properties)
      Ln.call(n.properties, c) && ta(a, c, n.properties[c], e);
  (u === "ol" || u === "ul") && e.listDepth++;
  const h = Rr(e, n);
  (u === "ol" || u === "ul") && e.listDepth--, e.schema = o;
  const g = n.position || {
    start: { line: null, column: null, offset: null },
    end: { line: null, column: null, offset: null }
  }, m = i.components && Ln.call(i.components, u) ? i.components[u] : u, d = typeof m == "string" || m === react__WEBPACK_IMPORTED_MODULE_0__.Fragment;
  if (!Du.isValidElementType(m))
    throw new TypeError(
      `Component for name \`${u}\` not defined or is not renderable`
    );
  if (a.key = t, u === "a" && i.linkTarget && (a.target = typeof i.linkTarget == "function" ? i.linkTarget(
    String(a.href || ""),
    n.children,
    typeof a.title == "string" ? a.title : null
  ) : i.linkTarget), u === "a" && l && (a.href = l(
    String(a.href || ""),
    n.children,
    typeof a.title == "string" ? a.title : null
  )), !d && u === "code" && r.type === "element" && r.tagName !== "pre" && (a.inline = !0), !d && (u === "h1" || u === "h2" || u === "h3" || u === "h4" || u === "h5" || u === "h6") && (a.level = Number.parseInt(u.charAt(1), 10)), u === "img" && i.transformImageUri && (a.src = i.transformImageUri(
    String(a.src || ""),
    String(a.alt || ""),
    typeof a.title == "string" ? a.title : null
  )), !d && u === "li" && r.type === "element") {
    const y = na(n);
    a.checked = y && y.properties ? !!y.properties.checked : null, a.index = Sn(r, n), a.ordered = r.tagName === "ol";
  }
  return !d && (u === "ol" || u === "ul") && (a.ordered = u === "ol", a.depth = e.listDepth), (u === "td" || u === "th") && (a.align && (a.style || (a.style = {}), a.style.textAlign = a.align, delete a.align), d || (a.isHeader = u === "th")), !d && u === "tr" && r.type === "element" && (a.isHeader = r.tagName === "thead"), i.sourcePos && (a["data-sourcepos"] = la(g)), !d && i.rawSourcePos && (a.sourcePosition = n.position), !d && i.includeElementIndex && (a.index = Sn(r, n), a.siblingCount = Sn(r)), d || (a.node = n), h.length > 0 ? react__WEBPACK_IMPORTED_MODULE_0__.createElement(m, a, h) : react__WEBPACK_IMPORTED_MODULE_0__.createElement(m, a);
}
function na(e) {
  let n = -1;
  for (; ++n < e.children.length; ) {
    const t = e.children[n];
    if (t.type === "element" && t.tagName === "input")
      return t;
  }
  return null;
}
function Sn(e, n) {
  let t = -1, r = 0;
  for (; ++t < e.children.length && e.children[t] !== n; )
    e.children[t].type === "element" && r++;
  return r;
}
function ta(e, n, t, r) {
  const i = yu(r.schema, n);
  let l = t;
  l == null || l !== l || (Array.isArray(l) && (l = i.commaSeparated ? Bu(l) : Mu(l)), i.property === "style" && typeof l == "string" && (l = ra(l)), i.space && i.property ? e[Ln.call(Nt, i.property) ? Nt[i.property] : i.property] = l : i.attribute && (e[i.attribute] = l));
}
function ra(e) {
  const n = {};
  try {
    Zu(e, t);
  } catch {
  }
  return n;
  function t(r, i) {
    const l = r.slice(0, 4) === "-ms-" ? `ms-${r.slice(4)}` : r;
    n[l.replace(/-([a-z])/g, ia)] = i;
  }
}
function ia(e, n) {
  return n.toUpperCase();
}
function la(e) {
  return [
    e.start.line,
    ":",
    e.start.column,
    "-",
    e.end.line,
    ":",
    e.end.column
  ].map(String).join("");
}
const Wt = {}.hasOwnProperty, oa = "https://github.com/remarkjs/react-markdown/blob/main/changelog.md", Ge = {
  plugins: { to: "remarkPlugins", id: "change-plugins-to-remarkplugins" },
  renderers: { to: "components", id: "change-renderers-to-components" },
  astPlugins: { id: "remove-buggy-html-in-markdown-parser" },
  allowDangerousHtml: { id: "remove-buggy-html-in-markdown-parser" },
  escapeHtml: { id: "remove-buggy-html-in-markdown-parser" },
  source: { to: "children", id: "change-source-to-children" },
  allowNode: {
    to: "allowElement",
    id: "replace-allownode-allowedtypes-and-disallowedtypes"
  },
  allowedTypes: {
    to: "allowedElements",
    id: "replace-allownode-allowedtypes-and-disallowedtypes"
  },
  disallowedTypes: {
    to: "disallowedElements",
    id: "replace-allownode-allowedtypes-and-disallowedtypes"
  },
  includeNodeIndex: {
    to: "includeElementIndex",
    id: "change-includenodeindex-to-includeelementindex"
  }
};
function ua(e) {
  for (const l in Ge)
    if (Wt.call(Ge, l) && Wt.call(e, l)) {
      const o = Ge[l];
      console.warn(
        `[react-markdown] Warning: please ${o.to ? `use \`${o.to}\` instead of` : "remove"} \`${l}\` (see <${oa}#${o.id}> for more info)`
      ), delete Ge[l];
    }
  const n = ti().use(uo).use(e.remarkPlugins || []).use(Jo, {
    ...e.remarkRehypeOptions,
    allowDangerousHtml: !0
  }).use(e.rehypePlugins || []).use(Fu, e), t = new Qt();
  typeof e.children == "string" ? t.value = e.children : e.children !== void 0 && e.children !== null && console.warn(
    `[react-markdown] Warning: please pass a string as \`children\` (not: \`${e.children}\`)`
  );
  const r = n.runSync(n.parse(t), t);
  if (r.type !== "root")
    throw new TypeError("Expected a `root` node");
  let i = react__WEBPACK_IMPORTED_MODULE_0__.createElement(
    react__WEBPACK_IMPORTED_MODULE_0__.Fragment,
    {},
    Rr({ options: e, schema: ku, listDepth: 0 }, r)
  );
  return e.className && (i = react__WEBPACK_IMPORTED_MODULE_0__.createElement("div", { className: e.className }, i)), i;
}
ua.propTypes = {
  // Core options:
  children: j.string,
  // Layout options:
  className: j.string,
  // Filter options:
  allowElement: j.func,
  allowedElements: j.arrayOf(j.string),
  disallowedElements: j.arrayOf(j.string),
  unwrapDisallowed: j.bool,
  // Plugin options:
  remarkPlugins: j.arrayOf(
    j.oneOfType([
      j.object,
      j.func,
      j.arrayOf(
        j.oneOfType([
          j.bool,
          j.string,
          j.object,
          j.func,
          j.arrayOf(
            // prettier-ignore
            // type-coverage:ignore-next-line
            j.any
          )
        ])
      )
    ])
  ),
  rehypePlugins: j.arrayOf(
    j.oneOfType([
      j.object,
      j.func,
      j.arrayOf(
        j.oneOfType([
          j.bool,
          j.string,
          j.object,
          j.func,
          j.arrayOf(
            // prettier-ignore
            // type-coverage:ignore-next-line
            j.any
          )
        ])
      )
    ])
  ),
  // Transform options:
  sourcePos: j.bool,
  rawSourcePos: j.bool,
  skipHtml: j.bool,
  includeElementIndex: j.bool,
  transformLinkUri: j.oneOfType([j.func, j.bool]),
  linkTarget: j.oneOfType([j.func, j.string]),
  transformImageUri: j.func,
  components: j.object
};



/***/ })

};
;