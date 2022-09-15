// To parse this data:
//
//   import { Convert, MarginsEquity } from "./file";
//
//   const marginsEquity = Convert.toMarginsEquity(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

export interface MarginsEquity {
    $ref:        string;
    $schema:     string;
    definitions: Definitions;
}

export interface Definitions {
    Available:     Available;
    Data:          Data;
    MarginsEquity: MarginsEquityClass;
}

export interface Available {
    additionalProperties: boolean;
    properties:           AvailableProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface AvailableProperties {
    adhoc_margin:    AdhocMargin;
    cash:            AdhocMargin;
    collateral:      AdhocMargin;
    intraday_payin:  AdhocMargin;
    live_balance:    AdhocMargin;
    opening_balance: AdhocMargin;
}

export interface AdhocMargin {
    type: string;
}

export interface Data {
    additionalProperties: boolean;
    properties:           DataProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface DataProperties {
    available: AvailableClass;
    enabled:   AdhocMargin;
    net:       AdhocMargin;
    utilised:  Utilised;
}

export interface AvailableClass {
    $ref: string;
}

export interface Utilised {
    additionalProperties: AdhocMargin;
    type:                 string;
}

export interface MarginsEquityClass {
    additionalProperties: boolean;
    properties:           MarginsEquityProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface MarginsEquityProperties {
    data:   AvailableClass;
    status: AdhocMargin;
}

// Converts JSON strings to/from your types
// and asserts the results of JSON.parse at runtime
export class Convert {
    public static toMarginsEquity(json: string): MarginsEquity {
        return cast(JSON.parse(json), r("MarginsEquity"));
    }

    public static marginsEquityToJson(value: MarginsEquity): string {
        return JSON.stringify(uncast(value, r("MarginsEquity")), null, 2);
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
    "MarginsEquity": o([
        { json: "$ref", js: "$ref", typ: "" },
        { json: "$schema", js: "$schema", typ: "" },
        { json: "definitions", js: "definitions", typ: r("Definitions") },
    ], false),
    "Definitions": o([
        { json: "Available", js: "Available", typ: r("Available") },
        { json: "Data", js: "Data", typ: r("Data") },
        { json: "MarginsEquity", js: "MarginsEquity", typ: r("MarginsEquityClass") },
    ], false),
    "Available": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("AvailableProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "AvailableProperties": o([
        { json: "adhoc_margin", js: "adhoc_margin", typ: r("AdhocMargin") },
        { json: "cash", js: "cash", typ: r("AdhocMargin") },
        { json: "collateral", js: "collateral", typ: r("AdhocMargin") },
        { json: "intraday_payin", js: "intraday_payin", typ: r("AdhocMargin") },
        { json: "live_balance", js: "live_balance", typ: r("AdhocMargin") },
        { json: "opening_balance", js: "opening_balance", typ: r("AdhocMargin") },
    ], false),
    "AdhocMargin": o([
        { json: "type", js: "type", typ: "" },
    ], false),
    "Data": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("DataProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "DataProperties": o([
        { json: "available", js: "available", typ: r("AvailableClass") },
        { json: "enabled", js: "enabled", typ: r("AdhocMargin") },
        { json: "net", js: "net", typ: r("AdhocMargin") },
        { json: "utilised", js: "utilised", typ: r("Utilised") },
    ], false),
    "AvailableClass": o([
        { json: "$ref", js: "$ref", typ: "" },
    ], false),
    "Utilised": o([
        { json: "additionalProperties", js: "additionalProperties", typ: r("AdhocMargin") },
        { json: "type", js: "type", typ: "" },
    ], false),
    "MarginsEquityClass": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("MarginsEquityProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "MarginsEquityProperties": o([
        { json: "data", js: "data", typ: r("AvailableClass") },
        { json: "status", js: "status", typ: r("AdhocMargin") },
    ], false),
};
