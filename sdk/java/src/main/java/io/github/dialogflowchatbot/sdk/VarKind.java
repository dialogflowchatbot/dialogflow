package io.github.dialogflowchatbot.sdk;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum VarKind {
    @JsonProperty("String")
    STRING,
    @JsonProperty("Number")
    NUMBER,
}
