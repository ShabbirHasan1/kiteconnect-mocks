// To parse this data:
//
//   import { Convert, Quote } from "./file";
//
//   const quote = Convert.toQuote(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

export interface Quote {
    $ref:        string;
    $schema:     string;
    definitions: Definitions;
}

export interface Definitions {
    Buy:     Buy;
    Data:    Data;
    Depth:   Depth;
    NseInfy: NseInfyClass;
    Ohlc:    Ohlc;
    Quote:   QuoteClass;
}

export interface Buy {
    additionalProperties: boolean;
    properties:           BuyProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface BuyProperties {
    orders:   Orders;
    price:    Orders;
    quantity: Orders;
}

export interface Orders {
    type: Type;
}

export enum Type {
    Integer = "integer",
    Number = "number",
    String = "string",
}

export interface Data {
    additionalProperties: boolean;
    properties:           DataProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface DataProperties {
    "NSE:INFY": NseInfy;
}

export interface NseInfy {
    $ref: string;
}

export interface Depth {
    additionalProperties: boolean;
    properties:           DepthProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface DepthProperties {
    buy:  BuyClass;
    sell: BuyClass;
}

export interface BuyClass {
    items: NseInfy;
    type:  string;
}

export interface NseInfyClass {
    additionalProperties: boolean;
    properties:           NseInfyProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface NseInfyProperties {
    average_price:       Orders;
    buy_quantity:        Orders;
    depth:               NseInfy;
    instrument_token:    Orders;
    last_price:          Orders;
    last_quantity:       Orders;
    last_trade_time:     LastTradeTime;
    lower_circuit_limit: Orders;
    net_change:          Orders;
    ohlc:                NseInfy;
    oi:                  Orders;
    oi_day_high:         Orders;
    oi_day_low:          Orders;
    sell_quantity:       Orders;
    timestamp:           LastTradeTime;
    upper_circuit_limit: Orders;
    volume:              Orders;
}

export interface LastTradeTime {
    format: string;
    type:   Type;
}

export interface Ohlc {
    additionalProperties: boolean;
    properties:           OhlcProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface OhlcProperties {
    close: Orders;
    high:  Orders;
    low:   Orders;
    open:  Orders;
}

export interface QuoteClass {
    additionalProperties: boolean;
    properties:           QuoteProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface QuoteProperties {
    data:   NseInfy;
    status: Orders;
}

// Converts JSON strings to/from your types
// and asserts the results of JSON.parse at runtime
export class Convert {
    public static toQuote(json: string): Quote {
        return cast(JSON.parse(json), r("Quote"));
    }

