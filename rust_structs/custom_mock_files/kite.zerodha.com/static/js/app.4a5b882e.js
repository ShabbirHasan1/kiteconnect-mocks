(function(e) {
    function t(t) {
        for (var a, n, o = t[0], c = t[1], l = t[2], d = 0, u = []; d < o.length; d++) n = o[d], Object.prototype.hasOwnProperty.call(r, n) && r[n] && u.push(r[n][0]), r[n] = 0;
        for (a in c) Object.prototype.hasOwnProperty.call(c, a) && (e[a] = c[a]);
        h && h(t);
        while (u.length) u.shift()();
        return s.push.apply(s, l || []), i()
    }

    function i() {
        for (var e, t = 0; t < s.length; t++) {
            for (var i = s[t], a = !0, n = 1; n < i.length; n++) {
                var o = i[n];
                0 !== r[o] && (a = !1)
            }
            a && (s.splice(t--, 1), e = c(c.s = i[0]))
        }
        return e
    }
    var a = {},
        n = {
            app: 0
        },
        r = {
            app: 0
        },
        s = [];

    function o(e) {
        return c.p + "static/js/" + ({
            "chartiq~connect~main~main-chartiq~main-tradingview~tradingview": "chartiq~connect~main~main-chartiq~main-tradingview~tradingview",
            "chartiq~connect~main~main-chartiq~tradingview": "chartiq~connect~main~main-chartiq~tradingview",
            "chartiq~connect~main~tradingview": "chartiq~connect~main~tradingview",
            connect: "connect",
            "chartiq~main~main-chartiq~outer~tradingview": "chartiq~main~main-chartiq~outer~tradingview",
            chartiq: "chartiq",
            main: "main",
            tradingview: "tradingview",
            "main-chartiq": "main-chartiq",
            "main-tradingview": "main-tradingview",
            outer: "outer",
            "holdings-auth": "holdings-auth"
        }[e] || e) + "." + {
            "chartiq~connect~main~main-chartiq~main-tradingview~tradingview": "917aed52",
            "chartiq~connect~main~main-chartiq~tradingview": "ccf7a4f3",
            "chartiq~connect~main~tradingview": "818bafa5",
            connect: "c986c30a",
            "chartiq~main~main-chartiq~outer~tradingview": "4d6aa12a",
            chartiq: "1ab52481",
            main: "7d89fc0f",
            tradingview: "feec5532",
            "main-chartiq": "0487907e",
            "main-tradingview": "89fc5217",
            outer: "5efb959a",
            "holdings-auth": "c1e84ece",
            "chunk-2d22c101": "cb0b4346"
        }[e] + ".js"
    }

    function c(t) {
        if (a[t]) return a[t].exports;
        var i = a[t] = {
            i: t,
            l: !1,
            exports: {}
        };
        return e[t].call(i.exports, i, i.exports, c), i.l = !0, i.exports
    }
    c.e = function(e) {
        var t = [],
            i = {
                chartiq: 1,
                main: 1,
                tradingview: 1,
                "main-tradingview": 1,
                outer: 1
            };
        n[e] ? t.push(n[e]) : 0 !== n[e] && i[e] && t.push(n[e] = new Promise((function(t, i) {
            for (var a = "static/css/" + ({
                    "chartiq~connect~main~main-chartiq~main-tradingview~tradingview": "chartiq~connect~main~main-chartiq~main-tradingview~tradingview",
                    "chartiq~connect~main~main-chartiq~tradingview": "chartiq~connect~main~main-chartiq~tradingview",
                    "chartiq~connect~main~tradingview": "chartiq~connect~main~tradingview",
                    connect: "connect",
                    "chartiq~main~main-chartiq~outer~tradingview": "chartiq~main~main-chartiq~outer~tradingview",
                    chartiq: "chartiq",
                    main: "main",
                    tradingview: "tradingview",
                    "main-chartiq": "main-chartiq",
                    "main-tradingview": "main-tradingview",
                    outer: "outer",
                    "holdings-auth": "holdings-auth"
                }[e] || e) + "." + {
                    "chartiq~connect~main~main-chartiq~main-tradingview~tradingview": "31d6cfe0",
                    "chartiq~connect~main~main-chartiq~tradingview": "31d6cfe0",
                    "chartiq~connect~main~tradingview": "31d6cfe0",
                    connect: "31d6cfe0",
                    "chartiq~main~main-chartiq~outer~tradingview": "31d6cfe0",
                    chartiq: "f8030126",
                    main: "9ffc3366",
                    tradingview: "f8030126",
                    "main-chartiq": "31d6cfe0",
                    "main-tradingview": "2d680389",
                    outer: "7abae0d6",
                    "holdings-auth": "31d6cfe0",
                    "chunk-2d22c101": "31d6cfe0"
                }[e] + ".css", r = c.p + a, s = document.getElementsByTagName("link"), o = 0; o < s.length; o++) {
                var l = s[o],
                    d = l.getAttribute("data-href") || l.getAttribute("href");
                if ("stylesheet" === l.rel && (d === a || d === r)) return t()
            }
            var u = document.getElementsByTagName("style");
            for (o = 0; o < u.length; o++) {
                l = u[o], d = l.getAttribute("data-href");
                if (d === a || d === r) return t()
            }
            var h = document.createElement("link");
            h.rel = "stylesheet", h.type = "text/css", h.onload = t, h.onerror = function(t) {
                var a = t && t.target && t.target.src || r,
                    s = new Error("Loading CSS chunk " + e + " failed.\n(" + a + ")");
                s.code = "CSS_CHUNK_LOAD_FAILED", s.request = a, delete n[e], h.parentNode.removeChild(h), i(s)
            }, h.href = r;
            var m = document.getElementsByTagName("head")[0];
            m.appendChild(h)
        })).then((function() {
            n[e] = 0
        })));
        var a = r[e];
        if (0 !== a)
            if (a) t.push(a[2]);
            else {
                var s = new Promise((function(t, i) {
                    a = r[e] = [t, i]
                }));
                t.push(a[2] = s);
                var l, d = document.createElement("script");
                d.charset = "utf-8", d.timeout = 120, c.nc && d.setAttribute("nonce", c.nc), d.src = o(e);
                var u = new Error;
                l = function(t) {
                    d.onerror = d.onload = null, clearTimeout(h);
                    var i = r[e];
                    if (0 !== i) {
                        if (i) {
                            var a = t && ("load" === t.type ? "missing" : t.type),
                                n = t && t.target && t.target.src;
                            u.message = "Loading chunk " + e + " failed.\n(" + a + ": " + n + ")", u.name = "ChunkLoadError", u.type = a, u.request = n, i[1](u)
                        }
                        r[e] = void 0
                    }
                };
                var h = setTimeout((function() {
                    l({
                        type: "timeout",
                        target: d
                    })
                }), 12e4);
                d.onerror = d.onload = l, document.head.appendChild(d)
            }
        return Promise.all(t)
    }, c.m = e, c.c = a, c.d = function(e, t, i) {
        c.o(e, t) || Object.defineProperty(e, t, {
            enumerable: !0,
            get: i
        })
    }, c.r = function(e) {
        "undefined" !== typeof Symbol && Symbol.toStringTag && Object.defineProperty(e, Symbol.toStringTag, {
            value: "Module"
        }), Object.defineProperty(e, "__esModule", {
            value: !0
        })
    }, c.t = function(e, t) {
        if (1 & t && (e = c(e)), 8 & t) return e;
        if (4 & t && "object" === typeof e && e && e.__esModule) return e;
        var i = Object.create(null);
        if (c.r(i), Object.defineProperty(i, "default", {
                enumerable: !0,
                value: e
            }), 2 & t && "string" != typeof e)
            for (var a in e) c.d(i, a, function(t) {
                return e[t]
            }.bind(null, a));
        return i
    }, c.n = function(e) {
        var t = e && e.__esModule ? function() {
            return e["default"]
        } : function() {
            return e
        };
        return c.d(t, "a", t), t
    }, c.o = function(e, t) {
        return Object.prototype.hasOwnProperty.call(e, t)
    }, c.p = "/", c.oe = function(e) {
        throw console.error(e), e
    };
    var l = window["webpackJsonp"] = window["webpackJsonp"] || [],
        d = l.push.bind(l);
    l.push = t, l = l.slice();
    for (var u = 0; u < l.length; u++) t(l[u]);
    var h = d;
    s.push([0, "chunk-vendors"]), i()
})({
    0: function(e, t, i) {
        e.exports = i("56d7")
    },
    "025e": function(e, t, i) {
        "use strict";
        i.d(t, "k", (function() {
            return d
        })), i.d(t, "o", (function() {
            return h
        })), i.d(t, "n", (function() {
            return u
        })), i.d(t, "g", (function() {
            return c
        })), i.d(t, "h", (function() {
            return l
        })), i.d(t, "e", (function() {
            return m
        })), i.d(t, "b", (function() {
            return p
        })), i.d(t, "f", (function() {
            return f
        })), i.d(t, "a", (function() {
            return g
        })), i.d(t, "c", (function() {
            return v
        })), i.d(t, "l", (function() {
            return b
        })), i.d(t, "i", (function() {
            return w
        })), i.d(t, "j", (function() {
            return k
        })), i.d(t, "m", (function() {
            return y
        })), i.d(t, "d", (function() {
            return _
        }));
        var a = i("bc3a"),
            n = i.n(a),
            r = i("0b16"),
            s = i.n(r),
            o = i("5fb0");

        function c({
            commit: e,
            apiPromise: t,
            data: i,
            status: a,
            error: r,
            successStatusFlag: s,
            errorStatusFlag: c
        }) {
            t.then(t => {
                e(a, s || o["a"].success), t && t.data && t.data.data && e(i, t.data.data)
            }).catch(t => {
                n.a.isCancel(t) && "cancel-duplicate-requests" === t.message || (e(a, c || o["a"].error), r && e(r, l(t)))
            })
        }

        function l(e) {
            let t = {
                status: "error",
                message: "Unknown error",
                error_type: "GeneralException",
                data: null,
                status_code: null
            };
            return e.response ? (e.response.data ? e.response.data.error_type ? t = e.response.data : e.response.data.message && (t.message = e.response.data.message) : (t.error_type = "NetworkException", t.message = e.response.statusText), t.status_code = e.response.status) : e.request ? (t.error_type = "NetworkException", t.message = "No response from server. Check if you are still connected to internet.") : e.message ? (t.error_type = "UnknownException", t.message = e.message) : (t.error_type = "ClientNetworkException", t.message = "Error while setting up the request"), t
        }

        function d(e, t, i) {
            let a = e.slice();
            return a.sort((e, i) => (e = parseInt(m(e, t)), i = parseInt(m(i, t)), e - i)), i ? a.reverse() : a
        }

        function u(e, t, i) {
            i = i || window;
            let a = !1,
                n = function() {
                    a || (a = !0, window.requestAnimationFrame((function() {
                        i.dispatchEvent(new window.CustomEvent(t)), a = !1
                    })))
                };
            i.addEventListener(e, n)
        }

        function h() {
            let e = "inner",
                t = window;
            return "innerWidth" in window || (e = "client", t = document.documentElement || document.body), {
                width: t[e + "Width"],
                height: t[e + "Height"]
            }
        }

        function m(e, t, i) {
            let a = t ? t.split(".") : [];
            while (a.length && e) {
                let t = a.shift(),
                    n = new RegExp("(.+)\\[([0-9]*)\\]").exec(t);
                if (null === n || 3 !== n.length) "undefined" !== typeof i && (void 0 === e[t] && (e[t] = {}), 0 === a.length && (e[t] = i)), e = e[t];
                else {
                    let t = {
                        arrName: n[1],
                        arrIndex: n[2]
                    };
                    void 0 !== e[t.arrName] ? ("undefined" !== typeof i && 0 === a.length && (e[t.arrName][t.arrIndex] = i), e = e[t.arrName][t.arrIndex]) : e = void 0
                }
            }
            return e
        }

        function p(e, t) {
            let i = s.a.parse(e, !0, !1);
            for (let a in t) i.query[a.toString()] = t[a];
            return delete i.search, s.a.format(i)
        }

        function f() {
            function e() {
                return Math.floor(65536 * (1 + Math.random())).toString(16).substring(1)
            }
            return e() + e() + "-" + e() + "-" + e() + "-" + e() + "-" + e() + e() + e()
        }
        class g {
            constructor(e, t) {
                this.interval = t || 1e3, this.sound = new window.Audio, this.running = !1, this.sound.src = e + "." + (this.sound.canPlayType("audio/mpeg") ? "mp3" : "ogg")
            }
            play() {
                this.sound.pause(), this.sound.load(), this.sound.play()
            }
            beep() {
                this.running || (this.running = !0, this.play(), setTimeout(() => {
                    this.running = !1
                }, this.interval))
            }
        }

        function v(e, t, i) {
            e.class_timer && clearTimeout(e.class_timer), e.classList.add(t), e.class_timer = setTimeout((function() {
                e.classList.remove(t)
            }), i || 175)
        }

        function b(e, t, i) {
            let a = new Promise((t, a) => {
                let n = setTimeout(() => {
                    clearTimeout(n), a(i || new Error("Promise timed out"))
                }, e)
            });
            return Promise.race([t, a])
        }

        function w(e) {
            return e.replace(/\[(.+?)\]\((https?:\/\/.+?)\)/gi, "<a href='$2' target='_blank'>$1</a>")
        }

        function k() {
            let e = new Date;
            e.getHours() > 5 && e.setTime(e.getTime() + 864e5);
            let t = new Date(e.getFullYear(), e.getMonth(), e.getDate(), 6, 0, 0, 0);
            return t.getTime()
        }

        function y(e) {
            return e.replace(/[^\w. ]/gi, (function(e) {
                return "&#" + e.charCodeAt(0) + ";"
            }))
        }

        function _(e, t) {
            let i = e.split(" ")[0],
                a = t.split(" ")[0],
                n = (new Date).getFullYear().toString(),
                r = i.split("-"),
                s = a.split("-"),
                o = parseInt(r[2]).toString() + C(parseInt(r[2]));
            parseInt(r[1]) !== parseInt(s[1]) && (o += " " + q(parseInt(r[1]))), parseInt(r[0]) !== parseInt(s[0]) && (o += " " + r[0]);
            let c = parseInt(s[2]).toString() + C(parseInt(s[2])) + " " + q(parseInt(s[1]));
            return s[0] === n && parseInt(r[0]) === parseInt(s[0]) || (c += " " + s[0]), [o, c]
        }

        function C(e) {
            if (e > 3 && e < 21) return "th";
            switch (e % 10) {
                case 1:
                    return "st";
                case 2:
                    return "nd";
                case 3:
                    return "rd";
                default:
                    return "th"
            }
        }

        function q(e) {
            switch (e) {
                case 1:
                    return "Jan";
                case 2:
                    return "Feb";
                case 3:
                    return "Mar";
                case 4:
                    return "Apr";
                case 5:
                    return "May";
                case 6:
                    return "Jun";
                case 7:
                    return "Jul";
                case 8:
                    return "Aug";
                case 9:
                    return "Sep";
                case 10:
                    return "Oct";
                case 11:
                    return "Nov";
                case 12:
                    return "Dec"
            }
        }
    },
    "0a3b": function(e, t, i) {
        "use strict";
        i.d(t, "a", (function() {
            return n
        }));
        const a = RegExp(/([0-9]+)(th)[ ]|([0-9]+)(rd)[ ]|([0-9]+)(st)[ ]|([0-9]+)(nd)[ ]/i);
        let n = e => e.replace(a, '$1$3$5$7<sup>$2$4$6$8 <span class="weekly">w</span></sup> ');
        t["b"] = {
            dateSuperScript: n
        }
    },
    "41cb": function(e, t, i) {
        "use strict";
        var a = i("8c4f"),
            n = i("5665"),
            r = i("025e"),
            s = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("div", {
                    staticClass: "not-found-container"
                }, [i("a", {
                    staticClass: "logo-wrapper",
                    attrs: {
                        href: "/"
                    }
                }, [i("img", {
                    staticClass: "logo",
                    attrs: {
                        src: e.dashboardLogo,
                        alt: "Kite logo"
                    }
                })]), e._v(" "), i("h2", {
                    staticClass: "title"
                }, [e._v("Page not found")]), e._v(" "), i("div", {
                    staticClass: "description"
                }, [e._v("The page you are looking for does not exist.")])])
            },
            o = [],
            c = i("db49"),
            l = {
                name: "not-found",
                data: function() {
                    return {
                        dashboardLogo: c["b"].logo
                    }
                }
            },
            d = l,
            u = (i("b86f"), i("2877")),
            h = Object(u["a"])(d, s, o, !1, null, null, null),
            m = h.exports;
        const p = () => Promise.all([i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("outer")]).then(i.bind(null, "75d7")),
            f = () => Promise.all([i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("outer")]).then(i.bind(null, "31bd")),
            g = () => Promise.all([i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("outer")]).then(i.bind(null, "a8ba")),
            v = () => Promise.all([i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("outer")]).then(i.bind(null, "1b97")),
            b = () => Promise.all([i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("outer")]).then(i.bind(null, "84d3")),
            w = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("connect")]).then(i.bind(null, "fb7c")),
            k = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("connect")]).then(i.bind(null, "6845")),
            y = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("main")]).then(i.bind(null, "3dfd")),
            _ = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("main")]).then(i.bind(null, "d627")),
            C = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("main")]).then(i.bind(null, "bfdc")),
            q = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("main")]).then(i.bind(null, "9806")),
            S = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("main")]).then(i.bind(null, "2a46")),
            E = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("main")]).then(i.bind(null, "04d3")),
            O = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("main")]).then(i.bind(null, "cb66")),
            x = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("main")]).then(i.bind(null, "eaf6")),
            T = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("main")]).then(i.bind(null, "c6e1")),
            $ = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("main")]).then(i.bind(null, "8e6f")),
            P = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("main")]).then(i.bind(null, "033e")),
            I = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("main")]).then(i.bind(null, "7c2a")),
            D = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("main")]).then(i.bind(null, "6f52")),
            L = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("main")]).then(i.bind(null, "0721")),
            B = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("main")]).then(i.bind(null, "2da9")),
            A = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("main-chartiq")]).then(i.bind(null, "a034")),
            N = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("main-chartiq")]).then(i.bind(null, "98c7")),
            R = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("main-tradingview")]).then(i.bind(null, "f930")),
            j = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("chartiq")]).then(i.bind(null, "60a2")),
            F = () => Promise.all([i.e("chartiq~connect~main~main-chartiq~main-tradingview~tradingview"), i.e("chartiq~connect~main~main-chartiq~tradingview"), i.e("chartiq~main~main-chartiq~outer~tradingview"), i.e("chartiq~connect~main~tradingview"), i.e("tradingview")]).then(i.bind(null, "5e9f")),
            V = () => i.e("holdings-auth").then(i.bind(null, "d4bd")),
            M = [{
                path: "/",
                component: p,
                children: [{
                    path: n["b"].login,
                    name: "login",
                    component: f
                }, {
                    path: n["b"].connectLogin,
                    name: "connect-login",
                    component: f
                }, {
                    path: n["b"].forgotPassword,
                    name: "forgot-password",
                    component: g,
                    meta: {
                        title: "Forgot password"
                    }
                }, {
                    path: n["b"].resetPassword,
                    name: "reset-password",
                    component: v,
                    meta: {
                        title: "Reset password"
                    }
                }, {
                    path: n["b"].connectAuthorize,
                    name: "connect-authorize",
                    component: w,
                    meta: {
                        title: "Authorize Kite"
                    }
                }, {
                    path: n["b"].connectBasket,
                    name: "connect-basket",
                    component: k,
                    meta: {
                        title: "Kite order basket"
                    }
                }, {
                    path: n["b"].connectLoginPopup,
                    name: "connect-popup",
                    component: b,
                    meta: {
                        title: "Login complete"
                    }
                }]
            }, {
                path: "/",
                component: y,
                children: [{
                    path: n["b"].orders,
                    name: "orders",
                    component: C,
                    meta: {
                        title: "Orders"
                    }
                }, {
                    path: n["b"].gtt,
                    name: "gtt",
                    component: q,
                    meta: {
                        title: "GTT"
                    }
                }, {
                    path: n["b"].baskets,
                    name: "baskets",
                    component: S,
                    meta: {
                        title: "Baskets"
                    }
                }, {
                    path: n["b"].sip,
                    name: "sip",
                    component: E,
                    meta: {
                        title: "SIP"
                    }
                }, {
                    path: n["b"].alerts,
                    name: "alerts",
                    component: O,
                    meta: {
                        title: "Alerts"
                    }
                }, {
                    path: n["b"].ipo,
                    name: "ipo",
                    component: x,
                    meta: {
                        title: "IPO"
                    }
                }, {
                    path: n["b"].margins,
                    name: "margins",
                    component: T,
                    meta: {
                        title: "Funds"
                    }
                }, {
                    path: n["b"].apps,
                    name: "apps",
                    component: B,
                    meta: {
                        title: "Apps"
                    }
                }, {
                    path: n["b"].profile,
                    name: "profile",
                    component: _,
                    meta: {
                        title: "Profile"
                    }
                }, {
                    path: n["b"].holdings,
                    name: "holdings",
                    component: $,
                    meta: {
                        title: "Holdings"
                    }
                }, {
                    path: n["b"].chart,
                    name: "chart",
                    component: A,
                    meta: {
                        title: "Chart"
                    },
                    props: !0
                }, {
                    path: n["b"].tvchart,
                    name: "tvchart",
                    component: R,
                    meta: {
                        title: "TVChart"
                    },
                    props: !0
                }, {
                    path: n["b"].positions,
                    name: "positions",
                    component: I,
                    meta: {
                        title: "Positions"
                    }
                }, {
                    path: n["b"].dashboard,
                    name: "dashboard",
                    component: D,
                    meta: {
                        title: "Dashboard"
                    }
                }, {
                    path: n["b"].marketwatch,
                    name: "marketwatch",
                    component: L,
                    meta: {
                        title: "Marketwatch"
                    }
                }, {
                    path: n["b"].editProfile,
                    name: "editProfile",
                    component: P,
                    meta: {
                        title: "Edit profile"
                    }
                }]
            }, {
                path: "/chart/ext/ciq",
                props: !0,
                component: j,
                children: [{
                    path: n["b"].extChart,
                    name: "externalChart",
                    component: A,
                    meta: {
                        title: "Chart"
                    },
                    props: !0
                }]
            }, {
                path: "/multiplechart/ext/ciq",
                props: !0,
                component: j,
                children: [{
                    path: n["b"].multipleChart,
                    name: "multipleChart",
                    component: N,
                    meta: {
                        title: "MultipleChart"
                    },
                    props: !0
                }]
            }, {
                path: "/chart/ext/tvc",
                props: !0,
                component: F,
                children: [{
                    path: n["b"].extTvchart,
                    name: "externalTvChart",
                    component: R,
                    meta: {
                        title: "TVChart"
                    },
                    props: !0
                }]
            }, {
                path: "/",
                props: !0,
                component: p,
                children: [{
                    path: n["b"].holdingsAuthorise,
                    name: "holdings-authorise",
                    component: V,
                    meta: {
                        title: "CDSL holdings Authorization"
                    },
                    props: !0
                }]
            }, {
                path: "/margins",
                redirect: n["b"].margins
            }, {
                path: "*",
                component: m
            }],
            U = new a["a"]({
                mode: "history",
                routes: M,
                scrollBehavior(e, t, i) {
                    if (e.name && ("chart" === e.name || "tvchart" === e.name)) return {
                        x: 0,
                        y: 0
                    }
                }
            });
        let z = "Kite - Zerodha's fast and elegant flagship trading platform";
        U.beforeEach((e, t, i) => {
            if ("marketwatch" === e.name) {
                let e = Object(r["o"])();
                e.width >= 1024 && U.push(n["b"].dashboard)
            }
            document.title = e.meta.title ? e.meta.title + " / Kite" : z, i()
        });
        t["a"] = U
    },
    5665: function(e, t, i) {
        "use strict";
        i.d(t, "b", (function() {
            return s
        })), i.d(t, "a", (function() {
            return o
        }));
        var a = i("e165"),
            n = i.n(a);
        const r = {
                "marketwatches.all": "/api/marketwatch",
                "marketwatches.info": "/api/marketwatch/{watchId}",
                "marketwatches.add": "/api/marketwatch/{watchId}/items",
                "marketwatches.update": "/api/marketwatch/{watchId}/items",
                "marketwatches.delete": "/api/marketwatch/{watchId}/{itemId}",
                "user.session": "/api/session",
                "user.appSessions": "/api/user/app_sessions",
                "user.login": "/api/login",
                "user.twofa": "/api/twofa",
                "user.initiatePasswordReset": "/api/login_reset",
                "user.validatePasswordReset": "/api/login_reset/validate",
                "user.doPasswordReset": "/api/login_reset",
                "user.changePassword": "/api/login",
                "user.changeTwoFA": "/api/twofa",
                "user.twofaValidate": "/api/validate_twofa",
                "user.totp": "/api/totp",
                "user.deleteAvatar": "/api/user/avatar",
                "user.margins.all": "/oms/user/margins",
                "user.margins.info": "/oms/user/margins/{segment}",
                "user.profile": "/oms/user/profile/full",
                "user.vpa": "/oms/user/profile/vpa",
                "user.vpa.validate": "/oms/user/profile/vpa/validate",
                "notifications.all": "/api/alerts",
                "notifications.update": "/api/alerts",
                "notifications.delete.all": "/api/alerts",
                "notifications.delete": "/api/alerts/{id}",
                "apps.connected": "/api/apps/connected",
                "apps.delete": "/api/apps/connected/{appId}",
                "orders.all": "/oms/orders",
                "orders.info": "/oms/orders/{orderId}",
                "orders.place": "/oms/orders/{variety}",
                "orders.modify": "/oms/orders/{variety}/{orderId}",
                "orders.cancel": "/oms/orders/{variety}/{orderId}",
                "orders.trades": "/oms/orders/{orderId}/trades",
                "orders.triggerRange": "/oms/instruments/trigger_range/{transactionType}",
                "trades.all": "/oms/trades",
                "gtt_orders.all": "/oms/gtt/triggers",
                "gtt_orders.info": "/oms/gtt/triggers/{orderId}",
                "gtt_orders.place": "/oms/gtt/triggers",
                "gtt_orders.modify": "/oms/gtt/triggers/{orderId}",
                "gtt_orders.cancel": "/oms/gtt/triggers/{orderId}",
                "portfolio.holdings.all": "/oms/portfolio/holdings",
                "portfolio.positions.all": "/oms/portfolio/positions",
                "portfolio.positions.modify": "/oms/portfolio/positions",
                "portfolio.holdings.app": "/api/portfolio/holdings/{appName}",
                "portfolio.holdings.createAuthorise": "/oms/portfolio/holdings/authorise",
                "portfolio.holdings.authorise": "/api/portfolio/authorise/holdings/{apiKey}/{reqID}",
                "portfolio.holdings.sendPin": "/api/portfolio/authorise/holdings/{apiKey}/{reqID}/pin",
                "market.quote": "/api/instruments/{exchange}/{tradingsymbol}",
                "market.pinnedChart": "/api/market-overview",
                "connect.connectedApps.all": "/api/apps/connected_apps",
                "connect.revokeAppAccess": "/api/apps/connected_apps/{apiKey}",
                "connect.app.authorize": "/api/connect/app/authorize",
                "connect.app.session": "/api/connect/session",
                "connect.basket.orders.place": "/api/connect/basket/orders/{variety}",
                "notice.all": "/api/notice",
                "preferences.all": "/api/preferences",
                "preferences.update": "/api/preferences",
                "preferences.chart": "/api/chart/preferences",
                "captcha.get": "/api/captcha",
                banner: "/api/banner",
                "nudge.orders": "/oms/nudge/orders",
                "otp.initiate": "/api/otp",
                "otp.validate": "/api/otp/{reqID}",
                "baskets.all": "/api/baskets",
                "baskets.one": "/api/baskets/{basketID}",
                "baskets.create": "/api/baskets",
                "baskets.modify": "/api/baskets/{basketID}",
                "baskets.delete": "/api/baskets/{basketID}",
                "baskets.items.modifyWeights": "/api/baskets/{basketID}/items",
                "baskets.items.add": "/api/baskets/{basketID}/items",
                "baskets.items.modify": "/api/baskets/{basketID}/items/{itemID}",
                "baskets.items.delete": "/api/baskets/{basketID}/items/{itemID}",
                "sip.all": "/api/sip",
                "sip.one": "/api/sip/{sipID}",
                "sip.create": "/api/sip",
                "sip.update": "/api/sip/{sipID}",
                "sip.update_status": "/api/sip/{sipID}/status",
                "sip.delete": "/api/sip/{sipID}",
                "sip.add_item": "/api/sip/{sipID}/items",
                "sip.delete_item": "/api/sip/{sipID}/items/{itemID}",
                "alerts.all": "/oms/alerts",
                "alerts.one": "/oms/alerts/{uuid}",
                "alerts.history": "/oms/alerts/{uuid}/history",
                "alerts.create": "/oms/alerts",
                "alerts.update": "/oms/alerts/{uuid}",
                "alerts.update_status": "/oms/alerts/{uuid}/status",
                "alerts.delete": "/oms/alerts/{uuid}",
                "alerts.deleteBulk": "/oms/alerts",
                "margins.orders": "/oms/margins/orders",
                "margins.basket": "/oms/margins/basket",
                "ipo.instruments": "/oms/ipo/instruments",
                "ipo.instruments.info": "/oms/ipo/instruments/{id}",
                "ipo.applications": "/oms/ipo/applications",
                "ipo.applications.info": "/oms/ipo/applications/{id}",
                "ipo.applications.create": "/oms/ipo/applications",
                "ipo.applications.modify": "/oms/ipo/applications/{id}",
                "ipo.applications.cancel": "/oms/ipo/applications/{id}"
            },
            s = {
                login: "/",
                apps: "/apps",
                orders: "/orders",
                gtt: "/orders/gtt",
                baskets: "/orders/baskets",
                sip: "/orders/sip",
                alerts: "/orders/alerts",
                ipo: "/orders/ipo",
                margins: "/funds",
                logout: "/logout",
                profile: "/profile",
                holdings: "/holdings",
                positions: "/positions",
                dashboard: "/dashboard",
                marketwatch: "/marketwatch",
                connectLogin: "/connect/login",
                forgotPassword: "/forgot",
                resetPassword: "/reset",
                connectLoginPopup: "/connect/login/popup",
                chart: "/chart/web/ciq/:segment/:tradingsymbol/:token",
                multipleChart: "/multiplechart/ext/ciq/",
                extChart: "/chart/ext/ciq/:segment/:tradingsymbol/:token",
                tvchart: "/chart/web/tvc/:segment/:tradingsymbol/:token",
                extTvchart: "/chart/ext/tvc/:segment/:tradingsymbol/:token",
                connectAuthorize: "/connect/authorize",
                connectFinish: "/connect/finish",
                connectBasket: "/connect/basket",
                editProfile: "/edit-profile",
                holdingsAuthorise: "/connect/portfolio/authorise/holdings/:apiKey/:reqID"
            };

        function o(e, t) {
            return n()(r[e], t)
        }
    },
    "56d7": function(e, t, i) {
        "use strict";
        i.r(t);
        var a = i("a026"),
            n = (i("bc3a"), i("8c4f")),
            r = i("0644"),
            s = i.n(r),
            o = i("c0d6"),
            c = i("41cb");
        class l {
            constructor() {
                this.eventBus = new a["a"]
            }
            on(e, t) {
                return this.eventBus.$on(e, t)
            }
            emit(e, t) {
                return this.eventBus.$emit(e, t)
            }
            off(e, t) {
                return this.eventBus.$off(e, t)
            }
        }
        var d = i("f1f7");
        let u = e => e;
        var h = {
                getTime: u
            },
            m = i("0a3b");
        class p {}
        p.install = function(e, t) {
            e.filter("decimalPad", d["a"].decimalPad), e.filter("inrFormat", d["a"].inrFormat), e.filter("round", d["a"].round), e.filter("kformat", d["a"].kformat), e.filter("getTime", h.getTime), e.filter("dateSuperscript", m["b"].dateSuperScript)
        };
        class f {}
        f.install = function(e, t) {};
        var g = {
                bind(e, t, i) {
                    e.customEventKeydown = function(e) {
                        27 === e.keyCode && i.context[t.expression](e)
                    }, document.body.addEventListener("keydown", e.customEventKeydown)
                },
                unbind(e) {
                    document.body.removeEventListener("keydown", e.customEventKeydown)
                }
            },
            v = i("72e2"),
            b = {
                bind(e, t, i) {
                    e.customEventClick = function(a) {
                        e === a.target || e.contains(a.target) || i.context[t.expression](a)
                    }, document.body.addEventListener("click", e.customEventClick)
                },
                unbind(e) {
                    document.body.removeEventListener("click", e.customEventClick)
                }
            };
        class w {
            constructor() {
                this.allEvents = {}, this.keyPressed = [], this.init()
            }
            register(e) {
                for (let t of e) {
                    let e = t.keys.sort().join();
                    this.allEvents[e] || (this.allEvents[e] = []), this.allEvents[e].push(t)
                }
            }
            deregister(e) {
                for (let t of e) {
                    let e = t.keys.sort().join();
                    if (!this.allEvents[e]) continue;
                    let i = -1;
                    for (let a = 0; a < this.allEvents[e].length; a++)
                        if (t.cb === this.allEvents[e][a].cb) {
                            i = a;
                            break
                        } - 1 !== i && this.allEvents[e].splice(i, 1)
                }
            }
            init() {
                window.addEventListener("keydown", e => this.onKeyDown(e)), window.addEventListener("keyup", e => this.onKeyUp(e)), window.addEventListener("blur", e => this.resetKeyPressed(e)), window.addEventListener("focus", e => this.resetKeyPressed(e))
            }
            destroy() {
                window.removeEventListener("keyup", this.onKeyUp), window.removeEventListener("keydown", this.onKeyDown), window.removeEventListener("blur", this.resetKeyPressed), window.removeEventListener("focus", this.resetKeyPressed)
            }
            onKeyDown(e) {
                let t = e && e.keyCode && e.keyCode.toString();
                if (!e.keyCode) return; - 1 === this.keyPressed.indexOf(t) && this.keyPressed.push(t);
                let i = this.keyPressed.sort().join();
                if (this.allEvents[i] && this.allEvents[i].length > 0)
                    for (let a of this.allEvents[i].slice().reverse())
                        if (this.keyboardEventHandler(e, a), a.stop) break
            }
            onKeyUp(e) {
                e.keyCode && ("Meta" === e.key ? this.keyPressed = [] : this.keyPressed.splice(this.keyPressed.indexOf(e.keyCode.toString()), 1))
            }
            resetKeyPressed(e) {
                this.keyPressed = []
            }
            keyboardEventHandler(e, t) {
                if (t.bind && e.target !== t.bind) return;
                const i = ["INPUT", "TEXTAREA", "SELECT"];
                !1 !== t.ignoreInput && -1 !== i.indexOf(e.target.tagName.toUpperCase()) || (t.cb(), t.stop && e.stopPropagation(), t.prevent && e.preventDefault())
            }
        }
        var k = i("b202"),
            y = i("df03"),
            _ = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("router-view")
            },
            C = [],
            q = {},
            S = q,
            E = i("2877"),
            O = Object(E["a"])(S, _, C, !1, null, null, null),
            x = O.exports,
            T = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("form", {
                    on: {
                        submit: e.onSubmit
                    }
                }, [e._t("default")], 2)
            },
            $ = [],
            P = {
                props: {
                    type: {
                        type: String,
                        default: "text"
                    },
                    placeholder: {
                        type: String,
                        default: ""
                    },
                    label: {
                        type: String,
                        default: ""
                    },
                    rules: {}
                },
                data() {
                    return {
                        errors: [],
                        state: {
                            hidden: !0
                        },
                        currentValue: this.value,
                        currentLabel: this.label,
                        currentPlaceholder: this.placeholder
                    }
                },
                watch: {},
                mounted() {},
                methods: {
                    reset() {
                        for (let e of this.$children) e.hasOwnProperty("hideError") && e.hideError()
                    },
                    validate() {
                        this.errors = [];
                        let e = !0;
                        for (let t of this.$children) t.hasOwnProperty("validate") && (t.validate() || (t.message && this.errors.push(t.message), e = !1));
                        return e
                    },
                    onFocus() {
                        this.currentValue || (this.state = ["visible", "slideInUp", "animated"], this.currentPlaceholder = "")
                    },
                    onBlur() {
                        this.currentValue || (this.state = ["hidden"], this.currentPlaceholder = this.placeholder)
                    },
                    handleInput(e) {
                        const t = e.target.value;
                        this.$emit("input", t), this.currentValue = t, this.$emit("change", t)
                    },
                    onSubmit(e) {
                        this.$emit("submit", e)
                    }
                }
            },
            I = P,
            D = Object(E["a"])(I, T, $, !1, null, null, null),
            L = D.exports,
            B = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("div", {
                    class: e.classes
                }, [this.staticLabel || this.animate ? i("label", {
                    class: e.labelClasses
                }, [e._v(e._s(e.currentLabel))]) : e._e(), e._v(" "), "textarea" != e.type ? i("input", e._b({
                    ref: "input",
                    attrs: {
                        type: e.type,
                        placeholder: e.currentPlaceholder,
                        autocorrect: "off"
                    },
                    domProps: {
                        value: e.currentValue
                    },
                    on: {
                        focus: e.onFocus,
                        blur: e.onBlur,
                        keyup: function(t) {
                            return !t.type.indexOf("key") && e._k(t.keyCode, "enter", 13, t.key, "Enter") ? null : e.onEnter(t)
                        },
                        input: e.handleInput
                    }
                }, "input", e.$props, !1)) : e._e(), e._v(" "), e.icon ? i("span", {
                    class: ["icon", "icon-" + e.icon]
                }) : e._e(), e._v(" "), "textarea" === e.type ? i("textarea", e._b({
                    ref: "input",
                    attrs: {
                        type: e.type,
                        placeholder: e.currentPlaceholder
                    },
                    on: {
                        focus: e.onFocus,
                        blur: e.onBlur,
                        keyup: function(t) {
                            return !t.type.indexOf("key") && e._k(t.keyCode, "enter", 13, t.key, "Enter") ? null : e.onEnter(t)
                        },
                        input: e.handleInput
                    }
                }, "textarea", e.$props, !1)) : e._e(), e._v(" "), e.noError || e.nativeError || !e.message ? e._e() : i("span", {
                    staticClass: "su-message"
                }, [e._v(e._s(e.message))])])
            },
            A = [],
            N = {
                props: {
                    id: String,
                    name: Number,
                    required: Boolean,
                    minlength: Number,
                    maxlength: Number,
                    min: Number,
                    max: Number,
                    step: Number,
                    value: [String, Number],
                    disabled: Boolean,
                    readonly: Boolean,
                    autofocus: Boolean,
                    rows: Number,
                    cols: Number,
                    icon: String,
                    title: String,
                    nativeError: Boolean,
                    noError: Boolean,
                    staticLabel: Boolean,
                    autocapitalize: String,
                    autocomplete: String,
                    validateOnChange: Boolean,
                    animate: {
                        type: Boolean,
                        default: !0
                    },
                    type: {
                        type: String,
                        default: "text"
                    },
                    placeholder: {
                        type: String,
                        default: ""
                    },
                    label: {
                        type: String,
                        default: ""
                    },
                    rules: {
                        type: Array,
                        default: function() {
                            return []
                        }
                    },
                    dynamicWidth: {
                        type: Boolean,
                        default: !1
                    },
                    dynamicWidthSize: {
                        type: Number,
                        default: 8
                    },
                    pattern: String
                },
                data() {
                    return {
                        currentValue: this.value,
                        currentLabel: this.label,
                        currentPlaceholder: this.placeholder,
                        labelVisible: !1,
                        message: ""
                    }
                },
                watch: {
                    value(e, t, i) {
                        t !== e && (this.currentValue = e), this.max && e > this.max && (this.currentValue = this.max, this.$emit("input", this.currentValue)), this.$nextTick(() => {
                            this.setDynamicWidth(e)
                        })
                    },
                    placeholder(e, t) {
                        t !== e && (this.currentPlaceholder = e)
                    },
                    label(e, t) {
                        t !== e && (this.currentLabel = e)
                    }
                },
                created() {},
                mounted() {
                    this.$nextTick(() => {
                        this.label || (this.currentLabel = this.currentPlaceholder), this.currentValue && (this.onFocus(), this.setDynamicWidth(this.currentValue)), this.autofocus && this.focus()
                    })
                },
                computed: {
                    classes() {
                        return {
                            "su-input-group": !0,
                            "su-error-state": !1,
                            "su-static-label": this.staticLabel,
                            disabled: this.disabled,
                            "su-has-icon": this.icon
                        }
                    },
                    labelClasses() {
                        return ["su-input-label", {
                            "su-visible": this.labelVisible,
                            "su-dynamic-label": !this.staticLabel
                        }]
                    }
                },
                methods: {
                    onFocus(e) {
                        this.labelVisible = this.animate, this.animate && (this.currentPlaceholder = ""), this.$emit("focus", e)
                    },
                    onBlur(e) {
                        this.currentValue || (this.labelVisible = !1, this.currentPlaceholder = this.placeholder), this.$emit("blur", e)
                    },
                    onEnter() {
                        this.$emit("enter", this.currentValue)
                    },
                    handleInput(e) {
                        const t = e.target.value;
                        this.$emit("input", t), this.currentValue = t, this.$emit("change", e), this.validateOnChange ? this.validate() : this.hideError(), this.setDynamicWidth(t)
                    },
                    showLabel() {
                        this.labelVisible = !0
                    },
                    hideLabel() {
                        this.labelVisible = !1
                    },
                    showError(e) {
                        this.classes["su-error-state"] = !0, this.message = e, this.nativeError && (this.$refs.input.setAttribute("pattern", "this-is-dummy-pattern-123"), this.$refs.input.setCustomValidity(e), this.$refs.input.reportValidity())
                    },
                    hideError(e) {
                        this.classes["su-error-state"] = !1, this.message = "", this.nativeError && (this.$refs.input.removeAttribute("pattern"), this.$refs.input.setCustomValidity(""))
                    },
                    focus() {
                        this.$refs.input && this.$refs.input.focus()
                    },
                    blur() {
                        this.$refs.input && this.$refs.input.blur()
                    },
                    validate() {
                        this.hideError();
                        let e = !0;
                        for (let t of this.rules)
                            if (t instanceof Object && (t.validator || t.required)) {
                                let i = null;
                                if (!t.required || this.currentValue && 0 !== this.currentValue.toString().trim().length ? t.validator && (i = t.validator(t, this.currentValue)) : i = t.message ? Error(t.message) : Error(this.currentLabel + " is required"), i instanceof Error) {
                                    this.showError(i.message), this.$emit("error", i), e = !1;
                                    break
                                }
                            }
                        return e
                    },
                    setDynamicWidth(e) {
                        this.dynamicWidth && e && (this.$refs.input.style.width = e.toString().length * this.dynamicWidthSize + "px")
                    }
                }
            },
            R = N,
            j = Object(E["a"])(R, B, A, !1, null, null, null),
            F = j.exports,
            V = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("div", {
                    class: e.classes,
                    attrs: {
                        title: e.title
                    }
                }, [i("input", e._b({
                    staticClass: "su-radio",
                    attrs: {
                        id: e.id,
                        type: "radio"
                    },
                    domProps: {
                        checked: e.isChecked
                    },
                    on: {
                        change: e.handleInput
                    }
                }, "input", e.$props, !1)), e._v(" "), i("label", {
                    staticClass: "su-radio-label",
                    attrs: {
                        for: e.id
                    },
                    domProps: {
                        innerHTML: e._s(e.currentLabel)
                    }
                })])
            },
            M = [],
            U = {
                name: "SuRadio",
                componentName: "SuRadio",
                props: {
                    name: String,
                    tabindex: Number,
                    value: String,
                    disabled: Boolean,
                    autofocus: Boolean,
                    title: String,
                    checked: Boolean,
                    label: {
                        type: String,
                        default: ""
                    }
                },
                data() {
                    return {
                        currentLabel: this.label,
                        id: "radio-0",
                        attrs: {},
                        isChecked: this.checked
                    }
                },
                mounted() {
                    this.id = "radio-" + this._uid
                },
                watch: {
                    label(e, t) {
                        t !== e && (this.currentLabel = e)
                    }
                },
                computed: {
                    classes() {
                        return {
                            "su-radio-wrap": !0,
                            checked: this.isChecked,
                            disabled: this.disabled
                        }
                    }
                },
                methods: {
                    handleInput(e) {
                        this.$parent.setValue(e.target.value, e)
                    },
                    setChecked(e) {
                        this.isChecked = e
                    }
                }
            },
            z = U,
            H = Object(E["a"])(z, V, M, !1, null, null, null),
            K = H.exports,
            W = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("div", {
                    staticClass: "su-checkbox-group",
                    class: e.groupClass
                }, [i("input", e._b({
                    staticClass: "su-checkbox",
                    attrs: {
                        id: e.id,
                        type: "checkbox"
                    },
                    domProps: {
                        checked: e.isChecked,
                        value: e.currentValue
                    },
                    on: {
                        change: e.handleInput
                    }
                }, "input", e.$props, !1)), e._v(" "), i("label", {
                    staticClass: "su-checkbox-label",
                    attrs: {
                        for: e.id
                    }
                }, [e._m(0), e._v(" "), e.currentLabel ? i("span", {
                    staticClass: "su-checkbox-value"
                }, [e._v(e._s(e.currentLabel))]) : e._e()])])
            },
            G = [function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("span", {
                    staticClass: "su-checkbox-box"
                }, [i("span", {
                    staticClass: "su-checkbox-tick"
                })])
            }],
            J = {
                name: "suCheckbox",
                componentName: "suCheckbox",
                props: {
                    name: String,
                    tabindex: Number,
                    value: Boolean,
                    disabled: Boolean,
                    autofocus: Boolean,
                    title: String,
                    checked: Boolean,
                    indeterminate: Boolean,
                    label: {
                        type: String,
                        default: ""
                    }
                },
                data() {
                    return {
                        attrs: {},
                        id: "checkbox-0",
                        currentLabel: this.label,
                        currentValue: this.value,
                        isChecked: this.checked,
                        isIndeterminate: this.indeterminate
                    }
                },
                mounted() {
                    this.id = "checkbox-" + this._uid, this.value && (this.isChecked = !0)
                },
                computed: {
                    groupClass() {
                        return {
                            indeterminate: this.isIndeterminate
                        }
                    }
                },
                watch: {
                    label(e, t) {
                        t !== e && (this.currentLabel = e)
                    },
                    value(e) {
                        this.currentValue = e, this.$emit("change", e), this.$emit("input", e), this.isChecked = !!e
                    },
                    indeterminate(e) {
                        this.isIndeterminate = e
                    }
                },
                methods: {
                    handleInput(e) {
                        const t = e.target.checked;
                        this.isChecked = t, this.currentValue = t, this.$emit("input", t), this.$emit("change", e)
                    }
                }
            },
            Y = J,
            X = Object(E["a"])(Y, W, G, !1, null, null, null),
            Z = X.exports,
            Q = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("button", {
                    attrs: {
                        disabled: e.isDisabled || e.isProcessing,
                        type: e.type,
                        title: e.title
                    }
                }, [e.isProcessing ? e._e() : e._t("default"), e._v(" "), e.isProcessing ? i("su-loader", {
                    attrs: {
                        size: 12
                    }
                }) : e._e()], 2)
            },
            ee = [],
            te = {
                props: {
                    disabled: Boolean,
                    processing: Boolean,
                    title: String,
                    type: {
                        type: String,
                        default: "button"
                    }
                },
                data() {
                    return {
                        isProcessing: this.processing,
                        isDisabled: this.disabled
                    }
                },
                watch: {
                    processing(e, t) {
                        this.isProcessing = e
                    },
                    disabled(e, t) {
                        this.isDisabled = e
                    }
                }
            },
            ie = te,
            ae = Object(E["a"])(ie, Q, ee, !1, null, null, null),
            ne = ae.exports,
            re = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("div", {
                    staticClass: "su-switch-group"
                }, [e.stateOffLabel ? i("span", {
                    staticClass: "su-switch-label su-switch-state-off"
                }, [e._v(e._s(e.stateOffLabel))]) : e._e(), e._v(" "), i("input", e._b({
                    staticClass: "su-switch",
                    attrs: {
                        id: e.id,
                        type: "checkbox"
                    },
                    domProps: {
                        checked: e.isChecked,
                        value: e.currentValue
                    },
                    on: {
                        change: e.handleInput
                    }
                }, "input", e.$props, !1)), e._v(" "), i("label", {
                    staticClass: "su-switch-control",
                    attrs: {
                        for: e.id
                    }
                }, [e._v(e._s(e.currentLabel))]), e._v(" "), e.stateOnLabel ? i("span", {
                    staticClass: "su-switch-label su-switch-state-on"
                }, [e._v(e._s(e.stateOnLabel))]) : e._e()])
            },
            se = [],
            oe = {
                name: "suSwitch",
                componentName: "suSwitch",
                props: {
                    name: String,
                    tabindex: Number,
                    disabled: Boolean,
                    autofocus: Boolean,
                    title: String,
                    value: String || Boolean,
                    stateOn: String || Boolean,
                    stateOff: String || Boolean,
                    stateOnLabel: String,
                    stateOffLabel: String,
                    label: {
                        type: String,
                        default: ""
                    }
                },
                data() {
                    return {
                        currentValue: this.value,
                        isChecked: this.value === this.stateOn,
                        currentLabel: this.label,
                        id: "switch-0",
                        attrs: {}
                    }
                },
                mounted() {
                    this.id = "switch-" + this._uid, this.isChecked = this.value === this.stateOn
                },
                watch: {
                    value(e) {
                        this.currentValue = e, this.isChecked = e === this.stateOn
                    }
                },
                methods: {
                    handleInput(e) {
                        this.isChecked = e.target.checked, this.currentValue = e.target.checked ? this.stateOn : this.stateOff, this.$emit("input", this.currentValue), this.$emit("change", e)
                    }
                }
            },
            ce = oe,
            le = Object(E["a"])(ce, re, se, !1, null, null, null),
            de = le.exports,
            ue = function() {
                var e = this,
                    t = e.$createElement;
                e._self._c;
                return e._m(0)
            },
            he = [function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("div", {
                    staticClass: "su-loader"
                }, [i("span", {
                    staticClass: "dot-spinner"
                }, [i("i"), i("i"), i("i"), i("i")])])
            }],
            me = {
                props: {
                    size: Number
                }
            },
            pe = me,
            fe = Object(E["a"])(pe, ue, he, !1, null, null, null),
            ge = fe.exports,
            ve = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return e.isOpen ? i("transition", {
                    attrs: {
                        name: "modal"
                    }
                }, [i("div", {
                    staticClass: "su-modal-mask",
                    style: {
                        "z-index": e.zIndex
                    }
                }, [i("div", {
                    staticClass: "su-modal-container layer-2",
                    style: {
                        "z-index": e.zIndex,
                        "max-width": e.width ? e.width + "px" : ""
                    }
                }, [e.$slots.header ? i("div", {
                    staticClass: "su-modal-header"
                }, [e._t("header")], 2) : e._e(), e._v(" "), e.$slots.body ? i("div", {
                    staticClass: "su-modal-body"
                }, [e._t("body")], 2) : e._e(), e._v(" "), e.$slots.footer ? i("div", {
                    staticClass: "su-modal-footer"
                }, [e._t("footer")], 2) : e.buttons ? i("div", {
                    staticClass: "su-modal-footer buttons"
                }, [null !== e.okLabel ? i("su-button", {
                    staticClass: "button-blue",
                    attrs: {
                        disabled: e.isProcessing || e.okDisabled,
                        processing: e.isProcessing
                    },
                    nativeOn: {
                        click: function(t) {
                            return e.onOk(t)
                        }
                    }
                }, [i("span", [e._v(e._s(this.okLabel ? this.okLabel : "Ok"))])]) : e._e(), e._v(" "), null !== e.cancelLabel ? i("su-button", {
                    staticClass: "button-outline",
                    attrs: {
                        disabled: e.isProcessing || e.cancelDisabled
                    },
                    nativeOn: {
                        click: function(t) {
                            return e.onCancel(t)
                        }
                    }
                }, [i("span", [e._v(e._s(this.cancelLabel ? this.cancelLabel : "Cancel"))])]) : e._e()], 1) : e._e()])])]) : e._e()
            },
            be = [],
            we = {
                name: "modal",
                props: {
                    open: {
                        type: Boolean,
                        default: !0
                    },
                    clickOut: {
                        type: Boolean,
                        default: !0
                    },
                    closeOnEsc: {
                        type: Boolean,
                        default: !0
                    },
                    zIndex: {
                        type: Number,
                        default: 15
                    },
                    width: {
                        type: Number,
                        default: 0
                    },
                    buttons: Boolean,
                    okLabel: "",
                    okDisabled: Boolean,
                    cancelLabel: "",
                    cancelDisabled: Boolean
                },
                data() {
                    return {
                        isOpen: this.open,
                        isProcessing: !1,
                        keyboardShortcutEvents: [{
                            keys: ["27"],
                            cb: this.close,
                            stop: !0,
                            prevent: !0
                        }]
                    }
                },
                methods: {
                    onOk() {
                        this.isProcessing = !0, this.$emit("ok", this.close, () => {
                            this.isProcessing = !1
                        })
                    },
                    onCancel() {
                        this.close(), this.$nextTick(() => {
                            this.$emit("cancel")
                        })
                    },
                    onEsc(e) {
                        27 === e.keyCode && this.onCancel()
                    },
                    onClickOutside(e) {
                        e.target === this.$el.querySelector(".su-modal-wrap") && this.onCancel()
                    },
                    close() {
                        this.isProcessing = !1, this.isOpen = !1, this.$nextTick(() => {
                            this.$emit("update:open", !1)
                        })
                    },
                    handleShortcutEvents() {
                        this.isOpen ? (this.clickOut && window.addEventListener("mousedown", this.onClickOutside), this.closeOnEsc && document.body.addEventListener("keydown", this.onEsc), document.body.classList.add("modal-open")) : (window.removeEventListener("mousedown", this.onClickOutside), document.body.classList.remove("modal-open"), document.body.removeEventListener("keydown", this.onEsc))
                    }
                },
                mounted() {
                    this.handleShortcutEvents()
                },
                watch: {
                    open(e) {
                        this.isOpen = e
                    },
                    isOpen() {
                        this.handleShortcutEvents()
                    }
                }
            },
            ke = we,
            ye = Object(E["a"])(ke, ve, be, !1, null, null, null),
            _e = ye.exports,
            Ce = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return e.isOpen ? i("su-modal", {
                    attrs: {
                        open: e.isOpen,
                        buttons: !0,
                        width: 350
                    },
                    on: {
                        "update:open": function(t) {
                            e.isOpen = t
                        },
                        ok: this.options.onOk,
                        cancel: e.onCancel
                    }
                }, [i("template", {
                    slot: "header"
                }, [i("h3", [e._v(e._s(this.options.title || "Confirm"))])]), e._v(" "), i("template", {
                    slot: "body"
                }, [i("p", [e._v(e._s(this.options.description || "Are you sure?"))])])], 2) : e._e()
            },
            qe = [],
            Se = {
                props: {
                    eventBus: {
                        type: Object,
                        default: null
                    }
                },
                data() {
                    return {
                        isOpen: !1,
                        options: {}
                    }
                },
                methods: {
                    onCancel() {
                        this.options.onCancel && this.options.onCancel()
                    }
                },
                created() {
                    if (!this.eventBus) throw Error("Missing 'event-bus' property in toast");
                    this.eventBus.$on("su-confirm", e => {
                        this.options = e, this.isOpen = !0
                    })
                }
            },
            Ee = Se,
            Oe = Object(E["a"])(Ee, Ce, qe, !1, null, null, null),
            xe = Oe.exports,
            Te = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("div", {
                    staticClass: "su-toast-groups"
                }, e._l(e.orientations, (function(t) {
                    return i("div", {
                        key: t,
                        staticClass: "su-toast-group",
                        class: t
                    }, [i("transition-group", {
                        attrs: {
                            name: "toast-list",
                            tag: "div"
                        }
                    }, e._l(e.toasts[t], (function(t) {
                        return i("div", {
                            key: t.id,
                            staticClass: "su-toast-item layer-2",
                            class: t.classes,
                            on: {
                                mouseover: function(i) {
                                    return e.handleToastHover(t)
                                },
                                mouseleave: function(i) {
                                    return e.handleToastLeave(t)
                                }
                            }
                        }, [i("div", {
                            staticClass: "content",
                            on: {
                                click: function(i) {
                                    return e.handleToastClick(t)
                                }
                            }
                        }, [t.title ? i("h4", {
                            staticClass: "title"
                        }, [e._v(e._s(t.title))]) : e._e(), e._v(" "), i("div", {
                            staticClass: "message",
                            domProps: {
                                innerHTML: e._s(t.message)
                            }
                        })]), e._v(" "), t.closable ? i("span", {
                            staticClass: "icon icon-times close",
                            on: {
                                click: function(i) {
                                    return e.remove(t.id, t.orientation)
                                }
                            }
                        }) : e._e()])
                    })), 0)], 1)
                })), 0)
            },
            $e = [],
            Pe = i("025e");

        function Ie(e, t) {
            var i = Object.keys(e);
            if (Object.getOwnPropertySymbols) {
                var a = Object.getOwnPropertySymbols(e);
                t && (a = a.filter((function(t) {
                    return Object.getOwnPropertyDescriptor(e, t).enumerable
                }))), i.push.apply(i, a)
            }
            return i
        }

        function De(e) {
            for (var t = 1; t < arguments.length; t++) {
                var i = null != arguments[t] ? arguments[t] : {};
                t % 2 ? Ie(Object(i), !0).forEach((function(t) {
                    Le(e, t, i[t])
                })) : Object.getOwnPropertyDescriptors ? Object.defineProperties(e, Object.getOwnPropertyDescriptors(i)) : Ie(Object(i)).forEach((function(t) {
                    Object.defineProperty(e, t, Object.getOwnPropertyDescriptor(i, t))
                }))
            }
            return e
        }

        function Le(e, t, i) {
            return t in e ? Object.defineProperty(e, t, {
                value: i,
                enumerable: !0,
                configurable: !0,
                writable: !0
            }) : e[t] = i, e
        }
        const Be = "toast-";
        var Ae = {
                props: {
                    eventBus: null,
                    maxToasts: {
                        type: Number,
                        default: 10
                    }
                },
                data() {
                    const e = {
                            INFO: "INFO",
                            SUCCESS: "SUCCESS",
                            ERROR: "ERROR",
                            WARNING: "WARNING",
                            LOADING: "LOADING"
                        },
                        t = {
                            INFO: "icon-info",
                            SUCCESS: "icon-check",
                            ERROR: "icon-alert-triangle",
                            WARNING: "icon-alert-triangle",
                            LOADING: "loading"
                        },
                        i = {
                            TOP_LEFT: "su-toast-top-left",
                            TOP_CENTER: "su-toast-top-center",
                            TOP_RIGHT: "su-toast-top-right",
                            BOTTOM_LEFT: "su-toast-bottom-left",
                            BOTTOM_CENTER: "su-toast-bottom-center",
                            BOTTOM_RIGHT: "su-toast-bottom-right"
                        };
                    return {
                        toasts: {},
                        test: null,
                        types: e,
                        orientations: i,
                        icons: t,
                        default: {
                            message: "",
                            duration: 3e3,
                            class: "info",
                            type: e.INFO,
                            closable: !0,
                            orientation: i.TOP_RIGHT
                        }
                    }
                },
                created() {
                    if (!this.eventBus) throw Error("Missing 'event-bus' property in toast");
                    for (let e in this.orientations) this.toasts[this.orientations[e]] = [];
                    this.eventBus.$on("su-toast", this.insert)
                },
                methods: {
                    insert(e) {
                        if (e.message = Object(Pe["i"])(e.message || ""), e.id) {
                            e.id = Be + e.id;
                            var t = this.hasID(e.id, e.orientation);
                            if (e.update && t) return void this.update(e, e.orientation);
                            if (!e.update && t) return
                        } else e.id = Be + Math.floor(1e5 * Math.random());
                        let i = !1;
                        e.onClick && (i = !0);
                        let a = e.class || this.default.class,
                            n = (e.type || this.default.type).toLowerCase(),
                            r = {
                                [n]: !0,
                                [a]: !0,
                                clickable: i
                            },
                            s = De(De(De({
                                id: e.id
                            }, this.default), e), {}, {
                                classes: r
                            });
                        s.timer = window.setTimeout(() => {
                            this.remove(e.id, s.orientation)
                        }, s.duration);
                        var o = De({}, this.toasts);
                        o[s.orientation].push(s), o[s.orientation].length > this.maxToasts && o[s.orientation].splice(0, o[s.orientation].length - this.maxToasts), this.toasts = o
                    },
                    hasID(e, t) {
                        for (let i of this.toasts[t])
                            if (i.id === e) return !0
                    },
                    update(e, t) {
                        let i = De({}, this.toasts),
                            a = [];
                        for (let n of i[t]) {
                            if (n.id === e.id) {
                                n.message = e.message, n.title = e.title, n.onClick = e.onClick;
                                let t = !1;
                                e.onClick && (t = !0);
                                let i = e.class || this.default.class,
                                    a = (e.type || this.default.type).toLowerCase();
                                n.classes = {
                                    [a]: !0,
                                    [i]: !0,
                                    clickable: t
                                }, n.timer && window.clearTimeout(n.timer), n.timer = window.setTimeout(() => {
                                    this.remove(n.id, n.orientation)
                                }, e.duration)
                            }
                            a.push(n)
                        }
                        i[t] = a, this.toasts = i
                    },
                    remove(e, t) {
                        if (!e || !t) return;
                        let i = De({}, this.toasts),
                            a = [];
                        for (let n of i[t]) n.id !== e && a.push(n);
                        i[t] = a, this.toasts = i
                    },
                    handleToastHover(e) {
                        e.timer && window.clearTimeout(e.timer)
                    },
                    handleToastLeave(e) {
                        e.timer = window.setTimeout(() => {
                            this.remove(e.id, e.orientation)
                        }, e.duration)
                    },
                    handleToastClick(e) {
                        e.onClick && (e.onClick(), this.remove(e.id, e.orientation))
                    }
                }
            },
            Ne = Ae,
            Re = Object(E["a"])(Ne, Te, $e, !1, null, null, null),
            je = Re.exports,
            Fe = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("div", {
                    staticClass: "su-select",
                    class: e.classes
                }, [i("select", e._b({
                    directives: [{
                        name: "model",
                        rawName: "v-model",
                        value: e.selected,
                        expression: "selected"
                    }],
                    ref: "select",
                    attrs: {
                        required: e.required,
                        multiple: e.multiple,
                        size: e.nativeSize
                    },
                    on: {
                        blur: function(t) {
                            return e.$emit("blur", t)
                        },
                        focus: function(t) {
                            return e.$emit("focus", t)
                        },
                        change: function(t) {
                            var i = Array.prototype.filter.call(t.target.options, (function(e) {
                                return e.selected
                            })).map((function(e) {
                                var t = "_value" in e ? e._value : e.value;
                                return t
                            }));
                            e.selected = t.target.multiple ? i : i[0]
                        }
                    }
                }, "select", e.$props, !1), [e.placeholder ? i("option", {
                    attrs: {
                        value: "",
                        selected: "",
                        disabled: ""
                    }
                }, [e._v("\n\t\t\t" + e._s(e.placeholder) + "\n\t\t")]) : e._e(), e._v(" "), e._t("default")], 2), e._v(" "), i("i", {
                    staticClass: "icon icon-chevron-down"
                }), e._v(" "), e.noError || e.nativeError || !e.message ? e._e() : i("span", {
                    staticClass: "su-message"
                }, [e._v(e._s(e.message))])])
            },
            Ve = [],
            Me = {
                name: "su-select",
                props: {
                    value: {
                        type: [String, Number, Boolean, Object, Array, Symbol, Function],
                        default: null
                    },
                    disabled: Boolean,
                    rules: {
                        type: Array,
                        default: function() {
                            return []
                        }
                    },
                    placeholder: String,
                    multiple: Boolean,
                    nativeSize: [String, Number],
                    required: Boolean,
                    nativeError: Boolean,
                    noError: Boolean
                },
                data() {
                    return {
                        selected: this.value,
                        message: "",
                        _isSelect: !0,
                        _elementRef: "select"
                    }
                },
                computed: {
                    classes() {
                        return {
                            "su-multiple": this.multiple,
                            "su-default": this.placeholder && !this.selected,
                            "su-error-state": !1
                        }
                    }
                },
                watch: {
                    value(e) {
                        this.selected = e
                    },
                    selected(e) {
                        this.hideError(), this.$emit("input", e)
                    }
                },
                methods: {
                    validate() {
                        this.hideError();
                        let e = !0;
                        for (let t of this.rules)
                            if (t instanceof Object && (t.validator || t.required)) {
                                let i = null;
                                if (!t.required || this.selected && 0 !== this.selected.toString().trim().length ? t.validator && (i = t.validator(t, this.selected)) : i = t.message ? Error(t.message) : Error(this.currentLabel + " is required"), i instanceof Error) {
                                    this.showError(i.message), this.$emit("error", i), e = !1;
                                    break
                                }
                            }
                        return e
                    },
                    showError(e) {
                        this.classes["su-error-state"] = !0, this.message = e, this.nativeError && (this.$refs.input.setAttribute("pattern", "this-is-dummy-pattern-123"), this.$refs.input.setCustomValidity(e), this.$refs.input.reportValidity())
                    },
                    hideError(e) {
                        this.classes["su-error-state"] = !1, this.message = ""
                    }
                }
            },
            Ue = Me,
            ze = Object(E["a"])(Ue, Fe, Ve, !1, null, null, null),
            He = ze.exports,
            Ke = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("div", {
                    staticClass: "su-radio-group"
                }, [e._t("default")], 2)
            },
            We = [],
            Ge = {
                name: "SuRadioGroup",
                componentName: "SuRadioGroup",
                props: {
                    value: String,
                    disabled: Boolean
                },
                watch: {
                    value(e) {
                        this.initChecked(), this.$emit("change", e)
                    }
                },
                mounted() {
                    this.initChecked()
                },
                methods: {
                    setValue(e, t) {
                        this.$emit("input", e)
                    },
                    initChecked() {
                        this.$nextTick(() => {
                            this.$children.forEach(e => {
                                "" !== this.value && e.setChecked(this.value === e.value)
                            })
                        })
                    }
                }
            },
            Je = Ge,
            Ye = Object(E["a"])(Je, Ke, We, !1, null, null, null),
            Xe = Ye.exports,
            Ze = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("div", {
                    attrs: {
                        id: "avatar-" + e._uid
                    }
                }, [e.isImage ? e._e() : i("div", {
                    staticClass: "avatar",
                    style: e.style
                }, [this.src ? e._e() : i("span", [e._v(e._s(e.userInitial))])]), e._v(" "), e.isImage ? i("img", {
                    staticClass: "avatar",
                    style: e.style,
                    attrs: {
                        src: e.src,
                        height: e.size,
                        width: e.size
                    }
                }) : e._e()])
            },
            Qe = [],
            et = {
                props: {
                    username: {
                        type: String,
                        required: !0
                    },
                    initials: {
                        type: String
                    },
                    backgroundColor: {
                        type: String
                    },
                    color: {
                        type: String,
                        default: "#ffffff"
                    },
                    size: {
                        type: Number,
                        default: 50
                    },
                    src: {
                        type: String
                    },
                    rounded: {
                        type: Boolean,
                        default: !0
                    },
                    lighten: {
                        type: Number,
                        default: 80
                    }
                },
                data() {
                    return {
                        backgroundColors: ["63, 81, 181", "156, 39, 176", "76, 175, 80", "255, 87, 34", "232, 30, 99", "156, 39, 176", "103, 58, 183"]
                    }
                },
                mounted() {
                    this.$emit("avatar-initials", this.username, this.userInitial)
                },
                computed: {
                    background() {
                        return this.backgroundColor || this.randomBackgroundColor(this.username.length, this.backgroundColors)
                    },
                    fontColor() {
                        return this.color || this.lightenColor(this.background, this.lighten)
                    },
                    isImage() {
                        return this.src
                    },
                    style() {
                        const e = {
                                width: this.size + "px",
                                height: this.size + "px",
                                borderRadius: this.rounded ? "50%" : 0,
                                textAlign: "center",
                                verticalAlign: "middle"
                            },
                            t = {
                                background: "url(" + this.src + ") no-repeat",
                                backgroundSize: this.size + "px " + this.size + "px",
                                backgroundOrigin: "content-box"
                            },
                            i = {
                                backgroundColor: "rgba(" + this.background + ", 0.1)",
                                fontSize: Math.floor(this.size / 2.75) + "px",
                                fontWeight: "300",
                                color: "rgb(" + this.background + ")",
                                lineHeight: this.size + Math.floor(this.size / 20) + "px"
                            },
                            a = this.isImage ? t : i;
                        return Object.assign(e, a), e
                    },
                    userInitial() {
                        return this.initials || this.initial(this.username)
                    }
                },
                methods: {
                    initial(e) {
                        let t = e.split(/[ -]/),
                            i = "";
                        for (var a = 0; a < t.length; a++) i += t[a].charAt(0);
                        return i.length > 2 && -1 !== i.search(/[A-Z]/) && (i = i.replace(/[a-z]+/g, "")), i = i.substr(0, 2).toUpperCase(), i
                    },
                    randomBackgroundColor(e, t) {
                        return t[e % t.length]
                    },
                    lightenColor(e, t) {
                        var i = !1;
                        "#" === e[0] && (e = e.slice(1), i = !0);
                        var a = parseInt(e, 16),
                            n = (a >> 16) + t;
                        n > 255 ? n = 255 : n < 0 && (n = 0);
                        var r = (a >> 8 & 255) + t;
                        r > 255 ? r = 255 : r < 0 && (r = 0);
                        var s = (255 & a) + t;
                        return s > 255 ? s = 255 : s < 0 && (s = 0), (i ? "#" : "") + (s | r << 8 | n << 16).toString(16)
                    }
                }
            },
            tt = et,
            it = Object(E["a"])(tt, Ze, Qe, !1, null, null, null),
            at = it.exports;

        function nt(e, t, i) {
            if (!(window.innerWidth < 1023)) {
                var a = e.getAttribute("title"),
                    n = e.getAttribute("tooltip-pos");
                if (a && "" !== a) {
                    var r = t.value;
                    e.removeAttribute("title"), e.setAttribute("data-balloon", a), e.setAttribute("data-balloon-pos", n || "up"), r && e.setAttribute("data-balloon-length", r)
                }
            }
        }
        var rt = {
                bind: nt,
                update: nt
            },
            st = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return e.data ? i("div", {
                    directives: [{
                        name: "click-outside",
                        rawName: "v-click-outside",
                        value: e.onClickOutside,
                        expression: "onClickOutside"
                    }],
                    staticClass: "data-table",
                    attrs: {
                        tabindex: "1"
                    }
                }, [e.toolbar ? i("div", {
                    staticClass: "toolbar"
                }, [e._t("toolbar-before-search"), e._v(" "), e.search ? i("span", {
                    staticClass: "search",
                    class: {
                        expand: e.isSearchExpaned, dirty: e.currentSearchTerm.length > 0
                    }
                }, [i("span", {
                    staticClass: "icon icon-search"
                }), e._v(" "), i("span", {
                    directives: [{
                        name: "show",
                        rawName: "v-show",
                        value: e.currentSearchTerm.length > 0,
                        expression: "currentSearchTerm.length > 0"
                    }],
                    staticClass: "clear icon icon-times",
                    on: {
                        click: e.clearSerachTerm
                    }
                }), e._v(" "), i("su-input", {
                    ref: "searchInput",
                    staticClass: "search-input",
                    attrs: {
                        type: "search",
                        maxlength: 15,
                        animate: !1,
                        placeholder: e.customSearchPlaceHolder
                    },
                    on: {
                        blur: e.onSearchBlur,
                        focus: e.onSearchFocus,
                        input: e.doDebounceFilterSearch
                    },
                    nativeOn: {
                        keyup: function(t) {
                            return !t.type.indexOf("key") && e._k(t.keyCode, "enter", 13, t.key, "Enter") ? null : e.doSearch(!1)
                        }
                    },
                    model: {
                        value: e.currentSearchTerm,
                        callback: function(t) {
                            e.currentSearchTerm = "string" === typeof t ? t.trim() : t
                        },
                        expression: "currentSearchTerm"
                    }
                })], 1) : e._e(), e._v(" "), e._t("toolbar-before"), e._v(" "), e.downloadCsv ? i("span", {
                    staticClass: "download"
                }, [i("span", {
                    staticClass: "download-csv link",
                    on: {
                        click: e.doCSVDownload
                    }
                }, [i("span", {
                    staticClass: "icon icon-download"
                }), e._v("Download\n\t\t\t\t"), i("a", {
                    ref: "csvDownloadLink",
                    staticClass: "hide download-link",
                    attrs: {
                        href: "#"
                    }
                })])]) : e._e(), e._v(" "), e._t("toolbar-after")], 2) : e._e(), e._v(" "), i("table", [e.headers ? i("thead", [i("tr", [e.select && e.uid ? i("th", {
                    staticClass: "select"
                }, [i("div", {
                    class: ["su-checkbox-group", {
                        indeterminate: this.isSelectAllIndeterminate
                    }]
                }, ["hide" !== e.selectAll ? i("input", {
                    directives: [{
                        name: "model",
                        rawName: "v-model",
                        value: e.selectAllValue,
                        expression: "selectAllValue"
                    }],
                    staticClass: "su-checkbox",
                    attrs: {
                        id: "selectall",
                        type: "checkbox",
                        disabled: "disable" === e.selectAll
                    },
                    domProps: {
                        checked: Array.isArray(e.selectAllValue) ? e._i(e.selectAllValue, null) > -1 : e.selectAllValue
                    },
                    on: {
                        input: e.handleSelectAll,
                        change: function(t) {
                            var i = e.selectAllValue,
                                a = t.target,
                                n = !!a.checked;
                            if (Array.isArray(i)) {
                                var r = null,
                                    s = e._i(i, r);
                                a.checked ? s < 0 && (e.selectAllValue = i.concat([r])) : s > -1 && (e.selectAllValue = i.slice(0, s).concat(i.slice(s + 1)))
                            } else e.selectAllValue = n
                        }
                    }
                }) : e._e(), e._v(" "), e._m(0)])]) : e._e(), e._v(" "), e._l(e.headers, (function(t, a) {
                    return i("th", e._g({
                        key: a,
                        class: [t.class, t._style, {
                            sortable: e.sort && t.sort
                        }]
                    }, e.sort && t.sort ? {
                        click: function() {
                            return e.doSort(t)
                        }
                    } : {}), [i("span", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip",
                            value: t.description,
                            expression: "header.description"
                        }],
                        attrs: {
                            title: t.description
                        }
                    }, [e._v("\n\t\t\t\t\t\t" + e._s(t.label || t.field) + "\n\t\t\t\t\t")])])
                }))], 2)]) : e._e(), e._v(" "), i("draggable", {
                    attrs: {
                        tag: "tbody",
                        draggable: ".draggable-row"
                    },
                    on: {
                        change: e.handleOrderChange
                    },
                    model: {
                        value: e.tableDataDragable,
                        callback: function(t) {
                            e.tableDataDragable = t
                        },
                        expression: "tableDataDragable"
                    }
                }, [e._l(e.tableData, (function(t, a) {
                    return [!t._hide && (-1 === e.currentLimit || a < e.currentLimit) ? i("tr", {
                        key: e.uid && t[e.uid] || a,
                        class: [t._style, {
                            "draggable-row": e.drag,
                            focused: e.focus && e.isRowFocused(t[e.uid]),
                            selected: e.select && e.isRowSelected(t[e.uid])
                        }],
                        on: {
                            click: function(i) {
                                return e.handleRowClick(t, i)
                            },
                            mouseenter: function(i) {
                                return e.handleMouseEnter(t, i)
                            },
                            mouseleave: function(i) {
                                return e.handleMouseLeave(t, i)
                            }
                        }
                    }, [e.select && e.uid ? i("td", {
                        staticClass: "select",
                        on: {
                            click: function(e) {
                                e.stopPropagation()
                            }
                        }
                    }, [i("div", {
                        staticClass: "su-checkbox-group"
                    }, [i("input", {
                        directives: [{
                            name: "model",
                            rawName: "v-model",
                            value: e.selectedUIDs[t[e.uid]],
                            expression: "selectedUIDs[row[uid]]"
                        }],
                        staticClass: "su-checkbox",
                        attrs: {
                            id: t[e.uid],
                            type: "checkbox",
                            disabled: t.disableSelect
                        },
                        domProps: {
                            checked: Array.isArray(e.selectedUIDs[t[e.uid]]) ? e._i(e.selectedUIDs[t[e.uid]], null) > -1 : e.selectedUIDs[t[e.uid]]
                        },
                        on: {
                            change: [function(i) {
                                var a = e.selectedUIDs[t[e.uid]],
                                    n = i.target,
                                    r = !!n.checked;
                                if (Array.isArray(a)) {
                                    var s = null,
                                        o = e._i(a, s);
                                    n.checked ? o < 0 && e.$set(e.selectedUIDs, t[e.uid], a.concat([s])) : o > -1 && e.$set(e.selectedUIDs, t[e.uid], a.slice(0, o).concat(a.slice(o + 1)))
                                } else e.$set(e.selectedUIDs, t[e.uid], r)
                            }, e.handleSelectRow]
                        }
                    }), e._v(" "), i("label", {
                        staticClass: "su-checkbox-label",
                        attrs: {
                            for: t[e.uid]
                        }
                    }, [i("span", {
                        staticClass: "su-checkbox-box"
                    }, [i("span", {
                        staticClass: "su-checkbox-tick"
                    })])])])]) : e._e(), e._v(" "), e._t("default", null, {
                        row: t,
                        index: a
                    })], 2) : e._e(), e._v(" "), e._t("after-row", null, {
                        row: t,
                        index: a
                    })]
                })), e._v(" "), e.currentLimit > -1 && e.data.length > e.limit ? i("tr", {
                    staticClass: "show-all-row",
                    attrs: {
                        slot: "footer"
                    },
                    slot: "footer"
                }, [e.select && e.uid ? i("td") : e._e(), e._v(" "), i("td", {
                    staticClass: "show-all-col",
                    attrs: {
                        colspan: e.headers.length
                    }
                }, [i("a", {
                    attrs: {
                        href: ""
                    },
                    on: {
                        click: function(t) {
                            return t.preventDefault(), t.stopPropagation(), e.showAllRows(t)
                        }
                    }
                }, [e._v(e._s(e.showAllLabel) + " (" + e._s(e.data.length - e.currentLimit) + " remaining items)")])])]) : e._e()], 2), e._v(" "), i("tfoot", [e._t("footer")], 2)], 1)]) : e._e()
            },
            ot = [function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("label", {
                    staticClass: "su-checkbox-label",
                    attrs: {
                        for: "selectall"
                    }
                }, [i("span", {
                    staticClass: "su-checkbox-box"
                }, [i("span", {
                    staticClass: "su-checkbox-tick"
                })])])
            }],
            ct = i("b047c"),
            lt = i.n(ct),
            dt = i("310e"),
            ut = i.n(dt),
            ht = {
                components: {
                    draggable: ut.a
                },
                props: {
                    data: {
                        required: !0,
                        type: Array,
                        default: []
                    },
                    headers: {
                        type: Array,
                        default: null
                    },
                    uid: {
                        type: String,
                        default: null
                    },
                    toolbar: {
                        type: Boolean,
                        default: !0
                    },
                    downloadCsv: String || Boolean,
                    sort: Boolean,
                    focus: Boolean,
                    select: Boolean,
                    drag: Boolean,
                    selectAll: String,
                    maxSelectAllLength: {
                        type: Number,
                        default: -1
                    },
                    selectOnRowClick: Boolean,
                    search: Boolean,
                    searchPlaceHolder: {
                        type: String,
                        default: "Search table"
                    },
                    limit: {
                        type: Number,
                        default: -1
                    },
                    showAllLabel: {
                        type: String,
                        default: "Show all rows"
                    },
                    rowEvents: Boolean,
                    rowClick: Function,
                    rowEnter: Function,
                    rowLeave: Function,
                    rowKeyDown: Function,
                    rowOrderChange: Function
                },
                data() {
                    return {
                        currentSearchTerm: "",
                        isSearchExpaned: !1,
                        customSearchPlaceHolder: "Search",
                        tableData: [],
                        selectedUIDs: {},
                        selectAllValue: !1,
                        isSelectAllIndeterminate: !1,
                        currentLimit: this.limit,
                        focusedRow: null
                    }
                },
                created() {
                    this.setData(this.data, this.currentLimit), this.currentSortedHeader = null, this.currentSearchFilterOnly = !0, this.searchableFields = this.headers.filter(e => e.search).map(e => e.field)
                },
                mounted() {
                    this.focus && this.$el.addEventListener("keydown", this.handleKeyDown)
                },
                beforeDestroy() {
                    this.focus && this.$el.removeEventListener("keydown", this.handleKeyDown), this.select && (this.selectAllValue = !1, this.handleSelectAll({
                        target: {
                            checked: !1
                        }
                    }))
                },
                activated() {},
                deactivated() {
                    this.select && (this.selectAllValue = !1, this.handleSelectAll({
                        target: {
                            checked: !1
                        }
                    }))
                },
                watch: {
                    data(e) {
                        this.setData(e, this.currentLimit)
                    }
                },
                computed: {
                    tableDataDragable: {
                        get() {
                            let e = [];
                            for (let t of this.tableData) e.push(t), e.push({
                                dummy: !0
                            });
                            return e
                        },
                        set(e) {
                            this.tableData = e.filter(e => !e.dummy)
                        }
                    }
                },
                methods: {
                    setData(e, t) {
                        this.tableData = -1 === t ? this.$clone(e) : this.$clone(e.slice(0, t)), this.currentSortedHeader && this.doSort(this.currentSortedHeader, !0), this.currentSearchTerm && this.doSearch(this.currentSearchFilterOnly);
                        let i = {};
                        for (let a of this.tableData) {
                            let e = this.uid && a[this.uid];
                            a.disableSelect ? i[e] = !1 : a.selected ? i[e] = !0 : i[e] = this.selectedUIDs[e]
                        }
                        this.selectedUIDs = i, this.$nextTick(() => this.emitSelectedEvent())
                    },
                    onClickOutside() {
                        this.focusedRow = null
                    },
                    clearSerachTerm() {
                        this.currentSearchTerm = "", this.doSearch()
                    },
                    onSearchBlur() {
                        this.currentSearchTerm.length || this.$nextTick(() => {
                            this.isSearchExpaned = !1, this.customSearchPlaceHolder = "Search"
                        })
                    },
                    onSearchFocus() {
                        this.$nextTick(() => {
                            this.isSearchExpaned = !0, this.customSearchPlaceHolder = this.searchPlaceHolder
                        })
                    },
                    doDebounceFilterSearch: lt()((function(e) {
                        this.doSearch(!0)
                    }), 200),
                    doSearch(e) {
                        this.currentSearchFilterOnly = e;
                        for (let t of this.tableData) {
                            let i = !1;
                            if (this.currentSearchTerm.length > 0)
                                for (let e of this.searchableFields) {
                                    let a = Object(Pe["e"])(t, e).toString().toUpperCase();
                                    if (a && -1 !== a.toUpperCase().indexOf(this.currentSearchTerm.toUpperCase())) {
                                        i = !0;
                                        break
                                    }
                                } else i = !0;
                            t._style || this.$set(t, "_style", {}), this.$set(t, "_hide", !(i || e)), this.$set(t._style, "search-term-present", i), this.$set(t._style, "search-term-absent", !i)
                        }
                    },
                    doSort(e, t) {
                        t ? e._sortOrder || (e._sortOrder = "asc") : e._sortOrder = "asc" === e._sortOrder ? "desc" : "asc", this.tableData.sort((t, i) => {
                            let a = e.sortMethod && e.sortMethod(t) || Object(Pe["e"])(t, e.field),
                                n = e.sortMethod && e.sortMethod(i) || Object(Pe["e"])(i, e.field);
                            if ("asc" === e._sortOrder) {
                                if (a < n) return -1;
                                if (a > n) return 1
                            } else {
                                if (a > n) return -1;
                                if (a < n) return 1
                            }
                            return 0
                        });
                        for (let i of this.headers) i._style = {
                            sortable: i.sort
                        };
                        e._style = {
                            sortable: !0,
                            sorted: !0,
                            [e._sortOrder]: !0
                        }, this.currentSortedHeader = e
                    },
                    doCSVDownload() {
                        let e = [],
                            t = [];
                        for (let r of this.headers) !0 !== r.excludeDownload && t.push(r.downloadLabel || r.label || r.field);
                        e.push(t);
                        for (let r of this.tableData) {
                            let t = [];
                            for (let e of this.headers) {
                                if (!0 === e.excludeDownload) continue;
                                let i = e.downloadField && Object(Pe["e"])(r, e.downloadField) || r[e.field];
                                void 0 === i && (i = ""), t.push(i)
                            }
                            e.push(t)
                        }
                        let i = "data:text/csv;charset=utf-8,";
                        e.forEach((function(t, a) {
                            let n = t.join(",");
                            i += a < e.length ? n + "\n" : n
                        }));
                        let a = encodeURI(i),
                            n = this.downloadCsv || "download.csv";
                        this.$refs.csvDownloadLink.setAttribute("href", a), this.$refs.csvDownloadLink.setAttribute("download", n), document.body.appendChild(this.$refs.csvDownloadLink), this.$refs.csvDownloadLink.click()
                    },
                    handleSelectAll(e) {
                        let t = {},
                            i = 0;
                        for (let a of this.tableData) {
                            if (a.disableSelect || a._hide) continue;
                            if (this.maxSelectAllLength && this.maxSelectAllLength > 0 && i >= this.maxSelectAllLength) break;
                            let n = this.uid && a[this.uid];
                            t[n] = e.target.checked, i += 1
                        }
                        this.selectedUIDs = t, this.$nextTick(() => this.emitSelectedEvent())
                    },
                    handleSelectRow(e) {
                        !e.target.checked && this.selectAllValue && (this.selectAllValue = !1), this.$nextTick(() => this.emitSelectedEvent())
                    },
                    emitSelectedEvent() {
                        if (!this.rowEvents) return;
                        let e = [];
                        for (let t of this.tableData) this.selectedUIDs[t[this.uid]] && e.push(t);
                        this.$emit("selected", e)
                    },
                    showAllRows() {
                        this.currentLimit = -1, this.setData(this.data, this.currentLimit)
                    },
                    handleRowClick(e, t) {
                        if (this.focusedRow = e, this.select && !e.disableSelect && this.selectOnRowClick) {
                            let t = this.uid && e[this.uid];
                            this.$set(this.selectedUIDs, t, !this.selectedUIDs[t]), this.emitSelectedEvent()
                        }
                        this.rowEvents && this.$emit("rowClick", {
                            row: e,
                            event: t
                        })
                    },
                    scrollToFocusedRow() {
                        if (!this.focusedRow) return;
                        let e = this.$el.querySelector("tr[data-uid='" + this.focusedRow[this.uid] + "']");
                        e && e.scrollIntoView && e.scrollIntoView(!1)
                    },
                    handleKeyDown(e) {
                        if (null !== this.focusedRow) {
                            if (e.preventDefault(), e.stopPropagation(), !this.select || 13 !== e.keyCode && 32 !== e.keyCode || this.$set(this.selectedUIDs, this.focusedRow[this.uid], !this.selectedUIDs[this.focusedRow[this.uid]]), 38 === e.keyCode || 40 === e.keyCode)
                                for (let t = 0; t < this.tableData.length; t++) {
                                    let i = this.tableData[t - 1 < 0 ? this.tableData.length - 1 : t - 1],
                                        a = this.tableData[t + 1 > this.tableData.length - 1 ? 0 : t + 1];
                                    if (this.focusedRow[this.uid] === this.tableData[t][this.uid]) {
                                        if (38 === e.keyCode) {
                                            this.focusedRow = i, this.$nextTick(() => this.scrollToFocusedRow());
                                            break
                                        }
                                        if (40 === e.keyCode) {
                                            this.focusedRow = a, this.$nextTick(() => this.scrollToFocusedRow());
                                            break
                                        }
                                    }
                                }
                            this.rowEvents && this.$emit("rowKeyDown", {
                                row: this.focusedRow,
                                event: e
                            })
                        }
                    },
                    handleMouseEnter(e, t) {
                        this.rowEvents && this.$emit("rowEnter", {
                            row: e,
                            event: t
                        })
                    },
                    handleMouseLeave(e, t) {
                        this.rowEvents && this.$emit("rowLeave", {
                            row: e,
                            event: t
                        })
                    },
                    isRowFocused(e) {
                        return !(!e || !this.focusedRow) && e === this.focusedRow[e]
                    },
                    isRowSelected(e) {
                        return !(!e || !this.selectedUIDs) && this.selectedUIDs[e]
                    },
                    handleOrderChange(e) {
                        this.rowEvents && this.$emit("rowOrderChange", {
                            newIndex: e.moved.newIndex / 2,
                            oldIndex: e.moved.oldIndex / 2
                        })
                    }
                }
            },
            mt = ht,
            pt = Object(E["a"])(mt, st, ot, !1, null, null, null),
            ft = pt.exports,
            gt = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("div", {
                    staticClass: "su-tag-selector"
                }, [i("su-input", {
                    ref: "search",
                    attrs: {
                        icon: "search"
                    },
                    model: {
                        value: e.query,
                        callback: function(t) {
                            e.query = t
                        },
                        expression: "query"
                    }
                }), e._v(" "), i("div", {
                    staticClass: "su-selected"
                }, e._l(e.selected, (function(t) {
                    return i("a", {
                        key: t.tag,
                        class: ["su-tag", t.tag],
                        style: {
                            background: t.background,
                            color: t.color
                        },
                        attrs: {
                            href: "#",
                            title: t.tag
                        },
                        on: {
                            click: function(i) {
                                return i.preventDefault(), e.onUnselect(t)
                            }
                        }
                    }, [i("span", {
                        staticClass: "icon icon-check"
                    }), e._v(" " + e._s(t.tag))])
                })), 0), e._v(" "), i("div", {
                    staticClass: "su-all"
                }, e._l(e.filtered, (function(t) {
                    return i("a", {
                        key: t.tag,
                        class: ["su-tag", t.tag],
                        style: {
                            background: t.background,
                            color: t.color
                        },
                        attrs: {
                            href: "#",
                            title: t.tag
                        },
                        on: {
                            click: function(i) {
                                return i.preventDefault(), e.onSelect(t)
                            }
                        }
                    }, [e._v(e._s(t.tag))])
                })), 0)], 1)
            },
            vt = [],
            bt = {
                name: "tag-selector",
                props: {
                    tags: Array,
                    value: Array,
                    sorted: {
                        type: Boolean,
                        default: !0
                    }
                },
                data() {
                    return {
                        all: [],
                        selected: [],
                        query: ""
                    }
                },
                methods: {
                    onSelect(e) {
                        this.moveTag(e, this.all, this.selected)
                    },
                    onUnselect(e) {
                        this.moveTag(e, this.selected, this.all)
                    },
                    moveTag(e, t, i) {
                        i.push(e), t.splice(t.indexOf(e), 1), this.$props.sorted && this.sortTags(this.all), this.$emit("input", this.selected)
                    },
                    sortTags(e) {
                        e.sort((e, t) => e.tag < t.tag ? -1 : e.tag > t.tag ? 1 : 0)
                    }
                },
                computed: {
                    filtered() {
                        const e = this.query.toLowerCase().split(" ").filter(e => e.trim().length > 1);
                        return 0 === e.length ? this.all : this.all.filter(t => e.filter(e => -1 !== t.tag.indexOf(e)).length === e.length)
                    }
                },
                mounted() {
                    this.all = [...this.$props.tags], this.$props.sorted && this.sortTags(this.all), this.$props.value.length > 0 && this.$props.value.forEach(e => {
                        const t = this.all.find(t => t.tag === e.tag);
                        t && this.moveTag(t, this.all, this.selected)
                    }), this.$refs["search"]["$el"].addEventListener("keydown", e => {
                        "Escape" === e.key ? (e.stopPropagation(), this.query = "") : "Enter" === e.key && (e.stopPropagation(), this.filtered.map(e => this.onSelect(e)), this.query = "")
                    })
                }
            },
            wt = bt,
            kt = Object(E["a"])(wt, gt, vt, !1, null, null, null),
            yt = kt.exports,
            _t = function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return e.steps && e.steps.length > 0 ? i("div", {
                    class: {
                        "stepper-wrapper": !0, vertical: e.vertical, horizontal: !e.vertical
                    }
                }, [e.vertical ? e._l(e.steps, (function(t, a) {
                    return i("div", {
                        key: a,
                        class: {
                            "stepper-item": !0, completed: t.isCompleted
                        }
                    }, [e._m(0, !0), e._v(" "), e.$scopedSlots.name ? i("div", {
                        staticClass: "stepper-name"
                    }, [e._t("name", null, {
                        step: t
                    })], 2) : t.name ? i("div", {
                        staticClass: "stepper-name"
                    }, [e._v("\n\t\t\t\t" + e._s(t.name) + "\n\t\t\t")]) : e._e()])
                })) : e._l(e.steps, (function(t, a) {
                    return i("div", {
                        key: a,
                        class: {
                            "stepper-item": !0, completed: t.isCompleted
                        }
                    }, [i("div", {
                        staticClass: "step-counter"
                    }, [e._v(e._s(t.count))]), e._v(" "), e.$scopedSlots.name ? i("div", {
                        staticClass: "stepper-name"
                    }, [e._t("name", null, {
                        step: t
                    })], 2) : t.name ? i("div", {
                        staticClass: "stepper-name"
                    }, [e._v("\n\t\t\t\t" + e._s(t.name) + "\n\t\t\t")]) : e._e()])
                }))], 2) : e._e()
            },
            Ct = [function() {
                var e = this,
                    t = e.$createElement,
                    i = e._self._c || t;
                return i("div", {
                    staticClass: "stepper-counter"
                }, [i("div", {
                    staticClass: "circle"
                }), e._v(" "), i("div", {
                    staticClass: "line"
                })])
            }],
            qt = {
                props: {
                    steps: {
                        type: Array
                    },
                    vertical: {
                        type: Boolean
                    }
                }
            },
            St = qt,
            Et = Object(E["a"])(St, _t, Ct, !1, null, null, null),
            Ot = Et.exports;
        class xt {}
        xt.install = function(e, t) {
            e.component("su-form", L), e.component("su-input", F), e.component("su-tag-selector", yt), e.component("su-radio-group", Xe), e.component("su-radio", K), e.component("su-checkbox", Z), e.component("su-button", ne), e.component("su-switch", de), e.component("su-loader", ge), e.component("su-avatar", at), e.component("su-select", He), e.component("su-table", ft), e.component("su-modal", _e), e.component("su-stepper", Ot), e.directive("tooltip", rt)
        };
        const Tt = a["a"].extend(je);
        xt.Toast = class {
            constructor(e) {
                this.eventBus = e, this.component = this.mountComponent(e), this.TOP_LEFT = this.component.orientations.TOP_LEFT, this.TOP_CENTER = this.component.orientations.TOP_CENTER, this.TOP_RIGHT = this.component.orientations.TOP_RIGHT, this.BOTTOM_LEFT = this.component.orientations.BOTTOM_LEFT, this.BOTTOM_CENTER = this.component.orientations.BOTTOM_CENTER, this.BOTTOM_RIGHT = this.component.orientations.BOTTOM_RIGHT
            }
            mountComponent(e) {
                let t = new Tt({
                    propsData: {
                        eventBus: e
                    }
                });
                return t.vm = t.$mount(), document.body.appendChild(t.vm.$el), t
            }
            open(e) {
                this.eventBus.$emit("su-toast", e)
            }
            success(e) {
                e.type = this.component.types.SUCCESS, this.eventBus.$emit("su-toast", e)
            }
            error(e) {
                e.type = this.component.types.ERROR, this.eventBus.$emit("su-toast", e)
            }
            info(e) {
                e.type = this.component.types.INFO, this.eventBus.$emit("su-toast", e)
            }
            warning(e) {
                e.type = this.component.types.WARNING, this.eventBus.$emit("su-toast", e)
            }
            loading(e) {
                e.type = this.component.types.LOADING, this.eventBus.$emit("su-toast", e)
            }
        };
        const $t = a["a"].extend(xe);
        xt.Confirm = class {
            constructor(e) {
                this.eventBus = e, this.component = this.mountComponent(e)
            }
            mountComponent(e) {
                let t = new $t({
                    propsData: {
                        eventBus: e
                    }
                });
                return t.vm = t.$mount(), document.body.appendChild(t.vm.$el), t
            }
            confirm(e) {
                this.eventBus.$emit("su-confirm", e)
            }
        }, o["a"].registerModule(y["b"], y["a"]), a["a"].config.debug = !1, a["a"].config.silent = !0, a["a"].config.performance = !1, a["a"].directive("click-outside", b), a["a"].directive("on-escape", g), a["a"].directive("tour", v["a"]), a["a"].use(p), a["a"].use(f), a["a"].use(xt), a["a"].use(n["a"]);
        let Pt = new l;
        a["a"].prototype.$events = Pt, a["a"].prototype.$clone = e => s()(e), a["a"].prototype.$localStore = k["c"], a["a"].prototype.$toast = new xt.Toast(Pt.eventBus);
        const It = new xt.Confirm(Pt.eventBus);
        a["a"].prototype.$confirm = It.confirm.bind(It);
        let Dt = new w;
        a["a"].prototype.$keyEvents = Dt, new a["a"]({
            router: c["a"],
            store: o["a"],
            render: e => e(x)
        }).$mount("#app")
    },
    "5fb0": function(e, t, i) {
        "use strict";
        i.d(t, "a", (function() {
            return a
        })), i.d(t, "b", (function() {
            return r
        }));
        const a = {
                initial: 0,
                fetching: 1,
                success: 2,
                error: -1
            },
            n = {
                ticker: "kite.ticker",
                ticks: "kite.ticks",
                refetch: "kite.refetch",
                getTicks: "kite.ticks.get",
                reload: "kite.reload",
                masterDisconnect: "kite.masterDisconnect",
                orderEvent: "kite.orderEvent"
            },
            r = {
                refetch: "refetchData:",
                showMarketDepthWidget: "showMarketDepthWidget",
                showStockWidget: "showStockWidget",
                showStocksAnalyzeWidget: "showStocksAnalyzeWidget",
                showTechnicalWidget: "showTechnicalWidget",
                showOptionChain: "showOptionChain",
                showBasketPerfWidget: "showBasketPerfWidget",
                confirm: "confirm"
            };
        t["c"] = {
            API_STATUSES: a,
            CROSSTAB_CHANNELS: n,
            EVENTS: r
        }
    },
    7005: function(e, t, i) {},
    "72e2": function(e, t, i) {
        "use strict";
        i.d(t, "b", (function() {
            return u
        }));
        var a = i("c0d6"),
            n = i("687d"),
            r = i.n(n);
        t["a"] = {
            bind: o,
            unbind: l
        };
        let s = {};

        function o(e, t, i) {
            let n = t.value,
                o = a["a"].state.tour && a["a"].state.tour.tours || {};
            if ("default" === n && o.default) {
                if (c()) return;
                r.a.startTour(s[n])
            } else if ((s[n] && s[n].replacePrevious || !r.a.getCurrTour()) && (r.a.getCurrTour() && r.a.endTour(), o && o[n])) {
                if (c()) return;
                setTimeout(() => {
                    r.a.startTour(s[n])
                }, 300)
            }
        }

        function c() {
            return a["a"].state.isMobile
        }

        function l(e, t, i) {
            let a = t.value,
                n = r.a.getCurrTour();
            n && n.id === a && r.a.endTour()
        }

        function d(e) {
            a["a"].commit("tour/setTour", {
                name: e,
                value: !1
            }), a["a"].dispatch("tour/updateTours")
        }

        function u(e, t) {
            e || (e = "default"), t && (a["a"].commit("tour/clearTours"), a["a"].dispatch("tour/updateTours"));
            let i = a["a"].state.tour && a["a"].state.tour.tours || {};
            i && i[e] && r.a.startTour(s[e])
        }
        s["default"] = {
            id: "default",
            steps: [{
                title: "Marketwatch",
                content: "This is where you add your favorite stocks and instruments you want to monitor.",
                target: ".marketwatch-sidebar",
                placement: "right",
                yOffset: "center",
                width: 400
            }, {
                title: "Universal search",
                content: "Search for any stock on any exchange here. For example, type: infy nse to find the Infosys stock. Once you've found it, click on the '+' to add it to the marketwatch.",
                target: ".marketwatch-sidebar .search-input-field",
                placement: "right",
                width: 400
            }, {
                title: "Marketwatch settings",
                content: "You can customize the display of items in the marketwatch here. For example, sorting the entries, hiding and showing certain values etc.",
                target: ".marketwatch-sidebar .marketwatch-selector .icon-settings",
                placement: "top",
                xOffset: -10,
                width: 400
            }, {
                title: "Multiple views",
                content: "You have 7 different marketwatch views here that you can toggle. Each can have up to 50 entries.",
                target: ".marketwatch-sidebar .marketwatch-selector",
                placement: "top",
                arrowOffset: "center",
                width: 400
            }, {
                title: "Buy, sell, and more",
                content: "Now that you have added a stock to your marketwatch, hover on it to buy, sell, and perform other actions like viewing market information or opening a chart.",
                target: ".marketwatch-sidebar .instrument",
                placement: "right",
                width: 400
            }],
            showPrevButton: !0,
            onShow: function() {},
            onEnd: function() {
                d("default")
            },
            onClose: function() {
                d("default")
            }
        }, s["order"] = {
            id: "order",
            steps: [{
                title: "Order window",
                content: "This is where you can buy and sell stocks. You can choose the desired quantity, price etc. and place the order, which is instantly sent to the exchange. For more information, refer to the <a href='http://kite.trade/docs/kite' target='_blank'>user manual</a>.",
                target: ".order-window-draggable",
                placement: "top",
                fixedElement: !0,
                width: 400
            }, {
                title: "Toggle switch",
                target: ".order-window .transaction-type-switch label",
                content: "Toggle between a buy and sell order by clicking the toggle switch.",
                placement: "top",
                fixedElement: !0,
                yOffset: -10,
                xOffset: -15,
                width: 400
            }, {
                title: "Advanced options",
                content: "There are several advanced options that you can use when buying and selling stocks. For more information, refer to the <a href='http://kite.trade/docs/kite' target='_blank'>user manual</a>.",
                target: ".order-window .advanced-options-toggle",
                placement: "top",
                fixedElement: !0,
                width: 400
            }],
            showPrevButton: !0,
            replacePrevious: !0,
            onShow: function() {},
            onEnd: function() {
                d("order")
            },
            onClose: function() {
                d("order")
            }
        }, s["table"] = {
            id: "table",
            steps: [{
                title: "Table toolbar",
                content: "Use this toolbar to quickly search and filter rows in a table, for instance, an instrument name or a status. Just type and hit enter. Use the icons to view reports or to instantly download the data seen in the table.",
                target: ".toolbar",
                placement: "left",
                width: 400,
                showNextButton: !1,
                showPrevButton: !1,
                showCTAButton: !1
            }],
            onShow: function() {},
            onEnd: function() {
                d("table")
            },
            onClose: function() {
                d("table")
            }
        }, s["chart"] = {
            id: "chart",
            steps: [{
                title: "Quick trade panel",
                content: "Click on this icon to toggle the quick-trade panel. The panel shows your orders, positions, and realtime market depth for the instrument and lets you trade without leaving the chart window.",
                target: ".icon-zap",
                placement: "right",
                width: 400
            }, {
                title: "Chart tools",
                content: "Explore the available charting and analysis tools, and customization options here.",
                placement: "bottom",
                arrowOffset: "center",
                width: 400
            }],
            onStart: function() {
                let e = document.getElementById("chart-iframe"),
                    t = e.contentDocument || e.contentWindow.document,
                    i = t.querySelector(".ciq-menu-section");
                s.chart.steps[1].target = i
            },
            onError: function() {
                r.a.endTour()
            },
            onEnd: function() {
                d("chart")
            },
            onClose: function() {
                d("chart")
            }
        }, s["notification"] = {
            id: "notification",
            steps: [{
                title: "Notifications",
                content: "Order related alerts such as completions and rejections, and important messages that we send will be shown here.",
                target: ".notification-count",
                placement: "left",
                width: 400,
                showNextButton: !1,
                showPrevButton: !1,
                showCTAButton: !1
            }],
            onShow: function() {},
            onEnd: function() {
                d("notification")
            },
            onClose: function() {
                d("notification")
            }
        }, s["marketdepth"] = {
            id: "marketdepth",
            steps: [{
                title: "Market depth",
                content: "The market depth shows you the current bids and offers for an instrument in the stock market. The dynamic bar shows the volume at a particular price point. Clicking on it lets you quickly place an order at that price.",
                target: ".marketwatch-sidebar .market-depth .depth-table",
                placement: "right",
                width: 400,
                showNextButton: !1,
                showPrevButton: !1,
                showCTAButton: !1
            }],
            replacePrevious: !0,
            onShow: function() {},
            onEnd: function() {
                d("marketdepth")
            },
            onClose: function() {
                d("marketdepth")
            }
        }
    },
    b202: function(e, t, i) {
        "use strict";
        i.d(t, "c", (function() {
            return f
        })), i.d(t, "a", (function() {
            return g
        })), i.d(t, "e", (function() {
            return v
        })), i.d(t, "b", (function() {
            return b
        })), i.d(t, "d", (function() {
            return w
        }));
        var a = i("e675"),
            n = i.n(a),
            r = i("0e54"),
            s = i.n(r),
            o = i("add5"),
            c = i.n(o),
            l = i("0ab6"),
            d = i.n(l),
            u = i("ee7c"),
            h = i.n(u),
            m = i("3928"),
            p = i.n(m);
        const f = n.a.createStore([c.a, d.a], [h.a, p.a]).namespace("kite"),
            g = n.a.createStore([s.a], [h.a, p.a]),
            v = n.a.createStore([d.a], [h.a, p.a]);

        function b(e, t, i, a) {
            let n = "";
            i = i || f, e && (n = e + "/"), n += t;
            try {
                let e = i.get(n);
                return void 0 === e ? a : e
            } catch (r) {
                console.log("localstorage error get: ", r)
            }
        }

        function w(e, t, i, a, n) {
            let r = "";
            a = a || f, e && (r = e + "/"), r += t;
            try {
                return a.set(r, i, n), !0
            } catch (s) {
                return console.log("localstorage error set: ", s), !1
            }
        }
    },
    b86f: function(e, t, i) {
        "use strict";
        var a = i("7005"),
            n = i.n(a);
        n.a
    },
    c0d6: function(e, t, i) {
        "use strict";
        var a = i("a026"),
            n = i("2f62"),
            r = i("b202");
        a["a"].use(n["a"]);
        const s = "gpay";
        t["a"] = new n["a"].Store({
            state: {
                isMobile: !1,
                isOnline: !0,
                theme: Object(r["b"])("", "theme") || "default",
                source: Object(r["b"])("", "source") || ""
            },
            getters: {
                isMobile: e => e.isMobile,
                isOnline: e => e.isOnline,
                theme: e => e.theme,
                source: e => e.source,
                isSourceGPay: e => e.source === s
            },
            mutations: {
                setMobile: (e, t) => {
                    e.isMobile = t
                },
                setOnline: (e, t) => {
                    e.isOnline = t
                },
                setTheme: (e, t) => {
                    e.theme = t, Object(r["d"])("", "theme", t)
                },
                setSource: (e, t) => {
                    e.source = t, Object(r["d"])("", "source", t)
                }
            },
            actions: {},
            modules: {},
            strict: !1
        })
    },
    db49: function(e, t, i) {
        "use strict";
        i.d(t, "b", (function() {
            return n
        }));
        const a = [{
                tradingsymbol: "NIFTY 50",
                segment: "INDICES",
                instrumentToken: 256265
            }, {
                tradingsymbol: "SENSEX",
                segment: "INDICES",
                instrumentToken: 265
            }],
            n = {
                alertSound: "/static/audio/pluck",
                logo: "/static/images/kite-logo.svg",
                cobrandLogo: "/static/images/cobrand/{name}-logo.svg",
                desktopNotificationIcon: "/static/images/kite-logo.png"
            };
        t["a"] = {
            pinnedInstruments: a,
            staticAssets: n
        }
    },
    df03: function(e, t, i) {
        "use strict";
        i.d(t, "b", (function() {
            return r
        }));
        var a = i("a026");
        let n = !0;
        const r = "httpStore",
            s = {
                ETags: {}
            },
            o = {
                allETags: e => e.ETags,
                getETag: (e, t) => t => e.ETags[t]
            },
            c = {
                setETag(e, {
                    key: t,
                    ETag: i,
                    response: n
                }) {
                    a["a"].set(e.ETags, t, {
                        ETag: i,
                        response: n
                    })
                }
            },
            l = {};
        t["a"] = {
            state: s,
            getters: o,
            mutations: c,
            actions: l,
            namespaced: n
        }
    },
    f1f7: function(e, t, i) {
        "use strict";
        i.d(t, "b", (function() {
            return n
        })), i.d(t, "c", (function() {
            return s
        }));
        let a = (e, t) => parseFloat(e).toFixed(t),
            n = (e, t, i, a) => {
                let n = t || 2,
                    r = parseFloat(e).toFixed(n).replace(/(\d)(?=(\d{2})+\d\.)/g, "$1,");
                return 0 === t && (r = r.split(".")[0]), a && (r = "₹" + r), r
            },
            r = (e, t) => {
                let i = [{
                    value: 1e7,
                    symbol: "Cr"
                }, {
                    value: 1e5,
                    symbol: "L"
                }, {
                    value: 1e3,
                    symbol: "k"
                }];
                t = t || 2, e = Math.round(e * Math.pow(10, t)) / Math.pow(10, t);
                for (let a = 0; a < i.length; a++)
                    if (Math.abs(e) >= i[a].value) return (e / i[a].value * 100 / 100).toFixed(2).replace(/\.0+$|(\.[0-9]*[1-9])0+$/, "$1") + i[a].symbol;
                return e
            },
            s = (e, t) => {
                var i = Math.pow(10, t),
                    a = e * i,
                    n = Math.round(a);
                return n / i
            };
        t["a"] = {
            decimalPad: a,
            inrFormat: n,
            kformat: r,
            round: s
        }
    }
});