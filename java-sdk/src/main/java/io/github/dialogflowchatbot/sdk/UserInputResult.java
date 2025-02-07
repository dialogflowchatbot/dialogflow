package io.github.dialogflowchatbot.sdk;

import com.fasterxml.jackson.annotation.JsonProperty;

public enum UserInputResult {
    @JsonProperty("Successful")
    SUCCESSFUL,
    @JsonProperty("Timeout")
    TIMEOUT,
}
