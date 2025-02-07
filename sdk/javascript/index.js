"use strict";
class DialogFlowChatBotSDK {
    constructor(url) {
        this.url = url;
    }

    const VarKind = Object.freeze({
        STRING: 'String',
        NUMBER: 'Number',
    });

    const UserInputResult = Object.freeze({
    SUCCESSFUL:'Successful',
    TIMEOUT:'Timeout',
    });

    genRequestData(robotId, mainFlowId) {
    return {
             robotId: robotId,
             mainFlowId: mainFlowId,
             sessionId: "",
             userInputResult: this.UserInputResult.SUCCESSFUL,
             userInput: "",
             importVariables: [],
             userInputIntent: ""
           };
    }

    async postData(data) {
        const response = await fetch(this.url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
//                'Authorization': `Bearer ${this.apiKey}`
            },
            body: JSON.stringify(data)
        });
        return await response.json();
    }
}

module.exports = DialogFlowChatBotSDK;
