(window["webpackJsonp"] = window["webpackJsonp"] || []).push([
    ["chartiq~connect~main~main-chartiq~main-tradingview~tradingview"], {
        "0a56": function(e, t, s) {
            "use strict";
            var i = s("4a7c"),
                n = s.n(i);
            class r {
                constructor() {
                    var e = this;
                    this.promise = new Promise((function(t, s) {
                        e.resolve = t, e.reject = s
                    }))
                }
            }
            class o {
                constructor() {
                    this.handlers = []
                }
                on(e) {
                    this.handlers.push(e)
                }
                off(e) {
                    this.handlers = this.handlers.filter(t => t !== e)
                }
                trigger(e) {
                    this.handlers.slice(0).forEach(t => t(e))
                }
            }
            class a {
                constructor(e) {
                    this.mSubscribe = "subscribe", this.mUnSubscribe = "unsubscribe", this.mSetMode = "mode", this.mGetQuote = "quote", this.mAlert = 10, this.mOrderStr = "order", this.mMessage = 11, this.mMessageStr = "message", this.mLogout = 12, this.mLogoutStr = "logout", this.mReload = 13, this.mReloadStr = "reload", this.mClearCache = 14, this.mClearCacheStr = "clear_cache", this.modeLTP = "ltp", this.modeLTPC = "ltpc", this.modeFull = "full", this.modeQuote = "quote", this.modeWeights = {
                        [this.modeFull]: 1,
                        [this.modeQuote]: 2,
                        [this.modeLTPC]: 3,
                        [this.modeLTP]: 4
                    }, this.weightModeMap = {
                        1: this.modeFull,
                        2: this.modeQuote,
                        3: this.modeLTPC,
                        4: this.modeLTP
                    }, this.segmentNseCM = 1, this.segmentNseFO = 2, this.segmentNseCD = 3, this.segmentBseCM = 4, this.segmentBseFO = 5, this.segmentBseCD = 6, this.segmentMcxFO = 7, this.segmentMcxSX = 8, this.segmentNseIndices = 9, this.segmentUS = 11, this.eventConnect = new o, this.eventTick = new o, this.eventData = new o, this.eventDisconnect = new o, this.eventReconnect = new o, this.eventNoReconnect = new o, this.eventAlert = new o, this.eventMessage = new o, this.eventReload = new o, this.eventClearCache = new o, this.eventLogout = new o, this.noReplyTimeout = 5, this.lazyDisconnectTimeout = 10, this.reconnectInterval = 5, this.reconnectTries = 300, this.isAutoReconnect = !0, this.reconnectionsCount = 0, this.currentWsUrl = null, this.tokenTags = {}, this.subscribedTokens = [], this.defaultTokenTag = "_", this.version = "1.0.0", this.userAgent = "kite3-web", this.quoteMap = {}, this.getQuoteTimeout = 5, this.isLazy = !1, this.isLazyInitialConnect = !1, this.lazyPayload = [], this.address = e.address, this.apiKey = e.apiKey, this.encToken = e.encToken, this.userId = e.userId, e.version && (this.version = e.version), this.debug = e.debug
                }
                setParams(e) {
                    this.address = e.address, this.apiKey = e.apiKey, this.encToken = e.encToken, this.userId = e.userId, this.debug = e.debug, e.version && (this.version = e.version), e.lazyDisconnectTimeout && (this.lazyDisconnectTimeout = e.lazyDisconnectTimeout)
                }
                isConnected() {
                    return !(!this.ws || this.ws.readyState !== this.ws.OPEN)
                }
                setAutoReconnect(e, t) {
                    this.isAutoReconnect = e, this.reconnectTries = t
                }
                getsubscribedTokens() {
                    return this.subscribedTokens
                }
                lazyConnect() {
                    this.isLazy = !0
                }
                processLazyPayload() {
                    if (this.isConnected())
                        for (let e of this.lazyPayload) this._send(e), this.lazyPayload.shift();
                    else this.ws && this.ws.readyState === this.ws.CONNECTING && setTimeout(() => {
                        this.processLazyPayload()
                    }, 500), this.isLazyInitialConnect || (this.isAutoReconnect = !0, this.isLazyInitialConnect = !0, this.connect(), this.processLazyPayload())
                }
                connect(e, t) {
                    if (this.ws && (this.ws.readyState === this.ws.CONNECTING || this.ws.readyState === this.ws.OPEN)) return;
                    let s = new n.a({
                        api_key: this.apiKey,
                        user_id: this.userId,
                        enctoken: this.encToken,
                        uid: (new Date).getTime().toString(),
                        "user-agent": this.userAgent,
                        version: this.version
                    });
                    this.ws = new WebSocket(this.address + "?" + s.toString()), this.ws.binaryType = "arraybuffer", this.ws.onopen = e => {
                        this.resubscribe(), this.eventConnect.trigger(), this.setConnectionTimer(), this.isLazy && this.setLazyDisconnect()
                    }, this.ws.onmessage = e => {
                        if (this.eventData.trigger(e.data), e.data instanceof ArrayBuffer) {
                            if (e.data.byteLength > 2) {
                                var t = this.parseBinary(e.data);
                                t && this.eventTick.trigger(t)
                            }
                        } else this.processMessage(e.data);
                        this.lastDataReceivedTime = new Date
                    }, this.ws.onerror = e => {
                        this.ws && this.ws.readyState === this.ws.OPEN && this.ws.close()
                    }, this.ws.onclose = e => {
                        this.currentWsUrl && this.url !== this.currentWsUrl || this.triggerDisconnect()
                    }
                }
                subscribe(e, t) {
                    t = this.getTag(t);
                    let s = [];
                    for (let i of e) "number" !== typeof i || isNaN(i) || this.isElementInArray(this.subscribedTokens, i) || (s.push(i), this.tokenTags[i] = {
                        mode: "",
                        tags: {}
                    }, this.subscribedTokens.push(i));
                    return s.length > 0 && this.send({
                        a: this.mSubscribe,
                        v: s
                    }), s
                }
                unsubscribe(e, t) {
                    t = this.getTag(t);
                    let s = [];
                    for (let i of e) "number" !== typeof i || isNaN(i) || (this.deleteTokenTags(i, t), this.canUnsubscribe(i, t) && (s.push(i), this.deleteSubscriptionToken(i), delete this.tokenTags[i]));
                    return s.length > 0 && this.send({
                        a: this.mUnSubscribe,
                        v: s
                    }), s
                }
                setMode(e, t, s) {
                    s = this.getTag(s);
                    let i = {};
                    for (let n of t) {
                        if (!this.isElementInArray(this.subscribedTokens, n)) {
                            this.deleteTokenTags(n, s);
                            continue
                        }
                        if (e === this.tokenTags[n].mode) continue;
                        if ("number" !== typeof n || isNaN(n)) continue;
                        this.updateTokenTags(n, e, s);
                        let t = this.getBestMode(n, e, s);
                        t && t !== this.tokenTags[n].mode && (i[t] || (i[t] = []), i[t].push(n)), this.tokenTags[n].mode = t
                    }
                    for (let n of Object.keys(i)) this.send({
                        a: this.mSetMode,
                        v: [n, i[n]]
                    })
                }
                resubscribe() {
                    if (0 === this.subscribedTokens.length) return;
                    let e = {},
                        t = [];
                    for (let i of this.subscribedTokens) "number" !== typeof i || isNaN(i) || (t.push(i), this.tokenTags[i] && this.tokenTags[i].mode && (e[this.tokenTags[i].mode] || (e[this.tokenTags[i].mode] = []), e[this.tokenTags[i].mode].push(i)));
                    for (var s of (this.send({
                            a: this.mSubscribe,
                            v: t
                        }), Object.keys(e))) this.send({
                        a: this.mSetMode,
                        v: [s, e[s]]
                    })
                }
                getQuote(e, t, s, i) {
                    return this.quoteMap[e] = new r, i || (i = this.getQuoteTimeout), setTimeout(() => {
                        let t = this.quoteMap[e];
                        t && (t.reject(), delete this.quoteMap[e])
                    }, 1e3 * i), this.send({
                        id: e,
                        a: this.mGetQuote,
                        v: {
                            fields: s,
                            tokens: t
                        }
                    }), this.quoteMap[e].promise
                }
                isElementInArray(e, t) {
                    let s = e.filter(e => e === t);
                    return s.length > 0
                }
                deleteSubscriptionToken(e) {
                    let t = this.subscribedTokens.indexOf(e);
                    t > -1 && this.subscribedTokens.splice(t, 1)
                }
                getTag(e) {
                    return e && "string" === typeof e ? e : this.defaultTokenTag
                }
                updateTokenTags(e, t, s) {
                    s !== this.defaultTokenTag && (this.tokenTags[e] || (this.tokenTags[e] = {
                        mode: t,
                        tags: {}
                    }), this.tokenTags[e]["tags"][s] = this.modeWeights[t])
                }
                deleteTokenTags(e, t) {
                    this.tokenTags[e] && this.tokenTags[e].tags && this.tokenTags[e].tags[t] && delete this.tokenTags[e].tags[t]
                }
                getBestMode(e, t, s) {
                    if (s === this.defaultTokenTag) return t;
                    let i = Math.min.apply(Math, Object.keys(this.tokenTags[e].tags).map(t => this.tokenTags[e].tags[t]));
                    return i ? this.weightModeMap[i] : t
                }
                canUnsubscribe(e, t) {
                    if (!this.isElementInArray(this.subscribedTokens, e)) return !1;
                    if (t === this.defaultTokenTag) return !0;
                    if (!this.tokenTags[e]) return !0;
                    let s = Object.keys(this.tokenTags[e].tags).filter(e => e !== t);
                    return !(s.length > 0)
                }
                triggerDisconnect() {
                    this.eventDisconnect.trigger(), this.isAutoReconnect ? this.attemptReconnection() : this.eventNoReconnect.trigger()
                }
                setConnectionTimer() {
                    clearInterval(this.connectionTimer), this.lastDataReceivedTime = new Date, this.connectionTimer = setInterval(() => {
                        ((new Date).getTime() - this.lastDataReceivedTime.getTime()) / 1e3 >= this.noReplyTimeout && (this.currentWsUrl = null, this.ws && this.ws.close(), clearInterval(this.connectionTimer), this.triggerDisconnect())
                    }, 1e3 * this.noReplyTimeout)
                }
                setLazyDisconnect() {
                    clearInterval(this.lazyDisconnectTimer), this.lazyDisconnectTimer = setInterval(() => {
                        let e = 0 === this.subscribedTokens.length;
                        e && (this.currentWsUrl = null, this.isLazyInitialConnect = !1, this.ws && this.ws.close(), clearInterval(this.lazyDisconnectTimer), this.isAutoReconnect = !1, this.triggerDisconnect())
                    }, 1e3 * this.lazyDisconnectTimeout)
                }
                attemptReconnection() {
                    this.reconnectionsCount > this.reconnectTries ? this.eventNoReconnect.trigger() : (this.eventReconnect.trigger(this.reconnectInterval), setTimeout(() => {
                        this.connect(!0)
                    }, 1e3 * this.reconnectInterval), this.reconnectionsCount++)
                }
                _send(e) {
                    try {
                        this.ws.send(JSON.stringify(e))
                    } catch (t) {
                        this.ws.close()
                    }
                }
                send(e) {
                    this.isConnected() ? this._send(e) : this.isLazy && (this.lazyPayload.push(e), this.processLazyPayload())
                }
                dateToString(e) {
                    let t = e.getFullYear().toString(),
                        s = (e.getMonth() + 1).toString(),
                        i = e.getDate().toString(),
                        n = e.getMinutes().toString(),
                        r = e.getHours().toString(),
                        o = e.getSeconds().toString();
                    s.length < 2 && (s = "0" + s), i.length < 2 && (i = "0" + i), r.length < 2 && (r = "0" + r), n.length < 2 && (n = "0" + n), o.length < 2 && (o = "0" + o);
                    let a = `${t}-${s}-${i} ${r}:${n}:${o}`;
                    return a
                }
                calculateChange(e) {
                    let t = 0,
                        s = 0,
                        i = 0,
                        n = 0;
                    return e.closePrice && (s = e.lastPrice - e.closePrice, t = 100 * s / e.closePrice), e.openPrice && (i = e.lastPrice - e.openPrice, n = 100 * i / e.openPrice), {
                        change: t,
                        absoluteChange: s,
                        openChange: i,
                        openChangePercent: n
                    }
                }
                parseBinary(e) {
                    let t = this.splitPackets(e),
                        s = [];
                    for (let i of t) {
                        let e, t = this.buf2long(i.slice(0, 4)),
                            n = 255 & t,
                            r = 100;
                        switch (n === this.segmentNseCD && (r = 1e7), n === this.segmentBseCD && (r = 1e4), n) {
                            case this.segmentMcxFO:
                            case this.segmentNseCM:
                            case this.segmentBseCM:
                            case this.segmentNseFO:
                            case this.segmentNseCD:
                            case this.segmentBseCD:
                            case this.segmentNseIndices:
                            case this.segmentUS:
                                if (8 === i.byteLength) s.push({
                                    mode: this.modeLTP,
                                    isTradeable: !0,
                                    token: t,
                                    lastPrice: this.buf2long(i.slice(4, 8)) / r
                                });
                                else if (12 === i.byteLength) e = {
                                    mode: this.modeLTPC,
                                    isTradeable: !0,
                                    token: t,
                                    lastPrice: this.buf2long(i.slice(4, 8)) / r,
                                    closePrice: this.buf2long(i.slice(8, 12)) / r
                                }, e = Object.assign(e, this.calculateChange(e)), s.push(e);
                                else if (28 === i.byteLength || 32 === i.byteLength) e = {
                                    mode: this.modeFull,
                                    isTradeable: !1,
                                    token: t,
                                    lastPrice: this.buf2long(i.slice(4, 8)) / r,
                                    highPrice: this.buf2long(i.slice(8, 12)) / r,
                                    lowPrice: this.buf2long(i.slice(12, 16)) / r,
                                    openPrice: this.buf2long(i.slice(16, 20)) / r,
                                    closePrice: this.buf2long(i.slice(20, 24)) / r
                                }, e = Object.assign(e, this.calculateChange(e)), s.push(e);
                                else if (492 === i.byteLength) {
                                    let e = {
                                            mode: this.modeFull,
                                            token: t,
                                            extendedDepth: {
                                                buy: [],
                                                sell: []
                                            }
                                        },
                                        n = 0,
                                        o = i.slice(12, 492);
                                    for (let t = 0; t < 40; t++) n = 12 * t, e.extendedDepth[t < 20 ? "buy" : "sell"].push({
                                        quantity: this.buf2long(o.slice(n, n + 4)),
                                        price: this.buf2long(o.slice(n + 4, n + 8)) / r,
                                        orders: this.buf2long(o.slice(n + 8, n + 12))
                                    });
                                    s.push(e)
                                } else {
                                    if (e = {
                                            mode: this.modeQuote,
                                            token: t,
                                            isTradeable: !0,
                                            volume: this.buf2long(i.slice(16, 20)),
                                            lastQuantity: this.buf2long(i.slice(8, 12)),
                                            totalBuyQuantity: this.buf2long(i.slice(20, 24)),
                                            totalSellQuantity: this.buf2long(i.slice(24, 28)),
                                            lastPrice: this.buf2long(i.slice(4, 8)) / r,
                                            averagePrice: this.buf2long(i.slice(12, 16)) / r,
                                            openPrice: this.buf2long(i.slice(28, 32)) / r,
                                            highPrice: this.buf2long(i.slice(32, 36)) / r,
                                            lowPrice: this.buf2long(i.slice(36, 40)) / r,
                                            closePrice: this.buf2long(i.slice(40, 44)) / r
                                        }, e = Object.assign(e, this.calculateChange(e)), 164 === i.byteLength || 184 === i.byteLength) {
                                        let t = 44;
                                        184 === i.byteLength && (t = 64);
                                        let s = t + 120;
                                        if (e.mode = this.modeFull, e.depth = {
                                                buy: [],
                                                sell: []
                                            }, 184 === i.byteLength) {
                                            let t = this.buf2long(i.slice(44, 48));
                                            e.lastTradedTime = t && t > 0 ? this.dateToString(new Date(1e3 * t)) : null, e.oi = this.buf2long(i.slice(48, 52)), e.oiDayHigh = this.buf2long(i.slice(52, 56)), e.oiDayLow = this.buf2long(i.slice(56, 60))
                                        }
                                        let n = 0,
                                            o = i.slice(t, s);
                                        for (let i = 0; i < 10; i++) n = 12 * i, e.depth[i < 5 ? "buy" : "sell"].push({
                                            price: this.buf2long(o.slice(n + 4, n + 8)) / r,
                                            orders: this.buf2long(o.slice(n + 8, n + 10)),
                                            quantity: this.buf2long(o.slice(n, n + 4))
                                        })
                                    }
                                    s.push(e)
                                }
                        }
                    }
                    return s
                }
                splitPackets(e) {
                    let t = this.buf2long(e.slice(0, 2)),
                        s = 2,
                        i = [];
                    for (let o = 0; o < t; o++) {
                        var n = this.buf2long(e.slice(s, s + 2)),
                            r = e.slice(s + 2, s + 2 + n);
                        i.push(r), s += 2 + n
                    }
                    return i
                }
                processMessage(e) {
                    try {
                        var t = JSON.parse(e)
                    } catch (n) {
                        return
                    }
                    if (!t.hasOwnProperty("t") && !t.hasOwnProperty("type")) return;
                    let s = t.t || t.type,
                        i = t.p || t.data;
                    switch (s) {
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
                                let e = JSON.parse(i);
                                this.eventClearCache.trigger(e)
                            } catch (n) {} else this.eventClearCache.trigger();
                            break;
                        case this.mGetQuote:
                            this.processQuoteMessage(t.id, i);
                            break
                    }
                }
                processQuoteMessage(e, t) {
                    let s = this.quoteMap[e];
                    s && (s.resolve(t), delete this.quoteMap[e])
                }
                buf2long(e) {
                    let t = new Uint8Array(e),
                        s = 0,
                        i = t.length;
                    for (let n = 0, r = i - 1; n < i; n++, r--) s += t[r] << 8 * n;
                    return s
                }
            }
            var c = a,
                h = s("39e3");
            s.d(t, "a", (function() {
                return l
            }));
            class l {}
            l.namespace = h["b"], l.Provider = c, l.install = function(e, t) {
                t.store && t.store.registerModule(this.namespace, h["a"])
            }
        },
        "39e3": function(e, t, s) {
            "use strict";
            s.d(t, "b", (function() {
                return o
            }));
            var i = s("5fb0"),
                n = s("b202");
            let r = !0;
            const o = "ticker";

            function a(e, t) {
                let s = e.change || 0,
                    i = e.absoluteChange || 0;
                t.closePrice && (i = e.lastPrice - t.closePrice, s = 100 * i / t.closePrice);
                let n = 0;
                return s && 0 !== s && (n = s > 0 ? 1 : -1), Object.assign({}, t, e, {
                    change: s,
                    absoluteChange: i,
                    tickChange: n
                })
            }
            const c = {
                    ticks: {},
                    tickerConnectionStatus: i["a"].initial
                },
                h = {
                    ticks: e => e.ticks,
                    tickerConnectionStatus: e => e.tickerConnectionStatus
                },
                l = {
                    setTick(e, t) {
                        let s = t,
                            i = e.ticks[t.token] || {};
                        s = a(t, i), e.ticks = Object.assign({}, e.ticks, {
                            [t.token]: s
                        }), Object(n["d"])(o, "ticks", e.ticks)
                    },
                    setTicks(e, t) {
                        let s = {};
                        for (let i of t) {
                            let t = i;
                            if (e.ticks[i.token]) {
                                let s = e.ticks[i.token] || {};
                                t = a(i, s)
                            }
                            s[i.token] = t
                        }
                        e.ticks = Object.assign({}, e.ticks, s), Object(n["d"])(o, "ticks", e.ticks)
                    },
                    setExtendedTicks(e, t) {
                        let s = {};
                        for (let i of t) {
                            let t = i;
                            if (e.ticks[i.token]) {
                                let s = e.ticks[i.token] || {};
                                t = Object.assign({}, s, i)
                            }
                            s[i.token] = t
                        }
                        e.ticks = Object.assign({}, e.ticks, s), Object(n["d"])(o, "ticks", e.ticks)
                    },
                    dumpTicks(e, t) {
                        e.ticks = t, Object(n["d"])(o, "ticks", e.ticks)
                    },
                    setTickerConnectionStatus(e, t) {
                        e.tickerConnectionStatus = t
                    }
                },
                u = {};
            t["a"] = {
                state: c,
                getters: h,
                mutations: l,
                actions: u,
                namespaced: r
            }
        },
        "4a7c": function(e, t, s) {
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
                function s(e) {
                    var t, s, n, r, o, a, c = Object.create(null);
                    if (this[h] = c, e)
                        if ("string" === typeof e)
                            for ("?" === e.charAt(0) && (e = e.slice(1)), r = e.split("&"), o = 0, a = r.length; o < a; o++) n = r[o], t = n.indexOf("="), -1 < t ? l(c, u(n.slice(0, t)), u(n.slice(t + 1))) : n.length && l(c, u(n), "");
                        else if (i(e))
                        for (o = 0, a = e.length; o < a; o++) n = e[o], l(c, n[0], n[1]);
                    else
                        for (s in e) l(c, s, e[s])
                }
                var i = Array.isArray,
                    n = s.prototype,
                    r = /[!'\(\)~]|%20|%00/g,
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
                    h = "__URLSearchParams__:" + Math.random();

                function l(e, t, s) {
                    t in e ? e[t].push("" + s) : e[t] = i(s) ? s : ["" + s]
                }

                function u(e) {
                    return decodeURIComponent(e.replace(o, " "))
                }

                function g(e) {
                    return encodeURIComponent(e).replace(r, c)
                }
                n.append = function(e, t) {
                        l(this[h], e, t)
                    }, n.delete = function(e) {
                        delete this[h][e]
                    }, n.get = function(e) {
                        var t = this[h];
                        return e in t ? t[e][0] : null
                    }, n.getAll = function(e) {
                        var t = this[h];
                        return e in t ? t[e].slice(0) : []
                    }, n.has = function(e) {
                        return e in this[h]
                    }, n.set = function(e, t) {
                        this[h][e] = ["" + t]
                    }, n.forEach = function(e, t) {
                        var s = this[h];
                        Object.getOwnPropertyNames(s).forEach((function(i) {
                            s[i].forEach((function(s) {
                                e.call(t, s, i, this)
                            }), this)
                        }), this)
                    }, n.toJSON = function() {
                        return {}
                    }, n.toString = function() {
                        var e, t, s, i, n = this[h],
                            r = [];
                        for (t in n)
                            for (s = g(t), e = 0, i = n[t]; e < i.length; e++) r.push(s + "=" + g(i[e]));
                        return r.join("&")
                    }, s = e.exports = t.URLSearchParams || s,
                    function(e) {
                        var t = function() {
                            try {
                                return !!Symbol.iterator
                            } catch (e) {
                                return !1
                            }
                        }();
                        "forEach" in e || (e.forEach = function(e, t) {
                            var s = Object.create(null);
                            this.toString().replace(/=[\s\S]*?(?:&|$)/g, "=").split("=").forEach((function(i) {
                                i.length && !(i in s) && (s[i] = this.getAll(i)).forEach((function(s) {
                                    e.call(t, s, i, this)
                                }), this)
                            }), this)
                        }), "keys" in e || (e.keys = function() {
                            var e = [];
                            this.forEach((function(t, s) {
                                e.push(s)
                            }));
                            var s = {
                                next: function() {
                                    var t = e.shift();
                                    return {
                                        done: void 0 === t,
                                        value: t
                                    }
                                }
                            };
                            return t && (s[Symbol.iterator] = function() {
                                return s
                            }), s
                        }), "values" in e || (e.values = function() {
                            var e = [];
                            this.forEach((function(t) {
                                e.push(t)
                            }));
                            var s = {
                                next: function() {
                                    var t = e.shift();
                                    return {
                                        done: void 0 === t,
                                        value: t
                                    }
                                }
                            };
                            return t && (s[Symbol.iterator] = function() {
                                return s
                            }), s
                        }), "entries" in e || (e.entries = function() {
                            var e = [];
                            this.forEach((function(t, s) {
                                e.push([s, t])
                            }));
                            var s = {
                                next: function() {
                                    var t = e.shift();
                                    return {
                                        done: void 0 === t,
                                        value: t
                                    }
                                }
                            };
                            return t && (s[Symbol.iterator] = function() {
                                return s
                            }), s
                        }), t && !(Symbol.iterator in e) && (e[Symbol.iterator] = e.entries), "sort" in e || (e.sort = function() {
                            var e, t, s, i = this.entries(),
                                n = i.next(),
                                r = n.done,
                                o = [],
                                a = Object.create(null);
                            while (!r) s = n.value, t = s[0], o.push(t), t in a || (a[t] = []), a[t].push(s[1]), n = i.next(), r = n.done;
                            for (o.sort(), e = 0; e < o.length; e++) this.delete(o[e]);
                            for (e = 0; e < o.length; e++) t = o[e], this.append(t, a[t].shift())
                        })
                    }(s.prototype)
            }).call(this, s("c8ba"))
        },
        ba6a: function(e, t, s) {
            "use strict";
            var i = s("bc3a"),
                n = s.n(i),
                r = s("b383"),
                o = s.n(r),
                a = s("c0d6"),
                c = s("b202"),
                h = s("0644"),
                l = s.n(h),
                u = s("df03");
            let g = {};
            const d = n.a.create({
                timeout: 8e3,
                responseType: "json",
                paramsSerializer: function(e) {
                    return o.a.stringify(e)
                }
            });
            let f = "2.9.11";
            const T = a["a"].getters[u["b"] + "/getETag"],
                b = (e, t, s) => a["a"].commit(u["b"] + "/setETag", {
                    key: e,
                    ETag: t,
                    response: s
                });
            d.interceptors.request.use(e => {
                if (e.headers["X-Kite-Version"] = f, "get" === e.method) {
                    let t = T(e.url);
                    t && t.ETag && (e.headers["X-If-None-Match"] = t.ETag);
                    let s = e.url + o.a.stringify(e.params);
                    g[s] && g[s].cancel("cancel-duplicate-requests");
                    let i = n.a.CancelToken,
                        r = i.source();
                    e.cancelToken = r.token, g[s] = r
                }
                if (e.data && e.data.user_id && (e.headers["X-Kite-Userid"] = e.data.user_id), e.params && e.params.user_id && (e.headers["X-Kite-Userid"] = e.params.user_id), "post" !== e.method && "put" !== e.method || e.headers["Content-Type"] || (e.headers["Content-Type"] = "application/x-www-form-urlencoded", e.data = o.a.stringify(e.data)), "/oms/" === e.url.substring(0, 5)) {
                    let t = Object(c["b"])(null, "enctoken", c["c"]);
                    t && (e.headers["Authorization"] = "enctoken " + Object(c["b"])(null, "enctoken", c["c"]))
                } else {
                    let t = Object(c["b"])(null, "public_token", c["c"]);
                    t && (e.headers["X-CSRFTOKEN"] = Object(c["b"])(null, "public_token", c["c"]))
                }
                return e
            }), d.interceptors.response.use(e => {
                if (e.headers["x-etag"]) b(e.config.url, e.headers["x-etag"], e.data);
                else if ("304" === e.headers["x-status"]) {
                    let t = T(e.config.url);
                    if (t && t.response) return Promise.resolve({
                        data: l()(t.response),
                        status: 200,
                        statusText: "OK",
                        headers: e.headers,
                        config: e.config
                    })
                }
                return e
            }, e => {
                if (e && e.response && 304 === e.response.status) {
                    let t = e.response,
                        s = T(t.config.url);
                    if (s && s.response) return t.data = s.response, Promise.resolve(t)
                }
                if (e && e.response && 403 === e.response.status && e.response.data && "TokenException" === e.response.data.error_type && (Object(c["d"])("", "user_id", "", c["c"]), Object(c["d"])("", "public_token", "", c["c"]), Object(c["d"])("", "enctoken", "", c["c"]), window.location = "/logout?invalidtoken=true"), e) return Promise.reject(e)
            }), t["a"] = d
        },
        d9d2: function(e, t, s) {
            "use strict";
            s.d(t, "a", (function() {
                return i
            }));
            const i = "orders",
                n = ["DAY", "IOC"],
                r = ["MIS", "NRML", "CNC"],
                o = ["regular", "bo", "co", "amo", "iceberg"],
                a = ["MARKET", "LIMIT", "SL", "SL-M"],
                c = ["BUY", "SELL"],
                h = {
                    DAY: "DAY",
                    IOC: "IOC",
                    TTL: "TTL"
                },
                l = {
                    MIS: "MIS",
                    NRML: "NRML",
                    CNC: "CNC"
                },
                u = {
                    REGULAR: "regular",
                    BO: "bo",
                    CO: "co",
                    AMO: "amo",
                    ICEBERG: "iceberg"
                },
                g = {
                    MARKET: "MARKET",
                    LIMIT: "LIMIT",
                    SL: "SL",
                    SLM: "SL-M"
                },
                d = {
                    BUY: "BUY",
                    SELL: "SELL"
                },
                f = {
                    ORDER_PLACE: i + ".order.place",
                    ORDER_PLACE_SUCCESS: i + ".order.place.success",
                    ORDER_PLACE_ERROR: i + ".order.place.error",
                    ORDER_MODIFY: i + ".order.modify",
                    ORDER_MODIFY_SUCCESS: i + ".order.modify.success",
                    ORDER_MODIFY_ERROR: i + ".order.modify.error",
                    ORDER_CANCEL: i + ".order.cancel",
                    ORDER_CANCEL_SUCCESS: i + ".order.cancel.success",
                    ORDER_CANCEL_ERROR: i + ".order.cancel.error",
                    PRODUCT_MODIFY: i + ".product.modify",
                    ORDER_INFO: i + ".order.ifo",
                    GTT_PLACE: i + ".gtt.place",
                    GTT_PLACE_SUCCESS: i + ".gtt.place.success",
                    GTT_PLACE_ERROR: i + ".gtt.place.error",
                    GTT_MODIFY: i + ".gtt.modify",
                    GTT_MODIFY_SUCCESS: i + ".gtt.modify.success",
                    GTT_MODIFY_ERROR: i + ".gtt.modify.error",
                    GTT_DELETE: i + ".gtt.delete",
                    GTT_DELETE_SUCCESS: i + ".gtt.delete.success",
                    GTT_DELETE_ERROR: i + ".gtt.delete.error",
                    GTT_INFO: i + ".gtt.info",
                    SPREAD_ORDER_OPEN: i + ".spread.open",
                    SPREAD_ORDER_CLOSE: i + ".spread.close",
                    SPREAD_ORDER_FINISH: i + ".spread.finish",
                    SPREAD_ORDER_MINIMIZE: i + ".spread.minimize"
                },
                T = {
                    INDICES: "INDICES",
                    NSE: "NSE",
                    BSE: "BSE",
                    NFO: "NFO",
                    "NFO-FUT": "NFO",
                    "NFO-OPT": "NFO",
                    CDS: "CDS",
                    "CDS-FUT": "CDS",
                    "CDS-OPT": "CDS",
                    BFO: "BFO",
                    "BFO-FUT": "BFO",
                    "BFO-OPT": "BFO",
                    MCX: "MCX",
                    MCXSX: "MCXSX"
                },
                b = {
                    typeSingle: "single",
                    typeTwoLeg: "two-leg",
                    statusExpired: "EXPIRED",
                    statusCancelled: "CANCELLED",
                    statusActive: "ACTIVE",
                    statusTriggered: "TRIGGERED",
                    statusDisabled: "DISABLED"
                };
            t["b"] = {
                NAMESPACE: i,
                VALIDITY: h,
                VALIDITIES: n,
                PRODUCT: l,
                PRODUCTS: r,
                VARIETY: u,
                VARIETIES: o,
                ORDER_TYPE: g,
                ORDER_TYPES: a,
                TRANSACTION_TYPE: d,
                TRANSACTION_TYPES: c,
                EVENTS: f,
                EXCHANGES: T,
                GTT: b
            }
        }
    }
]);
