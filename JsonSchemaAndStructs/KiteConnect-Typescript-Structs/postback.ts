// To parse this data:
//
//   import { Convert, Postback } from "./file";
//
//   const postback = Convert.toPostback(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

export interface Postback {
    $ref:        string;
    $schema:     string;
    definitions: Definitions;
}

export interface Definitions {
    Meta:     Meta;
    Postback: PostbackClass;
}

export interface Meta {
    additionalProperties: boolean;
    title:                string;
    type:                 string;
}

export interface PostbackClass {
    additionalProperties: boolean;
    properties:           Properties;
    required:             string[];
    title:                string;
    type:                 string;
}

export interface Properties {
    app_id:                    AppID;
    average_price:             AppID;
    cancelled_quantity:        AppID;
    checksum:                  AppID;
    disclosed_quantity:        AppID;
    exchange:                  AppID;
    exchange_order_id:         AppID;
    exchange_timestamp:        Timestamp;
    exchange_update_timestamp: Timestamp;
    filled_quantity:           AppID;
    guid:                      AppID;
    instrument_token:          AppID;
    market_protection:         AppID;
    meta:                      MetaClass;
    order_id:                  AppID;
    order_timestamp:           Timestamp;
    order_type:                AppID;
    parent_order_id:           AppID;
    pending_quantity:          AppID;
    placed_by:                 AppID;
    price:                     AppID;
    product:                   AppID;
    quantity:                  AppID;
    status:                    AppID;
    status_message:            AppID;
    status_message_raw:        AppID;
    tag:                       AppID;
    tradingsymbol:             AppID;
    transaction_type:          AppID;
    trigger_price:             AppID;
    unfilled_quantity:         AppID;
    user_id:                   AppID;
    validity:                  AppID;
    variety:                   AppID;
}

export interface AppID {
    type: Type;
}

export enum Type {
    Integer = "integer",
    Null = "null",
    String = "string",
}

export interface Timestamp {
    format: string;
    type:   Type;
}

export interface MetaClass {
    $ref: string;
}

// Converts JSON strings to/from your types
// and asserts the results of JSON.parse at runtime
export class Convert {
    public static toPostback(json: string): Postback {
        return cast(JSON.parse(json), r("Postback"));
    }

    public static postbackToJson(value: Postback): string {
        return JSON.stringify(uncast(value, r("Postback")), null, 2);
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
    "Postback": o([
        { json: "$ref", js: "$ref", typ: "" },
        { json: "$schema", js: "$schema", typ: "" },
        { json: "definitions", js: "definitions", typ: r("Definitions") },
    ], false),
    "Definitions": o([
        { json: "Meta", js: "Meta", typ: r("Meta") },
        { json: "Postback", js: "Postback", typ: r("PostbackClass") },
    ], false),
    "Meta": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "PostbackClass": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("Properties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "Properties": o([
        { json: "app_id", js: "app_id", typ: r("AppID") },
        { json: "average_price", js: "average_price", typ: r("AppID") },
        { json: "cancelled_quantity", js: "cancelled_quantity", typ: r("AppID") },
        { json: "checksum", js: "checksum", typ: r("AppID") },
        { json: "disclosed_quantity", js: "disclosed_quantity", typ: r("AppID") },
        { json: "exchange", js: "exchange", typ: r("AppID") },
        { json: "exchange_order_id", js: "exchange_order_id", typ: r("AppID") },
        { json: "exchange_timestamp", js: "exchange_timestamp", typ: r("Timestamp") },
        { json: "exchange_update_timestamp", js: "exchange_update_timestamp", typ: r("Timestamp") },
        { json: "filled_quantity", js: "filled_quantity", typ: r("AppID") },
        { json: "guid", js: "guid", typ: r("AppID") },
        { json: "instrument_token", js: "instrument_token", typ: r("AppID") },
        { json: "market_protection", js: "market_protection", typ: r("AppID") },
        { json: "meta", js: "meta", typ: r("MetaClass") },
        { json: "order_id", js: "order_id", typ: r("AppID") },
        { json: "order_timestamp", js: "order_timestamp", typ: r("Timestamp") },
        { json: "order_type", js: "order_type", typ: r("AppID") },
        { json: "parent_order_id", js: "parent_order_id", typ: r("AppID") },
        { json: "pending_quantity", js: "pending_quantity", typ: r("AppID") },
        { json: "placed_by", js: "placed_by", typ: r("AppID") },
        { json: "price", js: "price", typ: r("AppID") },
        { json: "product", js: "product", typ: r("AppID") },
        { json: "quantity", js: "quantity", typ: r("AppID") },
        { json: "status", js: "status", typ: r("AppID") },
        { json: "status_message", js: "status_message", typ: r("AppID") },
        { json: "status_message_raw", js: "status_message_raw", typ: r("AppID") },
        { json: "tag", js: "tag", typ: r("AppID") },
        { json: "tradingsymbol", js: "tradingsymbol", typ: r("AppID") },
        { json: "transaction_type", js: "transaction_type", typ: r("AppID") },
        { json: "trigger_price", js: "trigger_price", typ: r("AppID") },
        { json: "unfilled_quantity", js: "unfilled_quantity", typ: r("AppID") },
        { json: "user_id", js: "user_id", typ: r("AppID") },
        { json: "validity", js: "validity", typ: r("AppID") },
        { json: "variety", js: "variety", typ: r("AppID") },
    ], false),
    "AppID": o([
        { json: "type", js: "type", typ: r("Type") },
    ], false),
    "Timestamp": o([
        { json: "format", js: "format", typ: "" },
        { json: "type", js: "type", typ: r("Type") },
    ], false),
    "MetaClass": o([
        { json: "$ref", js: "$ref", typ: "" },
    ], false),
    "Type": [
        "integer",
        "null",
        "string",
    ],
};
