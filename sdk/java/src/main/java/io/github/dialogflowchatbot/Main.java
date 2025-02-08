package io.github.dialogflowchatbot;

import io.github.dialogflowchatbot.sdk.*;

import java.io.IOException;
import java.util.List;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        try (Scanner scanner = new Scanner(System.in)) {
            System.out.println("Robot id:");
            String robotId = scanner.nextLine();
            System.out.println("Main flow id:");
            String mainFlowId = scanner.nextLine();
            RequestData request = RequestData.create(robotId, mainFlowId, null);
            RequestHandler requestHandler = new RequestHandler("http://10.247.144.182:12715/flow/answer");
            Response response;
            while (true) {
                try {
                    response = requestHandler.req(request, 1000);
                } catch (IOException | InterruptedException e) {
                    System.out.println("Request failed, err: " + e.getMessage());
                    response = null;
                }
                if (response == null) {
                    System.out.println("Response failed, please try again.");
                } else if (response.getStatus() != 200) {
                    System.out.println("Response failed: " + response.getErr());
                }  else if (response.getData() == null) {
                    System.out.println("Request failed, please try again.");
                } else {
                    ResponseData data = response.getData();
                    List<Answer> answers = data.getAnswers();
                    if (answers == null || answers.isEmpty())
                        System.out.println("No answer.");
                    else {
                        System.out.println(answers.size() == 1 ? "Answer:" : "Answers:");
                        for (Answer answer : answers) {
                            System.out.println(answer.getContent());
                        }
                    }
                    if (NextAction.TERMINATE.equals(data.getNextAction())) {
                        System.out.println();
                        System.exit(0);
                    }
                    if (request.getSessionId().isEmpty())
                        request.setSessionId(data.getSessionId());
                }
                System.out.println("Input your question:");
                request.setUserInput(scanner.nextLine());
                if (request.getUserInput().isEmpty())
                    request.setUserInputResult(UserInputResult.TIMEOUT);
                else
                    request.setUserInputResult(UserInputResult.SUCCESSFUL);
            }
        }
    }
}
