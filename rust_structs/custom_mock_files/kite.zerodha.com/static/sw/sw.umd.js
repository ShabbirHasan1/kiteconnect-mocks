(function(e, t) {
    "object" === typeof exports && "object" === typeof module ? module.exports = t() : "function" === typeof define && define.amd ? define([], t) : "object" === typeof exports ? exports["sw"] = t() : e["sw"] = t()
})("undefined" !== typeof self ? self : this, (function() {
    return function(e) {
        var t = {};

        function n(i) {
            if (t[i]) return t[i].exports;
            var r = t[i] = {
                i: i,
                l: !1,
                exports: {}
            };
            return e[i].call(r.exports, r, r.exports, n), r.l = !0, r.exports
        }
        return n.m = e, n.c = t, n.d = function(e, t, i) {
            n.o(e, t) || Object.defineProperty(e, t, {
                enumerable: !0,
                get: i
            })
        }, n.r = function(e) {
            "undefined" !== typeof Symbol && Symbol.toStringTag && Object.defineProperty(e, Symbol.toStringTag, {
                value: "Module"
            }), Object.defineProperty(e, "__esModule", {
                value: !0
            })
        }, n.t = function(e, t) {
            if (1 & t && (e = n(e)), 8 & t) return e;
            if (4 & t && "object" === typeof e && e && e.__esModule) return e;
            var i = Object.create(null);
            if (n.r(i), Object.defineProperty(i, "default", {
                    enumerable: !0,
                    value: e
                }), 2 & t && "string" != typeof e)
                for (var r in e) n.d(i, r, function(t) {
                    return e[t]
                }.bind(null, r));
            return i
        }, n.n = function(e) {
            var t = e && e.__esModule ? function() {
                return e["default"]
            } : function() {
                return e
            };
            return n.d(t, "a", t), t
        }, n.o = function(e, t) {
            return Object.prototype.hasOwnProperty.call(e, t)
        }, n.p = "", n(n.s = "fae3")
    }({
        "0ab6": function(e, t, n) {
            var i = n("3053"),
                r = i.Global;

            function s() {
                return r.sessionStorage
            }

            function o(e) {
                return s().getItem(e)
            }

            function a(e, t) {
                return s().setItem(e, t)
            }

            function c(e) {
                for (var t = s().length - 1; t >= 0; t--) {
                    var n = s().key(t);
                    e(o(n), n)
                }
            }

            function u(e) {
                return s().removeItem(e)
            }

            function l() {
                return s().clear()
            }
            e.exports = {
                name: "sessionStorage",
                read: o,
                write: a,
                each: c,
                remove: u,
                clearAll: l
            }
        },
        "0e54": function(e, t, n) {
            var i = n("3053"),
                r = i.Global,
                s = i.trim;
            e.exports = {
                name: "cookieStorage",
                read: a,
                write: u,
                each: c,
                remove: l,
                clearAll: h
            };
            var o = r.document;

            function a(e) {
                if (!e || !f(e)) return null;
                var t = "(?:^|.*;\\s*)" + escape(e).replace(/[\-\.\+\*]/g, "\\$&") + "\\s*\\=\\s*((?:[^;](?!;))*[^;]?).*";
                return unescape(o.cookie.replace(new RegExp(t), "$1"))
            }

            function c(e) {
                for (var t = o.cookie.split(/; ?/g), n = t.length - 1; n >= 0; n--)
                    if (s(t[n])) {
                        var i = t[n].split("="),
                            r = unescape(i[0]),
                            a = unescape(i[1]);
                        e(a, r)
                    }
            }

            function u(e, t) {
                e && (o.cookie = escape(e) + "=" + escape(t) + "; expires=Tue, 19 Jan 2038 03:14:07 GMT; path=/")
            }

            function l(e) {
                e && f(e) && (o.cookie = escape(e) + "=; expires=Thu, 01 Jan 1970 00:00:00 GMT; path=/")
            }

            function h() {
                c((function(e, t) {
                    l(t)
                }))
            }

            function f(e) {
                return new RegExp("(?:^|;\\s*)" + escape(e).replace(/[\-\.\+\*]/g, "\\$&") + "\\s*\\=").test(o.cookie)
            }
        },
        3053: function(e, t, n) {
            (function(t) {
                var n = o(),
                    i = a(),
                    r = c(),
                    s = "undefined" !== typeof window ? window : t;

                function o() {
                    return Object.assign ? Object.assign : function(e, t, n, i) {
                        for (var r = 1; r < arguments.length; r++) h(Object(arguments[r]), (function(t, n) {
                            e[n] = t
                        }));
                        return e
                    }
                }

                function a() {
                    if (Object.create) return function(e, t, i, r) {
                        var s = l(arguments, 1);
                        return n.apply(this, [Object.create(e)].concat(s))
                    }; {
                        function e() {}
                        return function(t, i, r, s) {
                            var o = l(arguments, 1);
                            return e.prototype = t, n.apply(this, [new e].concat(o))
                        }
                    }
                }

                function c() {
                    return String.prototype.trim ? function(e) {
                        return String.prototype.trim.call(e)
                    } : function(e) {
                        return e.replace(/^[\s\uFEFF\xA0]+|[\s\uFEFF\xA0]+$/g, "")
                    }
                }

                function u(e, t) {
                    return function() {
                        return t.apply(e, Array.prototype.slice.call(arguments, 0))
                    }
                }

                function l(e, t) {
                    return Array.prototype.slice.call(e, t || 0)
                }

                function h(e, t) {
                    g(e, (function(e, n) {
                        return t(e, n), !1
                    }))
                }

                function f(e, t) {
                    var n = d(e) ? [] : {};
                    return g(e, (function(e, i) {
                        return n[i] = t(e, i), !1
                    })), n
                }

                function g(e, t) {
                    if (d(e)) {
                        for (var n = 0; n < e.length; n++)
                            if (t(e[n], n)) return e[n]
                    } else
                        for (var i in e)
                            if (e.hasOwnProperty(i) && t(e[i], i)) return e[i]
                }

                function d(e) {
                    return null != e && "function" != typeof e && "number" == typeof e.length
                }

                function v(e) {
                    return e && "[object Function]" === {}.toString.call(e)
                }

                function p(e) {
                    return e && "[object Object]" === {}.toString.call(e)
                }
                e.exports = {
                    assign: n,
                    create: i,
                    trim: r,
                    bind: u,
                    slice: l,
                    each: h,
                    map: f,
                    pluck: g,
                    isList: d,
                    isFunction: v,
                    isObject: p,
                    Global: s
                }
            }).call(this, n("c8ba"))
        },
        3928: function(e, t) {
            function n() {
                var e = {};
                return {
                    defaults: t,
                    get: n
                };

                function t(t, n) {
                    e = n
                }

                function n(t, n) {
                    var i = t();
                    return void 0 !== i ? i : e[n]
                }
            }
            e.exports = n
        },
        "4a7c": function(e, t, n) {
            "use strict";
            (function(t) {
                /*!
                Copyright (C) 2015-2017 Andrea Giammarchi - @WebReflection

                Permission is hereby granted, free of charge, to any person obtaining a copy
                of this software and associated documentation files (the "Software"), to deal
                in the Software without restriction, including without limitation the rights
                to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
                copies of the Software, and to permit persons to whom the Software is
                furnished to do so, subject to the following conditions:

                The above copyright notice and this permission notice shall be included in
                all copies or substantial portions of the Software.

                THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
                IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
                FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
                AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
                LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
                OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
                THE SOFTWARE.

                */
                function n(e) {
                    var t, n, r, s, o, a, c = Object.create(null);
                    if (this[u] = c, e)
                        if ("string" === typeof e)
                            for ("?" === e.charAt(0) && (e = e.slice(1)), s = e.split("&"), o = 0, a = s.length; o < a; o++) r = s[o], t = r.indexOf("="), -1 < t ? l(c, h(r.slice(0, t)), h(r.slice(t + 1))) : r.length && l(c, h(r), "");
                        else if (i(e))
                        for (o = 0, a = e.length; o < a; o++) r = e[o], l(c, r[0], r[1]);
                    else
                        for (n in e) l(c, n, e[n])
                }
                var i = Array.isArray,
                    r = n.prototype,
                    s = /[!'\(\)~]|%20|%00/g,
                    o = /\+/g,
                    a = {
                        "!": "%21",
                        "'": "%27",
                        "(": "%28",
                        ")": "%29",
                        "~": "%7E",
                        "%20": "+",
                        "%00": "\0"
                    },
                    c = function(e) {
                        return a[e]
                    },
                    u = "__URLSearchParams__:" + Math.random();

                function l(e, t, n) {
                    t in e ? e[t].push("" + n) : e[t] = i(n) ? n : ["" + n]
                }

                function h(e) {
                    return decodeURIComponent(e.replace(o, " "))
                }

                function f(e) {
                    return encodeURIComponent(e).replace(s, c)
                }
                r.append = function(e, t) {
                        l(this[u], e, t)
                    }, r.delete = function(e) {
                        delete this[u][e]
                    }, r.get = function(e) {
                        var t = this[u];
                        return e in t ? t[e][0] : null
                    }, r.getAll = function(e) {
                        var t = this[u];
                        return e in t ? t[e].slice(0) : []
                    }, r.has = function(e) {
                        return e in this[u]
                    }, r.set = function(e, t) {
                        this[u][e] = ["" + t]
                    }, r.forEach = function(e, t) {
                        var n = this[u];
                        Object.getOwnPropertyNames(n).forEach((function(i) {
                            n[i].forEach((function(n) {
                                e.call(t, n, i, this)
                            }), this)
                        }), this)
                    }, r.toJSON = function() {
                        return {}
                    }, r.toString = function() {
                        var e, t, n, i, r = this[u],
                            s = [];
                        for (t in r)
                            for (n = f(t), e = 0, i = r[t]; e < i.length; e++) s.push(n + "=" + f(i[e]));
                        return s.join("&")
                    }, n = e.exports = t.URLSearchParams || n,
                    function(e) {
                        var t = function() {
                            try {
                                return !!Symbol.iterator
                            } catch (e) {
                                return !1
                            }
                        }();
                        "forEach" in e || (e.forEach = function(e, t) {
                            var n = Object.create(null);
                            this.toString().replace(/=[\s\S]*?(?:&|$)/g, "=").split("=").forEach((function(i) {
                                i.length && !(i in n) && (n[i] = this.getAll(i)).forEach((function(n) {
                                    e.call(t, n, i, this)
                                }), this)
                            }), this)
                        }), "keys" in e || (e.keys = function() {
                            var e = [];
                            this.forEach((function(t, n) {
                                e.push(n)
                            }));
                            var n = {
                                next: function() {
                                    var t = e.shift();
                                    return {
                                        done: void 0 === t,
                                        value: t
                                    }
                                }
                            };
                            return t && (n[Symbol.iterator] = function() {
                                return n
                            }), n
                        }), "values" in e || (e.values = function() {
                            var e = [];
                            this.forEach((function(t) {
                                e.push(t)
                            }));
                            var n = {
                                next: function() {
                                    var t = e.shift();
                                    return {
                                        done: void 0 === t,
                                        value: t
                                    }
                                }
                            };
                            return t && (n[Symbol.iterator] = function() {
                                return n
                            }), n
                        }), "entries" in e || (e.entries = function() {
                            var e = [];
                            this.forEach((function(t, n) {
                                e.push([n, t])
                            }));
                            var n = {
                                next: function() {
                                    var t = e.shift();
                                    return {
                                        done: void 0 === t,
                                        value: t
                                    }
                                }
                            };
                            return t && (n[Symbol.iterator] = function() {
                                return n
                            }), n
                        }), t && !(Symbol.iterator in e) && (e[Symbol.iterator] = e.entries), "sort" in e || (e.sort = function() {
                            var e, t, n, i = this.entries(),
                                r = i.next(),
                                s = r.done,
                                o = [],
                                a = Object.create(null);
                            while (!s) n = r.value, t = n[0], o.push(t), t in a || (a[t] = []), a[t].push(n[1]), r = i.next(), s = r.done;
                            for (o.sort(), e = 0; e < o.length; e++) this.delete(o[e]);
                            for (e = 0; e < o.length; e++) t = o[e], this.append(t, a[t].shift())
                        })
                    }(n.prototype)
            }).call(this, n("c8ba"))
        },
        8875: function(e, t, n) {
            var i, r, s;
            (function(n, o) {
                r = [], i = o, s = "function" === typeof i ? i.apply(t, r) : i, void 0 === s || (e.exports = s)
            })("undefined" !== typeof self && self, (function() {
                function e() {
                    var t = Object.getOwnPropertyDescriptor(document, "currentScript");
                    if (!t && "currentScript" in document && document.currentScript) return document.currentScript;
                    if (t && t.get !== e && document.currentScript) return document.currentScript;
                    try {
                        throw new Error
                    } catch (g) {
                        var n, i, r, s = /.*at [^(]*\((.*):(.+):(.+)\)$/gi,
                            o = /@([^@]*):(\d+):(\d+)\s*$/gi,
                            a = s.exec(g.stack) || o.exec(g.stack),
                            c = a && a[1] || !1,
                            u = a && a[2] || !1,
                            l = document.location.href.replace(document.location.hash, ""),
                            h = document.getElementsByTagName("script");
                        c === l && (n = document.documentElement.outerHTML, i = new RegExp("(?:[^\\n]+?\\n){0," + (u - 2) + "}[^<]*<script>([\\d\\D]*?)<\\/script>[\\d\\D]*", "i"), r = n.replace(i, "$1").trim());
                        for (var f = 0; f < h.length; f++) {
                            if ("interactive" === h[f].readyState) return h[f];
                            if (h[f].src === c) return h[f];
                            if (c === l && h[f].innerHTML && h[f].innerHTML.trim() === r) return h[f]
                        }
                        return null
                    }
                }
                return e
            }))
        },
        add5: function(e, t, n) {
            var i = n("3053"),
                r = i.Global;

            function s() {
                return r.localStorage
            }

            function o(e) {
                return s().getItem(e)
            }

            function a(e, t) {
                return s().setItem(e, t)
            }

            function c(e) {
                for (var t = s().length - 1; t >= 0; t--) {
                    var n = s().key(t);
                    e(o(n), n)
                }
            }

            function u(e) {
                return s().removeItem(e)
            }

            function l() {
                return s().clear()
            }
            e.exports = {
                name: "localStorage",
                read: o,
                write: a,
                each: c,
                remove: u,
                clearAll: l
            }
        },
        c8ba: function(e, t) {
            var n;
            n = function() {
                return this
            }();
            try {
                n = n || new Function("return this")()
            } catch (i) {
                "object" === typeof window && (n = window)
            }
            e.exports = n
        },
        e675: function(e, t, n) {
            var i = n("3053"),
                r = i.slice,
                s = i.pluck,
                o = i.each,
                a = i.bind,
                c = i.create,
                u = i.isList,
                l = i.isFunction,
                h = i.isObject;
            e.exports = {
                createStore: d
            };
            var f = {
                version: "2.0.12",
                enabled: !1,
                get: function(e, t) {
                    var n = this.storage.read(this._namespacePrefix + e);
                    return this._deserialize(n, t)
                },
                set: function(e, t) {
                    return void 0 === t ? this.remove(e) : (this.storage.write(this._namespacePrefix + e, this._serialize(t)), t)
                },
                remove: function(e) {
                    this.storage.remove(this._namespacePrefix + e)
                },
                each: function(e) {
                    var t = this;
                    this.storage.each((function(n, i) {
                        e.call(t, t._deserialize(n), (i || "").replace(t._namespaceRegexp, ""))
                    }))
                },
                clearAll: function() {
                    this.storage.clearAll()
                },
                hasNamespace: function(e) {
                    return this._namespacePrefix == "__storejs_" + e + "_"
                },
                createStore: function() {
                    return d.apply(this, arguments)
                },
                addPlugin: function(e) {
                    this._addPlugin(e)
                },
                namespace: function(e) {
                    return d(this.storage, this.plugins, e)
                }
            };

            function g() {
                var e = "undefined" == typeof console ? null : console;
                if (e) {
                    var t = e.warn ? e.warn : e.log;
                    t.apply(e, arguments)
                }
            }

            function d(e, t, n) {
                n || (n = ""), e && !u(e) && (e = [e]), t && !u(t) && (t = [t]);
                var i = n ? "__storejs_" + n + "_" : "",
                    d = n ? new RegExp("^" + i) : null,
                    v = /^[a-zA-Z0-9_\-]*$/;
                if (!v.test(n)) throw new Error("store.js namespaces can only have alphanumerics + underscores and dashes");
                var p = {
                        _namespacePrefix: i,
                        _namespaceRegexp: d,
                        _testStorage: function(e) {
                            try {
                                var t = "__storejs__test__";
                                e.write(t, t);
                                var n = e.read(t) === t;
                                return e.remove(t), n
                            } catch (i) {
                                return !1
                            }
                        },
                        _assignPluginFnProp: function(e, t) {
                            var n = this[t];
                            this[t] = function() {
                                var t = r(arguments, 0),
                                    i = this;

                                function s() {
                                    if (n) return o(arguments, (function(e, n) {
                                        t[n] = e
                                    })), n.apply(i, t)
                                }
                                var a = [s].concat(t);
                                return e.apply(i, a)
                            }
                        },
                        _serialize: function(e) {
                            return JSON.stringify(e)
                        },
                        _deserialize: function(e, t) {
                            if (!e) return t;
                            var n = "";
                            try {
                                n = JSON.parse(e)
                            } catch (i) {
                                n = e
                            }
                            return void 0 !== n ? n : t
                        },
                        _addStorage: function(e) {
                            this.enabled || this._testStorage(e) && (this.storage = e, this.enabled = !0)
                        },
                        _addPlugin: function(e) {
                            var t = this;
                            if (u(e)) o(e, (function(e) {
                                t._addPlugin(e)
                            }));
                            else {
                                var n = s(this.plugins, (function(t) {
                                    return e === t
                                }));
                                if (!n) {
                                    if (this.plugins.push(e), !l(e)) throw new Error("Plugins must be function values that return objects");
                                    var i = e.call(this);
                                    if (!h(i)) throw new Error("Plugins must return an object of function properties");
                                    o(i, (function(n, i) {
                                        if (!l(n)) throw new Error("Bad plugin property: " + i + " from plugin " + e.name + ". Plugins should only return functions.");
                                        t._assignPluginFnProp(n, i)
                                    }))
                                }
                            }
                        },
                        addStorage: function(e) {
                            g("store.addStorage(storage) is deprecated. Use createStore([storages])"), this._addStorage(e)
                        }
                    },
                    y = c(p, f, {
                        plugins: []
                    });
                return y.raw = {}, o(y, (function(e, t) {
                    l(e) && (y.raw[t] = a(y, e))
                })), o(e, (function(e) {
                    y._addStorage(e)
                })), o(t, (function(e) {
                    y._addPlugin(e)
                })), y
            }
        },
        ee7c: function(e, t) {
            var n = "expire_mixin";

            function i() {
                var e = this.createStore(this.storage, null, this._namespacePrefix + n);
                return {
                    set: t,
                    get: i,
                    remove: r,
                    getExpiration: s,
                    removeExpiredKeys: o
                };

                function t(t, i, r, s) {
                    return this.hasNamespace(n) || e.set(i, s), t()
                }

                function i(e, t) {
                    return this.hasNamespace(n) || a.call(this, t), e()
                }

                function r(t, i) {
                    return this.hasNamespace(n) || e.remove(i), t()
                }

                function s(t, n) {
                    return e.get(n)
                }

                function o(e) {
                    var t = [];
                    this.each((function(e, n) {
                        t.push(n)
                    }));
                    for (var n = 0; n < t.length; n++) a.call(this, t[n])
                }

                function a(t) {
                    var n = e.get(t, Number.MAX_VALUE);
                    n <= (new Date).getTime() && (this.raw.remove(t), e.remove(t))
                }
            }
            e.exports = i
        },
        fae3: function(e, t, n) {
            "use strict";
            if (n.r(t), "undefined" !== typeof window) {
                var i = window.document.currentScript,
                    r = n("8875");
                i = r(), "currentScript" in document || Object.defineProperty(document, "currentScript", {
                    get: r
                });
                var s = i && i.src.match(/(.+\/)[^/]+\.js(\?.*)?$/);
                s && (n.p = s[1])
            }
            var o = n("4a7c"),
                a = n.n(o);

            function c(e, t) {
                var n;
                if ("undefined" === typeof Symbol || null == e[Symbol.iterator]) {
                    if (Array.isArray(e) || (n = u(e)) || t && e && "number" === typeof e.length) {
                        n && (e = n);
                        var i = 0,
                            r = function() {};
                        return {
                            s: r,
                            n: function() {
                                return i >= e.length ? {
                                    done: !0
                                } : {
                                    done: !1,
                                    value: e[i++]
                                }
                            },
                            e: function(e) {
                                throw e
                            },
                            f: r
                        }
                    }
                    throw new TypeError("Invalid attempt to iterate non-iterable instance.\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.")
                }
                var s, o = !0,
                    a = !1;
                return {
                    s: function() {
                        n = e[Symbol.iterator]()
                    },
                    n: function() {
                        var e = n.next();
                        return o = e.done, e
                    },
                    e: function(e) {
                        a = !0, s = e
                    },
                    f: function() {
                        try {
                            o || null == n.return || n.return()
                        } finally {
                            if (a) throw s
                        }
                    }
                }
            }

            function u(e, t) {
                if (e) {
                    if ("string" === typeof e) return l(e, t);
                    var n = Object.prototype.toString.call(e).slice(8, -1);
                    return "Object" === n && e.constructor && (n = e.constructor.name), "Map" === n || "Set" === n ? Array.from(e) : "Arguments" === n || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n) ? l(e, t) : void 0
                }
            }

            function l(e, t) {
                (null == t || t > e.length) && (t = e.length);
                for (var n = 0, i = new Array(t); n < t; n++) i[n] = e[n];
                return i
            }

            function h(e, t, n) {
                return t in e ? Object.defineProperty(e, t, {
                    value: n,
                    enumerable: !0,
                    configurable: !0,
                    writable: !0
                }) : e[t] = n, e
            }

            function f(e, t) {
                for (var n = 0; n < t.length; n++) {
                    var i = t[n];
                    i.enumerable = i.enumerable || !1, i.configurable = !0, "value" in i && (i.writable = !0), Object.defineProperty(e, i.key, i)
                }
            }

            function g(e, t, n) {
                return t && f(e.prototype, t), n && f(e, n), e
            }

            function d(e, t) {
                if (!(e instanceof t)) throw new TypeError("Cannot call a class as a function")
            }
            var v = function e() {
                    d(this, e);
                    var t = this;
                    this.promise = new Promise((function(e, n) {
                        t.resolve = e, t.reject = n
                    }))
                },
                p = function() {
                    function e() {
                        d(this, e), this.handlers = []
                    }
                    return g(e, [{
                        key: "on",
                        value: function(e) {
                            this.handlers.push(e)
                        }
                    }, {
                        key: "off",
                        value: function(e) {
                            this.handlers = this.handlers.filter((function(t) {
                                return t !== e
                            }))
                        }
                    }, {
                        key: "trigger",
                        value: function(e) {
                            this.handlers.slice(0).forEach((function(t) {
                                return t(e)
                            }))
                        }
                    }]), e
                }(),
                y = function() {
                    function e(t) {
                        var n;
                        d(this, e), this.mSubscribe = "subscribe", this.mUnSubscribe = "unsubscribe", this.mSetMode = "mode", this.mGetQuote = "quote", this.mAlert = 10, this.mOrderStr = "order", this.mMessage = 11, this.mMessageStr = "message", this.mLogout = 12, this.mLogoutStr = "logout", this.mReload = 13, this.mReloadStr = "reload", this.mClearCache = 14, this.mClearCacheStr = "clear_cache", this.modeLTP = "ltp", this.modeLTPC = "ltpc", this.modeFull = "full", this.modeQuote = "quote", this.modeWeights = (n = {}, h(n, this.modeFull, 1), h(n, this.modeQuote, 2), h(n, this.modeLTPC, 3), h(n, this.modeLTP, 4), n), this.weightModeMap = {
                            1: this.modeFull,
                            2: this.modeQuote,
                            3: this.modeLTPC,
                            4: this.modeLTP
                        }, this.segmentNseCM = 1, this.segmentNseFO = 2, this.segmentNseCD = 3, this.segmentBseCM = 4, this.segmentBseFO = 5, this.segmentBseCD = 6, this.segmentMcxFO = 7, this.segmentMcxSX = 8, this.segmentNseIndices = 9, this.segmentUS = 11, this.eventConnect = new p, this.eventTick = new p, this.eventData = new p, this.eventDisconnect = new p, this.eventReconnect = new p, this.eventNoReconnect = new p, this.eventAlert = new p, this.eventMessage = new p, this.eventReload = new p, this.eventClearCache = new p, this.eventLogout = new p, this.noReplyTimeout = 5, this.lazyDisconnectTimeout = 10, this.reconnectInterval = 5, this.reconnectTries = 300, this.isAutoReconnect = !0, this.reconnectionsCount = 0, this.currentWsUrl = null, this.tokenTags = {}, this.subscribedTokens = [], this.defaultTokenTag = "_", this.version = "1.0.0", this.userAgent = "kite3-web", this.quoteMap = {}, this.getQuoteTimeout = 5, this.isLazy = !1, this.isLazyInitialConnect = !1, this.lazyPayload = [], this.address = t.address, this.apiKey = t.apiKey, this.encToken = t.encToken, this.userId = t.userId, t.version && (this.version = t.version), this.debug = t.debug
                    }
                    return g(e, [{
                        key: "setParams",
                        value: function(e) {
                            this.address = e.address, this.apiKey = e.apiKey, this.encToken = e.encToken, this.userId = e.userId, this.debug = e.debug, e.version && (this.version = e.version), e.lazyDisconnectTimeout && (this.lazyDisconnectTimeout = e.lazyDisconnectTimeout)
                        }
                    }, {
                        key: "isConnected",
                        value: function() {
                            return !(!this.ws || this.ws.readyState !== this.ws.OPEN)
                        }
                    }, {
                        key: "setAutoReconnect",
                        value: function(e, t) {
                            this.isAutoReconnect = e, this.reconnectTries = t
                        }
                    }, {
                        key: "getsubscribedTokens",
                        value: function() {
                            return this.subscribedTokens
                        }
                    }, {
                        key: "lazyConnect",
                        value: function() {
                            this.isLazy = !0
                        }
                    }, {
                        key: "processLazyPayload",
                        value: function() {
                            var e = this;
                            if (this.isConnected()) {
                                var t, n = c(this.lazyPayload);
                                try {
                                    for (n.s(); !(t = n.n()).done;) {
                                        var i = t.value;
                                        this._send(i), this.lazyPayload.shift()
                                    }
                                } catch (r) {
                                    n.e(r)
                                } finally {
                                    n.f()
                                }
                            } else this.ws && this.ws.readyState === this.ws.CONNECTING && setTimeout((function() {
                                e.processLazyPayload()
                            }), 500), this.isLazyInitialConnect || (this.isAutoReconnect = !0, this.isLazyInitialConnect = !0, this.connect(), this.processLazyPayload())
                        }
                    }, {
                        key: "connect",
                        value: function(e, t) {
                            var n = this;
                            if (!this.ws || this.ws.readyState !== this.ws.CONNECTING && this.ws.readyState !== this.ws.OPEN) {
                                var i = new a.a({
                                    api_key: this.apiKey,
                                    user_id: this.userId,
                                    enctoken: this.encToken,
                                    uid: (new Date).getTime().toString(),
                                    "user-agent": this.userAgent,
                                    version: this.version
                                });
                                this.ws = new WebSocket(this.address + "?" + i.toString()), this.ws.binaryType = "arraybuffer", this.ws.onopen = function(e) {
                                    n.resubscribe(), n.eventConnect.trigger(), n.setConnectionTimer(), n.isLazy && n.setLazyDisconnect()
                                }, this.ws.onmessage = function(e) {
                                    if (n.eventData.trigger(e.data), e.data instanceof ArrayBuffer) {
                                        if (e.data.byteLength > 2) {
                                            var t = n.parseBinary(e.data);
                                            t && n.eventTick.trigger(t)
                                        }
                                    } else n.processMessage(e.data);
                                    n.lastDataReceivedTime = new Date
                                }, this.ws.onerror = function(e) {
                                    n.ws && n.ws.readyState === n.ws.OPEN && n.ws.close()
                                }, this.ws.onclose = function(e) {
                                    n.currentWsUrl && n.url !== n.currentWsUrl || n.triggerDisconnect()
                                }
                            }
                        }
                    }, {
                        key: "subscribe",
                        value: function(e, t) {
                            t = this.getTag(t);
                            var n, i = [],
                                r = c(e);
                            try {
                                for (r.s(); !(n = r.n()).done;) {
                                    var s = n.value;
                                    "number" !== typeof s || isNaN(s) || (this.isElementInArray(this.subscribedTokens, s) || (i.push(s), this.tokenTags[s] = {
                                        mode: "",
                                        tags: {}
                                    }, this.subscribedTokens.push(s)))
                                }
                            } catch (o) {
                                r.e(o)
                            } finally {
                                r.f()
                            }
                            return i.length > 0 && this.send({
                                a: this.mSubscribe,
                                v: i
                            }), i
                        }
                    }, {
                        key: "unsubscribe",
                        value: function(e, t) {
                            t = this.getTag(t);
                            var n, i = [],
                                r = c(e);
                            try {
                                for (r.s(); !(n = r.n()).done;) {
                                    var s = n.value;
                                    "number" !== typeof s || isNaN(s) || (this.deleteTokenTags(s, t), this.canUnsubscribe(s, t) && (i.push(s), this.deleteSubscriptionToken(s), delete this.tokenTags[s]))
                                }
                            } catch (o) {
                                r.e(o)
                            } finally {
                                r.f()
                            }
                            return i.length > 0 && this.send({
                                a: this.mUnSubscribe,
                                v: i
                            }), i
                        }
                    }, {
                        key: "setMode",
                        value: function(e, t, n) {
                            n = this.getTag(n);
                            var i, r = {},
                                s = c(t);
                            try {
                                for (s.s(); !(i = s.n()).done;) {
                                    var o = i.value;
                                    if (this.isElementInArray(this.subscribedTokens, o)) {
                                        if (e !== this.tokenTags[o].mode && "number" === typeof o && !isNaN(o)) {
                                            this.updateTokenTags(o, e, n);
                                            var a = this.getBestMode(o, e, n);
                                            a && a !== this.tokenTags[o].mode && (r[a] || (r[a] = []), r[a].push(o)), this.tokenTags[o].mode = a
                                        }
                                    } else this.deleteTokenTags(o, n)
                                }
                            } catch (f) {
                                s.e(f)
                            } finally {
                                s.f()
                            }
                            for (var u = 0, l = Object.keys(r); u < l.length; u++) {
                                var h = l[u];
                                this.send({
                                    a: this.mSetMode,
                                    v: [h, r[h]]
                                })
                            }
                        }
                    }, {
                        key: "resubscribe",
                        value: function() {
                            if (0 !== this.subscribedTokens.length) {
                                var e, t = {},
                                    n = [],
                                    i = c(this.subscribedTokens);
                                try {
                                    for (i.s(); !(e = i.n()).done;) {
                                        var r = e.value;
                                        "number" !== typeof r || isNaN(r) || (n.push(r), this.tokenTags[r] && this.tokenTags[r].mode && (t[this.tokenTags[r].mode] || (t[this.tokenTags[r].mode] = []), t[this.tokenTags[r].mode].push(r)))
                                    }
                                } catch (u) {
                                    i.e(u)
                                } finally {
                                    i.f()
                                }
                                this.send({
                                    a: this.mSubscribe,
                                    v: n
                                });
                                for (var s = 0, o = Object.keys(t); s < o.length; s++) {
                                    var a = o[s];
                                    this.send({
                                        a: this.mSetMode,
                                        v: [a, t[a]]
                                    })
                                }
                            }
                        }
                    }, {
                        key: "getQuote",
                        value: function(e, t, n, i) {
                            var r = this;
                            return this.quoteMap[e] = new v, i || (i = this.getQuoteTimeout), setTimeout((function() {
                                var t = r.quoteMap[e];
                                t && (t.reject(), delete r.quoteMap[e])
                            }), 1e3 * i), this.send({
                                id: e,
                                a: this.mGetQuote,
                                v: {
                                    fields: n,
                                    tokens: t
                                }
                            }), this.quoteMap[e].promise
                        }
                    }, {
                        key: "isElementInArray",
                        value: function(e, t) {
                            var n = e.filter((function(e) {
                                return e === t
                            }));
                            return n.length > 0
                        }
                    }, {
                        key: "deleteSubscriptionToken",
                        value: function(e) {
                            var t = this.subscribedTokens.indexOf(e);
                            t > -1 && this.subscribedTokens.splice(t, 1)
                        }
                    }, {
                        key: "getTag",
                        value: function(e) {
                            return e && "string" === typeof e ? e : this.defaultTokenTag
                        }
                    }, {
                        key: "updateTokenTags",
                        value: function(e, t, n) {
                            n !== this.defaultTokenTag && (this.tokenTags[e] || (this.tokenTags[e] = {
                                mode: t,
                                tags: {}
                            }), this.tokenTags[e]["tags"][n] = this.modeWeights[t])
                        }
                    }, {
                        key: "deleteTokenTags",
                        value: function(e, t) {
                            this.tokenTags[e] && this.tokenTags[e].tags && this.tokenTags[e].tags[t] && delete this.tokenTags[e].tags[t]
                        }
                    }, {
                        key: "getBestMode",
                        value: function(e, t, n) {
                            var i = this;
                            if (n === this.defaultTokenTag) return t;
                            var r = Math.min.apply(Math, Object.keys(this.tokenTags[e].tags).map((function(t) {
                                return i.tokenTags[e].tags[t]
                            })));
                            return r ? this.weightModeMap[r] : t
                        }
                    }, {
                        key: "canUnsubscribe",
                        value: function(e, t) {
                            if (!this.isElementInArray(this.subscribedTokens, e)) return !1;
                            if (t === this.defaultTokenTag) return !0;
                            if (!this.tokenTags[e]) return !0;
                            var n = Object.keys(this.tokenTags[e].tags).filter((function(e) {
                                return e !== t
                            }));
                            return !(n.length > 0)
                        }
                    }, {
                        key: "triggerDisconnect",
                        value: function() {
                            this.eventDisconnect.trigger(), this.isAutoReconnect ? this.attemptReconnection() : this.eventNoReconnect.trigger()
                        }
                    }, {
                        key: "setConnectionTimer",
                        value: function() {
                            var e = this;
                            clearInterval(this.connectionTimer), this.lastDataReceivedTime = new Date, this.connectionTimer = setInterval((function() {
                                ((new Date).getTime() - e.lastDataReceivedTime.getTime()) / 1e3 >= e.noReplyTimeout && (e.currentWsUrl = null, e.ws && e.ws.close(), clearInterval(e.connectionTimer), e.triggerDisconnect())
                            }), 1e3 * this.noReplyTimeout)
                        }
                    }, {
                        key: "setLazyDisconnect",
                        value: function() {
                            var e = this;
                            clearInterval(this.lazyDisconnectTimer), this.lazyDisconnectTimer = setInterval((function() {
                                var t = 0 === e.subscribedTokens.length;
                                t && (e.currentWsUrl = null, e.isLazyInitialConnect = !1, e.ws && e.ws.close(), clearInterval(e.lazyDisconnectTimer), e.isAutoReconnect = !1, e.triggerDisconnect())
                            }), 1e3 * this.lazyDisconnectTimeout)
                        }
                    }, {
                        key: "attemptReconnection",
                        value: function() {
                            var e = this;
                            this.reconnectionsCount > this.reconnectTries ? this.eventNoReconnect.trigger() : (this.eventReconnect.trigger(this.reconnectInterval), setTimeout((function() {
                                e.connect(!0)
                            }), 1e3 * this.reconnectInterval), this.reconnectionsCount++)
                        }
                    }, {
                        key: "_send",
                        value: function(e) {
                            try {
                                this.ws.send(JSON.stringify(e))
                            } catch (t) {
                                this.ws.close()
                            }
                        }
                    }, {
                        key: "send",
                        value: function(e) {
                            this.isConnected() ? this._send(e) : this.isLazy && (this.lazyPayload.push(e), this.processLazyPayload())
                        }
                    }, {
                        key: "dateToString",
                        value: function(e) {
                            var t = e.getFullYear().toString(),
                                n = (e.getMonth() + 1).toString(),
                                i = e.getDate().toString(),
                                r = e.getMinutes().toString(),
                                s = e.getHours().toString(),
                                o = e.getSeconds().toString();
                            n.length < 2 && (n = "0" + n), i.length < 2 && (i = "0" + i), s.length < 2 && (s = "0" + s), r.length < 2 && (r = "0" + r), o.length < 2 && (o = "0" + o);
                            var a = "".concat(t, "-").concat(n, "-").concat(i, " ").concat(s, ":").concat(r, ":").concat(o);
                            return a
                        }
                    }, {
                        key: "calculateChange",
                        value: function(e) {
                            var t = 0,
                                n = 0,
                                i = 0,
                                r = 0;
                            return e.closePrice && (n = e.lastPrice - e.closePrice, t = 100 * n / e.closePrice), e.openPrice && (i = e.lastPrice - e.openPrice, r = 100 * i / e.openPrice), {
                                change: t,
                                absoluteChange: n,
                                openChange: i,
                                openChangePercent: r
                            }
                        }
                    }, {
                        key: "parseBinary",
                        value: function(e) {
                            var t, n = this.splitPackets(e),
                                i = [],
                                r = c(n);
                            try {
                                for (r.s(); !(t = r.n()).done;) {
                                    var s = t.value,
                                        o = this.buf2long(s.slice(0, 4)),
                                        a = 255 & o,
                                        u = 100,
                                        l = void 0;
                                    switch (a === this.segmentNseCD && (u = 1e7), a === this.segmentBseCD && (u = 1e4), a) {
                                        case this.segmentMcxFO:
                                        case this.segmentNseCM:
                                        case this.segmentBseCM:
                                        case this.segmentNseFO:
                                        case this.segmentNseCD:
                                        case this.segmentBseCD:
                                        case this.segmentNseIndices:
                                        case this.segmentUS:
                                            if (8 === s.byteLength) i.push({
                                                mode: this.modeLTP,
                                                isTradeable: !0,
                                                token: o,
                                                lastPrice: this.buf2long(s.slice(4, 8)) / u
                                            });
                                            else if (12 === s.byteLength) l = {
                                                mode: this.modeLTPC,
                                                isTradeable: !0,
                                                token: o,
                                                lastPrice: this.buf2long(s.slice(4, 8)) / u,
                                                closePrice: this.buf2long(s.slice(8, 12)) / u
                                            }, l = Object.assign(l, this.calculateChange(l)), i.push(l);
                                            else if (28 === s.byteLength || 32 === s.byteLength) l = {
                                                mode: this.modeFull,
                                                isTradeable: !1,
                                                token: o,
                                                lastPrice: this.buf2long(s.slice(4, 8)) / u,
                                                highPrice: this.buf2long(s.slice(8, 12)) / u,
                                                lowPrice: this.buf2long(s.slice(12, 16)) / u,
                                                openPrice: this.buf2long(s.slice(16, 20)) / u,
                                                closePrice: this.buf2long(s.slice(20, 24)) / u
                                            }, l = Object.assign(l, this.calculateChange(l)), i.push(l);
                                            else if (492 === s.byteLength) {
                                                for (var h = {
                                                        mode: this.modeFull,
                                                        token: o,
                                                        extendedDepth: {
                                                            buy: [],
                                                            sell: []
                                                        }
                                                    }, f = 0, g = s.slice(12, 492), d = 0; d < 40; d++) f = 12 * d, h.extendedDepth[d < 20 ? "buy" : "sell"].push({
                                                    quantity: this.buf2long(g.slice(f, f + 4)),
                                                    price: this.buf2long(g.slice(f + 4, f + 8)) / u,
                                                    orders: this.buf2long(g.slice(f + 8, f + 12))
                                                });
                                                i.push(h)
                                            } else {
                                                if (l = {
                                                        mode: this.modeQuote,
                                                        token: o,
                                                        isTradeable: !0,
                                                        volume: this.buf2long(s.slice(16, 20)),
                                                        lastQuantity: this.buf2long(s.slice(8, 12)),
                                                        totalBuyQuantity: this.buf2long(s.slice(20, 24)),
                                                        totalSellQuantity: this.buf2long(s.slice(24, 28)),
                                                        lastPrice: this.buf2long(s.slice(4, 8)) / u,
                                                        averagePrice: this.buf2long(s.slice(12, 16)) / u,
                                                        openPrice: this.buf2long(s.slice(28, 32)) / u,
                                                        highPrice: this.buf2long(s.slice(32, 36)) / u,
                                                        lowPrice: this.buf2long(s.slice(36, 40)) / u,
                                                        closePrice: this.buf2long(s.slice(40, 44)) / u
                                                    }, l = Object.assign(l, this.calculateChange(l)), 164 === s.byteLength || 184 === s.byteLength) {
                                                    var v = 44;
                                                    184 === s.byteLength && (v = 64);
                                                    var p = v + 120;
                                                    if (l.mode = this.modeFull, l.depth = {
                                                            buy: [],
                                                            sell: []
                                                        }, 184 === s.byteLength) {
                                                        var y = this.buf2long(s.slice(44, 48));
                                                        l.lastTradedTime = y && y > 0 ? this.dateToString(new Date(1e3 * y)) : null, l.oi = this.buf2long(s.slice(48, 52)), l.oiDayHigh = this.buf2long(s.slice(52, 56)), l.oiDayLow = this.buf2long(s.slice(56, 60))
                                                    }
                                                    for (var m = 0, b = s.slice(v, p), k = 0; k < 10; k++) m = 12 * k, l.depth[k < 5 ? "buy" : "sell"].push({
                                                        price: this.buf2long(b.slice(m + 4, m + 8)) / u,
                                                        orders: this.buf2long(b.slice(m + 8, m + 10)),
                                                        quantity: this.buf2long(b.slice(m, m + 4))
                                                    })
                                                }
                                                i.push(l)
                                            }
                                    }
                                }
                            } catch (T) {
                                r.e(T)
                            } finally {
                                r.f()
                            }
                            return i
                        }
                    }, {
                        key: "splitPackets",
                        value: function(e) {
                            for (var t = this.buf2long(e.slice(0, 2)), n = 2, i = [], r = 0; r < t; r++) {
                                var s = this.buf2long(e.slice(n, n + 2)),
                                    o = e.slice(n + 2, n + 2 + s);
                                i.push(o), n += 2 + s
                            }
                            return i
                        }
                    }, {
                        key: "processMessage",
                        value: function(e) {
                            try {
                                var t = JSON.parse(e)
                            } catch (s) {
                                return
                            }
                            if (t.hasOwnProperty("t") || t.hasOwnProperty("type")) {
                                var n = t.t || t.type,
                                    i = t.p || t.data;
                                switch (n) {
                                    case this.mAlert:
                                    case this.mOrderStr:
                                        this.eventAlert.trigger(t);
                                        break;
                                    case this.mMessage:
                                    case this.mMessageStr:
                                        this.eventMessage.trigger(i);
                                        break;
                                    case this.mLogout:
                                    case this.mLogoutStr:
                                        this.eventLogout.trigger();
                                        break;
                                    case this.mReload:
                                    case this.mReloadStr:
                                        this.eventReload.trigger();
                                        break;
                                    case this.mClearCache:
                                    case this.mClearCacheStr:
                                        if (i) try {
                                            var r = JSON.parse(i);
                                            this.eventClearCache.trigger(r)
                                        } catch (s) {} else this.eventClearCache.trigger();
                                        break;
                                    case this.mGetQuote:
                                        this.processQuoteMessage(t.id, i);
                                        break
                                }
                            }
                        }
                    }, {
                        key: "processQuoteMessage",
                        value: function(e, t) {
                            var n = this.quoteMap[e];
                            n && (n.resolve(t), delete this.quoteMap[e])
                        }
                    }, {
                        key: "buf2long",
                        value: function(e) {
                            for (var t = new Uint8Array(e), n = 0, i = t.length, r = 0, s = i - 1; r < i; r++, s--) n += t[s] << 8 * r;
                            return n
                        }
                    }]), e
                }(),
                m = y,
                b = {
                    initial: 0,
                    fetching: 1,
                    success: 2,
                    error: -1
                },
                k = n("e675"),
                T = n.n(k),
                w = n("0e54"),
                S = n.n(w),
                P = n("add5"),
                C = n.n(P),
                M = n("0ab6"),
                O = n.n(M),
                A = n("ee7c"),
                _ = n.n(A),
                j = n("3928"),
                x = n.n(j),
                L = T.a.createStore([C.a, O.a], [_.a, x.a]).namespace("kite");
            T.a.createStore([S.a], [_.a, x.a]), T.a.createStore([O.a], [_.a, x.a]);

            function E(e, t, n, i, r) {
                var s = "";
                i = i || L, e && (s = e + "/"), s += t;
                try {
                    return i.set(s, n, r), !0
                } catch (o) {
                    return console.log("localstorage error set: ", o), !1
                }
            }

            function D(e, t) {
                var n;
                if ("undefined" === typeof Symbol || null == e[Symbol.iterator]) {
                    if (Array.isArray(e) || (n = I(e)) || t && e && "number" === typeof e.length) {
                        n && (e = n);
                        var i = 0,
                            r = function() {};
                        return {
                            s: r,
                            n: function() {
                                return i >= e.length ? {
                                    done: !0
                                } : {
                                    done: !1,
                                    value: e[i++]
                                }
                            },
                            e: function(e) {
                                throw e
                            },
                            f: r
                        }
                    }
                    throw new TypeError("Invalid attempt to iterate non-iterable instance.\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.")
                }
                var s, o = !0,
                    a = !1;
                return {
                    s: function() {
                        n = e[Symbol.iterator]()
                    },
                    n: function() {
                        var e = n.next();
                        return o = e.done, e
                    },
                    e: function(e) {
                        a = !0, s = e
                    },
                    f: function() {
                        try {
                            o || null == n.return || n.return()
                        } finally {
                            if (a) throw s
                        }
                    }
                }
            }

            function I(e, t) {
                if (e) {
                    if ("string" === typeof e) return N(e, t);
                    var n = Object.prototype.toString.call(e).slice(8, -1);
                    return "Object" === n && e.constructor && (n = e.constructor.name), "Map" === n || "Set" === n ? Array.from(e) : "Arguments" === n || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n) ? N(e, t) : void 0
                }
            }

            function N(e, t) {
                (null == t || t > e.length) && (t = e.length);
                for (var n = 0, i = new Array(t); n < t; n++) i[n] = e[n];
                return i
            }

            function R(e, t, n) {
                return t in e ? Object.defineProperty(e, t, {
                    value: n,
                    enumerable: !0,
                    configurable: !0,
                    writable: !0
                }) : e[t] = n, e
            }
            var z = !0,
                F = "ticker";

            function U(e, t) {
                var n = e.change || 0,
                    i = e.absoluteChange || 0;
                t.closePrice && (i = e.lastPrice - t.closePrice, n = 100 * i / t.closePrice);
                var r = 0;
                return n && 0 !== n && (r = n > 0 ? 1 : -1), Object.assign({}, t, e, {
                    change: n,
                    absoluteChange: i,
                    tickChange: r
                })
            }
            var Q = {
                    ticks: {},
                    tickerConnectionStatus: b.initial
                },
                B = {
                    ticks: function(e) {
                        return e.ticks
                    },
                    tickerConnectionStatus: function(e) {
                        return e.tickerConnectionStatus
                    }
                },
                $ = {
                    setTick: function(e, t) {
                        var n = t,
                            i = e.ticks[t.token] || {};
                        n = U(t, i), e.ticks = Object.assign({}, e.ticks, R({}, t.token, n)), E(F, "ticks", e.ticks)
                    },
                    setTicks: function(e, t) {
                        var n, i = {},
                            r = D(t);
                        try {
                            for (r.s(); !(n = r.n()).done;) {
                                var s = n.value,
                                    o = s;
                                if (e.ticks[s.token]) {
                                    var a = e.ticks[s.token] || {};
                                    o = U(s, a)
                                }
                                i[s.token] = o
                            }
                        } catch (c) {
                            r.e(c)
                        } finally {
                            r.f()
                        }
                        e.ticks = Object.assign({}, e.ticks, i), E(F, "ticks", e.ticks)
                    },
                    setExtendedTicks: function(e, t) {
                        var n, i = {},
                            r = D(t);
                        try {
                            for (r.s(); !(n = r.n()).done;) {
                                var s = n.value,
                                    o = s;
                                if (e.ticks[s.token]) {
                                    var a = e.ticks[s.token] || {};
                                    o = Object.assign({}, a, s)
                                }
                                i[s.token] = o
                            }
                        } catch (c) {
                            r.e(c)
                        } finally {
                            r.f()
                        }
                        e.ticks = Object.assign({}, e.ticks, i), E(F, "ticks", e.ticks)
                    },
                    dumpTicks: function(e, t) {
                        e.ticks = t, E(F, "ticks", e.ticks)
                    },
                    setTickerConnectionStatus: function(e, t) {
                        e.tickerConnectionStatus = t
                    }
                },
                q = {},
                G = {
                    state: Q,
                    getters: B,
                    mutations: $,
                    actions: q,
                    namespaced: z
                };

            function W(e, t) {
                if (!(e instanceof t)) throw new TypeError("Cannot call a class as a function")
            }
            var J = function e() {
                W(this, e)
            };
            J.namespace = F, J.Provider = m, J.install = function(e, t) {
                t.store && t.store.registerModule(this.namespace, G)
            }, self.ticker = null, self.addEventListener("connect", (function(e) {
                var t = e.ports[0];
                t.addEventListener("message", (function(e) {
                    if (e.data && e.data.type && e.data.type.startsWith("ticker.")) {
                        var n = e.data.data;
                        if ("ticker.connect" === e.data.type) self.connectTicker(t, n);
                        else if ("ticker.lazy_connect" === e.data.type) self.connectTicker(t, n, !0);
                        else {
                            if (!self.ticker) return;
                            "ticker.subscribe" === e.data.type ? self.ticker.subscribe(n.tokens, n.tag) : "ticker.unsubscribe" === e.data.type ? self.ticker.unsubscribe(n.tokens, n.tag) : "ticker.setMode" === e.data.type ? self.ticker.setMode(n.mode, n.tokens, n.tag) : "ticker.getQuote" === e.data.type && self.ticker.getQuote(n.id, n.tokens, n.fields, n.timeout).then((function(n) {
                                self.sendMessageToAll(t, "ticker.getQuote", n, e.data.id)
                            }))
                        }
                    }
                })), t.start()
            })), self.sendHeartbeat = function(e) {
                setInterval((function() {
                    self.sendMessageToAll(e, "heartbeat")
                }), 1e3)
            }, self.sendMessageToAll = function(e, t, n, i) {
                e.postMessage({
                    id: i,
                    type: t,
                    data: n
                })
            }, self.connectTicker = function(e, t, n) {
                var i = !1;
                self.ticker || (self.ticker && delete self.ticker, self.ticker = new J.Provider({
                    address: t.address,
                    apiKey: t.apiKey,
                    userId: t.userId,
                    encToken: t.encToken,
                    version: t.version
                }), i = !0), self.ticker.eventTick.on((function(t) {
                    self.sendMessageToAll(e, "ticker.tick", t)
                })), self.ticker.eventData.on((function(t) {
                    self.sendMessageToAll(e, "ticker.data", t)
                })), self.ticker.eventReconnect.on((function(t) {
                    self.sendMessageToAll(e, "ticker.reconnect", t)
                })), self.ticker.eventConnect.on((function(t) {
                    self.sendMessageToAll(e, "ticker.connect", t)
                })), self.ticker.eventNoReconnect.on((function(t) {
                    self.sendMessageToAll(e, "ticker.noreconnect", t)
                })), self.ticker.eventAlert.on((function(t) {
                    self.sendMessageToAll(e, "ticker.alert", t)
                })), self.ticker.eventReload.on((function(t) {
                    self.sendMessageToAll(e, "ticker.reload", t)
                })), self.ticker.eventClearCache.on((function(t) {
                    self.sendMessageToAll(e, "ticker.clearcache", t)
                })), self.ticker.eventLogout.on((function(t) {
                    self.sendMessageToAll(e, "ticker.logout", t)
                })), self.ticker.eventMessage.on((function(t) {
                    self.sendMessageToAll(e, "ticker.message", t)
                })), self.ticker.eventDisconnect.on((function(t) {
                    self.sendMessageToAll(e, "ticker.disconnect", t)
                })), i && (n ? self.ticker.lazyConnect() : self.ticker.connect())
            }
        }
    })
}));
