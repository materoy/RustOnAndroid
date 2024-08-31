package com.example.rustlib

class RustLib {

    external fun stringFromJNI(): String

    external fun inputFun(input: String): String

    companion object {
        init {
            System.loadLibrary("rust")
        }
    }
}
