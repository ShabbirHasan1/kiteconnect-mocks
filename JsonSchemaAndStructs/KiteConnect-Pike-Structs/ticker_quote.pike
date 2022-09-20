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

typedef array(TriggerRangeElement) TickerQuote;

TickerQuote TickerQuote_from_JSON(mixed json) {
    return map(json, TriggerRangeElement_from_JSON);
}

class TriggerRangeElement {
    float|mixed  average_traded_price; // json: "average_traded_price"
    float|mixed  change;               // json: "change"
    int|mixed    instrument_token;     // json: "instrument_token"
    int|mixed    last_price;           // json: "last_price"
    int|mixed    last_traded_quantity; // json: "last_traded_quantity"
    mixed|string mode;                 // json: "mode"
    Ohlc|mixed   ohlc;                 // json: "ohlc"
    int|mixed    total_buy_quantity;   // json: "total_buy_quantity"
    int|mixed    total_sell_quantity;  // json: "total_sell_quantity"
    bool|mixed   tradable;             // json: "tradable"
    int|mixed    volume_traded;        // json: "volume_traded"

    string encode_json() {
        mapping(string:mixed) json = ([
            "average_traded_price" : average_traded_price,
            "change" : change,
            "instrument_token" : instrument_token,
            "last_price" : last_price,
            "last_traded_quantity" : last_traded_quantity,
            "mode" : mode,
            "ohlc" : ohlc,
            "total_buy_quantity" : total_buy_quantity,
            "total_sell_quantity" : total_sell_quantity,
            "tradable" : tradable,
            "volume_traded" : volume_traded,
        ]);

        return Standards.JSON.encode(json);
    }
}

TriggerRangeElement TriggerRangeElement_from_JSON(mixed json) {
    TriggerRangeElement retval = TriggerRangeElement();

    retval.average_traded_price = json["average_traded_price"];
    retval.change = json["change"];
    retval.instrument_token = json["instrument_token"];
    retval.last_price = json["last_price"];
    retval.last_traded_quantity = json["last_traded_quantity"];
    retval.mode = json["mode"];
    retval.ohlc = json["ohlc"];
    retval.total_buy_quantity = json["total_buy_quantity"];
    retval.total_sell_quantity = json["total_sell_quantity"];
    retval.tradable = json["tradable"];
    retval.volume_traded = json["volume_traded"];

    return retval;
}

class Ohlc {
    int|mixed close; // json: "close"
    int|mixed high;  // json: "high"
    int|mixed low;   // json: "low"
    int|mixed open;  // json: "open"

    string encode_json() {
        mapping(string:mixed) json = ([
            "close" : close,
            "high" : high,
            "low" : low,
            "open" : open,
        ]);

        return Standards.JSON.encode(json);
    }
}

Ohlc Ohlc_from_JSON(mixed json) {
    Ohlc retval = Ohlc();

    retval.close = json["close"];
    retval.high = json["high"];
    retval.low = json["low"];
    retval.open = json["open"];

    return retval;
}
