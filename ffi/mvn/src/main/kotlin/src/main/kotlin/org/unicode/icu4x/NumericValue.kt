package org.unicode.icu4x

import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface NumericValueLib: Library {
    fun icu4x_NumericValue_for_char_mv1(ch: Int): NumericValueNative
    fun icu4x_NumericValue_get_mv1(nativeStruct: NumericValueNative): OptionDouble
    fun icu4x_NumericValue_to_integer_value_mv1(nativeStruct: NumericValueNative): Int
    fun icu4x_NumericValue_from_integer_value_mv1(value: Int): NumericValueNative
}

internal class NumericValueNative: Structure(), Structure.ByValue {
    @JvmField
    internal var value: Int = 0;

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value")
    }
}




internal class OptionNumericValueNative constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: NumericValueNative = NumericValueNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): NumericValueNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: NumericValueNative, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: NumericValueNative): OptionNumericValueNative {
            return OptionNumericValueNative(value, 1)
        }

        internal fun none(): OptionNumericValueNative {
            return OptionNumericValueNative(NumericValueNative(), 0)
        }
    }

}

/** See the [Rust documentation for `NumericValue`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NumericValue.html) for more information.
*/
class NumericValue (var value: Int) {
    companion object {

        internal val libClass: Class<NumericValueLib> = NumericValueLib::class.java
        internal val lib: NumericValueLib = Native.load("icu4x", libClass)
        val NATIVESIZE: Long = Native.getNativeSize(NumericValueNative::class.java).toLong()

        internal fun fromNative(nativeStruct: NumericValueNative): NumericValue {
            val value: Int = nativeStruct.value

            return NumericValue(value)
        }

        @JvmStatic
        
        /** See the [Rust documentation for `for_char`](https://docs.rs/icu/2.1.1/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
        */
        fun forChar(ch: Int): NumericValue {
            
            val returnVal = lib.icu4x_NumericValue_for_char_mv1(ch);
            
            val returnStruct = NumericValue.fromNative(returnVal)
            return returnStruct
        }
        @JvmStatic
        
        /** Convert from an integer value from ICU4C or CodePointMapData
        *
        *See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NumericValue.html#method.from_icu4c_value) for more information.
        */
        fun fromIntegerValue(value: Int): NumericValue {
            
            val returnVal = lib.icu4x_NumericValue_from_integer_value_mv1(value);
            
            val returnStruct = NumericValue.fromNative(returnVal)
            return returnStruct
        }
    }
    internal fun toNative(): NumericValueNative {
        var native = NumericValueNative()
        native.value = this.value
        return native
    }

    
    /** See the [Rust documentation for `get`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NumericValue.html#method.get) for more information.
    */
    fun get(): Double? {
        
        val returnVal = lib.icu4x_NumericValue_get_mv1(this.toNative());
        return returnVal.option()
    }
    
    /** Convert to an integer value usable with ICU4C and CodePointMapData
    *
    *See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/2.1.1/icu/properties/props/struct.NumericValue.html#method.to_icu4c_value) for more information.
    */
    fun toIntegerValue(): Int {
        
        val returnVal = lib.icu4x_NumericValue_to_integer_value_mv1(this.toNative());
        return (returnVal)
    }
}