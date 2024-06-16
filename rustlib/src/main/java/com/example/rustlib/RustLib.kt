package com.example.rustlib

class RustLib {

    external fun stringFromJNI(): String

    companion object {
        init {
            System.loadLibrary("rust")
        }
    }
}
