(window["webpackJsonp"] = window["webpackJsonp"] || []).push([
    ["chartiq~main~main-chartiq~outer~tradingview"], {
        "11df": function(_, e, t) {
            "use strict";
            var i = window.URL,
                s = window.Blob,
                n = window.Worker;

            function r(_) {
                var e = {};
                _.onmessage = function(t) {
                    switch (t.data.type) {
                        case "ping":
                            _.postMessage({
                                type: "pong"
                            });
                            break;
                        case "set_timeout":
                            e[t.data.id] = setTimeout((function() {
                                _.postMessage({
                                    type: "tick",
                                    id: t.data.id
                                })
                            }), t.data.delay);
                            break;
                        case "clear_timeout":
                            clearTimeout(e[t.data.id]);
                            break;
                        case "set_interval":
                            e[t.data.id] = setInterval((function() {
                                _.postMessage({
                                    type: "tick",
                                    id: t.data.id
                                })
                            }), t.data.interval);
                            break;
                        case "clear_interval":
                            clearInterval(e[t.data.id]);
                            break
                    }
                }
            }

            function a(_) {
                var e = this;
                this.__ww__ = _, this.__timer_handlers__ = {}, this.__ww__.onmessage = function(_) {
                    "tick" === _.data.type && e.__timer_handlers__[_.data.id]()
                }
            }

            function o() {
                this.__timers__ = {}
            }
            a.prototype.setTimeout = function(_, e, t) {
                this.__timer_handlers__[_] = e, this.__ww__.postMessage({
                    type: "set_timeout",
                    delay: t,
                    id: _
                })
            }, a.prototype.clearTimeout = function(_) {
                this.__ww__.postMessage({
                    type: "clear_timeout",
                    id: _
                }), delete this.__timer_handlers__[_]
            }, a.prototype.setInterval = function(_, e, t) {
                this.__timer_handlers__[_] = e, this.__ww__.postMessage({
                    type: "set_interval",
                    interval: t,
                    id: _
                })
            }, a.prototype.clearInterval = function(_) {
                this.__ww__.postMessage({
                    type: "clear_interval",
                    id: _
                }), delete this.__timer_handlers__[_]
            }, o.prototype.setTimeout = function(_, e, t) {
                this.__timers__[_] = setTimeout(e, t)
            }, o.prototype.clearTimeout = function(_) {
                clearInterval(this.__timers__[_])
            }, o.prototype.setInterval = function(_, e, t) {
                this.__timers__[_] = setInterval(e, t)
            }, o.prototype.clearInterval = function(_) {
                clearInterval(this.__timers__[_])
            };
            var c = null,
                h = [];

            function l(_) {
                if (c) _(c);
                else if (h.push(_), 1 === h.length) {
                    var e, t, l;
                    try {
                        t = i.createObjectURL(new s(["(" + r.toString() + ")(this)"], {
                            type: "text/javascript"
                        })), l = new n(t)
                    } catch (u) {
                        return d(), c = new o, void h.forEach((function(_) {
                            _(c)
                        }))
                    }
                    l.onmessage = function(_) {
                        "pong" === _.data.type && (clearInterval(e), c = new a(l), h.forEach((function(_) {
                            _(c)
                        })))
                    }, e = setTimeout((function() {
                        d(), c = new o, h.forEach((function(_) {
                            _(c)
                        }))
                    }), 300), l.postMessage({
                        type: "ping"
                    })
                }

                function d() {
                    t && i.revokeObjectURL(t), l && l.terminate(), e && clearTimeout(e)
                }
            }
            var d = 1;
            e.setTimeout = function(_, e) {
                var t = d++;
                return l((function(i) {
                    i.setTimeout(t, _, e)
                })), t
            }, e.clearTimeout = function(_) {
                l((function(e) {
                    e.clearTimeout(_)
                }))
            }, e.setInterval = function(_, e) {
                var t = d++;
                return l((function(i) {
                    i.setInterval(t, _, e)
                })), t
            }, e.clearInterval = function(_) {
                l((function(e) {
                    e.clearInterval(_)
                }))
            }
        },
        "1a5a": function(_, e, t) {
            "use strict";
            var i = t("d68e");

            function s(_) {
                var e = this;
                this.__filters_in__ = [], this.__filters_out__ = [], this.__subscriptions__ = [], this.__node_id__ = Math.floor(1e10 * Math.random()) + 1, this.__last_message_cnt__ = 0, this.__ignore_list__ = {}, this.__router__ = _.router, this.__router__.onmessage((function(_, t) {
                    e.__onmessage__(_, t)
                })), this.__lock_handlers__ = {}, this.filterOut((function(_, t, i) {
                    if ("!sys.lock.request" === _) {
                        var s = t.data.fn,
                            n = t.data.id;
                        delete t.data.fn, e.__lock_handlers__[t.id] = function() {
                            s((function() {
                                e.emit("!sys.lock.release", {
                                    id: n
                                })
                            }))
                        }
                    }
                    i(_, t)
                })), this.filterIn((function(_, t, i) {
                    "!sys.lock.acquired" === _ && e.__lock_handlers__[t.data.request_id] && (e.__lock_handlers__[t.data.request_id](), delete e.__lock_handlers__[t.data.request_id]), i(_, t)
                }))
            }
            s.prototype.emit = function(_, e, t) {
                var s = this,
                    n = {
                        id: this.__node_id__ + "_" + this.__last_message_cnt__++,
                        node_id: this.__node_id__,
                        data: e
                    };
                t || (this.__ignore_list__[n.id] = !0), i.asyncEach(this.__filters_out__, _, n, (function(_, e) {
                    s.__router__.broadcast(_, e)
                }))
            }, s.prototype.on = function(_, e) {
                return this.__subscriptions__.push({
                    channel: _,
                    handler: e
                }), this.emit("!sys.channels.add", {
                    channel: _
                }), this
            }, s.prototype.off = function(_, e) {
                var t = this;
                this.__subscriptions__ = this.__subscriptions__.reduce((function(i, s) {
                    return s.channel !== _ || e && e !== s.handler ? (i.push(s), i) : (t.emit("!sys.channels.remove", {
                        channel: _
                    }), i)
                }), [])
            }, s.prototype.lock = function(_, e, t) {
                t || (t = e, e = 5e3), this.emit("!sys.lock.request", {
                    id: _,
                    timeout: e,
                    fn: t
                })
            }, s.prototype.filterIn = function(_) {
                return this.__filters_in__.push(_), this
            }, s.prototype.filterOut = function(_) {
                return this.__filters_out__.push(_), this
            }, s.prototype.__onmessage__ = function(_, e) {
                var t = this;
                i.asyncEach(this.__filters_in__, _, e, (function(_, e) {
                    t.__ignore_list__[e.id] || t.__subscriptions__.forEach((function(t) {
                        t.channel === _ && t.handler(e.data, _)
                    }))
                }))
            }, _.exports = s
        },
        2456: function(_, e, t) {
            "use strict";
            var i, s = {},
                n = function() {
                    try {
                        if (i = window.localStorage, !i) return !1;
                        i.setItem("live_local_storage_is_writable_test", ""), i.removeItem("live_local_storage_is_writable_test")
                    } catch (_) {
                        return !1
                    }
                    return !(document.documentMode && document.documentMode < 9) && !navigator.userAgent.match("CriOS")
                }();

            function r() {}
            Object.defineProperty(r.prototype, "length", {
                get: function() {
                    return n ? i.length : Object.keys(s).length
                }
            }), r.prototype.getItem = function(_) {
                return n ? i.getItem(_) : s.hasOwnProperty(_) ? s[_] : null
            }, r.prototype.setItem = function(_, e) {
                n ? i.setItem(_, e) : s[_] = e
            }, r.prototype.removeItem = function(_) {
                n ? i.removeItem(_) : s[_] = null
            }, r.prototype.key = function(_) {
                return n ? i.key(_) : Object.keys(s)[_]
            }, _.exports = r
        },
        "29db": function(_, e, t) {
            "use strict";
            var i = t("11df"),
                s = t("2456"),
                n = t("d68e"),
                r = 4e3,
                a = r / 4;

            function o(_) {
                var e = this;
                _ = _ || {}, this.__namespace__ = _.namespace || "tabex_default_", this.__node_id__ = Math.floor(1e10 * Math.random()) + 1, this.__last_message_cnt__ = 0, this.__handlers__ = [], this.__router_channels__ = {}, this.__router_id_prefix__ = this.__namespace__ + "router_", this.__router_channels_prefix__ = this.__namespace__ + "subscribed_", this.__lock_prefix__ = this.__namespace__ + "lock_", this.__storage_events_filter__ = [];
                for (var t = 0; t < 100; t++) this.__storage_events_filter__.push("");
                this.__ls__ = new s, this.__master_id__ = null, n.addEvent(window, "storage", (function(_) {
                    null !== _.key ? "onstoragecommit" in document ? setTimeout((function() {
                        e.__on_changed__(_)
                    }), 1) : e.__on_changed__(_) : e.__check_master__()
                })), this.__destroyed__ = !1, n.addEvent(window, "beforeunload", (function() {
                    e.__destroy__()
                })), n.addEvent(window, "unload", (function() {
                    e.__destroy__()
                })), this.__check_master__(), this.__timer_cm__ = i.setInterval((function() {
                    e.__check_master__()
                }), a), this.__timer_lc__ = i.setInterval((function() {
                    e.__locks_cleanup__()
                }), 1e3)
            }
            o.prototype.broadcast = function(_, e) {
                if ("!sys.channels.add" === _) return this.__router_channels__[e.data.channel] = this.__router_channels__[e.data.channel] || 0, this.__router_channels__[e.data.channel]++, void this.__update_channels_list__();
                if ("!sys.channels.remove" === _) return this.__router_channels__[e.data.channel] = this.__router_channels__[e.data.channel] || 0, this.__router_channels__[e.data.channel]--, void this.__update_channels_list__();
                if ("!sys.lock.request" !== _)
                    if ("!sys.lock.release" !== _) {
                        var t = JSON.stringify({
                            channel: _,
                            message: e,
                            random: Math.floor(1e10 * Math.random())
                        });
                        this.__storage_events_filter__.shift(), this.__storage_events_filter__.push(this.__namespace__ + "broadcast_" + t), this.__ls__.setItem(this.__namespace__ + "broadcast", t), this.__handlers__.forEach((function(t) {
                            t(_, e)
                        }))
                    } else this.__ls__.removeItem(this.__lock_prefix__ + e.data.id);
                else this.__lock__(e.data.id, e.id, e.data.timeout)
            }, o.prototype.onmessage = function(_) {
                var e = this;
                this.__handlers__.push(_), setTimeout((function() {
                    _("!sys.master", {
                        data: {
                            node_id: e.__node_id__,
                            master_id: e.__master_id__
                        },
                        node_id: e.__node_id__,
                        id: e.__node_id__ + "_" + e.__last_message_cnt__++
                    }), e.__on_channels_list_changed__()
                }), 0)
            }, o.prototype.__lock__ = function(_, e, t) {
                var i = this,
                    s = this.__lock_prefix__ + _,
                    n = this.__ls__.getItem(s);
                if (n) try {
                    n = JSON.parse(n)
                } catch (r) {
                    n = null
                }
                n && n.expire > Date.now() || (this.__ls__.setItem(s, JSON.stringify({
                    expire: t + Date.now(),
                    requestId: e
                })), setTimeout((function() {
                    if (n = i.__ls__.getItem(s), n) try {
                        n = JSON.parse(n)
                    } catch (r) {
                        n = null
                    }
                    n && n.requestId === e && i.__handlers__.forEach((function(_) {
                        _("!sys.lock.acquired", {
                            data: {
                                request_id: e
                            },
                            node_id: i.__node_id__,
                            id: i.__node_id__ + "_" + i.__last_message_cnt__++
                        })
                    }))
                }), 30))
            }, o.prototype.__locks_cleanup__ = function() {
                for (var _, e, t = 0; t < this.__ls__.length; t++)
                    if (_ = this.__ls__.key(t), 0 === _.indexOf(this.__lock_prefix__)) {
                        e = this.__ls__.getItem(_);
                        try {
                            e = JSON.parse(e)
                        } catch (i) {
                            e = null
                        }(!e || e.expire < Date.now()) && this.__ls__.removeItem(_)
                    }
            }, o.prototype.__on_master_changed__ = function(_) {
                var e = this;
                _ ? (this.__master_id__ = +_, this.__handlers__.forEach((function(_) {
                    _("!sys.master", {
                        data: {
                            node_id: e.__node_id__,
                            master_id: e.__master_id__
                        },
                        node_id: e.__node_id__,
                        id: e.__node_id__ + "_" + e.__last_message_cnt__++
                    })
                })), e.__update_channels_list__(!0), e.__on_channels_list_changed__()) : this.__get_alive_router_ids__().sort()[0] === this.__node_id__ && (this.__storage_events_filter__.pop(), this.__storage_events_filter__.push(this.__namespace__ + "master_" + this.__node_id__), this.__ls__.setItem(this.__namespace__ + "master", this.__node_id__), this.__on_master_changed__(this.__node_id__))
            }, o.prototype.__on_changed__ = function(_) {
                if (-1 === this.__storage_events_filter__.indexOf(_.key + "_" + _.newValue) && (_.key === this.__namespace__ + "master" && this.__on_master_changed__(_.newValue), 0 === _.key.indexOf(this.__router_channels_prefix__) && this.__on_channels_list_changed__(), _.key === this.__namespace__ + "broadcast")) {
                    var e = JSON.parse(_.newValue);
                    this.__handlers__.forEach((function(_) {
                        _(e.channel, e.message)
                    }))
                }
            }, o.prototype.__destroy__ = function() {
                this.__destroyed__ || (this.__destroyed__ = !0, i.clearInterval(this.__timer_cm__), i.clearInterval(this.__timer_lc__), this.__ls__.removeItem(this.__router_id_prefix__ + this.__node_id__), this.__ls__.removeItem(this.__router_channels_prefix__ + this.__node_id__), this.__master_id__ === this.__node_id__ && this.__ls__.removeItem(this.__namespace__ + "master"))
            }, o.prototype.__get_alive_router_ids__ = function() {
                for (var _, e, t = Date.now() - r, i = [], s = 0; s < this.__ls__.length; s++) e = this.__ls__.key(s), 0 === e.indexOf(this.__router_id_prefix__) && (_ = +e.substr(this.__router_id_prefix__.length), this.__ls__.getItem(e) < t ? (this.__ls__.removeItem(e), this.__ls__.removeItem(this.__router_channels_prefix__ + _)) : i.push(_));
                return i
            }, o.prototype.__update_channels_list__ = function(_) {
                var e = this,
                    t = [];
                Object.keys(this.__router_channels__).forEach((function(_) {
                    e.__router_channels__[_] > 0 && t.push(_)
                }));
                var i = JSON.stringify(t.sort());
                this.__ls__.getItem(this.__router_channels_prefix__ + this.__node_id__) !== i && (this.__storage_events_filter__.pop(), this.__storage_events_filter__.push(this.__router_channels_prefix__ + this.__node_id__ + "_" + i), this.__ls__.setItem(this.__router_channels_prefix__ + this.__node_id__, i), _ || this.__on_channels_list_changed__())
            }, o.prototype.__on_channels_list_changed__ = function() {
                for (var _, e = this, t = [], i = 0; i < this.__ls__.length; i++) _ = this.__ls__.key(i), 0 === _.indexOf(this.__router_channels_prefix__) && (t = t.concat(JSON.parse(this.__ls__.getItem(_))));
                t = t.reduce((function(_, e) {
                    return -1 === _.indexOf(e) && _.push(e), _
                }), []), this.__handlers__.forEach((function(_) {
                    _("!sys.channels.refresh", {
                        id: e.__node_id__ + "_" + e.__last_message_cnt__++,
                        node_id: e.__node_id__,
                        data: {
                            channels: t
                        }
                    })
                }))
            }, o.prototype.__check_master__ = function() {
                this.__ls__.setItem(this.__router_id_prefix__ + this.__node_id__, Date.now()), this.__master_id__ = +this.__ls__.getItem(this.__namespace__ + "master"), -1 === this.__get_alive_router_ids__().indexOf(this.__master_id__) && (this.__storage_events_filter__.pop(), this.__storage_events_filter__.push(this.__namespace__ + "master_" + this.__node_id__), this.__ls__.setItem(this.__namespace__ + "master", this.__node_id__), this.__on_master_changed__(this.__node_id__))
            }, _.exports = o
        },
        "506b": function(_, e, t) {
            "use strict";
            var i = t("d68e");

            function s(_) {
                var e = this;
                this.__namespace__ = _.namespace || "tabex_default_", this.__handlers__ = [], this.__iframe_url__ = _.iframe, this.__iframe_done__ = !1, this.__pending__ = [], this.__iframe__ = document.createElement("iframe"), this.__iframe__.style.left = "-1000px", this.__iframe__.style.position = "absolute", this.__iframe__.onload = function() {
                    e.__iframe__.contentWindow.postMessage(JSON.stringify({
                        origin: window.location.origin || window.location.protocol + "//" + window.location.host,
                        namespace: e.__namespace__
                    }), e.__iframe_url__), e.__iframe_done__ = !0, e.__pending__.forEach((function(_) {
                        e.__iframe__.contentWindow.postMessage(JSON.stringify(_), e.__iframe_url__)
                    })), e.__pending__ = null
                }, i.addEvent(window, "message", (function(_) {
                    if (0 === e.__iframe_url__.indexOf(_.origin)) {
                        var t;
                        try {
                            t = JSON.parse(_.data)
                        } catch (i) {
                            return
                        }
                        t.namespace === e.__namespace__ && e.__handlers__.forEach((function(_) {
                            _(t.channel, t.message)
                        }))
                    }
                })), this.__iframe__.src = this.__iframe_url__;
                var t = function() {
                    document.body.appendChild(e.__iframe__)
                };
                "loading" === document.readyState ? i.addEvent(document, "DOMContentLoaded", t) : t()
            }

            function n(_) {
                var e, t = this;
                for (this.__namespace__ = _.namespace || "tabex_default_", this.__origin_first_check__ = _.origin || window.location.origin || window.location.protocol + "//" + window.location.host, Array.isArray(this.__origin_first_check__) || (this.__origin_first_check__ = [this.__origin_first_check__]), e = 0; e < this.__origin_first_check__.length; e++) this.__origin_first_check__[e] = this.__origin_first_check__[e].replace(/[-\/\\^$+?.()|[\]{}]/g, "\\$&"), this.__origin_first_check__[e] = this.__origin_first_check__[e].replace(/[*]/g, ".+?"), this.__origin_first_check__[e] = new RegExp(this.__origin_first_check__[e]);
                this.__origin__ = null, this.__router__ = _.router, i.addEvent(window, "message", (function(_) {
                    var i, s = !1;
                    if (!t.__origin__ || t.__origin__ !== _.origin) {
                        for (e = 0; e < t.__origin_first_check__.length; e++)
                            if (t.__origin_first_check__[e].test(_.origin)) {
                                s = !0;
                                break
                            }
                        if (!s) return
                    }
                    try {
                        i = JSON.parse(_.data)
                    } catch (n) {
                        return
                    }
                    if (i.namespace === t.__namespace__) return !t.__origin__ && i.origin ? (t.__origin__ = i.origin, void t.__router__.onmessage((function(_, e) {
                        window.parent.postMessage(JSON.stringify({
                            channel: _,
                            message: e,
                            namespace: t.__namespace__
                        }), t.__origin__)
                    }))) : void t.__router__.broadcast(i.channel, i.message)
                }))
            }
            s.prototype.broadcast = function(_, e) {
                this.__iframe_done__ ? this.__iframe__.contentWindow.postMessage(JSON.stringify({
                    channel: _,
                    message: e,
                    namespace: this.__namespace__
                }), this.__iframe_url__) : this.__pending__.push({
                    channel: _,
                    message: e,
                    namespace: this.__namespace__
                })
            }, s.prototype.onmessage = function(_) {
                this.__handlers__.push(_)
            }, e.TunnelClient = s, e.TunnelRouter = n
        },
        b23e: function(_, e, t) {
            "use strict";
            _.exports = t("f3b7")
        },
        d68e: function(_, e, t) {
            "use strict";
            e.asyncEach = function(_) {
                _ = _.slice(0);
                var e = arguments[arguments.length - 1],
                    t = Array.prototype.slice.call(arguments, 1);

                function i() {
                    if (0 !== _.length) {
                        var t = _.shift();
                        t.apply(this, Array.prototype.slice.call(arguments, 0).concat(i))
                    } else e.apply(this, arguments)
                }
                t.pop(), i.apply(this, t)
            }, e.addEvent = function(_, e, t) {
                document.addEventListener ? _.addEventListener(e, t) : _.attachEvent("on" + e, t)
            }
        },
        f3b7: function(_, e, t) {
            "use strict";
            var i = t("29db"),
                s = t("1a5a"),
                n = t("506b"),
                r = {},
                a = {
                    _: {}
                };
            a._.Router = i, a._.Client = s, a._.Tunnel = n, a.client = function(_) {
                _ = _ || {};
                var e, t = _.namespace || "tabex_default_";
                return _.iframe ? e = new n.TunnelClient(_) : (r[t] || (r[t] = new i({
                    namespace: t
                })), e = r[t]), new s({
                    router: e
                })
            }, a.router = function(_) {
                _ = _ || {};
                var e = _.namespace || "tabex_default_";
                return r[e] || (r[e] = new i({
                    namespace: e
                })), new n.TunnelRouter({
                    router: r[e],
                    namespace: e,
                    origin: _.origin
                }), r[e]
            }, _.exports = a
        },
        f8fb: function(_, e, t) {
            "use strict";
            var i = function() {
                    var _ = this,
                        e = _.$createElement,
                        t = _._self._c || e;
                    return _.title && _.error && _.error.message ? t("div", {
                        staticClass: "error-state"
                    }, [t("div", {
                        staticClass: "error-state-wrapper"
                    }, [_._m(0), _._v(" "), t("div", {
                        staticClass: "error-info"
                    }, [t("h3", {
                        staticClass: "title"
                    }, [_._v(_._s(_.title))]), _._v(" "), _.error && _.error.message ? t("div", {
                        staticClass: "message"
                    }, [_._v("\n\t\t\t\t" + _._s(_.error.message) + " "), t("span", {
                        staticClass: "error-type"
                    }, [_._v("(" + _._s(_.error.error_type) + ")")])]) : _._e()]), _._v(" "), t("hr")])]) : _._e()
                },
                s = [function() {
                    var _ = this,
                        e = _.$createElement,
                        t = _._self._c || e;
                    return t("div", {
                        staticClass: "error-icon"
                    }, [t("span", {
                        staticClass: "icon icon-alert-triangle"
                    })])
                }],
                n = {
                    name: "error-state",
                    props: {
                        error: Object,
                        title: String
                    }
                },
                r = n,
                a = t("2877"),
                o = Object(a["a"])(r, i, s, !1, null, null, null);
            e["a"] = o.exports
        }
    }
]);