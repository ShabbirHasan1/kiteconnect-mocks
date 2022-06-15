(window["webpackJsonp"] = window["webpackJsonp"] || []).push([
    ["chartiq~connect~main~tradingview"], {
        "0d07": function(t, e, r) {
            "use strict";
            var s = r("ba6a"),
                i = r("5665");

            function n() {
                return s["a"].get(Object(i["a"])("portfolio.holdings.all"))
            }

            function a(t) {
                return s["a"].get(Object(i["a"])("portfolio.holdings.app", {
                    appName: t
                }))
            }

            function o(t) {
                return s["a"].post(Object(i["a"])("portfolio.holdings.createAuthorise"), t)
            }

            function l(t, e) {
                return s["a"].get(Object(i["a"])("portfolio.holdings.authorise", {
                    apiKey: t,
                    reqID: e
                }))
            }

            function c(t, e, r) {
                return s["a"].post(Object(i["a"])("portfolio.holdings.authorise", {
                    apiKey: t,
                    reqID: e
                }), r)
            }

            function u(t, e) {
                return s["a"].post(Object(i["a"])("portfolio.holdings.sendPin", {
                    apiKey: t,
                    reqID: e
                }))
            }
            e["a"] = {
                getHoldings: n,
                getAppHoldings: a,
                getHoldingsAuthSession: l,
                makeHoldingsAuthPayload: c,
                createHoldingsAuthSession: o,
                sendHoldingsAuthPIN: u
            }
        },
        "100e": function(t, e, r) {
            var s = r("cd9d"),
                i = r("2286"),
                n = r("c1c9");

            function a(t, e) {
                return n(i(t, e, s), t + "")
            }
            t.exports = a
        },
        1737: function(t, e, r) {
            "use strict";
            var s = function() {
                    var t = this,
                        e = t.$createElement,
                        r = t._self._c || e;
                    return r("modal", {
                        directives: [{
                            name: "on-escape",
                            rawName: "v-on-escape",
                            value: t.close,
                            expression: "close"
                        }],
                        staticClass: "holdings-auth-confirm-modal",
                        on: {
                            close: t.close
                        }
                    }, [r("h2", {
                        attrs: {
                            slot: "header"
                        },
                        slot: "header"
                    }, [t._v("Authorisation")]), t._v(" "), r("div", {
                        attrs: {
                            slot: "body"
                        },
                        slot: "body"
                    }, [t.isAuthFinished ? r("div", ["success" === t.authStatus ? r("p", [t._v("\n\t\t\t\tDone. You can now execute your sell transaction.\n\t\t\t")]) : t.authError ? r("p", [t._v("\n\t\t\t\t" + t._s(t.authError) + "\n\t\t\t")]) : r("p", [t._v("\n\t\t\t\tSomething went wrong while authorising holdings. Please try again.\n\t\t\t")])]) : r("div", [t.createSessInProgress ? r("div", {
                        staticClass: "text-center"
                    }, [t.createSessInProgress ? r("su-loader") : t._e()], 1) : t._e(), t._v(" "), t.createSessError && !t.createSessInProgress ? r("div", [r("p", [t._v(t._s(t.createSessError.message) + " (" + t._s(t.createSessError.error_type) + ")")])]) : t._e(), t._v(" "), t.createSessFinished && t.reqID ? r("div", [r("p", [t._v("Authorise your holdings at CDSL (your Demat depository) to execute sell transactions. "), r("a", {
                        attrs: {
                            href: "https://support.zerodha.com/category/trading-and-markets/trading-faqs/articles/generate-cdsl-tpin",
                            target: "_blank"
                        }
                    }, [t._v("Learn more.")])])]) : t._e(), t._v(" "), t.canSkip ? r("div", {
                        staticClass: "text-box blue no-border"
                    }, [r("a", {
                        staticClass: "text-bold",
                        attrs: {
                            href: ""
                        },
                        on: {
                            click: function(e) {
                                return e.preventDefault(), t.skip(e)
                            }
                        }
                    }, [t._v("Skip for now")]), t._v(" and authorise any time before 5:00 PM.\n\t\t\t")]) : t._e()])]), t._v(" "), r("div", {
                        staticClass: "actions",
                        attrs: {
                            slot: "footer"
                        },
                        slot: "footer"
                    }, [t.canRetry ? r("su-button", {
                        staticClass: "button-blue",
                        attrs: {
                            processing: t.createSessInProgress
                        },
                        nativeOn: {
                            click: function(e) {
                                return t.createHoldingsAuth(e)
                            }
                        }
                    }, [t._v("Retry")]) : t._e(), t._v(" "), !t.isAuthFinished && t.createSessFinished && t.reqID ? r("su-button", {
                        staticClass: "button button-blue",
                        attrs: {
                            processing: t.authInProgress
                        },
                        nativeOn: {
                            click: function(e) {
                                return t.openHoldingsAuth(e)
                            }
                        }
                    }, [t._v("Continue")]) : t._e(), t._v(" "), r("su-button", {
                        staticClass: "button button-outline",
                        nativeOn: {
                            click: function(e) {
                                return t.close(e)
                            }
                        }
                    }, [t._v("Close")])], 1)])
                },
                i = [],
                n = r("e165"),
                a = r.n(n),
                o = r("025e"),
                l = r("0d07");
            const c = "/connect/portfolio/authorise/holdings/{apiKey}/{reqID}";
            var u = {
                    name: "twofa-confirm",
                    props: {
                        message: null,
                        title: null,
                        canSkip: {
                            type: Boolean,
                            default: !1
                        },
                        overnight: {
                            type: Boolean,
                            default: !1
                        }
                    },
                    data() {
                        return {
                            reqID: null,
                            createSessError: null,
                            createSessInProgress: !1,
                            createSessFinished: !1,
                            isAuthFinished: !1,
                            authStatus: null,
                            authError: null,
                            authInProgress: !1,
                            canRetry: !1
                        }
                    },
                    mounted() {
                        this.createHoldingsAuth()
                    },
                    methods: {
                        close() {
                            this.$emit("close", !0)
                        },
                        skip() {
                            this.finishAuth("skip", null), this.close()
                        },
                        reset() {
                            this.isAuthFinished = !1, this.authStatus = null, this.authError = null, this.authInProgress = !1, this.createSessError = null, this.createSessFinished = !1, this.canRetry = !1, this.createSessInProgress = !1
                        },
                        createHoldingsAuth() {
                            this.reset(), this.createSessInProgress = !0;
                            let t = {};
                            this.overnight && (t.overnight = !0), l["a"].createHoldingsAuthSession(t).then(t => {
                                this.createSessInProgress = !1, this.createSessFinished = !0, this.reqID = t.data.data.request_id
                            }).catch(t => {
                                this.createSessError = Object(o["h"])(t), this.canRetry = !0, this.createSessFinished = !0, this.createSessInProgress = !1
                            })
                        },
                        openHoldingsAuth() {
                            this.authInProgress = !0;
                            var t = 900;
                            let e = 700,
                                r = window.screen.width ? (window.screen.width - t) / 2 : 100,
                                s = window.screen.height ? (window.screen.height - e) / 2 : 100,
                                i = "width=" + t + ",height=" + e + ",top=" + s + ",left=" + r;
                            i += ",scrollbars=yes,location=no,directories=no,status=no,menubar=no,toolbar=no,resizable=no";
                            let n = a()(c, {
                                    apiKey: "kitefront",
                                    reqID: this.reqID
                                }),
                                o = window.open(n, "holdings-auth", i);
                            this.timer && window.clearInterval(this.timer), this.timer = window.setInterval(() => {
                                window.addEventListener("message", t => {
                                    try {
                                        let e = JSON.parse(t.data);
                                        e && "holdings.auth" === e.type && setTimeout(() => {
                                            this.finishAuth(e.status, e.data), o.close()
                                        }, 500)
                                    } catch (t) {
                                        console.log("error", t)
                                    }
                                }, !1), !1 !== o.closed && (window.clearInterval(this.timer), this.isAuthFinished || this.finishAuth())
                            }, 500)
                        },
                        finishAuth(t, e) {
                            this.authInProgress = !1, this.isAuthFinished = !0, this.authStatus = t, "success" !== t && (this.canRetry = !0, this.authError = e), this.$emit("finish", {
                                status: this.authStatus
                            })
                        }
                    }
                },
                d = u,
                h = r("2877"),
                p = Object(h["a"])(d, s, i, !1, null, null, null);
            e["a"] = p.exports
        },
        2286: function(t, e, r) {
            var s = r("85e3"),
                i = Math.max;

            function n(t, e, r) {
                return e = i(void 0 === e ? t.length - 1 : e, 0),
                    function() {
                        var n = arguments,
                            a = -1,
                            o = i(n.length - e, 0),
                            l = Array(o);
                        while (++a < o) l[a] = n[e + a];
                        a = -1;
                        var c = Array(e + 1);
                        while (++a < e) c[a] = n[a];
                        return c[e] = r(l), s(t, this, c)
                    }
            }
            t.exports = n
        },
        2411: function(t, e, r) {
            var s = r("f909"),
                i = r("2ec1"),
                n = i((function(t, e, r, i) {
                    s(t, e, r, i)
                }));
            t.exports = n
        },
        "2ec1": function(t, e, r) {
            var s = r("100e"),
                i = r("9aff");

            function n(t) {
                return s((function(e, r) {
                    var s = -1,
                        n = r.length,
                        a = n > 1 ? r[n - 1] : void 0,
                        o = n > 2 ? r[2] : void 0;
                    a = t.length > 3 && "function" == typeof a ? (n--, a) : void 0, o && i(r[0], r[1], o) && (a = n < 3 ? void 0 : a, n = 1), e = Object(e);
                    while (++s < n) {
                        var l = r[s];
                        l && t(e, l, s, a)
                    }
                    return e
                }))
            }
            t.exports = n
        },
        "4e10": function(t, e, r) {
            "use strict";
            var s = function() {
                    var t = this,
                        e = t.$createElement,
                        r = t._self._c || e;
                    return r("transition", {
                        attrs: {
                            name: "order-window"
                        }
                    }, [t.isOpen ? r("su-form", {
                        directives: [{
                            name: "on-escape",
                            rawName: "v-on-escape",
                            value: t.closeWindow,
                            expression: "closeWindow"
                        }],
                        ref: "order-window",
                        staticClass: "order-window layer-2",
                        class: [t.windowType, t.order.transactionType.toLowerCase()],
                        attrs: {
                            "data-drag-boundary": "true"
                        },
                        on: {
                            submit: function(e) {
                                return e.preventDefault(), t.onSubmit(e)
                            }
                        }
                    }, [r("div", {
                        staticClass: "drag-handle"
                    }), t._v(" "), r("header", [r("div", {
                        staticClass: "row"
                    }, [r("div", {
                        staticClass: "eight columns"
                    }, [r("div", {
                        staticClass: "instrument"
                    }, [r("span", {
                        staticClass: "transaction-type"
                    }, [t._v(t._s(t.txType))]), t._v(" "), r("span", {
                        staticClass: "tradingsymbol"
                    }, [r("span", {
                        staticClass: "name",
                        domProps: {
                            innerHTML: t._s(t.dateSuperScript(t.curInstrument.niceName))
                        }
                    }, [t._v("\n\t\t\t\t\t\t\t" + t._s(t.curInstrument.niceName) + "\n\t\t\t\t\t\t")]), t._v(" "), r("span", {
                        staticClass: "exchange"
                    }, [t._v(t._s(t.curInstrument.exchange))])]), t._v("\n\t\t\t\t\t×\n\t\t\t\t\t"), r("span", {
                        staticClass: "qty"
                    }, [t._v(t._s(t.order.quantity) + " Qty")])])]), t._v(" "), r("div", {
                        staticClass: "four columns text-right"
                    }, [r("div", {
                        staticClass: "wrap-right"
                    }, [r("div", {
                        staticClass: "nudge"
                    }, [t.nudge.isExpanded && t.nudge.data && t.nudge.data.length > 0 ? r("div", {
                        staticClass: "popup"
                    }, [r("a", {
                        staticClass: "close icon icon-times",
                        attrs: {
                            href: ""
                        },
                        on: {
                            click: function(e) {
                                return e.preventDefault(), t.onToggleNudge(e)
                            }
                        }
                    }), t._v(" "), r("h4", [r("img", {
                        attrs: {
                            src: "/static/images/nudge-logo-dark.svg"
                        }
                    })]), t._v(" "), r("ul", {
                        staticClass: "list-flat"
                    }, t._l(t.nudge.data, (function(e, s) {
                        return r("li", {
                            key: s,
                            class: ["severity-" + e.severity]
                        }, [r("span", {
                            staticClass: "message",
                            domProps: {
                                innerHTML: t._s(t.formatDescription(e.message))
                            }
                        })])
                    })), 0)]) : t._e(), t._v(" "), t.nudge.data && t.nudge.data.length > 0 ? [r("span", {
                        staticClass: "count"
                    }, [t._v(t._s(t.nudge.data.length))]), t._v(" "), r("a", {
                        attrs: {
                            href: ""
                        },
                        on: {
                            click: function(e) {
                                return e.preventDefault(), t.onToggleNudge(e)
                            }
                        }
                    }, [r("img", {
                        attrs: {
                            src: "/static/images/nudge-icon-transparent.svg"
                        }
                    })])] : t._e()], 2), t._v(" "), r("div", [r("span", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        attrs: {
                            title: "Toggle Buy / Sell"
                        }
                    }, [r("su-switch", {
                        staticClass: "tx-toggle",
                        attrs: {
                            "state-on": "SELL",
                            "state-off": "BUY",
                            disabled: !t.constraints.transactionType
                        },
                        on: {
                            change: t.onOrderChange
                        },
                        model: {
                            value: t.order.transactionType,
                            callback: function(e) {
                                t.$set(t.order, "transactionType", e)
                            },
                            expression: "order.transactionType"
                        }
                    })], 1), t._v(" "), r("a", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        staticClass: "info",
                        attrs: {
                            target: "_blank",
                            href: "https://kite.trade/docs/kite/orders/#orders",
                            title: "Help"
                        }
                    }, [r("span", {
                        staticClass: "icon icon-info"
                    })])])])])]), t._v(" "), t.allInstruments.length > 1 ? r("div", {
                        staticClass: "exchange-selector"
                    }, [r("su-radio-group", {
                        on: {
                            change: function(e) {
                                t.onInstrumentSwitch(e), t.onOrderChange(e)
                            }
                        },
                        model: {
                            value: t.curExchange,
                            callback: function(e) {
                                t.curExchange = e
                            },
                            expression: "curExchange"
                        }
                    }, t._l(t.allInstruments, (function(e) {
                        return r("su-radio", {
                            key: e.exchange,
                            staticClass: "exchange",
                            attrs: {
                                name: "exchange",
                                label: e.exchange + ": <span class='last-price'>" + t.printLTP(e.instrumentToken) + "</span>",
                                value: e.exchange
                            }
                        })
                    })), 1)], 1) : r("div", [t._v("\n\t\t\t" + t._s(t.allInstruments[0].exchange) + ": "), r("span", {
                        staticClass: "last-price"
                    }, [t._v(t._s(t.printLTP(t.allInstruments[0].instrumentToken)))])])]), t._v(" "), r("section", {
                        staticClass: "wrap"
                    }, [t.isTagsVisible ? r("div", {
                        staticClass: "tags"
                    }, [r("span", {
                        staticClass: "arrow"
                    }), t._v(" "), r("su-tag-selector", {
                        attrs: {
                            tags: t.allTags
                        },
                        on: {
                            input: t.updateOrderPrefs
                        },
                        model: {
                            value: t.order.tags,
                            callback: function(e) {
                                t.$set(t.order, "tags", e)
                            },
                            expression: "order.tags"
                        }
                    })], 1) : t._e(), t._v(" "), r("div", {
                        staticClass: "content"
                    }, [r("div", {
                        staticClass: "variety"
                    }, [r("su-radio-group", {
                        on: {
                            change: t.onOrderChange
                        },
                        model: {
                            value: t.order.variety,
                            callback: function(e) {
                                t.$set(t.order, "variety", e)
                            },
                            expression: "order.variety"
                        }
                    }, t._l(t.enabledVarieties, (function(e) {
                        return r("su-radio", {
                            directives: [{
                                name: "tooltip",
                                rawName: "v-tooltip"
                            }],
                            key: e.name,
                            staticClass: "type",
                            attrs: {
                                name: "variety",
                                label: e.label,
                                title: e.tooltip,
                                value: e.name,
                                disabled: !t.constraints.variety
                            }
                        })
                    })), 1)], 1), t._v(" "), r("div", {
                        staticClass: "body"
                    }, [r("div", {
                        staticClass: "product row"
                    }, [t.enabledProducts.length > 0 ? r("su-radio-group", {
                        on: {
                            change: t.onOrderChange
                        },
                        model: {
                            value: t.order.product,
                            callback: function(e) {
                                t.$set(t.order, "product", e)
                            },
                            expression: "order.product"
                        }
                    }, t._l(t.enabledProducts, (function(e) {
                        return r("su-radio", {
                            directives: [{
                                name: "tooltip",
                                rawName: "v-tooltip",
                                value: "large",
                                expression: "'large'"
                            }],
                            key: e.name,
                            staticClass: "type four columns",
                            attrs: {
                                name: "product",
                                label: e.label + " <span>" + e.name + "</span>",
                                title: t.constraints.product ? e.tooltip : "",
                                value: e.name,
                                disabled: !t.constraints.product
                            }
                        })
                    })), 1) : t._e()], 1), t._v(" "), r("div", {
                        staticClass: "first-fields"
                    }, [r("div", {
                        staticClass: "row"
                    }, [r("div", {
                        staticClass: "four columns quantity"
                    }, [r("su-input", {
                        ref: "quantity",
                        staticClass: "no",
                        attrs: {
                            type: "number",
                            rules: t.rules.quantity,
                            min: t.curInstrument.lotSize,
                            step: t.curInstrument.lotSize,
                            disabled: !t.constraints.quantity,
                            autofocus: "",
                            "static-label": "",
                            "native-error": "",
                            label: "Qty."
                        },
                        on: {
                            change: function(e) {
                                t.queueFetchMargins(), t.queueFetchNudge(), t.resetForm()
                            }
                        },
                        model: {
                            value: t.order.quantity,
                            callback: function(e) {
                                t.$set(t.order, "quantity", e)
                            },
                            expression: "order.quantity"
                        }
                    })], 1), t._v(" "), r("div", {
                        staticClass: "four columns price"
                    }, [r("su-input", {
                        staticClass: "no",
                        attrs: {
                            type: "number",
                            rules: t.rules.price,
                            min: t.curInstrument.tickSize,
                            step: t.curInstrument.tickSize,
                            disabled: !t.constraints.price,
                            "static-label": "",
                            "native-error": "",
                            label: "Price"
                        },
                        on: {
                            change: function(e) {
                                t.queueFetchMargins(), t.queueFetchNudge(), t.resetForm()
                            }
                        },
                        model: {
                            value: t.order.price,
                            callback: function(e) {
                                t.$set(t.order, "price", e)
                            },
                            expression: "order.price"
                        }
                    })], 1), t._v(" "), r("div", {
                        staticClass: "four columns trigger"
                    }, [r("su-input", {
                        staticClass: "no",
                        attrs: {
                            type: "number",
                            rules: t.rules.triggerPrice,
                            min: 0,
                            step: t.curInstrument.tickSize,
                            disabled: !t.constraints.triggerPrice,
                            "static-label": "",
                            "native-error": "",
                            label: t.order.variety !== t.constants.VARIETY.CO ? "Trigger price" : "Stoploss trigger Price"
                        },
                        on: {
                            change: function(e) {
                                t.queueFetchMargins(), t.queueFetchNudge(), t.resetForm()
                            }
                        },
                        model: {
                            value: t.order.triggerPrice,
                            callback: function(e) {
                                t.$set(t.order, "triggerPrice", e)
                            },
                            expression: "order.triggerPrice"
                        }
                    })], 1)]), t._v(" "), r("div", {
                        staticClass: "row"
                    }, [r("div", {
                        staticClass: "four columns"
                    }, ["NSE" !== t.curInstrument.exchange && "BSE" !== t.curInstrument.exchange ? r("span", {
                        staticClass: "x-small text-light"
                    }, [t._v(t._s(Math.max(t.order.quantity / t.curInstrument.lotSize, 1)) + " lot(s)")]) : t._e(), t._v("\n\t\t\t\t\t\t\t \n\t\t\t\t\t\t")]), t._v(" "), r("div", {
                        staticClass: "four columns price"
                    }, [r("su-radio-group", {
                        staticClass: "order-type",
                        on: {
                            change: t.onOrderChange
                        },
                        model: {
                            value: t.order.orderType,
                            callback: function(e) {
                                t.$set(t.order, "orderType", e)
                            },
                            expression: "order.orderType"
                        }
                    }, [r("su-radio", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        attrs: {
                            name: "orderType",
                            value: t.constants.ORDER_TYPE.MARKET,
                            label: "Market",
                            disabled: !t.constraints.MARKET,
                            title: t.txType + " at market price",
                            "tooltip-pos": "down"
                        }
                    }), t._v(" "), r("su-radio", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        attrs: {
                            name: "orderType",
                            value: t.constants.ORDER_TYPE.LIMIT,
                            label: "Limit",
                            disabled: !t.constraints.LIMIT,
                            title: t.txType + " at a preferred price",
                            "tooltip-pos": "down"
                        }
                    })], 1)], 1), t._v(" "), r("div", {
                        staticClass: "four columns trigger"
                    }, [r("su-radio-group", {
                        staticClass: "text-right order-type",
                        on: {
                            change: t.onOrderChange
                        },
                        model: {
                            value: t.order.orderType,
                            callback: function(e) {
                                t.$set(t.order, "orderType", e)
                            },
                            expression: "order.orderType"
                        }
                    }, [r("su-radio", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        attrs: {
                            disabled: !t.constraints.SL,
                            name: "orderType",
                            value: t.constants.ORDER_TYPE.SL,
                            label: "SL",
                            title: t.txType + " at a preferred price with a stoploss",
                            "tooltip-pos": "down"
                        }
                    }), t._v(" "), r("su-radio", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        attrs: {
                            disabled: !t.constraints["SL-M"],
                            name: "orderType",
                            value: t.constants.ORDER_TYPE.SLM,
                            label: "SL-M",
                            title: t.txType + " at market price with stoploss",
                            "tooltip-pos": "down"
                        }
                    })], 1)], 1)]), t._v(" "), t.order.variety === t.constants.VARIETY.BO ? r("div", {
                        staticClass: "row bo-fields"
                    }, [r("div", {
                        staticClass: "four columns"
                    }, [r("su-input", {
                        staticClass: "no",
                        attrs: {
                            type: "number",
                            rules: t.rules.stoploss,
                            min: t.curInstrument.tickSize,
                            step: t.curInstrument.tickSize,
                            disabled: !t.constraints.stoploss,
                            "static-label": "",
                            label: "Stoploss"
                        },
                        model: {
                            value: t.order.stoploss,
                            callback: function(e) {
                                t.$set(t.order, "stoploss", e)
                            },
                            expression: "order.stoploss"
                        }
                    })], 1), t._v(" "), r("div", {
                        staticClass: "four columns"
                    }, [r("su-input", {
                        staticClass: "no",
                        attrs: {
                            type: "number",
                            rules: t.rules.squareoff,
                            min: t.curInstrument.tickSize,
                            step: t.curInstrument.tickSize,
                            disabled: !t.constraints.squareoff,
                            "static-label": "",
                            "native-error": "",
                            label: "Target"
                        },
                        model: {
                            value: t.order.squareoff,
                            callback: function(e) {
                                t.$set(t.order, "squareoff", e)
                            },
                            expression: "order.squareoff"
                        }
                    })], 1), t._v(" "), r("div", {
                        staticClass: "four columns"
                    }, [r("su-input", {
                        staticClass: "no",
                        attrs: {
                            type: "number",
                            rules: t.rules.trailingStoploss,
                            min: 0,
                            step: t.curInstrument.tickSize,
                            disabled: !t.constraints.trailingStoploss,
                            "static-label": "",
                            "native-error": "",
                            label: "Trailing stoploss"
                        },
                        model: {
                            value: t.order.trailingStoploss,
                            callback: function(e) {
                                t.$set(t.order, "trailingStoploss", e)
                            },
                            expression: "order.trailingStoploss"
                        }
                    })], 1)]) : t._e()]), t._v(" "), t.constraints.icebergLegs ? r("div", {
                        staticClass: "row second-fields"
                    }, [t.constraints.icebergLegs ? r("div", {
                        staticClass: "four columns quantity iceberg"
                    }, [r("su-input", {
                        staticClass: "no",
                        attrs: {
                            type: "number",
                            required: "",
                            rules: t.rules.icebergLegs,
                            min: 2,
                            max: 10,
                            step: 1,
                            disabled: !t.constraints.editIcebergLegs,
                            "static-label": "",
                            "native-error": "",
                            label: "Number of legs"
                        },
                        model: {
                            value: t.order.iceberg.legs,
                            callback: function(e) {
                                t.$set(t.order.iceberg, "legs", e)
                            },
                            expression: "order.iceberg.legs"
                        }
                    }), t._v(" "), t.windowType !== t.orderTypeModify ? r("span", {
                        staticClass: "x-small text-light"
                    }, [t.curInstrument.lotSize > 1 ? r("span", [t._v(t._s(t.icebergQty / t.curInstrument.lotSize) + " lot(s) / ")]) : t._e(), t._v(t._s(t.icebergQty) + " qty. per leg\n\t\t\t\t\t\t")]) : t._e()], 1) : r("div", {
                        staticClass: "four columns"
                    }, [t._v(" ")]), t._v(" "), r("div", {
                        staticClass: "four columns"
                    }, [t._v(" ")])]) : t._e(), t._v(" "), r("div", {
                        staticClass: "more-options"
                    }, [r("a", {
                        staticClass: "text-xsmall more-options",
                        attrs: {
                            href: "#"
                        },
                        on: {
                            click: function(e) {
                                return e.preventDefault(), t.onToggleMore(e)
                            }
                        }
                    }, [this.isMoreVisible ? r("span", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        attrs: {
                            title: "Hide options"
                        }
                    }, [t._v("Hide options "), r("span", {
                        staticClass: "icon icon-chevron-up"
                    })]) : r("span", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        attrs: {
                            title: "More options"
                        }
                    }, [t._v("More options "), r("span", {
                        staticClass: "icon icon-chevron-down"
                    })])])]), t._v(" "), t.isMoreVisible ? r("div", {
                        staticClass: "row more"
                    }, [r("div", {
                        staticClass: "four columns validity"
                    }, [r("label", [t._v("Validity")]), t._v(" "), r("su-radio-group", {
                        on: {
                            change: t.onOrderChange
                        },
                        model: {
                            value: t.order.validity,
                            callback: function(e) {
                                t.$set(t.order, "validity", e)
                            },
                            expression: "order.validity"
                        }
                    }, [r("su-radio", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        attrs: {
                            disabled: !t.constraints.validity || !t.constraints.DAY,
                            name: "validity",
                            value: t.constants.VALIDITY.DAY,
                            label: "Day",
                            title: "Valid throughout the day until executed"
                        }
                    }), t._v(" "), r("br"), t._v(" "), r("su-radio", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        attrs: {
                            disabled: !t.constraints.validity || !t.constraints.IOC,
                            name: "validity",
                            value: t.constants.VALIDITY.IOC,
                            label: "Immediate <span>IOC</span>",
                            title: "Cancel if not executed immediately"
                        }
                    }), t._v(" "), r("br"), t._v(" "), r("su-radio", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        attrs: {
                            disabled: !t.constraints.validity || !t.constraints.TTL,
                            name: "validity",
                            value: t.constants.VALIDITY.TTL,
                            label: "Minutes",
                            title: "Cancel if not executed within N minutes"
                        }
                    })], 1)], 1), t._v(" "), r("div", {
                        staticClass: "four columns"
                    }, [r("su-select", {
                        attrs: {
                            placeholder: "Validity minutes",
                            disabled: !t.constraints.validity || !t.constraints.ttlMinutes
                        },
                        model: {
                            value: t.order.validityTTL,
                            callback: function(e) {
                                t.$set(t.order, "validityTTL", e)
                            },
                            expression: "order.validityTTL"
                        }
                    }, t._l(t.options.ttlMinutes, (function(e) {
                        return r("option", {
                            key: e,
                            domProps: {
                                value: e
                            }
                        }, [t._v(t._s(e) + " " + t._s(1 === e ? "minute" : "minutes"))])
                    })), 0), t._v(" \n\t\t\t\t\t")], 1), t._v(" "), r("div", {
                        staticClass: "four columns"
                    }, [r("su-input", {
                        staticClass: "no",
                        attrs: {
                            type: "number",
                            disabled: !t.constraints.disclosedQuantity,
                            rules: t.rules.disclosedQuantity,
                            min: 0,
                            step: t.curInstrument.lotSize,
                            "static-label": "",
                            "native-error": "",
                            label: "Disclosed qty."
                        },
                        model: {
                            value: t.order.disclosedQuantity,
                            callback: function(e) {
                                t.$set(t.order, "disclosedQuantity", e)
                            },
                            expression: "order.disclosedQuantity"
                        }
                    })], 1)]) : t._e()]), t._v(" "), t.constraints.gtt ? r("div", {
                        staticClass: "gtt"
                    }, [r("div", {
                        staticClass: "row"
                    }, [r("div", {
                        staticClass: "one column"
                    }, [r("a", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        staticClass: "gtt-logo text-xxsmall dim",
                        attrs: {
                            href: "https://zrd.sh/gtt",
                            title: "Automatically create a GTT for the position on order completion",
                            target: "_blank"
                        }
                    }, [r("img", {
                        attrs: {
                            alt: "GTT logo",
                            src: "/static/images/gtt-logo.svg"
                        }
                    })])]), t._v(" "), r("div", {
                        staticClass: "four columns group"
                    }, [r("su-checkbox", {
                        attrs: {
                            disabled: !t.constraints.editGTT,
                            label: "Stoploss"
                        },
                        on: {
                            change: function(e) {
                                return t.onToggleGTT(e)
                            }
                        },
                        model: {
                            value: t.gtt.isStopLossEnabled,
                            callback: function(e) {
                                t.$set(t.gtt, "isStopLossEnabled", e)
                            },
                            expression: "gtt.isStopLossEnabled"
                        }
                    }), t._v(" "), r("div", {
                        staticClass: "input"
                    }, [r("su-input", {
                        ref: "gtt-stoploss",
                        attrs: {
                            disabled: !t.gtt.isStopLossEnabled,
                            readonly: !t.constraints.editGTT,
                            type: "number",
                            step: t.curInstrument.tickSize,
                            rules: t.rules.gttStoploss,
                            "native-error": ""
                        },
                        model: {
                            value: t.order.gtt.stoploss,
                            callback: function(e) {
                                t.$set(t.order.gtt, "stoploss", e)
                            },
                            expression: "order.gtt.stoploss"
                        }
                    }), r("span", [t._v("%")])], 1)], 1), t._v(" "), r("div", {
                        staticClass: "four columns group"
                    }, [r("su-checkbox", {
                        attrs: {
                            disabled: !t.constraints.editGTT,
                            label: "Target"
                        },
                        on: {
                            change: function(e) {
                                return t.onToggleGTT(e)
                            }
                        },
                        model: {
                            value: t.gtt.isTargetEnabled,
                            callback: function(e) {
                                t.$set(t.gtt, "isTargetEnabled", e)
                            },
                            expression: "gtt.isTargetEnabled"
                        }
                    }), t._v(" "), r("div", {
                        staticClass: "input"
                    }, [r("su-input", {
                        ref: "gtt-target",
                        attrs: {
                            disabled: !t.gtt.isTargetEnabled,
                            readonly: !t.constraints.editGTT,
                            type: "number",
                            step: t.curInstrument.tickSize,
                            rules: t.rules.gttTarget,
                            "native-error": ""
                        },
                        model: {
                            value: t.order.gtt.target,
                            callback: function(e) {
                                t.$set(t.order.gtt, "target", e)
                            },
                            expression: "order.gtt.target"
                        }
                    }), r("span", [t._v("%")])], 1)], 1), t._v(" "), r("div", {
                        staticClass: "two columns"
                    }, [r("a", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        staticClass: "text-xxsmall dim",
                        attrs: {
                            href: "https://zrd.sh/gtt",
                            title: "Automatically create a GTT for the position on order completion",
                            target: "_blank"
                        }
                    }, [t._v("Learn more")])])])]) : t._e(), t._v(" "), r("footer", {
                        staticClass: "footer row"
                    }, [r("div", {
                        staticClass: "six columns"
                    }, [r("div", {
                        staticClass: "row margins"
                    }, [r("span", {
                        staticClass: "label"
                    }, [t._v("Margin required\n\t\t\t\t\t\t\t"), r("a", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        staticClass: "info",
                        attrs: {
                            target: "_blank",
                            href: "https://support.zerodha.com/category/trading-and-markets/kite-web-and-mobile/articles/margins-on-kite-order-window",
                            title: "Margin calculation explained"
                        }
                    }, [r("span", {
                        staticClass: "icon icon-info"
                    })])]), t._v(" "), t.margins.data ? r("span", {
                        staticClass: "margin-value"
                    }, [t._v("\n\t\t\t\t\t\t\t" + t._s(t._f("inrFormat")(t.margins.data.total, 2, null, !0)) + "\n\t\t\t\t\t\t\t"), t.margins.data.leverage && t.margins.data.leverage > 1 ? r("span", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        staticClass: "text-light leverage",
                        attrs: {
                            title: "Leverage"
                        }
                    }, [t._v("\n\t\t\t\t\t\t\t\t(" + t._s(t._f("round")(t.margins.data.leverage, 2)) + "x)\n\t\t\t\t\t\t\t")]) : t._e()]) : r("span", [t._v("N/A")]), t._v(" "), t.margins.isProcessing ? r("su-loader") : t._e(), t._v(" "), t.margins.isProcessing ? t._e() : r("a", {
                        directives: [{
                            name: "tooltip",
                            rawName: "v-tooltip"
                        }],
                        attrs: {
                            href: "#",
                            title: "Refresh"
                        },
                        on: {
                            click: function(e) {
                                return e.preventDefault(), t.fetchMargins(e)
                            }
                        }
                    }, [r("span", {
                        staticClass: "reload-margin icon icon-reload"
                    })])], 1)]), t._v(" "), r("div", {
                        staticClass: "six columns text-right actions"
                    }, [r("su-button", {
                        staticClass: "submit",
                        attrs: {
                            type: "submit",
                            processing: t.isProcessing
                        }
                    }, [t.actionLabel ? r("span", [t._v(t._s(t.actionLabel))]) : r("span", [t._v(t._s(t.txType))])]), t._v(" "), r("su-button", {
                        staticClass: "button-outline cancel",
                        attrs: {
                            disabled: t.isProcessing
                        },
                        nativeOn: {
                            click: function(e) {
                                return e.preventDefault(), t.closeWindow()
                            }
                        }
                    }, [t._v("Cancel")])], 1)])])]), t._v(" "), t.isNudgePromptVisible ? r("nudge-confirm", {
                        attrs: {
                            nudges: t.nudge.data
                        },
                        on: {
                            proceed: t.onConfirmNudge,
                            close: function() {
                                t.isNudgePromptVisible = !1
                            }
                        }
                    }) : t._e(), t._v(" "), t.holdingsAuth.visible ? r("holdings-auth-confirm", {
                        attrs: {
                            "can-skip": t.holdingsAuth.canSkip
                        },
                        on: {
                            finish: t.finishHoldingsAuth,
                            close: t.hideHoldingsAuth
                        }
                    }) : t._e()], 1) : t._e()], 1)
                },
                i = [],
                n = r("2f62"),
                a = r("0a3b"),
                o = r("f1f7");
            let l = !1,
                c = [];
            const u = void 0 !== window.ontouchstart,
                d = function(t, e, r, s) {
                    l || document.addEventListener(u ? "touchmove" : "mousemove", (function(t) {
                        let e = t;
                        t.touches && (e = t.touches[0]);
                        for (var r = 0; r < c.length; r++) c[r](e.clientX, e.clientY)
                    })), l = !0;
                    let i = !1,
                        n = !1,
                        a = 0,
                        o = 0,
                        d = 0,
                        h = 0;
                    e.addEventListener(u ? "touchstart" : "mousedown", (function(e) {
                        if (e.stopPropagation(), e.preventDefault(), "false" === t.dataset.dragEnabled) return;
                        let r = e;
                        e.touches && (r = e.touches[0]), i = !0, a = t.offsetLeft - r.clientX, o = t.offsetTop - r.clientY
                    })), document.addEventListener(u ? "touchend" : "mouseup", (function() {
                        i = !1, n = !1, s && s(parseInt(t.style.left), parseInt(t.style.top))
                    })), c.push((function(e, s) {
                        if (i) {
                            if (n || (n = !0, r && r(d, h)), d = e + a, h = s + o, "true" === t.dataset.dragBoundary) {
                                if (d < 1 || d >= window.innerWidth - t.offsetWidth) return;
                                if (h < 1 || h >= window.innerHeight - t.offsetHeight) return
                            }
                            t.style.left = d + "px", t.style.top = h + "px"
                        }
                    }))
                };
            var h = r("025e"),
                p = r("5fb0"),
                g = r("2411"),
                f = r.n(g),
                b = r("d9d2"),
                m = r("0e50"),
                v = r("b202"),
                y = r("d1de"),
                T = r("39e3"),
                S = r("9dcd"),
                w = r.n(S);
            class E {
                constructor(t, e) {
                    this.order = t, this.state = e
                }
                disclosedQuantity(t, e) {
                    if (e = parseInt(e), e > this.order.quantity) return new Error(t.label + " can't be higher than qty. ")
                }
                triggerPrice(t, e) {
                    if (e = parseFloat(e), this.order.orderType === b["b"].ORDER_TYPE.SL || this.order.variety === b["b"].VARIETY.CO && this.order.orderType === b["b"].ORDER_TYPE.LIMIT && "modify" !== this.state.windowType) {
                        if (this.order.transactionType === b["b"].TRANSACTION_TYPE.BUY && e > this.order.price) return new Error(t.label + " can't be higher than price. ");
                        if (this.order.transactionType === b["b"].TRANSACTION_TYPE.SELL && e < this.order.price) return new Error(t.label + " can't be lesser than price. ")
                    }
                }
                multipleOfTickSize(t, e) {
                    if (e = parseFloat(e), isNaN(e)) return new Error(t.label + " is invalid.");
                    try {
                        if (!w()(e).mod(w()(this.order.instrument.tickSize)).eq(w()(0))) return new Error(t.label + " should be a multiple of " + this.order.instrument.tickSize)
                    } catch (r) {
                        return new Error(t.label + " is invalid.")
                    }
                }
                multipleOfLotSize(t, e) {
                    e = parseInt(e);
                    try {
                        if (e % this.order.instrument.lotSize !== 0) return new Error(t.label + " should be a multiple of " + this.order.instrument.lotSize)
                    } catch (r) {
                        return new Error(t.label + " is invalid.")
                    }
                }
                greaterThanZero(t, e) {
                    e = parseFloat(e);
                    try {
                        if (e <= 0) return new Error(t.label + " should be greater than 0")
                    } catch (r) {
                        return new Error(t.label + " is invalid.")
                    }
                }
                trailingStoploss(t, e) {
                    if (e = parseFloat(e), this.order.variety !== b["b"].VARIETY.BO) return;
                    let r = this.order.instrument.exchange === b["b"].EXCHANGES.CDS ? .05 : 1;
                    try {
                        if (e && 0 !== e && e < r) return new Error(t.label + " should be minimum " + r)
                    } catch (s) {
                        return new Error(t.label + " is invalid.")
                    }
                }
                isInteger(t, e) {
                    const r = /^\d+$/;
                    if (!r.exec(e)) return new Error(t.label + " is invalid.")
                }
                countDecimals(t) {
                    return Math.floor(t) === t ? 0 : t.toString().split(".")[1].length || 0
                }
                gttStoploss(t, e) {
                    if (this.state.gtt.isStopLossEnabled) try {
                        if (e = parseFloat(e), this.order.transactionType === b["b"].TRANSACTION_TYPE.BUY && e) {
                            if (e >= 0) return new Error(t.label + " percent should be less than 0, eg. -5%");
                            if (e < -100) return new Error(t.label + " percent can't be less than -100%. ")
                        }
                        if (this.order.transactionType === b["b"].TRANSACTION_TYPE.SELL && e && e <= 0) return new Error(t.label + " percent should be greater than 0, eg. 5%");
                        if (!e) return new Error(t.label + " is invalid. Set a % or uncheck to not create a GTT order.");
                        if (e && this.countDecimals(e) > 2) return new Error(t.label + " percent should be multiple of 0.01");
                        let r = this.state.getLTP(this.state.curInstrument.instrumentToken);
                        if (r) {
                            if (r >= 50 && Math.abs(e) < .25) return new Error(t.label + " is too close to LTP. It should be at least 0.25% or 10 paisa away. ");
                            if (r < 50 && r * Math.abs(e) < 10) return new Error(t.label + " is too close to LTP. It should be at least 0.25% or 10 paisa away. ")
                        }
                    } catch (r) {
                        return new Error(t.label + " is invalid.")
                    }
                }
                gttTarget(t, e) {
                    if (this.state.gtt.isTargetEnabled) try {
                        if (e = parseFloat(e), this.order.transactionType === b["b"].TRANSACTION_TYPE.BUY && e && e <= 0) return new Error(t.label + " percent should be greater than 0, eg. 5%");
                        if (this.order.transactionType === b["b"].TRANSACTION_TYPE.SELL && e) {
                            if (e >= 0) return new Error(t.label + " percent should be less than 0, eg. -5%");
                            if (e < -100) return new Error(t.label + " percent can't be less than -100%")
                        }
                        if (!e) return new Error(t.label + " is invalid. Set a % or uncheck to not create a GTT order.");
                        if (e && this.countDecimals(e) > 2) return new Error(t.label + " percent should be multiple of 0.01");
                        let r = this.state.getLTP(this.state.curInstrument.instrumentToken);
                        if (r) {
                            if (r >= 50 && Math.abs(e) < .25) return new Error(t.label + " is too close to LTP. It should be at least 0.25% or 10 paisa away. ");
                            if (r < 50 && r * Math.abs(e) < 10) return new Error(t.label + " is too close to LTP. It should be at least 0.25% or 10 paisa away. ")
                        }
                    } catch (r) {
                        return new Error(t.label + " is invalid.")
                    }
                }
                icebergLegs(t, e) {
                    try {
                        if ("place" !== this.state.windowType) return;
                        e = parseFloat(e);
                        const r = this.state.curInstrument;
                        if (this.order.quantity / r.lotSize < e) {
                            const e = "NSE" === r.exchange || "BSE" === r.exchange ? "qty." : "lots";
                            return new Error(`Order ${e} should be greater than ${t.label}`)
                        }
                        let s = this.state.getLTP(r.instrumentToken);
                        if (s && "EQ" === r.type) {
                            if (this.order.price) {
                                if (this.order.quantity * this.order.price < 1e5) return new Error(`Equity Iceberg orders require minimum 1,00,000 value (current value: ${this.order.quantity*this.order.price})`)
                            } else if (this.order.quantity * s < 1e5) return new Error(`Equity Iceberg orders require minimum 1,00,000 value (current value: ${this.order.quantity*s})`)
                        } else if (("FUT" === r.type || "OPT" === r.type) && this.order.quantity / r.lotSize < 5) return new Error("F&O Iceberg orders require minimum 5 lots")
                    } catch (r) {
                        return new Error(t.label + " is invalid.")
                    }
                }
            }
            const O = (t, e) => {
                const r = new E(t, e);
                return {
                    quantity: [{
                        type: "number",
                        required: !0,
                        message: "Quantity is required"
                    }, {
                        validator: (...t) => r.isInteger(...t),
                        label: "Quantity is not multiple of 1"
                    }, {
                        validator: (...t) => r.multipleOfLotSize(...t),
                        label: "Quantity"
                    }],
                    disclosedQuantity: [{
                        validator: (...t) => r.disclosedQuantity(...t),
                        label: "Disclosed quantity"
                    }, {
                        validator: (...t) => r.multipleOfLotSize(...t),
                        label: "Disclosed quantity"
                    }],
                    price: [{
                        validator: (...t) => r.multipleOfTickSize(...t),
                        label: "Price"
                    }],
                    triggerPrice: [{
                        validator: (...t) => r.triggerPrice(...t),
                        label: "Trigger price"
                    }, {
                        validator: (...t) => r.multipleOfTickSize(...t),
                        label: "Trigger price"
                    }],
                    squareoff: [{
                        type: "number",
                        required: !0,
                        message: "Target is required"
                    }, {
                        validator: (...t) => r.multipleOfTickSize(...t),
                        label: "Target"
                    }, {
                        validator: (...t) => r.greaterThanZero(...t),
                        label: "Target"
                    }],
                    stoploss: [{
                        type: "number",
                        required: !0,
                        message: "Stoploss is required"
                    }, {
                        validator: (...t) => r.multipleOfTickSize(...t),
                        label: "Stoploss"
                    }, {
                        validator: (...t) => r.greaterThanZero(...t),
                        label: "Stoploss"
                    }],
                    trailingStoploss: [{
                        validator: (...t) => r.multipleOfTickSize(...t),
                        label: "Trailing stoploss"
                    }, {
                        validator: (...t) => r.trailingStoploss(...t),
                        label: "Trailing stoploss"
                    }],
                    product: [{
                        type: "string",
                        required: !0,
                        message: "Product is required"
                    }],
                    variety: [{
                        type: "string",
                        required: !0,
                        message: "Variety is required"
                    }],
                    orderType: [{
                        type: "string",
                        required: !0,
                        message: "Order types is required"
                    }],
                    validity: [{
                        type: "string",
                        required: !0,
                        message: "Order validity is required"
                    }],
                    gttStoploss: [{
                        validator: (...t) => r.gttStoploss(...t),
                        label: "GTT stoploss"
                    }],
                    gttTarget: [{
                        validator: (...t) => r.gttTarget(...t),
                        label: "GTT target"
                    }],
                    icebergLegs: [{
                        validator: (...t) => r.icebergLegs(...t),
                        label: "Iceberg legs"
                    }]
                }
            };
            class _ {
                constructor() {
                    this.tags = ["product", "orderType", "variety", "regular", "amo", "bo", "co", "iceberg", "BUY", "SELL", "MARKET", "LIMIT", "SL", "SL-M", "DAY", "IOC", "TTL", "NRML", "MIS", "CNC", "NSE", "BSE", "NFO", "BFO", "CDS", "MCX", "BCD", "quantity", "price", "triggerPrice", "disclosedQuantity", "equity", "lotSize", "transactionType", "icebergLegs", "editIcebergLegs", "ttlMinutes", "validity", "squareoff", "stoploss", "trailingStoploss", "gtt", "editGTT", "basket"], this.items = []
                }
                add(t) {
                    -1 === this.items.indexOf(t) && this.items.push(t)
                }
                addMany(t) {
                    for (let e of t) this.add(e)
                }
                delete(t) {
                    var e = this.items.indexOf(t);
                    e > -1 && this.items.splice(e, 1)
                }
                flush() {
                    this.items = []
                }
                has(t) {
                    return -1 !== this.items.indexOf(t)
                }
                compute() {
                    var t = {};
                    for (let e of this.tags) t[e] = !0;
                    return t["squareoff"] = !1, t["stoploss"] = !1, t["trailingStoploss"] = !1, this.has("NSE") || this.has("BSE") ? (t["lotSize"] = !1, t["NRML"] = !1) : (t["equity"] = !1, t["CNC"] = !1), (this.has("MCX") || this.has("BCD") || this.has("BSE")) && (t["bo"] = !1), this.has("NSE") ? t["co"] = !0 : t["co"] = !1, (this.has("NFO") || this.has("BFO")) && (t["disclosedQuantity"] = !1), this.has("IOC") && (t["disclosedQuantity"] = !1), this.has("MARKET") ? (t["price"] = !1, t["triggerPrice"] = !1) : this.has("LIMIT") ? t["triggerPrice"] = !1 : this.has("SL-M") && (t["price"] = !1), (this.has("bo") || this.has("co")) && (t["IOC"] = !1, t["TTL"] = !1, t["disclosedQuantity"] = !1, t["product"] = !1, t["CNC"] = !1, t["NRML"] = !1, t["MIS"] = !0, this.has("modify") && this.has("parentOrderId") && (t["quantity"] = !1)), this.has("co") && (t["triggerPrice"] = !0, this.has("modify") ? (t["MARKET"] = !1, t["LIMIT"] = !1, t["SL"] = !1, t["SL-M"] = !1, this.has("LIMIT") && (t["triggerPrice"] = !1)) : (t["SL"] = !1, t["SL-M"] = !1)), this.has("bo") ? this.has("modify") ? (t["orderType"] = !1, t["MARKET"] = !1, t["LIMIT"] = !1, t["SL-M"] = !1, t["SL"] = !1, this.has("SL") && (t["price"] = !1)) : (t["stoploss"] = !0, t["squareoff"] = !0, t["trailingStoploss"] = !0, t["SL-M"] = !1, t["MARKET"] = !1) : t["bo"] = !1, this.has("place") && (t["place"] = !0, t["modify"] = !1), this.has("modify") && (t["product"] = !1, t["modify"] = !0, t["place"] = !1, t["transactionType"] = !1, t["validity"] = !1, this.has("co") && (t["quantity"] = !1)), (this.has("MCX") || this.has("BCD") || this.has("CDS")) && (t["gtt"] = !1), t["gtt"] && (this.has("amo") || this.has("co") || this.has("bo")) && (t["gtt"] = !1), t["gtt"] && (t["gtt"] = this.has("CNC") && this.has("BUY") || this.has("NRML")), t["editGTT"] = this.has("place"), t["variety"] = !this.has("modify"), this.has("place") || (this.has("DAY") ? t["TTL"] = !1 : t["DAY"] = !1, t["ttlMinutes"] = !1), this.has("TTL") || (t["ttlMinutes"] = !1), (this.has("MCX") || this.has("basket") || this.has("BCD")) && (t["iceberg"] = !1), this.has("iceberg") ? (t["gtt"] = !1, t["icebergLegs"] = !0, t["disclosedQuantity"] = !1, this.has("NSE") || this.has("NFO") || this.has("CDS") || (t["MARKET"] = !1, t["price"] = !0, t["IOC"] = !1), (this.has("BSE") || this.has("BCD")) && (t["MARKET"] = !1, t["price"] = !0, t["IOC"] = !1, t["SL-M"] = !1), this.has("modify") && (t["quantity"] = !1, t["editIcebergLegs"] = !1)) : t["icebergLegs"] = !1, t
                }
            }
            var P = r("3da7"),
                I = function() {
                    var t = this,
                        e = t.$createElement,
                        r = t._self._c || e;
                    return r("modal", {
                        directives: [{
                            name: "on-escape",
                            rawName: "v-on-escape",
                            value: t.close,
                            expression: "close"
                        }],
                        staticClass: "nudge-confirm-modal",
                        on: {
                            close: t.close
                        }
                    }, [r("div", {
                        attrs: {
                            slot: "header"
                        },
                        slot: "header"
                    }, [r("h4", {
                        staticClass: "title"
                    }, [r("img", {
                        attrs: {
                            src: t.getStatic(t.appendThemeName("images/nudge-icon.svg"))
                        }
                    }), t._v("\n\t\t\tNudge\n\t\t")])]), t._v(" "), r("div", {
                        attrs: {
                            slot: "body"
                        },
                        slot: "body"
                    }, [r("ul", {
                        staticClass: "list-flat"
                    }, t._l(t.nudges, (function(e, s) {
                        return r("li", {
                            key: s,
                            class: ["severity-" + e.severity]
                        }, [e.description ? r("span", {
                            staticClass: "description",
                            domProps: {
                                innerHTML: t._s(t.formatDescription(e.description))
                            }
                        }) : t._e()])
                    })), 0)]), t._v(" "), r("div", {
                        staticClass: "actions",
                        attrs: {
                            slot: "footer"
                        },
                        slot: "footer"
                    }, [r("su-button", {
                        staticClass: "button-orange",
                        nativeOn: {
                            click: function(e) {
                                return t.proceed(e)
                            }
                        }
                    }, [r("span", [t._v("Proceed")])]), t._v(" "), r("su-button", {
                        staticClass: "button-outline",
                        nativeOn: {
                            click: function(e) {
                                return t.close(e)
                            }
                        }
                    }, [r("span", [t._v("Cancel")])])], 1)])
                },
                C = [],
                k = r("9c9e"),
                A = {
                    name: "nudge-confirm-window",
                    mixins: [k["a"]],
                    props: {
                        nudges: Array
                    },
                    data() {
                        return {}
                    },
                    methods: {
                        proceed() {
                            this.$emit("proceed", !0)
                        },
                        close() {
                            this.$emit("close", !0)
                        },
                        formatDescription(t) {
                            return Object(h["i"])(t)
                        }
                    }
                },
                x = A,
                q = r("2877"),
                R = Object(q["a"])(x, I, C, !1, null, null, null),
                L = R.exports,
                N = r("1737");

            function M(t, e) {
                var r = Object.keys(t);
                if (Object.getOwnPropertySymbols) {
                    var s = Object.getOwnPropertySymbols(t);
                    e && (s = s.filter((function(e) {
                        return Object.getOwnPropertyDescriptor(t, e).enumerable
                    }))), r.push.apply(r, s)
                }
                return r
            }

            function D(t) {
                for (var e = 1; e < arguments.length; e++) {
                    var r = null != arguments[e] ? arguments[e] : {};
                    e % 2 ? M(Object(r), !0).forEach((function(e) {
                        F(t, e, r[e])
                    })) : Object.getOwnPropertyDescriptors ? Object.defineProperties(t, Object.getOwnPropertyDescriptors(r)) : M(Object(r)).forEach((function(e) {
                        Object.defineProperty(t, e, Object.getOwnPropertyDescriptor(r, e))
                    }))
                }
                return t
            }

            function F(t, e, r) {
                return e in t ? Object.defineProperty(t, e, {
                    value: r,
                    enumerable: !0,
                    configurable: !0,
                    writable: !0
                }) : t[e] = r, t
            }
            const j = "place",
                $ = "modify",
                Y = {
                    transactionType: b["b"].TRANSACTION_TYPE.BUY,
                    variety: b["b"].VARIETY.REGULAR,
                    orderType: b["b"].ORDER_TYPE.LIMIT,
                    product: b["b"].PRODUCT.CNC,
                    validity: b["b"].VALIDITY.DAY,
                    quantity: 0,
                    disclosedQuantity: 0,
                    price: 0,
                    triggerPrice: 0,
                    stoploss: 0,
                    squareoff: 0,
                    trailingStoploss: 0,
                    validityTTL: 1,
                    instrument: {},
                    gtt: {
                        stoploss: 0,
                        target: 0
                    },
                    iceberg: {
                        legs: 2
                    },
                    tags: [],
                    isCallback: !1,
                    context: null
                },
                V = {
                    varities: [{
                        name: b["b"].VARIETY.REGULAR,
                        label: "Regular",
                        tooltip: "Regular order"
                    }, {
                        name: b["b"].VARIETY.CO,
                        label: "Cover",
                        tooltip: "Cover order"
                    }, {
                        name: b["b"].VARIETY.BO,
                        label: "Bracket",
                        tooltip: "Bracket order"
                    }, {
                        name: b["b"].VARIETY.AMO,
                        label: "AMO",
                        tooltip: "After Market Order (for the next day)"
                    }, {
                        name: b["b"].VARIETY.ICEBERG,
                        label: "Iceberg",
                        tooltip: "Split an order into smaller orders"
                    }],
                    products: [{
                        name: b["b"].PRODUCT.MIS,
                        label: "Intraday",
                        tooltip: "Margin Intraday Squareoff: Requires lower margin for equity. Has to be exited before market close."
                    }, {
                        name: b["b"].PRODUCT.CNC,
                        label: "Longterm",
                        tooltip: "CashNCarry: Longterm investment. Requires full upfront margin."
                    }, {
                        name: b["b"].PRODUCT.NRML,
                        label: "Overnight",
                        tooltip: "Normal: Carry forward positions overnight."
                    }],
                    ttlMinutes: [1, 2, 3, 5, 10, 15, 30, 45, 60, 90, 120]
                };
            var z = {
                    name: "order-window",
                    components: {
                        "nudge-confirm": L,
                        "holdings-auth-confirm": N["a"]
                    },
                    inject: ["ticker", "instrumentManager"],
                    data() {
                        return {
                            constants: b["b"],
                            options: V,
                            orderTypePlace: j,
                            orderTypeModify: $,
                            isOpen: !1,
                            isMoreVisible: !1,
                            isProcessing: !1,
                            isDraggable: !1,
                            isEquity: !1,
                            isFO: !1,
                            lastOrder: {},
                            order: {},
                            windowType: j,
                            actionLabel: "",
                            curExchange: "",
                            curInstrument: {},
                            subInstruments: [],
                            allInstruments: [],
                            constraints: {},
                            margins: {
                                data: null,
                                tick: null,
                                isProcessing: !1,
                                isError: !1
                            },
                            holdingsAuth: {
                                visible: !1,
                                canSkip: !1,
                                isSkipped: !1
                            },
                            isNudgePromptVisible: !1,
                            isNudgeConfirmed: !1,
                            nudge: {
                                isExpanded: !1,
                                data: [],
                                tick: null
                            },
                            gtt: {
                                isStopLossEnabled: !1,
                                isTargetEnabled: !1
                            },
                            isTagsVisible: !0 === Object(v["b"])(m["b"], "isTagsVisible"),
                            allTags: []
                        }
                    },
                    methods: D(D({}, Object(n["d"])(m["b"], ["setOrderPrefs"])), {}, {
                        setupForm() {
                            this.resetForm();
                            const t = new _;
                            t.addMany([this.windowType, this.order.orderType, this.order.variety, this.order.transactionType, this.order.product, this.order.validity, this.order.parentOrderId && "parentOrderId", this.order.instrument.exchange, this.order.instrument.segment, this.order.context]), this.constraints = t.compute(), this.sanitizeFields(this.constraints)
                        },
                        resetForm() {
                            this.$refs["order-window"] && this.$refs["order-window"].reset()
                        },
                        openWindow(t, e) {
                            this.windowType = t, e.label ? this.actionLabel = e.label : this.actionLabel = t === $ ? "Modify" : "", this.holdingsAuth = {
                                visible: !1,
                                canSkip: !1,
                                isSkipped: !1
                            }, this.order = f()(this.$clone(Y), this.$clone(this.orderPrefs), this.$clone(e), (t, e) => null === e || void 0 === e ? t : e), this.setDefaultProduct(), this.windowType === $ && this.order.filledQuantity > 0 && (this.order.quantity = parseInt(this.order.quantity) - parseInt(this.order.filledQuantity || 0)), this.margins.data = null, this.nudge = {
                                isExpanded: !1,
                                data: [],
                                tick: null
                            }, this.allInstruments = this.makeAllInstruments(e.instrument, this.windowType), this.curExchange = e.instrument.exchange, this.curInstrument = this.allInstruments.find(t => t.exchange === e.instrument.exchange), e.gtt ? this.gtt = {
                                isStopLossEnabled: !!e.gtt.stoploss,
                                isTargetEnabled: !!e.gtt.target
                            } : this.gtt = {
                                isStopLossEnabled: !1,
                                isTargetEnabled: !1
                            }, this.rules = O(this.order, this), this.setupForm(), this.isOpen = !0, this.isNudgePromptVisible = !1, this.$nextTick(() => {
                                this.makeDraggable()
                            }), this.subscribeTicks(this.allInstruments.map(t => t.instrumentToken)), this.queueFetchMargins(), this.queueFetchNudge(), this.$nextTick(() => {
                                this.$refs.quantity && (this.$refs.quantity.focus(), this.$refs.quantity.$el.querySelector("input").select())
                            })
                        },
                        onOrderChange() {
                            this.$nextTick(() => {
                                this.setupForm(), this.constraints.gtt || (this.gtt = {
                                    isStopLossEnabled: !1,
                                    isTargetEnabled: !1
                                }), this.constraints.disclosedQuantity || (this.order.disclosedQuantity = 0), this.isMarginChanged(this.lastOrder, this.order) && (this.queueFetchMargins(), this.queueFetchNudge()), this.updateOrderPrefs(), this.lastOrder = this.$clone(this.order)
                            })
                        },
                        showHoldingsAuth() {
                            this.holdingsAuth.visible = !0
                        },
                        hideHoldingsAuth() {
                            this.holdingsAuth.visible = !1
                        },
                        closeWindow() {
                            this.isOpen = !1, this.unsubscribeTicks(this.subInstruments)
                        },
                        sanitizeFields(t) {
                            this.order.variety === b["b"].VARIETY.CO ? (this.order.orderType !== b["b"].ORDER_TYPE.MARKET && this.order.orderType !== b["b"].ORDER_TYPE.LIMIT && this.$nextTick(() => {
                                this.order.orderType = b["b"].ORDER_TYPE.MARKET
                            }), this.order.validity = b["b"].VALIDITY.DAY, this.order.product = b["b"].PRODUCT.MIS) : this.order.variety === b["b"].VARIETY.BO && (this.order.orderType !== b["b"].ORDER_TYPE.SL && this.order.orderType !== b["b"].ORDER_TYPE.LIMIT && (this.order.orderType = b["b"].ORDER_TYPE.LIMIT), this.$nextTick(() => {
                                this.order.validity = b["b"].VALIDITY.DAY, this.order.product = b["b"].PRODUCT.MIS
                            })), t[this.order.variety] || (this.order.variety = b["b"].VARIETY.REGULAR), this.setDefaultProduct(), this.windowType !== $ && (t.price ? 0 === parseFloat(this.order.price) && (this.order.price = this.getLTP(this.curInstrument.instrumentToken) || this.curInstrument.tickSize) : this.order.price = 0, t.triggerPrice ? 0 === parseFloat(this.order.triggerPrice) && (this.order.triggerPrice = this.getLTP(this.curInstrument.instrumentToken) || this.curInstrument.tickSize) : this.order.triggerPrice = 0, t.gtt && 0 === this.order.gtt.stoploss && 0 === this.order.gtt.target && (this.order.transactionType === b["b"].TRANSACTION_TYPE.SELL ? this.order.gtt = {
                                stoploss: 5,
                                target: -5
                            } : this.order.gtt = {
                                stoploss: -5,
                                target: 5
                            })), this.order.variety === b["b"].VARIETY.BO && (this.order.squareoff || (this.order.squareoff = this.curInstrument.tickSize), this.order.stoploss || (this.order.stoploss = this.curInstrument.tickSize)), this.order.variety === b["b"].VARIETY.ICEBERG && this.curInstrument.exchange === b["b"].EXCHANGES.BSE && (this.order.orderType !== b["b"].ORDER_TYPE.MARKET && this.order.orderType !== b["b"].ORDER_TYPE.SLM || (this.order.orderType = b["b"].ORDER_TYPE.LIMIT), this.order.validity === b["b"].VALIDITY.IOC && (this.order.validity = b["b"].VALIDITY.DAY)), 0 === this.order.quantity && (this.order.quantity = this.curInstrument.lotSize)
                        },
                        makeDraggable() {
                            const t = 50,
                                e = document.querySelector(".order-window");
                            e && d(e, e.querySelector(".drag-handle"), (function() {
                                e.style.top = e.offsetTop + "px", e.style.bottom = "auto"
                            }), (function() {
                                window.innerHeight - (e.offsetTop + e.offsetHeight) < t && (e.style.top = "auto", e.style.bottom = "0px"), window.innerWidth - (e.offsetLeft + e.offsetWidth) < t && (e.style.left = "auto", e.style.right = "0px"), e.offsetTop < t && (e.style.top = "0px"), e.offsetLeft < t && (e.style.left = "0px")
                            }))
                        },
                        dateSuperScript(t) {
                            return Object(a["a"])(t)
                        },
                        inrFormat(t) {
                            return Object(o["b"])(t)
                        },
                        printLTP(t) {
                            const e = this.getLTP(t);
                            return 0 === e ? "N/A" : "₹" + this.inrFormat(e, null, !0)
                        },
                        getLTP(t) {
                            return this.ticks && this.ticks[t] ? this.ticks[t].lastPrice : 0
                        },
                        makeAllInstruments(t) {
                            if (!t.related || this.windowType === $) return [t];
                            let e = [t, ...t.related];
                            return e.sort((t, e) => t.exchange > e.exchange ? 1 : e.exchange > t.exchange ? -1 : 0), e
                        },
                        onInstrumentSwitch() {
                            this.curInstrument = this.allInstruments.find(t => t.exchange === this.curExchange), this.order.instrument = this.curInstrument, this.queueFetchMargins(), this.queueFetchNudge()
                        },
                        onToggleNudge() {
                            this.nudge.isExpanded = !this.nudge.isExpanded
                        },
                        onConfirmNudge() {
                            this.isNudgeConfirmed = !0, this.isNudgePromptVisible = !1, this.$nextTick(() => this.onSubmit())
                        },
                        onToggleMore() {
                            this.isMoreVisible = !this.isMoreVisible, this.updateOrderPrefs()
                        },
                        onToggleGTT() {
                            this.$nextTick(() => {
                                let t = "gtt-stoploss";
                                this.gtt.isTargetEnabled && (t = "gtt-target");
                                const e = this.$refs[t];
                                e && e.$el && (e.focus(), e.$el.querySelector("input").select())
                            })
                        },
                        onOrderPlaced(t, e) {
                            if (this.isProcessing = !1, t) {
                                let e = t.data.data.order_id;
                                return this.$events.emit(b["b"].EVENTS.ORDER_PLACE_SUCCESS, this.$clone(D(D({}, this.order), {}, {
                                    orderId: e
                                }))), this.$toast.info({
                                    class: "order-toast",
                                    title: "Placed",
                                    message: `${this.order.transactionType} ${this.order.instrument.tradingsymbol} is placed.\n\t\t\t\t\t\t\t<br />Check the orderbook for status.<br /><span class='order-id'>#${t.data.data.order_id}</span>`,
                                    orientation: this.$toast.BOTTOM_RIGHT,
                                    update: !1,
                                    id: t.data.data.order_id,
                                    onClick: () => {
                                        this.$router.push({
                                            name: "orders"
                                        })
                                    }
                                }), void(this.isStickyOrderWindow || this.closeWindow())
                            }
                            if (!e) return;
                            if (e.response && 428 === e.response.status) return this.holdingsAuth.canSkip = e.response.data && e.response.data.data && e.response.data.data.can_skip, void this.showHoldingsAuth();
                            let r = Object(h["h"])(e);
                            this.$events.emit(b["b"].EVENTS.ORDER_PLACE_ERROR, D(D({}, this.order), {}, {
                                error: r
                            })), r.status_code >= 502 && r.status_code <= 504 ? ("orders" === this.$route.name && this.$events.emit(p["c"].EVENTS.refetch + this.$route.name), this.$toast.warning({
                                title: "Error",
                                message: r.message + "<br />Please check the orderbook before placing the order again.",
                                orientation: this.$toast.BOTTOM_RIGHT,
                                duration: 6e3
                            })) : this.$toast.error({
                                title: "Error",
                                message: r.message,
                                orientation: this.$toast.BOTTOM_RIGHT,
                                duration: 6e3
                            })
                        },
                        onOrderModified(t, e) {
                            if (this.isProcessing = !1, t) return this.$events.emit(b["b"].EVENTS.ORDER_MODIFY_SUCCESS, this.order), this.$toast.success({
                                title: "Request sent",
                                message: "Modification request sent.",
                                orientation: this.$toast.BOTTOM_RIGHT
                            }), void this.closeWindow();
                            if (e.response && 428 === e.response.status) return this.holdingsAuth.canSkip = e.response.data && e.response.data.data && e.response.data.data.can_skip, void this.showHoldingsAuth();
                            let r = Object(h["h"])(e);
                            this.$events.emit(b["b"].EVENTS.ORDER_MODIFY_ERROR, D(D({}, this.order), {}, {
                                error: r
                            })), this.$toast.error({
                                title: "Error",
                                message: r.message,
                                orientation: this.$toast.BOTTOM_RIGHT
                            })
                        },
                        onSubmit() {
                            if (this.isProcessing) return !1;
                            if (this.isProcessing = !0, this.$refs["order-window"] && !this.$refs["order-window"].validate()) return this.errors = this.$refs["order-window"].errors, void(this.isProcessing = !1);
                            if (this.order.isCallback) return this.$events.emit(b["b"].EVENTS.ORDER_PLACE_SUCCESS, this.makeCallbackParams(this.order)), this.isProcessing = !1, void this.closeWindow();
                            if (this.windowType === j) {
                                if (this.isNudgePrompt && !this.isNudgeConfirmed) return this.isNudgePromptVisible = !0, void(this.isProcessing = !1);
                                P["a"].placeOrder(this.makeOrderParams(this.order)).then(t => {
                                    this.onOrderPlaced(t, null)
                                }).catch(t => {
                                    this.onOrderPlaced(null, t)
                                })
                            } else this.windowType === $ && P["a"].modifyOrder(this.order.orderId, this.makeOrderParams(this.order)).then(t => {
                                this.onOrderModified(t, null)
                            }).catch(t => {
                                this.onOrderModified(null, t)
                            })
                        },
                        queueFetchMargins() {
                            null !== this.margins.tick && clearTimeout(this.margins.tick), this.margins.tick = setTimeout(this.fetchMargins, 500)
                        },
                        fetchMargins() {
                            const t = [{
                                exchange: this.order.instrument.exchange,
                                tradingsymbol: this.order.instrument.tradingsymbol,
                                transaction_type: this.order.transactionType,
                                variety: this.order.variety,
                                product: this.order.product,
                                order_type: this.order.orderType,
                                quantity: parseInt(this.order.quantity),
                                price: parseFloat(this.order.price || 0) || void 0,
                                trigger_price: parseFloat(this.order.triggerPrice) || void 0,
                                squareoff: parseFloat(this.order.squareoff) || void 0,
                                stoploss: parseFloat(this.order.stoploss) || void 0,
                                order_id: this.order.orderId || void 0
                            }];
                            this.margins.isProcessing = !0, this.margins.isError = !1, P["a"].fetchOrderMargins(t).then(t => {
                                this.margins.isProcessing = !1, t.data.data && t.data.data[0] ? this.margins.data = t.data.data[0] : (this.margins.data = null, this.margins.isError = !0)
                            }).catch(t => {
                                this.margins.data = null, this.margins.isProcessing = !1, this.margins.isError = !0
                            })
                        },
                        queueFetchNudge() {
                            null !== this.nudge.tick && clearTimeout(this.nudge.tick), this.nudge.tick = setTimeout(this.fetchNudge, 500)
                        },
                        fetchNudge() {
                            var t = {
                                exchange: this.order.instrument.exchange,
                                tradingsymbol: this.order.instrument.tradingsymbol,
                                transaction_type: this.order.transactionType,
                                variety: this.order.variety,
                                product: this.order.product,
                                order_type: this.order.orderType,
                                quantity: parseInt(this.order.quantity),
                                price: parseFloat(this.order.price || 0) || void 0,
                                trigger_price: parseFloat(this.order.triggerPrice) || void 0,
                                squareoff: parseFloat(this.order.squareoff) || void 0,
                                stoploss: parseFloat(this.order.stoploss) || void 0
                            };
                            this.order.variety === b["b"].VARIETY.ICEBERG && (t.iceberg_legs = this.order.iceberg.legs, t.iceberg_quantity = this.icebergQty), P["a"].checkOrdersNudge([t]).then(t => {
                                let e = t.data.data[0];
                                e && 0 !== e.length ? this.nudge = {
                                    data: e,
                                    isExpanded: !0,
                                    tick: null
                                } : this.nudge = {
                                    data: [],
                                    isExpanded: !1,
                                    tick: null
                                }
                            })
                        },
                        subscribeTicks(t) {
                            this.subInstruments.length > 0 && this.unsubscribeTicks(this.subInstruments), this.subInstruments = [...t], this.ticker.subscribe(t, "order-window"), this.ticker.setMode(this.ticker.modeLTP, t, "order-window")
                        },
                        unsubscribeTicks(t) {
                            this.ticker.unsubscribe(t, "order-window"), this.subInstruments = []
                        },
                        makeOrderParams(t) {
                            let e = {
                                variety: t.variety,
                                exchange: t.instrument.exchange,
                                tradingsymbol: t.instrument.tradingsymbol,
                                transaction_type: t.transactionType,
                                order_type: t.orderType,
                                quantity: parseInt(t.quantity).toString(),
                                price: parseFloat(t.price || 0).toString(),
                                product: t.product,
                                validity: t.validity,
                                disclosed_quantity: parseInt(t.disclosedQuantity || 0).toString(),
                                trigger_price: parseFloat(t.triggerPrice || 0).toString(),
                                squareoff: parseFloat(t.squareoff || 0).toString(),
                                stoploss: parseFloat(t.stoploss || 0).toString(),
                                trailing_stoploss: parseFloat(t.trailingStoploss || 0).toString(),
                                user_id: t.userId || this.userId,
                                tags: t.tags.map(t => t.tag)
                            };
                            if (t.validity === b["b"].VALIDITY.TTL && (e.validity_ttl = parseInt(t.validityTTL || 0).toString()), t.variety === b["b"].VARIETY.ICEBERG && (e.iceberg_legs = this.order.iceberg.legs, e.iceberg_quantity = this.icebergQty), this.gtt.isStopLossEnabled || this.gtt.isTargetEnabled) {
                                const t = [],
                                    r = this.order.transactionType === b["b"].TRANSACTION_TYPE.BUY ? 1 : 0;
                                this.gtt.isStopLossEnabled && t.push([r, parseFloat(this.order.gtt.stoploss)]), this.gtt.isTargetEnabled && t.push([r, parseFloat(this.order.gtt.target)]), this.order.transactionType === b["b"].TRANSACTION_TYPE.SELL && t.reverse(), e.gtt_params = JSON.stringify(t)
                            }
                            return this.windowType === $ && (e.order_id = t.orderId, e.parent_order_id = t.parent_order_id, e.quantity = (parseInt(t.quantity) + (parseInt(t.filledQuantity) || 0)).toString()), this.holdingsAuth.isSkipped && (e.poa = "post"), e
                        },
                        makeCallbackParams(t) {
                            let e = this.$clone(t);
                            return e.quantity = parseInt(e.quantity), e.price = parseFloat(e.price || 0), e.disclosed_quantity = parseInt(e.disclosedQuantity || 0), e.trigger_price = parseFloat(e.triggerPrice || 0), e.squareoff = parseFloat(e.squareoff || 0), e.stoploss = parseFloat(e.stoploss || 0), e.trailing_stoploss = parseFloat(e.trailingStoploss || 0), this.gtt.isStopLossEnabled || this.gtt.isTargetEnabled ? (this.gtt.isStopLossEnabled ? e.gtt.stoploss = parseFloat(e.gtt.stoploss) : e.gtt.stoploss = 0, this.gtt.isTargetEnabled ? e.gtt.target = parseFloat(e.gtt.target) : e.gtt.target = 0) : delete e.gtt, e
                        },
                        isMarginChanged(t, e) {
                            return t.transactionType !== e.transactionType || t.quantity !== e.quantity || t.orderType !== e.orderType || t.variety !== e.variety || t.product !== e.product || t.triggerPrice !== e.triggerPrice
                        },
                        formatDescription(t) {
                            return Object(h["i"])(t)
                        },
                        finishHoldingsAuth(t) {
                            this.holdingsAuth.isSkipped = t && "skip" === t.status
                        },
                        setDefaultProduct() {
                            this.order.instrument.exchange === b["b"].EXCHANGES.NSE || this.order.instrument.exchange === b["b"].EXCHANGES.BSE ? (this.isEquity = !0, this.isFO = !1) : (this.isEquity = !1, this.isFO = !0), this.isEquity ? this.order.product !== b["b"].PRODUCT.CNC && this.order.product !== b["b"].PRODUCT.MIS && (this.order.product = b["b"].PRODUCT.CNC) : this.isFO && this.order.product !== b["b"].PRODUCT.NRML && this.order.product !== b["b"].PRODUCT.MIS && (this.order.product = b["b"].PRODUCT.NRML)
                        },
                        updateOrderPrefs() {
                            this.setOrderPrefs({
                                product: this.order.product,
                                validity: this.order.validity,
                                variety: this.order.variety,
                                orderType: this.order.orderType,
                                tags: this.order.tags,
                                isMoreVisible: this.isMoreVisible
                            })
                        },
                        onToggleTags() {
                            this.isTagsVisible = !this.isTagsVisible, this.adjustWidth(), Object(v["d"])(m["b"], "isTagsVisible", this.isTagsVisible)
                        },
                        adjustWidth() {
                            const t = this.isTagsVisible ? 725 : 550,
                                e = document.querySelector(":root");
                            e.style.setProperty("--order-window-width", t + "px")
                        }
                    }),
                    computed: D(D(D(D(D({}, Object(n["c"])(["isMobile"])), Object(n["c"])(y["b"], ["userId", "preferences"])), Object(n["c"])(m["b"], ["orderPrefs"])), Object(n["c"])(T["b"], ["ticks", "tickerConnectionStatus"])), {}, {
                        txType() {
                            let t = this.order.transactionType.toLowerCase();
                            return t.charAt(0).toUpperCase() + t.slice(1)
                        },
                        icebergQty() {
                            const t = this.order.quantity / Math.max(this.order.iceberg.legs, 2);
                            return Math.ceil(t / this.curInstrument.lotSize) * this.curInstrument.lotSize
                        },
                        enabledProducts() {
                            const t = [];
                            for (let e of V.products) this.constraints[e.name] && t.push(e);
                            return t
                        },
                        enabledVarieties() {
                            const t = [];
                            for (let e of V.varities) this.constraints[e.name] && t.push(e);
                            return t
                        },
                        isNudgePrompt() {
                            if (this.nudge && this.nudge.data)
                                for (let t of this.nudge.data)
                                    if (t.prompt) return !0;
                            return !1
                        },
                        isStickyOrderWindow() {
                            return this.preferences && this.preferences.sticky_orderwindow && "true" === this.preferences.sticky_orderwindow
                        }
                    }),
                    created() {
                        this.isMoreVisible = this.orderPrefs.isMoreVisible, this.adjustWidth(), this.$events.on(b["b"].EVENTS.ORDER_PLACE, t => {
                            this.openWindow(j, t)
                        }), this.$events.on(b["b"].EVENTS.ORDER_MODIFY, t => {
                            this.openWindow($, t)
                        })
                    }
                },
                B = z,
                Q = Object(q["a"])(B, s, i, !1, null, null, null);
            e["a"] = Q.exports
        },
        "4f50": function(t, e, r) {
            var s = r("b760"),
                i = r("e538"),
                n = r("c8fe"),
                a = r("4359"),
                o = r("fa21"),
                l = r("d370"),
                c = r("6747"),
                u = r("dcbe"),
                d = r("0d24"),
                h = r("9520"),
                p = r("1a8c"),
                g = r("60ed"),
                f = r("73ac"),
                b = r("8adb"),
                m = r("8de2");

            function v(t, e, r, v, y, T, S) {
                var w = b(t, r),
                    E = b(e, r),
                    O = S.get(E);
                if (O) s(t, r, O);
                else {
                    var _ = T ? T(w, E, r + "", t, e, S) : void 0,
                        P = void 0 === _;
                    if (P) {
                        var I = c(E),
                            C = !I && d(E),
                            k = !I && !C && f(E);
                        _ = E, I || C || k ? c(w) ? _ = w : u(w) ? _ = a(w) : C ? (P = !1, _ = i(E, !0)) : k ? (P = !1, _ = n(E, !0)) : _ = [] : g(E) || l(E) ? (_ = w, l(w) ? _ = m(w) : p(w) && !h(w) || (_ = o(E))) : P = !1
                    }
                    P && (S.set(E, _), y(_, E, v, T, S), S["delete"](E)), s(t, r, _)
                }
            }
            t.exports = v
        },
        "598f": function(t, e, r) {
            "use strict";
            var s = r("ba6a"),
                i = r("5665");

            function n() {
                return s["a"].get(Object(i["a"])("user.profile"))
            }

            function a() {
                return s["a"].get(Object(i["a"])("user.session"))
            }

            function o() {
                return s["a"].get(Object(i["a"])("user.appSessions"))
            }

            function l(t) {
                return s["a"].delete(Object(i["a"])("user.appSessions"), {
                    params: t
                })
            }

            function c(t) {
                return s["a"].post(Object(i["a"])("user.login"), t)
            }

            function u(t) {
                return s["a"].post(Object(i["a"])("user.twofa"), t)
            }

            function d(t) {
                return s["a"].put(Object(i["a"])("user.changePassword"), t)
            }

            function h(t) {
                return s["a"].put(Object(i["a"])("user.changeTwoFA"), t)
            }

            function p(t) {
                return s["a"].get(Object(i["a"])("user.totp"), t)
            }

            function g(t) {
                return s["a"].post(Object(i["a"])("user.initiatePasswordReset"), t)
            }

            function f(t) {
                return s["a"].post(Object(i["a"])("user.validatePasswordReset"), t)
            }

            function b(t) {
                return s["a"].put(Object(i["a"])("user.doPasswordReset"), t)
            }

            function m(t) {
                return s["a"].delete(Object(i["a"])("user.deleteAvatar"))
            }

            function v(t) {
                return s["a"].get(Object(i["a"])("preferences.all"), {
                    params: t
                })
            }

            function y(t) {
                return s["a"].post(Object(i["a"])("preferences.update"), t)
            }

            function T(t) {
                return s["a"].post(Object(i["a"])("user.twofaValidate"), t)
            }

            function S() {
                return s["a"].put(Object(i["a"])("otp.initiate"))
            }

            function w(t) {
                return s["a"].post(Object(i["a"])("otp.validate", {
                    reqID: t
                }))
            }

            function E(t) {
                return s["a"].post(Object(i["a"])("user.vpa.validate"), {
                    vpa: t
                })
            }

            function O() {
                return s["a"].get(Object(i["a"])("user.vpa"))
            }
            e["a"] = {
                getProfile: n,
                getSession: a,
                getAppSessions: o,
                clearAppSessions: l,
                login: c,
                twofa: u,
                changePassword: d,
                changeTwoFA: h,
                getTOTP: p,
                initiatePasswordReset: g,
                validatePasswordReset: f,
                doPasswordReset: b,
                deleteAvatar: m,
                getPreferences: v,
                updatePreferences: y,
                twofaValidate: T,
                initiateOTP: S,
                validateOTP: w,
                getVPA: O,
                validateVPA: E
            }
        },
        "60ed": function(t, e, r) {
            var s = r("3729"),
                i = r("2dcb"),
                n = r("1310"),
                a = "[object Object]",
                o = Function.prototype,
                l = Object.prototype,
                c = o.toString,
                u = l.hasOwnProperty,
                d = c.call(Object);

            function h(t) {
                if (!n(t) || s(t) != a) return !1;
                var e = i(t);
                if (null === e) return !0;
                var r = u.call(e, "constructor") && e.constructor;
                return "function" == typeof r && r instanceof r && c.call(r) == d
            }
            t.exports = h
        },
        "72af": function(t, e, r) {
            var s = r("99cd"),
                i = s();
            t.exports = i
        },
        "72f0": function(t, e) {
            function r(t) {
                return function() {
                    return t
                }
            }
            t.exports = r
        },
        "85e3": function(t, e) {
            function r(t, e, r) {
                switch (r.length) {
                    case 0:
                        return t.call(e);
                    case 1:
                        return t.call(e, r[0]);
                    case 2:
                        return t.call(e, r[0], r[1]);
                    case 3:
                        return t.call(e, r[0], r[1], r[2])
                }
                return t.apply(e, r)
            }
            t.exports = r
        },
        "8adb": function(t, e) {
            function r(t, e) {
                if (("constructor" !== e || "function" !== typeof t[e]) && "__proto__" != e) return t[e]
            }
            t.exports = r
        },
        "8de2": function(t, e, r) {
            var s = r("8eeb"),
                i = r("9934");

            function n(t) {
                return s(t, i(t))
            }
            t.exports = n
        },
        9285: function(t, e, r) {
            "use strict";
            r.d(e, "a", (function() {
                return s
            })), r.d(e, "c", (function() {
                return i
            })), r.d(e, "b", (function() {
                return n
            }));
            const s = "chartiq",
                i = "tradingview",
                n = {
                    [s]: "chart",
                    [i]: "tvchart"
                }
        },
        "99cd": function(t, e) {
            function r(t) {
                return function(e, r, s) {
                    var i = -1,
                        n = Object(e),
                        a = s(e),
                        o = a.length;
                    while (o--) {
                        var l = a[t ? o : ++i];
                        if (!1 === r(n[l], l, n)) break
                    }
                    return e
                }
            }
            t.exports = r
        },
        "9aff": function(t, e, r) {
            var s = r("9638"),
                i = r("30c9"),
                n = r("c098"),
                a = r("1a8c");

            function o(t, e, r) {
                if (!a(r)) return !1;
                var o = typeof e;
                return !!("number" == o ? i(r) && n(e, r.length) : "string" == o && e in r) && s(r[e], t)
            }
            t.exports = o
        },
        "9c9e": function(t, e, r) {
            "use strict";
            var s = r("5665"),
                i = r("2f62"),
                n = r("d1de"),
                a = r("9285");

            function o(t, e) {
                var r = Object.keys(t);
                if (Object.getOwnPropertySymbols) {
                    var s = Object.getOwnPropertySymbols(t);
                    e && (s = s.filter((function(e) {
                        return Object.getOwnPropertyDescriptor(t, e).enumerable
                    }))), r.push.apply(r, s)
                }
                return r
            }

            function l(t) {
                for (var e = 1; e < arguments.length; e++) {
                    var r = null != arguments[e] ? arguments[e] : {};
                    e % 2 ? o(Object(r), !0).forEach((function(e) {
                        c(t, e, r[e])
                    })) : Object.getOwnPropertyDescriptors ? Object.defineProperties(t, Object.getOwnPropertyDescriptors(r)) : o(Object(r)).forEach((function(e) {
                        Object.defineProperty(t, e, Object.getOwnPropertyDescriptor(r, e))
                    }))
                }
                return t
            }

            function c(t, e, r) {
                return e in t ? Object.defineProperty(t, e, {
                    value: r,
                    enumerable: !0,
                    configurable: !0,
                    writable: !0
                }) : t[e] = r, t
            }
            e["a"] = {
                inject: [],
                data() {
                    return {
                        sentinelURL: "https://sentinel.zerodha.com/dashboard?stockA="
                    }
                },
                computed: l(l({}, Object(i["c"])(["theme"])), Object(i["c"])(n["b"], ["preferences"])),
                methods: {
                    getStatic(t) {
                        return "/static/" + t
                    },
                    appendThemeName(t) {
                        let e = t.split(".");
                        return this.theme && "default" !== this.theme && e.length >= 2 ? (e[e.length - 2] = e[e.length - 2] + "-" + this.theme, e.join(".")) : t
                    },
                    openInAppChart(t, e, r) {
                        let s = this.preferences.chart_type,
                            i = {
                                name: a["b"][s] || a["b"][a["a"]],
                                params: {
                                    token: t,
                                    segment: e,
                                    tradingsymbol: r
                                }
                            };
                        "dark" === this.theme && (i["query"] = {
                            theme: "dark"
                        }), this.$router.push(i)
                    },
                    openPopoutChart(t, e, r) {
                        let i = this.preferences.chart_type,
                            n = s["b"].extChart;
                        return i === a["c"] && (n = s["b"].extTvchart), n = n.replace(":segment", e), n = n.replace(":tradingsymbol", r), n = n.replace(":token", t), "dark" === this.theme && (n += "?theme=dark"), n
                    }
                }
            }
        },
        "9dcd": function(t, e, r) {
            var s;
            (function(i) {
                "use strict";
                var n, a = 20,
                    o = 1,
                    l = 1e6,
                    c = 1e6,
                    u = -7,
                    d = 21,
                    h = "[big.js] ",
                    p = h + "Invalid ",
                    g = p + "decimal places",
                    f = p + "rounding mode",
                    b = h + "Division by zero",
                    m = {},
                    v = void 0,
                    y = /^-?(\d+(\.\d*)?|\.\d+)(e[+-]?\d+)?$/i;

                function T() {
                    function t(e) {
                        var r = this;
                        if (!(r instanceof t)) return e === v ? T() : new t(e);
                        e instanceof t ? (r.s = e.s, r.e = e.e, r.c = e.c.slice()) : S(r, e), r.constructor = t
                    }
                    return t.prototype = m, t.DP = a, t.RM = o, t.NE = u, t.PE = d, t.version = "5.2.2", t
                }

                function S(t, e) {
                    var r, s, i;
                    if (0 === e && 1 / e < 0) e = "-0";
                    else if (!y.test(e += "")) throw Error(p + "number");
                    for (t.s = "-" == e.charAt(0) ? (e = e.slice(1), -1) : 1, (r = e.indexOf(".")) > -1 && (e = e.replace(".", "")), (s = e.search(/e/i)) > 0 ? (r < 0 && (r = s), r += +e.slice(s + 1), e = e.substring(0, s)) : r < 0 && (r = e.length), i = e.length, s = 0; s < i && "0" == e.charAt(s);) ++s;
                    if (s == i) t.c = [t.e = 0];
                    else {
                        for (; i > 0 && "0" == e.charAt(--i););
                        for (t.e = r - s - 1, t.c = [], r = 0; s <= i;) t.c[r++] = +e.charAt(s++)
                    }
                    return t
                }

                function w(t, e, r, s) {
                    var i = t.c,
                        n = t.e + e + 1;
                    if (n < i.length) {
                        if (1 === r) s = i[n] >= 5;
                        else if (2 === r) s = i[n] > 5 || 5 == i[n] && (s || n < 0 || i[n + 1] !== v || 1 & i[n - 1]);
                        else if (3 === r) s = s || !!i[0];
                        else if (s = !1, 0 !== r) throw Error(f);
                        if (n < 1) i.length = 1, s ? (t.e = -e, i[0] = 1) : i[0] = t.e = 0;
                        else {
                            if (i.length = n--, s)
                                for (; ++i[n] > 9;) i[n] = 0, n-- || (++t.e, i.unshift(1));
                            for (n = i.length; !i[--n];) i.pop()
                        }
                    } else if (r < 0 || r > 3 || r !== ~~r) throw Error(f);
                    return t
                }

                function E(t, e, r, s) {
                    var i, n, a = t.constructor,
                        o = !t.c[0];
                    if (r !== v) {
                        if (r !== ~~r || r < (3 == e) || r > l) throw Error(3 == e ? p + "precision" : g);
                        for (t = new a(t), r = s - t.e, t.c.length > ++s && w(t, r, a.RM), 2 == e && (s = t.e + r + 1); t.c.length < s;) t.c.push(0)
                    }
                    if (i = t.e, n = t.c.join(""), r = n.length, 2 != e && (1 == e || 3 == e && s <= i || i <= a.NE || i >= a.PE)) n = n.charAt(0) + (r > 1 ? "." + n.slice(1) : "") + (i < 0 ? "e" : "e+") + i;
                    else if (i < 0) {
                        for (; ++i;) n = "0" + n;
                        n = "0." + n
                    } else if (i > 0)
                        if (++i > r)
                            for (i -= r; i--;) n += "0";
                        else i < r && (n = n.slice(0, i) + "." + n.slice(i));
                    else r > 1 && (n = n.charAt(0) + "." + n.slice(1));
                    return t.s < 0 && (!o || 4 == e) ? "-" + n : n
                }
                m.abs = function() {
                    var t = new this.constructor(this);
                    return t.s = 1, t
                }, m.cmp = function(t) {
                    var e, r = this,
                        s = r.c,
                        i = (t = new r.constructor(t)).c,
                        n = r.s,
                        a = t.s,
                        o = r.e,
                        l = t.e;
                    if (!s[0] || !i[0]) return s[0] ? n : i[0] ? -a : 0;
                    if (n != a) return n;
                    if (e = n < 0, o != l) return o > l ^ e ? 1 : -1;
                    for (a = (o = s.length) < (l = i.length) ? o : l, n = -1; ++n < a;)
                        if (s[n] != i[n]) return s[n] > i[n] ^ e ? 1 : -1;
                    return o == l ? 0 : o > l ^ e ? 1 : -1
                }, m.div = function(t) {
                    var e = this,
                        r = e.constructor,
                        s = e.c,
                        i = (t = new r(t)).c,
                        n = e.s == t.s ? 1 : -1,
                        a = r.DP;
                    if (a !== ~~a || a < 0 || a > l) throw Error(g);
                    if (!i[0]) throw Error(b);
                    if (!s[0]) return new r(0 * n);
                    var o, c, u, d, h, p = i.slice(),
                        f = o = i.length,
                        m = s.length,
                        y = s.slice(0, o),
                        T = y.length,
                        S = t,
                        E = S.c = [],
                        O = 0,
                        _ = a + (S.e = e.e - t.e) + 1;
                    for (S.s = n, n = _ < 0 ? 0 : _, p.unshift(0); T++ < o;) y.push(0);
                    do {
                        for (u = 0; u < 10; u++) {
                            if (o != (T = y.length)) d = o > T ? 1 : -1;
                            else
                                for (h = -1, d = 0; ++h < o;)
                                    if (i[h] != y[h]) {
                                        d = i[h] > y[h] ? 1 : -1;
                                        break
                                    } if (!(d < 0)) break;
                            for (c = T == o ? i : p; T;) {
                                if (y[--T] < c[T]) {
                                    for (h = T; h && !y[--h];) y[h] = 9;
                                    --y[h], y[T] += 10
                                }
                                y[T] -= c[T]
                            }
                            for (; !y[0];) y.shift()
                        }
                        E[O++] = d ? u : ++u, y[0] && d ? y[T] = s[f] || 0 : y = [s[f]]
                    } while ((f++ < m || y[0] !== v) && n--);
                    return E[0] || 1 == O || (E.shift(), S.e--), O > _ && w(S, a, r.RM, y[0] !== v), S
                }, m.eq = function(t) {
                    return !this.cmp(t)
                }, m.gt = function(t) {
                    return this.cmp(t) > 0
                }, m.gte = function(t) {
                    return this.cmp(t) > -1
                }, m.lt = function(t) {
                    return this.cmp(t) < 0
                }, m.lte = function(t) {
                    return this.cmp(t) < 1
                }, m.minus = m.sub = function(t) {
                    var e, r, s, i, n = this,
                        a = n.constructor,
                        o = n.s,
                        l = (t = new a(t)).s;
                    if (o != l) return t.s = -l, n.plus(t);
                    var c = n.c.slice(),
                        u = n.e,
                        d = t.c,
                        h = t.e;
                    if (!c[0] || !d[0]) return d[0] ? (t.s = -l, t) : new a(c[0] ? n : 0);
                    if (o = u - h) {
                        for ((i = o < 0) ? (o = -o, s = c) : (h = u, s = d), s.reverse(), l = o; l--;) s.push(0);
                        s.reverse()
                    } else
                        for (r = ((i = c.length < d.length) ? c : d).length, o = l = 0; l < r; l++)
                            if (c[l] != d[l]) {
                                i = c[l] < d[l];
                                break
                            } if (i && (s = c, c = d, d = s, t.s = -t.s), (l = (r = d.length) - (e = c.length)) > 0)
                        for (; l--;) c[e++] = 0;
                    for (l = e; r > o;) {
                        if (c[--r] < d[r]) {
                            for (e = r; e && !c[--e];) c[e] = 9;
                            --c[e], c[r] += 10
                        }
                        c[r] -= d[r]
                    }
                    for (; 0 === c[--l];) c.pop();
                    for (; 0 === c[0];) c.shift(), --h;
                    return c[0] || (t.s = 1, c = [h = 0]), t.c = c, t.e = h, t
                }, m.mod = function(t) {
                    var e, r = this,
                        s = r.constructor,
                        i = r.s,
                        n = (t = new s(t)).s;
                    if (!t.c[0]) throw Error(b);
                    return r.s = t.s = 1, e = 1 == t.cmp(r), r.s = i, t.s = n, e ? new s(r) : (i = s.DP, n = s.RM, s.DP = s.RM = 0, r = r.div(t), s.DP = i, s.RM = n, this.minus(r.times(t)))
                }, m.plus = m.add = function(t) {
                    var e, r = this,
                        s = r.constructor,
                        i = r.s,
                        n = (t = new s(t)).s;
                    if (i != n) return t.s = -n, r.minus(t);
                    var a = r.e,
                        o = r.c,
                        l = t.e,
                        c = t.c;
                    if (!o[0] || !c[0]) return c[0] ? t : new s(o[0] ? r : 0 * i);
                    if (o = o.slice(), i = a - l) {
                        for (i > 0 ? (l = a, e = c) : (i = -i, e = o), e.reverse(); i--;) e.push(0);
                        e.reverse()
                    }
                    for (o.length - c.length < 0 && (e = c, c = o, o = e), i = c.length, n = 0; i; o[i] %= 10) n = (o[--i] = o[i] + c[i] + n) / 10 | 0;
                    for (n && (o.unshift(n), ++l), i = o.length; 0 === o[--i];) o.pop();
                    return t.c = o, t.e = l, t
                }, m.pow = function(t) {
                    var e = this,
                        r = new e.constructor(1),
                        s = r,
                        i = t < 0;
                    if (t !== ~~t || t < -c || t > c) throw Error(p + "exponent");
                    for (i && (t = -t);;) {
                        if (1 & t && (s = s.times(e)), t >>= 1, !t) break;
                        e = e.times(e)
                    }
                    return i ? r.div(s) : s
                }, m.round = function(t, e) {
                    var r = this.constructor;
                    if (t === v) t = 0;
                    else if (t !== ~~t || t < -l || t > l) throw Error(g);
                    return w(new r(this), t, e === v ? r.RM : e)
                }, m.sqrt = function() {
                    var t, e, r, s = this,
                        i = s.constructor,
                        n = s.s,
                        a = s.e,
                        o = new i(.5);
                    if (!s.c[0]) return new i(s);
                    if (n < 0) throw Error(h + "No square root");
                    n = Math.sqrt(s + ""), 0 === n || n === 1 / 0 ? (e = s.c.join(""), e.length + a & 1 || (e += "0"), n = Math.sqrt(e), a = ((a + 1) / 2 | 0) - (a < 0 || 1 & a), t = new i((n == 1 / 0 ? "1e" : (n = n.toExponential()).slice(0, n.indexOf("e") + 1)) + a)) : t = new i(n), a = t.e + (i.DP += 4);
                    do {
                        r = t, t = o.times(r.plus(s.div(r)))
                    } while (r.c.slice(0, a).join("") !== t.c.slice(0, a).join(""));
                    return w(t, i.DP -= 4, i.RM)
                }, m.times = m.mul = function(t) {
                    var e, r = this,
                        s = r.constructor,
                        i = r.c,
                        n = (t = new s(t)).c,
                        a = i.length,
                        o = n.length,
                        l = r.e,
                        c = t.e;
                    if (t.s = r.s == t.s ? 1 : -1, !i[0] || !n[0]) return new s(0 * t.s);
                    for (t.e = l + c, a < o && (e = i, i = n, n = e, c = a, a = o, o = c), e = new Array(c = a + o); c--;) e[c] = 0;
                    for (l = o; l--;) {
                        for (o = 0, c = a + l; c > l;) o = e[c] + n[l] * i[c - l - 1] + o, e[c--] = o % 10, o = o / 10 | 0;
                        e[c] = (e[c] + o) % 10
                    }
                    for (o ? ++t.e : e.shift(), l = e.length; !e[--l];) e.pop();
                    return t.c = e, t
                }, m.toExponential = function(t) {
                    return E(this, 1, t, t)
                }, m.toFixed = function(t) {
                    return E(this, 2, t, this.e + t)
                }, m.toPrecision = function(t) {
                    return E(this, 3, t, t - 1)
                }, m.toString = function() {
                    return E(this)
                }, m.valueOf = m.toJSON = function() {
                    return E(this, 4)
                }, n = T(), n["default"] = n.Big = n, s = function() {
                    return n
                }.call(e, r, e, t), void 0 === s || (t.exports = s)
            })()
        },
        a454: function(t, e, r) {
            var s = r("72f0"),
                i = r("3b4a"),
                n = r("cd9d"),
                a = i ? function(t, e) {
                    return i(t, "toString", {
                        configurable: !0,
                        enumerable: !1,
                        value: s(e),
                        writable: !0
                    })
                } : n;
            t.exports = a
        },
        b760: function(t, e, r) {
            var s = r("872a"),
                i = r("9638");

            function n(t, e, r) {
                (void 0 !== r && !i(t[e], r) || void 0 === r && !(e in t)) && s(t, e, r)
            }
            t.exports = n
        },
        c1c9: function(t, e, r) {
            var s = r("a454"),
                i = r("f3c1"),
                n = i(s);
            t.exports = n
        },
        cd9d: function(t, e) {
            function r(t) {
                return t
            }
            t.exports = r
        },
        d1de: function(t, e, r) {
            "use strict";
            r.d(e, "b", (function() {
                return d
            }));
            var s = r("598f"),
                i = r("5fb0"),
                n = r("025e"),
                a = r("b202");

            function o(t, e) {
                var r = Object.keys(t);
                if (Object.getOwnPropertySymbols) {
                    var s = Object.getOwnPropertySymbols(t);
                    e && (s = s.filter((function(e) {
                        return Object.getOwnPropertyDescriptor(t, e).enumerable
                    }))), r.push.apply(r, s)
                }
                return r
            }

            function l(t) {
                for (var e = 1; e < arguments.length; e++) {
                    var r = null != arguments[e] ? arguments[e] : {};
                    e % 2 ? o(Object(r), !0).forEach((function(e) {
                        c(t, e, r[e])
                    })) : Object.getOwnPropertyDescriptors ? Object.defineProperties(t, Object.getOwnPropertyDescriptors(r)) : o(Object(r)).forEach((function(e) {
                        Object.defineProperty(t, e, Object.getOwnPropertyDescriptor(r, e))
                    }))
                }
                return t
            }

            function c(t, e, r) {
                return e in t ? Object.defineProperty(t, e, {
                    value: r,
                    enumerable: !0,
                    configurable: !0,
                    writable: !0
                }) : t[e] = r, t
            }
            let u = !0;
            const d = "user",
                h = {
                    questions: null,
                    questionsError: null,
                    profile: Object(a["b"])(d, "profile") || {},
                    profileError: null,
                    profileFetchStatus: i["a"].initial,
                    userName: Object(a["b"])(d, "user_name") || {},
                    userAvatar: Object(a["b"])(d, "user_avatar") || "",
                    userId: Object(a["b"])(d, "user_id") || "",
                    userEmail: Object(a["b"])(d, "user_email") || "",
                    userUPI: Object(a["b"])(d, "user_upi") || "",
                    questionsFetchStatus: i["a"].initial,
                    appSessions: null,
                    appSessionsError: null,
                    appSessionsFetchStatus: i["a"].initial,
                    twoFAType: null,
                    preferences: null,
                    preferencesError: null,
                    preferencesFetchStatus: i["a"].initial
                },
                p = {
                    profile: t => t.profile,
                    profileError: t => t.profileError,
                    questions: t => t.questions,
                    questionsError: t => t.questionsError,
                    userId: t => t.userId,
                    userEmail: t => t.userEmail,
                    userName: t => t.userName,
                    userAvatar: t => t.userAvatar,
                    userUPI: t => t.userUPI,
                    questionsFetchStatus: t => t.questionsFetchStatus,
                    profileFetchStatus: t => t.profileFetchStatus,
                    appSessions: t => t.appSessions,
                    appSessionsError: t => t.appSessionsError,
                    appSessionsFetchStatus: t => t.appSessionsFetchStatus,
                    twoFAType: t => t.twoFAType,
                    preferences: t => t.preferences,
                    preferencesError: t => t.preferencesError,
                    preferencesFetchStatus: t => t.preferencesFetchStatus
                },
                g = {
                    clearProfile(t) {
                        t.profile = null, t.userName = null, t.userAvatar = null, t.userEmail = null
                    },
                    setProfile(t, e) {
                        t.profile = e, t.userName = {
                            userName: e.user_name,
                            userShortName: e.user_shortname
                        }, t.userAvatar = e.avatar_url, t.userEmail = e.email, t.twoFAType = e.twofa_type, Object(a["d"])(d, "user_name", t.userName), Object(a["d"])(d, "user_avatar", t.userAvatar), Object(a["d"])(d, "user_email", t.userEmail)
                    },
                    setProfileError(t, e) {
                        t.profileError = e
                    },
                    setProfileFetchStatus(t, e) {
                        t.profileFetchStatus = e
                    },
                    setQuestions(t, e) {
                        t.questions = e
                    },
                    setQuestionsError(t, e) {
                        t.questionsError = e
                    },
                    setQuestionsFetchStatus(t, e) {
                        t.questionsFetchStatus = e
                    },
                    setUserName(t, e, r) {
                        t.userName = {
                            userName: e,
                            userShortName: r
                        }, Object(a["d"])(d, "user_name", t.userName)
                    },
                    setUserAvatar(t, e) {
                        t.userAvatar = e, Object(a["d"])(d, "user_avatar", t.userAvatar)
                    },
                    setUserId(t, e) {
                        t.userId = e, Object(a["d"])(d, "user_id", t.userId)
                    },
                    setUserUPI(t, e) {
                        t.userUPI = e, Object(a["d"])(d, "user_upi", t.userUPI)
                    },
                    setAppSessions(t, e) {
                        t.appSessions = e
                    },
                    setAppSessionsError(t, e) {
                        t.appSessionsError = e
                    },
                    setAppSessionsFetchStatus(t, e) {
                        t.appSessionsFetchStatus = e
                    },
                    setTwoFAType(t, e) {
                        t.twoFAType = e
                    },
                    setPreferences(t, e) {
                        t.preferences = e
                    },
                    setPreferencesError(t, e) {
                        t.preferencesError = e
                    },
                    setPreferencesFetchStatus(t, e) {
                        t.preferencesFetchStatus = e
                    },
                    updatePreference(t, e) {
                        t.preferences = l(l({}, t.preferences), {}, {
                            [e.key]: e.value
                        })
                    }
                },
                f = {
                    fetchProfile({
                        commit: t,
                        state: e
                    }) {
                        t("setProfileFetchStatus", i["a"].fetching);
                        let r = s["a"].getProfile();
                        Object(n["g"])({
                            commit: t,
                            apiPromise: r,
                            data: "setProfile",
                            error: "setProfileError",
                            status: "setProfileFetchStatus"
                        })
                    },
                    fetchQuestions({
                        commit: t,
                        state: e
                    }) {
                        t("setQuestionsFetchStatus", i["a"].fetching);
                        let r = s["a"].getTwofaQuestions();
                        Object(n["g"])({
                            commit: t,
                            apiPromise: r,
                            data: "setQuestions",
                            error: "setQuestionsError",
                            status: "setQuestionsFetchStatus"
                        })
                    },
                    fetchAppSessions({
                        commit: t,
                        state: e
                    }) {
                        t("setAppSessionsFetchStatus", i["a"].fetching);
                        let r = s["a"].getAppSessions();
                        Object(n["g"])({
                            commit: t,
                            apiPromise: r,
                            data: "setAppSessions",
                            error: "setAppSessionsError",
                            status: "setAppSessionsFetchStatus"
                        })
                    },
                    fetchPreferences({
                        commit: t,
                        state: e
                    }) {
                        t("setPreferencesFetchStatus", i["a"].fetching);
                        let r = s["a"].getPreferences();
                        Object(n["g"])({
                            commit: t,
                            apiPromise: r,
                            data: "setPreferences",
                            error: "setPreferencesError",
                            status: "setPreferencesFetchStatus"
                        })
                    }
                };
            e["a"] = {
                state: h,
                getters: p,
                mutations: g,
                actions: f,
                namespaced: u
            }
        },
        dcbe: function(t, e, r) {
            var s = r("30c9"),
                i = r("1310");

            function n(t) {
                return i(t) && s(t)
            }
            t.exports = n
        },
        f3c1: function(t, e) {
            var r = 800,
                s = 16,
                i = Date.now;

            function n(t) {
                var e = 0,
                    n = 0;
                return function() {
                    var a = i(),
                        o = s - (a - n);
                    if (n = a, o > 0) {
                        if (++e >= r) return arguments[0]
                    } else e = 0;
                    return t.apply(void 0, arguments)
                }
            }
            t.exports = n
        },
        f909: function(t, e, r) {
            var s = r("7e64"),
                i = r("b760"),
                n = r("72af"),
                a = r("4f50"),
                o = r("1a8c"),
                l = r("9934"),
                c = r("8adb");

            function u(t, e, r, d, h) {
                t !== e && n(e, (function(n, l) {
                    if (h || (h = new s), o(n)) a(t, e, l, r, u, d, h);
                    else {
                        var p = d ? d(c(t, l), n, l + "", t, e, h) : void 0;
                        void 0 === p && (p = n), i(t, l, p)
                    }
                }), l)
            }
            t.exports = u
        }
    }
]);