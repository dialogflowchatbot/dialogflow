package io.github.dialogflowchatbot;

import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import io.github.dialogflowchatbot.sdk.ImportVariable;
import io.github.dialogflowchatbot.sdk.RequestData;
import io.github.dialogflowchatbot.sdk.Response;
import io.github.dialogflowchatbot.sdk.UserInputResult;
import lombok.extern.slf4j.Slf4j;

import java.io.IOException;
import java.io.InputStream;
import java.net.URI;
import java.net.http.HttpClient;
import java.net.http.HttpRequest;
import java.net.http.HttpResponse;
import java.time.Duration;

@Slf4j
public class RequestHandler {
    private final HttpClient client;
    private final URI endpoint;
    private final ObjectMapper mapper;
    public RequestHandler(String endpoint) {
        this.client = HttpClient.newBuilder()
                .version(HttpClient.Version.HTTP_1_1)
                .build();
        this.endpoint = URI.create(endpoint);
        this.mapper = new ObjectMapper();
        this.mapper.configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);
        System.setProperty("jdk.httpclient.keepalive.timeout", "1800");
    }

    public Response req(String robotId, String mainFlowId, String userInput) throws IOException, InterruptedException {
        return req(userInput, robotId, mainFlowId, 2500);
    }

    public Response req(String robotId, String mainFlowId, String userInput, int timeoutMillis) throws IOException, InterruptedException {
        RequestData requestData = RequestData.create(robotId, mainFlowId, userInput);
        return req(requestData, timeoutMillis);
    }

    public Response req(RequestData requestData, int timeoutMillis) throws IOException, InterruptedException {
        // Check default value of request params
        if (requestData.getSessionId() == null)
            requestData.setSessionId("");
        if (requestData.getUserInput() == null)
            requestData.setUserInput("");
        if (requestData.getUserInputResult() == null)
            requestData.setUserInputResult(UserInputResult.SUCCESSFUL);
        if (requestData.getImportVariables() == null)
            requestData.setImportVariables(new ImportVariable[0]);
        // End
        return post(requestData, timeoutMillis);
    }

    private Response post(RequestData requestData, int timeoutMillis) throws IOException, InterruptedException {
        HttpRequest request = HttpRequest.newBuilder()
                .uri(endpoint)
                .timeout(Duration.ofMillis(timeoutMillis))
                .POST(HttpRequest.BodyPublishers.ofByteArray(mapper.writeValueAsBytes(requestData)))
                .header("Content-Type", "application/json")
                .build();

        // Send request and handle response
        HttpResponse<InputStream> response = client.send(request, HttpResponse.BodyHandlers.ofInputStream());
        try (InputStream body = response.body()) {
            return mapper.readValue(body, Response.class);
        }
    }
}
