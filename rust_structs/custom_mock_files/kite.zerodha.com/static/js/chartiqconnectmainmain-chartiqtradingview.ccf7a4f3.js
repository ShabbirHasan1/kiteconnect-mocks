(window["webpackJsonp"] = window["webpackJsonp"] || []).push([
    ["chartiq~connect~main~main-chartiq~tradingview"], {
        "0e50": function(e, t, r) {
            "use strict";
            var s = r("3da7"),
                n = r("d9d2"),
                i = r("025e"),
                a = r("b202");

            function o(e) {
                let t = [],
                    r = [];
                for (let s of e) - 1 !== s.status.indexOf("COMPLETE") || -1 !== s.status.indexOf("REJECT") || -1 !== s.status.indexOf("CANCEL") || -1 !== s.status.indexOf("LAPSED") ? r.push(s) : t.push(s);
                return {
                    pendingOrders: t,
                    completedOrders: r
                }
            }
            r.d(t, "b", (function() {
                return u
            }));
            let l = !0;
            const u = n["a"],
                c = {
                    orders: null,
                    ordersError: null,
                    pendingOrders: null,
                    completedOrders: null,
                    orderPrefs: Object(a["b"])(u, "orderPrefs") || {},
                    trades: null,
                    tradesError: null,
                    orderFetchStatus: 0,
                    tradeFetchStatus: 0,
                    gtt: null,
                    gttError: null,
                    gttFetchStatus: 0,
                    ordersNotificationCount: parseInt(Object(a["b"])(u, "ordersNotificationCount") || 0)
                },
                h = {
                    orders: e => e.orders,
                    ordersError: e => e.ordersError,
                    pendingOrders: e => e.pendingOrders,
                    completedOrders: e => e.completedOrders,
                    orderPrefs: e => e.orderPrefs,
                    ordersNotificationCount: e => e.ordersNotificationCount,
                    trades: e => e.trades,
                    tradesError: e => e.tradesError,
                    orderFetchStatus: e => e.orderFetchStatus,
                    tradeFetchStatus: e => e.tradeFetchStatus,
                    gtt: e => e.gtt,
                    gttError: e => e.gttError,
                    gttFetchStatus: e => e.gttFetchStatus
                },
                d = {
                    setOrders(e, t) {
                        e.orders = t;
                        let {
                            pendingOrders: r,
                            completedOrders: s
                        } = o(t);
                        e.pendingOrders = r, e.completedOrders = s
                    },
                    setOrdersError(e, t) {
                        e.ordersError = t
                    },
                    setOrderPrefs(e, t) {
                        e.orderPrefs = t, Object(a["d"])(u, "orderPrefs", t)
                    },
                    setTrades(e, t) {
                        e.trades = t
                    },
                    setTradesError(e, t) {
                        e.tradesError = t
                    },
                    setOrderFetchStatus(e, t) {
                        e.orderFetchStatus = t
                    },
                    setTradeFetchStatus(e, t) {
                        e.tradeFetchStatus = t
                    },
                    setGTT(e, t) {
                        e.gtt = t
                    },
                    setGTTError(e, t) {
                        e.gttError = t
                    },
                    setGTTFetchStatus(e, t) {
                        e.gttFetchStatus = t
                    },
                    setOrderNotificationsCount(e, t) {
                        e.ordersNotificationCount = t, Object(a["d"])(u, "ordersNotificationCount", e.ordersNotificationCount, null, Object(i["j"])())
                    },
                    incOrderNotificationsCount(e, t) {
                        e.ordersNotificationCount += t, Object(a["d"])(u, "ordersNotificationCount", e.ordersNotificationCount, null, Object(i["j"])())
                    }
                },
                m = {
                    fetchOrders({
                        commit: e,
                        state: t
                    }) {
                        e("setOrderFetchStatus", 1);
                        let r = s["a"].getOrders();
                        Object(i["g"])({
                            commit: e,
                            apiPromise: r,
                            data: "setOrders",
                            error: "setOrdersError",
                            status: "setOrderFetchStatus"
                        })
                    },
                    fetchTrades({
                        commit: e,
                        state: t
                    }) {
                        e("setTradeFetchStatus", 1);
                        let r = s["a"].getTrades();
                        Object(i["g"])({
                            commit: e,
                            apiPromise: r,
                            data: "setTrades",
                            error: "setTradesError",
                            status: "setTradeFetchStatus"
                        })
                    },
                    fetchGTT({
                        commit: e,
                        state: t
                    }) {
                        e("setGTTFetchStatus", 1);
                        let r = s["a"].getGTT();
                        Object(i["g"])({
                            commit: e,
                            apiPromise: r,
                            data: "setGTT",
                            error: "setGTTError",
                            status: "setGTTFetchStatus"
                        })
                    }
                };
            t["a"] = {
                state: c,
                getters: h,
                mutations: d,
                actions: m,
                namespaced: l
            }
        },
        "3da7": function(e, t, r) {
            "use strict";
            var s = r("ba6a"),
                n = r("5665");

            function i(e) {
                return e ? s["a"].get(Object(n["a"])("orders.info", {
                    orderId: e
                })) : s["a"].get(Object(n["a"])("orders.all"))
            }

            function a() {
                return s["a"].get(Object(n["a"])("trades.all"))
            }

            function o(e) {
                return s["a"].get(Object(n["a"])("orders.trades", {
                    orderId: e
                }))
            }

            function l(e) {
                return s["a"].post(Object(n["a"])("orders.place", {
                    variety: e.variety
                }), e)
            }

            function u(e, t) {
                return s["a"].put(Object(n["a"])("orders.modify", {
                    orderId: e,
                    variety: t.variety
                }), t)
            }

            function c(e, t) {
                return s["a"].delete(Object(n["a"])("orders.cancel", {
                    orderId: e,
                    variety: t.variety
                }), {
                    params: t
                })
            }

            function h(e, t) {
                return s["a"].get(Object(n["a"])("orders.triggerRange", {
                    transactionType: e
                }), {
                    params: t
                })
            }

            function d(e) {
                return s["a"].post(Object(n["a"])("gtt_orders.place"), e)
            }

            function m(e, t) {
                return s["a"].put(Object(n["a"])("gtt_orders.modify", {
                    orderId: e
                }), t)
            }

            function p(e, t) {
                return s["a"].delete(Object(n["a"])("gtt_orders.cancel", {
                    orderId: e
                }), {
                    params: t
                })
            }

            function g(e) {
                return e ? s["a"].get(Object(n["a"])("gtt_orders.info", {
                    orderId: e
                })) : s["a"].get(Object(n["a"])("gtt_orders.all"))
            }

            function f(e) {
                return s["a"].post(Object(n["a"])("nudge.orders"), e, {
                    headers: {
                        "Content-Type": "application/json"
                    }
                })
            }

            function y(e) {
                return s["a"].post(Object(n["a"])("margins.orders"), e, {
                    headers: {
                        "Content-Type": "application/json"
                    }
                })
            }
            t["a"] = {
                getOrders: i,
                getTrades: a,
                getOrderTrades: o,
                placeOrder: l,
                cancelOrder: c,
                modifyOrder: u,
                getTriggerRange: h,
                placeGTT: d,
                modifyGTT: m,
                deleteGTT: p,
                getGTT: g,
                checkOrdersNudge: f,
                fetchOrderMargins: y
            }
        },
        6825: function(e, t, r) {
            "use strict";
            var s = function() {
                    var e = this,
                        t = e.$createElement,
                        r = e._self._c || t;
                    return r("div", {
                        directives: [{
                            name: "click-outside",
                            rawName: "v-click-outside",
                            value: e.hide,
                            expression: "hide"
                        }, {
                            name: "on-escape",
                            rawName: "v-on-escape",
                            value: e.hide,
                            expression: "hide"
                        }],
                        staticClass: "context-menu",
                        attrs: {
                            id: "context-menu-" + e._uid
                        }
                    }, [r("div", {
                        staticClass: "context-menu-button-wrap",
                        on: {
                            click: function(t) {
                                return t.stopPropagation(), t.preventDefault(), e.handleButtonClick(t)
                            }
                        }
                    }, [e._t("button")], 2), e._v(" "), r("transition", {
                        attrs: {
                            name: "dropdown"
                        }
                    }, [e.showMenu ? r("ul", {
                        staticClass: "context-menu-list list-flat layer-2"
                    }, [e._t("menu")], 2) : e._e()])], 1)
                },
                n = [],
                i = {
                    name: "mobile-context-menu",
                    props: {
                        orientation: {
                            type: String,
                            default: "left"
                        },
                        marginTop: {
                            type: Number,
                            default: 8
                        },
                        parentSelector: String
                    },
                    data() {
                        return {
                            showMenu: !1
                        }
                    },
                    deactivated() {
                        this.hide(), this.$emit("delete")
                    },
                    methods: {
                        hide() {
                            this.showMenu && (this.showMenu = !1)
                        },
                        handleButtonClick(e) {
                            this.showMenu ? this.showMenu = !1 : (this.showMenu = !0, this.$nextTick(() => {
                                let e = 0;
                                if (this.parentSelector) {
                                    let t = document.querySelector(this.parentSelector);
                                    e = t && t.scrollTop
                                } else e = window.pageYOffset || document.documentElement.scrollTop;
                                let t, r = this.$el.querySelector(".context-menu-list"),
                                    s = r.getBoundingClientRect(),
                                    n = this.$el.querySelector(".context-menu-button").getBoundingClientRect(),
                                    i = 0;
                                s.height + s.top - e <= window.innerHeight ? (t = "bottom", i = n.top + n.height + this.marginTop) : (t = "top", i = n.top - s.height - this.marginTop), i >= 0 ? r.style.top = i + "px" : (r.style.top = "0px", t = null), "bottom" === t ? r.classList.add("position-bottom") : "top" === t && r.classList.add("position-top");
                                let a = 0;
                                "left" === this.orientation ? a = n.left + n.width - s.width : "right" === this.orientation && (a = n.left + n.width), r.style.left = t ? a + "px" : a - n.width + "px"
                            }))
                        }
                    }
                },
                a = i,
                o = r("2877"),
                l = Object(o["a"])(a, s, n, !1, null, null, null);
            t["a"] = l.exports
        },
        "714b": function(e, t, r) {
            "use strict";
            var s = function() {
                    var e = this,
                        t = e.$createElement,
                        r = e._self._c || t;
                    return r("transition", {
                        attrs: {
                            name: "modal"
                        }
                    }, [r("div", {
                        staticClass: "modal-mask"
                    }, [r("div", {
                        staticClass: "modal-wrapper"
                    }, [r("div", {
                        staticClass: "modal-container layer-2"
                    }, [r("div", {
                        staticClass: "modal-header"
                    }, [e._t("header")], 2), e._v(" "), r("div", {
                        staticClass: "modal-body"
                    }, [e._t("body")], 2), e._v(" "), r("div", {
                        staticClass: "modal-footer"
                    }, [e._t("footer")], 2)])])])])
                },
                n = [],
                i = r("2f62");

            function a(e, t) {
                var r = Object.keys(e);
                if (Object.getOwnPropertySymbols) {
                    var s = Object.getOwnPropertySymbols(e);
                    t && (s = s.filter((function(t) {
                        return Object.getOwnPropertyDescriptor(e, t).enumerable
                    }))), r.push.apply(r, s)
                }
                return r
            }

            function o(e) {
                for (var t = 1; t < arguments.length; t++) {
                    var r = null != arguments[t] ? arguments[t] : {};
                    t % 2 ? a(Object(r), !0).forEach((function(t) {
                        l(e, t, r[t])
                    })) : Object.getOwnPropertyDescriptors ? Object.defineProperties(e, Object.getOwnPropertyDescriptors(r)) : a(Object(r)).forEach((function(t) {
                        Object.defineProperty(e, t, Object.getOwnPropertyDescriptor(r, t))
                    }))
                }
                return e
            }

            function l(e, t, r) {
                return t in e ? Object.defineProperty(e, t, {
                    value: r,
                    enumerable: !0,
                    configurable: !0,
                    writable: !0
                }) : e[t] = r, e
            }
            var u = {
                    name: "modal",
                    props: {
                        closeOnClick: {
                            type: Boolean,
                            default: !0
                        },
                        zIndex: {
                            type: Number,
                            default: 15
                        }
                    },
                    computed: o({}, Object(i["c"])(["isMobile"])),
                    data() {
                        return {
                            keyboardShortcutEvents: [{
                                keys: ["27"],
                                cb: this.close,
                                stop: !0,
                                prevent: !0
                            }]
                        }
                    },
                    mounted() {
                        this.$nextTick(() => {
                            this.closeOnClick && window.addEventListener("mousedown", this.closeOnDocumentClick), this.$el.style.zIndex = this.zIndex, document.body.classList.add("modal-open")
                        })
                    },
                    beforeDestroy() {
                        window.removeEventListener("mousedown", this.closeOnDocumentClick)
                    },
                    methods: {
                        closeOnDocumentClick(e) {
                            e.target === this.$el.querySelector(".modal-wrapper") && this.close()
                        },
                        close() {
                            this.$emit("close"), document.body.classList.remove("modal-open")
                        }
                    }
                },
                c = u,
                h = r("2877"),
                d = Object(h["a"])(c, s, n, !1, null, null, null);
            t["a"] = d.exports
        },
        "808c": function(e, t, r) {
            "use strict";

            function s(e, t, r) {
                return t in e ? Object.defineProperty(e, t, {
                    value: r,
                    enumerable: !0,
                    configurable: !0,
                    writable: !0
                }) : e[t] = r, e
            }
            class n {
                constructor() {
                    s(this, "regSymbol", RegExp(/(.+?)((-EQ)|([0-9]{2})(([A-Z]{3})|(([0-9OND])([0-9]{2})))(FUT|([0-9.]+)(CE|PE)(W.)?))/i)), s(this, "regWeeklyExpiry", RegExp(/([0-9]{2})(([A-Z]{3})|(([0-9OND])([0-9]{2})))/i)), this.maxResults = 25, this.lastResults = null, this.currentYear = (new Date).getFullYear(), this.weeklyMonthsMap = {
                        1: "JAN",
                        2: "FEB",
                        3: "MAR",
                        4: "APR",
                        5: "MAY",
                        6: "JUN",
                        7: "JUL",
                        8: "AUG",
                        9: "SEP",
                        O: "OCT",
                        N: "NOV",
                        D: "DEC"
                    }, this.strikePrecision = {}, this.eventsInstruments = {}, this.defaultTickSize = {
                        "MCX-OPT": 1,
                        "MCX-FUT": 1,
                        MCX: 1
                    }
                }
                init(e) {
                    this.months = e.months, this.weeklyMonthsMap = e.weekly_months, this.segments = this.arrayToSet(e.segments), this.segmentsList = e.segments, this.equitySegments = this.arrayToSet(e.equity_segments), this.optionsSegments = this.arrayToSet(e.options_segments), this.futuresSegments = this.arrayToSet(e.futures_segments), this.exchangeSegments = this.arrayToSet(e.exchange_segments), this.tradeableSegments = this.arrayToSet(e.tradeable_segments), this.segmentsID = e.segments_id_map, this.segmentsAliases = e.segments_aliases, this.segmentsExchangeMap = e.segments_exchange_map, this.eventsInstruments = Object.assign({}, this.arrayToMap(e.events)), this.instrumentsMap = {}, this.instrumentsArray = {}, this.equitySymbolMap = [];
                    for (let t of e.equity_segments) this.buildEquitySymbolMap(t, e.instruments[t]);
                    for (let t of e.segments) e.instruments[t] && this.feed(t, e.instruments[t])
                }
                buildEquitySymbolMap(e, t) {
                    for (let r of t) {
                        let t = r[5] || r[1];
                        this.equitySymbolMap[t] || (this.equitySymbolMap[t] = []), this.equitySymbolMap[t].push([e, r[1]])
                    }
                }
                feed(e, t) {
                    this.instrumentsMap[e] = {}, this.instrumentsArray[e] = [], this.equitySegments.has(e) || "INDICES" === e ? this.loadEquity(e, t) : this.optionsSegments.has(e) ? this.loadOptions(e, t) : this.futuresSegments.has(e) ? this.loadFutures(e, t) : console.log("skip loading segment: ", e)
                }
                search(e, t) {
                    t || (t = this.maxResults);
                    let r = this.tokenize(e),
                        s = this.searchSegments(r),
                        n = this.subtract(r, s.matchedTokens),
                        i = this.rankSegments(s.results);
                    0 === i.length && (i = this.allSegements());
                    let a = this.searchSymbols(i, n, e, t);
                    if (a.length > 0) this.lastResults = a;
                    else if (this.lastResults) return this.lastResults;
                    return a
                }
                get(e, t, r) {
                    if (e = e.toUpperCase(), -1 !== e.indexOf("-EQ") && (e = e.split("-EQ")[0]), r && !t && (t = this.getSegment(e, r)), e && t && this.instrumentsMap[t]) {
                        let r = this.instrumentsMap[t][e];
                        if (r) {
                            let e = this.makeInstrument({
                                exchangeToken: r[0],
                                tradingsymbol: r[1],
                                segment: t,
                                exchange: this.segmentsExchangeMap[t],
                                tickSize: r[2],
                                lotSize: r[3],
                                company: r[4] || "",
                                isin: r[5] || ""
                            });
                            return e.isFound = !0, e
                        }
                    }
                    let s = this.makeInstrument({
                        exchangeToken: 0,
                        tradingsymbol: e,
                        segment: t || r,
                        exchange: r || t,
                        tickSize: this.defaultTickSize[r || t] || .05,
                        lotSize: 1,
                        company: "",
                        isin: ""
                    });
                    return s.isFound = !1, s
                }
                getSegment(e, t) {
                    if (this.exchangeSegments.has(t)) return t;
                    let r = this.regSymbol.exec(e);
                    return r ? "FUT" === r[10] ? t + "-FUT" : r[11] && r[12] ? t + "-OPT" : null : null
                }
                loadEquity(e, t) {
                    let r = [],
                        s = {},
                        n = -1,
                        i = -1,
                        a = -1;
                    for (let o of t) {
                        let e, t, l, u = o[1],
                            c = o[2];
                        e = -1 === n ? o[0] : n + o[0], n = e, o[3] ? (t = o[3], i = t) : t = i, o[4] ? (l = o[4], a = l) : l = a;
                        let h = [e, u, t, l, c, o[5]];
                        r.push(h), s[u] = h
                    }
                    this.instrumentsArray[e] = r, this.instrumentsMap[e] = s
                }
                loadOptions(e, t) {
                    let r = [],
                        s = {};
                    for (let n = 0; n < t.length; n++) {
                        let i = t[n],
                            a = i[0],
                            o = i[1],
                            l = i[2];
                        for (let t = 0; t < i[3].length; t++) {
                            let n = i[3][t][0],
                                u = i[3][t][1];
                            for (let t in u)
                                if (u.hasOwnProperty(t)) {
                                    let i = u[t],
                                        c = -1,
                                        h = -1;
                                    for (let u = 0; u < i.length; u++) {
                                        let d, m, p; - 1 === c && -1 === h ? (m = i[u][0], d = i[u][1]) : (m = c + i[u][0], d = Math.round(h * Math.pow(10, 9) + i[u][1] * Math.pow(10, 9)) / Math.pow(10, 9)), h = d, c = m, p = i[u][2] ? i[u][2] : l;
                                        let g = "";
                                        g = this.strikePrecision[e] ? a + n + parseFloat(d).toFixed(this.strikePrecision[e]) + t : a + n + d + t;
                                        let f = this.expandExpiryString(n),
                                            y = f !== n ? f : null,
                                            b = [m, g, o, p, y];
                                        r.push(b), s[b[1]] = b
                                    }
                                }
                        }
                    }
                    this.instrumentsArray[e] = r, this.instrumentsMap[e] = s
                }
                loadFutures(e, t) {
                    let r = [],
                        s = {};
                    for (let n = 0; n < t.length; n++) {
                        let e = t[n][0],
                            i = -1,
                            a = t[n][1],
                            o = t[n][2];
                        for (let l = 0; l < t[n][3].length; l++) {
                            let u, c, h;
                            h = -1 === i ? t[n][3][l][0] : i + t[n][3][l][0], i = h, u = t[n][3][l][1], c = t[n][3][l][2] ? t[n][3][l][2] : o;
                            let d = this.expandExpiryString(u),
                                m = d !== u ? d : null,
                                p = [h, e + u + "FUT", a, c, m];
                            r.push(p), s[p[1]] = p
                        }
                    }
                    this.instrumentsArray[e] = r, this.instrumentsMap[e] = s
                }
                searchSymbols(e, t, r, s) {
                    let n = [];
                    for (let i of this.segmentsList) {
                        if (!this.instrumentsArray[i]) continue;
                        let s = -1 !== e.indexOf(i);
                        for (let e = 0; e < this.instrumentsArray[i].length; e++) {
                            let a = 0,
                                o = !0;
                            if (this.instrumentsArray[i][e][1] === r.toUpperCase()) a = -100;
                            else if (s)
                                for (let r = 0; r < t.length; r++) {
                                    let s = this.instrumentsArray[i][e][1].indexOf(t[r]),
                                        n = -1;
                                    if (this.instrumentsArray[i][e][4] && this.instrumentsArray[i][e].length > 4 && (n = this.instrumentsArray[i][e][4].indexOf(t[r]), n >= 0 && (n += 1)), -1 === s && -1 === n) {
                                        o = !1;
                                        break
                                    }
                                    0 === s ? a -= 2 : n && (a -= 1)
                                } else o = !1;
                            o && n.push([i, this.instrumentsArray[i][e], a, n.length])
                        }
                    }
                    return this.formatResults(n, s)
                }
                formatResults(e, t) {
                    e.sort((function(e, t) {
                        return e[2] === t[2] ? e[3] - t[3] : e[2] < t[2] ? -1 : 1
                    })), e = e.slice(0, t);
                    let r = [];
                    for (let s = 0; s < e.length; s++) {
                        let t = e[s][1],
                            n = e[s][0];
                        r.push(this.makeInstrument({
                            exchangeToken: t[0],
                            tradingsymbol: t[1],
                            segment: n,
                            exchange: this.segmentsExchangeMap[n],
                            tickSize: t[2],
                            lotSize: t[3],
                            company: t[4] || "",
                            isin: t[5] || ""
                        }))
                    }
                    return r
                }
                allSegements() {
                    let e = [];
                    return this.segments.forEach(t => e.push(t)), e
                }
                rankSegments(e) {
                    let t = [],
                        r = [];
                    for (let a in e) e.hasOwnProperty(a) && (t.push(a), r.push(e[a]));
                    let s = [],
                        n = -1,
                        i = -1;
                    while (1) {
                        if (n = r.indexOf(Math.max.apply(Math, r)), -1 === n) break;
                        let e = r[n];
                        if (r[n] = 0, e < i) break;
                        i = e, s.push(t[n])
                    }
                    return s
                }
                searchSegments(e) {
                    let t = {},
                        r = [];
                    for (let s of this.segmentsList)
                        for (let n = 0; n < e.length; n++) - 1 !== this.segmentsAliases[s].indexOf(e[n]) && (r.push(e[n]), t.hasOwnProperty(s) || (t[s] = 0), t[s]++);
                    return {
                        results: t,
                        matchedTokens: this.unique(r)
                    }
                }
                tokenize(e) {
                    e = this.trim(e).toUpperCase(), e = this.trim(e.replace(/[^a-z0-9.\s&]/gi, " ")), e = e.replace(/[\s+]/gi, " ");
                    let t = e.split(" ");
                    return this.unique(t)
                }
                trim(e) {
                    return "undefined" === typeof String.prototype.trim ? e.replace(/^\s+|\s+$/gm, "") : e.trim()
                }
                unique(e) {
                    return e.sort().filter((function(t, r) {
                        return !r || t !== e[r - 1]
                    }))
                }
                subtract(e, t) {
                    let r = [];
                    for (let s = 0; s < e.length; s++) - 1 === t.indexOf(e[s]) && r.push(e[s]);
                    return r
                }
                arrayToSet(e) {
                    let t = new Set;
                    return e.forEach(e => t.add(e)), t
                }
                makeInstrument({
                    exchangeToken: e,
                    tradingsymbol: t,
                    segment: r,
                    exchange: s,
                    tickSize: n,
                    lotSize: i,
                    company: a,
                    isin: o,
                    ignoreRelated: l
                }) {
                    let u = this.parse(t, s);
                    if (u.segment = r, u.exchange = s, u.tickSize = n, u.lotSize = i, u.company = a, u.tradable = this.tradeableSegments.has(r), u.precision = "CDS" === s || "BCD" === s ? 4 : 2, u.fullName = this.getFullName(u), u.niceName = this.getNiceName(u), u.stockWidget = this.isStockWidget(u), u.exchangeToken = e, u.instrumentToken = (e << 8) + this.segmentsID[r], u.isin = o, u.related = [], u.underlying = this.getUnderlyingInstrument(u), !l) {
                        let e = this.equitySymbolMap[o || t];
                        if (e)
                            for (let t of e)
                                if (t[0] !== r) {
                                    let e = this.instrumentsMap[t[0]][t[1]];
                                    if (!e) break;
                                    u.related.push(this.makeInstrument({
                                        exchangeToken: e[0],
                                        tradingsymbol: e[1],
                                        segment: t[0],
                                        exchange: this.segmentsExchangeMap[t[0]],
                                        tickSize: e[2],
                                        lotSize: e[3],
                                        company: e[4] || "",
                                        isin: e[5] || "",
                                        ignoreRelated: !0
                                    }))
                                }
                    }
                    if (!a && u.related.length > 0)
                        for (let c of u.related)
                            if (c.company) {
                                u.company = c.company;
                                break
                            }
                    return u.isEvent = ("NSE" === s || "BSE" === s || "INDICES" === s) && this.eventsInstruments[t], u.isWeekly = u.expiryWeek > 0, u
                }
                getUnderlyingInstrument(e) {
                    if (e && !this.IsEquity(e.exchange) && e.symbol) {
                        let t = e.symbol,
                            r = e.exchange;
                        if ("NFO" === e.exchange) r = "NSE";
                        else {
                            if ("BFO" !== e.exchange) return null;
                            r = "BSE"
                        }
                        "BANKNIFTY" === t ? (t = "NIFTY BANK", r = "INDICES") : "NIFTY" === t ? (t = "NIFTY 50", r = "INDICES") : "FINNIFTY" === t ? (t = "NIFTY FIN SERVICE", r = "INDICES") : "MIDCPNIFTY" === t && (t = "NIFTY MID SELECT", r = "INDICES");
                        let s = this.get(t, null, r);
                        return s && s.exchangeToken ? s : null
                    }
                    return null
                }
                getFullName(e) {
                    return "BSE" === e.exchange ? e.symbol + " (" + e.tradingsymbol + ")" : e.tradingsymbol
                }
                getNiceName(e) {
                    if ("EQ" === e.type) return e.tradingsymbol;
                    let t = e.symbol;
                    return e.expiryDay && (t += " " + e.expiryDay + this.dateSuffix(e.expiryDay)), e.expiryMonth && e.expiryYear && (t += " " + (e.expiryYear !== this.currentYear ? e.expiryYear.toString().substr(2, 4) : "") + this.months[e.expiryMonth - 1]), "OPT" === e.type ? t += " " + e.strike + " " + e.optionType : "FUT" === e.type && (t += " " + e.type), t
                }
                isStockWidget(e) {
                    return this.equitySegments.has(e.exchange)
                }
                IsEquity(e) {
                    return this.equitySegments.has(e) || "INDICES" === e
                }
                extractEqSymbol(e) {
                    return e.length > 3 && "-" === e[e.length - 3] ? e.slice(0, -3) : e
                }
                parse(e, t) {
                    let r = "";
                    if (-1 !== e.indexOf("|") && "BSE" === t) {
                        var s = e.split("|");
                        e = s[0], r = s[1]
                    }
                    let n = {};
                    n.tradingsymbol = e, n.scripCode = r;
                    let i = this.regSymbol.exec(e);
                    if (!i) return n.type = "EQ", n.symbol = this.extractEqSymbol(e), n;
                    if (n.symbol = i[1], "-EQ" === i[2] && (n.type = "EQ"), "FUT" === i[10] ? n.type = "FUT" : i[11] && i[12] && (n.type = "OPT", n.optionType = i[12], n.strike = parseFloat(i[11])), i[4] && i[8] && i[9] && this.weeklyMonthsMap[i[8]]) {
                        let e = this.weeklyMonthsMap[i[8]];
                        n.expiryMonth = this.months.indexOf(e.toUpperCase()) + 1, n.expiryYear = parseInt(i[4]) + 2e3, n.expiryDay = parseInt(i[9]), n.expiryWeek = Math.floor(parseInt(i[9]) / 7) + 1
                    } else i[4] && i[5] && (n.expiryYear = parseInt(i[4]) + 2e3, n.expiryMonth = this.months.indexOf(i[5].toUpperCase()) + 1);
                    return n
                }
                expandExpiryString(e) {
                    const t = this.regWeeklyExpiry.exec(e);
                    return t && t[5] && t[6] ? t[6] + " " + this.weeklyMonthsMap[t[5]] + " WEEKLY" : e
                }
                dateSuffix(e) {
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
                arrayToMap(e, t) {
                    if (!e) return {};
                    t || (t = "");
                    let r = {};
                    for (let s of e) r[t + s] = !0;
                    return r
                }
            }
            var i = n;
            r.d(t, "a", (function() {
                return a
            }));
            class a {}
            a.InstrumentManager = i, a.install = function(e, t) {}
        },
        aada: function(e, t, r) {
            "use strict";
            var s = function() {
                    var e = this,
                        t = e.$createElement,
                        r = e._self._c || t;
                    return r("div", {
                        staticClass: "empty-state"
                    }, [e._t("header"), e._v(" "), e.image ? r("img", {
                        staticClass: "empty-img",
                        attrs: {
                            src: e.image
                        }
                    }) : e._e(), e._v(" "), e._t("message"), e._v(" "), e.showOmnisearch ? r("su-button", {
                        class: e.omnisearchClass,
                        nativeOn: {
                            click: function(t) {
                                return t.preventDefault(), t.stopPropagation(), e.showMarketDepthWidget(t)
                            }
                        }
                    }, [e._v(e._s(e.omnisearchActionTitle))]) : e._e(), e._v(" "), e._t("action"), e._v(" "), e._t("footer")], 2)
                },
                n = [],
                i = r("5fb0"),
                a = {
                    name: "empty-state",
                    props: {
                        image: String,
                        showOmnisearch: Boolean,
                        omnisearchActionTitle: {
                            type: String,
                            default: "Get started"
                        },
                        omnisearchClass: {
                            type: String,
                            default: "button-blue"
                        }
                    },
                    data() {
                        return {}
                    },
                    methods: {
                        showMarketDepthWidget() {
                            this.$events.emit(i["c"].EVENTS.showMarketDepthWidget)
                        }
                    }
                },
                o = a,
                l = r("2877"),
                u = Object(l["a"])(o, s, n, !1, null, null, null);
            t["a"] = u.exports
        },
        da42: function(e, t, r) {
            "use strict";
            r.d(t, "a", (function() {
                return a
            }));
            var s = r("3da7"),
                n = r("0e50"),
                i = r("d9d2");
            class a {}
            a.api = s["a"], a.constants = i["b"], a.events = i["b"].EVENTS, a.namespace = i["b"].NAMESPACE, a.install = function(e, t) {
                t.store && t.store.registerModule(i["b"].NAMESPACE, n["a"])
            }
        }
    }
]);