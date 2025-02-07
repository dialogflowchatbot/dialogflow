package io.github.dialogflowchatbot.sdk;

import lombok.Data;

@Data
public class RequestData {
    private String robotId;
    private String mainFlowId;
    private String sessionId;
    private UserInputResult userInputResult;
    private String userInput;
    private ImportVariable[] importVariables;
    private String userInputIntent;

    public static RequestData create(String robotId, String mainFlowId) {
        RequestData requestData = new RequestData();
        requestData.setRobotId(robotId);
        requestData.setMainFlowId(mainFlowId);
        return requestData;
    }

    public static RequestData create(String robotId, String mainFlowId, String userInput) {
        RequestData requestData = create(robotId, mainFlowId);
        requestData.setUserInput(userInput);
        return requestData;
    }
}
