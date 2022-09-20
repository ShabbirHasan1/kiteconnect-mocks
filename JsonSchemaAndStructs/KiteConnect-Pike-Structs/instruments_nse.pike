// This source has been automatically generated by quicktype.
// ( https://github.com/quicktype/quicktype )
//
// To use this code, simply import it into your project as a Pike module.
// To JSON-encode your object, you can pass it to `Standards.JSON.encode`
// or call `encode_json` on it.
//
// To decode a JSON string, first pass it to `Standards.JSON.decode`,
// and then pass the result to `<YourClass>_from_JSON`.
// It will return an instance of <YourClass>.
// Bear in mind that these functions have unexpected behavior,
// and will likely throw an error, if the JSON string does not
// match the expected interface, even if the JSON itself is valid.

typedef array(InstrumentsNseElement) InstrumentsNse;

InstrumentsNse InstrumentsNse_from_JSON(mixed json) {
    return map(json, InstrumentsNseElement_from_JSON);
}

class InstrumentsNseElement {
    Exchange|mixed       exchange;         // json: "exchange"
    int|mixed            exchange_token;   // json: "exchange_token"
    mixed|string         expiry;           // json: "expiry"
    int|mixed            instrument_token; // json: "instrument_token"
    InstrumentType|mixed instrument_type;  // json: "instrument_type"
    int|mixed            last_price;       // json: "last_price"
    int|mixed            lot_size;         // json: "lot_size"
    mixed|string         name;             // json: "name"
    Exchange|mixed       segment;          // json: "segment"
    int|mixed            strike;           // json: "strike"
    float|mixed          tick_size;        // json: "tick_size"
    mixed|string         tradingsymbol;    // json: "tradingsymbol"

    string encode_json() {
        mapping(string:mixed) json = ([
            "exchange" : exchange,
            "exchange_token" : exchange_token,
            "expiry" : expiry,
            "instrument_token" : instrument_token,
            "instrument_type" : instrument_type,
            "last_price" : last_price,
            "lot_size" : lot_size,
            "name" : name,
            "segment" : segment,
            "strike" : strike,
            "tick_size" : tick_size,
            "tradingsymbol" : tradingsymbol,
        ]);

        return Standards.JSON.encode(json);
    }
}

InstrumentsNseElement InstrumentsNseElement_from_JSON(mixed json) {
    InstrumentsNseElement retval = InstrumentsNseElement();

    retval.exchange = json["exchange"];
    retval.exchange_token = json["exchange_token"];
    retval.expiry = json["expiry"];
    retval.instrument_token = json["instrument_token"];
    retval.instrument_type = json["instrument_type"];
    retval.last_price = json["last_price"];
    retval.lot_size = json["lot_size"];
    retval.name = json["name"];
    retval.segment = json["segment"];
    retval.strike = json["strike"];
    retval.tick_size = json["tick_size"];
    retval.tradingsymbol = json["tradingsymbol"];

    return retval;
}

enum Exchange {
    NSE = "NSE", // json: "NSE"
}

enum InstrumentType {
    EQ = "EQ", // json: "EQ"
}
