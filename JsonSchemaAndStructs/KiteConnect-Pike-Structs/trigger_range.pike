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

class TriggerRange {
    mapping(string:Datum)|mixed data;   // json: "data"
    mixed|string                status; // json: "status"

    string encode_json() {
        mapping(string:mixed) json = ([
            "data" : data,
            "status" : status,
        ]);

        return Standards.JSON.encode(json);
    }
}

TriggerRange TriggerRange_from_JSON(mixed json) {
    TriggerRange retval = TriggerRange();

    retval.data = json["data"];
    retval.status = json["status"];

    return retval;
}

class Datum {
    int|mixed   instrument_token; // json: "instrument_token"
    float|mixed lower;            // json: "lower"
    float|mixed upper;            // json: "upper"

    string encode_json() {
        mapping(string:mixed) json = ([
            "instrument_token" : instrument_token,
            "lower" : lower,
            "upper" : upper,
        ]);

        return Standards.JSON.encode(json);
    }
}

Datum Datum_from_JSON(mixed json) {
    Datum retval = Datum();

    retval.instrument_token = json["instrument_token"];
    retval.lower = json["lower"];
    retval.upper = json["upper"];

    return retval;
}
