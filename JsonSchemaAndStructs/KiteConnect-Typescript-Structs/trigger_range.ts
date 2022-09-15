// To parse this data:
//
//   import { Convert, TriggerRange } from "./file";
//
//   const triggerRange = Convert.toTriggerRange(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

export interface TriggerRange {
    $ref:        string;
    $schema:     string;
    definitions: Definitions;
}

export interface Definitions {
    Data:         Data;
    Nse:          Nse;
    TriggerRange: TriggerRangeClass;
}

export interface Data {
    additionalProperties: boolean;
    properties:           DataProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface DataProperties {
    "NSE:INFY":     NseInfy;
    "NSE:RELIANCE": NseInfy;
}

export interface NseInfy {
    $ref: string;
}

export interface Nse {
    additionalProperties: boolean;
    properties:           NseProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface NseProperties {
    instrument_token: InstrumentToken;
    lower:            InstrumentToken;
    upper:            InstrumentToken;
}

export interface InstrumentToken {
    type: string;
}

export interface TriggerRangeClass {
    additionalProperties: boolean;
    properties:           TriggerRangeProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface TriggerRangeProperties {
    data:   NseInfy;
    status: InstrumentToken;
}

// Converts JSON strings to/from your types
// and asserts the results of JSON.parse at runtime
export class Convert {
    public static toTriggerRange(json: string): TriggerRange {
        return cast(JSON.parse(json), r("TriggerRange"));
    }

    public static triggerRangeToJson(value: TriggerRange): string {
        return JSON.stringify(uncast(value, r("TriggerRange")), null, 2);
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
    "TriggerRange": o([
        { json: "$ref", js: "$ref", typ: "" },
        { json: "$schema", js: "$schema", typ: "" },
        { json: "definitions", js: "definitions", typ: r("Definitions") },
    ], false),
    "Definitions": o([
        { json: "Data", js: "Data", typ: r("Data") },
        { json: "Nse", js: "Nse", typ: r("Nse") },
        { json: "TriggerRange", js: "TriggerRange", typ: r("TriggerRangeClass") },
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
        { json: "NSE:RELIANCE", js: "NSE:RELIANCE", typ: r("NseInfy") },
    ], false),
    "NseInfy": o([
        { json: "$ref", js: "$ref", typ: "" },
    ], false),
    "Nse": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("NseProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "NseProperties": o([
        { json: "instrument_token", js: "instrument_token", typ: r("InstrumentToken") },
        { json: "lower", js: "lower", typ: r("InstrumentToken") },
        { json: "upper", js: "upper", typ: r("InstrumentToken") },
    ], false),
    "InstrumentToken": o([
        { json: "type", js: "type", typ: "" },
    ], false),
    "TriggerRangeClass": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("TriggerRangeProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "TriggerRangeProperties": o([
        { json: "data", js: "data", typ: r("NseInfy") },
        { json: "status", js: "status", typ: r("InstrumentToken") },
    ], false),
};
