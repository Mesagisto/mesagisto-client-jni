package org.meowcat.mesagisto.jni;

class MesagistoClient {

    private String foo = "";

    // This declares that the static `hello` method will be provided
    // a native library.
    private static native String hello(String input);
    private static native void initLogger();

    static {
        System.loadLibrary("mesagisto_client_jni");
    }

    // The rest is just regular ol' Java!
    public static void main(String[] args) {
    	MesagistoClient.initLogger();
        String output = MesagistoClient.hello("josh");
        System.out.println(output);
    }
}
