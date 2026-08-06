//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
//

use jni::{
    objects::{AutoLocal, JByteArray, JClass, JList, JMap, JObject, JObjectArray, JString, JValue},
    sys::jobject,
    JNIEnv,
};
use prebindgen_jni_runtime::JniBindingError;
use zenoh::bytes::ZBytes;
use zenoh_ext::{VarInt, ZDeserializeError, ZDeserializer, ZSerializer};

type JResult<T> = core::result::Result<T, JniBindingError<()>>;

/// Deliver an error to the foreign `onError` handler — the SAME model the
/// generated wrappers use (no direct `throw` from native). `on_error` is a
/// `io.zenoh.jni.JniErrorHandler<R>` instance (the binding-error arity); we
/// invoke its typed `run` with the message, through the same process-wide
/// cached interface-method the generated code uses. The handler throws
/// `ZError`, so a JVM exception is pending when we return the sentinel. (This
/// hand-written surface holds no native handle locks, so invoking the
/// throwing callback straight from the upcall is safe — no lock is held
/// across the throw.)
fn signal_error(env: &mut JNIEnv, on_error: &JObject, err: &impl core::fmt::Display) {
    static MID: prebindgen_jni_runtime::CachedIfaceMethod =
        prebindgen_jni_runtime::CachedIfaceMethod::new();
    // If a JVM exception is already pending (a Java upcall threw), let it
    // propagate untouched — do NOT invoke the callback over it (and do not
    // clear it): the pending exception surfaces when we return the sentinel.
    if env.exception_check().unwrap_or(false) {
        return;
    }
    let je: JObject = match env.new_string(err.to_string()) {
        Ok(s) => s.into(),
        Err(_) => JObject::null(),
    };
    // The handler throws `ZError`, leaving a pending exception; we ignore the
    // `Err` and return so it propagates.
    let _ = MID.call_object(
        env,
        "io/zenoh/jni/JniErrorHandler",
        "run",
        "(Ljava/lang/String;)Ljava/lang/Object;",
        on_error,
        &[jni::sys::jvalue { l: je.as_raw() }],
    );
}

/// Helper function to convert a JByteArray into a Vec<u8>.
fn decode_byte_array(env: &JNIEnv<'_>, payload: JByteArray) -> JResult<Vec<u8>> {
    let payload_len = env
        .get_array_length(&payload)
        .map(|length| length as usize)
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
    let mut buff = vec![0; payload_len];
    env.get_byte_array_region(payload, 0, &mut buff[..])
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
    let buff: Vec<u8> = unsafe { std::mem::transmute::<Vec<i8>, Vec<u8>>(buff) };
    Ok(buff)
}

fn bytes_to_java_array<'a>(env: &JNIEnv<'a>, slice: &ZBytes) -> JResult<JByteArray<'a>> {
    env.byte_array_from_slice(&slice.to_bytes())
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))
}

enum JavaType {
    Boolean,
    String,
    ByteArray,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    List(Box<JavaType>),
    Map(Box<JavaType>, Box<JavaType>),
}

