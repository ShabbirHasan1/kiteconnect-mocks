// To parse this data:
//
//   import { Convert, Holdings } from "./file";
//
//   const holdings = Convert.toHoldings(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

export interface Holdings {
    $ref:        string;
    $schema:     string;
    definitions: Definitions;
}

export interface Definitions {
    Datum:    Datum;
    Holdings: HoldingsClass;
}

export interface Datum {
    additionalProperties: boolean;
    properties:           DatumProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface DatumProperties {
    authorised_date:       AuthorisedDate;
    authorised_quantity:   AuthorisedQuantity;
    average_price:         AuthorisedQuantity;
    close_price:           AuthorisedQuantity;
    collateral_quantity:   AuthorisedQuantity;
    collateral_type:       AuthorisedQuantity;
    day_change:            AuthorisedQuantity;
    day_change_percentage: AuthorisedQuantity;
    discrepancy:           AuthorisedQuantity;
    exchange:              AuthorisedQuantity;
    instrument_token:      AuthorisedQuantity;
    isin:                  AuthorisedQuantity;
    last_price:            AuthorisedQuantity;
    opening_quantity:      AuthorisedQuantity;
    pnl:                   AuthorisedQuantity;
    price:                 AuthorisedQuantity;
    product:               AuthorisedQuantity;
    quantity:              AuthorisedQuantity;
    realised_quantity:     AuthorisedQuantity;
    t1_quantity:           AuthorisedQuantity;
    tradingsymbol:         AuthorisedQuantity;
    used_quantity:         AuthorisedQuantity;
}

export interface AuthorisedDate {
    format: string;
    type:   Type;
}

export enum Type {
    Boolean = "boolean",
    Integer = "integer",
    Number = "number",
    String = "string",
}

export interface AuthorisedQuantity {
    type: Type;
}

export interface HoldingsClass {
    additionalProperties: boolean;
    properties:           HoldingsProperties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface HoldingsProperties {
    data:   Data;
    status: AuthorisedQuantity;
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
    public static toHoldings(json: string): Holdings {
        return cast(JSON.parse(json), r("Holdings"));
    }

    public static holdingsToJson(value: Holdings): string {
        return JSON.stringify(uncast(value, r("Holdings")), null, 2);
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
    "Holdings": o([
        { json: "$ref", js: "$ref", typ: "" },
        { json: "$schema", js: "$schema", typ: "" },
        { json: "definitions", js: "definitions", typ: r("Definitions") },
    ], false),
    "Definitions": o([
        { json: "Datum", js: "Datum", typ: r("Datum") },
        { json: "Holdings", js: "Holdings", typ: r("HoldingsClass") },
    ], false),
    "Datum": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("DatumProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "DatumProperties": o([
        { json: "authorised_date", js: "authorised_date", typ: r("AuthorisedDate") },
        { json: "authorised_quantity", js: "authorised_quantity", typ: r("AuthorisedQuantity") },
        { json: "average_price", js: "average_price", typ: r("AuthorisedQuantity") },
        { json: "close_price", js: "close_price", typ: r("AuthorisedQuantity") },
        { json: "collateral_quantity", js: "collateral_quantity", typ: r("AuthorisedQuantity") },
        { json: "collateral_type", js: "collateral_type", typ: r("AuthorisedQuantity") },
        { json: "day_change", js: "day_change", typ: r("AuthorisedQuantity") },
        { json: "day_change_percentage", js: "day_change_percentage", typ: r("AuthorisedQuantity") },
        { json: "discrepancy", js: "discrepancy", typ: r("AuthorisedQuantity") },
        { json: "exchange", js: "exchange", typ: r("AuthorisedQuantity") },
        { json: "instrument_token", js: "instrument_token", typ: r("AuthorisedQuantity") },
        { json: "isin", js: "isin", typ: r("AuthorisedQuantity") },
        { json: "last_price", js: "last_price", typ: r("AuthorisedQuantity") },
        { json: "opening_quantity", js: "opening_quantity", typ: r("AuthorisedQuantity") },
        { json: "pnl", js: "pnl", typ: r("AuthorisedQuantity") },
        { json: "price", js: "price", typ: r("AuthorisedQuantity") },
        { json: "product", js: "product", typ: r("AuthorisedQuantity") },
        { json: "quantity", js: "quantity", typ: r("AuthorisedQuantity") },
        { json: "realised_quantity", js: "realised_quantity", typ: r("AuthorisedQuantity") },
        { json: "t1_quantity", js: "t1_quantity", typ: r("AuthorisedQuantity") },
        { json: "tradingsymbol", js: "tradingsymbol", typ: r("AuthorisedQuantity") },
        { json: "used_quantity", js: "used_quantity", typ: r("AuthorisedQuantity") },
    ], false),
    "AuthorisedDate": o([
        { json: "format", js: "format", typ: "" },
        { json: "type", js: "type", typ: r("Type") },
    ], false),
    "AuthorisedQuantity": o([
        { json: "type", js: "type", typ: r("Type") },
    ], false),
    "HoldingsClass": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("HoldingsProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "HoldingsProperties": o([
        { json: "data", js: "data", typ: r("Data") },
        { json: "status", js: "status", typ: r("AuthorisedQuantity") },
    ], false),
    "Data": o([
        { json: "items", js: "items", typ: r("Items") },
        { json: "type", js: "type", typ: "" },
    ], false),
    "Items": o([
        { json: "$ref", js: "$ref", typ: "" },
    ], false),
    "Type": [
        "boolean",
        "integer",
        "number",
        "string",
    ],
};
