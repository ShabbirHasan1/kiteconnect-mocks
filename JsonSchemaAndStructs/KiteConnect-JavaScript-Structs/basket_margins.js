// To parse this data:
//
//   const Convert = require("./file");
//
//   const basketMargins = Convert.toBasketMargins(json);
//
// These functions will throw an error if the JSON doesn't
// match the expected interface, even if the JSON is valid.

// Converts JSON strings to/from your types
// and asserts the results of JSON.parse at runtime
function toBasketMargins(json) {
    return cast(JSON.parse(json), r("BasketMargins"));
}

function basketMarginsToJson(value) {
    return JSON.stringify(uncast(value, r("BasketMargins")), null, 2);
}

function invalidValue(typ, val, key = '') {
    if (key) {
        throw Error(`Invalid value for key "${key}". Expected type ${JSON.stringify(typ)} but got ${JSON.stringify(val)}`);
    }
    throw Error(`Invalid value ${JSON.stringify(val)} for type ${JSON.stringify(typ)}`, );
}

function jsonToJSProps(typ) {
    if (typ.jsonToJS === undefined) {
        const map = {};
        typ.props.forEach((p) => map[p.json] = { key: p.js, typ: p.typ });
        typ.jsonToJS = map;
    }
    return typ.jsonToJS;
}

function jsToJSONProps(typ) {
    if (typ.jsToJSON === undefined) {
        const map = {};
        typ.props.forEach((p) => map[p.js] = { key: p.json, typ: p.typ });
        typ.jsToJSON = map;
    }
    return typ.jsToJSON;
}

function transform(val, typ, getProps, key = '') {
    function transformPrimitive(typ, val) {
        if (typeof typ === typeof val) return val;
        return invalidValue(typ, val, key);
    }

    function transformUnion(typs, val) {
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

    function transformEnum(cases, val) {
        if (cases.indexOf(val) !== -1) return val;
        return invalidValue(cases, val);
    }

    function transformArray(typ, val) {
        // val must be an array with no invalid elements
        if (!Array.isArray(val)) return invalidValue("array", val);
        return val.map(el => transform(el, typ, getProps));
    }

    function transformDate(val) {
        if (val === null) {
            return null;
        }
        const d = new Date(val);
        if (isNaN(d.valueOf())) {
            return invalidValue("Date", val);
        }
        return d;
    }

    function transformObject(props, additional, val) {
        if (val === null || typeof val !== "object" || Array.isArray(val)) {
            return invalidValue("object", val);
        }
        const result = {};
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

function cast(val, typ) {
    return transform(val, typ, jsonToJSProps);
}

function uncast(val, typ) {
    return transform(val, typ, jsToJSONProps);
}

function a(typ) {
    return { arrayItems: typ };
}

function u(...typs) {
    return { unionMembers: typs };
}

function o(props, additional) {
    return { props, additional };
}

function m(additional) {
    return { props: [], additional };
}

function r(name) {
    return { ref: name };
}

const typeMap = {
    "BasketMargins": o([
        { json: "$ref", js: "$ref", typ: "" },
        { json: "$schema", js: "$schema", typ: "" },
        { json: "definitions", js: "definitions", typ: r("Definitions") },
    ], false),
    "Definitions": o([
        { json: "BasketMargins", js: "BasketMargins", typ: r("BasketMarginsClass") },
        { json: "Data", js: "Data", typ: r("DataClass") },
        { json: "Final", js: "Final", typ: r("Final") },
        { json: "Pnl", js: "Pnl", typ: r("Pnl") },
    ], false),
    "BasketMarginsClass": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("BasketMarginsProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "BasketMarginsProperties": o([
        { json: "data", js: "data", typ: r("Data") },
        { json: "status", js: "status", typ: r("Status") },
    ], false),
    "Data": o([
        { json: "$ref", js: "$ref", typ: "" },
    ], false),
    "Status": o([
        { json: "type", js: "type", typ: r("Type") },
    ], false),
    "DataClass": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("DataProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "DataProperties": o([
        { json: "final", js: "final", typ: r("Data") },
        { json: "initial", js: "initial", typ: r("Data") },
        { json: "orders", js: "orders", typ: r("Orders") },
    ], false),
    "Orders": o([
        { json: "items", js: "items", typ: r("Data") },
        { json: "type", js: "type", typ: "" },
    ], false),
    "Final": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("FinalProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "FinalProperties": o([
        { json: "additional", js: "additional", typ: r("Status") },
        { json: "bo", js: "bo", typ: r("Status") },
        { json: "cash", js: "cash", typ: r("Status") },
        { json: "exchange", js: "exchange", typ: r("Status") },
        { json: "exposure", js: "exposure", typ: r("Status") },
        { json: "option_premium", js: "option_premium", typ: r("Status") },
        { json: "pnl", js: "pnl", typ: r("Data") },
        { json: "span", js: "span", typ: r("Status") },
        { json: "total", js: "total", typ: r("Status") },
        { json: "tradingsymbol", js: "tradingsymbol", typ: r("Status") },
        { json: "type", js: "type", typ: r("Status") },
        { json: "var", js: "var", typ: r("Status") },
    ], false),
    "Pnl": o([
        { json: "additionalProperties", js: "additionalProperties", typ: true },
        { json: "properties", js: "properties", typ: r("PnlProperties") },
        { json: "required", js: "required", typ: a("") },
        { json: "title", js: "title", typ: "" },
        { json: "type", js: "type", typ: "" },
    ], false),
    "PnlProperties": o([
        { json: "realised", js: "realised", typ: r("Status") },
        { json: "unrealised", js: "unrealised", typ: r("Status") },
    ], false),
    "Type": [
        "integer",
        "number",
        "string",
    ],
};

module.exports = {
    "basketMarginsToJson": basketMarginsToJson,
    "toBasketMargins": toBasketMargins,
};
