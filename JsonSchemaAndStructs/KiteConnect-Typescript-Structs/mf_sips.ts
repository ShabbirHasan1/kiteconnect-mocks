// To parse this data:
//
//   import { Convert, MFSips } from "./file";
//
//   const mFSips = Convert.toMFSips(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

export interface MFSips {
    $ref:        string;
    $schema:     string;
    definitions: Definitions;
}

export interface Definitions {
    Datum:  Datum;
    MFSips: MFSipsClass;
}

export interface Datum {
    additionalProperties: boolean;
    properties:           DatumProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface DatumProperties {
    completed_instalments: CompletedInstalments;
    created:               Created;
    dividend_type:         CompletedInstalments;
    frequency:             CompletedInstalments;
    fund:                  CompletedInstalments;
    instalment_amount:     CompletedInstalments;
    instalment_day:        CompletedInstalments;
    instalments:           CompletedInstalments;
    last_instalment:       Created;
    next_instalment:       Created;
    pending_instalments:   CompletedInstalments;
    sip_id:                CompletedInstalments;
    sip_reg_num:           SIPRegNum;
    sip_type:              CompletedInstalments;
    status:                CompletedInstalments;
    step_up:               StepUp;
    tag:                   CompletedInstalments;
    tradingsymbol:         CompletedInstalments;
    transaction_type:      CompletedInstalments;
    trigger_price:         CompletedInstalments;
}

export interface CompletedInstalments {
    type: Type;
}

export enum Type {
    Integer = "integer",
    Number = "number",
    String = "string",
}

export interface Created {
    format?: string;
    type:    string;
}

export interface SIPRegNum {
    anyOf: Created[];
}

export interface StepUp {
    additionalProperties: CompletedInstalments;
    type:                 string;
}

export interface MFSipsClass {
    additionalProperties: boolean;
    properties:           MFSipsProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface MFSipsProperties {
    data: Data;
}

export interface Data {
    items: Items;
    type:  string;
}

export interface Items {
    $ref: string;
}

// Converts JSON strings to/from your types
// and asserts the results of JSON.parse at runtime
export class Convert {
    public static toMFSips(json: string): MFSips {
        return cast(JSON.parse(json), r("MFSips"));
    }

    public static mFSipsToJson(value: MFSips): string {
        return JSON.stringify(uncast(value, r("MFSips")), null, 2);
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
    "MFSips": o([
        { json: "$ref", js: "$ref", typ: "" },
        { json: "$schema", js: "$schema", typ: "" },
        { json: "definitions", js: "definitions", typ: r("Definitions") },
    ], false),
    "Definitions": o([
        { json: "Datum", js: "Datum", typ: r("Datum") },
        { json: "MFSips", js: "MFSips", typ: r("MFSipsClass") },
    ], false),
    "Datum": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("DatumProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "DatumProperties": o([
        { json: "completed_instalments", js: "completed_instalments", typ: r("CompletedInstalments") },
        { json: "created", js: "created", typ: r("Created") },
        { json: "dividend_type", js: "dividend_type", typ: r("CompletedInstalments") },
        { json: "frequency", js: "frequency", typ: r("CompletedInstalments") },
        { json: "fund", js: "fund", typ: r("CompletedInstalments") },
        { json: "instalment_amount", js: "instalment_amount", typ: r("CompletedInstalments") },
        { json: "instalment_day", js: "instalment_day", typ: r("CompletedInstalments") },
        { json: "instalments", js: "instalments", typ: r("CompletedInstalments") },
        { json: "last_instalment", js: "last_instalment", typ: r("Created") },
        { json: "next_instalment", js: "next_instalment", typ: r("Created") },
        { json: "pending_instalments", js: "pending_instalments", typ: r("CompletedInstalments") },
        { json: "sip_id", js: "sip_id", typ: r("CompletedInstalments") },
        { json: "sip_reg_num", js: "sip_reg_num", typ: r("SIPRegNum") },
        { json: "sip_type", js: "sip_type", typ: r("CompletedInstalments") },
        { json: "status", js: "status", typ: r("CompletedInstalments") },
        { json: "step_up", js: "step_up", typ: r("StepUp") },
        { json: "tag", js: "tag", typ: r("CompletedInstalments") },
        { json: "tradingsymbol", js: "tradingsymbol", typ: r("CompletedInstalments") },
        { json: "transaction_type", js: "transaction_type", typ: r("CompletedInstalments") },
        { json: "trigger_price", js: "trigger_price", typ: r("CompletedInstalments") },
    ], false),
    "CompletedInstalments": o([
        { json: "type", js: "type", typ: r("Type") },
    ], false),
    "Created": o([
        { json: "format", js: "format", typ: u(undefined, "") },
        { json: "type", js: "type", typ: "" },
    ], false),
    "SIPRegNum": o([
        { json: "anyOf", js: "anyOf", typ: a(r("Created")) },
    ], false),
    "StepUp": o([
        { json: "additionalProperties", js: "additionalProperties", typ: r("CompletedInstalments") },
        { json: "type", js: "type", typ: "" },
    ], false),
    "MFSipsClass": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("MFSipsProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "MFSipsProperties": o([
        { json: "data", js: "data", typ: r("Data") },
    ], false),
    "Data": o([
        { json: "items", js: "items", typ: r("Items") },
        { json: "type", js: "type", typ: "" },
    ], false),
    "Items": o([
        { json: "$ref", js: "$ref", typ: "" },
    ], false),
    "Type": [
        "integer",
        "number",
        "string",
    ],
};
