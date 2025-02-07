package io.github.dialogflowchatbot.sdk;

import lombok.Data;
import lombok.EqualsAndHashCode;

@EqualsAndHashCode(callSuper = true)
@Data
public class ImportVariable extends VarData {
    private VarKind varKind;

    public static ImportVariable create(String varName, String varValue, VarKind varKind) {
        ImportVariable v = new ImportVariable();
        v.setVarName(varName);
        v.setVarValue(varValue);
        v.setVarKind(varKind);
        return v;
    }
}
