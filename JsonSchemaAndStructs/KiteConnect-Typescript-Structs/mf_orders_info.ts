// To parse this data:
//
//   import { Convert, MFOrdersInfo } from "./file";
//
//   const mFOrdersInfo = Convert.toMFOrdersInfo(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

export interface MFOrdersInfo {
    $ref:        string;
    $schema:     string;
    definitions: Definitions;
}

export interface Definitions {
    Data:         Data;
    MFOrdersInfo: MFOrdersInfoClass;
}

export interface Data {
    additionalProperties: boolean;
    properties:           DataProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface DataProperties {
    amount:             Amount;
    average_price:      Amount;
    exchange_order_id:  Amount;
    exchange_timestamp: Amount;
    folio:              Amount;
    fund:               Amount;
    last_price:         Amount;
    last_price_date:    LastPriceDate;
    order_id:           LastPriceDate;
    order_timestamp:    LastPriceDate;
    placed_by:          Amount;
    purchase_type:      Amount;
    quantity:           Amount;
    settlement_id:      Amount;
    status:             Amount;
    status_message:     Amount;
    tag:                Amount;
    tradingsymbol:      Amount;
    transaction_type:   Amount;
    variety:            Amount;
}

export interface Amount {
    type: Type;
}

export enum Type {
    Integer = "integer",
    Null = "null",
    Number = "number",
    String = "string",
}

export interface LastPriceDate {
    format: string;
    type:   Type;
}

export interface MFOrdersInfoClass {
    additionalProperties: boolean;
    properties:           MFOrdersInfoProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface MFOrdersInfoProperties {
    data:   DataClass;
    status: Amount;
}

export interface DataClass {
    $ref: string;
}

// Converts JSON strings to/from your types
// and asserts the results of JSON.parse at runtime
export class Convert {
    public static toMFOrdersInfo(json: string): MFOrdersInfo {
        return cast(JSON.parse(json), r("MFOrdersInfo"));
    }

    public static mFOrdersInfoToJson(value: MFOrdersInfo): string {
        return JSON.stringify(uncast(value, r("MFOrdersInfo")), null, 2);
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
    "MFOrdersInfo": o([
        { json: "$ref", js: "$ref", typ: "" },
        { json: "$schema", js: "$schema", typ: "" },
        { json: "definitions", js: "definitions", typ: r("Definitions") },
    ], false),
    "Definitions": o([
        { json: "Data", js: "Data", typ: r("Data") },
        { json: "MFOrdersInfo", js: "MFOrdersInfo", typ: r("MFOrdersInfoClass") },
    ], false),
    "Data": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("DataProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "DataProperties": o([
        { json: "amount", js: "amount", typ: r("Amount") },
        { json: "average_price", js: "average_price", typ: r("Amount") },
        { json: "exchange_order_id", js: "exchange_order_id", typ: r("Amount") },
        { json: "exchange_timestamp", js: "exchange_timestamp", typ: r("Amount") },
        { json: "folio", js: "folio", typ: r("Amount") },
        { json: "fund", js: "fund", typ: r("Amount") },
        { json: "last_price", js: "last_price", typ: r("Amount") },
        { json: "last_price_date", js: "last_price_date", typ: r("LastPriceDate") },
        { json: "order_id", js: "order_id", typ: r("LastPriceDate") },
        { json: "order_timestamp", js: "order_timestamp", typ: r("LastPriceDate") },
        { json: "placed_by", js: "placed_by", typ: r("Amount") },
        { json: "purchase_type", js: "purchase_type", typ: r("Amount") },
        { json: "quantity", js: "quantity", typ: r("Amount") },
        { json: "settlement_id", js: "settlement_id", typ: r("Amount") },
        { json: "status", js: "status", typ: r("Amount") },
        { json: "status_message", js: "status_message", typ: r("Amount") },
        { json: "tag", js: "tag", typ: r("Amount") },
        { json: "tradingsymbol", js: "tradingsymbol", typ: r("Amount") },
        { json: "transaction_type", js: "transaction_type", typ: r("Amount") },
        { json: "variety", js: "variety", typ: r("Amount") },
    ], false),
    "Amount": o([
        { json: "type", js: "type", typ: r("Type") },
    ], false),
    "LastPriceDate": o([
        { json: "format", js: "format", typ: "" },
        { json: "type", js: "type", typ: r("Type") },
    ], false),
    "MFOrdersInfoClass": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("MFOrdersInfoProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "MFOrdersInfoProperties": o([
        { json: "data", js: "data", typ: r("DataClass") },
        { json: "status", js: "status", typ: r("Amount") },
    ], false),
    "DataClass": o([
        { json: "$ref", js: "$ref", typ: "" },
    ], false),
    "Type": [
        "integer",
        "null",
        "number",
        "string",
    ],
};
