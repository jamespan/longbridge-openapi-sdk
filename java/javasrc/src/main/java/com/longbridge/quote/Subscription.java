package com.longbridge.quote;

public class Subscription {
    private String symbol;
    private int subTypes;

    public String getSymbol() {
        return symbol;
    }

    public int getSubTypes() {
        return subTypes;
    }

    @Override
    public String toString() {
        return "Subscription [subTypes=" + subTypes + ", symbol=" + symbol + "]";
    }
}