    public static quoteToJson(value: Quote): string {
        return JSON.stringify(uncast(value, r("Quote")), null, 2);
    }
}

function invalidValue(typ: any, val: any, key: any = ''): never {
    if (key) {
        throw Error(`Invalid value for key "${key}". Expected type ${JSON.stringify(typ)} but got ${JSON.stringify(val)}`);
    }
    throw Error(`Invalid value ${JSON.stringify(val)} for type ${JSON.stringify(typ)}`, );
}

function jsonToJSProps(typ: any): any {
    if (typ.jsonToJS === undefined) {
        const map: any = {};
        typ.props.forEach((p: any) => map[p.json] = { key: p.js, typ: p.typ });
        typ.jsonToJS = map;
    }
    return typ.jsonToJS;
}

function jsToJSONProps(typ: any): any {
    if (typ.jsToJSON === undefined) {
        const map: any = {};
        typ.props.forEach((p: any) => map[p.js] = { key: p.json, typ: p.typ });
        typ.jsToJSON = map;
    }
    return typ.jsToJSON;
}

function transform(val: any, typ: any, getProps: any, key: any = ''): any {
    function transformPrimitive(typ: string, val: any): any {
        if (typeof typ === typeof val) return val;
        return invalidValue(typ, val, key);
    }

    function transformUnion(typs: any[], val: any): any {
        // val must validate against one typ in typs
        const l = typs.length;
        for (let i = 0; i < l; i++) {
            const typ = typs[i];
            try {
                return transform(val, typ, getProps);
            } catch (_) {}
        }
        return invalidValue(typs, val);
    }

    function transformEnum(cases: string[], val: any): any {
        if (cases.indexOf(val) !== -1) return val;
        return invalidValue(cases, val);
    }

    function transformArray(typ: any, val: any): any {
        // val must be an array with no invalid elements
        if (!Array.isArray(val)) return invalidValue("array", val);
        return val.map(el => transform(el, typ, getProps));
    }

    function transformDate(val: any): any {
        if (val === null) {
            return null;
        }
        const d = new Date(val);
        if (isNaN(d.valueOf())) {
            return invalidValue("Date", val);
        }
        return d;
    }

    function transformObject(props: { [k: string]: any }, additional: any, val: any): any {
        if (val === null || typeof val !== "object" || Array.isArray(val)) {
            return invalidValue("object", val);
        }
        const result: any = {};
        Object.getOwnPropertyNames(props).forEach(key => {
            const prop = props[key];
            const v = Object.prototype.hasOwnProperty.call(val, key) ? val[key] : undefined;
            result[prop.key] = transform(v, prop.typ, getProps, prop.key);
        });
        Object.getOwnPropertyNames(val).forEach(key => {
            if (!Object.prototype.hasOwnProperty.call(props, key)) {
                result[key] = transform(val[key], additional, getProps, key);
            }
        });
        return result;
    }

    if (typ === "any") return val;
    if (typ === null) {
        if (val === null) return val;
        return invalidValue(typ, val);
    }
    if (typ === false) return invalidValue(typ, val);
    while (typeof typ === "object" && typ.ref !== undefined) {
        typ = typeMap[typ.ref];
    }
    if (Array.isArray(typ)) return transformEnum(typ, val);
    if (typeof typ === "object") {
        return typ.hasOwnProperty("unionMembers") ? transformUnion(typ.unionMembers, val)
            : typ.hasOwnProperty("arrayItems")    ? transformArray(typ.arrayItems, val)
            : typ.hasOwnProperty("props")         ? transformObject(getProps(typ), typ.additional, val)
            : invalidValue(typ, val);
    }
    // Numbers can be parsed by Date but shouldn't be.
    if (typ === Date && typeof val !== "number") return transformDate(val);
    return transformPrimitive(typ, val);
}

function cast<T>(val: any, typ: any): T {
    return transform(val, typ, jsonToJSProps);
}

function uncast<T>(val: T, typ: any): any {
    return transform(val, typ, jsToJSONProps);
}

function a(typ: any) {
    return { arrayItems: typ };
}

function u(...typs: any[]) {
    return { unionMembers: typs };
}

function o(props: any[], additional: any) {
    return { props, additional };
}

function m(additional: any) {
    return { props: [], additional };
}

function r(name: string) {
    return { ref: name };
}

const typeMap: any = {
    "Quote": o([
        { json: "$ref", js: "$ref", typ: "" },
        { json: "$schema", js: "$schema", typ: "" },
        { json: "definitions", js: "definitions", typ: r("Definitions") },
    ], false),
    "Definitions": o([
        { json: "Buy", js: "Buy", typ: r("Buy") },
        { json: "Data", js: "Data", typ: r("Data") },
        { json: "Depth", js: "Depth", typ: r("Depth") },
        { json: "NseInfy", js: "NseInfy", typ: r("NseInfyClass") },
        { json: "Ohlc", js: "Ohlc", typ: r("Ohlc") },
        { json: "Quote", js: "Quote", typ: r("QuoteClass") },
    ], false),
    "Buy": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("BuyProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "BuyProperties": o([
        { json: "orders", js: "orders", typ: r("Orders") },
        { json: "price", js: "price", typ: r("Orders") },
        { json: "quantity", js: "quantity", typ: r("Orders") },
    ], false),
    "Orders": o([
        { json: "type", js: "type", typ: r("Type") },
    ], false),
    "Data": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("DataProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "DataProperties": o([
        { json: "NSE:INFY", js: "NSE:INFY", typ: r("NseInfy") },
    ], false),
    "NseInfy": o([
        { json: "$ref", js: "$ref", typ: "" },
    ], false),
    "Depth": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("DepthProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "DepthProperties": o([
        { json: "buy", js: "buy", typ: r("BuyClass") },
        { json: "sell", js: "sell", typ: r("BuyClass") },
    ], false),
    "BuyClass": o([
        { json: "items", js: "items", typ: r("NseInfy") },
        { json: "type", js: "type", typ: "" },
    ], false),
    "NseInfyClass": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("NseInfyProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "NseInfyProperties": o([
        { json: "average_price", js: "average_price", typ: r("Orders") },
        { json: "buy_quantity", js: "buy_quantity", typ: r("Orders") },
        { json: "depth", js: "depth", typ: r("NseInfy") },
        { json: "instrument_token", js: "instrument_token", typ: r("Orders") },
        { json: "last_price", js: "last_price", typ: r("Orders") },
        { json: "last_quantity", js: "last_quantity", typ: r("Orders") },
        { json: "last_trade_time", js: "last_trade_time", typ: r("LastTradeTime") },
        { json: "lower_circuit_limit", js: "lower_circuit_limit", typ: r("Orders") },
        { json: "net_change", js: "net_change", typ: r("Orders") },
        { json: "ohlc", js: "ohlc", typ: r("NseInfy") },
        { json: "oi", js: "oi", typ: r("Orders") },
        { json: "oi_day_high", js: "oi_day_high", typ: r("Orders") },
        { json: "oi_day_low", js: "oi_day_low", typ: r("Orders") },
        { json: "sell_quantity", js: "sell_quantity", typ: r("Orders") },
        { json: "timestamp", js: "timestamp", typ: r("LastTradeTime") },
        { json: "upper_circuit_limit", js: "upper_circuit_limit", typ: r("Orders") },
        { json: "volume", js: "volume", typ: r("Orders") },
    ], false),
    "LastTradeTime": o([
        { json: "format", js: "format", typ: "" },
        { json: "type", js: "type", typ: r("Type") },
    ], false),
    "Ohlc": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("OhlcProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "OhlcProperties": o([
        { json: "close", js: "close", typ: r("Orders") },
        { json: "high", js: "high", typ: r("Orders") },
        { json: "low", js: "low", typ: r("Orders") },
        { json: "open", js: "open", typ: r("Orders") },
    ], false),
    "QuoteClass": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("QuoteProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "QuoteProperties": o([
        { json: "data", js: "data", typ: r("NseInfy") },
        { json: "status", js: "status", typ: r("Orders") },
    ], false),
    "Type": [
        "integer",
        "number",
        "string",
    ],
};
