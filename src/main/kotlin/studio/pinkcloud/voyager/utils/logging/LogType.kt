package studio.pinkcloud.voyager.utils.logging

import com.github.ajalt.colormath.model.RGB
import com.github.ajalt.mordant.rendering.TextColors
import com.github.ajalt.mordant.rendering.TextColors.*
import com.github.ajalt.mordant.rendering.TextStyle

object LogType {
    val TRACE = CustomLogType("TRACE", brightCyan, 0)
    val DEBUG = CustomLogType("DEBUG", brightGreen, 1)
    val INFO = CustomLogType("INFO", brightWhite, 2)
    val WARN = CustomLogType("WARN", brightYellow, 3)
    val ERROR = CustomLogType("ERROR", mix(brightYellow, brightRed), 4)
    val FATAL = CustomLogType("FATAL", mix(brightRed, mix(brightRed, white)), 5)
}

data class CustomLogType(
    val name: String,
    val color: TextStyle,
    val severity: Int,
)

fun mix(c1: RGB, c2: RGB): RGB {

    val c3r = (c1.r + c2.r) / 2
    val c3g = (c1.g + c2.g) / 2
    val c3b = (c1.b + c2.b) / 2

    return RGB(c3r, c3g, c3b)
}

fun mix(c1: TextStyle, c2: TextStyle): TextStyle {
    val c3 = mix(c1.color!!.toSRGB(), c2.color!!.toSRGB())
    return TextColors.rgb(c3.r, c3.g, c3.b)
}
