"use strict";
exports.id = 825;
exports.ids = [825];
exports.modules = {

/***/ 9825:
/***/ ((__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   "default": () => (/* binding */ Fu),
/* harmony export */   uriTransformer: () => (/* binding */ gr)
/* harmony export */ });
/* harmony import */ var react__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(7294);
/* harmony import */ var _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(8713);


const ze = ["http", "https", "mailto", "tel"];
function gr(n) {
  const e = (n || "").trim(), t = e.charAt(0);
  if (t === "#" || t === "/")
    return e;
  const r = e.indexOf(":");
  if (r === -1)
    return e;
  let i = -1;
  for (; ++i < ze.length; ) {
    const l = ze[i];
    if (r === l.length && e.slice(0, l.length).toLowerCase() === l)
      return e;
  }
  return i = e.indexOf("?"), i !== -1 && r > i || (i = e.indexOf("#"), i !== -1 && r > i) ? e : "javascript:void(0)";
}
/*!
 * Determine if an object is a Buffer
 *
 * @author   Feross Aboukhadijeh <https://feross.org>
 * @license  MIT
 */
var yr = function(e) {
  return e != null && e.constructor != null && typeof e.constructor.isBuffer == "function" && e.constructor.isBuffer(e);
};
const Pt = yr;
function xr(n) {
  return !n || typeof n != "object" ? "" : "position" in n || "type" in n ? Re(n.position) : "start" in n || "end" in n ? Re(n) : "line" in n || "column" in n ? he(n) : "";
}
function he(n) {
  return _e(n && n.line) + ":" + _e(n && n.column);
}
function Re(n) {
  return he(n && n.start) + "-" + he(n && n.end);
}
function _e(n) {
  return n && typeof n == "number" ? n : 1;
}
class cn extends Error {
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
  constructor(e, t, r) {
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
    t && ("type" in t || "position" in t ? t.position && (l = t.position) : "start" in t || "end" in t ? l = t : ("line" in t || "column" in t) && (l.start = t)), this.name = xr(t) || "1:1", this.message = typeof e == "object" ? e.message : e, this.stack = "", typeof e == "object" && e.stack && (this.stack = e.stack), this.reason = this.message, this.fatal, this.line = l.start.line, this.column = l.start.column, this.position = l, this.source = i[0], this.ruleId = i[1], this.file, this.actual, this.expected, this.url, this.note;
  }
}
cn.prototype.file = "";
cn.prototype.name = "";
cn.prototype.reason = "";
cn.prototype.message = "";
cn.prototype.stack = "";
cn.prototype.fatal = null;
cn.prototype.column = null;
cn.prototype.line = null;
cn.prototype.source = null;
cn.prototype.ruleId = null;
cn.prototype.position = null;
const mn = { basename: kr, dirname: br, extname: wr, join: Sr, sep: "/" };
function kr(n, e) {
  if (e !== void 0 && typeof e != "string")
    throw new TypeError('"ext" argument must be a string');
  Hn(n);
  let t = 0, r = -1, i = n.length, l;
  if (e === void 0 || e.length === 0 || e.length > n.length) {
    for (; i--; )
      if (n.charCodeAt(i) === 47) {
        if (l) {
          t = i + 1;
          break;
        }
      } else
        r < 0 && (l = !0, r = i + 1);
    return r < 0 ? "" : n.slice(t, r);
  }
  if (e === n)
    return "";
  let o = -1, u = e.length - 1;
  for (; i--; )
    if (n.charCodeAt(i) === 47) {
      if (l) {
        t = i + 1;
        break;
      }
    } else
      o < 0 && (l = !0, o = i + 1), u > -1 && (n.charCodeAt(i) === e.charCodeAt(u--) ? u < 0 && (r = i) : (u = -1, r = o));
  return t === r ? r = o : r < 0 && (r = n.length), n.slice(t, r);
}
function br(n) {
  if (Hn(n), n.length === 0)
    return ".";
  let e = -1, t = n.length, r;
  for (; --t; )
    if (n.charCodeAt(t) === 47) {
      if (r) {
        e = t;
        break;
      }
    } else
      r || (r = !0);
  return e < 0 ? n.charCodeAt(0) === 47 ? "/" : "." : e === 1 && n.charCodeAt(0) === 47 ? "//" : n.slice(0, e);
}
function wr(n) {
  Hn(n);
  let e = n.length, t = -1, r = 0, i = -1, l = 0, o;
  for (; e--; ) {
    const u = n.charCodeAt(e);
    if (u === 47) {
      if (o) {
        r = e + 1;
        break;
      }
      continue;
    }
    t < 0 && (o = !0, t = e + 1), u === 46 ? i < 0 ? i = e : l !== 1 && (l = 1) : i > -1 && (l = -1);
  }
  return i < 0 || t < 0 || // We saw a non-dot character immediately before the dot.
  l === 0 || // The (right-most) trimmed path component is exactly `..`.
  l === 1 && i === t - 1 && i === r + 1 ? "" : n.slice(i, t);
}
function Sr(...n) {
  let e = -1, t;
  for (; ++e < n.length; )
    Hn(n[e]), n[e] && (t = t === void 0 ? n[e] : t + "/" + n[e]);
  return t === void 0 ? "." : Er(t);
}
function Er(n) {
  Hn(n);
  const e = n.charCodeAt(0) === 47;
  let t = Cr(n, !e);
  return t.length === 0 && !e && (t = "."), t.length > 0 && n.charCodeAt(n.length - 1) === 47 && (t += "/"), e ? "/" + t : t;
}
function Cr(n, e) {
  let t = "", r = 0, i = -1, l = 0, o = -1, u, a;
  for (; ++o <= n.length; ) {
    if (o < n.length)
      u = n.charCodeAt(o);
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
          e && (t = t.length > 0 ? t + "/.." : "..", r = 2);
        } else
          t.length > 0 ? t += "/" + n.slice(i + 1, o) : t = n.slice(i + 1, o), r = o - i - 1;
      i = o, l = 0;
    } else
      u === 46 && l > -1 ? l++ : l = -1;
  }
  return t;
}
function Hn(n) {
  if (typeof n != "string")
    throw new TypeError(
      "Path must be a string. Received " + JSON.stringify(n)
    );
}
const Ar = { cwd: Pr };
function Pr() {
  return "/";
}
function me(n) {
  return n !== null && typeof n == "object" && // @ts-expect-error: indexable.
  n.href && // @ts-expect-error: indexable.
  n.origin;
}
function Fr(n) {
  if (typeof n == "string")
    n = new URL(n);
  else if (!me(n)) {
    const e = new TypeError(
      'The "path" argument must be of type string or an instance of URL. Received `' + n + "`"
    );
    throw e.code = "ERR_INVALID_ARG_TYPE", e;
  }
  if (n.protocol !== "file:") {
    const e = new TypeError("The URL must be of scheme file");
    throw e.code = "ERR_INVALID_URL_SCHEME", e;
  }
  return Tr(n);
}
function Tr(n) {
  if (n.hostname !== "") {
    const r = new TypeError(
      'File URL host must be "localhost" or empty on darwin'
    );
    throw r.code = "ERR_INVALID_FILE_URL_HOST", r;
  }
  const e = n.pathname;
  let t = -1;
  for (; ++t < e.length; )
    if (e.charCodeAt(t) === 37 && e.charCodeAt(t + 1) === 50) {
      const r = e.charCodeAt(t + 2);
      if (r === 70 || r === 102) {
        const i = new TypeError(
          "File URL path must not include encoded / characters"
        );
        throw i.code = "ERR_INVALID_FILE_URL_PATH", i;
      }
    }
  return decodeURIComponent(e);
}
const re = ["history", "path", "basename", "stem", "extname", "dirname"];
class Ft {
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
  constructor(e) {
    let t;
    e ? typeof e == "string" || Ir(e) ? t = { value: e } : me(e) ? t = { path: e } : t = e : t = {}, this.data = {}, this.messages = [], this.history = [], this.cwd = Ar.cwd(), this.value, this.stored, this.result, this.map;
    let r = -1;
    for (; ++r < re.length; ) {
      const l = re[r];
      l in t && t[l] !== void 0 && t[l] !== null && (this[l] = l === "history" ? [...t[l]] : t[l]);
    }
    let i;
    for (i in t)
      re.includes(i) || (this[i] = t[i]);
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
  set path(e) {
    me(e) && (e = Fr(e)), le(e, "path"), this.path !== e && this.history.push(e);
  }
  /**
   * Get the parent path (example: `'~'`).
   */
  get dirname() {
    return typeof this.path == "string" ? mn.dirname(this.path) : void 0;
  }
  /**
   * Set the parent path (example: `'~'`).
   *
   * Cannot be set if there’s no `path` yet.
   */
  set dirname(e) {
    Me(this.basename, "dirname"), this.path = mn.join(e || "", this.basename);
  }
  /**
   * Get the basename (including extname) (example: `'index.min.js'`).
   */
  get basename() {
    return typeof this.path == "string" ? mn.basename(this.path) : void 0;
  }
  /**
   * Set basename (including extname) (`'index.min.js'`).
   *
   * Cannot contain path separators (`'/'` on unix, macOS, and browsers, `'\'`
   * on windows).
   * Cannot be nullified (use `file.path = file.dirname` instead).
   */
  set basename(e) {
    le(e, "basename"), ie(e, "basename"), this.path = mn.join(this.dirname || "", e);
  }
  /**
   * Get the extname (including dot) (example: `'.js'`).
   */
  get extname() {
    return typeof this.path == "string" ? mn.extname(this.path) : void 0;
  }
  /**
   * Set the extname (including dot) (example: `'.js'`).
   *
   * Cannot contain path separators (`'/'` on unix, macOS, and browsers, `'\'`
   * on windows).
   * Cannot be set if there’s no `path` yet.
   */
  set extname(e) {
    if (ie(e, "extname"), Me(this.dirname, "extname"), e) {
      if (e.charCodeAt(0) !== 46)
        throw new Error("`extname` must start with `.`");
      if (e.includes(".", 1))
        throw new Error("`extname` cannot contain multiple dots");
    }
    this.path = mn.join(this.dirname, this.stem + (e || ""));
  }
  /**
   * Get the stem (basename w/o extname) (example: `'index.min'`).
   */
  get stem() {
    return typeof this.path == "string" ? mn.basename(this.path, this.extname) : void 0;
  }
  /**
   * Set the stem (basename w/o extname) (example: `'index.min'`).
   *
   * Cannot contain path separators (`'/'` on unix, macOS, and browsers, `'\'`
   * on windows).
   * Cannot be nullified (use `file.path = file.dirname` instead).
   */
  set stem(e) {
    le(e, "stem"), ie(e, "stem"), this.path = mn.join(this.dirname || "", e + (this.extname || ""));
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
  toString(e) {
    return (this.value || "").toString(e || void 0);
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
  message(e, t, r) {
    const i = new cn(e, t, r);
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
  info(e, t, r) {
    const i = this.message(e, t, r);
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
  fail(e, t, r) {
    const i = this.message(e, t, r);
    throw i.fatal = !0, i;
  }
}
function ie(n, e) {
  if (n && n.includes(mn.sep))
    throw new Error(
      "`" + e + "` cannot be a path: did not expect `" + mn.sep + "`"
    );
}
function le(n, e) {
  if (!n)
    throw new Error("`" + e + "` cannot be empty");
}
function Me(n, e) {
  if (!n)
    throw new Error("Setting `" + e + "` requires `path` to be set too");
}
function Ir(n) {
  return Pt(n);
}
function Be(n) {
  if (n)
    throw n;
}
var Yn = Object.prototype.hasOwnProperty, Tt = Object.prototype.toString, Ne = Object.defineProperty, je = Object.getOwnPropertyDescriptor, $e = function(e) {
  return typeof Array.isArray == "function" ? Array.isArray(e) : Tt.call(e) === "[object Array]";
}, He = function(e) {
  if (!e || Tt.call(e) !== "[object Object]")
    return !1;
  var t = Yn.call(e, "constructor"), r = e.constructor && e.constructor.prototype && Yn.call(e.constructor.prototype, "isPrototypeOf");
  if (e.constructor && !t && !r)
    return !1;
  var i;
  for (i in e)
    ;
  return typeof i > "u" || Yn.call(e, i);
}, Ue = function(e, t) {
  Ne && t.name === "__proto__" ? Ne(e, t.name, {
    enumerable: !0,
    configurable: !0,
    value: t.newValue,
    writable: !0
  }) : e[t.name] = t.newValue;
}, Ve = function(e, t) {
  if (t === "__proto__")
    if (Yn.call(e, t)) {
      if (je)
        return je(e, t).value;
    } else
      return;
  return e[t];
}, qe = function n() {
  var e, t, r, i, l, o, u = arguments[0], a = 1, c = arguments.length, s = !1;
  for (typeof u == "boolean" && (s = u, u = arguments[1] || {}, a = 2), (u == null || typeof u != "object" && typeof u != "function") && (u = {}); a < c; ++a)
    if (e = arguments[a], e != null)
      for (t in e)
        r = Ve(u, t), i = Ve(e, t), u !== i && (s && i && (He(i) || (l = $e(i))) ? (l ? (l = !1, o = r && $e(r) ? r : []) : o = r && He(r) ? r : {}, Ue(u, { name: t, newValue: n(s, o, i) })) : typeof i < "u" && Ue(u, { name: t, newValue: i }));
  return u;
};
function de(n) {
  if (typeof n != "object" || n === null)
    return !1;
  const e = Object.getPrototypeOf(n);
  return (e === null || e === Object.prototype || Object.getPrototypeOf(e) === null) && !(Symbol.toStringTag in n) && !(Symbol.iterator in n);
}
function Or() {
  const n = [], e = { run: t, use: r };
  return e;
  function t(...i) {
    let l = -1;
    const o = i.pop();
    if (typeof o != "function")
      throw new TypeError("Expected function as last argument, not " + o);
    u(null, ...i);
    function u(a, ...c) {
      const s = n[++l];
      let h = -1;
      if (a) {
        o(a);
        return;
      }
      for (; ++h < i.length; )
        (c[h] === null || c[h] === void 0) && (c[h] = i[h]);
      i = c, s ? Lr(s, u)(...c) : o(null, ...c);
    }
  }
  function r(i) {
    if (typeof i != "function")
      throw new TypeError(
        "Expected `middelware` to be a function, not " + i
      );
    return n.push(i), e;
  }
}
function Lr(n, e) {
  let t;
  return r;
  function r(...o) {
    const u = n.length > o.length;
    let a;
    u && o.push(i);
    try {
      a = n.apply(this, o);
    } catch (c) {
      const s = (
        /** @type {Error} */
        c
      );
      if (u && t)
        throw s;
      return i(s);
    }
    u || (a instanceof Promise ? a.then(l, i) : a instanceof Error ? i(a) : l(a));
  }
  function i(o, ...u) {
    t || (t = !0, e(o, ...u));
  }
  function l(o) {
    i(null, o);
  }
}
const Dr = Ot().freeze(), It = {}.hasOwnProperty;
function Ot() {
  const n = Or(), e = [];
  let t = {}, r, i = -1;
  return l.data = o, l.Parser = void 0, l.Compiler = void 0, l.freeze = u, l.attachers = e, l.use = a, l.parse = c, l.stringify = s, l.run = h, l.runSync = g, l.process = d, l.processSync = m, l;
  function l() {
    const y = Ot();
    let x = -1;
    for (; ++x < e.length; )
      y.use(...e[x]);
    return y.data(qe(!0, {}, t)), y;
  }
  function o(y, x) {
    return typeof y == "string" ? arguments.length === 2 ? (ae("data", r), t[y] = x, l) : It.call(t, y) && t[y] || null : y ? (ae("data", r), t = y, l) : t;
  }
  function u() {
    if (r)
      return l;
    for (; ++i < e.length; ) {
      const [y, ...x] = e[i];
      if (x[0] === !1)
        continue;
      x[0] === !0 && (x[0] = void 0);
      const w = y.call(l, ...x);
      typeof w == "function" && n.use(w);
    }
    return r = !0, i = Number.POSITIVE_INFINITY, l;
  }
  function a(y, ...x) {
    let w;
    if (ae("use", r), y != null)
      if (typeof y == "function")
        T(y, ...x);
      else if (typeof y == "object")
        Array.isArray(y) ? _(y) : A(y);
      else
        throw new TypeError("Expected usable value, not `" + y + "`");
    return w && (t.settings = Object.assign(t.settings || {}, w)), l;
    function v(b) {
      if (typeof b == "function")
        T(b);
      else if (typeof b == "object")
        if (Array.isArray(b)) {
          const [I, ...R] = b;
          T(I, ...R);
        } else
          A(b);
      else
        throw new TypeError("Expected usable value, not `" + b + "`");
    }
    function A(b) {
      _(b.plugins), b.settings && (w = Object.assign(w || {}, b.settings));
    }
    function _(b) {
      let I = -1;
      if (b != null)
        if (Array.isArray(b))
          for (; ++I < b.length; ) {
            const R = b[I];
            v(R);
          }
        else
          throw new TypeError("Expected a list of plugins, not `" + b + "`");
    }
    function T(b, I) {
      let R = -1, M;
      for (; ++R < e.length; )
        if (e[R][0] === b) {
          M = e[R];
          break;
        }
      M ? (de(M[1]) && de(I) && (I = qe(!0, M[1], I)), M[1] = I) : e.push([...arguments]);
    }
  }
  function c(y) {
    l.freeze();
    const x = jn(y), w = l.Parser;
    return oe("parse", w), We(w, "parse") ? new w(String(x), x).parse() : w(String(x), x);
  }
  function s(y, x) {
    l.freeze();
    const w = jn(x), v = l.Compiler;
    return ue("stringify", v), Ye(y), We(v, "compile") ? new v(y, w).compile() : v(y, w);
  }
  function h(y, x, w) {
    if (Ye(y), l.freeze(), !w && typeof x == "function" && (w = x, x = void 0), !w)
      return new Promise(v);
    v(null, w);
    function v(A, _) {
      n.run(y, jn(x), T);
      function T(b, I, R) {
        I = I || y, b ? _(b) : A ? A(I) : w(null, I, R);
      }
    }
  }
  function g(y, x) {
    let w, v;
    return l.run(y, x, A), Qe("runSync", "run", v), w;
    function A(_, T) {
      Be(_), w = T, v = !0;
    }
  }
  function d(y, x) {
    if (l.freeze(), oe("process", l.Parser), ue("process", l.Compiler), !x)
      return new Promise(w);
    w(null, x);
    function w(v, A) {
      const _ = jn(y);
      l.run(l.parse(_), _, (b, I, R) => {
        if (b || !I || !R)
          T(b);
        else {
          const M = l.stringify(I, R);
          M == null || (Rr(M) ? R.value = M : R.result = M), T(b, R);
        }
      });
      function T(b, I) {
        b || !I ? A(b) : v ? v(I) : x(null, I);
      }
    }
  }
  function m(y) {
    let x;
    l.freeze(), oe("processSync", l.Parser), ue("processSync", l.Compiler);
    const w = jn(y);
    return l.process(w, v), Qe("processSync", "process", x), w;
    function v(A) {
      x = !0, Be(A);
    }
  }
}
function We(n, e) {
  return typeof n == "function" && // Prototypes do exist.
  // type-coverage:ignore-next-line
  n.prototype && // A function with keys in its prototype is probably a constructor.
  // Classes’ prototype methods are not enumerable, so we check if some value
  // exists in the prototype.
  // type-coverage:ignore-next-line
  (vr(n.prototype) || e in n.prototype);
}
function vr(n) {
  let e;
  for (e in n)
    if (It.call(n, e))
      return !0;
  return !1;
}
function oe(n, e) {
  if (typeof e != "function")
    throw new TypeError("Cannot `" + n + "` without `Parser`");
}
function ue(n, e) {
  if (typeof e != "function")
    throw new TypeError("Cannot `" + n + "` without `Compiler`");
}
function ae(n, e) {
  if (e)
    throw new Error(
      "Cannot call `" + n + "` on a frozen processor.\nCreate a new processor first, by calling it: use `processor()` instead of `processor`."
    );
}
function Ye(n) {
  if (!de(n) || typeof n.type != "string")
    throw new TypeError("Expected node, got `" + n + "`");
}
function Qe(n, e, t) {
  if (!t)
    throw new Error(
      "`" + n + "` finished async. Use `" + e + "` instead"
    );
}
function jn(n) {
  return zr(n) ? n : new Ft(n);
}
function zr(n) {
  return !!(n && typeof n == "object" && "message" in n && "messages" in n);
}
function Rr(n) {
  return typeof n == "string" || Pt(n);
}
function _r(n, e) {
  const t = (e || {}).includeImageAlt;
  return Lt(
    n,
    typeof t == "boolean" ? t : !0
  );
}
function Lt(n, e) {
  return Mr(n) && ("value" in n && n.value || e && "alt" in n && n.alt || "children" in n && Xe(n.children, e)) || Array.isArray(n) && Xe(n, e) || "";
}
function Xe(n, e) {
  const t = [];
  let r = -1;
  for (; ++r < n.length; )
    t[r] = Lt(n[r], e);
  return t.join("");
}
function Mr(n) {
  return !!(n && typeof n == "object");
}
function gn(n, e, t, r) {
  const i = n.length;
  let l = 0, o;
  if (e < 0 ? e = -e > i ? 0 : i + e : e = e > i ? i : e, t = t > 0 ? t : 0, r.length < 1e4)
    o = Array.from(r), o.unshift(e, t), [].splice.apply(n, o);
  else
    for (t && [].splice.apply(n, [e, t]); l < r.length; )
      o = r.slice(l, l + 1e4), o.unshift(e, 0), [].splice.apply(n, o), l += 1e4, e += 1e4;
}
function an(n, e) {
  return n.length > 0 ? (gn(n, n.length, 0, e), n) : e;
}
const Ke = {}.hasOwnProperty;
function Br(n) {
  const e = {};
  let t = -1;
  for (; ++t < n.length; )
    Nr(e, n[t]);
  return e;
}
function Nr(n, e) {
  let t;
  for (t in e) {
    const i = (Ke.call(n, t) ? n[t] : void 0) || (n[t] = {}), l = e[t];
    let o;
    for (o in l) {
      Ke.call(i, o) || (i[o] = []);
      const u = l[o];
      jr(
        // @ts-expect-error Looks like a list.
        i[o],
        Array.isArray(u) ? u : u ? [u] : []
      );
    }
  }
}
function jr(n, e) {
  let t = -1;
  const r = [];
  for (; ++t < e.length; )
    (e[t].add === "after" ? n : r).push(e[t]);
  gn(n, 0, 0, r);
}
const $r = /[!-/:-@[-`{-~\u00A1\u00A7\u00AB\u00B6\u00B7\u00BB\u00BF\u037E\u0387\u055A-\u055F\u0589\u058A\u05BE\u05C0\u05C3\u05C6\u05F3\u05F4\u0609\u060A\u060C\u060D\u061B\u061E\u061F\u066A-\u066D\u06D4\u0700-\u070D\u07F7-\u07F9\u0830-\u083E\u085E\u0964\u0965\u0970\u09FD\u0A76\u0AF0\u0C77\u0C84\u0DF4\u0E4F\u0E5A\u0E5B\u0F04-\u0F12\u0F14\u0F3A-\u0F3D\u0F85\u0FD0-\u0FD4\u0FD9\u0FDA\u104A-\u104F\u10FB\u1360-\u1368\u1400\u166E\u169B\u169C\u16EB-\u16ED\u1735\u1736\u17D4-\u17D6\u17D8-\u17DA\u1800-\u180A\u1944\u1945\u1A1E\u1A1F\u1AA0-\u1AA6\u1AA8-\u1AAD\u1B5A-\u1B60\u1BFC-\u1BFF\u1C3B-\u1C3F\u1C7E\u1C7F\u1CC0-\u1CC7\u1CD3\u2010-\u2027\u2030-\u2043\u2045-\u2051\u2053-\u205E\u207D\u207E\u208D\u208E\u2308-\u230B\u2329\u232A\u2768-\u2775\u27C5\u27C6\u27E6-\u27EF\u2983-\u2998\u29D8-\u29DB\u29FC\u29FD\u2CF9-\u2CFC\u2CFE\u2CFF\u2D70\u2E00-\u2E2E\u2E30-\u2E4F\u2E52\u3001-\u3003\u3008-\u3011\u3014-\u301F\u3030\u303D\u30A0\u30FB\uA4FE\uA4FF\uA60D-\uA60F\uA673\uA67E\uA6F2-\uA6F7\uA874-\uA877\uA8CE\uA8CF\uA8F8-\uA8FA\uA8FC\uA92E\uA92F\uA95F\uA9C1-\uA9CD\uA9DE\uA9DF\uAA5C-\uAA5F\uAADE\uAADF\uAAF0\uAAF1\uABEB\uFD3E\uFD3F\uFE10-\uFE19\uFE30-\uFE52\uFE54-\uFE61\uFE63\uFE68\uFE6A\uFE6B\uFF01-\uFF03\uFF05-\uFF0A\uFF0C-\uFF0F\uFF1A\uFF1B\uFF1F\uFF20\uFF3B-\uFF3D\uFF3F\uFF5B\uFF5D\uFF5F-\uFF65]/, dn = An(/[A-Za-z]/), ge = An(/\d/), Hr = An(/[\dA-Fa-f]/), on = An(/[\dA-Za-z]/), Ur = An(/[!-/:-@[-`{-~]/), Ge = An(/[#-'*+\--9=?A-Z^-~]/);
function ye(n) {
  return (
    // Special whitespace codes (which have negative values), C0 and Control
    // character DEL
    n !== null && (n < 32 || n === 127)
  );
}
function sn(n) {
  return n !== null && (n < 0 || n === 32);
}
function L(n) {
  return n !== null && n < -2;
}
function G(n) {
  return n === -2 || n === -1 || n === 32;
}
const Vr = An(/\s/), qr = An($r);
function An(n) {
  return e;
  function e(t) {
    return t !== null && n.test(String.fromCharCode(t));
  }
}
function U(n, e, t, r) {
  const i = r ? r - 1 : Number.POSITIVE_INFINITY;
  let l = 0;
  return o;
  function o(a) {
    return G(a) ? (n.enter(t), u(a)) : e(a);
  }
  function u(a) {
    return G(a) && l++ < i ? (n.consume(a), u) : (n.exit(t), e(a));
  }
}
const Wr = {
  tokenize: Yr
};
function Yr(n) {
  const e = n.attempt(
    this.parser.constructs.contentInitial,
    r,
    i
  );
  let t;
  return e;
  function r(u) {
    if (u === null) {
      n.consume(u);
      return;
    }
    return n.enter("lineEnding"), n.consume(u), n.exit("lineEnding"), U(n, e, "linePrefix");
  }
  function i(u) {
    return n.enter("paragraph"), l(u);
  }
  function l(u) {
    const a = n.enter("chunkText", {
      contentType: "text",
      previous: t
    });
    return t && (t.next = a), t = a, o(u);
  }
  function o(u) {
    if (u === null) {
      n.exit("chunkText"), n.exit("paragraph"), n.consume(u);
      return;
    }
    return L(u) ? (n.consume(u), n.exit("chunkText"), l) : (n.consume(u), o);
  }
}
const Qr = {
  tokenize: Xr
}, Ze = {
  tokenize: Kr
};
function Xr(n) {
  const e = this, t = [];
  let r = 0, i, l, o;
  return u;
  function u(A) {
    if (r < t.length) {
      const _ = t[r];
      return e.containerState = _[1], n.attempt(
        _[0].continuation,
        a,
        c
      )(A);
    }
    return c(A);
  }
  function a(A) {
    if (r++, e.containerState._closeFlow) {
      e.containerState._closeFlow = void 0, i && v();
      const _ = e.events.length;
      let T = _, b;
      for (; T--; )
        if (e.events[T][0] === "exit" && e.events[T][1].type === "chunkFlow") {
          b = e.events[T][1].end;
          break;
        }
      w(r);
      let I = _;
      for (; I < e.events.length; )
        e.events[I][1].end = Object.assign({}, b), I++;
      return gn(
        e.events,
        T + 1,
        0,
        e.events.slice(_)
      ), e.events.length = I, c(A);
    }
    return u(A);
  }
  function c(A) {
    if (r === t.length) {
      if (!i)
        return g(A);
      if (i.currentConstruct && i.currentConstruct.concrete)
        return m(A);
      e.interrupt = !!(i.currentConstruct && !i._gfmTableDynamicInterruptHack);
    }
    return e.containerState = {}, n.check(
      Ze,
      s,
      h
    )(A);
  }
  function s(A) {
    return i && v(), w(r), g(A);
  }
  function h(A) {
    return e.parser.lazy[e.now().line] = r !== t.length, o = e.now().offset, m(A);
  }
  function g(A) {
    return e.containerState = {}, n.attempt(
      Ze,
      d,
      m
    )(A);
  }
  function d(A) {
    return r++, t.push([e.currentConstruct, e.containerState]), g(A);
  }
  function m(A) {
    if (A === null) {
      i && v(), w(0), n.consume(A);
      return;
    }
    return i = i || e.parser.flow(e.now()), n.enter("chunkFlow", {
      contentType: "flow",
      previous: l,
      _tokenizer: i
    }), y(A);
  }
  function y(A) {
    if (A === null) {
      x(n.exit("chunkFlow"), !0), w(0), n.consume(A);
      return;
    }
    return L(A) ? (n.consume(A), x(n.exit("chunkFlow")), r = 0, e.interrupt = void 0, u) : (n.consume(A), y);
  }
  function x(A, _) {
    const T = e.sliceStream(A);
    if (_ && T.push(null), A.previous = l, l && (l.next = A), l = A, i.defineSkip(A.start), i.write(T), e.parser.lazy[A.start.line]) {
      let b = i.events.length;
      for (; b--; )
        if (
          // The token starts before the line ending…
          i.events[b][1].start.offset < o && // …and either is not ended yet…
          (!i.events[b][1].end || // …or ends after it.
          i.events[b][1].end.offset > o)
        )
          return;
      const I = e.events.length;
      let R = I, M, X;
      for (; R--; )
        if (e.events[R][0] === "exit" && e.events[R][1].type === "chunkFlow") {
          if (M) {
            X = e.events[R][1].end;
            break;
          }
          M = !0;
        }
      for (w(r), b = I; b < e.events.length; )
        e.events[b][1].end = Object.assign({}, X), b++;
      gn(
        e.events,
        R + 1,
        0,
        e.events.slice(I)
      ), e.events.length = b;
    }
  }
  function w(A) {
    let _ = t.length;
    for (; _-- > A; ) {
      const T = t[_];
      e.containerState = T[1], T[0].exit.call(e, n);
    }
    t.length = A;
  }
  function v() {
    i.write([null]), l = void 0, i = void 0, e.containerState._closeFlow = void 0;
  }
}
function Kr(n, e, t) {
  return U(
    n,
    n.attempt(this.parser.constructs.document, e, t),
    "linePrefix",
    this.parser.constructs.disable.null.includes("codeIndented") ? void 0 : 4
  );
}
function Je(n) {
  if (n === null || sn(n) || Vr(n))
    return 1;
  if (qr(n))
    return 2;
}
function Ce(n, e, t) {
  const r = [];
  let i = -1;
  for (; ++i < n.length; ) {
    const l = n[i].resolveAll;
    l && !r.includes(l) && (e = l(e, t), r.push(l));
  }
  return e;
}
const xe = {
  name: "attention",
  tokenize: Zr,
  resolveAll: Gr
};
function Gr(n, e) {
  let t = -1, r, i, l, o, u, a, c, s;
  for (; ++t < n.length; )
    if (n[t][0] === "enter" && n[t][1].type === "attentionSequence" && n[t][1]._close) {
      for (r = t; r--; )
        if (n[r][0] === "exit" && n[r][1].type === "attentionSequence" && n[r][1]._open && // If the markers are the same:
        e.sliceSerialize(n[r][1]).charCodeAt(0) === e.sliceSerialize(n[t][1]).charCodeAt(0)) {
          if ((n[r][1]._close || n[t][1]._open) && (n[t][1].end.offset - n[t][1].start.offset) % 3 && !((n[r][1].end.offset - n[r][1].start.offset + n[t][1].end.offset - n[t][1].start.offset) % 3))
            continue;
          a = n[r][1].end.offset - n[r][1].start.offset > 1 && n[t][1].end.offset - n[t][1].start.offset > 1 ? 2 : 1;
          const h = Object.assign({}, n[r][1].end), g = Object.assign({}, n[t][1].start);
          nt(h, -a), nt(g, a), o = {
            type: a > 1 ? "strongSequence" : "emphasisSequence",
            start: h,
            end: Object.assign({}, n[r][1].end)
          }, u = {
            type: a > 1 ? "strongSequence" : "emphasisSequence",
            start: Object.assign({}, n[t][1].start),
            end: g
          }, l = {
            type: a > 1 ? "strongText" : "emphasisText",
            start: Object.assign({}, n[r][1].end),
            end: Object.assign({}, n[t][1].start)
          }, i = {
            type: a > 1 ? "strong" : "emphasis",
            start: Object.assign({}, o.start),
            end: Object.assign({}, u.end)
          }, n[r][1].end = Object.assign({}, o.start), n[t][1].start = Object.assign({}, u.end), c = [], n[r][1].end.offset - n[r][1].start.offset && (c = an(c, [
            ["enter", n[r][1], e],
            ["exit", n[r][1], e]
          ])), c = an(c, [
            ["enter", i, e],
            ["enter", o, e],
            ["exit", o, e],
            ["enter", l, e]
          ]), c = an(
            c,
            Ce(
              e.parser.constructs.insideSpan.null,
              n.slice(r + 1, t),
              e
            )
          ), c = an(c, [
            ["exit", l, e],
            ["enter", u, e],
            ["exit", u, e],
            ["exit", i, e]
          ]), n[t][1].end.offset - n[t][1].start.offset ? (s = 2, c = an(c, [
            ["enter", n[t][1], e],
            ["exit", n[t][1], e]
          ])) : s = 0, gn(n, r - 1, t - r + 3, c), t = r + c.length - s - 2;
          break;
        }
    }
  for (t = -1; ++t < n.length; )
    n[t][1].type === "attentionSequence" && (n[t][1].type = "data");
  return n;
}
function Zr(n, e) {
  const t = this.parser.constructs.attentionMarkers.null, r = this.previous, i = Je(r);
  let l;
  return o;
  function o(a) {
    return n.enter("attentionSequence"), l = a, u(a);
  }
  function u(a) {
    if (a === l)
      return n.consume(a), u;
    const c = n.exit("attentionSequence"), s = Je(a), h = !s || s === 2 && i || t.includes(a), g = !i || i === 2 && s || t.includes(r);
    return c._open = !!(l === 42 ? h : h && (i || !g)), c._close = !!(l === 42 ? g : g && (s || !h)), e(a);
  }
}
function nt(n, e) {
  n.column += e, n.offset += e, n._bufferIndex += e;
}
const Jr = {
  name: "autolink",
  tokenize: ni
};
function ni(n, e, t) {
  let r = 1;
  return i;
  function i(m) {
    return n.enter("autolink"), n.enter("autolinkMarker"), n.consume(m), n.exit("autolinkMarker"), n.enter("autolinkProtocol"), l;
  }
  function l(m) {
    return dn(m) ? (n.consume(m), o) : Ge(m) ? c(m) : t(m);
  }
  function o(m) {
    return m === 43 || m === 45 || m === 46 || on(m) ? u(m) : c(m);
  }
  function u(m) {
    return m === 58 ? (n.consume(m), a) : (m === 43 || m === 45 || m === 46 || on(m)) && r++ < 32 ? (n.consume(m), u) : c(m);
  }
  function a(m) {
    return m === 62 ? (n.exit("autolinkProtocol"), d(m)) : m === null || m === 32 || m === 60 || ye(m) ? t(m) : (n.consume(m), a);
  }
  function c(m) {
    return m === 64 ? (n.consume(m), r = 0, s) : Ge(m) ? (n.consume(m), c) : t(m);
  }
  function s(m) {
    return on(m) ? h(m) : t(m);
  }
  function h(m) {
    return m === 46 ? (n.consume(m), r = 0, s) : m === 62 ? (n.exit("autolinkProtocol").type = "autolinkEmail", d(m)) : g(m);
  }
  function g(m) {
    return (m === 45 || on(m)) && r++ < 63 ? (n.consume(m), m === 45 ? g : h) : t(m);
  }
  function d(m) {
    return n.enter("autolinkMarker"), n.consume(m), n.exit("autolinkMarker"), n.exit("autolink"), e;
  }
}
const Zn = {
  tokenize: ei,
  partial: !0
};
function ei(n, e, t) {
  return U(n, r, "linePrefix");
  function r(i) {
    return i === null || L(i) ? e(i) : t(i);
  }
}
const Dt = {
  name: "blockQuote",
  tokenize: ti,
  continuation: {
    tokenize: ri
  },
  exit: ii
};
function ti(n, e, t) {
  const r = this;
  return i;
  function i(o) {
    if (o === 62) {
      const u = r.containerState;
      return u.open || (n.enter("blockQuote", {
        _container: !0
      }), u.open = !0), n.enter("blockQuotePrefix"), n.enter("blockQuoteMarker"), n.consume(o), n.exit("blockQuoteMarker"), l;
    }
    return t(o);
  }
  function l(o) {
    return G(o) ? (n.enter("blockQuotePrefixWhitespace"), n.consume(o), n.exit("blockQuotePrefixWhitespace"), n.exit("blockQuotePrefix"), e) : (n.exit("blockQuotePrefix"), e(o));
  }
}
function ri(n, e, t) {
  return U(
    n,
    n.attempt(Dt, e, t),
    "linePrefix",
    this.parser.constructs.disable.null.includes("codeIndented") ? void 0 : 4
  );
}
function ii(n) {
  n.exit("blockQuote");
}
const vt = {
  name: "characterEscape",
  tokenize: li
};
function li(n, e, t) {
  return r;
  function r(l) {
    return n.enter("characterEscape"), n.enter("escapeMarker"), n.consume(l), n.exit("escapeMarker"), i;
  }
  function i(l) {
    return Ur(l) ? (n.enter("characterEscapeValue"), n.consume(l), n.exit("characterEscapeValue"), n.exit("characterEscape"), e) : t(l);
  }
}
const et = document.createElement("i");
function Ae(n) {
  const e = "&" + n + ";";
  et.innerHTML = e;
  const t = et.textContent;
  return t.charCodeAt(t.length - 1) === 59 && n !== "semi" || t === e ? !1 : t;
}
const zt = {
  name: "characterReference",
  tokenize: oi
};
function oi(n, e, t) {
  const r = this;
  let i = 0, l, o;
  return u;
  function u(h) {
    return n.enter("characterReference"), n.enter("characterReferenceMarker"), n.consume(h), n.exit("characterReferenceMarker"), a;
  }
  function a(h) {
    return h === 35 ? (n.enter("characterReferenceMarkerNumeric"), n.consume(h), n.exit("characterReferenceMarkerNumeric"), c) : (n.enter("characterReferenceValue"), l = 31, o = on, s(h));
  }
  function c(h) {
    return h === 88 || h === 120 ? (n.enter("characterReferenceMarkerHexadecimal"), n.consume(h), n.exit("characterReferenceMarkerHexadecimal"), n.enter("characterReferenceValue"), l = 6, o = Hr, s) : (n.enter("characterReferenceValue"), l = 7, o = ge, s(h));
  }
  function s(h) {
    let g;
    return h === 59 && i ? (g = n.exit("characterReferenceValue"), o === on && !Ae(r.sliceSerialize(g)) ? t(h) : (n.enter("characterReferenceMarker"), n.consume(h), n.exit("characterReferenceMarker"), n.exit("characterReference"), e)) : o(h) && i++ < l ? (n.consume(h), s) : t(h);
  }
}
const tt = {
  name: "codeFenced",
  tokenize: ui,
  concrete: !0
};
function ui(n, e, t) {
  const r = this, i = {
    tokenize: T,
    partial: !0
  }, l = {
    tokenize: _,
    partial: !0
  }, o = this.events[this.events.length - 1], u = o && o[1].type === "linePrefix" ? o[2].sliceSerialize(o[1], !0).length : 0;
  let a = 0, c;
  return s;
  function s(b) {
    return n.enter("codeFenced"), n.enter("codeFencedFence"), n.enter("codeFencedFenceSequence"), c = b, h(b);
  }
  function h(b) {
    return b === c ? (n.consume(b), a++, h) : (n.exit("codeFencedFenceSequence"), a < 3 ? t(b) : U(n, g, "whitespace")(b));
  }
  function g(b) {
    return b === null || L(b) ? x(b) : (n.enter("codeFencedFenceInfo"), n.enter("chunkString", {
      contentType: "string"
    }), d(b));
  }
  function d(b) {
    return b === null || sn(b) ? (n.exit("chunkString"), n.exit("codeFencedFenceInfo"), U(n, m, "whitespace")(b)) : b === 96 && b === c ? t(b) : (n.consume(b), d);
  }
  function m(b) {
    return b === null || L(b) ? x(b) : (n.enter("codeFencedFenceMeta"), n.enter("chunkString", {
      contentType: "string"
    }), y(b));
  }
  function y(b) {
    return b === null || L(b) ? (n.exit("chunkString"), n.exit("codeFencedFenceMeta"), x(b)) : b === 96 && b === c ? t(b) : (n.consume(b), y);
  }
  function x(b) {
    return n.exit("codeFencedFence"), r.interrupt ? e(b) : w(b);
  }
  function w(b) {
    return b === null ? A(b) : L(b) ? n.attempt(
      l,
      n.attempt(
        i,
        A,
        u ? U(
          n,
          w,
          "linePrefix",
          u + 1
        ) : w
      ),
      A
    )(b) : (n.enter("codeFlowValue"), v(b));
  }
  function v(b) {
    return b === null || L(b) ? (n.exit("codeFlowValue"), w(b)) : (n.consume(b), v);
  }
  function A(b) {
    return n.exit("codeFenced"), e(b);
  }
  function _(b, I, R) {
    const M = this;
    return X;
    function X(F) {
      return b.enter("lineEnding"), b.consume(F), b.exit("lineEnding"), C;
    }
    function C(F) {
      return M.parser.lazy[M.now().line] ? R(F) : I(F);
    }
  }
  function T(b, I, R) {
    let M = 0;
    return U(
      b,
      X,
      "linePrefix",
      this.parser.constructs.disable.null.includes("codeIndented") ? void 0 : 4
    );
    function X(B) {
      return b.enter("codeFencedFence"), b.enter("codeFencedFenceSequence"), C(B);
    }
    function C(B) {
      return B === c ? (b.consume(B), M++, C) : M < a ? R(B) : (b.exit("codeFencedFenceSequence"), U(b, F, "whitespace")(B));
    }
    function F(B) {
      return B === null || L(B) ? (b.exit("codeFencedFence"), I(B)) : R(B);
    }
  }
}
const se = {
  name: "codeIndented",
  tokenize: si
}, ai = {
  tokenize: ci,
  partial: !0
};
function si(n, e, t) {
  const r = this;
  return i;
  function i(c) {
    return n.enter("codeIndented"), U(n, l, "linePrefix", 4 + 1)(c);
  }
  function l(c) {
    const s = r.events[r.events.length - 1];
    return s && s[1].type === "linePrefix" && s[2].sliceSerialize(s[1], !0).length >= 4 ? o(c) : t(c);
  }
  function o(c) {
    return c === null ? a(c) : L(c) ? n.attempt(ai, o, a)(c) : (n.enter("codeFlowValue"), u(c));
  }
  function u(c) {
    return c === null || L(c) ? (n.exit("codeFlowValue"), o(c)) : (n.consume(c), u);
  }
  function a(c) {
    return n.exit("codeIndented"), e(c);
  }
}
function ci(n, e, t) {
  const r = this;
  return i;
  function i(o) {
    return r.parser.lazy[r.now().line] ? t(o) : L(o) ? (n.enter("lineEnding"), n.consume(o), n.exit("lineEnding"), i) : U(n, l, "linePrefix", 4 + 1)(o);
  }
  function l(o) {
    const u = r.events[r.events.length - 1];
    return u && u[1].type === "linePrefix" && u[2].sliceSerialize(u[1], !0).length >= 4 ? e(o) : L(o) ? i(o) : t(o);
  }
}
const pi = {
  name: "codeText",
  tokenize: mi,
  resolve: fi,
  previous: hi
};
function fi(n) {
  let e = n.length - 4, t = 3, r, i;
  if ((n[t][1].type === "lineEnding" || n[t][1].type === "space") && (n[e][1].type === "lineEnding" || n[e][1].type === "space")) {
    for (r = t; ++r < e; )
      if (n[r][1].type === "codeTextData") {
        n[t][1].type = "codeTextPadding", n[e][1].type = "codeTextPadding", t += 2, e -= 2;
        break;
      }
  }
  for (r = t - 1, e++; ++r <= e; )
    i === void 0 ? r !== e && n[r][1].type !== "lineEnding" && (i = r) : (r === e || n[r][1].type === "lineEnding") && (n[i][1].type = "codeTextData", r !== i + 2 && (n[i][1].end = n[r - 1][1].end, n.splice(i + 2, r - i - 2), e -= r - i - 2, r = i + 2), i = void 0);
  return n;
}
function hi(n) {
  return n !== 96 || this.events[this.events.length - 1][1].type === "characterEscape";
}
function mi(n, e, t) {
  let r = 0, i, l;
  return o;
  function o(h) {
    return n.enter("codeText"), n.enter("codeTextSequence"), u(h);
  }
  function u(h) {
    return h === 96 ? (n.consume(h), r++, u) : (n.exit("codeTextSequence"), a(h));
  }
  function a(h) {
    return h === null ? t(h) : h === 96 ? (l = n.enter("codeTextSequence"), i = 0, s(h)) : h === 32 ? (n.enter("space"), n.consume(h), n.exit("space"), a) : L(h) ? (n.enter("lineEnding"), n.consume(h), n.exit("lineEnding"), a) : (n.enter("codeTextData"), c(h));
  }
  function c(h) {
    return h === null || h === 32 || h === 96 || L(h) ? (n.exit("codeTextData"), a(h)) : (n.consume(h), c);
  }
  function s(h) {
    return h === 96 ? (n.consume(h), i++, s) : i === r ? (n.exit("codeTextSequence"), n.exit("codeText"), e(h)) : (l.type = "codeTextData", c(h));
  }
}
function Rt(n) {
  const e = {};
  let t = -1, r, i, l, o, u, a, c;
  for (; ++t < n.length; ) {
    for (; t in e; )
      t = e[t];
    if (r = n[t], t && r[1].type === "chunkFlow" && n[t - 1][1].type === "listItemPrefix" && (a = r[1]._tokenizer.events, l = 0, l < a.length && a[l][1].type === "lineEndingBlank" && (l += 2), l < a.length && a[l][1].type === "content"))
      for (; ++l < a.length && a[l][1].type !== "content"; )
        a[l][1].type === "chunkText" && (a[l][1]._isInFirstContentOfListItem = !0, l++);
    if (r[0] === "enter")
      r[1].contentType && (Object.assign(e, di(n, t)), t = e[t], c = !0);
    else if (r[1]._container) {
      for (l = t, i = void 0; l-- && (o = n[l], o[1].type === "lineEnding" || o[1].type === "lineEndingBlank"); )
        o[0] === "enter" && (i && (n[i][1].type = "lineEndingBlank"), o[1].type = "lineEnding", i = l);
      i && (r[1].end = Object.assign({}, n[i][1].start), u = n.slice(i, t), u.unshift(r), gn(n, i, t - i + 1, u));
    }
  }
  return !c;
}
function di(n, e) {
  const t = n[e][1], r = n[e][2];
  let i = e - 1;
  const l = [], o = t._tokenizer || r.parser[t.contentType](t.start), u = o.events, a = [], c = {};
  let s, h, g = -1, d = t, m = 0, y = 0;
  const x = [y];
  for (; d; ) {
    for (; n[++i][1] !== d; )
      ;
    l.push(i), d._tokenizer || (s = r.sliceStream(d), d.next || s.push(null), h && o.defineSkip(d.start), d._isInFirstContentOfListItem && (o._gfmTasklistFirstContentOfListItem = !0), o.write(s), d._isInFirstContentOfListItem && (o._gfmTasklistFirstContentOfListItem = void 0)), h = d, d = d.next;
  }
  for (d = t; ++g < u.length; )
    // Find a void token that includes a break.
    u[g][0] === "exit" && u[g - 1][0] === "enter" && u[g][1].type === u[g - 1][1].type && u[g][1].start.line !== u[g][1].end.line && (y = g + 1, x.push(y), d._tokenizer = void 0, d.previous = void 0, d = d.next);
  for (o.events = [], d ? (d._tokenizer = void 0, d.previous = void 0) : x.pop(), g = x.length; g--; ) {
    const w = u.slice(x[g], x[g + 1]), v = l.pop();
    a.unshift([v, v + w.length - 1]), gn(n, v, 2, w);
  }
  for (g = -1; ++g < a.length; )
    c[m + a[g][0]] = m + a[g][1], m += a[g][1] - a[g][0] - 1;
  return c;
}
const gi = {
  tokenize: ki,
  resolve: xi
}, yi = {
  tokenize: bi,
  partial: !0
};
function xi(n) {
  return Rt(n), n;
}
function ki(n, e) {
  let t;
  return r;
  function r(u) {
    return n.enter("content"), t = n.enter("chunkContent", {
      contentType: "content"
    }), i(u);
  }
  function i(u) {
    return u === null ? l(u) : L(u) ? n.check(
      yi,
      o,
      l
    )(u) : (n.consume(u), i);
  }
  function l(u) {
    return n.exit("chunkContent"), n.exit("content"), e(u);
  }
  function o(u) {
    return n.consume(u), n.exit("chunkContent"), t.next = n.enter("chunkContent", {
      contentType: "content",
      previous: t
    }), t = t.next, i;
  }
}
function bi(n, e, t) {
  const r = this;
  return i;
  function i(o) {
    return n.exit("chunkContent"), n.enter("lineEnding"), n.consume(o), n.exit("lineEnding"), U(n, l, "linePrefix");
  }
  function l(o) {
    if (o === null || L(o))
      return t(o);
    const u = r.events[r.events.length - 1];
    return !r.parser.constructs.disable.null.includes("codeIndented") && u && u[1].type === "linePrefix" && u[2].sliceSerialize(u[1], !0).length >= 4 ? e(o) : n.interrupt(r.parser.constructs.flow, t, e)(o);
  }
}
function _t(n, e, t, r, i, l, o, u, a) {
  const c = a || Number.POSITIVE_INFINITY;
  let s = 0;
  return h;
  function h(w) {
    return w === 60 ? (n.enter(r), n.enter(i), n.enter(l), n.consume(w), n.exit(l), g) : w === null || w === 41 || ye(w) ? t(w) : (n.enter(r), n.enter(o), n.enter(u), n.enter("chunkString", {
      contentType: "string"
    }), y(w));
  }
  function g(w) {
    return w === 62 ? (n.enter(l), n.consume(w), n.exit(l), n.exit(i), n.exit(r), e) : (n.enter(u), n.enter("chunkString", {
      contentType: "string"
    }), d(w));
  }
  function d(w) {
    return w === 62 ? (n.exit("chunkString"), n.exit(u), g(w)) : w === null || w === 60 || L(w) ? t(w) : (n.consume(w), w === 92 ? m : d);
  }
  function m(w) {
    return w === 60 || w === 62 || w === 92 ? (n.consume(w), d) : d(w);
  }
  function y(w) {
    return w === 40 ? ++s > c ? t(w) : (n.consume(w), y) : w === 41 ? s-- ? (n.consume(w), y) : (n.exit("chunkString"), n.exit(u), n.exit(o), n.exit(r), e(w)) : w === null || sn(w) ? s ? t(w) : (n.exit("chunkString"), n.exit(u), n.exit(o), n.exit(r), e(w)) : ye(w) ? t(w) : (n.consume(w), w === 92 ? x : y);
  }
  function x(w) {
    return w === 40 || w === 41 || w === 92 ? (n.consume(w), y) : y(w);
  }
}
function Mt(n, e, t, r, i, l) {
  const o = this;
  let u = 0, a;
  return c;
  function c(d) {
    return n.enter(r), n.enter(i), n.consume(d), n.exit(i), n.enter(l), s;
  }
  function s(d) {
    return d === null || d === 91 || d === 93 && !a || /* To do: remove in the future once we’ve switched from
     * `micromark-extension-footnote` to `micromark-extension-gfm-footnote`,
     * which doesn’t need this */
    /* Hidden footnotes hook */
    /* c8 ignore next 3 */
    d === 94 && !u && "_hiddenFootnoteSupport" in o.parser.constructs || u > 999 ? t(d) : d === 93 ? (n.exit(l), n.enter(i), n.consume(d), n.exit(i), n.exit(r), e) : L(d) ? (n.enter("lineEnding"), n.consume(d), n.exit("lineEnding"), s) : (n.enter("chunkString", {
      contentType: "string"
    }), h(d));
  }
  function h(d) {
    return d === null || d === 91 || d === 93 || L(d) || u++ > 999 ? (n.exit("chunkString"), s(d)) : (n.consume(d), a = a || !G(d), d === 92 ? g : h);
  }
  function g(d) {
    return d === 91 || d === 92 || d === 93 ? (n.consume(d), u++, h) : h(d);
  }
}
function Bt(n, e, t, r, i, l) {
  let o;
  return u;
  function u(g) {
    return n.enter(r), n.enter(i), n.consume(g), n.exit(i), o = g === 40 ? 41 : g, a;
  }
  function a(g) {
    return g === o ? (n.enter(i), n.consume(g), n.exit(i), n.exit(r), e) : (n.enter(l), c(g));
  }
  function c(g) {
    return g === o ? (n.exit(l), a(o)) : g === null ? t(g) : L(g) ? (n.enter("lineEnding"), n.consume(g), n.exit("lineEnding"), U(n, c, "linePrefix")) : (n.enter("chunkString", {
      contentType: "string"
    }), s(g));
  }
  function s(g) {
    return g === o || g === null || L(g) ? (n.exit("chunkString"), c(g)) : (n.consume(g), g === 92 ? h : s);
  }
  function h(g) {
    return g === o || g === 92 ? (n.consume(g), s) : s(g);
  }
}
function $n(n, e) {
  let t;
  return r;
  function r(i) {
    return L(i) ? (n.enter("lineEnding"), n.consume(i), n.exit("lineEnding"), t = !0, r) : G(i) ? U(
      n,
      r,
      t ? "linePrefix" : "lineSuffix"
    )(i) : e(i);
  }
}
function Ln(n) {
  return n.replace(/[\t\n\r ]+/g, " ").replace(/^ | $/g, "").toLowerCase().toUpperCase();
}
const wi = {
  name: "definition",
  tokenize: Ei
}, Si = {
  tokenize: Ci,
  partial: !0
};
function Ei(n, e, t) {
  const r = this;
  let i;
  return l;
  function l(a) {
    return n.enter("definition"), Mt.call(
      r,
      n,
      o,
      t,
      "definitionLabel",
      "definitionLabelMarker",
      "definitionLabelString"
    )(a);
  }
  function o(a) {
    return i = Ln(
      r.sliceSerialize(r.events[r.events.length - 1][1]).slice(1, -1)
    ), a === 58 ? (n.enter("definitionMarker"), n.consume(a), n.exit("definitionMarker"), $n(
      n,
      _t(
        n,
        n.attempt(
          Si,
          U(n, u, "whitespace"),
          U(n, u, "whitespace")
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
    return a === null || L(a) ? (n.exit("definition"), r.parser.defined.includes(i) || r.parser.defined.push(i), e(a)) : t(a);
  }
}
function Ci(n, e, t) {
  return r;
  function r(o) {
    return sn(o) ? $n(n, i)(o) : t(o);
  }
  function i(o) {
    return o === 34 || o === 39 || o === 40 ? Bt(
      n,
      U(n, l, "whitespace"),
      t,
      "definitionTitle",
      "definitionTitleMarker",
      "definitionTitleString"
    )(o) : t(o);
  }
  function l(o) {
    return o === null || L(o) ? e(o) : t(o);
  }
}
const Ai = {
  name: "hardBreakEscape",
  tokenize: Pi
};
function Pi(n, e, t) {
  return r;
  function r(l) {
    return n.enter("hardBreakEscape"), n.enter("escapeMarker"), n.consume(l), i;
  }
  function i(l) {
    return L(l) ? (n.exit("escapeMarker"), n.exit("hardBreakEscape"), e(l)) : t(l);
  }
}
const Fi = {
  name: "headingAtx",
  tokenize: Ii,
  resolve: Ti
};
function Ti(n, e) {
  let t = n.length - 2, r = 3, i, l;
  return n[r][1].type === "whitespace" && (r += 2), t - 2 > r && n[t][1].type === "whitespace" && (t -= 2), n[t][1].type === "atxHeadingSequence" && (r === t - 1 || t - 4 > r && n[t - 2][1].type === "whitespace") && (t -= r + 1 === t ? 2 : 4), t > r && (i = {
    type: "atxHeadingText",
    start: n[r][1].start,
    end: n[t][1].end
  }, l = {
    type: "chunkText",
    start: n[r][1].start,
    end: n[t][1].end,
    // @ts-expect-error Constants are fine to assign.
    contentType: "text"
  }, gn(n, r, t - r + 1, [
    ["enter", i, e],
    ["enter", l, e],
    ["exit", l, e],
    ["exit", i, e]
  ])), n;
}
function Ii(n, e, t) {
  const r = this;
  let i = 0;
  return l;
  function l(s) {
    return n.enter("atxHeading"), n.enter("atxHeadingSequence"), o(s);
  }
  function o(s) {
    return s === 35 && i++ < 6 ? (n.consume(s), o) : s === null || sn(s) ? (n.exit("atxHeadingSequence"), r.interrupt ? e(s) : u(s)) : t(s);
  }
  function u(s) {
    return s === 35 ? (n.enter("atxHeadingSequence"), a(s)) : s === null || L(s) ? (n.exit("atxHeading"), e(s)) : G(s) ? U(n, u, "whitespace")(s) : (n.enter("atxHeadingText"), c(s));
  }
  function a(s) {
    return s === 35 ? (n.consume(s), a) : (n.exit("atxHeadingSequence"), u(s));
  }
  function c(s) {
    return s === null || s === 35 || sn(s) ? (n.exit("atxHeadingText"), u(s)) : (n.consume(s), c);
  }
}
const Oi = [
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
], rt = ["pre", "script", "style", "textarea"], Li = {
  name: "htmlFlow",
  tokenize: zi,
  resolveTo: vi,
  concrete: !0
}, Di = {
  tokenize: Ri,
  partial: !0
};
function vi(n) {
  let e = n.length;
  for (; e-- && !(n[e][0] === "enter" && n[e][1].type === "htmlFlow"); )
    ;
  return e > 1 && n[e - 2][1].type === "linePrefix" && (n[e][1].start = n[e - 2][1].start, n[e + 1][1].start = n[e - 2][1].start, n.splice(e - 2, 2)), n;
}
function zi(n, e, t) {
  const r = this;
  let i, l, o, u, a;
  return c;
  function c(p) {
    return n.enter("htmlFlow"), n.enter("htmlFlowData"), n.consume(p), s;
  }
  function s(p) {
    return p === 33 ? (n.consume(p), h) : p === 47 ? (n.consume(p), m) : p === 63 ? (n.consume(p), i = 3, r.interrupt ? e : J) : dn(p) ? (n.consume(p), o = String.fromCharCode(p), l = !0, y) : t(p);
  }
  function h(p) {
    return p === 45 ? (n.consume(p), i = 2, g) : p === 91 ? (n.consume(p), i = 5, o = "CDATA[", u = 0, d) : dn(p) ? (n.consume(p), i = 4, r.interrupt ? e : J) : t(p);
  }
  function g(p) {
    return p === 45 ? (n.consume(p), r.interrupt ? e : J) : t(p);
  }
  function d(p) {
    return p === o.charCodeAt(u++) ? (n.consume(p), u === o.length ? r.interrupt ? e : C : d) : t(p);
  }
  function m(p) {
    return dn(p) ? (n.consume(p), o = String.fromCharCode(p), y) : t(p);
  }
  function y(p) {
    return p === null || p === 47 || p === 62 || sn(p) ? p !== 47 && l && rt.includes(o.toLowerCase()) ? (i = 1, r.interrupt ? e(p) : C(p)) : Oi.includes(o.toLowerCase()) ? (i = 6, p === 47 ? (n.consume(p), x) : r.interrupt ? e(p) : C(p)) : (i = 7, r.interrupt && !r.parser.lazy[r.now().line] ? t(p) : l ? v(p) : w(p)) : p === 45 || on(p) ? (n.consume(p), o += String.fromCharCode(p), y) : t(p);
  }
  function x(p) {
    return p === 62 ? (n.consume(p), r.interrupt ? e : C) : t(p);
  }
  function w(p) {
    return G(p) ? (n.consume(p), w) : M(p);
  }
  function v(p) {
    return p === 47 ? (n.consume(p), M) : p === 58 || p === 95 || dn(p) ? (n.consume(p), A) : G(p) ? (n.consume(p), v) : M(p);
  }
  function A(p) {
    return p === 45 || p === 46 || p === 58 || p === 95 || on(p) ? (n.consume(p), A) : _(p);
  }
  function _(p) {
    return p === 61 ? (n.consume(p), T) : G(p) ? (n.consume(p), _) : v(p);
  }
  function T(p) {
    return p === null || p === 60 || p === 61 || p === 62 || p === 96 ? t(p) : p === 34 || p === 39 ? (n.consume(p), a = p, b) : G(p) ? (n.consume(p), T) : (a = null, I(p));
  }
  function b(p) {
    return p === null || L(p) ? t(p) : p === a ? (n.consume(p), R) : (n.consume(p), b);
  }
  function I(p) {
    return p === null || p === 34 || p === 39 || p === 60 || p === 61 || p === 62 || p === 96 || sn(p) ? _(p) : (n.consume(p), I);
  }
  function R(p) {
    return p === 47 || p === 62 || G(p) ? v(p) : t(p);
  }
  function M(p) {
    return p === 62 ? (n.consume(p), X) : t(p);
  }
  function X(p) {
    return G(p) ? (n.consume(p), X) : p === null || L(p) ? C(p) : t(p);
  }
  function C(p) {
    return p === 45 && i === 2 ? (n.consume(p), en) : p === 60 && i === 1 ? (n.consume(p), un) : p === 62 && i === 4 ? (n.consume(p), W) : p === 63 && i === 3 ? (n.consume(p), J) : p === 93 && i === 5 ? (n.consume(p), q) : L(p) && (i === 6 || i === 7) ? n.check(
      Di,
      W,
      F
    )(p) : p === null || L(p) ? F(p) : (n.consume(p), C);
  }
  function F(p) {
    return n.exit("htmlFlowData"), B(p);
  }
  function B(p) {
    return p === null ? f(p) : L(p) ? n.attempt(
      {
        tokenize: Z,
        partial: !0
      },
      B,
      f
    )(p) : (n.enter("htmlFlowData"), C(p));
  }
  function Z(p, kn, Pn) {
    return bn;
    function bn(rn) {
      return p.enter("lineEnding"), p.consume(rn), p.exit("lineEnding"), Y;
    }
    function Y(rn) {
      return r.parser.lazy[r.now().line] ? Pn(rn) : kn(rn);
    }
  }
  function en(p) {
    return p === 45 ? (n.consume(p), J) : C(p);
  }
  function un(p) {
    return p === 47 ? (n.consume(p), o = "", tn) : C(p);
  }
  function tn(p) {
    return p === 62 && rt.includes(o.toLowerCase()) ? (n.consume(p), W) : dn(p) && o.length < 8 ? (n.consume(p), o += String.fromCharCode(p), tn) : C(p);
  }
  function q(p) {
    return p === 93 ? (n.consume(p), J) : C(p);
  }
  function J(p) {
    return p === 62 ? (n.consume(p), W) : p === 45 && i === 2 ? (n.consume(p), J) : C(p);
  }
  function W(p) {
    return p === null || L(p) ? (n.exit("htmlFlowData"), f(p)) : (n.consume(p), W);
  }
  function f(p) {
    return n.exit("htmlFlow"), e(p);
  }
}
function Ri(n, e, t) {
  return r;
  function r(i) {
    return n.exit("htmlFlowData"), n.enter("lineEndingBlank"), n.consume(i), n.exit("lineEndingBlank"), n.attempt(Zn, e, t);
  }
}
const _i = {
  name: "htmlText",
  tokenize: Mi
};
function Mi(n, e, t) {
  const r = this;
  let i, l, o, u;
  return a;
  function a(f) {
    return n.enter("htmlText"), n.enter("htmlTextData"), n.consume(f), c;
  }
  function c(f) {
    return f === 33 ? (n.consume(f), s) : f === 47 ? (n.consume(f), I) : f === 63 ? (n.consume(f), T) : dn(f) ? (n.consume(f), X) : t(f);
  }
  function s(f) {
    return f === 45 ? (n.consume(f), h) : f === 91 ? (n.consume(f), l = "CDATA[", o = 0, x) : dn(f) ? (n.consume(f), _) : t(f);
  }
  function h(f) {
    return f === 45 ? (n.consume(f), g) : t(f);
  }
  function g(f) {
    return f === null || f === 62 ? t(f) : f === 45 ? (n.consume(f), d) : m(f);
  }
  function d(f) {
    return f === null || f === 62 ? t(f) : m(f);
  }
  function m(f) {
    return f === null ? t(f) : f === 45 ? (n.consume(f), y) : L(f) ? (u = m, q(f)) : (n.consume(f), m);
  }
  function y(f) {
    return f === 45 ? (n.consume(f), W) : m(f);
  }
  function x(f) {
    return f === l.charCodeAt(o++) ? (n.consume(f), o === l.length ? w : x) : t(f);
  }
  function w(f) {
    return f === null ? t(f) : f === 93 ? (n.consume(f), v) : L(f) ? (u = w, q(f)) : (n.consume(f), w);
  }
  function v(f) {
    return f === 93 ? (n.consume(f), A) : w(f);
  }
  function A(f) {
    return f === 62 ? W(f) : f === 93 ? (n.consume(f), A) : w(f);
  }
  function _(f) {
    return f === null || f === 62 ? W(f) : L(f) ? (u = _, q(f)) : (n.consume(f), _);
  }
  function T(f) {
    return f === null ? t(f) : f === 63 ? (n.consume(f), b) : L(f) ? (u = T, q(f)) : (n.consume(f), T);
  }
  function b(f) {
    return f === 62 ? W(f) : T(f);
  }
  function I(f) {
    return dn(f) ? (n.consume(f), R) : t(f);
  }
  function R(f) {
    return f === 45 || on(f) ? (n.consume(f), R) : M(f);
  }
  function M(f) {
    return L(f) ? (u = M, q(f)) : G(f) ? (n.consume(f), M) : W(f);
  }
  function X(f) {
    return f === 45 || on(f) ? (n.consume(f), X) : f === 47 || f === 62 || sn(f) ? C(f) : t(f);
  }
  function C(f) {
    return f === 47 ? (n.consume(f), W) : f === 58 || f === 95 || dn(f) ? (n.consume(f), F) : L(f) ? (u = C, q(f)) : G(f) ? (n.consume(f), C) : W(f);
  }
  function F(f) {
    return f === 45 || f === 46 || f === 58 || f === 95 || on(f) ? (n.consume(f), F) : B(f);
  }
  function B(f) {
    return f === 61 ? (n.consume(f), Z) : L(f) ? (u = B, q(f)) : G(f) ? (n.consume(f), B) : C(f);
  }
  function Z(f) {
    return f === null || f === 60 || f === 61 || f === 62 || f === 96 ? t(f) : f === 34 || f === 39 ? (n.consume(f), i = f, en) : L(f) ? (u = Z, q(f)) : G(f) ? (n.consume(f), Z) : (n.consume(f), i = void 0, tn);
  }
  function en(f) {
    return f === i ? (n.consume(f), un) : f === null ? t(f) : L(f) ? (u = en, q(f)) : (n.consume(f), en);
  }
  function un(f) {
    return f === 62 || f === 47 || sn(f) ? C(f) : t(f);
  }
  function tn(f) {
    return f === null || f === 34 || f === 39 || f === 60 || f === 61 || f === 96 ? t(f) : f === 62 || sn(f) ? C(f) : (n.consume(f), tn);
  }
  function q(f) {
    return n.exit("htmlTextData"), n.enter("lineEnding"), n.consume(f), n.exit("lineEnding"), U(
      n,
      J,
      "linePrefix",
      r.parser.constructs.disable.null.includes("codeIndented") ? void 0 : 4
    );
  }
  function J(f) {
    return n.enter("htmlTextData"), u(f);
  }
  function W(f) {
    return f === 62 ? (n.consume(f), n.exit("htmlTextData"), n.exit("htmlText"), e) : t(f);
  }
}
const Pe = {
  name: "labelEnd",
  tokenize: Ui,
  resolveTo: Hi,
  resolveAll: $i
}, Bi = {
  tokenize: Vi
}, Ni = {
  tokenize: qi
}, ji = {
  tokenize: Wi
};
function $i(n) {
  let e = -1, t;
  for (; ++e < n.length; )
    t = n[e][1], (t.type === "labelImage" || t.type === "labelLink" || t.type === "labelEnd") && (n.splice(e + 1, t.type === "labelImage" ? 4 : 2), t.type = "data", e++);
  return n;
}
function Hi(n, e) {
  let t = n.length, r = 0, i, l, o, u;
  for (; t--; )
    if (i = n[t][1], l) {
      if (i.type === "link" || i.type === "labelLink" && i._inactive)
        break;
      n[t][0] === "enter" && i.type === "labelLink" && (i._inactive = !0);
    } else if (o) {
      if (n[t][0] === "enter" && (i.type === "labelImage" || i.type === "labelLink") && !i._balanced && (l = t, i.type !== "labelLink")) {
        r = 2;
        break;
      }
    } else
      i.type === "labelEnd" && (o = t);
  const a = {
    type: n[l][1].type === "labelLink" ? "link" : "image",
    start: Object.assign({}, n[l][1].start),
    end: Object.assign({}, n[n.length - 1][1].end)
  }, c = {
    type: "label",
    start: Object.assign({}, n[l][1].start),
    end: Object.assign({}, n[o][1].end)
  }, s = {
    type: "labelText",
    start: Object.assign({}, n[l + r + 2][1].end),
    end: Object.assign({}, n[o - 2][1].start)
  };
  return u = [
    ["enter", a, e],
    ["enter", c, e]
  ], u = an(u, n.slice(l + 1, l + r + 3)), u = an(u, [["enter", s, e]]), u = an(
    u,
    Ce(
      e.parser.constructs.insideSpan.null,
      n.slice(l + r + 4, o - 3),
      e
    )
  ), u = an(u, [
    ["exit", s, e],
    n[o - 2],
    n[o - 1],
    ["exit", c, e]
  ]), u = an(u, n.slice(o + 1)), u = an(u, [["exit", a, e]]), gn(n, l, n.length, u), n;
}
function Ui(n, e, t) {
  const r = this;
  let i = r.events.length, l, o;
  for (; i--; )
    if ((r.events[i][1].type === "labelImage" || r.events[i][1].type === "labelLink") && !r.events[i][1]._balanced) {
      l = r.events[i][1];
      break;
    }
  return u;
  function u(s) {
    return l ? l._inactive ? c(s) : (o = r.parser.defined.includes(
      Ln(
        r.sliceSerialize({
          start: l.end,
          end: r.now()
        })
      )
    ), n.enter("labelEnd"), n.enter("labelMarker"), n.consume(s), n.exit("labelMarker"), n.exit("labelEnd"), a) : t(s);
  }
  function a(s) {
    return s === 40 ? n.attempt(
      Bi,
      e,
      o ? e : c
    )(s) : s === 91 ? n.attempt(
      Ni,
      e,
      o ? n.attempt(ji, e, c) : c
    )(s) : o ? e(s) : c(s);
  }
  function c(s) {
    return l._balanced = !0, t(s);
  }
}
function Vi(n, e, t) {
  return r;
  function r(a) {
    return n.enter("resource"), n.enter("resourceMarker"), n.consume(a), n.exit("resourceMarker"), $n(n, i);
  }
  function i(a) {
    return a === 41 ? u(a) : _t(
      n,
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
    return sn(a) ? $n(n, o)(a) : u(a);
  }
  function o(a) {
    return a === 34 || a === 39 || a === 40 ? Bt(
      n,
      $n(n, u),
      t,
      "resourceTitle",
      "resourceTitleMarker",
      "resourceTitleString"
    )(a) : u(a);
  }
  function u(a) {
    return a === 41 ? (n.enter("resourceMarker"), n.consume(a), n.exit("resourceMarker"), n.exit("resource"), e) : t(a);
  }
}
function qi(n, e, t) {
  const r = this;
  return i;
  function i(o) {
    return Mt.call(
      r,
      n,
      l,
      t,
      "reference",
      "referenceMarker",
      "referenceString"
    )(o);
  }
  function l(o) {
    return r.parser.defined.includes(
      Ln(
        r.sliceSerialize(r.events[r.events.length - 1][1]).slice(1, -1)
      )
    ) ? e(o) : t(o);
  }
}
function Wi(n, e, t) {
  return r;
  function r(l) {
    return n.enter("reference"), n.enter("referenceMarker"), n.consume(l), n.exit("referenceMarker"), i;
  }
  function i(l) {
    return l === 93 ? (n.enter("referenceMarker"), n.consume(l), n.exit("referenceMarker"), n.exit("reference"), e) : t(l);
  }
}
const Yi = {
  name: "labelStartImage",
  tokenize: Qi,
  resolveAll: Pe.resolveAll
};
function Qi(n, e, t) {
  const r = this;
  return i;
  function i(u) {
    return n.enter("labelImage"), n.enter("labelImageMarker"), n.consume(u), n.exit("labelImageMarker"), l;
  }
  function l(u) {
    return u === 91 ? (n.enter("labelMarker"), n.consume(u), n.exit("labelMarker"), n.exit("labelImage"), o) : t(u);
  }
  function o(u) {
    return u === 94 && "_hiddenFootnoteSupport" in r.parser.constructs ? t(u) : e(u);
  }
}
const Xi = {
  name: "labelStartLink",
  tokenize: Ki,
  resolveAll: Pe.resolveAll
};
function Ki(n, e, t) {
  const r = this;
  return i;
  function i(o) {
    return n.enter("labelLink"), n.enter("labelMarker"), n.consume(o), n.exit("labelMarker"), n.exit("labelLink"), l;
  }
  function l(o) {
    return o === 94 && "_hiddenFootnoteSupport" in r.parser.constructs ? t(o) : e(o);
  }
}
const ce = {
  name: "lineEnding",
  tokenize: Gi
};
function Gi(n, e) {
  return t;
  function t(r) {
    return n.enter("lineEnding"), n.consume(r), n.exit("lineEnding"), U(n, e, "linePrefix");
  }
}
const Qn = {
  name: "thematicBreak",
  tokenize: Zi
};
function Zi(n, e, t) {
  let r = 0, i;
  return l;
  function l(a) {
    return n.enter("thematicBreak"), i = a, o(a);
  }
  function o(a) {
    return a === i ? (n.enter("thematicBreakSequence"), u(a)) : G(a) ? U(n, o, "whitespace")(a) : r < 3 || a !== null && !L(a) ? t(a) : (n.exit("thematicBreak"), e(a));
  }
  function u(a) {
    return a === i ? (n.consume(a), r++, u) : (n.exit("thematicBreakSequence"), o(a));
  }
}
const nn = {
  name: "list",
  tokenize: el,
  continuation: {
    tokenize: tl
  },
  exit: il
}, Ji = {
  tokenize: ll,
  partial: !0
}, nl = {
  tokenize: rl,
  partial: !0
};
function el(n, e, t) {
  const r = this, i = r.events[r.events.length - 1];
  let l = i && i[1].type === "linePrefix" ? i[2].sliceSerialize(i[1], !0).length : 0, o = 0;
  return u;
  function u(d) {
    const m = r.containerState.type || (d === 42 || d === 43 || d === 45 ? "listUnordered" : "listOrdered");
    if (m === "listUnordered" ? !r.containerState.marker || d === r.containerState.marker : ge(d)) {
      if (r.containerState.type || (r.containerState.type = m, n.enter(m, {
        _container: !0
      })), m === "listUnordered")
        return n.enter("listItemPrefix"), d === 42 || d === 45 ? n.check(Qn, t, c)(d) : c(d);
      if (!r.interrupt || d === 49)
        return n.enter("listItemPrefix"), n.enter("listItemValue"), a(d);
    }
    return t(d);
  }
  function a(d) {
    return ge(d) && ++o < 10 ? (n.consume(d), a) : (!r.interrupt || o < 2) && (r.containerState.marker ? d === r.containerState.marker : d === 41 || d === 46) ? (n.exit("listItemValue"), c(d)) : t(d);
  }
  function c(d) {
    return n.enter("listItemMarker"), n.consume(d), n.exit("listItemMarker"), r.containerState.marker = r.containerState.marker || d, n.check(
      Zn,
      // Can’t be empty when interrupting.
      r.interrupt ? t : s,
      n.attempt(
        Ji,
        g,
        h
      )
    );
  }
  function s(d) {
    return r.containerState.initialBlankLine = !0, l++, g(d);
  }
  function h(d) {
    return G(d) ? (n.enter("listItemPrefixWhitespace"), n.consume(d), n.exit("listItemPrefixWhitespace"), g) : t(d);
  }
  function g(d) {
    return r.containerState.size = l + r.sliceSerialize(n.exit("listItemPrefix"), !0).length, e(d);
  }
}
function tl(n, e, t) {
  const r = this;
  return r.containerState._closeFlow = void 0, n.check(Zn, i, l);
  function i(u) {
    return r.containerState.furtherBlankLines = r.containerState.furtherBlankLines || r.containerState.initialBlankLine, U(
      n,
      e,
      "listItemIndent",
      r.containerState.size + 1
    )(u);
  }
  function l(u) {
    return r.containerState.furtherBlankLines || !G(u) ? (r.containerState.furtherBlankLines = void 0, r.containerState.initialBlankLine = void 0, o(u)) : (r.containerState.furtherBlankLines = void 0, r.containerState.initialBlankLine = void 0, n.attempt(nl, e, o)(u));
  }
  function o(u) {
    return r.containerState._closeFlow = !0, r.interrupt = void 0, U(
      n,
      n.attempt(nn, e, t),
      "linePrefix",
      r.parser.constructs.disable.null.includes("codeIndented") ? void 0 : 4
    )(u);
  }
}
function rl(n, e, t) {
  const r = this;
  return U(
    n,
    i,
    "listItemIndent",
    r.containerState.size + 1
  );
  function i(l) {
    const o = r.events[r.events.length - 1];
    return o && o[1].type === "listItemIndent" && o[2].sliceSerialize(o[1], !0).length === r.containerState.size ? e(l) : t(l);
  }
}
function il(n) {
  n.exit(this.containerState.type);
}
function ll(n, e, t) {
  const r = this;
  return U(
    n,
    i,
    "listItemPrefixWhitespace",
    r.parser.constructs.disable.null.includes("codeIndented") ? void 0 : 4 + 1
  );
  function i(l) {
    const o = r.events[r.events.length - 1];
    return !G(l) && o && o[1].type === "listItemPrefixWhitespace" ? e(l) : t(l);
  }
}
const it = {
  name: "setextUnderline",
  tokenize: ul,
  resolveTo: ol
};
function ol(n, e) {
  let t = n.length, r, i, l;
  for (; t--; )
    if (n[t][0] === "enter") {
      if (n[t][1].type === "content") {
        r = t;
        break;
      }
      n[t][1].type === "paragraph" && (i = t);
    } else
      n[t][1].type === "content" && n.splice(t, 1), !l && n[t][1].type === "definition" && (l = t);
  const o = {
    type: "setextHeading",
    start: Object.assign({}, n[i][1].start),
    end: Object.assign({}, n[n.length - 1][1].end)
  };
  return n[i][1].type = "setextHeadingText", l ? (n.splice(i, 0, ["enter", o, e]), n.splice(l + 1, 0, ["exit", n[r][1], e]), n[r][1].end = Object.assign({}, n[l][1].end)) : n[r][1] = o, n.push(["exit", o, e]), n;
}
function ul(n, e, t) {
  const r = this;
  let i = r.events.length, l, o;
  for (; i--; )
    if (r.events[i][1].type !== "lineEnding" && r.events[i][1].type !== "linePrefix" && r.events[i][1].type !== "content") {
      o = r.events[i][1].type === "paragraph";
      break;
    }
  return u;
  function u(s) {
    return !r.parser.lazy[r.now().line] && (r.interrupt || o) ? (n.enter("setextHeadingLine"), n.enter("setextHeadingLineSequence"), l = s, a(s)) : t(s);
  }
  function a(s) {
    return s === l ? (n.consume(s), a) : (n.exit("setextHeadingLineSequence"), U(n, c, "lineSuffix")(s));
  }
  function c(s) {
    return s === null || L(s) ? (n.exit("setextHeadingLine"), e(s)) : t(s);
  }
}
const al = {
  tokenize: sl
};
function sl(n) {
  const e = this, t = n.attempt(
    // Try to parse a blank line.
    Zn,
    r,
    // Try to parse initial flow (essentially, only code).
    n.attempt(
      this.parser.constructs.flowInitial,
      i,
      U(
        n,
        n.attempt(
          this.parser.constructs.flow,
          i,
          n.attempt(gi, i)
        ),
        "linePrefix"
      )
    )
  );
  return t;
  function r(l) {
    if (l === null) {
      n.consume(l);
      return;
    }
    return n.enter("lineEndingBlank"), n.consume(l), n.exit("lineEndingBlank"), e.currentConstruct = void 0, t;
  }
  function i(l) {
    if (l === null) {
      n.consume(l);
      return;
    }
    return n.enter("lineEnding"), n.consume(l), n.exit("lineEnding"), e.currentConstruct = void 0, t;
  }
}
const cl = {
  resolveAll: jt()
}, pl = Nt("string"), fl = Nt("text");
function Nt(n) {
  return {
    tokenize: e,
    resolveAll: jt(
      n === "text" ? hl : void 0
    )
  };
  function e(t) {
    const r = this, i = this.parser.constructs[n], l = t.attempt(i, o, u);
    return o;
    function o(s) {
      return c(s) ? l(s) : u(s);
    }
    function u(s) {
      if (s === null) {
        t.consume(s);
        return;
      }
      return t.enter("data"), t.consume(s), a;
    }
    function a(s) {
      return c(s) ? (t.exit("data"), l(s)) : (t.consume(s), a);
    }
    function c(s) {
      if (s === null)
        return !0;
      const h = i[s];
      let g = -1;
      if (h)
        for (; ++g < h.length; ) {
          const d = h[g];
          if (!d.previous || d.previous.call(r, r.previous))
            return !0;
        }
      return !1;
    }
  }
}
function jt(n) {
  return e;
  function e(t, r) {
    let i = -1, l;
    for (; ++i <= t.length; )
      l === void 0 ? t[i] && t[i][1].type === "data" && (l = i, i++) : (!t[i] || t[i][1].type !== "data") && (i !== l + 2 && (t[l][1].end = t[i - 1][1].end, t.splice(l + 2, i - l - 2), i = l + 2), l = void 0);
    return n ? n(t, r) : t;
  }
}
function hl(n, e) {
  let t = 0;
  for (; ++t <= n.length; )
    if ((t === n.length || n[t][1].type === "lineEnding") && n[t - 1][1].type === "data") {
      const r = n[t - 1][1], i = e.sliceStream(r);
      let l = i.length, o = -1, u = 0, a;
      for (; l--; ) {
        const c = i[l];
        if (typeof c == "string") {
          for (o = c.length; c.charCodeAt(o - 1) === 32; )
            u++, o--;
          if (o)
            break;
          o = -1;
        } else if (c === -2)
          a = !0, u++;
        else if (c !== -1) {
          l++;
          break;
        }
      }
      if (u) {
        const c = {
          type: t === n.length || a || u < 2 ? "lineSuffix" : "hardBreakTrailing",
          start: {
            line: r.end.line,
            column: r.end.column - u,
            offset: r.end.offset - u,
            _index: r.start._index + l,
            _bufferIndex: l ? o : r.start._bufferIndex + o
          },
          end: Object.assign({}, r.end)
        };
        r.end = Object.assign({}, c.start), r.start.offset === r.end.offset ? Object.assign(r, c) : (n.splice(
          t,
          0,
          ["enter", c, e],
          ["exit", c, e]
        ), t += 2);
      }
      t++;
    }
  return n;
}
function ml(n, e, t) {
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
    consume: v,
    enter: A,
    exit: _,
    attempt: I(T),
    check: I(b),
    interrupt: I(b, {
      interrupt: !0
    })
  }, c = {
    previous: null,
    code: null,
    containerState: {},
    events: [],
    parser: n,
    sliceStream: d,
    sliceSerialize: g,
    now: m,
    defineSkip: y,
    write: h
  };
  let s = e.tokenize.call(c, a);
  return e.resolveAll && l.push(e), c;
  function h(C) {
    return o = an(o, C), x(), o[o.length - 1] !== null ? [] : (R(e, 0), c.events = Ce(l, c.events, c), c.events);
  }
  function g(C, F) {
    return gl(d(C), F);
  }
  function d(C) {
    return dl(o, C);
  }
  function m() {
    return Object.assign({}, r);
  }
  function y(C) {
    i[C.line] = C.column, X();
  }
  function x() {
    let C;
    for (; r._index < o.length; ) {
      const F = o[r._index];
      if (typeof F == "string")
        for (C = r._index, r._bufferIndex < 0 && (r._bufferIndex = 0); r._index === C && r._bufferIndex < F.length; )
          w(F.charCodeAt(r._bufferIndex));
      else
        w(F);
    }
  }
  function w(C) {
    s = s(C);
  }
  function v(C) {
    L(C) ? (r.line++, r.column = 1, r.offset += C === -3 ? 2 : 1, X()) : C !== -1 && (r.column++, r.offset++), r._bufferIndex < 0 ? r._index++ : (r._bufferIndex++, r._bufferIndex === o[r._index].length && (r._bufferIndex = -1, r._index++)), c.previous = C;
  }
  function A(C, F) {
    const B = F || {};
    return B.type = C, B.start = m(), c.events.push(["enter", B, c]), u.push(B), B;
  }
  function _(C) {
    const F = u.pop();
    return F.end = m(), c.events.push(["exit", F, c]), F;
  }
  function T(C, F) {
    R(C, F.from);
  }
  function b(C, F) {
    F.restore();
  }
  function I(C, F) {
    return B;
    function B(Z, en, un) {
      let tn, q, J, W;
      return Array.isArray(Z) ? (
        /* c8 ignore next 1 */
        p(Z)
      ) : "tokenize" in Z ? p([Z]) : f(Z);
      function f(Y) {
        return rn;
        function rn(hn) {
          const wn = hn !== null && Y[hn], Sn = hn !== null && Y.null, Rn = [
            // To do: add more extension tests.
            /* c8 ignore next 2 */
            ...Array.isArray(wn) ? wn : wn ? [wn] : [],
            ...Array.isArray(Sn) ? Sn : Sn ? [Sn] : []
          ];
          return p(Rn)(hn);
        }
      }
      function p(Y) {
        return tn = Y, q = 0, Y.length === 0 ? un : kn(Y[q]);
      }
      function kn(Y) {
        return rn;
        function rn(hn) {
          return W = M(), J = Y, Y.partial || (c.currentConstruct = Y), Y.name && c.parser.constructs.disable.null.includes(Y.name) ? bn() : Y.tokenize.call(
            // If we do have fields, create an object w/ `context` as its
            // prototype.
            // This allows a “live binding”, which is needed for `interrupt`.
            F ? Object.assign(Object.create(c), F) : c,
            a,
            Pn,
            bn
          )(hn);
        }
      }
      function Pn(Y) {
        return C(J, W), en;
      }
      function bn(Y) {
        return W.restore(), ++q < tn.length ? kn(tn[q]) : un;
      }
    }
  }
  function R(C, F) {
    C.resolveAll && !l.includes(C) && l.push(C), C.resolve && gn(
      c.events,
      F,
      c.events.length - F,
      C.resolve(c.events.slice(F), c)
    ), C.resolveTo && (c.events = C.resolveTo(c.events, c));
  }
  function M() {
    const C = m(), F = c.previous, B = c.currentConstruct, Z = c.events.length, en = Array.from(u);
    return {
      restore: un,
      from: Z
    };
    function un() {
      r = C, c.previous = F, c.currentConstruct = B, c.events.length = Z, u = en, X();
    }
  }
  function X() {
    r.line in i && r.column < 2 && (r.column = i[r.line], r.offset += i[r.line] - 1);
  }
}
function dl(n, e) {
  const t = e.start._index, r = e.start._bufferIndex, i = e.end._index, l = e.end._bufferIndex;
  let o;
  return t === i ? o = [n[t].slice(r, l)] : (o = n.slice(t, i), r > -1 && (o[0] = o[0].slice(r)), l > 0 && o.push(n[i].slice(0, l))), o;
}
function gl(n, e) {
  let t = -1;
  const r = [];
  let i;
  for (; ++t < n.length; ) {
    const l = n[t];
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
          o = e ? " " : "	";
          break;
        }
        case -1: {
          if (!e && i)
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
const yl = {
  [42]: nn,
  [43]: nn,
  [45]: nn,
  [48]: nn,
  [49]: nn,
  [50]: nn,
  [51]: nn,
  [52]: nn,
  [53]: nn,
  [54]: nn,
  [55]: nn,
  [56]: nn,
  [57]: nn,
  [62]: Dt
}, xl = {
  [91]: wi
}, kl = {
  [-2]: se,
  [-1]: se,
  [32]: se
}, bl = {
  [35]: Fi,
  [42]: Qn,
  [45]: [it, Qn],
  [60]: Li,
  [61]: it,
  [95]: Qn,
  [96]: tt,
  [126]: tt
}, wl = {
  [38]: zt,
  [92]: vt
}, Sl = {
  [-5]: ce,
  [-4]: ce,
  [-3]: ce,
  [33]: Yi,
  [38]: zt,
  [42]: xe,
  [60]: [Jr, _i],
  [91]: Xi,
  [92]: [Ai, vt],
  [93]: Pe,
  [95]: xe,
  [96]: pi
}, El = {
  null: [xe, cl]
}, Cl = {
  null: [42, 95]
}, Al = {
  null: []
}, Pl = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  attentionMarkers: Cl,
  contentInitial: xl,
  disable: Al,
  document: yl,
  flow: bl,
  flowInitial: kl,
  insideSpan: El,
  string: wl,
  text: Sl
}, Symbol.toStringTag, { value: "Module" }));
function Fl(n = {}) {
  const e = Br(
    // @ts-expect-error Same as above.
    [Pl].concat(n.extensions || [])
  ), t = {
    defined: [],
    lazy: {},
    constructs: e,
    content: r(Wr),
    document: r(Qr),
    flow: r(al),
    string: r(pl),
    text: r(fl)
  };
  return t;
  function r(i) {
    return l;
    function l(o) {
      return ml(t, i, o);
    }
  }
}
const lt = /[\0\t\n\r]/g;
function Tl() {
  let n = 1, e = "", t = !0, r;
  return i;
  function i(l, o, u) {
    const a = [];
    let c, s, h, g, d;
    for (l = e + l.toString(o), h = 0, e = "", t && (l.charCodeAt(0) === 65279 && h++, t = void 0); h < l.length; ) {
      if (lt.lastIndex = h, c = lt.exec(l), g = c && c.index !== void 0 ? c.index : l.length, d = l.charCodeAt(g), !c) {
        e = l.slice(h);
        break;
      }
      if (d === 10 && h === g && r)
        a.push(-3), r = void 0;
      else
        switch (r && (a.push(-5), r = void 0), h < g && (a.push(l.slice(h, g)), n += g - h), d) {
          case 0: {
            a.push(65533), n++;
            break;
          }
          case 9: {
            for (s = Math.ceil(n / 4) * 4, a.push(-2); n++ < s; )
              a.push(-1);
            break;
          }
          case 10: {
            a.push(-4), n = 1;
            break;
          }
          default:
            r = !0, n = 1;
        }
      h = g + 1;
    }
    return u && (r && a.push(-5), e && a.push(e), a.push(null)), a;
  }
}
function Il(n) {
  for (; !Rt(n); )
    ;
  return n;
}
function $t(n, e) {
  const t = Number.parseInt(n, e);
  return (
    // C0 except for HT, LF, FF, CR, space
    t < 9 || t === 11 || t > 13 && t < 32 || // Control character (DEL) of the basic block and C1 controls.
    t > 126 && t < 160 || // Lone high surrogates and low surrogates.
    t > 55295 && t < 57344 || // Noncharacters.
    t > 64975 && t < 65008 || (t & 65535) === 65535 || (t & 65535) === 65534 || // Out of range
    t > 1114111 ? "�" : String.fromCharCode(t)
  );
}
const Ol = /\\([!-/:-@[-`{-~])|&(#(?:\d{1,7}|x[\da-f]{1,6})|[\da-z]{1,31});/gi;
function Ll(n) {
  return n.replace(Ol, Dl);
}
function Dl(n, e, t) {
  if (e)
    return e;
  if (t.charCodeAt(0) === 35) {
    const i = t.charCodeAt(1), l = i === 120 || i === 88;
    return $t(t.slice(l ? 2 : 1), l ? 16 : 10);
  }
  return Ae(t) || n;
}
function Xn(n) {
  return !n || typeof n != "object" ? "" : "position" in n || "type" in n ? ot(n.position) : "start" in n || "end" in n ? ot(n) : "line" in n || "column" in n ? ke(n) : "";
}
function ke(n) {
  return ut(n && n.line) + ":" + ut(n && n.column);
}
function ot(n) {
  return ke(n && n.start) + "-" + ke(n && n.end);
}
function ut(n) {
  return n && typeof n == "number" ? n : 1;
}
const Ht = {}.hasOwnProperty, vl = (
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
  function(n, e, t) {
    return typeof e != "string" && (t = e, e = void 0), zl(t)(
      Il(
        // @ts-expect-error: micromark types need to accept `null`.
        Fl(t).document().write(Tl()(n, e, !0))
      )
    );
  }
);
function zl(n) {
  const e = {
    transforms: [],
    canContainEols: ["emphasis", "fragment", "heading", "paragraph", "strong"],
    enter: {
      autolink: u(De),
      autolinkProtocol: C,
      autolinkEmail: C,
      atxHeading: u(Fn),
      blockQuote: u(Rn),
      characterEscape: C,
      characterReference: C,
      codeFenced: u(Vn),
      codeFencedFenceInfo: a,
      codeFencedFenceMeta: a,
      codeIndented: u(Vn, a),
      codeText: u(ee, a),
      codeTextData: C,
      data: C,
      codeFlowValue: C,
      definition: u(P),
      definitionDestinationString: a,
      definitionLabelString: a,
      definitionTitleString: a,
      emphasis: u(_n),
      hardBreakEscape: u(Mn),
      hardBreakTrailing: u(Mn),
      htmlFlow: u(Le, a),
      htmlFlowData: C,
      htmlText: u(Le, a),
      htmlTextData: C,
      image: u(sr),
      label: a,
      link: u(De),
      listItem: u(cr),
      listItemValue: m,
      listOrdered: u(ve, d),
      listUnordered: u(ve),
      paragraph: u(pr),
      reference: bn,
      referenceString: a,
      resourceDestinationString: a,
      resourceTitleString: a,
      setextHeading: u(Fn),
      strong: u(fr),
      thematicBreak: u(mr)
    },
    exit: {
      atxHeading: s(),
      atxHeadingSequence: I,
      autolink: s(),
      autolinkEmail: Sn,
      autolinkProtocol: wn,
      blockQuote: s(),
      characterEscapeValue: F,
      characterReferenceMarkerHexadecimal: rn,
      characterReferenceMarkerNumeric: rn,
      characterReferenceValue: hn,
      codeFenced: s(v),
      codeFencedFence: w,
      codeFencedFenceInfo: y,
      codeFencedFenceMeta: x,
      codeFlowValue: F,
      codeIndented: s(A),
      codeText: s(tn),
      codeTextData: F,
      data: F,
      definition: s(),
      definitionDestinationString: b,
      definitionLabelString: _,
      definitionTitleString: T,
      emphasis: s(),
      hardBreakEscape: s(Z),
      hardBreakTrailing: s(Z),
      htmlFlow: s(en),
      htmlFlowData: F,
      htmlText: s(un),
      htmlTextData: F,
      image: s(J),
      label: f,
      labelText: W,
      lineEnding: B,
      link: s(q),
      listItem: s(),
      listOrdered: s(),
      listUnordered: s(),
      paragraph: s(),
      referenceString: Y,
      resourceDestinationString: p,
      resourceTitleString: kn,
      resource: Pn,
      setextHeading: s(X),
      setextHeadingLineSequence: M,
      setextHeadingText: R,
      strong: s(),
      thematicBreak: s()
    }
  };
  Ut(e, (n || {}).mdastExtensions || []);
  const t = {};
  return r;
  function r(k) {
    let E = {
      type: "root",
      children: []
    };
    const O = {
      stack: [E],
      tokenStack: [],
      config: e,
      enter: c,
      exit: h,
      buffer: a,
      resume: g,
      setData: l,
      getData: o
    }, $ = [];
    let H = -1;
    for (; ++H < k.length; )
      if (k[H][1].type === "listOrdered" || k[H][1].type === "listUnordered")
        if (k[H][0] === "enter")
          $.push(H);
        else {
          const fn = $.pop();
          H = i(k, fn, H);
        }
    for (H = -1; ++H < k.length; ) {
      const fn = e[k[H][0]];
      Ht.call(fn, k[H][1].type) && fn[k[H][1].type].call(
        Object.assign(
          {
            sliceSerialize: k[H][2].sliceSerialize
          },
          O
        ),
        k[H][1]
      );
    }
    if (O.tokenStack.length > 0) {
      const fn = O.tokenStack[O.tokenStack.length - 1];
      (fn[1] || at).call(O, void 0, fn[0]);
    }
    for (E.position = {
      start: Cn(
        k.length > 0 ? k[0][1].start : {
          line: 1,
          column: 1,
          offset: 0
        }
      ),
      end: Cn(
        k.length > 0 ? k[k.length - 2][1].end : {
          line: 1,
          column: 1,
          offset: 0
        }
      )
    }, H = -1; ++H < e.transforms.length; )
      E = e.transforms[H](E) || E;
    return E;
  }
  function i(k, E, O) {
    let $ = E - 1, H = -1, fn = !1, En, yn, Bn, Nn;
    for (; ++$ <= O; ) {
      const Q = k[$];
      if (Q[1].type === "listUnordered" || Q[1].type === "listOrdered" || Q[1].type === "blockQuote" ? (Q[0] === "enter" ? H++ : H--, Nn = void 0) : Q[1].type === "lineEndingBlank" ? Q[0] === "enter" && (En && !Nn && !H && !Bn && (Bn = $), Nn = void 0) : Q[1].type === "linePrefix" || Q[1].type === "listItemValue" || Q[1].type === "listItemMarker" || Q[1].type === "listItemPrefix" || Q[1].type === "listItemPrefixWhitespace" || (Nn = void 0), !H && Q[0] === "enter" && Q[1].type === "listItemPrefix" || H === -1 && Q[0] === "exit" && (Q[1].type === "listUnordered" || Q[1].type === "listOrdered")) {
        if (En) {
          let te = $;
          for (yn = void 0; te--; ) {
            const xn = k[te];
            if (xn[1].type === "lineEnding" || xn[1].type === "lineEndingBlank") {
              if (xn[0] === "exit")
                continue;
              yn && (k[yn][1].type = "lineEndingBlank", fn = !0), xn[1].type = "lineEnding", yn = te;
            } else if (!(xn[1].type === "linePrefix" || xn[1].type === "blockQuotePrefix" || xn[1].type === "blockQuotePrefixWhitespace" || xn[1].type === "blockQuoteMarker" || xn[1].type === "listItemIndent"))
              break;
          }
          Bn && (!yn || Bn < yn) && (En._spread = !0), En.end = Object.assign(
            {},
            yn ? k[yn][1].start : Q[1].end
          ), k.splice(yn || $, 0, ["exit", En, Q[2]]), $++, O++;
        }
        Q[1].type === "listItemPrefix" && (En = {
          type: "listItem",
          // @ts-expect-error Patched
          _spread: !1,
          start: Object.assign({}, Q[1].start)
        }, k.splice($, 0, ["enter", En, Q[2]]), $++, O++, Bn = void 0, Nn = !0);
      }
    }
    return k[E][1]._spread = fn, O;
  }
  function l(k, E) {
    t[k] = E;
  }
  function o(k) {
    return t[k];
  }
  function u(k, E) {
    return O;
    function O($) {
      c.call(this, k($), $), E && E.call(this, $);
    }
  }
  function a() {
    this.stack.push({
      type: "fragment",
      children: []
    });
  }
  function c(k, E, O) {
    return this.stack[this.stack.length - 1].children.push(k), this.stack.push(k), this.tokenStack.push([E, O]), k.position = {
      start: Cn(E.start)
    }, k;
  }
  function s(k) {
    return E;
    function E(O) {
      k && k.call(this, O), h.call(this, O);
    }
  }
  function h(k, E) {
    const O = this.stack.pop(), $ = this.tokenStack.pop();
    if ($)
      $[0].type !== k.type && (E ? E.call(this, k, $[0]) : ($[1] || at).call(this, k, $[0]));
    else
      throw new Error(
        "Cannot close `" + k.type + "` (" + Xn({
          start: k.start,
          end: k.end
        }) + "): it’s not open"
      );
    return O.position.end = Cn(k.end), O;
  }
  function g() {
    return _r(this.stack.pop());
  }
  function d() {
    l("expectingFirstListItemValue", !0);
  }
  function m(k) {
    if (o("expectingFirstListItemValue")) {
      const E = this.stack[this.stack.length - 2];
      E.start = Number.parseInt(this.sliceSerialize(k), 10), l("expectingFirstListItemValue");
    }
  }
  function y() {
    const k = this.resume(), E = this.stack[this.stack.length - 1];
    E.lang = k;
  }
  function x() {
    const k = this.resume(), E = this.stack[this.stack.length - 1];
    E.meta = k;
  }
  function w() {
    o("flowCodeInside") || (this.buffer(), l("flowCodeInside", !0));
  }
  function v() {
    const k = this.resume(), E = this.stack[this.stack.length - 1];
    E.value = k.replace(/^(\r?\n|\r)|(\r?\n|\r)$/g, ""), l("flowCodeInside");
  }
  function A() {
    const k = this.resume(), E = this.stack[this.stack.length - 1];
    E.value = k.replace(/(\r?\n|\r)$/g, "");
  }
  function _(k) {
    const E = this.resume(), O = this.stack[this.stack.length - 1];
    O.label = E, O.identifier = Ln(
      this.sliceSerialize(k)
    ).toLowerCase();
  }
  function T() {
    const k = this.resume(), E = this.stack[this.stack.length - 1];
    E.title = k;
  }
  function b() {
    const k = this.resume(), E = this.stack[this.stack.length - 1];
    E.url = k;
  }
  function I(k) {
    const E = this.stack[this.stack.length - 1];
    if (!E.depth) {
      const O = this.sliceSerialize(k).length;
      E.depth = O;
    }
  }
  function R() {
    l("setextHeadingSlurpLineEnding", !0);
  }
  function M(k) {
    const E = this.stack[this.stack.length - 1];
    E.depth = this.sliceSerialize(k).charCodeAt(0) === 61 ? 1 : 2;
  }
  function X() {
    l("setextHeadingSlurpLineEnding");
  }
  function C(k) {
    const E = this.stack[this.stack.length - 1];
    let O = E.children[E.children.length - 1];
    (!O || O.type !== "text") && (O = hr(), O.position = {
      start: Cn(k.start)
    }, E.children.push(O)), this.stack.push(O);
  }
  function F(k) {
    const E = this.stack.pop();
    E.value += this.sliceSerialize(k), E.position.end = Cn(k.end);
  }
  function B(k) {
    const E = this.stack[this.stack.length - 1];
    if (o("atHardBreak")) {
      const O = E.children[E.children.length - 1];
      O.position.end = Cn(k.end), l("atHardBreak");
      return;
    }
    !o("setextHeadingSlurpLineEnding") && e.canContainEols.includes(E.type) && (C.call(this, k), F.call(this, k));
  }
  function Z() {
    l("atHardBreak", !0);
  }
  function en() {
    const k = this.resume(), E = this.stack[this.stack.length - 1];
    E.value = k;
  }
  function un() {
    const k = this.resume(), E = this.stack[this.stack.length - 1];
    E.value = k;
  }
  function tn() {
    const k = this.resume(), E = this.stack[this.stack.length - 1];
    E.value = k;
  }
  function q() {
    const k = this.stack[this.stack.length - 1];
    if (o("inReference")) {
      const E = o("referenceType") || "shortcut";
      k.type += "Reference", k.referenceType = E, delete k.url, delete k.title;
    } else
      delete k.identifier, delete k.label;
    l("referenceType");
  }
  function J() {
    const k = this.stack[this.stack.length - 1];
    if (o("inReference")) {
      const E = o("referenceType") || "shortcut";
      k.type += "Reference", k.referenceType = E, delete k.url, delete k.title;
    } else
      delete k.identifier, delete k.label;
    l("referenceType");
  }
  function W(k) {
    const E = this.sliceSerialize(k), O = this.stack[this.stack.length - 2];
    O.label = Ll(E), O.identifier = Ln(E).toLowerCase();
  }
  function f() {
    const k = this.stack[this.stack.length - 1], E = this.resume(), O = this.stack[this.stack.length - 1];
    if (l("inReference", !0), O.type === "link") {
      const $ = k.children;
      O.children = $;
    } else
      O.alt = E;
  }
  function p() {
    const k = this.resume(), E = this.stack[this.stack.length - 1];
    E.url = k;
  }
  function kn() {
    const k = this.resume(), E = this.stack[this.stack.length - 1];
    E.title = k;
  }
  function Pn() {
    l("inReference");
  }
  function bn() {
    l("referenceType", "collapsed");
  }
  function Y(k) {
    const E = this.resume(), O = this.stack[this.stack.length - 1];
    O.label = E, O.identifier = Ln(
      this.sliceSerialize(k)
    ).toLowerCase(), l("referenceType", "full");
  }
  function rn(k) {
    l("characterReferenceType", k.type);
  }
  function hn(k) {
    const E = this.sliceSerialize(k), O = o("characterReferenceType");
    let $;
    O ? ($ = $t(
      E,
      O === "characterReferenceMarkerNumeric" ? 10 : 16
    ), l("characterReferenceType")) : $ = Ae(E);
    const H = this.stack.pop();
    H.value += $, H.position.end = Cn(k.end);
  }
  function wn(k) {
    F.call(this, k);
    const E = this.stack[this.stack.length - 1];
    E.url = this.sliceSerialize(k);
  }
  function Sn(k) {
    F.call(this, k);
    const E = this.stack[this.stack.length - 1];
    E.url = "mailto:" + this.sliceSerialize(k);
  }
  function Rn() {
    return {
      type: "blockquote",
      children: []
    };
  }
  function Vn() {
    return {
      type: "code",
      lang: null,
      meta: null,
      value: ""
    };
  }
  function ee() {
    return {
      type: "inlineCode",
      value: ""
    };
  }
  function P() {
    return {
      type: "definition",
      identifier: "",
      label: null,
      title: null,
      url: ""
    };
  }
  function _n() {
    return {
      type: "emphasis",
      children: []
    };
  }
  function Fn() {
    return {
      type: "heading",
      depth: void 0,
      children: []
    };
  }
  function Mn() {
    return {
      type: "break"
    };
  }
  function Le() {
    return {
      type: "html",
      value: ""
    };
  }
  function sr() {
    return {
      type: "image",
      title: null,
      url: "",
      alt: null
    };
  }
  function De() {
    return {
      type: "link",
      title: null,
      url: "",
      children: []
    };
  }
  function ve(k) {
    return {
      type: "list",
      ordered: k.type === "listOrdered",
      start: null,
      // @ts-expect-error Patched.
      spread: k._spread,
      children: []
    };
  }
  function cr(k) {
    return {
      type: "listItem",
      // @ts-expect-error Patched.
      spread: k._spread,
      checked: null,
      children: []
    };
  }
  function pr() {
    return {
      type: "paragraph",
      children: []
    };
  }
  function fr() {
    return {
      type: "strong",
      children: []
    };
  }
  function hr() {
    return {
      type: "text",
      value: ""
    };
  }
  function mr() {
    return {
      type: "thematicBreak"
    };
  }
}
function Cn(n) {
  return {
    line: n.line,
    column: n.column,
    offset: n.offset
  };
}
function Ut(n, e) {
  let t = -1;
  for (; ++t < e.length; ) {
    const r = e[t];
    Array.isArray(r) ? Ut(n, r) : Rl(n, r);
  }
}
function Rl(n, e) {
  let t;
  for (t in e)
    if (Ht.call(e, t)) {
      if (t === "canContainEols") {
        const r = e[t];
        r && n[t].push(...r);
      } else if (t === "transforms") {
        const r = e[t];
        r && n[t].push(...r);
      } else if (t === "enter" || t === "exit") {
        const r = e[t];
        r && Object.assign(n[t], r);
      }
    }
}
function at(n, e) {
  throw n ? new Error(
    "Cannot close `" + n.type + "` (" + Xn({
      start: n.start,
      end: n.end
    }) + "): a different token (`" + e.type + "`, " + Xn({
      start: e.start,
      end: e.end
    }) + ") is open"
  ) : new Error(
    "Cannot close document, a token (`" + e.type + "`, " + Xn({
      start: e.start,
      end: e.end
    }) + ") is still open"
  );
}
function _l(n) {
  Object.assign(this, { Parser: (t) => {
    const r = (
      /** @type {Options} */
      this.data("settings")
    );
    return vl(
      t,
      Object.assign({}, r, n, {
        // Note: these options are not in the readme.
        // The goal is for them to be set by plugins on `data` instead of being
        // passed by users.
        extensions: this.data("micromarkExtensions") || [],
        mdastExtensions: this.data("fromMarkdownExtensions") || []
      })
    );
  } });
}
function Ml(n, e) {
  const t = {
    type: "element",
    tagName: "blockquote",
    properties: {},
    children: n.wrap(n.all(e), !0)
  };
  return n.patch(e, t), n.applyData(e, t);
}
function Bl(n, e) {
  const t = { type: "element", tagName: "br", properties: {}, children: [] };
  return n.patch(e, t), [n.applyData(e, t), { type: "text", value: `
` }];
}
function Nl(n, e) {
  const t = e.value ? e.value + `
` : "", r = e.lang ? e.lang.match(/^[^ \t]+(?=[ \t]|$)/) : null, i = {};
  r && (i.className = ["language-" + r]);
  let l = {
    type: "element",
    tagName: "code",
    properties: i,
    children: [{ type: "text", value: t }]
  };
  return e.meta && (l.data = { meta: e.meta }), n.patch(e, l), l = n.applyData(e, l), l = { type: "element", tagName: "pre", properties: {}, children: [l] }, n.patch(e, l), l;
}
function jl(n, e) {
  const t = {
    type: "element",
    tagName: "del",
    properties: {},
    children: n.all(e)
  };
  return n.patch(e, t), n.applyData(e, t);
}
function $l(n, e) {
  const t = {
    type: "element",
    tagName: "em",
    properties: {},
    children: n.all(e)
  };
  return n.patch(e, t), n.applyData(e, t);
}
function vn(n) {
  const e = [];
  let t = -1, r = 0, i = 0;
  for (; ++t < n.length; ) {
    const l = n.charCodeAt(t);
    let o = "";
    if (l === 37 && on(n.charCodeAt(t + 1)) && on(n.charCodeAt(t + 2)))
      i = 2;
    else if (l < 128)
      /[!#$&-;=?-Z_a-z~]/.test(String.fromCharCode(l)) || (o = String.fromCharCode(l));
    else if (l > 55295 && l < 57344) {
      const u = n.charCodeAt(t + 1);
      l < 56320 && u > 56319 && u < 57344 ? (o = String.fromCharCode(l, u), i = 1) : o = "�";
    } else
      o = String.fromCharCode(l);
    o && (e.push(n.slice(r, t), encodeURIComponent(o)), r = t + i + 1, o = ""), i && (t += i, i = 0);
  }
  return e.join("") + n.slice(r);
}
function Vt(n, e) {
  const t = String(e.identifier).toUpperCase(), r = vn(t.toLowerCase()), i = n.footnoteOrder.indexOf(t);
  let l;
  i === -1 ? (n.footnoteOrder.push(t), n.footnoteCounts[t] = 1, l = n.footnoteOrder.length) : (n.footnoteCounts[t]++, l = i + 1);
  const o = n.footnoteCounts[t], u = {
    type: "element",
    tagName: "a",
    properties: {
      href: "#" + n.clobberPrefix + "fn-" + r,
      id: n.clobberPrefix + "fnref-" + r + (o > 1 ? "-" + o : ""),
      dataFootnoteRef: !0,
      ariaDescribedBy: ["footnote-label"]
    },
    children: [{ type: "text", value: String(l) }]
  };
  n.patch(e, u);
  const a = {
    type: "element",
    tagName: "sup",
    properties: {},
    children: [u]
  };
  return n.patch(e, a), n.applyData(e, a);
}
function Hl(n, e) {
  const t = n.footnoteById;
  let r = 1;
  for (; r in t; )
    r++;
  const i = String(r);
  return t[i] = {
    type: "footnoteDefinition",
    identifier: i,
    children: [{ type: "paragraph", children: e.children }],
    position: e.position
  }, Vt(n, {
    type: "footnoteReference",
    identifier: i,
    position: e.position
  });
}
function Ul(n, e) {
  const t = {
    type: "element",
    tagName: "h" + e.depth,
    properties: {},
    children: n.all(e)
  };
  return n.patch(e, t), n.applyData(e, t);
}
function Vl(n, e) {
  if (n.dangerous) {
    const t = { type: "raw", value: e.value };
    return n.patch(e, t), n.applyData(e, t);
  }
  return null;
}
function qt(n, e) {
  const t = e.referenceType;
  let r = "]";
  if (t === "collapsed" ? r += "[]" : t === "full" && (r += "[" + (e.label || e.identifier) + "]"), e.type === "imageReference")
    return { type: "text", value: "![" + e.alt + r };
  const i = n.all(e), l = i[0];
  l && l.type === "text" ? l.value = "[" + l.value : i.unshift({ type: "text", value: "[" });
  const o = i[i.length - 1];
  return o && o.type === "text" ? o.value += r : i.push({ type: "text", value: r }), i;
}
function ql(n, e) {
  const t = n.definition(e.identifier);
  if (!t)
    return qt(n, e);
  const r = { src: vn(t.url || ""), alt: e.alt };
  t.title !== null && t.title !== void 0 && (r.title = t.title);
  const i = { type: "element", tagName: "img", properties: r, children: [] };
  return n.patch(e, i), n.applyData(e, i);
}
function Wl(n, e) {
  const t = { src: vn(e.url) };
  e.alt !== null && e.alt !== void 0 && (t.alt = e.alt), e.title !== null && e.title !== void 0 && (t.title = e.title);
  const r = { type: "element", tagName: "img", properties: t, children: [] };
  return n.patch(e, r), n.applyData(e, r);
}
function Yl(n, e) {
  const t = { type: "text", value: e.value.replace(/\r?\n|\r/g, " ") };
  n.patch(e, t);
  const r = {
    type: "element",
    tagName: "code",
    properties: {},
    children: [t]
  };
  return n.patch(e, r), n.applyData(e, r);
}
function Ql(n, e) {
  const t = n.definition(e.identifier);
  if (!t)
    return qt(n, e);
  const r = { href: vn(t.url || "") };
  t.title !== null && t.title !== void 0 && (r.title = t.title);
  const i = {
    type: "element",
    tagName: "a",
    properties: r,
    children: n.all(e)
  };
  return n.patch(e, i), n.applyData(e, i);
}
function Xl(n, e) {
  const t = { href: vn(e.url) };
  e.title !== null && e.title !== void 0 && (t.title = e.title);
  const r = {
    type: "element",
    tagName: "a",
    properties: t,
    children: n.all(e)
  };
  return n.patch(e, r), n.applyData(e, r);
}
function Kl(n, e, t) {
  const r = n.all(e), i = t ? Gl(t) : Wt(e), l = {}, o = [];
  if (typeof e.checked == "boolean") {
    const s = r[0];
    let h;
    s && s.type === "element" && s.tagName === "p" ? h = s : (h = { type: "element", tagName: "p", properties: {}, children: [] }, r.unshift(h)), h.children.length > 0 && h.children.unshift({ type: "text", value: " " }), h.children.unshift({
      type: "element",
      tagName: "input",
      properties: { type: "checkbox", checked: e.checked, disabled: !0 },
      children: []
    }), l.className = ["task-list-item"];
  }
  let u = -1;
  for (; ++u < r.length; ) {
    const s = r[u];
    (i || u !== 0 || s.type !== "element" || s.tagName !== "p") && o.push({ type: "text", value: `
` }), s.type === "element" && s.tagName === "p" && !i ? o.push(...s.children) : o.push(s);
  }
  const a = r[r.length - 1];
  a && (i || a.type !== "element" || a.tagName !== "p") && o.push({ type: "text", value: `
` });
  const c = { type: "element", tagName: "li", properties: l, children: o };
  return n.patch(e, c), n.applyData(e, c);
}
function Gl(n) {
  let e = !1;
  if (n.type === "list") {
    e = n.spread || !1;
    const t = n.children;
    let r = -1;
    for (; !e && ++r < t.length; )
      e = Wt(t[r]);
  }
  return e;
}
function Wt(n) {
  const e = n.spread;
  return e ?? n.children.length > 1;
}
function Zl(n, e) {
  const t = {}, r = n.all(e);
  let i = -1;
  for (typeof e.start == "number" && e.start !== 1 && (t.start = e.start); ++i < r.length; ) {
    const o = r[i];
    if (o.type === "element" && o.tagName === "li" && o.properties && Array.isArray(o.properties.className) && o.properties.className.includes("task-list-item")) {
      t.className = ["contains-task-list"];
      break;
    }
  }
  const l = {
    type: "element",
    tagName: e.ordered ? "ol" : "ul",
    properties: t,
    children: n.wrap(r, !0)
  };
  return n.patch(e, l), n.applyData(e, l);
}
function Jl(n, e) {
  const t = {
    type: "element",
    tagName: "p",
    properties: {},
    children: n.all(e)
  };
  return n.patch(e, t), n.applyData(e, t);
}
function no(n, e) {
  const t = { type: "root", children: n.wrap(n.all(e)) };
  return n.patch(e, t), n.applyData(e, t);
}
function eo(n, e) {
  const t = {
    type: "element",
    tagName: "strong",
    properties: {},
    children: n.all(e)
  };
  return n.patch(e, t), n.applyData(e, t);
}
const Fe = Yt("start"), Te = Yt("end");
function to(n) {
  return { start: Fe(n), end: Te(n) };
}
function Yt(n) {
  return e;
  function e(t) {
    const r = t && t.position && t.position[n] || {};
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
function ro(n, e) {
  const t = n.all(e), r = t.shift(), i = [];
  if (r) {
    const o = {
      type: "element",
      tagName: "thead",
      properties: {},
      children: n.wrap([r], !0)
    };
    n.patch(e.children[0], o), i.push(o);
  }
  if (t.length > 0) {
    const o = {
      type: "element",
      tagName: "tbody",
      properties: {},
      children: n.wrap(t, !0)
    }, u = Fe(e.children[1]), a = Te(e.children[e.children.length - 1]);
    u.line && a.line && (o.position = { start: u, end: a }), i.push(o);
  }
  const l = {
    type: "element",
    tagName: "table",
    properties: {},
    children: n.wrap(i, !0)
  };
  return n.patch(e, l), n.applyData(e, l);
}
function io(n, e, t) {
  const r = t ? t.children : void 0, l = (r ? r.indexOf(e) : 1) === 0 ? "th" : "td", o = t && t.type === "table" ? t.align : void 0, u = o ? o.length : e.children.length;
  let a = -1;
  const c = [];
  for (; ++a < u; ) {
    const h = e.children[a], g = {}, d = o ? o[a] : void 0;
    d && (g.align = d);
    let m = { type: "element", tagName: l, properties: g, children: [] };
    h && (m.children = n.all(h), n.patch(h, m), m = n.applyData(e, m)), c.push(m);
  }
  const s = {
    type: "element",
    tagName: "tr",
    properties: {},
    children: n.wrap(c, !0)
  };
  return n.patch(e, s), n.applyData(e, s);
}
function lo(n, e) {
  const t = {
    type: "element",
    tagName: "td",
    // Assume body cell.
    properties: {},
    children: n.all(e)
  };
  return n.patch(e, t), n.applyData(e, t);
}
const st = 9, ct = 32;
function oo(n) {
  const e = String(n), t = /\r?\n|\r/g;
  let r = t.exec(e), i = 0;
  const l = [];
  for (; r; )
    l.push(
      pt(e.slice(i, r.index), i > 0, !0),
      r[0]
    ), i = r.index + r[0].length, r = t.exec(e);
  return l.push(pt(e.slice(i), i > 0, !1)), l.join("");
}
function pt(n, e, t) {
  let r = 0, i = n.length;
  if (e) {
    let l = n.codePointAt(r);
    for (; l === st || l === ct; )
      r++, l = n.codePointAt(r);
  }
  if (t) {
    let l = n.codePointAt(i - 1);
    for (; l === st || l === ct; )
      i--, l = n.codePointAt(i - 1);
  }
  return i > r ? n.slice(r, i) : "";
}
function uo(n, e) {
  const t = { type: "text", value: oo(String(e.value)) };
  return n.patch(e, t), n.applyData(e, t);
}
function ao(n, e) {
  const t = {
    type: "element",
    tagName: "hr",
    properties: {},
    children: []
  };
  return n.patch(e, t), n.applyData(e, t);
}
const so = {
  blockquote: Ml,
  break: Bl,
  code: Nl,
  delete: jl,
  emphasis: $l,
  footnoteReference: Vt,
  footnote: Hl,
  heading: Ul,
  html: Vl,
  imageReference: ql,
  image: Wl,
  inlineCode: Yl,
  linkReference: Ql,
  link: Xl,
  listItem: Kl,
  list: Zl,
  paragraph: Jl,
  root: no,
  strong: eo,
  table: ro,
  tableCell: lo,
  tableRow: io,
  text: uo,
  thematicBreak: ao,
  toml: qn,
  yaml: qn,
  definition: qn,
  footnoteDefinition: qn
};
function qn() {
  return null;
}
const Qt = (
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
  function(n) {
    if (n == null)
      return ho;
    if (typeof n == "string")
      return fo(n);
    if (typeof n == "object")
      return Array.isArray(n) ? co(n) : po(n);
    if (typeof n == "function")
      return Jn(n);
    throw new Error("Expected function, string, or object as test");
  }
);
function co(n) {
  const e = [];
  let t = -1;
  for (; ++t < n.length; )
    e[t] = Qt(n[t]);
  return Jn(r);
  function r(...i) {
    let l = -1;
    for (; ++l < e.length; )
      if (e[l].call(this, ...i))
        return !0;
    return !1;
  }
}
function po(n) {
  return Jn(e);
  function e(t) {
    let r;
    for (r in n)
      if (t[r] !== n[r])
        return !1;
    return !0;
  }
}
function fo(n) {
  return Jn(e);
  function e(t) {
    return t && t.type === n;
  }
}
function Jn(n) {
  return e;
  function e(t, ...r) {
    return !!(t && typeof t == "object" && "type" in t && n.call(this, t, ...r));
  }
}
function ho() {
  return !0;
}
const mo = !0, ft = !1, go = "skip", yo = (
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
  function(n, e, t, r) {
    typeof e == "function" && typeof t != "function" && (r = t, t = e, e = null);
    const i = Qt(e), l = r ? -1 : 1;
    o(n, void 0, [])();
    function o(u, a, c) {
      const s = u && typeof u == "object" ? u : {};
      if (typeof s.type == "string") {
        const g = (
          // `hast`
          typeof s.tagName == "string" ? s.tagName : (
            // `xast`
            typeof s.name == "string" ? s.name : void 0
          )
        );
        Object.defineProperty(h, "name", {
          value: "node (" + (u.type + (g ? "<" + g + ">" : "")) + ")"
        });
      }
      return h;
      function h() {
        let g = [], d, m, y;
        if ((!e || i(u, a, c[c.length - 1] || null)) && (g = xo(t(u, c)), g[0] === ft))
          return g;
        if (u.children && g[0] !== go)
          for (m = (r ? u.children.length : -1) + l, y = c.concat(u); m > -1 && m < u.children.length; ) {
            if (d = o(u.children[m], m, y)(), d[0] === ft)
              return d;
            m = typeof d[1] == "number" ? d[1] : m + l;
          }
        return g;
      }
    }
  }
);
function xo(n) {
  return Array.isArray(n) ? n : typeof n == "number" ? [mo, n] : [n];
}
const Xt = (
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
  function(n, e, t, r) {
    typeof e == "function" && typeof t != "function" && (r = t, t = e, e = null), yo(n, e, i, r);
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
function ko(n) {
  return !n || !n.position || !n.position.start || !n.position.start.line || !n.position.start.column || !n.position.end || !n.position.end.line || !n.position.end.column;
}
const ht = {}.hasOwnProperty;
function bo(n) {
  const e = /* @__PURE__ */ Object.create(null);
  if (!n || !n.type)
    throw new Error("mdast-util-definitions expected node");
  return Xt(n, "definition", (r) => {
    const i = mt(r.identifier);
    i && !ht.call(e, i) && (e[i] = r);
  }), t;
  function t(r) {
    const i = mt(r);
    return i && ht.call(e, i) ? e[i] : null;
  }
}
function mt(n) {
  return String(n || "").toUpperCase();
}
const Kn = {}.hasOwnProperty;
function wo(n, e) {
  const t = e || {}, r = t.allowDangerousHtml || !1, i = {};
  return o.dangerous = r, o.clobberPrefix = t.clobberPrefix === void 0 || t.clobberPrefix === null ? "user-content-" : t.clobberPrefix, o.footnoteLabel = t.footnoteLabel || "Footnotes", o.footnoteLabelTagName = t.footnoteLabelTagName || "h2", o.footnoteLabelProperties = t.footnoteLabelProperties || {
    className: ["sr-only"]
  }, o.footnoteBackLabel = t.footnoteBackLabel || "Back to content", o.unknownHandler = t.unknownHandler, o.passThrough = t.passThrough, o.handlers = { ...so, ...t.handlers }, o.definition = bo(n), o.footnoteById = i, o.footnoteOrder = [], o.footnoteCounts = {}, o.patch = So, o.applyData = Eo, o.one = u, o.all = a, o.wrap = Ao, o.augment = l, Xt(n, "footnoteDefinition", (c) => {
    const s = String(c.identifier).toUpperCase();
    Kn.call(i, s) || (i[s] = c);
  }), o;
  function l(c, s) {
    if (c && "data" in c && c.data) {
      const h = c.data;
      h.hName && (s.type !== "element" && (s = {
        type: "element",
        tagName: "",
        properties: {},
        children: []
      }), s.tagName = h.hName), s.type === "element" && h.hProperties && (s.properties = { ...s.properties, ...h.hProperties }), "children" in s && s.children && h.hChildren && (s.children = h.hChildren);
    }
    if (c) {
      const h = "type" in c ? c : { position: c };
      ko(h) || (s.position = { start: Fe(h), end: Te(h) });
    }
    return s;
  }
  function o(c, s, h, g) {
    return Array.isArray(h) && (g = h, h = {}), l(c, {
      type: "element",
      tagName: s,
      properties: h || {},
      children: g || []
    });
  }
  function u(c, s) {
    return Kt(o, c, s);
  }
  function a(c) {
    return Ie(o, c);
  }
}
function So(n, e) {
  n.position && (e.position = to(n));
}
function Eo(n, e) {
  let t = e;
  if (n && n.data) {
    const r = n.data.hName, i = n.data.hChildren, l = n.data.hProperties;
    typeof r == "string" && (t.type === "element" ? t.tagName = r : t = {
      type: "element",
      tagName: r,
      properties: {},
      children: []
    }), t.type === "element" && l && (t.properties = { ...t.properties, ...l }), "children" in t && t.children && i !== null && i !== void 0 && (t.children = i);
  }
  return t;
}
function Kt(n, e, t) {
  const r = e && e.type;
  if (!r)
    throw new Error("Expected node, got `" + e + "`");
  return Kn.call(n.handlers, r) ? n.handlers[r](n, e, t) : n.passThrough && n.passThrough.includes(r) ? "children" in e ? { ...e, children: Ie(n, e) } : e : n.unknownHandler ? n.unknownHandler(n, e, t) : Co(n, e);
}
function Ie(n, e) {
  const t = [];
  if ("children" in e) {
    const r = e.children;
    let i = -1;
    for (; ++i < r.length; ) {
      const l = Kt(n, r[i], e);
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
function Co(n, e) {
  const t = e.data || {}, r = "value" in e && !(Kn.call(t, "hProperties") || Kn.call(t, "hChildren")) ? { type: "text", value: e.value } : {
    type: "element",
    tagName: "div",
    properties: {},
    children: Ie(n, e)
  };
  return n.patch(e, r), n.applyData(e, r);
}
function Ao(n, e) {
  const t = [];
  let r = -1;
  for (e && t.push({ type: "text", value: `
` }); ++r < n.length; )
    r && t.push({ type: "text", value: `
` }), t.push(n[r]);
  return e && n.length > 0 && t.push({ type: "text", value: `
` }), t;
}
function Po(n) {
  const e = [];
  let t = -1;
  for (; ++t < n.footnoteOrder.length; ) {
    const r = n.footnoteById[n.footnoteOrder[t]];
    if (!r)
      continue;
    const i = n.all(r), l = String(r.identifier).toUpperCase(), o = vn(l.toLowerCase());
    let u = 0;
    const a = [];
    for (; ++u <= n.footnoteCounts[l]; ) {
      const h = {
        type: "element",
        tagName: "a",
        properties: {
          href: "#" + n.clobberPrefix + "fnref-" + o + (u > 1 ? "-" + u : ""),
          dataFootnoteBackref: !0,
          className: ["data-footnote-backref"],
          ariaLabel: n.footnoteBackLabel
        },
        children: [{ type: "text", value: "↩" }]
      };
      u > 1 && h.children.push({
        type: "element",
        tagName: "sup",
        children: [{ type: "text", value: String(u) }]
      }), a.length > 0 && a.push({ type: "text", value: " " }), a.push(h);
    }
    const c = i[i.length - 1];
    if (c && c.type === "element" && c.tagName === "p") {
      const h = c.children[c.children.length - 1];
      h && h.type === "text" ? h.value += " " : c.children.push({ type: "text", value: " " }), c.children.push(...a);
    } else
      i.push(...a);
    const s = {
      type: "element",
      tagName: "li",
      properties: { id: n.clobberPrefix + "fn-" + o },
      children: n.wrap(i, !0)
    };
    n.patch(r, s), e.push(s);
  }
  if (e.length !== 0)
    return {
      type: "element",
      tagName: "section",
      properties: { dataFootnotes: !0, className: ["footnotes"] },
      children: [
        {
          type: "element",
          tagName: n.footnoteLabelTagName,
          properties: {
            // To do: use structured clone.
            ...JSON.parse(JSON.stringify(n.footnoteLabelProperties)),
            id: "footnote-label"
          },
          children: [{ type: "text", value: n.footnoteLabel }]
        },
        { type: "text", value: `
` },
        {
          type: "element",
          tagName: "ol",
          properties: {},
          children: n.wrap(e, !0)
        },
        { type: "text", value: `
` }
      ]
    };
}
function Gt(n, e) {
  const t = wo(n, e), r = t.one(n, null), i = Po(t);
  return i && r.children.push({ type: "text", value: `
` }, i), Array.isArray(r) ? { type: "root", children: r } : r;
}
const Fo = (
  /** @type {(import('unified').Plugin<[Processor, Options?]|[null|undefined, Options?]|[Options]|[], MdastRoot>)} */
  function(n, e) {
    return n && "run" in n ? Io(n, e) : Oo(n || e);
  }
), To = Fo;
function Io(n, e) {
  return (t, r, i) => {
    n.run(Gt(t, e), r, (l) => {
      i(l);
    });
  };
}
function Oo(n) {
  return (e) => Gt(e, n);
}
class Un {
  /**
   * @constructor
   * @param {Properties} property
   * @param {Normal} normal
   * @param {string} [space]
   */
  constructor(e, t, r) {
    this.property = e, this.normal = t, r && (this.space = r);
  }
}
Un.prototype.property = {};
Un.prototype.normal = {};
Un.prototype.space = null;
function Zt(n, e) {
  const t = {}, r = {};
  let i = -1;
  for (; ++i < n.length; )
    Object.assign(t, n[i].property), Object.assign(r, n[i].normal);
  return new Un(t, r, e);
}
function be(n) {
  return n.toLowerCase();
}
class pn {
  /**
   * @constructor
   * @param {string} property
   * @param {string} attribute
   */
  constructor(e, t) {
    this.property = e, this.attribute = t;
  }
}
pn.prototype.space = null;
pn.prototype.boolean = !1;
pn.prototype.booleanish = !1;
pn.prototype.overloadedBoolean = !1;
pn.prototype.number = !1;
pn.prototype.commaSeparated = !1;
pn.prototype.spaceSeparated = !1;
pn.prototype.commaOrSpaceSeparated = !1;
pn.prototype.mustUseProperty = !1;
pn.prototype.defined = !1;
let Lo = 0;
const z = In(), K = In(), Jt = In(), S = In(), V = In(), Dn = In(), ln = In();
function In() {
  return 2 ** ++Lo;
}
const we = /* @__PURE__ */ Object.freeze(/* @__PURE__ */ Object.defineProperty({
  __proto__: null,
  boolean: z,
  booleanish: K,
  commaOrSpaceSeparated: ln,
  commaSeparated: Dn,
  number: S,
  overloadedBoolean: Jt,
  spaceSeparated: V
}, Symbol.toStringTag, { value: "Module" })), pe = Object.keys(we);
class Oe extends pn {
  /**
   * @constructor
   * @param {string} property
   * @param {string} attribute
   * @param {number|null} [mask]
   * @param {string} [space]
   */
  constructor(e, t, r, i) {
    let l = -1;
    if (super(e, t), dt(this, "space", i), typeof r == "number")
      for (; ++l < pe.length; ) {
        const o = pe[l];
        dt(this, pe[l], (r & we[o]) === we[o]);
      }
  }
}
Oe.prototype.defined = !0;
function dt(n, e, t) {
  t && (n[e] = t);
}
const Do = {}.hasOwnProperty;
function zn(n) {
  const e = {}, t = {};
  let r;
  for (r in n.properties)
    if (Do.call(n.properties, r)) {
      const i = n.properties[r], l = new Oe(
        r,
        n.transform(n.attributes || {}, r),
        i,
        n.space
      );
      n.mustUseProperty && n.mustUseProperty.includes(r) && (l.mustUseProperty = !0), e[r] = l, t[be(r)] = r, t[be(l.attribute)] = r;
    }
  return new Un(e, t, n.space);
}
const nr = zn({
  space: "xlink",
  transform(n, e) {
    return "xlink:" + e.slice(5).toLowerCase();
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
}), er = zn({
  space: "xml",
  transform(n, e) {
    return "xml:" + e.slice(3).toLowerCase();
  },
  properties: { xmlLang: null, xmlBase: null, xmlSpace: null }
});
function tr(n, e) {
  return e in n ? n[e] : e;
}
function rr(n, e) {
  return tr(n, e.toLowerCase());
}
const ir = zn({
  space: "xmlns",
  attributes: { xmlnsxlink: "xmlns:xlink" },
  transform: rr,
  properties: { xmlns: null, xmlnsXLink: null }
}), lr = zn({
  transform(n, e) {
    return e === "role" ? e : "aria-" + e.slice(4).toLowerCase();
  },
  properties: {
    ariaActiveDescendant: null,
    ariaAtomic: K,
    ariaAutoComplete: null,
    ariaBusy: K,
    ariaChecked: K,
    ariaColCount: S,
    ariaColIndex: S,
    ariaColSpan: S,
    ariaControls: V,
    ariaCurrent: null,
    ariaDescribedBy: V,
    ariaDetails: null,
    ariaDisabled: K,
    ariaDropEffect: V,
    ariaErrorMessage: null,
    ariaExpanded: K,
    ariaFlowTo: V,
    ariaGrabbed: K,
    ariaHasPopup: null,
    ariaHidden: K,
    ariaInvalid: null,
    ariaKeyShortcuts: null,
    ariaLabel: null,
    ariaLabelledBy: V,
    ariaLevel: S,
    ariaLive: null,
    ariaModal: K,
    ariaMultiLine: K,
    ariaMultiSelectable: K,
    ariaOrientation: null,
    ariaOwns: V,
    ariaPlaceholder: null,
    ariaPosInSet: S,
    ariaPressed: K,
    ariaReadOnly: K,
    ariaRelevant: null,
    ariaRequired: K,
    ariaRoleDescription: V,
    ariaRowCount: S,
    ariaRowIndex: S,
    ariaRowSpan: S,
    ariaSelected: K,
    ariaSetSize: S,
    ariaSort: null,
    ariaValueMax: S,
    ariaValueMin: S,
    ariaValueNow: S,
    ariaValueText: null,
    role: null
  }
}), vo = zn({
  space: "html",
  attributes: {
    acceptcharset: "accept-charset",
    classname: "class",
    htmlfor: "for",
    httpequiv: "http-equiv"
  },
  transform: rr,
  mustUseProperty: ["checked", "multiple", "muted", "selected"],
  properties: {
    // Standard Properties.
    abbr: null,
    accept: Dn,
    acceptCharset: V,
    accessKey: V,
    action: null,
    allow: null,
    allowFullScreen: z,
    allowPaymentRequest: z,
    allowUserMedia: z,
    alt: null,
    as: null,
    async: z,
    autoCapitalize: null,
    autoComplete: V,
    autoFocus: z,
    autoPlay: z,
    capture: z,
    charSet: null,
    checked: z,
    cite: null,
    className: V,
    cols: S,
    colSpan: null,
    content: null,
    contentEditable: K,
    controls: z,
    controlsList: V,
    coords: S | Dn,
    crossOrigin: null,
    data: null,
    dateTime: null,
    decoding: null,
    default: z,
    defer: z,
    dir: null,
    dirName: null,
    disabled: z,
    download: Jt,
    draggable: K,
    encType: null,
    enterKeyHint: null,
    form: null,
    formAction: null,
    formEncType: null,
    formMethod: null,
    formNoValidate: z,
    formTarget: null,
    headers: V,
    height: S,
    hidden: z,
    high: S,
    href: null,
    hrefLang: null,
    htmlFor: V,
    httpEquiv: V,
    id: null,
    imageSizes: null,
    imageSrcSet: null,
    inputMode: null,
    integrity: null,
    is: null,
    isMap: z,
    itemId: null,
    itemProp: V,
    itemRef: V,
    itemScope: z,
    itemType: V,
    kind: null,
    label: null,
    lang: null,
    language: null,
    list: null,
    loading: null,
    loop: z,
    low: S,
    manifest: null,
    max: null,
    maxLength: S,
    media: null,
    method: null,
    min: null,
    minLength: S,
    multiple: z,
    muted: z,
    name: null,
    nonce: null,
    noModule: z,
    noValidate: z,
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
    open: z,
    optimum: S,
    pattern: null,
    ping: V,
    placeholder: null,
    playsInline: z,
    poster: null,
    preload: null,
    readOnly: z,
    referrerPolicy: null,
    rel: V,
    required: z,
    reversed: z,
    rows: S,
    rowSpan: S,
    sandbox: V,
    scope: null,
    scoped: z,
    seamless: z,
    selected: z,
    shape: null,
    size: S,
    sizes: null,
    slot: null,
    span: S,
    spellCheck: K,
    src: null,
    srcDoc: null,
    srcLang: null,
    srcSet: null,
    start: S,
    step: null,
    style: null,
    tabIndex: S,
    target: null,
    title: null,
    translate: null,
    type: null,
    typeMustMatch: z,
    useMap: null,
    value: K,
    width: S,
    wrap: null,
    // Legacy.
    // See: https://html.spec.whatwg.org/#other-elements,-attributes-and-apis
    align: null,
    // Several. Use CSS `text-align` instead,
    aLink: null,
    // `<body>`. Use CSS `a:active {color}` instead
    archive: V,
    // `<object>`. List of URIs to archives
    axis: null,
    // `<td>` and `<th>`. Use `scope` on `<th>`
    background: null,
    // `<body>`. Use CSS `background-image` instead
    bgColor: null,
    // `<body>` and table elements. Use CSS `background-color` instead
    border: S,
    // `<table>`. Use CSS `border-width` instead,
    borderColor: null,
    // `<table>`. Use CSS `border-color` instead,
    bottomMargin: S,
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
    compact: z,
    // Lists. Use CSS to reduce space between items instead
    declare: z,
    // `<object>`
    event: null,
    // `<script>`
    face: null,
    // `<font>`. Use CSS instead
    frame: null,
    // `<table>`
    frameBorder: null,
    // `<iframe>`. Use CSS `border` instead
    hSpace: S,
    // `<img>` and `<object>`
    leftMargin: S,
    // `<body>`
    link: null,
    // `<body>`. Use CSS `a:link {color: *}` instead
    longDesc: null,
    // `<frame>`, `<iframe>`, and `<img>`. Use an `<a>`
    lowSrc: null,
    // `<img>`. Use a `<picture>`
    marginHeight: S,
    // `<body>`
    marginWidth: S,
    // `<body>`
    noResize: z,
    // `<frame>`
    noHref: z,
    // `<area>`. Use no href instead of an explicit `nohref`
    noShade: z,
    // `<hr>`. Use background-color and height instead of borders
    noWrap: z,
    // `<td>` and `<th>`
    object: null,
    // `<applet>`
    profile: null,
    // `<head>`
    prompt: null,
    // `<isindex>`
    rev: null,
    // `<link>`
    rightMargin: S,
    // `<body>`
    rules: null,
    // `<table>`
    scheme: null,
    // `<meta>`
    scrolling: K,
    // `<frame>`. Use overflow in the child context
    standby: null,
    // `<object>`
    summary: null,
    // `<table>`
    text: null,
    // `<body>`. Use CSS `color` instead
    topMargin: S,
    // `<body>`
    valueType: null,
    // `<param>`
    version: null,
    // `<html>`. Use a doctype.
    vAlign: null,
    // Several. Use CSS `vertical-align` instead
    vLink: null,
    // `<body>`. Use CSS `a:visited {color}` instead
    vSpace: S,
    // `<img>` and `<object>`
    // Non-standard Properties.
    allowTransparency: null,
    autoCorrect: null,
    autoSave: null,
    disablePictureInPicture: z,
    disableRemotePlayback: z,
    prefix: null,
    property: null,
    results: S,
    security: null,
    unselectable: null
  }
}), zo = zn({
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
  transform: tr,
  properties: {
    about: ln,
    accentHeight: S,
    accumulate: null,
    additive: null,
    alignmentBaseline: null,
    alphabetic: S,
    amplitude: S,
    arabicForm: null,
    ascent: S,
    attributeName: null,
    attributeType: null,
    azimuth: S,
    bandwidth: null,
    baselineShift: null,
    baseFrequency: null,
    baseProfile: null,
    bbox: null,
    begin: null,
    bias: S,
    by: null,
    calcMode: null,
    capHeight: S,
    className: V,
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
    descent: S,
    diffuseConstant: S,
    direction: null,
    display: null,
    dur: null,
    divisor: S,
    dominantBaseline: null,
    download: z,
    dx: null,
    dy: null,
    edgeMode: null,
    editable: null,
    elevation: S,
    enableBackground: null,
    end: null,
    event: null,
    exponent: S,
    externalResourcesRequired: null,
    fill: null,
    fillOpacity: S,
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
    g1: Dn,
    g2: Dn,
    glyphName: Dn,
    glyphOrientationHorizontal: null,
    glyphOrientationVertical: null,
    glyphRef: null,
    gradientTransform: null,
    gradientUnits: null,
    handler: null,
    hanging: S,
    hatchContentUnits: null,
    hatchUnits: null,
    height: null,
    href: null,
    hrefLang: null,
    horizAdvX: S,
    horizOriginX: S,
    horizOriginY: S,
    id: null,
    ideographic: S,
    imageRendering: null,
    initialVisibility: null,
    in: null,
    in2: null,
    intercept: S,
    k: S,
    k1: S,
    k2: S,
    k3: S,
    k4: S,
    kernelMatrix: ln,
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
    limitingConeAngle: S,
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
    mediaSize: S,
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
    overlinePosition: S,
    overlineThickness: S,
    paintOrder: null,
    panose1: null,
    path: null,
    pathLength: S,
    patternContentUnits: null,
    patternTransform: null,
    patternUnits: null,
    phase: null,
    ping: V,
    pitch: null,
    playbackOrder: null,
    pointerEvents: null,
    points: null,
    pointsAtX: S,
    pointsAtY: S,
    pointsAtZ: S,
    preserveAlpha: null,
    preserveAspectRatio: null,
    primitiveUnits: null,
    propagate: null,
    property: ln,
    r: null,
    radius: null,
    referrerPolicy: null,
    refX: null,
    refY: null,
    rel: ln,
    rev: ln,
    renderingIntent: null,
    repeatCount: null,
    repeatDur: null,
    requiredExtensions: ln,
    requiredFeatures: ln,
    requiredFonts: ln,
    requiredFormats: ln,
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
    specularConstant: S,
    specularExponent: S,
    spreadMethod: null,
    spacing: null,
    startOffset: null,
    stdDeviation: null,
    stemh: null,
    stemv: null,
    stitchTiles: null,
    stopColor: null,
    stopOpacity: null,
    strikethroughPosition: S,
    strikethroughThickness: S,
    string: null,
    stroke: null,
    strokeDashArray: ln,
    strokeDashOffset: null,
    strokeLineCap: null,
    strokeLineJoin: null,
    strokeMiterLimit: S,
    strokeOpacity: S,
    strokeWidth: null,
    style: null,
    surfaceScale: S,
    syncBehavior: null,
    syncBehaviorDefault: null,
    syncMaster: null,
    syncTolerance: null,
    syncToleranceDefault: null,
    systemLanguage: ln,
    tabIndex: S,
    tableValues: null,
    target: null,
    targetX: S,
    targetY: S,
    textAnchor: null,
    textDecoration: null,
    textRendering: null,
    textLength: null,
    timelineBegin: null,
    title: null,
    transformBehavior: null,
    type: null,
    typeOf: ln,
    to: null,
    transform: null,
    u1: null,
    u2: null,
    underlinePosition: S,
    underlineThickness: S,
    unicode: null,
    unicodeBidi: null,
    unicodeRange: null,
    unitsPerEm: S,
    values: null,
    vAlphabetic: S,
    vMathematical: S,
    vectorEffect: null,
    vHanging: S,
    vIdeographic: S,
    version: null,
    vertAdvY: S,
    vertOriginX: S,
    vertOriginY: S,
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
    xHeight: S,
    y: null,
    y1: null,
    y2: null,
    yChannelSelector: null,
    z: null,
    zoomAndPan: null
  }
}), Ro = /^data[-\w.:]+$/i, gt = /-[a-z]/g, _o = /[A-Z]/g;
function Mo(n, e) {
  const t = be(e);
  let r = e, i = pn;
  if (t in n.normal)
    return n.property[n.normal[t]];
  if (t.length > 4 && t.slice(0, 4) === "data" && Ro.test(e)) {
    if (e.charAt(4) === "-") {
      const l = e.slice(5).replace(gt, No);
      r = "data" + l.charAt(0).toUpperCase() + l.slice(1);
    } else {
      const l = e.slice(4);
      if (!gt.test(l)) {
        let o = l.replace(_o, Bo);
        o.charAt(0) !== "-" && (o = "-" + o), e = "data" + o;
      }
    }
    i = Oe;
  }
  return new i(r, e);
}
function Bo(n) {
  return "-" + n.toLowerCase();
}
function No(n) {
  return n.charAt(1).toUpperCase();
}
const yt = {
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
}, jo = Zt([er, nr, ir, lr, vo], "html"), $o = Zt([er, nr, ir, lr, zo], "svg"), or = (
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
  function(n) {
    if (n == null)
      return qo;
    if (typeof n == "string")
      return Vo(n);
    if (typeof n == "object")
      return Array.isArray(n) ? Ho(n) : Uo(n);
    if (typeof n == "function")
      return ne(n);
    throw new Error("Expected function, string, or object as test");
  }
);
function Ho(n) {
  const e = [];
  let t = -1;
  for (; ++t < n.length; )
    e[t] = or(n[t]);
  return ne(r);
  function r(...i) {
    let l = -1;
    for (; ++l < e.length; )
      if (e[l].call(this, ...i))
        return !0;
    return !1;
  }
}
function Uo(n) {
  return ne(e);
  function e(t) {
    let r;
    for (r in n)
      if (t[r] !== n[r])
        return !1;
    return !0;
  }
}
function Vo(n) {
  return ne(e);
  function e(t) {
    return t && t.type === n;
  }
}
function ne(n) {
  return e;
  function e(t, ...r) {
    return !!(t && typeof t == "object" && "type" in t && n.call(this, t, ...r));
  }
}
function qo() {
  return !0;
}
const Wo = !0, xt = !1, Yo = "skip", Qo = (
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
  function(n, e, t, r) {
    typeof e == "function" && typeof t != "function" && (r = t, t = e, e = null);
    const i = or(e), l = r ? -1 : 1;
    o(n, void 0, [])();
    function o(u, a, c) {
      const s = u && typeof u == "object" ? u : {};
      if (typeof s.type == "string") {
        const g = (
          // `hast`
          typeof s.tagName == "string" ? s.tagName : (
            // `xast`
            typeof s.name == "string" ? s.name : void 0
          )
        );
        Object.defineProperty(h, "name", {
          value: "node (" + (u.type + (g ? "<" + g + ">" : "")) + ")"
        });
      }
      return h;
      function h() {
        let g = [], d, m, y;
        if ((!e || i(u, a, c[c.length - 1] || null)) && (g = Xo(t(u, c)), g[0] === xt))
          return g;
        if (u.children && g[0] !== Yo)
          for (m = (r ? u.children.length : -1) + l, y = c.concat(u); m > -1 && m < u.children.length; ) {
            if (d = o(u.children[m], m, y)(), d[0] === xt)
              return d;
            m = typeof d[1] == "number" ? d[1] : m + l;
          }
        return g;
      }
    }
  }
);
function Xo(n) {
  return Array.isArray(n) ? n : typeof n == "number" ? [Wo, n] : [n];
}
const Ko = (
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
  function(n, e, t, r) {
    typeof e == "function" && typeof t != "function" && (r = t, t = e, e = null), Qo(n, e, i, r);
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
function Go(n) {
  if (n.allowedElements && n.disallowedElements)
    throw new TypeError(
      "Only one of `allowedElements` and `disallowedElements` should be defined"
    );
  if (n.allowedElements || n.disallowedElements || n.allowElement)
    return (e) => {
      Ko(e, "element", (t, r, i) => {
        const l = (
          /** @type {Element|Root} */
          i
        );
        let o;
        if (n.allowedElements ? o = !n.allowedElements.includes(t.tagName) : n.disallowedElements && (o = n.disallowedElements.includes(t.tagName)), !o && n.allowElement && typeof r == "number" && (o = !n.allowElement(t, r, l)), o && typeof r == "number")
          return n.unwrapDisallowed && t.children ? l.children.splice(r, 1, ...t.children) : l.children.splice(r, 1), r;
      });
    };
}
var Se = {}, Zo = {
  get exports() {
    return Se;
  },
  set exports(n) {
    Se = n;
  }
}, N = {};
/**
 * @license React
 * react-is.production.min.js
 *
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
var kt;
function Jo() {
  if (kt)
    return N;
  kt = 1;
  var n = Symbol.for("react.element"), e = Symbol.for("react.portal"), t = Symbol.for("react.fragment"), r = Symbol.for("react.strict_mode"), i = Symbol.for("react.profiler"), l = Symbol.for("react.provider"), o = Symbol.for("react.context"), u = Symbol.for("react.server_context"), a = Symbol.for("react.forward_ref"), c = Symbol.for("react.suspense"), s = Symbol.for("react.suspense_list"), h = Symbol.for("react.memo"), g = Symbol.for("react.lazy"), d = Symbol.for("react.offscreen"), m;
  m = Symbol.for("react.module.reference");
  function y(x) {
    if (typeof x == "object" && x !== null) {
      var w = x.$$typeof;
      switch (w) {
        case n:
          switch (x = x.type, x) {
            case t:
            case i:
            case r:
            case c:
            case s:
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
                  return w;
              }
          }
        case e:
          return w;
      }
    }
  }
  return N.ContextConsumer = o, N.ContextProvider = l, N.Element = n, N.ForwardRef = a, N.Fragment = t, N.Lazy = g, N.Memo = h, N.Portal = e, N.Profiler = i, N.StrictMode = r, N.Suspense = c, N.SuspenseList = s, N.isAsyncMode = function() {
    return !1;
  }, N.isConcurrentMode = function() {
    return !1;
  }, N.isContextConsumer = function(x) {
    return y(x) === o;
  }, N.isContextProvider = function(x) {
    return y(x) === l;
  }, N.isElement = function(x) {
    return typeof x == "object" && x !== null && x.$$typeof === n;
  }, N.isForwardRef = function(x) {
    return y(x) === a;
  }, N.isFragment = function(x) {
    return y(x) === t;
  }, N.isLazy = function(x) {
    return y(x) === g;
  }, N.isMemo = function(x) {
    return y(x) === h;
  }, N.isPortal = function(x) {
    return y(x) === e;
  }, N.isProfiler = function(x) {
    return y(x) === i;
  }, N.isStrictMode = function(x) {
    return y(x) === r;
  }, N.isSuspense = function(x) {
    return y(x) === c;
  }, N.isSuspenseList = function(x) {
    return y(x) === s;
  }, N.isValidElementType = function(x) {
    return typeof x == "string" || typeof x == "function" || x === t || x === i || x === r || x === c || x === s || x === d || typeof x == "object" && x !== null && (x.$$typeof === g || x.$$typeof === h || x.$$typeof === l || x.$$typeof === o || x.$$typeof === a || x.$$typeof === m || x.getModuleId !== void 0);
  }, N.typeOf = y, N;
}
var j = {};
/**
 * @license React
 * react-is.development.js
 *
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
var bt;
function nu() {
  return bt || (bt = 1,  false && 0), j;
}
(function(n) {
   true ? n.exports = Jo() : 0;
})(Zo);
const eu = /* @__PURE__ */ (0,_index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.g)(Se);
function tu(n) {
  const e = (
    // @ts-expect-error looks like a node.
    n && typeof n == "object" && n.type === "text" ? (
      // @ts-expect-error looks like a text.
      n.value || ""
    ) : n
  );
  return typeof e == "string" && e.replace(/[ \t\n\f\r]/g, "") === "";
}
function ru(n) {
  return n.join(" ").trim();
}
function iu(n, e) {
  const t = e || {};
  return (n[n.length - 1] === "" ? [...n, ""] : n).join(
    (t.padRight ? " " : "") + "," + (t.padLeft === !1 ? "" : " ")
  ).trim();
}
var Gn = {}, lu = {
  get exports() {
    return Gn;
  },
  set exports(n) {
    Gn = n;
  }
}, wt = /\/\*[^*]*\*+([^/*][^*]*\*+)*\//g, ou = /\n/g, uu = /^\s*/, au = /^(\*?[-#/*\\\w]+(\[[0-9a-z_-]+\])?)\s*/, su = /^:\s*/, cu = /^((?:'(?:\\'|.)*?'|"(?:\\"|.)*?"|\([^)]*?\)|[^};])+)/, pu = /^[;\s]*/, fu = /^\s+|\s+$/g, hu = `
`, St = "/", Et = "*", Tn = "", mu = "comment", du = "declaration", gu = function(n, e) {
  if (typeof n != "string")
    throw new TypeError("First argument must be a string");
  if (!n)
    return [];
  e = e || {};
  var t = 1, r = 1;
  function i(m) {
    var y = m.match(ou);
    y && (t += y.length);
    var x = m.lastIndexOf(hu);
    r = ~x ? m.length - x : r + m.length;
  }
  function l() {
    var m = { line: t, column: r };
    return function(y) {
      return y.position = new o(m), c(), y;
    };
  }
  function o(m) {
    this.start = m, this.end = { line: t, column: r }, this.source = e.source;
  }
  o.prototype.content = n;
  function u(m) {
    var y = new Error(
      e.source + ":" + t + ":" + r + ": " + m
    );
    if (y.reason = m, y.filename = e.source, y.line = t, y.column = r, y.source = n, !e.silent)
      throw y;
  }
  function a(m) {
    var y = m.exec(n);
    if (y) {
      var x = y[0];
      return i(x), n = n.slice(x.length), y;
    }
  }
  function c() {
    a(uu);
  }
  function s(m) {
    var y;
    for (m = m || []; y = h(); )
      y !== !1 && m.push(y);
    return m;
  }
  function h() {
    var m = l();
    if (!(St != n.charAt(0) || Et != n.charAt(1))) {
      for (var y = 2; Tn != n.charAt(y) && (Et != n.charAt(y) || St != n.charAt(y + 1)); )
        ++y;
      if (y += 2, Tn === n.charAt(y - 1))
        return u("End of comment missing");
      var x = n.slice(2, y - 2);
      return r += 2, i(x), n = n.slice(y), r += 2, m({
        type: mu,
        comment: x
      });
    }
  }
  function g() {
    var m = l(), y = a(au);
    if (y) {
      if (h(), !a(su))
        return u("property missing ':'");
      var x = a(cu), w = m({
        type: du,
        property: Ct(y[0].replace(wt, Tn)),
        value: x ? Ct(x[0].replace(wt, Tn)) : Tn
      });
      return a(pu), w;
    }
  }
  function d() {
    var m = [];
    s(m);
    for (var y; y = g(); )
      y !== !1 && (m.push(y), s(m));
    return m;
  }
  return c(), d();
};
function Ct(n) {
  return n ? n.replace(fu, Tn) : Tn;
}
var yu = gu;
function ur(n, e) {
  var t = null;
  if (!n || typeof n != "string")
    return t;
  for (var r, i = yu(n), l = typeof e == "function", o, u, a = 0, c = i.length; a < c; a++)
    r = i[a], o = r.property, u = r.value, l ? e(o, u, r) : u && (t || (t = {}), t[o] = u);
  return t;
}
lu.exports = ur;
Gn.default = ur;
const xu = Gn, Ee = {}.hasOwnProperty, ku = /* @__PURE__ */ new Set(["table", "thead", "tbody", "tfoot", "tr"]);
function ar(n, e) {
  const t = [];
  let r = -1, i;
  for (; ++r < e.children.length; )
    i = e.children[r], i.type === "element" ? t.push(bu(n, i, r, e)) : i.type === "text" ? (e.type !== "element" || !ku.has(e.tagName) || !tu(i)) && t.push(i.value) : i.type === "raw" && !n.options.skipHtml && t.push(i.value);
  return t;
}
function bu(n, e, t, r) {
  const i = n.options, l = i.transformLinkUri === void 0 ? gr : i.transformLinkUri, o = n.schema, u = e.tagName, a = {};
  let c = o, s;
  if (o.space === "html" && u === "svg" && (c = $o, n.schema = c), e.properties)
    for (s in e.properties)
      Ee.call(e.properties, s) && Su(a, s, e.properties[s], n);
  (u === "ol" || u === "ul") && n.listDepth++;
  const h = ar(n, e);
  (u === "ol" || u === "ul") && n.listDepth--, n.schema = o;
  const g = e.position || {
    start: { line: null, column: null, offset: null },
    end: { line: null, column: null, offset: null }
  }, d = i.components && Ee.call(i.components, u) ? i.components[u] : u, m = typeof d == "string" || d === react__WEBPACK_IMPORTED_MODULE_0__.Fragment;
  if (!eu.isValidElementType(d))
    throw new TypeError(
      `Component for name \`${u}\` not defined or is not renderable`
    );
  if (a.key = [
    u,
    g.start.line,
    g.start.column,
    t
  ].join("-"), u === "a" && i.linkTarget && (a.target = typeof i.linkTarget == "function" ? i.linkTarget(
    String(a.href || ""),
    e.children,
    typeof a.title == "string" ? a.title : null
  ) : i.linkTarget), u === "a" && l && (a.href = l(
    String(a.href || ""),
    e.children,
    typeof a.title == "string" ? a.title : null
  )), !m && u === "code" && r.type === "element" && r.tagName !== "pre" && (a.inline = !0), !m && (u === "h1" || u === "h2" || u === "h3" || u === "h4" || u === "h5" || u === "h6") && (a.level = Number.parseInt(u.charAt(1), 10)), u === "img" && i.transformImageUri && (a.src = i.transformImageUri(
    String(a.src || ""),
    String(a.alt || ""),
    typeof a.title == "string" ? a.title : null
  )), !m && u === "li" && r.type === "element") {
    const y = wu(e);
    a.checked = y && y.properties ? !!y.properties.checked : null, a.index = fe(r, e), a.ordered = r.tagName === "ol";
  }
  return !m && (u === "ol" || u === "ul") && (a.ordered = u === "ol", a.depth = n.listDepth), (u === "td" || u === "th") && (a.align && (a.style || (a.style = {}), a.style.textAlign = a.align, delete a.align), m || (a.isHeader = u === "th")), !m && u === "tr" && r.type === "element" && (a.isHeader = r.tagName === "thead"), i.sourcePos && (a["data-sourcepos"] = Au(g)), !m && i.rawSourcePos && (a.sourcePosition = e.position), !m && i.includeElementIndex && (a.index = fe(r, e), a.siblingCount = fe(r)), m || (a.node = e), h.length > 0 ? react__WEBPACK_IMPORTED_MODULE_0__.createElement(d, a, h) : react__WEBPACK_IMPORTED_MODULE_0__.createElement(d, a);
}
function wu(n) {
  let e = -1;
  for (; ++e < n.children.length; ) {
    const t = n.children[e];
    if (t.type === "element" && t.tagName === "input")
      return t;
  }
  return null;
}
function fe(n, e) {
  let t = -1, r = 0;
  for (; ++t < n.children.length && n.children[t] !== e; )
    n.children[t].type === "element" && r++;
  return r;
}
function Su(n, e, t, r) {
  const i = Mo(r.schema, e);
  let l = t;
  l == null || l !== l || (Array.isArray(l) && (l = i.commaSeparated ? iu(l) : ru(l)), i.property === "style" && typeof l == "string" && (l = Eu(l)), i.space && i.property ? n[Ee.call(yt, i.property) ? yt[i.property] : i.property] = l : i.attribute && (n[i.attribute] = l));
}
function Eu(n) {
  const e = {};
  try {
    xu(n, t);
  } catch {
  }
  return e;
  function t(r, i) {
    const l = r.slice(0, 4) === "-ms-" ? `ms-${r.slice(4)}` : r;
    e[l.replace(/-([a-z])/g, Cu)] = i;
  }
}
function Cu(n, e) {
  return e.toUpperCase();
}
function Au(n) {
  return [
    n.start.line,
    ":",
    n.start.column,
    "-",
    n.end.line,
    ":",
    n.end.column
  ].map(String).join("");
}
const At = {}.hasOwnProperty, Pu = "https://github.com/remarkjs/react-markdown/blob/main/changelog.md", Wn = {
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
function Fu(n) {
  for (const l in Wn)
    if (At.call(Wn, l) && At.call(n, l)) {
      const o = Wn[l];
      console.warn(
        `[react-markdown] Warning: please ${o.to ? `use \`${o.to}\` instead of` : "remove"} \`${l}\` (see <${Pu}#${o.id}> for more info)`
      ), delete Wn[l];
    }
  const e = Dr().use(_l).use(n.remarkPlugins || []).use(To, {
    ...n.remarkRehypeOptions,
    allowDangerousHtml: !0
  }).use(n.rehypePlugins || []).use(Go, n), t = new Ft();
  typeof n.children == "string" ? t.value = n.children : n.children !== void 0 && n.children !== null && console.warn(
    `[react-markdown] Warning: please pass a string as \`children\` (not: \`${n.children}\`)`
  );
  const r = e.runSync(e.parse(t), t);
  if (r.type !== "root")
    throw new TypeError("Expected a `root` node");
  let i = react__WEBPACK_IMPORTED_MODULE_0__.createElement(
    react__WEBPACK_IMPORTED_MODULE_0__.Fragment,
    {},
    ar({ options: n, schema: jo, listDepth: 0 }, r)
  );
  return n.className && (i = react__WEBPACK_IMPORTED_MODULE_0__.createElement("div", { className: n.className }, i)), i;
}
Fu.propTypes = {
  // Core options:
  children: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.string,
  // Layout options:
  className: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.string,
  // Filter options:
  allowElement: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.func,
  allowedElements: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.arrayOf(_index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.string),
  disallowedElements: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.arrayOf(_index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.string),
  unwrapDisallowed: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.bool,
  // Plugin options:
  remarkPlugins: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.arrayOf(
    _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.oneOfType([
      _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.object,
      _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.func,
      _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.arrayOf(
        _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.oneOfType([
          _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.bool,
          _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.string,
          _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.object,
          _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.func,
          _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.arrayOf(
            // prettier-ignore
            // type-coverage:ignore-next-line
            _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.any
          )
        ])
      )
    ])
  ),
  rehypePlugins: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.arrayOf(
    _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.oneOfType([
      _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.object,
      _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.func,
      _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.arrayOf(
        _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.oneOfType([
          _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.bool,
          _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.string,
          _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.object,
          _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.func,
          _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.arrayOf(
            // prettier-ignore
            // type-coverage:ignore-next-line
            _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.any
          )
        ])
      )
    ])
  ),
  // Transform options:
  sourcePos: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.bool,
  rawSourcePos: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.bool,
  skipHtml: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.bool,
  includeElementIndex: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.bool,
  transformLinkUri: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.oneOfType([_index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.func, _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.bool]),
  linkTarget: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.oneOfType([_index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.func, _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.string]),
  transformImageUri: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.func,
  components: _index_601da423_js__WEBPACK_IMPORTED_MODULE_1__.p.object
};



/***/ })

};
;