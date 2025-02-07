package io.github.dialogflowchatbot.sdk;

import lombok.AllArgsConstructor;
import lombok.Data;

@Data
@AllArgsConstructor
public class ImportVariable {
    private String varName;
    private VarKind varKind;
    private String varValue;
}
