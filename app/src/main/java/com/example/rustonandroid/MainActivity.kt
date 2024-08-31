package com.example.rustonandroid

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.ui.Modifier
import com.example.cpplib.NativeLib
import com.example.rustlib.RustLib
import com.example.rustonandroid.ui.theme.RustOnAndroidTheme

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val fromCpp = NativeLib().stringFromJNI()
        val rustLib = RustLib()
        val fromRust = rustLib.stringFromJNI()
        val fromRustWithParam = rustLib.inputFun("Kotlin")
        enableEdgeToEdge()
        setContent {
            RustOnAndroidTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    Column(Modifier.padding(innerPadding)) {
                        Text(text = fromCpp)
                        Text(text = fromRust)
                        Text(text = fromRustWithParam)
                    }
                }
            }
        }
    }
}