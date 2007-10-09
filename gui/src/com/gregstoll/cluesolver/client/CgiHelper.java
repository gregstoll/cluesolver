package com.gregstoll.cluesolver.client;

import com.google.gwt.http.client.Request;
import com.google.gwt.http.client.RequestBuilder;
import com.google.gwt.http.client.RequestCallback;
import com.google.gwt.http.client.RequestException;
import com.google.gwt.http.client.Response;
import com.google.gwt.http.client.URL;

public class CgiHelper {
    public static void doRequest(RequestBuilder.Method method, String script, String arguments, final CgiResponseHandler handler) {
        if (method == RequestBuilder.GET) {
            script = script + "?" + arguments;
        }
        RequestBuilder builder = new RequestBuilder(method, script);
        if (method == RequestBuilder.POST) {
            builder.setHeader("Content-type", "application/x-www-form-urlencoded");
        }
        String body = null;
        if (method == RequestBuilder.POST) {
            body = arguments;
        }
        try {
            Request response = builder.sendRequest(body, new RequestCallback() {
                public void onError(Request request, Throwable exception) {
                    handler.onError(exception);
                }
                public void onResponseReceived(Request request, Response response) {
                    if (200 == response.getStatusCode()) {
                        handler.onSuccess(response.getText());
                    } else {
                        handler.onError(new Exception("Internal error - got response code " + response.getStatusCode()));
                    }
                }
            });
        } catch (RequestException ex) {
            handler.onError(ex);
        }
    }
}
