package io.quicktype;

import com.fasterxml.jackson.annotation.*;

public class InstrumentToken {
    private String type;

    @JsonProperty("type")
    public String getType() { return type; }
    @JsonProperty("type")
    public void setType(String value) { this.type = value; }
}
