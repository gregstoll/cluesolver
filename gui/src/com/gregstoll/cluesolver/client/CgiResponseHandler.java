package com.gregstoll.cluesolver.client;

public interface CgiResponseHandler {
    public void onSuccess(String body);
    public void onError(Throwable ex);
}