fn decode_token_type(env: &mut JNIEnv, type_obj: JObject) -> JResult<JavaType> {
    let type_name_jobject = env
        .call_method(&type_obj, "getTypeName", "()Ljava/lang/String;", &[])
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
        .l()
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;

    let qualified_name: String = env
        .get_string(&JString::from(type_name_jobject))
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
        .into();

    match qualified_name.as_str() {
        "java.lang.Boolean" => Ok(JavaType::Boolean),
        "java.lang.String" => Ok(JavaType::String),
        "byte[]" => Ok(JavaType::ByteArray),
        "java.lang.Byte" => Ok(JavaType::Byte),
        "java.lang.Short" => Ok(JavaType::Short),
        "java.lang.Integer" => Ok(JavaType::Int),
        "java.lang.Long" => Ok(JavaType::Long),
        "java.lang.Float" => Ok(JavaType::Float),
        "java.lang.Double" => Ok(JavaType::Double),
        _ => {
            let type_token_class = env
                .find_class("com/google/common/reflect/TypeToken")
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let token_type = env
                .call_static_method(
                    type_token_class,
                    "of",
                    "(Ljava/lang/reflect/Type;)Lcom/google/common/reflect/TypeToken;",
                    &[JValue::Object(&type_obj)],
                )
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .l()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let map_class: JObject = env
                .find_class("java/util/Map")
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .into();
            let is_map_subtype = env
                .call_method(
                    &token_type,
                    "isSubtypeOf",
                    "(Ljava/lang/reflect/Type;)Z",
                    &[JValue::Object(&map_class)],
                )
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .z()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;

            if is_map_subtype {
                let args = env
                    .call_method(
                        &type_obj,
                        "getActualTypeArguments",
                        "()[Ljava/lang/reflect/Type;",
                        &[],
                    )
                    .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                    .l()
                    .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
                let jobject_array = JObjectArray::from(args);
                let arg1 = env
                    .get_object_array_element(&jobject_array, 0)
                    .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
                let arg2 = env
                    .get_object_array_element(&jobject_array, 1)
                    .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;

                return Ok(JavaType::Map(
                    Box::new(decode_token_type(env, arg1)?),
                    Box::new(decode_token_type(env, arg2)?),
                ));
            }

            let list_class: JObject = env
                .find_class("java/util/List")
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .into();
            let is_list_subtype = env
                .call_method(
                    &token_type,
                    "isSubtypeOf",
                    "(Ljava/lang/reflect/Type;)Z",
                    &[JValue::Object(&list_class)],
                )
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .z()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;

            if is_list_subtype {
                let args = env
                    .call_method(
                        &type_obj,
                        "getActualTypeArguments",
                        "()[Ljava/lang/reflect/Type;",
                        &[],
                    )
                    .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                    .l()
                    .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
                let jobject_array = JObjectArray::from(args);
                let arg1 = env
                    .get_object_array_element(&jobject_array, 0)
                    .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;

                return Ok(JavaType::List(Box::new(decode_token_type(env, arg1)?)));
            }

            Err(JniBindingError::<()>::JniError(format!(
                "Unsupported type: {}",
                qualified_name
            )))
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_test_Serialization_serializeViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    any: JObject,
    token_type: JObject,
    on_error: JObject,
) -> jobject {
    || -> JResult<jobject> {
        let mut serializer = ZSerializer::new();
        let jtype = decode_token_type(&mut env, token_type)?;
        serialize(&mut env, &mut serializer, any, &jtype)?;
        let zbytes = serializer.finish();

        let byte_array = bytes_to_java_array(&env, &zbytes)?;
        Ok(byte_array.as_raw())
    }()
    .unwrap_or_else(|err| {
        signal_error(&mut env, &on_error, &err);
        JObject::default().as_raw()
    })
}

fn serialize(
    env: &mut JNIEnv,
    serializer: &mut ZSerializer,
    any: JObject,
    jtype: &JavaType,
) -> JResult<()> {
    match jtype {
        JavaType::Byte => {
            let byte_value = env
                .call_method(any, "byteValue", "()B", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .b()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(byte_value);
        }
        JavaType::Short => {
            let short_value = env
                .call_method(any, "shortValue", "()S", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .s()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(short_value);
        }
        JavaType::Int => {
            let int_value = env
                .call_method(any, "intValue", "()I", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .i()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(int_value);
        }
        JavaType::Long => {
            let long_value = env
                .call_method(any, "longValue", "()J", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .j()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(long_value);
        }
        JavaType::Float => {
            let float_value = env
                .call_method(any, "floatValue", "()F", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .f()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(float_value);
        }
        JavaType::Double => {
            let double_value = env
                .call_method(any, "doubleValue", "()D", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .d()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(double_value);
        }
        JavaType::Boolean => {
            let boolean_value = env
                .call_method(any, "booleanValue", "()Z", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .z()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(boolean_value);
        }
        JavaType::String => {
            let jstring = JString::from(any);
            let string_value: String = env
                .get_string(&jstring)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .into();
            serializer.serialize(string_value);
        }
        JavaType::ByteArray => {
            let jbyte_array = JByteArray::from(any);
            let bytes = decode_byte_array(env, jbyte_array)?;
            serializer.serialize(bytes);
        }
        JavaType::List(kotlin_type) => {
            let jlist: JList<'_, '_, '_> = JList::from_env(env, &any)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let mut iterator = jlist
                .iter(env)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let list_size = jlist
                .size(env)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(VarInt(list_size as usize));
            while let Some(value) = iterator
                .next(env)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
            {
                serialize(env, serializer, value, kotlin_type)?;
            }
        }
        JavaType::Map(key_type, value_type) => {
            let jmap = JMap::from_env(env, &any)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;

            let map_size = env
                .call_method(&jmap, "size", "()I", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .i()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;

            serializer.serialize(VarInt(map_size as usize));

            let mut iterator = jmap
                .iter(env)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            while let Some((key, value)) = iterator
                .next(env)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
            {
                serialize(env, serializer, key, key_type)?;
                serialize(env, serializer, value, value_type)?;
            }
        }
    }
    Ok(())
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_test_Serialization_deserializeViaJNI(
    mut env: JNIEnv,
    _class: JClass,
    bytes: JByteArray,
    jtype: JObject,
    on_error: JObject,
) -> jobject {
    || -> JResult<jobject> {
        let decoded_bytes: Vec<u8> = decode_byte_array(&env, bytes)?;
        let zbytes = ZBytes::from(decoded_bytes);
        let mut deserializer = ZDeserializer::new(&zbytes);
        let jtype = decode_token_type(&mut env, jtype)?;
        let obj = deserialize(&mut env, &mut deserializer, &jtype)?;
        if !deserializer.done() {
            return Err(JniBindingError::<()>::JniError(
                ZDeserializeError.to_string(),
            ));
        }
        Ok(obj)
    }()
    .unwrap_or_else(|err| {
        signal_error(&mut env, &on_error, &err);
        JObject::default().as_raw()
    })
}

fn deserialize(
    env: &mut JNIEnv,
    deserializer: &mut ZDeserializer,
    jtype: &JavaType,
) -> JResult<jobject> {
    match jtype {
        JavaType::Byte => {
            let byte = deserializer
                .deserialize::<i8>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let byte_obj = env
                .new_object("java/lang/Byte", "(B)V", &[JValue::Byte(byte)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(byte_obj.as_raw())
        }
        JavaType::Short => {
            let short = deserializer
                .deserialize::<i16>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let short_obj = env
                .new_object("java/lang/Short", "(S)V", &[JValue::Short(short)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(short_obj.as_raw())
        }
        JavaType::Int => {
            let integer = deserializer
                .deserialize::<i32>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let integer_obj = env
                .new_object("java/lang/Integer", "(I)V", &[JValue::Int(integer)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(integer_obj.as_raw())
        }
        JavaType::Long => {
            let long = deserializer
                .deserialize::<i64>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let long_obj = env
                .new_object("java/lang/Long", "(J)V", &[JValue::Long(long)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(long_obj.as_raw())
        }
        JavaType::Float => {
            let float = deserializer
                .deserialize::<f32>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let float_obj = env
                .new_object("java/lang/Float", "(F)V", &[JValue::Float(float)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(float_obj.as_raw())
        }
        JavaType::Double => {
            let double = deserializer
                .deserialize::<f64>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let double_obj = env
                .new_object("java/lang/Double", "(D)V", &[JValue::Double(double)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(double_obj.as_raw())
        }
        JavaType::Boolean => {
            let boolean_value = deserializer
                .deserialize::<bool>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let jboolean = if boolean_value { 1u8 } else { 0u8 };
            let boolean_obj = env
                .new_object("java/lang/Boolean", "(Z)V", &[JValue::Bool(jboolean)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(boolean_obj.as_raw())
        }
        JavaType::String => {
            let deserialized_string = deserializer
                .deserialize::<String>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let jstring = env
                .new_string(&deserialized_string)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(jstring.into_raw())
        }
        JavaType::ByteArray => {
            let deserialized_bytes = deserializer
                .deserialize::<Vec<u8>>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let jbytes = env
                .byte_array_from_slice(deserialized_bytes.as_slice())
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(jbytes.into_raw())
        }
        JavaType::List(kotlin_type) => {
            let list_size = deserializer
                .deserialize::<VarInt<usize>>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .0;
            let array_list = env
                .new_object("java/util/ArrayList", "()V", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let jlist = JList::from_env(env, &array_list)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;

            for _ in 0..list_size {
                let item = deserialize(env, deserializer, kotlin_type)?;
                let item_obj = unsafe { JObject::from_raw(item) };
                jlist
                    .add(env, &item_obj)
                    .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            }
            Ok(array_list.as_raw())
        }
        JavaType::Map(key_type, value_type) => {
            let map_size = deserializer
                .deserialize::<VarInt<usize>>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .0;
            let map = env
                .new_object("java/util/HashMap", "()V", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let jmap = JMap::from_env(env, &map)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;

            for _ in 0..map_size {
                let key = deserialize(env, deserializer, key_type)?;
                let key_obj = unsafe { JObject::from_raw(key) };
                let value = deserialize(env, deserializer, value_type)?;
                let value_obj = unsafe { JObject::from_raw(value) };
                jmap.put(env, &key_obj, &value_obj)
                    .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            }
            Ok(map.as_raw())
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// KType path — the Kotlin-reflection-aware serializer.
//
// zenoh-java calls the `JavaType` path above with a `java.lang.reflect.Type`,
// which cannot express Kotlin's unsigned value classes (`UInt` erases to `int`)
// or `Pair`/`Triple`. zenoh-kotlin instead passes the full `kotlin.reflect.KType`
// to the functions below, whose `KotlinType` decoder inspects the `KType`'s
// classifier (`KClass.qualifiedName`) and so recognizes the unsigned family and
// `Pair`/`Triple`. Logic ported from the embedded-jni serializer, re-plumbed
// through the split error-handler (`signal_error`/`JniBindingError`), NOT a JVM
// throw. The `JavaType` path is intentionally left untouched.
// ─────────────────────────────────────────────────────────────────────────────

enum KotlinType {
    Boolean,
    String,
    ByteArray,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    UByte,
    UShort,
    UInt,
    ULong,
    List(Box<KotlinType>),
    Map(Box<KotlinType>, Box<KotlinType>),
    Pair(Box<KotlinType>, Box<KotlinType>),
    Triple(Box<KotlinType>, Box<KotlinType>, Box<KotlinType>),
}

fn decode_ktype(env: &mut JNIEnv, ktype: JObject) -> JResult<KotlinType> {
    let classifier_obj = env
        .call_method(
            &ktype,
            "getClassifier",
            "()Lkotlin/reflect/KClassifier;",
            &[],
        )
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
        .l()
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
    let classifier_obj = AutoLocal::new(classifier_obj, env);

    let kclass_class = env
        .find_class("kotlin/reflect/KClass")
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
    let is_kclass = env
        .is_instance_of(&classifier_obj, kclass_class)
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
    if !is_kclass {
        return Err(JniBindingError::<()>::JniError(
            "KType classifier is not a KClass".to_string(),
        ));
    }
    let qualified_name_jobject = env
        .call_method(
            &classifier_obj,
            "getQualifiedName",
            "()Ljava/lang/String;",
            &[],
        )
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
        .l()
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
    let qualified_name: String = env
        .get_string(&JString::from(qualified_name_jobject))
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
        .into();

    match qualified_name.as_str() {
        "kotlin.Boolean" => Ok(KotlinType::Boolean),
        "kotlin.String" => Ok(KotlinType::String),
        "kotlin.ByteArray" => Ok(KotlinType::ByteArray),
        "kotlin.Byte" => Ok(KotlinType::Byte),
        "kotlin.Short" => Ok(KotlinType::Short),
        "kotlin.Int" => Ok(KotlinType::Int),
        "kotlin.Long" => Ok(KotlinType::Long),
        "kotlin.Float" => Ok(KotlinType::Float),
        "kotlin.Double" => Ok(KotlinType::Double),
        "kotlin.UByte" => Ok(KotlinType::UByte),
        "kotlin.UShort" => Ok(KotlinType::UShort),
        "kotlin.UInt" => Ok(KotlinType::UInt),
        "kotlin.ULong" => Ok(KotlinType::ULong),
        "kotlin.collections.List" => Ok(KotlinType::List(Box::new(decode_ktype_arg(
            env, &ktype, 0,
        )?))),
        "kotlin.collections.Map" => Ok(KotlinType::Map(
            Box::new(decode_ktype_arg(env, &ktype, 0)?),
            Box::new(decode_ktype_arg(env, &ktype, 1)?),
        )),
        "kotlin.Pair" => Ok(KotlinType::Pair(
            Box::new(decode_ktype_arg(env, &ktype, 0)?),
            Box::new(decode_ktype_arg(env, &ktype, 1)?),
        )),
        "kotlin.Triple" => Ok(KotlinType::Triple(
            Box::new(decode_ktype_arg(env, &ktype, 0)?),
            Box::new(decode_ktype_arg(env, &ktype, 1)?),
            Box::new(decode_ktype_arg(env, &ktype, 2)?),
        )),
        _ => Err(JniBindingError::<()>::JniError(format!(
            "Unsupported type: {qualified_name}"
        ))),
    }
}

fn decode_ktype_arg(env: &mut JNIEnv, ktype: &JObject, idx: i32) -> JResult<KotlinType> {
    let arguments = env
        .call_method(ktype, "getArguments", "()Ljava/util/List;", &[])
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
        .l()
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
    let arg = env
        .call_method(
            &arguments,
            "get",
            "(I)Ljava/lang/Object;",
            &[JValue::Int(idx)],
        )
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
        .l()
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
    let arg_ktype = env
        .call_method(arg, "getType", "()Lkotlin/reflect/KType;", &[])
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
        .l()
        .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
    decode_ktype(env, arg_ktype)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_test_Serialization_serializeViaJNIKType(
    mut env: JNIEnv,
    _class: JClass,
    any: JObject,
    ktype: JObject,
    on_error: JObject,
) -> jobject {
    || -> JResult<jobject> {
        let mut serializer = ZSerializer::new();
        let kotlin_type = decode_ktype(&mut env, ktype)?;
        serialize_ktype(&mut env, &mut serializer, any, &kotlin_type)?;
        let zbytes = serializer.finish();

        let byte_array = bytes_to_java_array(&env, &zbytes)?;
        Ok(byte_array.as_raw())
    }()
    .unwrap_or_else(|err| {
        signal_error(&mut env, &on_error, &err);
        JObject::default().as_raw()
    })
}

fn serialize_ktype(
    env: &mut JNIEnv,
    serializer: &mut ZSerializer,
    any: JObject,
    ktype: &KotlinType,
) -> JResult<()> {
    match ktype {
        KotlinType::Byte => {
            let byte_value = env
                .call_method(any, "byteValue", "()B", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .b()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(byte_value);
        }
        KotlinType::Short => {
            let short_value = env
                .call_method(any, "shortValue", "()S", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .s()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(short_value);
        }
        KotlinType::Int => {
            let int_value = env
                .call_method(any, "intValue", "()I", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .i()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(int_value);
        }
        KotlinType::Long => {
            let long_value = env
                .call_method(any, "longValue", "()J", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .j()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(long_value);
        }
        KotlinType::Float => {
            let float_value = env
                .call_method(any, "floatValue", "()F", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .f()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(float_value);
        }
        KotlinType::Double => {
            let double_value = env
                .call_method(any, "doubleValue", "()D", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .d()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(double_value);
        }
        KotlinType::Boolean => {
            let boolean_value = env
                .call_method(any, "booleanValue", "()Z", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .z()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(boolean_value);
        }
        KotlinType::String => {
            let jstring = JString::from(any);
            let string_value: String = env
                .get_string(&jstring)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .into();
            serializer.serialize(string_value);
        }
        KotlinType::ByteArray => {
            let jbyte_array = JByteArray::from(any);
            let bytes = decode_byte_array(env, jbyte_array)?;
            serializer.serialize(bytes);
        }
        // Kotlin unsigned value classes box as a `data` field of the signed
        // width; read it and reinterpret. `VarInt` matches the wire form
        // zenoh uses for the unsigned family.
        KotlinType::UByte => {
            let byte = env
                .get_field(any, "data", "B")
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .b()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(byte as u8);
        }
        KotlinType::UShort => {
            let short = env
                .get_field(any, "data", "S")
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .s()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(short as u16);
        }
        KotlinType::UInt => {
            let int = env
                .get_field(any, "data", "I")
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .i()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(int as u32);
        }
        KotlinType::ULong => {
            let long = env
                .get_field(any, "data", "J")
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .j()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(long as u64);
        }
        KotlinType::List(elem_type) => {
            let jlist: JList<'_, '_, '_> = JList::from_env(env, &any)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let list_size = jlist
                .size(env)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(VarInt(list_size as usize));
            let mut iterator = jlist
                .iter(env)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            while let Some(value) = iterator
                .next(env)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
            {
                serialize_ktype(env, serializer, value, elem_type)?;
            }
        }
        KotlinType::Map(key_type, value_type) => {
            let jmap = JMap::from_env(env, &any)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let map_size = env
                .call_method(&jmap, "size", "()I", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .i()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serializer.serialize(VarInt(map_size as usize));
            let mut iterator = jmap
                .iter(env)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            while let Some((key, value)) = iterator
                .next(env)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
            {
                serialize_ktype(env, serializer, key, key_type)?;
                serialize_ktype(env, serializer, value, value_type)?;
            }
        }
        KotlinType::Pair(first_type, second_type) => {
            let first = env
                .call_method(&any, "getFirst", "()Ljava/lang/Object;", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .l()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let second = env
                .call_method(&any, "getSecond", "()Ljava/lang/Object;", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .l()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serialize_ktype(env, serializer, first, first_type)?;
            serialize_ktype(env, serializer, second, second_type)?;
        }
        KotlinType::Triple(first_type, second_type, third_type) => {
            let first = env
                .call_method(&any, "getFirst", "()Ljava/lang/Object;", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .l()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let second = env
                .call_method(&any, "getSecond", "()Ljava/lang/Object;", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .l()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let third = env
                .call_method(&any, "getThird", "()Ljava/lang/Object;", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .l()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            serialize_ktype(env, serializer, first, first_type)?;
            serialize_ktype(env, serializer, second, second_type)?;
            serialize_ktype(env, serializer, third, third_type)?;
        }
    }
    Ok(())
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_io_zenoh_jni_test_Serialization_deserializeViaJNIKType(
    mut env: JNIEnv,
    _class: JClass,
    bytes: JByteArray,
    ktype: JObject,
    on_error: JObject,
) -> jobject {
    || -> JResult<jobject> {
        let decoded_bytes = decode_byte_array(&env, bytes)?;
        let zbytes = ZBytes::from(decoded_bytes);
        let mut deserializer = ZDeserializer::new(&zbytes);
        let kotlin_type = decode_ktype(&mut env, ktype)?;
        let obj = deserialize_ktype(&mut env, &mut deserializer, &kotlin_type)?;
        if !deserializer.done() {
            return Err(JniBindingError::<()>::JniError(
                ZDeserializeError.to_string(),
            ));
        }
        Ok(obj)
    }()
    .unwrap_or_else(|err| {
        signal_error(&mut env, &on_error, &err);
        JObject::default().as_raw()
    })
}

fn deserialize_ktype(
    env: &mut JNIEnv,
    deserializer: &mut ZDeserializer,
    ktype: &KotlinType,
) -> JResult<jobject> {
    match ktype {
        KotlinType::Byte => {
            let byte = deserializer
                .deserialize::<i8>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let byte_obj = env
                .new_object("java/lang/Byte", "(B)V", &[JValue::Byte(byte)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(byte_obj.as_raw())
        }
        KotlinType::Short => {
            let short = deserializer
                .deserialize::<i16>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let short_obj = env
                .new_object("java/lang/Short", "(S)V", &[JValue::Short(short)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(short_obj.as_raw())
        }
        KotlinType::Int => {
            let integer = deserializer
                .deserialize::<i32>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let integer_obj = env
                .new_object("java/lang/Integer", "(I)V", &[JValue::Int(integer)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(integer_obj.as_raw())
        }
        KotlinType::Long => {
            let long = deserializer
                .deserialize::<i64>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let long_obj = env
                .new_object("java/lang/Long", "(J)V", &[JValue::Long(long)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(long_obj.as_raw())
        }
        KotlinType::Float => {
            let float = deserializer
                .deserialize::<f32>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let float_obj = env
                .new_object("java/lang/Float", "(F)V", &[JValue::Float(float)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(float_obj.as_raw())
        }
        KotlinType::Double => {
            let double = deserializer
                .deserialize::<f64>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let double_obj = env
                .new_object("java/lang/Double", "(D)V", &[JValue::Double(double)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(double_obj.as_raw())
        }
        KotlinType::Boolean => {
            let boolean_value = deserializer
                .deserialize::<bool>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let jboolean = if boolean_value { 1u8 } else { 0u8 };
            let boolean_obj = env
                .new_object("java/lang/Boolean", "(Z)V", &[JValue::Bool(jboolean)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(boolean_obj.as_raw())
        }
        KotlinType::String => {
            let deserialized_string = deserializer
                .deserialize::<String>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let jstring = env
                .new_string(&deserialized_string)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(jstring.into_raw())
        }
        KotlinType::ByteArray => {
            let deserialized_bytes = deserializer
                .deserialize::<Vec<u8>>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let jbytes = env
                .byte_array_from_slice(deserialized_bytes.as_slice())
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(jbytes.into_raw())
        }
        KotlinType::UByte => {
            let ubyte = deserializer
                .deserialize::<u8>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let ubyte_obj = env
                .new_object("kotlin/UByte", "(B)V", &[JValue::Byte(ubyte as i8)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(ubyte_obj.as_raw())
        }
        KotlinType::UShort => {
            let ushort = deserializer
                .deserialize::<u16>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let ushort_obj = env
                .new_object("kotlin/UShort", "(S)V", &[JValue::Short(ushort as i16)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(ushort_obj.as_raw())
        }
        KotlinType::UInt => {
            let uint = deserializer
                .deserialize::<u32>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let uint_obj = env
                .new_object("kotlin/UInt", "(I)V", &[JValue::Int(uint as i32)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(uint_obj.as_raw())
        }
        KotlinType::ULong => {
            let ulong = deserializer
                .deserialize::<u64>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let ulong_obj = env
                .new_object("kotlin/ULong", "(J)V", &[JValue::Long(ulong as i64)])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(ulong_obj.as_raw())
        }
        KotlinType::List(elem_type) => {
            let list_size = deserializer
                .deserialize::<VarInt<usize>>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .0;
            let array_list = env
                .new_object("java/util/ArrayList", "()V", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let jlist = JList::from_env(env, &array_list)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            for _ in 0..list_size {
                let item = deserialize_ktype(env, deserializer, elem_type)?;
                let item_obj = unsafe { JObject::from_raw(item) };
                jlist
                    .add(env, &item_obj)
                    .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            }
            Ok(array_list.as_raw())
        }
        KotlinType::Map(key_type, value_type) => {
            let map_size = deserializer
                .deserialize::<VarInt<usize>>()
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?
                .0;
            let map = env
                .new_object("java/util/HashMap", "()V", &[])
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            let jmap = JMap::from_env(env, &map)
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            for _ in 0..map_size {
                let key = deserialize_ktype(env, deserializer, key_type)?;
                let key_obj = unsafe { JObject::from_raw(key) };
                let value = deserialize_ktype(env, deserializer, value_type)?;
                let value_obj = unsafe { JObject::from_raw(value) };
                jmap.put(env, &key_obj, &value_obj)
                    .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            }
            Ok(map.as_raw())
        }
        KotlinType::Pair(first_type, second_type) => {
            let first = deserialize_ktype(env, deserializer, first_type)?;
            let second = deserialize_ktype(env, deserializer, second_type)?;
            let pair = env
                .new_object(
                    "kotlin/Pair",
                    "(Ljava/lang/Object;Ljava/lang/Object;)V",
                    &[
                        JValue::Object(&unsafe { JObject::from_raw(first) }),
                        JValue::Object(&unsafe { JObject::from_raw(second) }),
                    ],
                )
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(pair.as_raw())
        }
        KotlinType::Triple(first_type, second_type, third_type) => {
            let first = deserialize_ktype(env, deserializer, first_type)?;
            let second = deserialize_ktype(env, deserializer, second_type)?;
            let third = deserialize_ktype(env, deserializer, third_type)?;
            let triple = env
                .new_object(
                    "kotlin/Triple",
                    "(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)V",
                    &[
                        JValue::Object(&unsafe { JObject::from_raw(first) }),
                        JValue::Object(&unsafe { JObject::from_raw(second) }),
                        JValue::Object(&unsafe { JObject::from_raw(third) }),
                    ],
                )
                .map_err(|err| JniBindingError::<()>::JniError(err.to_string()))?;
            Ok(triple.as_raw())
        }
    }
}
