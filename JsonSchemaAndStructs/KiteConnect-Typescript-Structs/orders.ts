// To parse this data:
//
//   import { Convert, Orders } from "./file";
//
//   const orders = Convert.toOrders(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

export interface Orders {
    data?:   Datum[];
    status?: string;
}

export interface Datum {
    averagePrice?:            number;
    cancelledQuantity?:       number;
    disclosedQuantity?:       number;
    exchange?:                string;
    exchangeOrderID?:         null | string;
    exchangeTimestamp?:       Date | null;
    exchangeUpdateTimestamp?: Date | null;
    filledQuantity?:          number;
    guid?:                    string;
    instrumentToken?:         number;
    marketProtection?:        number;
    meta?:                    Meta;
    modified?:                boolean;
    orderID?:                 string;
    orderTimestamp?:          Date;
    orderType?:               string;
    parentOrderID?:           null;
    pendingQuantity?:         number;
    placedBy?:                string;
    price?:                   number;
    product?:                 string;
    quantity?:                number;
    status?:                  string;
    statusMessage?:           null | string;
    statusMessageRaw?:        null | string;
    tag?:                     null | string;
    tags?:                    string[];
    tradingsymbol?:           string;
    transactionType?:         string;
    triggerPrice?:            number;
    validity?:                string;
    validityTTL?:             number;
    variety?:                 string;
}

export interface Meta {
    iceberg?: Iceberg;
}

export interface Iceberg {
    leg?:               number;
    legQuantity?:       number;
    legs?:              number;
    remainingQuantity?: number;
    totalQuantity?:     number;
}

// Converts JSON strings to/from your types
// and asserts the results of JSON.parse at runtime
export class Convert {
    public static toOrders(json: string): Orders {
        return cast(JSON.parse(json), r("Orders"));
    }

    public static ordersToJson(value: Orders): string {
        return JSON.stringify(uncast(value, r("Orders")), null, 2);
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
    "Orders": o([
        { json: "data", js: "data", typ: u(undefined, a(r("Datum"))) },
        { json: "status", js: "status", typ: u(undefined, "") },
    ], false),
    "Datum": o([
        { json: "average_price", js: "averagePrice", typ: u(undefined, 0) },
        { json: "cancelled_quantity", js: "cancelledQuantity", typ: u(undefined, 0) },
        { json: "disclosed_quantity", js: "disclosedQuantity", typ: u(undefined, 0) },
        { json: "exchange", js: "exchange", typ: u(undefined, "") },
        { json: "exchange_order_id", js: "exchangeOrderID", typ: u(undefined, u(null, "")) },
        { json: "exchange_timestamp", js: "exchangeTimestamp", typ: u(undefined, u(Date, null)) },
        { json: "exchange_update_timestamp", js: "exchangeUpdateTimestamp", typ: u(undefined, u(Date, null)) },
        { json: "filled_quantity", js: "filledQuantity", typ: u(undefined, 0) },
        { json: "guid", js: "guid", typ: u(undefined, "") },
        { json: "instrument_token", js: "instrumentToken", typ: u(undefined, 0) },
        { json: "market_protection", js: "marketProtection", typ: u(undefined, 0) },
        { json: "meta", js: "meta", typ: u(undefined, r("Meta")) },
        { json: "modified", js: "modified", typ: u(undefined, true) },
        { json: "order_id", js: "orderID", typ: u(undefined, "") },
        { json: "order_timestamp", js: "orderTimestamp", typ: u(undefined, Date) },
        { json: "order_type", js: "orderType", typ: u(undefined, "") },
        { json: "parent_order_id", js: "parentOrderID", typ: u(undefined, null) },
        { json: "pending_quantity", js: "pendingQuantity", typ: u(undefined, 0) },
        { json: "placed_by", js: "placedBy", typ: u(undefined, "") },
        { json: "price", js: "price", typ: u(undefined, 0) },
        { json: "product", js: "product", typ: u(undefined, "") },
        { json: "quantity", js: "quantity", typ: u(undefined, 0) },
        { json: "status", js: "status", typ: u(undefined, "") },
        { json: "status_message", js: "statusMessage", typ: u(undefined, u(null, "")) },
        { json: "status_message_raw", js: "statusMessageRaw", typ: u(undefined, u(null, "")) },
        { json: "tag", js: "tag", typ: u(undefined, u(null, "")) },
        { json: "tags", js: "tags", typ: u(undefined, a("")) },
        { json: "tradingsymbol", js: "tradingsymbol", typ: u(undefined, "") },
        { json: "transaction_type", js: "transactionType", typ: u(undefined, "") },
        { json: "trigger_price", js: "triggerPrice", typ: u(undefined, 0) },
        { json: "validity", js: "validity", typ: u(undefined, "") },
        { json: "validity_ttl", js: "validityTTL", typ: u(undefined, 0) },
        { json: "variety", js: "variety", typ: u(undefined, "") },
    ], false),
    "Meta": o([
        { json: "iceberg", js: "iceberg", typ: u(undefined, r("Iceberg")) },
    ], false),
    "Iceberg": o([
        { json: "leg", js: "leg", typ: u(undefined, 0) },
        { json: "leg_quantity", js: "legQuantity", typ: u(undefined, 0) },
        { json: "legs", js: "legs", typ: u(undefined, 0) },
        { json: "remaining_quantity", js: "remainingQuantity", typ: u(undefined, 0) },
        { json: "total_quantity", js: "totalQuantity", typ: u(undefined, 0) },
    ], false),
};
