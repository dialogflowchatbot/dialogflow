package io.github.dialogflowchatbot.sdk;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum NextAction {
    @JsonProperty("Terminate")
    TERMINATE,
    @JsonProperty("None")
    NONE,
}
