// To parse this data:
//
//   import { Convert, MFSIPInfo } from "./file";
//
//   const mFSIPInfo = Convert.toMFSIPInfo(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

export interface MFSIPInfo {
    $ref:        string;
    $schema:     string;
    definitions: Definitions;
}

export interface Definitions {
    Data:      Data;
    MFSIPInfo: MFSIPInfoClass;
    StepUp:    StepUpClass;
}

export interface Data {
    additionalProperties: boolean;
    properties:           DataProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface DataProperties {
    completed_instalments: CompletedInstalments;
    created:               Created;
    dividend_type:         CompletedInstalments;
    frequency:             CompletedInstalments;
    fund:                  CompletedInstalments;
    fund_source:           CompletedInstalments;
    instalment_amount:     CompletedInstalments;
    instalment_day:        CompletedInstalments;
    instalments:           CompletedInstalments;
    last_instalment:       Created;
    next_instalment:       Created;
    pending_instalments:   CompletedInstalments;
    sip_id:                CompletedInstalments;
    sip_reg_num:           CompletedInstalments;
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
    Null = "null",
    Number = "number",
    String = "string",
}

export interface Created {
    format: string;
    type:   Type;
}

export interface StepUp {
    $ref: string;
}

export interface MFSIPInfoClass {
    additionalProperties: boolean;
    properties:           MFSIPInfoProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface MFSIPInfoProperties {
    data:   StepUp;
    status: CompletedInstalments;
}

export interface StepUpClass {
    additionalProperties: boolean;
    properties:           StepUpProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface StepUpProperties {
    "15-02": CompletedInstalments;
}

// Converts JSON strings to/from your types
// and asserts the results of JSON.parse at runtime
export class Convert {
    public static toMFSIPInfo(json: string): MFSIPInfo {
        return cast(JSON.parse(json), r("MFSIPInfo"));
    }

    public static mFSIPInfoToJson(value: MFSIPInfo): string {
        return JSON.stringify(uncast(value, r("MFSIPInfo")), null, 2);
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
    "MFSIPInfo": o([
        { json: "$ref", js: "$ref", typ: "" },
        { json: "$schema", js: "$schema", typ: "" },
        { json: "definitions", js: "definitions", typ: r("Definitions") },
    ], false),
    "Definitions": o([
        { json: "Data", js: "Data", typ: r("Data") },
        { json: "MFSIPInfo", js: "MFSIPInfo", typ: r("MFSIPInfoClass") },
        { json: "StepUp", js: "StepUp", typ: r("StepUpClass") },
    ], false),
    "Data": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("DataProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "DataProperties": o([
        { json: "completed_instalments", js: "completed_instalments", typ: r("CompletedInstalments") },
        { json: "created", js: "created", typ: r("Created") },
        { json: "dividend_type", js: "dividend_type", typ: r("CompletedInstalments") },
        { json: "frequency", js: "frequency", typ: r("CompletedInstalments") },
        { json: "fund", js: "fund", typ: r("CompletedInstalments") },
        { json: "fund_source", js: "fund_source", typ: r("CompletedInstalments") },
        { json: "instalment_amount", js: "instalment_amount", typ: r("CompletedInstalments") },
        { json: "instalment_day", js: "instalment_day", typ: r("CompletedInstalments") },
        { json: "instalments", js: "instalments", typ: r("CompletedInstalments") },
        { json: "last_instalment", js: "last_instalment", typ: r("Created") },
        { json: "next_instalment", js: "next_instalment", typ: r("Created") },
        { json: "pending_instalments", js: "pending_instalments", typ: r("CompletedInstalments") },
        { json: "sip_id", js: "sip_id", typ: r("CompletedInstalments") },
        { json: "sip_reg_num", js: "sip_reg_num", typ: r("CompletedInstalments") },
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
        { json: "format", js: "format", typ: "" },
        { json: "type", js: "type", typ: r("Type") },
    ], false),
    "StepUp": o([
        { json: "$ref", js: "$ref", typ: "" },
    ], false),
    "MFSIPInfoClass": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("MFSIPInfoProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "MFSIPInfoProperties": o([
        { json: "data", js: "data", typ: r("StepUp") },
        { json: "status", js: "status", typ: r("CompletedInstalments") },
    ], false),
    "StepUpClass": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("StepUpProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "StepUpProperties": o([
        { json: "15-02", js: "15-02", typ: r("CompletedInstalments") },
    ], false),
    "Type": [
        "integer",
        "null",
        "number",
        "string",
    ],
};
