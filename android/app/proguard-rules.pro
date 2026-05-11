# Don't obfuscate — this is an open-source project
-dontobfuscate

# Keep JNI native methods
-keep class com.noctiro.wherebus.data.NativeWhereBusBridge { *; }

# Keep kotlinx.serialization
-keepattributes *Annotation*, InnerClasses
-keepclassmembers @kotlinx.serialization.Serializable class com.noctiro.wherebus.** {
    *** Companion;
    *** INSTANCE;
    kotlinx.serialization.KSerializer serializer(...);
}
-keepclasseswithmembers class com.noctiro.wherebus.** {
    kotlinx.serialization.KSerializer serializer(...);
}
-keep,includedescriptorclasses class com.noctiro.wherebus.**$$serializer { *; }
-keep class kotlinx.serialization.** { *; }
-dontwarn kotlinx.serialization.**
