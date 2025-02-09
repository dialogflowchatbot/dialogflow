import requests
from enum import Enum
from dataclasses import dataclass, field, asdict
from typing import List, Optional
from pydantic import BaseModel


class Status(Enum):
    SUCCESS = "success"
    FAILURE = "failure"


class VarKind(Enum):
    STRING = "String"
    NUMBER = "Number"


class UserInputResult(Enum):
    SUCCESSFUL = "Successful"
    TIMEOUT = "Timeout"


class NextAction(Enum):
    TERMINATE = "Terminate"
    NONE = "None"


class VarDataBase(BaseModel):
    varName: str
    varValue: str


class ImportVariable(VarDataBase):
    varKind: VarKind


@dataclass
class RequestData:
    robotId: str = ""
    mainFlowId: str = ""
    sessionId: str = ""
    userInputResult: Optional[UserInputResult] = None
    userInput: str = ""
    importVariables: List[ImportVariable] = field(default_factory=list)
    userInputIntent: str = ""

    def set_robot_id(self, robotId: str):
        self.robotId = robotId

    def set_main_flow_id(self, mainFlowId: str):
        self.mainFlowId = mainFlowId

    def set_session_id(self, sessionId: str):
        self.sessionId = sessionId

    def set_user_input_result(self, userInputResult: UserInputResult):
        self.userInputResult = userInputResult

    def set_user_input(self, userInput: str):
        self.userInput = userInput

    def add_import_variable(self, importVariable: ImportVariable):
        self.importVariables.append(importVariable)

    def set_user_input_intent(self, userInputIntent: str):
        self.userInputIntent = userInputIntent

    def to_dict(self):
        data = asdict(self)
        if data['userInputResult'] is not None:
            data['userInputResult'] = data['userInputResult'].value
        return data


class Answer(BaseModel):
    content: str
    contentType: str


class ExtraData(BaseModel):
    externalLink: str


class ResponseData(BaseModel):
    sessionId: str
    answers: List['Answer']
    collectData: List['CollectData']
    nextAction: NextAction
    extraData: ExtraData


class CollectData(VarDataBase):
    pass


ResponseData.update_forward_refs()


class Response(BaseModel):
    status: int
    data: ResponseData
    err: Optional[str] = None


class DialogFlowChatBotSDK:
    def __init__(self, endpoint: str):
        if not endpoint:
            raise ValueError("Endpoint cannot be empty.")
        if not (endpoint.startswith("http://") or endpoint.startswith("https://")):
            raise ValueError("Endpoint must start with 'http://' or 'https://'.")
        self.endpoint = endpoint

    def send_post_request(self, data: RequestData) -> Response:
        # Validate and set default values if necessary
        if data.sessionId is None:
            data.sessionId = ""
        if data.userInput is None:
            data.userInput = ""
        if data.userInputResult is None:
            data.userInputResult = UserInputResult.SUCCESSFUL
        if data.importVariables is None:
            data.importVariables = []

        try:
            print(data.to_dict())
            res = requests.post(self.endpoint, json=data.to_dict())
            if res.status_code == 200:
                return Response(**res.json())
            else:
                return Response(
                    status=res.status_code,
                    data=ResponseData(
                        sessionId="",
                        answers=[],
                        collectData=[],
                        nextAction=NextAction.NONE,
                        extraData=ExtraData(externalLink="")
                    ),
                    err=res.text
                )
        except requests.exceptions.RequestException as e:
            return Response(
                status=-1,
                data=ResponseData(
                    sessionId="",
                    answers=[],
                    collectData=[],
                    nextAction=NextAction.NONE,
                    extraData=ExtraData(externalLink="")
                ),
                err=str(e)
            )


# 示例用法
if __name__ == "__main__":
    try:
        sdk = DialogFlowChatBotSDK("http://192.168.0.108:12715/flow/answer")
        request_data = RequestData(
            robotId="r03d3slzkhr7y368qwqfkxfdtp",
            mainFlowId="103d3slzkp1pdrzu1fnnve2wwm"
        )

        while True:
            try:
                response = sdk.send_post_request(request_data)

                if response is None:
                    print("Response is None")
                    break

                if response.status != 200:
                    print(f"Response failed with status code: {response.status}")
                    if response.err:
                        print(f"Error: {response.err}")
                    break

                if response.data is None:
                    print("Response data is None")
                    break

                for answer in response.data.answers:
                    print(f"Answer: {answer.content} (Type: {answer.contentType})")

                if response.data.nextAction == NextAction.TERMINATE:
                    print("Terminating the conversation.")
                    break

                if request_data.userInputIntent is not None and len(request_data.userInputIntent) == 0:
                    request_data.userInputIntent = None

                if request_data.sessionId is None or len(request_data.sessionId) == 0:
                    request_data.sessionId = response.data.sessionId

                user_input = input("Input your question: ")
                request_data.set_user_input(user_input)

            except Exception as e:
                print(f"An error occurred: {e}")

    except ValueError as ve:
        print(f"Invalid endpoint: {ve}")



