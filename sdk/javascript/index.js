"use strict";
class DialogFlowChatBotSDK {
    constructor(url) {
        this.url = url;
    }

    VarKind = Object.freeze({
        STRING: 'String',
        NUMBER: 'Number',
    });

    UserInputResult = Object.freeze({
        SUCCESSFUL: 'Successful',
        TIMEOUT: 'Timeout',
    });

    genRequestData(robotId, mainFlowId) {
        return {
            robotId: robotId,
            mainFlowId: mainFlowId,
            sessionId: "",
            userInputResult: this.UserInputResult.SUCCESSFUL,
            userInput: "",
            importVariables: [],
            userInputIntent: null
        };
    }

    async postData(data) {
        if (data.sessionId == null)
            data.sessionId = '';
        if (data.userInput == null)
            data.userInput = '';
        if (data.userInputResult == null)
            data.userInputResult = this.UserInputResult.SUCCESSFUL;
        if (data.importVariables == null)
            data.importVariables = [];
        if (data.userInputIntent != null && data.userInputIntent == '')
            data.userInputIntent = null;
        const response = await fetch(this.url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                // 'Authorization': `Bearer ${this.apiKey}`
            },
            body: JSON.stringify(data)
        });
        const r = await response.json();
        if (r && r.data) {
            if (!data.sessionId)
                data.sessionId = r.data.sessionId;
        }
        return r;
    }
}

export { DialogFlowChatBotSDK };
