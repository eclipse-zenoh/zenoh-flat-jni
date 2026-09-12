#[allow(dead_code)]
pub(crate) type __JniErr = ::prebindgen_jni_runtime::JniBindingError<()>;
/// See module-level docs at [`owned_object_prerequisite_items`].
#[allow(dead_code)]
pub(crate) struct OwnedObject<T: ?Sized> {
    ptr: *const T,
}
impl<T: ?Sized> std::ops::Deref for OwnedObject<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}
impl<T: ?Sized> std::ops::DerefMut for OwnedObject<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self.ptr as *mut T) }
    }
}
impl<T: ?Sized> OwnedObject<T> {
    /// Borrow a `T` whose backing `Box<T>` lives on the
    /// Java side. Stores only the pointer; the wrapper
    /// does not own the heap allocation and never frees
    /// it on drop.
    ///
    /// # Safety
    ///
    /// `ptr` must be the result of an earlier
    /// `Box::into_raw(Box::new(v))` and the allocation
    /// must still be live (Java still owns it). The Java
    /// side is responsible for sequencing this call
    /// against any concurrent free or consume (via
    /// `NativeHandle.withPtr` read-lock vs `consume` /
    /// `close` write-lock) so the borrow cannot race a
    /// deallocation on the same pointer.
    #[allow(dead_code)]
    pub(crate) unsafe fn from_raw(ptr: *const T) -> Self {
        Self { ptr }
    }
}
#[allow(non_snake_case, dead_code)]
pub(crate) fn signal_binding_error(
    env: &mut jni::JNIEnv,
    sink: &jni::objects::JObject,
    mid: &::prebindgen_jni_runtime::CachedIfaceMethod,
    fqn: &str,
    descr: &str,
    je: &str,
) {
    if env.exception_check().unwrap_or(false) {
        return;
    }
    let __je: jni::objects::JObject = match env.new_string(je) {
        Ok(s) => s.into(),
        Err(e) => {
            tracing::error!("signal_binding_error: new_string failed: {}", e);
            return;
        }
    };
    let __args = [
        jni::sys::jvalue {
            l: __je.as_raw(),
        },
    ];
    if let Err(e) = mid.call_object(env, fqn, "run", descr, sink, &__args) {
        tracing::error!("signal_binding_error: error-callback invoke failed: {}", e);
    }
}
#[allow(non_snake_case, dead_code)]
pub(crate) fn signal_domain_error(
    env: &mut jni::JNIEnv,
    sink: &jni::objects::JObject,
    mid: &::prebindgen_jni_runtime::CachedIfaceMethod,
    fqn: &str,
    descr: &str,
    ze: &[jni::sys::jvalue],
) {
    if env.exception_check().unwrap_or(false) {
        return;
    }
    if let Err(e) = mid.call_object(env, fqn, "run", descr, sink, ze) {
        tracing::error!("signal_domain_error: error-callback invoke failed: {}", e);
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_bytes_Encoding_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Encoding));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Encoding>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_bytes_ZBytes_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ZBytes));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::ZBytes>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_config_Config_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Config));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Config>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_keyexpr_KeyExpr_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::KeyExpr));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::KeyExpr>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_liveliness_LivelinessToken_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::LivelinessToken));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::LivelinessToken>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_pubsub_AdvancedPublisher_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::AdvancedPublisher));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::AdvancedPublisher>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_pubsub_AdvancedSubscriber_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::AdvancedSubscriber));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::AdvancedSubscriber>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_pubsub_MatchingListener_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::MatchingListener));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::MatchingListener>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_pubsub_Publisher_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Publisher));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Publisher>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_pubsub_SampleMissListener_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::SampleMissListener));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::SampleMissListener>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_pubsub_Subscriber_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Subscriber));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Subscriber>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_Querier_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Querier));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Querier>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_Query_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Query));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Query>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_Queryable_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Queryable));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Queryable>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_ReplyError_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::ReplyError));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::ReplyError>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_query_Reply_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Reply));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Reply>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_sample_Sample_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Sample));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Sample>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_scouting_Hello_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Hello));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Hello>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_scouting_Scout_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Scout));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Scout>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_session_Session_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::Session));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::Session>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub(crate) unsafe extern "C" fn Java_io_zenoh_jni_time_TimestampStack_freePtr(
    _env: jni::JNIEnv,
    _class: jni::objects::JClass,
    ptr: jni::sys::jlong,
) {
    if ptr != 0 && (ptr & 1) == 0 {
        drop(Box::from_raw(ptr as *mut zenoh_flat::TimestampStack));
    }
}
const _: () = {
    if ::core::mem::align_of::<zenoh_flat::TimestampStack>() < 2 {
        panic!("opaque handle types must have alignment >= 2 (bit 0 is the closed tag)");
    }
};
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingZenohBytes<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_zenoh_bytes())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingZenohBytesId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_zenoh_bytes())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingZenohString<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_zenoh_string())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingZenohStringId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_zenoh_string())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingZenohSerialized<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_zenoh_serialized())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingZenohSerializedId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_zenoh_serialized())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationOctetStream<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_octet_stream())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationOctetStreamId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_octet_stream())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextPlain<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_plain())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextPlainId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_plain())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJson<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_json())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJsonId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_json())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextJson<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_json())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextJsonId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_json())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationCdr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_cdr())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationCdrId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_cdr())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationCbor<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_cbor())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationCborId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_cbor())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationYaml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_yaml())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationYamlId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_yaml())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextYaml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_yaml())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextYamlId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_yaml())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextJson5<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_json5())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextJson5Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_json5())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationPythonSerializedObject<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_python_serialized_object())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationPythonSerializedObjectId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_python_serialized_object())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationProtobuf<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_protobuf())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationProtobufId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_protobuf())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJavaSerializedObject<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_java_serialized_object())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJavaSerializedObjectId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_java_serialized_object())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationOpenmetricsText<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_openmetrics_text())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationOpenmetricsTextId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_openmetrics_text())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImagePng<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_image_png())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImagePngId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_image_png())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageJpeg<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_image_jpeg())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageJpegId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_image_jpeg())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageGif<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_image_gif())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageGifId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_image_gif())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageBmp<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_image_bmp())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageBmpId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_image_bmp())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageWebp<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_image_webp())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingImageWebpId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_image_webp())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationXml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_xml())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationXmlId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_xml())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationXWwwFormUrlencoded<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_x_www_form_urlencoded())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationXWwwFormUrlencodedId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_x_www_form_urlencoded())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextHtml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_html())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextHtmlId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_html())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextXml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_xml())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextXmlId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_xml())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextCss<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_css())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextCssId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_css())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextJavascript<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_javascript())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextJavascriptId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_javascript())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextMarkdown<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_markdown())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextMarkdownId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_markdown())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextCsv<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_text_csv())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingTextCsvId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_text_csv())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationSql<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_sql())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationSqlId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_sql())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationCoapPayload<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_coap_payload())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationCoapPayloadId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_coap_payload())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJsonPatchJson<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_json_patch_json())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJsonPatchJsonId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_json_patch_json())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJsonSeq<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_json_seq())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJsonSeqId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_json_seq())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJsonpath<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_jsonpath())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJsonpathId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_jsonpath())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJwt<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_jwt())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationJwtId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_jwt())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationMp4<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_mp4())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationMp4Id<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_mp4())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationSoapXml<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_soap_xml())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationSoapXmlId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_soap_xml())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationYang<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_application_yang())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingApplicationYangId<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_application_yang())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioAac<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_audio_aac())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioAacId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_audio_aac())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioFlac<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_audio_flac())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioFlacId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_audio_flac())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioMp4<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_audio_mp4())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioMp4Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_audio_mp4())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioOgg<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_audio_ogg())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioOggId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_audio_ogg())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioVorbis<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_audio_vorbis())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingAudioVorbisId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_audio_vorbis())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH261<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_h261())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH261Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_h261())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH263<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_h263())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH263Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_h263())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH264<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_h264())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH264Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_h264())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH265<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_h265())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH265Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_h265())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH266<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_h266())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoH266Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_h266())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoMp4<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_mp4())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoMp4Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_mp4())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoOgg<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_ogg())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoOggId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_ogg())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoRaw<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_raw())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoRawId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_raw())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoVp8<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_vp8())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoVp8Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_vp8())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoVp9<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_to_string(encoding_const_video_vp9())
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_constGetEncodingVideoVp9Id<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = {
        #[allow(unused_imports)]
        use zenoh_flat::*;
        encoding_get_id(encoding_const_video_vp9())
    };
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_AdvancedPublisher_jni_handle_codec_borrow_input_298801d5fc997bb9<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::AdvancedPublisher>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::AdvancedPublisher) })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_AdvancedSubscriber_jni_handle_codec_borrow_input_0931ab6ee88953e1<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::AdvancedSubscriber>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::AdvancedSubscriber) })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Config_jni_handle_codec_borrow_input_77739071df1dbe47<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Config>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Config) })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Encoding_jni_handle_codec_borrow_input_11ed5ebde83c779c<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Encoding>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Encoding) })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Encoding_jni_handle_codec_clone_output_to_wire_ec144ca078ed717e<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::Encoding,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Hello_jni_handle_codec_borrow_input_21e25a9262ca3e93<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Hello>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Hello) })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_KeyExpr_jni_handle_codec_borrow_input_d9700f3a873cda93<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::KeyExpr>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::KeyExpr) })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_KeyExpr_jni_handle_codec_clone_output_to_wire_76ff23f685cd5e3d<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::KeyExpr,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Publisher_jni_handle_codec_borrow_input_de8fa64048befc2f<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Publisher>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Publisher) })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Querier_jni_handle_codec_borrow_input_7d494263029d7b3c<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Querier>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Querier) })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Query_jni_handle_codec_borrow_input_0edb5f83bccc458b<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Query>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Query) })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Reply_jni_handle_codec_borrow_input_7db518929d6a9b8f<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Reply>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Reply) })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_ReplyError_jni_handle_codec_borrow_input_622062d39f49b357<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ReplyError>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ReplyError) })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_ReplyError_jni_handle_codec_clone_output_to_wire_51a28958dc9897d1<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::ReplyError,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Sample_jni_handle_codec_borrow_input_4939cf7fe8217cff<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Sample>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Sample) })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Sample_jni_handle_codec_clone_output_to_wire_d1f5e4a6d0634011<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::Sample,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::Session>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Session) })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_TimestampStack_jni_handle_codec_borrow_input_0b5530c7da6cc169<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::TimestampStack>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::TimestampStack) })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_TimestampStack_jni_handle_codec_clone_output_to_wire_44d7555d3e6ddb13<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::TimestampStack,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_ZBytes_jni_handle_codec_borrow_input_d849d0f26d3372f8<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<OwnedObject<zenoh_flat::ZBytes>, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    Ok(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::ZBytes) })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_ZBytes_jni_handle_codec_clone_output_to_wire_631cd85333755af2<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: &zenoh_flat::ZBytes,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v.clone())) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_in_convert_wire_to_ZenohId_jni_product_intermediate_tuple_e59310f235d3e3e1<
    'env,
    'a,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (jni::objects::JByteArray<'a>,),
) -> ::core::result::Result<zenoh_flat::ZenohId, __JniErr> {
    ::core::result::Result::Ok(zenoh_flat::ZenohId {
        bytes: __jni_in_convert_wire_to_u8_ZENOH_ID_MAX_SIZE_9ba98e7d0e8ec60b(
            env,
            &((v).0),
        )?,
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_ZenohId_826016536a47668b<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<zenoh_flat::ZenohId, __JniErr> {
    Ok({
        let __bytes_jobj: jni::objects::JObject = env
            .get_field(v, "bytes", "[B")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("ZenohId.bytes: {}", e)))?;
        let __bytes_raw: jni::objects::JByteArray = __bytes_jobj.into();
        zenoh_flat::ZenohId {
            bytes: __jni_in_convert_wire_to_u8_ZENOH_ID_MAX_SIZE_9ba98e7d0e8ec60b(
                env,
                &__bytes_raw,
            )?,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JString<'v>,
) -> ::core::result::Result<String, __JniErr> {
    Ok({
        let s = env
            .get_string(v)
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("decode_string: {}", e))
            })?;
        s.into()
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_jni_text_codec_borrowed_to_wire_bc4fe45698de9c2e<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: &str,
) -> ::core::result::Result<jni::objects::JString<'a>, __JniErr> {
    Ok({
        env.new_string(v)
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("encode_str: {}", e))
            })?
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_unit_to_wire_9e1510fd173c1fd6<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: (),
) -> ::core::result::Result<(), __JniErr> {
    Ok(v)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_AdvancedPublisher_jni_handle_codec_own_output_to_wire_2e93ba1588e21f36<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::AdvancedPublisher,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_AdvancedSubscriber_jni_handle_codec_own_output_to_wire_b81d795d1d1470ce<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::AdvancedSubscriber,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_in_convert_wire_to_CacheConfig_jni_product_intermediate_tuple_283f94b284781d72<
    'env,
    'a,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (jni::sys::jlong, (jni::sys::jint, jni::sys::jint, jni::sys::jboolean)),
) -> ::core::result::Result<zenoh_flat::CacheConfig, __JniErr> {
    ::core::result::Result::Ok(zenoh_flat::CacheConfig {
        max_samples: __jni_in_convert_wire_to_u64_8507143745dc33b9(env, &((v).0))?,
        replies_config: __jni_in_convert_wire_to_RepliesConfig_jni_product_intermediate_tuple_956439fbd765eecc(
            env,
            (v).1,
        )?,
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_CacheConfig_cf33c287d7f35ae3<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<zenoh_flat::CacheConfig, __JniErr> {
    Ok({
        let __max_samples_raw: jni::sys::jlong = env
            .get_field(v, "maxSamples", "J")
            .and_then(|val| val.j())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("CacheConfig.maxSamples: {}", e)))?;
        let __replies_config_raw: jni::objects::JObject = env
            .get_field(v, "repliesConfig", "Lio/zenoh/jni/pubsub/RepliesConfig;")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("CacheConfig.repliesConfig: {}", e)))?;
        zenoh_flat::CacheConfig {
            max_samples: __jni_in_convert_wire_to_u64_8507143745dc33b9(
                env,
                &__max_samples_raw,
            )?,
            replies_config: __jni_in_convert_wire_to_RepliesConfig_7a81db0e8d2214ef(
                env,
                &__replies_config_raw,
            )?,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_CacheConfig_to_wire_11c1095825a2d31a<'a>(
    mut env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::CacheConfig,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    Ok({
        let __obj0: jni::sys::jlong = {
            let __enc0 = match __jni_out_convert_u64_to_wire_c9db59f6e5bef648(
                &mut env,
                (&(&v).max_samples).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc0
        };
        let __obj1: jni::sys::jint = {
            let __enc1 = match __jni_out_convert_Priority_to_wire_55b65fa623d4787e(
                &mut env,
                (&(&(&v).replies_config).priority).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc1
        };
        let __obj2: jni::sys::jint = {
            let __enc2 = match __jni_out_convert_CongestionControl_to_wire_dd1f6047005395ca(
                &mut env,
                (&(&(&v).replies_config).congestion_control).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc2
        };
        let __obj3: jni::sys::jboolean = {
            let __enc3 = match __jni_out_convert_bool_to_wire_3ee62077915d5228(
                &mut env,
                (&(&(&v).replies_config).is_express).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc3
        };
        let __obj = env
            .call_static_method(
                "io/zenoh/jni/pubsub/CacheConfig",
                "fromParts",
                "(JIIZ)Lio/zenoh/jni/pubsub/CacheConfig;",
                &[
                    jni::objects::JValue::from(__obj0),
                    jni::objects::JValue::from(__obj1),
                    jni::objects::JValue::from(__obj2),
                    jni::objects::JValue::from(__obj3),
                ],
            )
            .and_then(|__v| __v.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("encode struct via fromParts: {}", e)))?;
        __obj
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Config_jni_handle_codec_consume_input_76dd1a8cf1a45c64<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<zenoh_flat::Config, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    ::core::result::Result::Ok(unsafe {
        *::std::boxed::Box::from_raw(*v as *mut zenoh_flat::Config)
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Config_jni_handle_codec_own_output_to_wire_3d8eeac173220d60<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Config,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_CongestionControl_f81af10c6d71d5f3<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::CongestionControl, __JniErr> {
    Ok({
        match *v as i64 {
            0 => zenoh_flat::CongestionControl::Drop,
            1 => zenoh_flat::CongestionControl::Block,
            2 => zenoh_flat::CongestionControl::BlockFirst,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!(
                            "invalid {} discriminant: {}", "CongestionControl", other
                        ),
                    ),
                );
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_CongestionControl_to_wire_dd1f6047005395ca<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::CongestionControl,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_ConsolidationMode_04151c85ad00ec2f<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::ConsolidationMode, __JniErr> {
    Ok({
        match *v as i64 {
            0 => zenoh_flat::ConsolidationMode::Auto,
            1 => zenoh_flat::ConsolidationMode::None,
            2 => zenoh_flat::ConsolidationMode::Monotonic,
            3 => zenoh_flat::ConsolidationMode::Latest,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!(
                            "invalid {} discriminant: {}", "ConsolidationMode", other
                        ),
                    ),
                );
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_ConsolidationMode_to_wire_9a61a4460f925f36<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ConsolidationMode,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Cow_u8_to_wire_eafa10ed25b05dd5<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: ::std::borrow::Cow<'_, [u8]>,
) -> ::core::result::Result<jni::objects::JByteArray<'a>, __JniErr> {
    Ok({
        env.byte_array_from_slice(&v)
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("encode_byte_array: {}", e))
            })?
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_u64_8507143745dc33b9<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<u64, __JniErr> {
    Ok(*v as ::core::primitive::u64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_stage_0_wire_to_Duration_814bb872b19d3627<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: u64,
) -> ::core::result::Result<zenoh_flat::Duration, __JniErr> {
    {
        if (true && true && (v) <= 18446744073709551614u64) && !(false) {
            ::core::result::Result::Ok(crate::duration_from_millis(v))
        } else {
            ::core::result::Result::Err(
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(
                    format!(
                        "{} representation is outside its declared domain", "Duration",
                    ),
                ),
            )
        }
    }
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_u64_to_wire_c9db59f6e5bef648<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: u64,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(v as jni::sys::jlong)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_stage_0_Duration_to_wire_37da00112022eac0<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Duration,
) -> ::core::result::Result<u64, __JniErr> {
    {
        match (crate::duration_to_millis(v))
            .map_err(|__e| {
                <__JniErr as ::core::convert::From<String>>::from(__e.to_string())
            })
        {
            ::core::result::Result::Ok(
                __repr,
            ) if (true && true && (__repr) <= 18446744073709551614u64) && !(false) => {
                ::core::result::Result::Ok(__repr)
            }
            ::core::result::Result::Ok(_) => {
                ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!(
                            "{} representation is outside its declared domain",
                            "Duration",
                        ),
                    ),
                )
            }
            ::core::result::Result::Err(__e) => ::core::result::Result::Err(__e),
        }
    }
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Encoding_jni_handle_codec_own_output_to_wire_832b680de1995a2b<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Encoding,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_EntityGlobalId_5c2de07f2d46bb63<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<zenoh_flat::EntityGlobalId, __JniErr> {
    Ok({
        let __zid_raw: jni::objects::JObject = env
            .get_field(v, "zid", "Lio/zenoh/jni/config/ZenohId;")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("EntityGlobalId.zid: {}", e)))?;
        let __eid_raw: jni::sys::jlong = env
            .get_field(v, "eid", "J")
            .and_then(|val| val.j())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("EntityGlobalId.eid: {}", e)))? as _;
        zenoh_flat::EntityGlobalId {
            zid: __jni_in_convert_wire_to_ZenohId_826016536a47668b(env, &__zid_raw)?,
            eid: __jni_in_convert_wire_to_u32_25dff6d476799035(env, &__eid_raw)?,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_out_convert_EntityGlobalId_jni_product_intermediate_tuple_to_wire_36316835bdf189ae<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::EntityGlobalId,
) -> ::core::result::Result<
    ((jni::objects::JByteArray<'a>,), jni::sys::jlong),
    __JniErr,
> {
    ::core::result::Result::Ok((
        __jni_out_convert_ZenohId_jni_product_intermediate_tuple_to_wire_c7bb0eb9058fd8ec(
            env,
            v.zid,
        )?,
        __jni_out_convert_u32_to_wire_b6376ae826304960(env, v.eid)?,
    ))
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_EntityGlobalId_to_wire_c471a4ba4053e110<'a>(
    mut env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::EntityGlobalId,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    Ok({
        let __obj0: jni::objects::JObject = {
            let __enc0 = match __jni_out_convert_u8_ZENOH_ID_MAX_SIZE_to_wire_7574f9b51f0f53e4(
                &mut env,
                (&(&(&v).zid).bytes).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc0.into()
        };
        let __obj1: jni::sys::jlong = {
            let __enc1 = match __jni_out_convert_u32_to_wire_b6376ae826304960(
                &mut env,
                (&(&v).eid).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc1
        };
        let __obj = env
            .call_static_method(
                "io/zenoh/jni/pubsub/EntityGlobalId",
                "fromParts",
                "([BJ)Lio/zenoh/jni/pubsub/EntityGlobalId;",
                &[
                    jni::objects::JValue::Object(&__obj0),
                    jni::objects::JValue::from(__obj1),
                ],
            )
            .and_then(|__v| __v.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("encode struct via fromParts: {}", e)))?;
        __obj
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_in_convert_wire_to_HistoryConfig_jni_product_intermediate_tuple_1d3a0c39c8a32bd8<
    'env,
    'a,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (
        jni::sys::jboolean,
        (jni::sys::jboolean, jni::sys::jlong),
        (jni::sys::jboolean, jni::sys::jdouble),
    ),
) -> ::core::result::Result<zenoh_flat::HistoryConfig, __JniErr> {
    ::core::result::Result::Ok(zenoh_flat::HistoryConfig {
        detect_late_publishers: __jni_in_convert_wire_to_bool_1be2f6c32f925207(
            env,
            &((v).0),
        )?,
        max_samples: __jni_in_convert_wire_to_Option_u64_jni_optional_intermediate_input_gated_914197c318e30b0e(
            env,
            (v).1,
        )?,
        max_age: __jni_in_convert_wire_to_Option_f64_jni_optional_intermediate_input_gated_10d98a298d62e1f5(
            env,
            (v).2,
        )?,
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_HistoryConfig_f6d85baa438b7d77<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<zenoh_flat::HistoryConfig, __JniErr> {
    Ok({
        let __detect_late_publishers_raw: jni::sys::jboolean = env
            .get_field(v, "detectLatePublishers", "Z")
            .and_then(|val| val.z())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("HistoryConfig.detectLatePublishers: {}", e)))? as _;
        let __max_samples_jobj: jni::objects::JObject = env
            .get_field(v, "maxSamples", "Lkotlin/ULong;")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("HistoryConfig.maxSamples: {}", e)))?;
        let __max_age_raw: jni::objects::JObject = env
            .get_field(v, "maxAge", "Ljava/lang/Double;")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("HistoryConfig.maxAge: {}", e)))?;
        zenoh_flat::HistoryConfig {
            detect_late_publishers: __jni_in_convert_wire_to_bool_1be2f6c32f925207(
                env,
                &__detect_late_publishers_raw,
            )?,
            max_samples: {
                let v = __max_samples_jobj;
                {
                    if v.is_null() {
                        ::core::option::Option::None
                    } else {
                        let __present = env
                            .call_method(&v, "unbox-impl", "()J", &[])
                            .and_then(|val| val.j())
                            .map_err(|e| <__JniErr as ::core::convert::From<
                                String,
                            >>::from(format!("HistoryConfig.maxSamples: {}", e)))?;
                        ::core::option::Option::Some(
                            __jni_in_convert_wire_to_u64_8507143745dc33b9(
                                env,
                                &__present,
                            )?,
                        )
                    }
                }
            },
            max_age: __jni_in_convert_wire_to_Option_f64_jni_optional_intermediate_input_boxed_63b0bb7e264e4a74(
                env,
                &__max_age_raw,
            )?,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_HistoryConfig_to_wire_8cd703df2038494e<'a>(
    mut env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::HistoryConfig,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    Ok({
        let __obj0: jni::sys::jboolean = {
            let __enc0 = match __jni_out_convert_bool_to_wire_3ee62077915d5228(
                &mut env,
                (&(&v).detect_late_publishers).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc0
        };
        let __obj1: jni::objects::JObject = {
            let __enc1 = match __jni_out_convert_Option_u64_jni_optional_intermediate_output_boxed_to_wire_0d6fbbfea0a177f9(
                &mut env,
                (&(&v).max_samples).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc1
        };
        let __obj2: jni::objects::JObject = {
            let __enc2 = match __jni_out_convert_Option_f64_jni_optional_intermediate_output_boxed_to_wire_980d49a7b3b2f2ae(
                &mut env,
                (&(&v).max_age).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc2
        };
        let __obj = env
            .call_static_method(
                "io/zenoh/jni/pubsub/HistoryConfig",
                "fromParts",
                "(ZLjava/lang/Long;Ljava/lang/Double;)Lio/zenoh/jni/pubsub/HistoryConfig;",
                &[
                    jni::objects::JValue::from(__obj0),
                    jni::objects::JValue::Object(&__obj1),
                    jni::objects::JValue::Object(&__obj2),
                ],
            )
            .and_then(|__v| __v.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("encode struct via fromParts: {}", e)))?;
        __obj
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_InstrumentationTimestamp_7254d25ac1aeaa71<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<zenoh_flat::InstrumentationTimestamp, __JniErr> {
    Ok({
        let __tag = {
            if v.is_null() {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        "InstrumentationTimestamp: null value where a variant was required"
                            .to_string(),
                    ),
                );
            }
            let __tag = (|| -> ::core::result::Result<i32, __JniErr> {
                if env
                    .is_instance_of(v, "io/zenoh/jni/time/InstrumentationTimestamp$Uhlc")
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!(
                            "InstrumentationTimestamp: instanceof io/zenoh/jni/time/InstrumentationTimestamp$Uhlc: {}",
                            e
                        ),
                    ))?
                {
                    return ::core::result::Result::Ok(0i32);
                }
                if env
                    .is_instance_of(
                        v,
                        "io/zenoh/jni/time/InstrumentationTimestamp$Custom",
                    )
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!(
                            "InstrumentationTimestamp: instanceof io/zenoh/jni/time/InstrumentationTimestamp$Custom: {}",
                            e
                        ),
                    ))?
                {
                    return ::core::result::Result::Ok(1i32);
                }
                ::core::result::Result::Ok(-1i32)
            })()?;
            __tag
        };
        match __tag {
            0i32 => {
                let __choice = v;
                let __arm = __choice;
                let __p_v0_raw: jni::objects::JObject = env
                    .get_field(__arm, "v0", "Lio/zenoh/jni/time/Timestamp;")
                    .and_then(|val| val.l())
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("InstrumentationTimestamp.Uhlc.v0: {}", e)))?;
                zenoh_flat::InstrumentationTimestamp::Uhlc(
                    __jni_in_convert_wire_to_Timestamp_37bfb116800bb25b(
                        env,
                        &__p_v0_raw,
                    )?,
                )
            }
            1i32 => {
                let __choice = v;
                let __arm = __choice;
                let __p_v0_obj: jni::objects::JObject = env
                    .get_field(__arm, "v0", "[B")
                    .and_then(|val| val.l())
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("InstrumentationTimestamp.Custom.v0: {}", e)))?;
                let __p_v0_raw: jni::objects::JByteArray = __p_v0_obj.into();
                zenoh_flat::InstrumentationTimestamp::Custom(
                    __jni_in_convert_wire_to_Vec_u8_80984e9556387695(env, &__p_v0_raw)?,
                )
            }
            _ => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        "InstrumentationTimestamp: value is not one of its declared variants"
                            .to_string(),
                    ),
                );
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_out_convert_InstrumentationTimestamp_jni_choice_intermediate_tagged_tuple_to_wire_cdbd34f354ae1eb6<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::InstrumentationTimestamp,
) -> ::core::result::Result<
    (jni::sys::jint, (jni::objects::JObject<'a>,), (jni::objects::JByteArray<'a>,)),
    __JniErr,
> {
    ::core::result::Result::Ok({
        match v {
            zenoh_flat::InstrumentationTimestamp::Uhlc(__part0) => {
                (
                    0i32,
                    (
                        __jni_out_convert_Timestamp_to_wire_66f4b22dc6f3c572(
                            env,
                            __part0,
                        )?,
                    ),
                    (jni::objects::JObject::null().into(),),
                )
            }
            zenoh_flat::InstrumentationTimestamp::Custom(__part0) => {
                (
                    1i32,
                    (jni::objects::JObject::null().into(),),
                    (__jni_out_convert_Vec_u8_to_wire_e9499bf0a706b1a2(env, __part0)?,),
                )
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_InterceptionPoint_f245d29a73d8add3<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::InterceptionPoint, __JniErr> {
    Ok({
        match *v as i64 {
            0 => zenoh_flat::InterceptionPoint::Send,
            1 => zenoh_flat::InterceptionPoint::Route,
            2 => zenoh_flat::InterceptionPoint::Receive,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!(
                            "invalid {} discriminant: {}", "InterceptionPoint", other
                        ),
                    ),
                );
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_InterceptionPoint_to_wire_cd6b109f4fdb1f8a<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::InterceptionPoint,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_KeyExpr_jni_handle_codec_consume_input_7fcb26b92cb14c18<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<zenoh_flat::KeyExpr, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    ::core::result::Result::Ok(unsafe {
        *::std::boxed::Box::from_raw(*v as *mut zenoh_flat::KeyExpr)
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_KeyExpr_jni_handle_codec_own_output_to_wire_2022bfd454e7e1bc<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::KeyExpr,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_LivelinessToken_jni_handle_codec_own_output_to_wire_9a3ca92ce7f5264b<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::LivelinessToken,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_MatchingListener_jni_handle_codec_own_output_to_wire_7ca5e1ac9818f8d1<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::MatchingListener,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Miss_ec6ca4a03a1450c5<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<zenoh_flat::Miss, __JniErr> {
    Ok({
        let __source_raw: jni::objects::JObject = env
            .get_field(v, "source", "Lio/zenoh/jni/pubsub/EntityGlobalId;")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Miss.source: {}", e)))?;
        let __nb_raw: jni::sys::jlong = env
            .get_field(v, "nb", "J")
            .and_then(|val| val.j())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Miss.nb: {}", e)))? as _;
        zenoh_flat::Miss {
            source: __jni_in_convert_wire_to_EntityGlobalId_5c2de07f2d46bb63(
                env,
                &__source_raw,
            )?,
            nb: __jni_in_convert_wire_to_u32_25dff6d476799035(env, &__nb_raw)?,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_out_convert_Miss_jni_product_intermediate_tuple_to_wire_c9ab00e074005f65<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Miss,
) -> ::core::result::Result<
    (((jni::objects::JByteArray<'a>,), jni::sys::jlong), jni::sys::jlong),
    __JniErr,
> {
    ::core::result::Result::Ok((
        __jni_out_convert_EntityGlobalId_jni_product_intermediate_tuple_to_wire_36316835bdf189ae(
            env,
            v.source,
        )?,
        __jni_out_convert_u32_to_wire_b6376ae826304960(env, v.nb)?,
    ))
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Miss_to_wire_78c8de523c586962<'a>(
    mut env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Miss,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    Ok({
        let __obj0: jni::objects::JObject = {
            let __enc0 = match __jni_out_convert_u8_ZENOH_ID_MAX_SIZE_to_wire_7574f9b51f0f53e4(
                &mut env,
                (&(&(&(&v).source).zid).bytes).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc0.into()
        };
        let __obj1: jni::sys::jlong = {
            let __enc1 = match __jni_out_convert_u32_to_wire_b6376ae826304960(
                &mut env,
                (&(&(&v).source).eid).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc1
        };
        let __obj2: jni::sys::jlong = {
            let __enc2 = match __jni_out_convert_u32_to_wire_b6376ae826304960(
                &mut env,
                (&(&v).nb).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc2
        };
        let __obj = env
            .call_static_method(
                "io/zenoh/jni/pubsub/Miss",
                "fromParts",
                "([BJJ)Lio/zenoh/jni/pubsub/Miss;",
                &[
                    jni::objects::JValue::Object(&__obj0),
                    jni::objects::JValue::from(__obj1),
                    jni::objects::JValue::from(__obj2),
                ],
            )
            .and_then(|__v| __v.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("encode struct via fromParts: {}", e)))?;
        __obj
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_in_convert_wire_to_MissDetectionConfig_jni_product_intermediate_tuple_cbd0c7174840b797<
    'env,
    'a,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (jni::sys::jlong, jni::sys::jboolean),
) -> ::core::result::Result<zenoh_flat::MissDetectionConfig, __JniErr> {
    ::core::result::Result::Ok(zenoh_flat::MissDetectionConfig {
        heartbeat: __jni_in_convert_wire_to_Option_Duration_jni_optional_intermediate_input_niche_104bdfd3431d40b9(
            env,
            &((v).0),
        )?,
        sporadic: __jni_in_convert_wire_to_bool_1be2f6c32f925207(env, &((v).1))?,
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_MissDetectionConfig_c56167689b5cbd8b<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<zenoh_flat::MissDetectionConfig, __JniErr> {
    Ok({
        let __heartbeat_jobj: jni::objects::JObject = env
            .get_field(v, "heartbeat", "Lkotlin/ULong;")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("MissDetectionConfig.heartbeat: {}", e)))?;
        let __sporadic_raw: jni::sys::jboolean = env
            .get_field(v, "sporadic", "Z")
            .and_then(|val| val.z())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("MissDetectionConfig.sporadic: {}", e)))? as _;
        zenoh_flat::MissDetectionConfig {
            heartbeat: {
                let v = __heartbeat_jobj;
                {
                    if v.is_null() {
                        ::core::option::Option::None
                    } else {
                        let __present = env
                            .call_method(&v, "unbox-impl", "()J", &[])
                            .and_then(|val| val.j())
                            .map_err(|e| <__JniErr as ::core::convert::From<
                                String,
                            >>::from(format!("MissDetectionConfig.heartbeat: {}", e)))?;
                        __jni_in_convert_wire_to_Option_Duration_jni_optional_intermediate_input_niche_104bdfd3431d40b9(
                            env,
                            &__present,
                        )?
                    }
                }
            },
            sporadic: __jni_in_convert_wire_to_bool_1be2f6c32f925207(
                env,
                &__sporadic_raw,
            )?,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_MissDetectionConfig_to_wire_57b0a5de6c0f893a<'a>(
    mut env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::MissDetectionConfig,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    Ok({
        let __obj0: jni::sys::jlong = {
            let __enc0 = match __jni_out_convert_Option_Duration_jni_optional_intermediate_output_niche_to_wire_dc11bc4c611f2997(
                &mut env,
                (&(&v).heartbeat).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc0
        };
        let __obj1: jni::sys::jboolean = {
            let __enc1 = match __jni_out_convert_bool_to_wire_3ee62077915d5228(
                &mut env,
                (&(&v).sporadic).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc1
        };
        let __obj = env
            .call_static_method(
                "io/zenoh/jni/pubsub/MissDetectionConfig",
                "fromParts",
                "(JZ)Lio/zenoh/jni/pubsub/MissDetectionConfig;",
                &[jni::objects::JValue::from(__obj0), jni::objects::JValue::from(__obj1)],
            )
            .and_then(|__v| __v.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("encode struct via fromParts: {}", e)))?;
        __obj
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_Config_3d9d8ed7ea1ee21b<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<Option<OwnedObject<zenoh_flat::Config>>, __JniErr> {
    if *v == 0 {
        Ok(None)
    } else if (*v & 1) == 1 {
        Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        )
    } else {
        Ok(Some(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Config) }))
    }
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_Encoding_1ea8d6cdb046d07b<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<Option<OwnedObject<zenoh_flat::Encoding>>, __JniErr> {
    if *v == 0 {
        Ok(None)
    } else if (*v & 1) == 1 {
        Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        )
    } else {
        Ok(Some(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::Encoding) }))
    }
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_Encoding_jni_optional_intermediate_output_niche_to_wire_d5cdae9fd96149aa<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<&zenoh_flat::Encoding>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                __jni_out_convert_Encoding_jni_handle_codec_clone_output_to_wire_ec144ca078ed717e(
                    env,
                    __value,
                )?
            }
            ::core::option::Option::None => 0i64,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_KeyExpr_75b6fc876caf57d7<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<Option<OwnedObject<zenoh_flat::KeyExpr>>, __JniErr> {
    if *v == 0 {
        Ok(None)
    } else if (*v & 1) == 1 {
        Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        )
    } else {
        Ok(Some(unsafe { OwnedObject::from_raw(*v as *const zenoh_flat::KeyExpr) }))
    }
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_ReplyError_jni_optional_intermediate_output_niche_to_wire_bc4aafd0982f3eab<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<&zenoh_flat::ReplyError>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                __jni_out_convert_ReplyError_jni_handle_codec_clone_output_to_wire_51a28958dc9897d1(
                    env,
                    __value,
                )?
            }
            ::core::option::Option::None => 0i64,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_Sample_jni_optional_intermediate_output_niche_to_wire_c6400b29b5a421b3<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<&zenoh_flat::Sample>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                __jni_out_convert_Sample_jni_handle_codec_clone_output_to_wire_d1f5e4a6d0634011(
                    env,
                    __value,
                )?
            }
            ::core::option::Option::None => 0i64,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_TimestampStack_jni_optional_intermediate_output_niche_to_wire_9e52e4193d223461<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<&zenoh_flat::TimestampStack>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                __jni_out_convert_TimestampStack_jni_handle_codec_clone_output_to_wire_44d7555d3e6ddb13(
                    env,
                    __value,
                )?
            }
            ::core::option::Option::None => 0i64,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_ZBytes_jni_optional_intermediate_output_niche_to_wire_1562c928235871aa<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<&zenoh_flat::ZBytes>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                __jni_out_convert_ZBytes_jni_handle_codec_clone_output_to_wire_631cd85333755af2(
                    env,
                    __value,
                )?
            }
            ::core::option::Option::None => 0i64,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_CacheConfig_jni_optional_intermediate_input_gated_db098990eea705d9<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (
        jni::sys::jboolean,
        (jni::sys::jlong, (jni::sys::jint, jni::sys::jint, jni::sys::jboolean)),
    ),
) -> ::core::result::Result<::core::option::Option<zenoh_flat::CacheConfig>, __JniErr> {
    ::core::result::Result::Ok({
        if (v).0 == 0u8 {
            ::core::option::Option::None
        } else {
            let __present = (v).1;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_CacheConfig_jni_product_intermediate_tuple_283f94b284781d72(
                    env,
                    __present,
                )?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_CongestionControl_jni_optional_intermediate_input_niche_b0f8f28507150661<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<
    ::core::option::Option<zenoh_flat::CongestionControl>,
    __JniErr,
> {
    ::core::result::Result::Ok({
        if *v == -2147483648i32 {
            ::core::option::Option::None
        } else {
            let __present = v;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_CongestionControl_f81af10c6d71d5f3(
                    env,
                    __present,
                )?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_ConsolidationMode_jni_optional_intermediate_input_niche_cbd6120cd28a09b0<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<
    ::core::option::Option<zenoh_flat::ConsolidationMode>,
    __JniErr,
> {
    ::core::result::Result::Ok({
        if *v == -2147483648i32 {
            ::core::option::Option::None
        } else {
            let __present = v;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_ConsolidationMode_04151c85ad00ec2f(
                    env,
                    __present,
                )?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_Duration_jni_optional_intermediate_input_niche_104bdfd3431d40b9<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<::core::option::Option<zenoh_flat::Duration>, __JniErr> {
    ::core::result::Result::Ok({
        if *v == -1i64 {
            ::core::option::Option::None
        } else {
            let __present = v;
            ::core::option::Option::Some(
                {
                    let __chain_s0 = __jni_in_convert_wire_to_u64_8507143745dc33b9(
                        env,
                        __present,
                    )?;
                    let __chain_s1 = __jni_in_stage_0_wire_to_Duration_814bb872b19d3627(
                            env,
                            __chain_s0,
                        )
                        .map_err(|__e| <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()))?;
                    ::core::result::Result::<_, __JniErr>::Ok(__chain_s1)
                }?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_Duration_jni_optional_intermediate_output_niche_to_wire_dc11bc4c611f2997<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<zenoh_flat::Duration>,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                {
                    let __chain_s0 = __jni_out_stage_0_Duration_to_wire_37da00112022eac0(
                            env,
                            __value,
                        )
                        .map_err(|__e| <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()))?;
                    __jni_out_convert_u64_to_wire_c9db59f6e5bef648(env, __chain_s0)
                }?
            }
            ::core::option::Option::None => -1i64,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_EntityGlobalId_jni_optional_intermediate_output_gated_to_wire_dd03e8279dedf2b8<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<zenoh_flat::EntityGlobalId>,
) -> ::core::result::Result<
    (jni::sys::jboolean, ((jni::objects::JByteArray<'a>,), jni::sys::jlong)),
    __JniErr,
> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                (
                    1u8,
                    __jni_out_convert_EntityGlobalId_jni_product_intermediate_tuple_to_wire_36316835bdf189ae(
                        env,
                        __value,
                    )?,
                )
            }
            ::core::option::Option::None => {
                (0u8, ((jni::objects::JObject::null().into(),), 0 as jni::sys::jlong))
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_EntityGlobalId_jni_optional_intermediate_output_niche_to_wire_be8cdb5085ebce04<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<zenoh_flat::EntityGlobalId>,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                __jni_out_convert_EntityGlobalId_to_wire_c471a4ba4053e110(env, __value)?
            }
            ::core::option::Option::None => jni::objects::JObject::null().into(),
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_HistoryConfig_jni_optional_intermediate_input_gated_9a07e6b3656908cb<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (
        jni::sys::jboolean,
        (
            jni::sys::jboolean,
            (jni::sys::jboolean, jni::sys::jlong),
            (jni::sys::jboolean, jni::sys::jdouble),
        ),
    ),
) -> ::core::result::Result<
    ::core::option::Option<zenoh_flat::HistoryConfig>,
    __JniErr,
> {
    ::core::result::Result::Ok({
        if (v).0 == 0u8 {
            ::core::option::Option::None
        } else {
            let __present = (v).1;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_HistoryConfig_jni_product_intermediate_tuple_1d3a0c39c8a32bd8(
                    env,
                    __present,
                )?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_KeyExpr_jni_optional_intermediate_input_niche_bbf13322cceb1d7d<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<::core::option::Option<zenoh_flat::KeyExpr>, __JniErr> {
    ::core::result::Result::Ok({
        if *v == 0 {
            ::core::option::Option::None
        } else {
            let __present = v;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_KeyExpr_jni_handle_codec_consume_input_7fcb26b92cb14c18(
                    env,
                    __present,
                )?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_MissDetectionConfig_jni_optional_intermediate_input_gated_d4115213f6ae9aaa<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (jni::sys::jboolean, (jni::sys::jlong, jni::sys::jboolean)),
) -> ::core::result::Result<
    ::core::option::Option<zenoh_flat::MissDetectionConfig>,
    __JniErr,
> {
    ::core::result::Result::Ok({
        if (v).0 == 0u8 {
            ::core::option::Option::None
        } else {
            let __present = (v).1;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_MissDetectionConfig_jni_product_intermediate_tuple_cbd0c7174840b797(
                    env,
                    __present,
                )?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_Priority_jni_optional_intermediate_input_niche_6f17c8de5a824021<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<::core::option::Option<zenoh_flat::Priority>, __JniErr> {
    ::core::result::Result::Ok({
        if *v == -2147483648i32 {
            ::core::option::Option::None
        } else {
            let __present = v;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_Priority_2f3eb78aa92f8bfd(env, __present)?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_QueryTarget_jni_optional_intermediate_input_niche_5f89727c78ac796e<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<::core::option::Option<zenoh_flat::QueryTarget>, __JniErr> {
    ::core::result::Result::Ok({
        if *v == -2147483648i32 {
            ::core::option::Option::None
        } else {
            let __present = v;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_QueryTarget_3f60976d959dff1b(env, __present)?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_RecoveryConfig_jni_optional_intermediate_input_gated_eff50792a3d16136<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (jni::sys::jboolean, (jni::objects::JObject<'v>, jni::sys::jlong)),
) -> ::core::result::Result<
    ::core::option::Option<zenoh_flat::RecoveryConfig>,
    __JniErr,
> {
    ::core::result::Result::Ok({
        if (v).0 == 0u8 {
            ::core::option::Option::None
        } else {
            let __present = (v).1;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_RecoveryConfig_jni_product_intermediate_tuple_36248bc1e1ce1eef(
                    env,
                    __present,
                )?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_RecoveryMode_jni_optional_intermediate_input_niche_f50c2860cde17f79<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<::core::option::Option<zenoh_flat::RecoveryMode>, __JniErr> {
    ::core::result::Result::Ok({
        if v.is_null() {
            ::core::option::Option::None
        } else {
            let __present = v;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_RecoveryMode_b4d38f00e9ec3191(env, __present)?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_Reliability_jni_optional_intermediate_input_niche_5451244c94aef911<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<::core::option::Option<zenoh_flat::Reliability>, __JniErr> {
    ::core::result::Result::Ok({
        if *v == -2147483648i32 {
            ::core::option::Option::None
        } else {
            let __present = v;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_Reliability_44f15ad2b611920f(env, __present)?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_ReplyKeyExpr_jni_optional_intermediate_input_niche_e3c3d07c93406d91<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<::core::option::Option<zenoh_flat::ReplyKeyExpr>, __JniErr> {
    ::core::result::Result::Ok({
        if *v == -2147483648i32 {
            ::core::option::Option::None
        } else {
            let __present = v;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_ReplyKeyExpr_66de709ea94f3a25(env, __present)?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_SourceInfo_jni_optional_intermediate_output_gated_to_wire_e84416f8e9b768c0<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<zenoh_flat::SourceInfo>,
) -> ::core::result::Result<
    (
        jni::sys::jboolean,
        (((jni::objects::JByteArray<'a>,), jni::sys::jlong), jni::sys::jlong),
    ),
    __JniErr,
> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                (
                    1u8,
                    __jni_out_convert_SourceInfo_jni_product_intermediate_tuple_to_wire_edc2ee9a82e917fa(
                        env,
                        __value,
                    )?,
                )
            }
            ::core::option::Option::None => {
                (
                    0u8,
                    (
                        ((jni::objects::JObject::null().into(),), 0 as jni::sys::jlong),
                        0 as jni::sys::jlong,
                    ),
                )
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_SourceInfo_jni_optional_intermediate_output_niche_to_wire_c31f0eb6b4d779fc<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<zenoh_flat::SourceInfo>,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                __jni_out_convert_SourceInfo_to_wire_ec86c882a01214d4(env, __value)?
            }
            ::core::option::Option::None => jni::objects::JObject::null().into(),
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JString<'v>,
) -> ::core::result::Result<::core::option::Option<::std::string::String>, __JniErr> {
    ::core::result::Result::Ok({
        if v.is_null() {
            ::core::option::Option::None
        } else {
            let __present = v;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
                    env,
                    __present,
                )?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_String_jni_optional_intermediate_output_niche_to_wire_72d3a6d24cbddfc8<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<::std::string::String>,
) -> ::core::result::Result<jni::objects::JString<'a>, __JniErr> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    env,
                    __value,
                )?
            }
            ::core::option::Option::None => jni::objects::JObject::null().into(),
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_Timestamp_jni_optional_intermediate_input_gated_0e1cf2a3b10b443d<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (jni::sys::jboolean, (jni::sys::jlong, jni::objects::JByteArray<'v>)),
) -> ::core::result::Result<::core::option::Option<zenoh_flat::Timestamp>, __JniErr> {
    ::core::result::Result::Ok({
        if (v).0 == 0u8 {
            ::core::option::Option::None
        } else {
            let __present = (v).1;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_Timestamp_jni_product_intermediate_tuple_8da7af64ea169b8e(
                    env,
                    __present,
                )?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_Timestamp_jni_optional_intermediate_output_gated_to_wire_20caf618dda4bcb7<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<zenoh_flat::Timestamp>,
) -> ::core::result::Result<
    (jni::sys::jboolean, (jni::sys::jlong, jni::objects::JByteArray<'a>)),
    __JniErr,
> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                (
                    1u8,
                    __jni_out_convert_Timestamp_jni_product_intermediate_tuple_to_wire_e28de6376a8e24b5(
                        env,
                        __value,
                    )?,
                )
            }
            ::core::option::Option::None => {
                (0u8, (0 as jni::sys::jlong, jni::objects::JObject::null().into()))
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_Timestamp_jni_optional_intermediate_output_niche_to_wire_744022bc4edbcd87<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<zenoh_flat::Timestamp>,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                __jni_out_convert_Timestamp_to_wire_66f4b22dc6f3c572(env, __value)?
            }
            ::core::option::Option::None => jni::objects::JObject::null().into(),
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JByteArray<'v>,
) -> ::core::result::Result<::core::option::Option<::std::vec::Vec<u8>>, __JniErr> {
    ::core::result::Result::Ok({
        if v.is_null() {
            ::core::option::Option::None
        } else {
            let __present = v;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_Vec_u8_80984e9556387695(env, __present)?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_Vec_u8_jni_optional_intermediate_output_niche_to_wire_beaf597ddc121688<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<::std::vec::Vec<u8>>,
) -> ::core::result::Result<jni::objects::JByteArray<'a>, __JniErr> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                __jni_out_convert_Vec_u8_to_wire_e9499bf0a706b1a2(env, __value)?
            }
            ::core::option::Option::None => jni::objects::JObject::null().into(),
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (jni::sys::jboolean, jni::sys::jboolean),
) -> ::core::result::Result<::core::option::Option<bool>, __JniErr> {
    ::core::result::Result::Ok({
        if (v).0 == 0u8 {
            ::core::option::Option::None
        } else {
            let __present = (v).1;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_bool_1be2f6c32f925207(env, &__present)?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_f64_jni_optional_intermediate_input_gated_10d98a298d62e1f5<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (jni::sys::jboolean, jni::sys::jdouble),
) -> ::core::result::Result<::core::option::Option<f64>, __JniErr> {
    ::core::result::Result::Ok({
        if (v).0 == 0u8 {
            ::core::option::Option::None
        } else {
            let __present = (v).1;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_f64_b312e1b95182cdfd(env, &__present)?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_f64_jni_optional_intermediate_input_boxed_63b0bb7e264e4a74<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<::core::option::Option<f64>, __JniErr> {
    ::core::result::Result::Ok({
        if (v).is_null() {
            ::core::option::Option::None
        } else {
            let __present = {
                env.call_method(&v, "doubleValue", "()D", &[])
                    .and_then(|__value| __value.d())
                    .map(|__value| __value as jni::sys::jdouble)
                    .map_err(|__error| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Option unbox: {}", __error)))?
            };
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_f64_b312e1b95182cdfd(env, &__present)?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_f64_jni_optional_intermediate_output_boxed_to_wire_980d49a7b3b2f2ae<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<f64>,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                let __raw: jni::sys::jdouble = __jni_out_convert_f64_to_wire_61461de12ea6bc04(
                    env,
                    __value,
                )?;
                ::prebindgen_jni_runtime::box_jdouble(env, __raw)
                    .map_err(|__error| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Option box: {}", __error)))?
            }
            ::core::option::Option::None => jni::objects::JObject::null(),
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_i64_jni_optional_intermediate_input_gated_89d088af3f259362<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (jni::sys::jboolean, jni::sys::jlong),
) -> ::core::result::Result<::core::option::Option<i64>, __JniErr> {
    ::core::result::Result::Ok({
        if (v).0 == 0u8 {
            ::core::option::Option::None
        } else {
            let __present = (v).1;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_i64_da07d745d9e26f71(env, &__present)?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_u16_jni_optional_intermediate_input_gated_ff09dbf66b457369<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (jni::sys::jboolean, jni::sys::jint),
) -> ::core::result::Result<::core::option::Option<u16>, __JniErr> {
    ::core::result::Result::Ok({
        if (v).0 == 0u8 {
            ::core::option::Option::None
        } else {
            let __present = (v).1;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_u16_fc24f387ddcec321(env, &__present)?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_u64_jni_optional_intermediate_input_gated_914197c318e30b0e<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (jni::sys::jboolean, jni::sys::jlong),
) -> ::core::result::Result<::core::option::Option<u64>, __JniErr> {
    ::core::result::Result::Ok({
        if (v).0 == 0u8 {
            ::core::option::Option::None
        } else {
            let __present = (v).1;
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_u64_8507143745dc33b9(env, &__present)?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Option_u64_jni_optional_intermediate_input_boxed_c4fe38a27050dd1f<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<::core::option::Option<u64>, __JniErr> {
    ::core::result::Result::Ok({
        if (v).is_null() {
            ::core::option::Option::None
        } else {
            let __present = {
                env.call_method(&v, "longValue", "()J", &[])
                    .and_then(|__value| __value.j())
                    .map(|__value| __value as jni::sys::jlong)
                    .map_err(|__error| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Option unbox: {}", __error)))?
            };
            ::core::option::Option::Some(
                __jni_in_convert_wire_to_u64_8507143745dc33b9(env, &__present)?,
            )
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Option_u64_jni_optional_intermediate_output_boxed_to_wire_0d6fbbfea0a177f9<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: ::core::option::Option<u64>,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    ::core::result::Result::Ok({
        match v {
            ::core::option::Option::Some(__value) => {
                let __raw: jni::sys::jlong = __jni_out_convert_u64_to_wire_c9db59f6e5bef648(
                    env,
                    __value,
                )?;
                ::prebindgen_jni_runtime::box_jlong(env, __raw)
                    .map_err(|__error| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Option box: {}", __error)))?
            }
            ::core::option::Option::None => jni::objects::JObject::null(),
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Priority_2f3eb78aa92f8bfd<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::Priority, __JniErr> {
    Ok({
        match *v as i64 {
            1 => zenoh_flat::Priority::RealTime,
            2 => zenoh_flat::Priority::InteractiveHigh,
            3 => zenoh_flat::Priority::InteractiveLow,
            4 => zenoh_flat::Priority::DataHigh,
            5 => zenoh_flat::Priority::Data,
            6 => zenoh_flat::Priority::DataLow,
            7 => zenoh_flat::Priority::Background,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("invalid {} discriminant: {}", "Priority", other)),
                );
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Priority_to_wire_55b65fa623d4787e<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Priority,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Publisher_jni_handle_codec_own_output_to_wire_94fe05baf6f4cff8<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Publisher,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Querier_jni_handle_codec_own_output_to_wire_26375716f15b424b<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Querier,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Query_jni_handle_codec_own_output_to_wire_59ce6bba5d75d144<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Query,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_QueryTarget_3f60976d959dff1b<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::QueryTarget, __JniErr> {
    Ok({
        match *v as i64 {
            0 => zenoh_flat::QueryTarget::BestMatching,
            1 => zenoh_flat::QueryTarget::All,
            2 => zenoh_flat::QueryTarget::AllComplete,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!("invalid {} discriminant: {}", "QueryTarget", other),
                    ),
                );
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_QueryTarget_to_wire_1322155a0f78cdf2<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::QueryTarget,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Queryable_jni_handle_codec_own_output_to_wire_3d471fba643e8930<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Queryable,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_in_convert_wire_to_RecoveryConfig_jni_product_intermediate_tuple_36248bc1e1ce1eef<
    'env,
    'a,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (jni::objects::JObject<'a>, jni::sys::jlong),
) -> ::core::result::Result<zenoh_flat::RecoveryConfig, __JniErr> {
    ::core::result::Result::Ok(zenoh_flat::RecoveryConfig {
        mode: __jni_in_convert_wire_to_Option_RecoveryMode_jni_optional_intermediate_input_niche_f50c2860cde17f79(
            env,
            &((v).0),
        )?,
        retention_period: __jni_in_convert_wire_to_Option_Duration_jni_optional_intermediate_input_niche_104bdfd3431d40b9(
            env,
            &((v).1),
        )?,
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_RecoveryConfig_1cf458c5987134f3<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<zenoh_flat::RecoveryConfig, __JniErr> {
    Ok({
        let __mode_raw: jni::objects::JObject = env
            .get_field(v, "mode", "Lio/zenoh/jni/pubsub/RecoveryMode;")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("RecoveryConfig.mode: {}", e)))?;
        let __retention_period_jobj: jni::objects::JObject = env
            .get_field(v, "retentionPeriod", "Lkotlin/ULong;")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("RecoveryConfig.retentionPeriod: {}", e)))?;
        zenoh_flat::RecoveryConfig {
            mode: __jni_in_convert_wire_to_Option_RecoveryMode_jni_optional_intermediate_input_niche_f50c2860cde17f79(
                env,
                &__mode_raw,
            )?,
            retention_period: {
                let v = __retention_period_jobj;
                {
                    if v.is_null() {
                        ::core::option::Option::None
                    } else {
                        let __present = env
                            .call_method(&v, "unbox-impl", "()J", &[])
                            .and_then(|val| val.j())
                            .map_err(|e| <__JniErr as ::core::convert::From<
                                String,
                            >>::from(format!("RecoveryConfig.retentionPeriod: {}", e)))?;
                        __jni_in_convert_wire_to_Option_Duration_jni_optional_intermediate_input_niche_104bdfd3431d40b9(
                            env,
                            &__present,
                        )?
                    }
                }
            },
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_RecoveryConfig_to_wire_f0118ada500eb098<'a>(
    mut env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::RecoveryConfig,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    Ok({
        let (
            __obj0,
            __obj1,
            __obj2,
        ): (jni::sys::jboolean, jni::sys::jint, jni::sys::jlong) = {
            let __so0: &::core::option::Option<_> = &(&v).mode;
            match __so0 {
                ::core::option::Option::Some(__sg0) => {
                    let __obj0: jni::sys::jboolean = 1u8;
                    let __obj1: jni::sys::jint;
                    let __obj2: jni::sys::jlong;
                    match __sg0 {
                        zenoh_flat::RecoveryMode::PeriodicQueries(__sv0) => {
                            let __enc___obj2 = match (|| -> ::core::result::Result<
                                _,
                                __JniErr,
                            > {
                                {
                                    let __chain_s0 = __jni_out_stage_0_Duration_to_wire_37da00112022eac0(
                                            &mut env,
                                            __sv0.clone(),
                                        )
                                        .map_err(|__e| <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()))?;
                                    __jni_out_convert_u64_to_wire_c9db59f6e5bef648(
                                        &mut env,
                                        __chain_s0,
                                    )
                                }
                            })() {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            __obj2 = __enc___obj2;
                            __obj1 = 0;
                        }
                        zenoh_flat::RecoveryMode::Heartbeat => {
                            __obj1 = 1;
                            __obj2 = 0i64;
                        }
                    }
                    (__obj0, __obj1, __obj2)
                }
                ::core::option::Option::None => (0u8, 0i32, 0i64),
            }
        };
        let __obj3: jni::sys::jlong = {
            let __enc3 = match __jni_out_convert_Option_Duration_jni_optional_intermediate_output_niche_to_wire_dc11bc4c611f2997(
                &mut env,
                (&(&v).retention_period).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc3
        };
        let __obj = env
            .call_static_method(
                "io/zenoh/jni/pubsub/RecoveryConfig",
                "fromParts",
                "(ZIJJ)Lio/zenoh/jni/pubsub/RecoveryConfig;",
                &[
                    jni::objects::JValue::from(__obj0),
                    jni::objects::JValue::from(__obj1),
                    jni::objects::JValue::from(__obj2),
                    jni::objects::JValue::from(__obj3),
                ],
            )
            .and_then(|__v| __v.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("encode struct via fromParts: {}", e)))?;
        __obj
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_RecoveryMode_b4d38f00e9ec3191<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<zenoh_flat::RecoveryMode, __JniErr> {
    Ok({
        let __tag = {
            if v.is_null() {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        "RecoveryMode: null value where a variant was required"
                            .to_string(),
                    ),
                );
            }
            let __tag = (|| -> ::core::result::Result<i32, __JniErr> {
                if env
                    .is_instance_of(
                        v,
                        "io/zenoh/jni/pubsub/RecoveryMode$PeriodicQueries",
                    )
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!(
                            "RecoveryMode: instanceof io/zenoh/jni/pubsub/RecoveryMode$PeriodicQueries: {}",
                            e
                        ),
                    ))?
                {
                    return ::core::result::Result::Ok(0i32);
                }
                if env
                    .is_instance_of(v, "io/zenoh/jni/pubsub/RecoveryMode$Heartbeat")
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!(
                            "RecoveryMode: instanceof io/zenoh/jni/pubsub/RecoveryMode$Heartbeat: {}",
                            e
                        ),
                    ))?
                {
                    return ::core::result::Result::Ok(1i32);
                }
                ::core::result::Result::Ok(-1i32)
            })()?;
            __tag
        };
        match __tag {
            0i32 => {
                let __choice = v;
                let __arm = __choice;
                let __p_v0_raw: jni::sys::jlong = env
                    .get_field(__arm, "v0", "J")
                    .and_then(|val| val.j())
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("RecoveryMode.PeriodicQueries.v0: {}", e)))? as _;
                zenoh_flat::RecoveryMode::PeriodicQueries(
                    {
                        let __chain_s0 = __jni_in_convert_wire_to_u64_8507143745dc33b9(
                            env,
                            &__p_v0_raw,
                        )?;
                        let __chain_s1 = __jni_in_stage_0_wire_to_Duration_814bb872b19d3627(
                                env,
                                __chain_s0,
                            )
                            .map_err(|__e| <__JniErr as ::core::convert::From<
                                String,
                            >>::from(__e.to_string()))?;
                        ::core::result::Result::<_, __JniErr>::Ok(__chain_s1)
                    }?,
                )
            }
            1i32 => zenoh_flat::RecoveryMode::Heartbeat,
            _ => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        "RecoveryMode: value is not one of its declared variants"
                            .to_string(),
                    ),
                );
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Reliability_44f15ad2b611920f<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::Reliability, __JniErr> {
    Ok({
        match *v as i64 {
            0 => zenoh_flat::Reliability::BestEffort,
            1 => zenoh_flat::Reliability::Reliable,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!("invalid {} discriminant: {}", "Reliability", other),
                    ),
                );
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Reliability_to_wire_1173d3c71693bfba<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Reliability,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_in_convert_wire_to_RepliesConfig_jni_product_intermediate_tuple_956439fbd765eecc<
    'env,
    'a,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (jni::sys::jint, jni::sys::jint, jni::sys::jboolean),
) -> ::core::result::Result<zenoh_flat::RepliesConfig, __JniErr> {
    ::core::result::Result::Ok(zenoh_flat::RepliesConfig {
        priority: __jni_in_convert_wire_to_Priority_2f3eb78aa92f8bfd(env, &((v).0))?,
        congestion_control: __jni_in_convert_wire_to_CongestionControl_f81af10c6d71d5f3(
            env,
            &((v).1),
        )?,
        is_express: __jni_in_convert_wire_to_bool_1be2f6c32f925207(env, &((v).2))?,
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_RepliesConfig_7a81db0e8d2214ef<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<zenoh_flat::RepliesConfig, __JniErr> {
    Ok({
        let __priority_jobj: jni::objects::JObject = env
            .get_field(v, "priority", "Lio/zenoh/jni/qos/Priority;")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("RepliesConfig.priority: {}", e)))?;
        let __priority_raw: jni::sys::jint = env
            .call_method(&__priority_jobj, "getValue", "()I", &[])
            .and_then(|val| val.i())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("RepliesConfig.priority: {}", e)))?;
        let __congestion_control_jobj: jni::objects::JObject = env
            .get_field(v, "congestionControl", "Lio/zenoh/jni/qos/CongestionControl;")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("RepliesConfig.congestionControl: {}", e)))?;
        let __congestion_control_raw: jni::sys::jint = env
            .call_method(&__congestion_control_jobj, "getValue", "()I", &[])
            .and_then(|val| val.i())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("RepliesConfig.congestionControl: {}", e)))?;
        let __is_express_raw: jni::sys::jboolean = env
            .get_field(v, "isExpress", "Z")
            .and_then(|val| val.z())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("RepliesConfig.isExpress: {}", e)))? as _;
        zenoh_flat::RepliesConfig {
            priority: __jni_in_convert_wire_to_Priority_2f3eb78aa92f8bfd(
                env,
                &__priority_raw,
            )?,
            congestion_control: __jni_in_convert_wire_to_CongestionControl_f81af10c6d71d5f3(
                env,
                &__congestion_control_raw,
            )?,
            is_express: __jni_in_convert_wire_to_bool_1be2f6c32f925207(
                env,
                &__is_express_raw,
            )?,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_RepliesConfig_to_wire_e1ad55cbf9b645b2<'a>(
    mut env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::RepliesConfig,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    Ok({
        let __obj0: jni::sys::jint = {
            let __enc0 = match __jni_out_convert_Priority_to_wire_55b65fa623d4787e(
                &mut env,
                (&(&v).priority).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc0
        };
        let __obj1: jni::sys::jint = {
            let __enc1 = match __jni_out_convert_CongestionControl_to_wire_dd1f6047005395ca(
                &mut env,
                (&(&v).congestion_control).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc1
        };
        let __obj2: jni::sys::jboolean = {
            let __enc2 = match __jni_out_convert_bool_to_wire_3ee62077915d5228(
                &mut env,
                (&(&v).is_express).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc2
        };
        let __obj = env
            .call_static_method(
                "io/zenoh/jni/pubsub/RepliesConfig",
                "fromParts",
                "(IIZ)Lio/zenoh/jni/pubsub/RepliesConfig;",
                &[
                    jni::objects::JValue::from(__obj0),
                    jni::objects::JValue::from(__obj1),
                    jni::objects::JValue::from(__obj2),
                ],
            )
            .and_then(|__v| __v.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("encode struct via fromParts: {}", e)))?;
        __obj
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_ReplyKeyExpr_66de709ea94f3a25<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::ReplyKeyExpr, __JniErr> {
    Ok({
        match *v as i64 {
            0 => zenoh_flat::ReplyKeyExpr::Any,
            1 => zenoh_flat::ReplyKeyExpr::MatchingQuery,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!("invalid {} discriminant: {}", "ReplyKeyExpr", other),
                    ),
                );
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_ReplyKeyExpr_to_wire_fed1222b1d494b5e<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ReplyKeyExpr,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Sample_jni_handle_codec_consume_input_ca6bc567cd12e1fc<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<zenoh_flat::Sample, __JniErr> {
    if *v == 0 || (*v & 1) == 1 {
        return ::core::result::Result::Err(
            <__JniErr as ::core::convert::From<
                String,
            >>::from("Operation on a closed native handle.".to_string()),
        );
    }
    ::core::result::Result::Ok(unsafe {
        *::std::boxed::Box::from_raw(*v as *mut zenoh_flat::Sample)
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Sample_jni_handle_codec_own_output_to_wire_c2cd4d4f2b134f88<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Sample,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_SampleKind_be4075e9e7829925<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::SampleKind, __JniErr> {
    Ok({
        match *v as i64 {
            0 => zenoh_flat::SampleKind::Put,
            1 => zenoh_flat::SampleKind::Delete,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("invalid {} discriminant: {}", "SampleKind", other)),
                );
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_SampleKind_to_wire_5850f5fbf98a86e6<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::SampleKind,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_SampleMissListener_jni_handle_codec_own_output_to_wire_1b1d4f70a3effe1c<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::SampleMissListener,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Scout_jni_handle_codec_own_output_to_wire_9667aa43f25707f6<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Scout,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Selector_jni_product_intermediate_tuple_f642894fba00fd17<
    'env,
    'a,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (jni::sys::jlong, jni::objects::JString<'a>),
) -> ::core::result::Result<zenoh_flat::Selector, __JniErr> {
    ::core::result::Result::Ok(zenoh_flat::Selector {
        key_expr: __jni_in_convert_wire_to_KeyExpr_jni_handle_codec_consume_input_7fcb26b92cb14c18(
            env,
            &((v).0),
        )?,
        parameters: __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
            env,
            &((v).1),
        )?,
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Selector_15f961eaa6f3dedb<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<zenoh_flat::Selector, __JniErr> {
    Ok({
        let __key_expr_jobj: jni::objects::JObject = env
            .get_field(v, "keyExpr", "Lio/zenoh/jni/keyexpr/KeyExpr;")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Selector.keyExpr: {}", e)))?;
        let __key_expr_raw: jni::sys::jlong = if __key_expr_jobj.is_null() {
            0
        } else {
            env.call_method(&__key_expr_jobj, "peek", "()J", &[])
                .and_then(|val| val.j())
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Selector.keyExpr: {}", e)))?
        };
        let __parameters_jobj: jni::objects::JObject = env
            .get_field(v, "parameters", "Ljava/lang/String;")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Selector.parameters: {}", e)))?;
        let __parameters_raw: jni::objects::JString = __parameters_jobj.into();
        zenoh_flat::Selector {
            key_expr: __jni_in_convert_wire_to_KeyExpr_jni_handle_codec_consume_input_7fcb26b92cb14c18(
                env,
                &__key_expr_raw,
            )?,
            parameters: __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
                env,
                &__parameters_raw,
            )?,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Selector_to_wire_3b1136cb339f934c<'a>(
    mut env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Selector,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    Ok({
        let __obj0: jni::sys::jlong = {
            let __enc0 = match __jni_out_convert_KeyExpr_jni_handle_codec_own_output_to_wire_2022bfd454e7e1bc(
                &mut env,
                (&(&v).key_expr).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc0
        };
        let __obj1: jni::objects::JObject = {
            let __enc1 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                &mut env,
                (&(&v).parameters).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc1.into()
        };
        let __obj = env
            .call_static_method(
                "io/zenoh/jni/query/Selector",
                "fromParts",
                "(JLjava/lang/String;)Lio/zenoh/jni/query/Selector;",
                &[
                    jni::objects::JValue::from(__obj0),
                    jni::objects::JValue::Object(&__obj1),
                ],
            )
            .and_then(|__v| __v.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("encode struct via fromParts: {}", e)))?;
        __obj
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Session_jni_handle_codec_own_output_to_wire_cdc63163a43ff710<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Session,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_SetIntersectionLevel_a4b2c44239e675ab<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::SetIntersectionLevel, __JniErr> {
    Ok({
        match *v as i64 {
            0 => zenoh_flat::SetIntersectionLevel::Disjoint,
            1 => zenoh_flat::SetIntersectionLevel::Intersects,
            2 => zenoh_flat::SetIntersectionLevel::Includes,
            3 => zenoh_flat::SetIntersectionLevel::Equals,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(
                        format!(
                            "invalid {} discriminant: {}", "SetIntersectionLevel", other
                        ),
                    ),
                );
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_SetIntersectionLevel_to_wire_7d91a182acc0b634<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::SetIntersectionLevel,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_SourceInfo_669b50ca532eff47<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<zenoh_flat::SourceInfo, __JniErr> {
    Ok({
        let __source_raw: jni::objects::JObject = env
            .get_field(v, "source", "Lio/zenoh/jni/pubsub/EntityGlobalId;")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("SourceInfo.source: {}", e)))?;
        let __sn_raw: jni::sys::jlong = env
            .get_field(v, "sn", "J")
            .and_then(|val| val.j())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("SourceInfo.sn: {}", e)))? as _;
        zenoh_flat::SourceInfo {
            source: __jni_in_convert_wire_to_EntityGlobalId_5c2de07f2d46bb63(
                env,
                &__source_raw,
            )?,
            sn: __jni_in_convert_wire_to_u32_25dff6d476799035(env, &__sn_raw)?,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_out_convert_SourceInfo_jni_product_intermediate_tuple_to_wire_edc2ee9a82e917fa<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::SourceInfo,
) -> ::core::result::Result<
    (((jni::objects::JByteArray<'a>,), jni::sys::jlong), jni::sys::jlong),
    __JniErr,
> {
    ::core::result::Result::Ok((
        __jni_out_convert_EntityGlobalId_jni_product_intermediate_tuple_to_wire_36316835bdf189ae(
            env,
            v.source,
        )?,
        __jni_out_convert_u32_to_wire_b6376ae826304960(env, v.sn)?,
    ))
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_SourceInfo_to_wire_ec86c882a01214d4<'a>(
    mut env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::SourceInfo,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    Ok({
        let __obj0: jni::objects::JObject = {
            let __enc0 = match __jni_out_convert_u8_ZENOH_ID_MAX_SIZE_to_wire_7574f9b51f0f53e4(
                &mut env,
                (&(&(&(&v).source).zid).bytes).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc0.into()
        };
        let __obj1: jni::sys::jlong = {
            let __enc1 = match __jni_out_convert_u32_to_wire_b6376ae826304960(
                &mut env,
                (&(&(&v).source).eid).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc1
        };
        let __obj2: jni::sys::jlong = {
            let __enc2 = match __jni_out_convert_u32_to_wire_b6376ae826304960(
                &mut env,
                (&(&v).sn).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc2
        };
        let __obj = env
            .call_static_method(
                "io/zenoh/jni/sample/SourceInfo",
                "fromParts",
                "([BJJ)Lio/zenoh/jni/sample/SourceInfo;",
                &[
                    jni::objects::JValue::Object(&__obj0),
                    jni::objects::JValue::from(__obj1),
                    jni::objects::JValue::from(__obj2),
                ],
            )
            .and_then(|__v| __v.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("encode struct via fromParts: {}", e)))?;
        __obj
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: String,
) -> ::core::result::Result<jni::objects::JString<'a>, __JniErr> {
    Ok({
        env.new_string(&*v)
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("encode_str: {}", e))
            })?
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Subscriber_jni_handle_codec_own_output_to_wire_4552f6e215a6371c<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Subscriber,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Timestamp_jni_product_intermediate_tuple_8da7af64ea169b8e<
    'env,
    'a,
>(
    env: &mut jni::JNIEnv<'env>,
    v: (jni::sys::jlong, jni::objects::JByteArray<'a>),
) -> ::core::result::Result<zenoh_flat::Timestamp, __JniErr> {
    ::core::result::Result::Ok(zenoh_flat::Timestamp {
        ntp64: __jni_in_convert_wire_to_u64_8507143745dc33b9(env, &((v).0))?,
        id: __jni_in_convert_wire_to_Vec_u8_80984e9556387695(env, &((v).1))?,
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Timestamp_37bfb116800bb25b<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<zenoh_flat::Timestamp, __JniErr> {
    Ok({
        let __ntp64_raw: jni::sys::jlong = env
            .get_field(v, "ntp64", "J")
            .and_then(|val| val.j())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Timestamp.ntp64: {}", e)))?;
        let __id_jobj: jni::objects::JObject = env
            .get_field(v, "id", "[B")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Timestamp.id: {}", e)))?;
        let __id_raw: jni::objects::JByteArray = __id_jobj.into();
        zenoh_flat::Timestamp {
            ntp64: __jni_in_convert_wire_to_u64_8507143745dc33b9(env, &__ntp64_raw)?,
            id: __jni_in_convert_wire_to_Vec_u8_80984e9556387695(env, &__id_raw)?,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_out_convert_Timestamp_jni_product_intermediate_tuple_to_wire_e28de6376a8e24b5<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Timestamp,
) -> ::core::result::Result<(jni::sys::jlong, jni::objects::JByteArray<'a>), __JniErr> {
    ::core::result::Result::Ok((
        __jni_out_convert_u64_to_wire_c9db59f6e5bef648(env, v.ntp64)?,
        __jni_out_convert_Vec_u8_to_wire_e9499bf0a706b1a2(env, v.id)?,
    ))
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Timestamp_to_wire_66f4b22dc6f3c572<'a>(
    mut env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::Timestamp,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    Ok({
        let __obj0: jni::sys::jlong = {
            let __enc0 = match __jni_out_convert_u64_to_wire_c9db59f6e5bef648(
                &mut env,
                (&(&v).ntp64).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc0
        };
        let __obj1: jni::objects::JObject = {
            let __enc1 = match __jni_out_convert_Vec_u8_to_wire_e9499bf0a706b1a2(
                &mut env,
                (&(&v).id).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc1.into()
        };
        let __obj = env
            .call_static_method(
                "io/zenoh/jni/time/Timestamp",
                "fromParts",
                "(J[B)Lio/zenoh/jni/time/Timestamp;",
                &[
                    jni::objects::JValue::from(__obj0),
                    jni::objects::JValue::Object(&__obj1),
                ],
            )
            .and_then(|__v| __v.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("encode struct via fromParts: {}", e)))?;
        __obj
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_TimestampInstrumentation_8446cbe1c44cb885<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<zenoh_flat::TimestampInstrumentation, __JniErr> {
    Ok({
        let __send_raw: jni::sys::jboolean = env
            .get_field(v, "send", "Z")
            .and_then(|val| val.z())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("TimestampInstrumentation.send: {}", e)))? as _;
        let __route_raw: jni::sys::jboolean = env
            .get_field(v, "route", "Z")
            .and_then(|val| val.z())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("TimestampInstrumentation.route: {}", e)))? as _;
        let __receive_raw: jni::sys::jboolean = env
            .get_field(v, "receive", "Z")
            .and_then(|val| val.z())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("TimestampInstrumentation.receive: {}", e)))? as _;
        zenoh_flat::TimestampInstrumentation {
            send: __jni_in_convert_wire_to_bool_1be2f6c32f925207(env, &__send_raw)?,
            route: __jni_in_convert_wire_to_bool_1be2f6c32f925207(env, &__route_raw)?,
            receive: __jni_in_convert_wire_to_bool_1be2f6c32f925207(env, &__receive_raw)?,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_out_convert_TimestampInstrumentation_jni_product_intermediate_tuple_to_wire_2be420083815e28f<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::TimestampInstrumentation,
) -> ::core::result::Result<
    (jni::sys::jboolean, jni::sys::jboolean, jni::sys::jboolean),
    __JniErr,
> {
    ::core::result::Result::Ok((
        __jni_out_convert_bool_to_wire_3ee62077915d5228(env, v.send)?,
        __jni_out_convert_bool_to_wire_3ee62077915d5228(env, v.route)?,
        __jni_out_convert_bool_to_wire_3ee62077915d5228(env, v.receive)?,
    ))
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_TimestampInstrumentation_to_wire_f76473b23dc1237a<
    'a,
>(
    mut env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::TimestampInstrumentation,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    Ok({
        let __obj0: jni::sys::jboolean = {
            let __enc0 = match __jni_out_convert_bool_to_wire_3ee62077915d5228(
                &mut env,
                (&(&v).send).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc0
        };
        let __obj1: jni::sys::jboolean = {
            let __enc1 = match __jni_out_convert_bool_to_wire_3ee62077915d5228(
                &mut env,
                (&(&v).route).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc1
        };
        let __obj2: jni::sys::jboolean = {
            let __enc2 = match __jni_out_convert_bool_to_wire_3ee62077915d5228(
                &mut env,
                (&(&v).receive).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc2
        };
        let __obj = env
            .call_static_method(
                "io/zenoh/jni/time/TimestampInstrumentation",
                "fromParts",
                "(ZZZ)Lio/zenoh/jni/time/TimestampInstrumentation;",
                &[
                    jni::objects::JValue::from(__obj0),
                    jni::objects::JValue::from(__obj1),
                    jni::objects::JValue::from(__obj2),
                ],
            )
            .and_then(|__v| __v.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("encode struct via fromParts: {}", e)))?;
        __obj
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_TimestampStackRecord_7870ebb255b34543<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<zenoh_flat::TimestampStackRecord, __JniErr> {
    Ok({
        let __point_jobj: jni::objects::JObject = env
            .get_field(v, "point", "Lio/zenoh/jni/time/InterceptionPoint;")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("TimestampStackRecord.point: {}", e)))?;
        let __point_raw: jni::sys::jint = env
            .call_method(&__point_jobj, "getValue", "()I", &[])
            .and_then(|val| val.i())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("TimestampStackRecord.point: {}", e)))?;
        let __timestamp_raw: jni::objects::JObject = env
            .get_field(v, "timestamp", "Lio/zenoh/jni/time/InstrumentationTimestamp;")
            .and_then(|val| val.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("TimestampStackRecord.timestamp: {}", e)))?;
        zenoh_flat::TimestampStackRecord {
            point: __jni_in_convert_wire_to_InterceptionPoint_f245d29a73d8add3(
                env,
                &__point_raw,
            )?,
            timestamp: __jni_in_convert_wire_to_InstrumentationTimestamp_7254d25ac1aeaa71(
                env,
                &__timestamp_raw,
            )?,
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_out_convert_TimestampStackRecord_jni_product_intermediate_tuple_to_wire_a61a345e64f503e6<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::TimestampStackRecord,
) -> ::core::result::Result<
    (
        jni::sys::jint,
        (jni::sys::jint, (jni::objects::JObject<'a>,), (jni::objects::JByteArray<'a>,)),
    ),
    __JniErr,
> {
    ::core::result::Result::Ok((
        __jni_out_convert_InterceptionPoint_to_wire_cd6b109f4fdb1f8a(env, v.point)?,
        __jni_out_convert_InstrumentationTimestamp_jni_choice_intermediate_tagged_tuple_to_wire_cdbd34f354ae1eb6(
            env,
            v.timestamp,
        )?,
    ))
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_TimestampStackRecord_to_wire_0fc73bc8e35125ec<'a>(
    mut env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::TimestampStackRecord,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    Ok({
        let __obj1: jni::sys::jint;
        let __obj2: jni::objects::JObject;
        let __obj3: jni::objects::JObject;
        match &(&v).timestamp {
            zenoh_flat::InstrumentationTimestamp::Uhlc(__sv0) => {
                let __enc___obj2 = match __jni_out_convert_Timestamp_to_wire_66f4b22dc6f3c572(
                    &mut env,
                    __sv0.clone(),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        return ::core::result::Result::Err(
                            <__JniErr as ::core::convert::From<
                                String,
                            >>::from(__e.to_string()),
                        );
                    }
                };
                __obj2 = __enc___obj2;
                __obj1 = 0;
                __obj3 = jni::objects::JObject::null();
            }
            zenoh_flat::InstrumentationTimestamp::Custom(__sv0) => {
                let __enc___obj3 = match __jni_out_convert_Vec_u8_to_wire_e9499bf0a706b1a2(
                    &mut env,
                    __sv0.clone(),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        return ::core::result::Result::Err(
                            <__JniErr as ::core::convert::From<
                                String,
                            >>::from(__e.to_string()),
                        );
                    }
                };
                __obj3 = __enc___obj3.into();
                __obj1 = 1;
                __obj2 = jni::objects::JObject::null();
            }
        }
        let __obj0: jni::sys::jint = {
            let __enc0 = match __jni_out_convert_InterceptionPoint_to_wire_cd6b109f4fdb1f8a(
                &mut env,
                (&(&v).point).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc0
        };
        let __obj = env
            .call_static_method(
                "io/zenoh/jni/time/TimestampStackRecord",
                "fromParts",
                "(IILio/zenoh/jni/time/Timestamp;[B)Lio/zenoh/jni/time/TimestampStackRecord;",
                &[
                    jni::objects::JValue::from(__obj0),
                    jni::objects::JValue::from(__obj1),
                    jni::objects::JValue::Object(&__obj2),
                    jni::objects::JValue::Object(&__obj3),
                ],
            )
            .and_then(|__v| __v.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("encode struct via fromParts: {}", e)))?;
        __obj
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_sequence_Vec_String_to_wire_6999758d96be80b1<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: ::std::vec::Vec<::std::string::String>,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    ::core::result::Result::Ok({
        let __sequence_source = v;
        let __sequence_output = env
            .new_object("java/util/ArrayList", "()V", &[])
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Vec<_>: new ArrayList: {}", e)))?;
        let __sequence_list = jni::objects::JList::from_env(env, &__sequence_output)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Vec<_>: list-from-env: {}", e)))?;
        for __sequence_element in __sequence_source.into_iter() {
            let __sequence_part = __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                env,
                __sequence_element,
            )?;
            let __sequence_object: jni::objects::JObject = __sequence_part.into();
            __sequence_list
                .add(env, &__sequence_object)
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Vec<_>: list-add: {}", e)))?;
        }
        __sequence_output
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_Vec_u8_80984e9556387695<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JByteArray<'v>,
) -> ::core::result::Result<::std::vec::Vec<u8>, __JniErr> {
    Ok({
        env.convert_byte_array(v)
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("decode_byte_array: {}", e))
            })?
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_Vec_u8_to_wire_e9499bf0a706b1a2<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: ::std::vec::Vec<u8>,
) -> ::core::result::Result<jni::objects::JByteArray<'a>, __JniErr> {
    Ok({
        env.byte_array_from_slice(v.as_slice())
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("encode_byte_array: {}", e))
            })?
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_WhatAmI_df2327086bc2ecc3<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<zenoh_flat::WhatAmI, __JniErr> {
    Ok({
        match *v as i64 {
            1 => zenoh_flat::WhatAmI::Router,
            2 => zenoh_flat::WhatAmI::Peer,
            4 => zenoh_flat::WhatAmI::Client,
            other => {
                return ::core::result::Result::Err(
                    <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("invalid {} discriminant: {}", "WhatAmI", other)),
                );
            }
        }
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_WhatAmI_to_wire_907d4a5187d437e6<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::WhatAmI,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok({ v as jni::sys::jint })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_ZBytes_jni_handle_codec_own_output_to_wire_1d6ceb9f821de6d7<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZBytes,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(std::boxed::Box::into_raw(std::boxed::Box::new(v)) as i64)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
#[inline(always)]
pub(crate) unsafe fn __jni_out_convert_ZenohId_jni_product_intermediate_tuple_to_wire_c7bb0eb9058fd8ec<
    'a,
>(
    env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZenohId,
) -> ::core::result::Result<(jni::objects::JByteArray<'a>,), __JniErr> {
    ::core::result::Result::Ok((
        __jni_out_convert_u8_ZENOH_ID_MAX_SIZE_to_wire_7574f9b51f0f53e4(env, v.bytes)?,
    ))
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_ZenohId_to_wire_267e7a72cf7e4d7a<'a>(
    mut env: &mut jni::JNIEnv<'a>,
    v: zenoh_flat::ZenohId,
) -> ::core::result::Result<jni::objects::JObject<'a>, __JniErr> {
    Ok({
        let __obj0: jni::objects::JObject = {
            let __enc0 = match __jni_out_convert_u8_ZENOH_ID_MAX_SIZE_to_wire_7574f9b51f0f53e4(
                &mut env,
                (&(&v).bytes).clone(),
            ) {
                ::core::result::Result::Ok(__w) => __w,
                ::core::result::Result::Err(__e) => {
                    return ::core::result::Result::Err(
                        <__JniErr as ::core::convert::From<
                            String,
                        >>::from(__e.to_string()),
                    );
                }
            };
            __enc0.into()
        };
        let __obj = env
            .call_static_method(
                "io/zenoh/jni/config/ZenohId",
                "fromParts",
                "([B)Lio/zenoh/jni/config/ZenohId;",
                &[jni::objects::JValue::Object(&__obj0)],
            )
            .and_then(|__v| __v.l())
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("encode struct via fromParts: {}", e)))?;
        __obj
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_u8_ZENOH_ID_MAX_SIZE_9ba98e7d0e8ec60b<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JByteArray<'v>,
) -> ::core::result::Result<[u8; zenoh_flat::ZENOH_ID_MAX_SIZE], __JniErr> {
    Ok({
        let __buf = env
            .convert_byte_array(v)
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("fixed-size array decode: {}", e))
            })?;
        let __arr: [u8; zenoh_flat::ZENOH_ID_MAX_SIZE] = __buf
            .as_slice()
            .try_into()
            .map_err(|_| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(
                    "fixed-size array decode: `[u8 ; ZENOH_ID_MAX_SIZE]` expects a different length"
                        .to_string(),
                )
            })?;
        __arr
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_u8_ZENOH_ID_MAX_SIZE_to_wire_7574f9b51f0f53e4<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: [u8; zenoh_flat::ZENOH_ID_MAX_SIZE],
) -> ::core::result::Result<jni::objects::JByteArray<'a>, __JniErr> {
    Ok({
        env.byte_array_from_slice(&v)
            .map_err(|e| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("fixed-size array encode: {}", e))
            })?
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_bool_1be2f6c32f925207<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jboolean,
) -> ::core::result::Result<bool, __JniErr> {
    Ok(*v != 0)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_bool_to_wire_3ee62077915d5228<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: bool,
) -> ::core::result::Result<jni::sys::jboolean, __JniErr> {
    Ok(v as jni::sys::jboolean)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_f64_b312e1b95182cdfd<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jdouble,
) -> ::core::result::Result<f64, __JniErr> {
    Ok(*v)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_f64_to_wire_61461de12ea6bc04<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: f64,
) -> ::core::result::Result<jni::sys::jdouble, __JniErr> {
    Ok(v as jni::sys::jdouble)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_i32_83b133e23cc76fc5<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<i32, __JniErr> {
    Ok(*v)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_i64_da07d745d9e26f71<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<i64, __JniErr> {
    Ok(*v)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_impl_Fn_Send_Sync_static_6d213555cb0eda7b<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<impl Fn() + Send + Sync + 'static, __JniErr> {
    Ok({
        use std::sync::Arc;
        let java_vm = Arc::new(
            env
                .get_java_vm()
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Unable to retrieve JVM: {}", e)))?,
        );
        let callback_global_ref = env
            .new_global_ref(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to global-ref callback: {}", e)))?;
        let __invoke_class = env
            .get_object_class(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to get callback class for {}: {}", "Fn()", e)))?;
        let __invoke_id = env
            .get_method_id(&__invoke_class, "run", "()V")
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn()", e)))?;
        Box::new(move || {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn()", e)))?;
                env.push_local_frame(16)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn()", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let __call_res: ::core::result::Result<(), __JniErr> = unsafe {
                        env.call_method_unchecked(
                            &callback_global_ref,
                            __invoke_id,
                            jni::signature::ReturnType::Primitive(
                                jni::signature::Primitive::Void,
                            ),
                            &[],
                        )
                    }
                        .map(|_| ())
                        .map_err(|e| {
                            let _ = env.exception_describe();
                            <__JniErr as ::core::convert::From<
                                String,
                            >>::from(e.to_string())
                        });
                    __call_res?;
                    Ok(())
                })();
                let _ = unsafe { env.pop_local_frame(&jni::objects::JObject::null()) };
                __frame_res?;
                Ok(())
            })()
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn()"));
        })
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_impl_Fn_Hello_Send_Sync_static_149e928a50c5a2b9<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<
    impl Fn(zenoh_flat::Hello) + Send + Sync + 'static,
    __JniErr,
> {
    Ok({
        use std::sync::Arc;
        let java_vm = Arc::new(
            env
                .get_java_vm()
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Unable to retrieve JVM: {}", e)))?,
        );
        let callback_global_ref = env
            .new_global_ref(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to global-ref callback: {}", e)))?;
        let __invoke_class = env
            .get_object_class(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(
                format!("Unable to get callback class for {}: {}", "Fn(Hello)", e),
            ))?;
        let __invoke_id = env
            .get_method_id(
                &__invoke_class,
                "run",
                "(ILio/zenoh/jni/config/ZenohId;Ljava/util/List;)V",
            )
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn(Hello)", e)))?;
        Box::new(move |__cb_arg0: zenoh_flat::Hello| {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn(Hello)", e)))?;
                env.push_local_frame(16)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn(Hello)", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let __cb0_obj0: jni::sys::jvalue = {
                        let __enc0 = match __jni_out_convert_WhatAmI_to_wire_907d4a5187d437e6(
                            &mut env,
                            zenoh_flat::hello_get_whatami(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc0 }
                    };
                    let __cb0_obj1: jni::objects::JObject = {
                        let __enc1 = match __jni_out_convert_ZenohId_to_wire_267e7a72cf7e4d7a(
                            &mut env,
                            zenoh_flat::hello_get_zid(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc1
                    };
                    let __cb0_obj2: jni::objects::JObject = {
                        let __enc2 = match __jni_out_convert_sequence_Vec_String_to_wire_6999758d96be80b1(
                            &mut env,
                            zenoh_flat::hello_get_locators(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc2
                    };
                    let __call_res: ::core::result::Result<(), __JniErr> = unsafe {
                        env.call_method_unchecked(
                            &callback_global_ref,
                            __invoke_id,
                            jni::signature::ReturnType::Primitive(
                                jni::signature::Primitive::Void,
                            ),
                            &[
                                __cb0_obj0,
                                jni::sys::jvalue {
                                    l: __cb0_obj1.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj2.as_raw(),
                                },
                            ],
                        )
                    }
                        .map(|_| ())
                        .map_err(|e| {
                            let _ = env.exception_describe();
                            <__JniErr as ::core::convert::From<
                                String,
                            >>::from(e.to_string())
                        });
                    __call_res?;
                    Ok(())
                })();
                let _ = unsafe { env.pop_local_frame(&jni::objects::JObject::null()) };
                __frame_res?;
                Ok(())
            })()
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn(Hello)"));
        })
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_impl_Fn_Miss_Send_Sync_static_4ae3837246d447c7<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<
    impl Fn(zenoh_flat::Miss) + Send + Sync + 'static,
    __JniErr,
> {
    Ok({
        use std::sync::Arc;
        let java_vm = Arc::new(
            env
                .get_java_vm()
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Unable to retrieve JVM: {}", e)))?,
        );
        let callback_global_ref = env
            .new_global_ref(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to global-ref callback: {}", e)))?;
        let __invoke_class = env
            .get_object_class(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(
                format!("Unable to get callback class for {}: {}", "Fn(Miss)", e),
            ))?;
        let __invoke_id = env
            .get_method_id(&__invoke_class, "run", "([BJJ)V")
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn(Miss)", e)))?;
        Box::new(move |__cb_arg0: zenoh_flat::Miss| {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn(Miss)", e)))?;
                env.push_local_frame(16)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn(Miss)", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let (((__chain_wire0,), __chain_wire1), __chain_wire2) = match __jni_out_convert_Miss_jni_product_intermediate_tuple_to_wire_c9ab00e074005f65(
                        &mut env,
                        __cb_arg0,
                    ) {
                        ::core::result::Result::Ok(__intermediate) => __intermediate,
                        ::core::result::Result::Err(__chain_error) => {
                            return ::core::result::Result::Err(
                                <__JniErr as ::core::convert::From<
                                    String,
                                >>::from(__chain_error.to_string()),
                            );
                        }
                    };
                    let __cb0_obj0: jni::objects::JObject = __chain_wire0.into();
                    let __cb0_obj1 = jni::sys::jvalue {
                        j: __chain_wire1,
                    };
                    let __cb0_obj2 = jni::sys::jvalue {
                        j: __chain_wire2,
                    };
                    let __call_res: ::core::result::Result<(), __JniErr> = unsafe {
                        env.call_method_unchecked(
                            &callback_global_ref,
                            __invoke_id,
                            jni::signature::ReturnType::Primitive(
                                jni::signature::Primitive::Void,
                            ),
                            &[
                                jni::sys::jvalue {
                                    l: __cb0_obj0.as_raw(),
                                },
                                __cb0_obj1,
                                __cb0_obj2,
                            ],
                        )
                    }
                        .map(|_| ())
                        .map_err(|e| {
                            let _ = env.exception_describe();
                            <__JniErr as ::core::convert::From<
                                String,
                            >>::from(e.to_string())
                        });
                    __call_res?;
                    Ok(())
                })();
                let _ = unsafe { env.pop_local_frame(&jni::objects::JObject::null()) };
                __frame_res?;
                Ok(())
            })()
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn(Miss)"));
        })
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_impl_Fn_Query_Send_Sync_static_448bc7311098de85<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<
    impl Fn(zenoh_flat::Query) + Send + Sync + 'static,
    __JniErr,
> {
    Ok({
        use std::sync::Arc;
        let java_vm = Arc::new(
            env
                .get_java_vm()
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Unable to retrieve JVM: {}", e)))?,
        );
        let callback_global_ref = env
            .new_global_ref(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to global-ref callback: {}", e)))?;
        let __invoke_class = env
            .get_object_class(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(
                format!("Unable to get callback class for {}: {}", "Fn(Query)", e),
            ))?;
        let __invoke_id = env
            .get_method_id(
                &__invoke_class,
                "run",
                "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/Long;Ljava/lang/Integer;[BLjava/lang/Long;IJ)V",
            )
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn(Query)", e)))?;
        Box::new(move |__cb_arg0: zenoh_flat::Query| {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn(Query)", e)))?;
                env.push_local_frame(22)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn(Query)", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let __cb0_obj0: jni::objects::JObject = {
                        let __enc0 = match __jni_out_convert_jni_text_codec_borrowed_to_wire_bc4fe45698de9c2e(
                            &mut env,
                            zenoh_flat::keyexpr_as_str(
                                zenoh_flat::query_get_key_expr(&__cb_arg0),
                            ),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc0.into()
                    };
                    let __cb0_obj1: jni::objects::JObject = {
                        let __enc1 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                            &mut env,
                            zenoh_flat::query_get_parameters(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc1.into()
                    };
                    let __cb0_obj3: jni::objects::JObject = match zenoh_flat::query_get_encoding(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc3 = match __jni_out_convert_u16_to_wire_279d2392219844b4(
                                &mut env,
                                zenoh_flat::encoding_get_id(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen_jni_runtime::box_jint(&mut env, __enc3) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<String>>::from(__e),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj4: jni::objects::JObject = match zenoh_flat::query_get_encoding(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc4 = match __jni_out_convert_Option_Vec_u8_jni_optional_intermediate_output_niche_to_wire_beaf597ddc121688(
                                &mut env,
                                zenoh_flat::encoding_get_schema(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            __enc4.into()
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj6: jni::sys::jvalue = {
                        let __enc6 = match __jni_out_convert_ReplyKeyExpr_to_wire_fed1222b1d494b5e(
                            &mut env,
                            zenoh_flat::query_get_accepts_replies(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc6 }
                    };
                    let __cb0_obj2: jni::objects::JObject = match zenoh_flat::query_get_payload(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h2: jni::sys::jlong = match __jni_out_convert_ZBytes_jni_handle_codec_clone_output_to_wire_631cd85333755af2(
                                &mut env,
                                __n0,
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen_jni_runtime::box_jlong(&mut env, __h2) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj5: jni::objects::JObject = match zenoh_flat::query_get_attachment(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h5: jni::sys::jlong = match __jni_out_convert_ZBytes_jni_handle_codec_clone_output_to_wire_631cd85333755af2(
                                &mut env,
                                __n0,
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen_jni_runtime::box_jlong(&mut env, __h5) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj7: jni::sys::jvalue = jni::sys::jvalue {
                        j: std::boxed::Box::into_raw(std::boxed::Box::new(__cb_arg0))
                            as jni::sys::jlong,
                    };
                    let __call_res: ::core::result::Result<(), __JniErr> = unsafe {
                        env.call_method_unchecked(
                            &callback_global_ref,
                            __invoke_id,
                            jni::signature::ReturnType::Primitive(
                                jni::signature::Primitive::Void,
                            ),
                            &[
                                jni::sys::jvalue {
                                    l: __cb0_obj0.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj1.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj2.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj3.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj4.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj5.as_raw(),
                                },
                                __cb0_obj6,
                                __cb0_obj7,
                            ],
                        )
                    }
                        .map(|_| ())
                        .map_err(|e| {
                            let _ = env.exception_describe();
                            <__JniErr as ::core::convert::From<
                                String,
                            >>::from(e.to_string())
                        });
                    __call_res?;
                    Ok(())
                })();
                let _ = unsafe { env.pop_local_frame(&jni::objects::JObject::null()) };
                __frame_res?;
                Ok(())
            })()
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn(Query)"));
        })
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_impl_Fn_Reply_Send_Sync_static_8acaa3be44c06271<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<
    impl Fn(zenoh_flat::Reply) + Send + Sync + 'static,
    __JniErr,
> {
    Ok({
        use std::sync::Arc;
        let java_vm = Arc::new(
            env
                .get_java_vm()
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Unable to retrieve JVM: {}", e)))?,
        );
        let callback_global_ref = env
            .new_global_ref(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to global-ref callback: {}", e)))?;
        let __invoke_class = env
            .get_object_class(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(
                format!("Unable to get callback class for {}: {}", "Fn(Reply)", e),
            ))?;
        let __invoke_id = env
            .get_method_id(
                &__invoke_class,
                "run",
                "(Lio/zenoh/jni/pubsub/EntityGlobalId;ZLjava/lang/String;Ljava/lang/Long;Ljava/lang/Integer;[BLjava/lang/Integer;Lio/zenoh/jni/time/Timestamp;Ljava/lang/Boolean;Ljava/lang/Integer;Ljava/lang/Integer;Ljava/lang/Long;Ljava/lang/Integer;Lio/zenoh/jni/sample/SourceInfo;Ljava/lang/Long;Ljava/lang/Integer;[B)V",
            )
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn(Reply)", e)))?;
        Box::new(move |__cb_arg0: zenoh_flat::Reply| {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn(Reply)", e)))?;
                env.push_local_frame(40)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn(Reply)", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let __vf0 = zenoh_flat::reply_get_sample(&__cb_arg0)
                        .map(|__hb0| zenoh_flat::sample_into_struct((__hb0).clone()));
                    let __cb0_obj0: jni::objects::JObject = {
                        let __enc0 = match __jni_out_convert_Option_EntityGlobalId_jni_optional_intermediate_output_niche_to_wire_be8cdb5085ebce04(
                            &mut env,
                            zenoh_flat::reply_get_replier_id(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc0
                    };
                    let __cb0_obj1: jni::sys::jvalue = {
                        let __enc1 = match __jni_out_convert_bool_to_wire_3ee62077915d5228(
                            &mut env,
                            zenoh_flat::reply_is_ok(&__cb_arg0),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { z: __enc1 }
                    };
                    let __cb0_obj15: jni::objects::JObject = match zenoh_flat::reply_get_err(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc15 = match __jni_out_convert_u16_to_wire_279d2392219844b4(
                                &mut env,
                                zenoh_flat::encoding_get_id(
                                    zenoh_flat::reply_error_get_encoding(__n0),
                                ),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen_jni_runtime::box_jint(&mut env, __enc15) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<String>>::from(__e),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj16: jni::objects::JObject = match zenoh_flat::reply_get_err(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __enc16 = match __jni_out_convert_Option_Vec_u8_jni_optional_intermediate_output_niche_to_wire_beaf597ddc121688(
                                &mut env,
                                zenoh_flat::encoding_get_schema(
                                    zenoh_flat::reply_error_get_encoding(__n0),
                                ),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            __enc16.into()
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let __cb0_obj14: jni::objects::JObject = match zenoh_flat::reply_get_err(
                        &__cb_arg0,
                    ) {
                        ::core::option::Option::Some(__n0) => {
                            let __h14: jni::sys::jlong = match __jni_out_convert_ZBytes_jni_handle_codec_clone_output_to_wire_631cd85333755af2(
                                &mut env,
                                zenoh_flat::reply_error_get_payload(__n0),
                            ) {
                                ::core::result::Result::Ok(__w) => __w,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            };
                            match ::prebindgen_jni_runtime::box_jlong(&mut env, __h14) {
                                ::core::result::Result::Ok(__o) => __o,
                                ::core::result::Result::Err(__e) => {
                                    return ::core::result::Result::Err(
                                        <__JniErr as ::core::convert::From<
                                            String,
                                        >>::from(__e.to_string()),
                                    );
                                }
                            }
                        }
                        ::core::option::Option::None => jni::objects::JObject::null(),
                    };
                    let (
                        __cb0_obj2,
                        __cb0_obj3,
                        __cb0_obj4,
                        __cb0_obj5,
                        __cb0_obj6,
                        __cb0_obj7,
                        __cb0_obj8,
                        __cb0_obj9,
                        __cb0_obj10,
                        __cb0_obj11,
                        __cb0_obj12,
                        __cb0_obj13,
                    ): (
                        jni::objects::JObject,
                        jni::objects::JObject,
                        jni::objects::JObject,
                        jni::objects::JObject,
                        jni::objects::JObject,
                        jni::objects::JObject,
                        jni::objects::JObject,
                        jni::objects::JObject,
                        jni::objects::JObject,
                        jni::objects::JObject,
                        jni::objects::JObject,
                        jni::objects::JObject,
                    ) = match __vf0 {
                        ::core::option::Option::Some(__u0) => {
                            let __cb0_obj2: jni::objects::JObject = {
                                let __enc2 = match __jni_out_convert_jni_text_codec_borrowed_to_wire_bc4fe45698de9c2e(
                                    &mut env,
                                    zenoh_flat::keyexpr_as_str(&__u0.key_expr),
                                ) {
                                    ::core::result::Result::Ok(__w) => __w,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<
                                                String,
                                            >>::from(__e.to_string()),
                                        );
                                    }
                                };
                                __enc2.into()
                            };
                            let __cb0_obj4: jni::objects::JObject = {
                                let __enc4 = match __jni_out_convert_u16_to_wire_279d2392219844b4(
                                    &mut env,
                                    zenoh_flat::encoding_get_id(&__u0.encoding),
                                ) {
                                    ::core::result::Result::Ok(__w) => __w,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<
                                                String,
                                            >>::from(__e.to_string()),
                                        );
                                    }
                                };
                                match ::prebindgen_jni_runtime::box_jint(&mut env, __enc4) {
                                    ::core::result::Result::Ok(__o) => __o,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<String>>::from(__e),
                                        );
                                    }
                                }
                            };
                            let __cb0_obj5: jni::objects::JObject = {
                                let __enc5 = match __jni_out_convert_Option_Vec_u8_jni_optional_intermediate_output_niche_to_wire_beaf597ddc121688(
                                    &mut env,
                                    zenoh_flat::encoding_get_schema(&__u0.encoding),
                                ) {
                                    ::core::result::Result::Ok(__w) => __w,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<
                                                String,
                                            >>::from(__e.to_string()),
                                        );
                                    }
                                };
                                __enc5.into()
                            };
                            let __cb0_obj6: jni::objects::JObject = {
                                let __enc6 = match __jni_out_convert_SampleKind_to_wire_5850f5fbf98a86e6(
                                    &mut env,
                                    __u0.kind,
                                ) {
                                    ::core::result::Result::Ok(__w) => __w,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<
                                                String,
                                            >>::from(__e.to_string()),
                                        );
                                    }
                                };
                                match ::prebindgen_jni_runtime::box_jint(&mut env, __enc6) {
                                    ::core::result::Result::Ok(__o) => __o,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<String>>::from(__e),
                                        );
                                    }
                                }
                            };
                            let __cb0_obj7: jni::objects::JObject = {
                                let __enc7 = match __jni_out_convert_Option_Timestamp_jni_optional_intermediate_output_niche_to_wire_744022bc4edbcd87(
                                    &mut env,
                                    __u0.timestamp,
                                ) {
                                    ::core::result::Result::Ok(__w) => __w,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<
                                                String,
                                            >>::from(__e.to_string()),
                                        );
                                    }
                                };
                                __enc7
                            };
                            let __cb0_obj8: jni::objects::JObject = {
                                let __enc8 = match __jni_out_convert_bool_to_wire_3ee62077915d5228(
                                    &mut env,
                                    __u0.express,
                                ) {
                                    ::core::result::Result::Ok(__w) => __w,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<
                                                String,
                                            >>::from(__e.to_string()),
                                        );
                                    }
                                };
                                match ::prebindgen_jni_runtime::box_jboolean(
                                    &mut env,
                                    __enc8,
                                ) {
                                    ::core::result::Result::Ok(__o) => __o,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<String>>::from(__e),
                                        );
                                    }
                                }
                            };
                            let __cb0_obj9: jni::objects::JObject = {
                                let __enc9 = match __jni_out_convert_Priority_to_wire_55b65fa623d4787e(
                                    &mut env,
                                    __u0.priority,
                                ) {
                                    ::core::result::Result::Ok(__w) => __w,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<
                                                String,
                                            >>::from(__e.to_string()),
                                        );
                                    }
                                };
                                match ::prebindgen_jni_runtime::box_jint(&mut env, __enc9) {
                                    ::core::result::Result::Ok(__o) => __o,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<String>>::from(__e),
                                        );
                                    }
                                }
                            };
                            let __cb0_obj10: jni::objects::JObject = {
                                let __enc10 = match __jni_out_convert_CongestionControl_to_wire_dd1f6047005395ca(
                                    &mut env,
                                    __u0.congestion_control,
                                ) {
                                    ::core::result::Result::Ok(__w) => __w,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<
                                                String,
                                            >>::from(__e.to_string()),
                                        );
                                    }
                                };
                                match ::prebindgen_jni_runtime::box_jint(
                                    &mut env,
                                    __enc10,
                                ) {
                                    ::core::result::Result::Ok(__o) => __o,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<String>>::from(__e),
                                        );
                                    }
                                }
                            };
                            let __cb0_obj12: jni::objects::JObject = {
                                let __enc12 = match __jni_out_convert_Reliability_to_wire_1173d3c71693bfba(
                                    &mut env,
                                    __u0.reliability,
                                ) {
                                    ::core::result::Result::Ok(__w) => __w,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<
                                                String,
                                            >>::from(__e.to_string()),
                                        );
                                    }
                                };
                                match ::prebindgen_jni_runtime::box_jint(
                                    &mut env,
                                    __enc12,
                                ) {
                                    ::core::result::Result::Ok(__o) => __o,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<String>>::from(__e),
                                        );
                                    }
                                }
                            };
                            let __cb0_obj13: jni::objects::JObject = {
                                let __enc13 = match __jni_out_convert_Option_SourceInfo_jni_optional_intermediate_output_niche_to_wire_c31f0eb6b4d779fc(
                                    &mut env,
                                    __u0.source_info,
                                ) {
                                    ::core::result::Result::Ok(__w) => __w,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<
                                                String,
                                            >>::from(__e.to_string()),
                                        );
                                    }
                                };
                                __enc13
                            };
                            let __cb0_obj3: jni::objects::JObject = {
                                let __h3: jni::sys::jlong = std::boxed::Box::into_raw(
                                    std::boxed::Box::new(__u0.payload),
                                ) as jni::sys::jlong;
                                match ::prebindgen_jni_runtime::box_jlong(&mut env, __h3) {
                                    ::core::result::Result::Ok(__o) => __o,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<
                                                String,
                                            >>::from(__e.to_string()),
                                        );
                                    }
                                }
                            };
                            let __cb0_obj11: jni::objects::JObject = {
                                match __u0.attachment {
                                    ::core::option::Option::Some(__n) => {
                                        let __h11: jni::sys::jlong = std::boxed::Box::into_raw(
                                            std::boxed::Box::new(__n),
                                        ) as jni::sys::jlong;
                                        match ::prebindgen_jni_runtime::box_jlong(&mut env, __h11) {
                                            ::core::result::Result::Ok(__o) => __o,
                                            ::core::result::Result::Err(__e) => {
                                                return ::core::result::Result::Err(
                                                    <__JniErr as ::core::convert::From<
                                                        String,
                                                    >>::from(__e.to_string()),
                                                );
                                            }
                                        }
                                    }
                                    ::core::option::Option::None => {
                                        jni::objects::JObject::null()
                                    }
                                }
                            };
                            (
                                __cb0_obj2,
                                __cb0_obj3,
                                __cb0_obj4,
                                __cb0_obj5,
                                __cb0_obj6,
                                __cb0_obj7,
                                __cb0_obj8,
                                __cb0_obj9,
                                __cb0_obj10,
                                __cb0_obj11,
                                __cb0_obj12,
                                __cb0_obj13,
                            )
                        }
                        ::core::option::Option::None => {
                            (
                                jni::objects::JObject::null(),
                                jni::objects::JObject::null(),
                                jni::objects::JObject::null(),
                                jni::objects::JObject::null(),
                                jni::objects::JObject::null(),
                                jni::objects::JObject::null(),
                                jni::objects::JObject::null(),
                                jni::objects::JObject::null(),
                                jni::objects::JObject::null(),
                                jni::objects::JObject::null(),
                                jni::objects::JObject::null(),
                                jni::objects::JObject::null(),
                            )
                        }
                    };
                    let __call_res: ::core::result::Result<(), __JniErr> = unsafe {
                        env.call_method_unchecked(
                            &callback_global_ref,
                            __invoke_id,
                            jni::signature::ReturnType::Primitive(
                                jni::signature::Primitive::Void,
                            ),
                            &[
                                jni::sys::jvalue {
                                    l: __cb0_obj0.as_raw(),
                                },
                                __cb0_obj1,
                                jni::sys::jvalue {
                                    l: __cb0_obj2.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj3.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj4.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj5.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj6.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj7.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj8.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj9.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj10.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj11.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj12.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj13.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj14.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj15.as_raw(),
                                },
                                jni::sys::jvalue {
                                    l: __cb0_obj16.as_raw(),
                                },
                            ],
                        )
                    }
                        .map(|_| ())
                        .map_err(|e| {
                            let _ = env.exception_describe();
                            <__JniErr as ::core::convert::From<
                                String,
                            >>::from(e.to_string())
                        });
                    __call_res?;
                    Ok(())
                })();
                let _ = unsafe { env.pop_local_frame(&jni::objects::JObject::null()) };
                __frame_res?;
                Ok(())
            })()
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn(Reply)"));
        })
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_impl_Fn_Sample_Send_Sync_static_de5b7c31ca7e8b9f<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<
    impl Fn(zenoh_flat::Sample) + Send + Sync + 'static,
    __JniErr,
> {
    Ok({
        use std::sync::Arc;
        let java_vm = Arc::new(
            env
                .get_java_vm()
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Unable to retrieve JVM: {}", e)))?,
        );
        let callback_global_ref = env
            .new_global_ref(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to global-ref callback: {}", e)))?;
        let __invoke_class = env
            .get_object_class(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(
                format!("Unable to get callback class for {}: {}", "Fn(Sample)", e),
            ))?;
        let __invoke_id = env
            .get_method_id(
                &__invoke_class,
                "run",
                "(Ljava/lang/String;JI[BILio/zenoh/jni/time/Timestamp;ZIILjava/lang/Long;ILio/zenoh/jni/sample/SourceInfo;)V",
            )
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn(Sample)", e)))?;
        Box::new(move |__cb_arg0: zenoh_flat::Sample| {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn(Sample)", e)))?;
                env.push_local_frame(30)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn(Sample)", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let __vf0 = zenoh_flat::sample_into_struct(__cb_arg0);
                    let __cb0_obj0: jni::objects::JObject = {
                        let __enc0 = match __jni_out_convert_jni_text_codec_borrowed_to_wire_bc4fe45698de9c2e(
                            &mut env,
                            zenoh_flat::keyexpr_as_str(&__vf0.key_expr),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc0.into()
                    };
                    let __cb0_obj2: jni::sys::jvalue = {
                        let __enc2 = match __jni_out_convert_u16_to_wire_279d2392219844b4(
                            &mut env,
                            zenoh_flat::encoding_get_id(&__vf0.encoding),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc2 }
                    };
                    let __cb0_obj3: jni::objects::JObject = {
                        let __enc3 = match __jni_out_convert_Option_Vec_u8_jni_optional_intermediate_output_niche_to_wire_beaf597ddc121688(
                            &mut env,
                            zenoh_flat::encoding_get_schema(&__vf0.encoding),
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc3.into()
                    };
                    let __cb0_obj4: jni::sys::jvalue = {
                        let __enc4 = match __jni_out_convert_SampleKind_to_wire_5850f5fbf98a86e6(
                            &mut env,
                            __vf0.kind,
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc4 }
                    };
                    let __cb0_obj5: jni::objects::JObject = {
                        let __enc5 = match __jni_out_convert_Option_Timestamp_jni_optional_intermediate_output_niche_to_wire_744022bc4edbcd87(
                            &mut env,
                            __vf0.timestamp,
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc5
                    };
                    let __cb0_obj6: jni::sys::jvalue = {
                        let __enc6 = match __jni_out_convert_bool_to_wire_3ee62077915d5228(
                            &mut env,
                            __vf0.express,
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { z: __enc6 }
                    };
                    let __cb0_obj7: jni::sys::jvalue = {
                        let __enc7 = match __jni_out_convert_Priority_to_wire_55b65fa623d4787e(
                            &mut env,
                            __vf0.priority,
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc7 }
                    };
                    let __cb0_obj8: jni::sys::jvalue = {
                        let __enc8 = match __jni_out_convert_CongestionControl_to_wire_dd1f6047005395ca(
                            &mut env,
                            __vf0.congestion_control,
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc8 }
                    };
                    let __cb0_obj10: jni::sys::jvalue = {
                        let __enc10 = match __jni_out_convert_Reliability_to_wire_1173d3c71693bfba(
                            &mut env,
                            __vf0.reliability,
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        jni::sys::jvalue { i: __enc10 }
                    };
                    let __cb0_obj11: jni::objects::JObject = {
                        let __enc11 = match __jni_out_convert_Option_SourceInfo_jni_optional_intermediate_output_niche_to_wire_c31f0eb6b4d779fc(
                            &mut env,
                            __vf0.source_info,
                        ) {
                            ::core::result::Result::Ok(__w) => __w,
                            ::core::result::Result::Err(__e) => {
                                return ::core::result::Result::Err(
                                    <__JniErr as ::core::convert::From<
                                        String,
                                    >>::from(__e.to_string()),
                                );
                            }
                        };
                        __enc11
                    };
                    let __cb0_obj1: jni::sys::jvalue = jni::sys::jvalue {
                        j: std::boxed::Box::into_raw(std::boxed::Box::new(__vf0.payload))
                            as jni::sys::jlong,
                    };
                    let __cb0_obj9: jni::objects::JObject = {
                        match __vf0.attachment {
                            ::core::option::Option::Some(__n) => {
                                let __h9: jni::sys::jlong = std::boxed::Box::into_raw(
                                    std::boxed::Box::new(__n),
                                ) as jni::sys::jlong;
                                match ::prebindgen_jni_runtime::box_jlong(&mut env, __h9) {
                                    ::core::result::Result::Ok(__o) => __o,
                                    ::core::result::Result::Err(__e) => {
                                        return ::core::result::Result::Err(
                                            <__JniErr as ::core::convert::From<
                                                String,
                                            >>::from(__e.to_string()),
                                        );
                                    }
                                }
                            }
                            ::core::option::Option::None => jni::objects::JObject::null(),
                        }
                    };
                    let __call_res: ::core::result::Result<(), __JniErr> = unsafe {
                        env.call_method_unchecked(
                            &callback_global_ref,
                            __invoke_id,
                            jni::signature::ReturnType::Primitive(
                                jni::signature::Primitive::Void,
                            ),
                            &[
                                jni::sys::jvalue {
                                    l: __cb0_obj0.as_raw(),
                                },
                                __cb0_obj1,
                                __cb0_obj2,
                                jni::sys::jvalue {
                                    l: __cb0_obj3.as_raw(),
                                },
                                __cb0_obj4,
                                jni::sys::jvalue {
                                    l: __cb0_obj5.as_raw(),
                                },
                                __cb0_obj6,
                                __cb0_obj7,
                                __cb0_obj8,
                                jni::sys::jvalue {
                                    l: __cb0_obj9.as_raw(),
                                },
                                __cb0_obj10,
                                jni::sys::jvalue {
                                    l: __cb0_obj11.as_raw(),
                                },
                            ],
                        )
                    }
                        .map(|_| ())
                        .map_err(|e| {
                            let _ = env.exception_describe();
                            <__JniErr as ::core::convert::From<
                                String,
                            >>::from(e.to_string())
                        });
                    __call_res?;
                    Ok(())
                })();
                let _ = unsafe { env.pop_local_frame(&jni::objects::JObject::null()) };
                __frame_res?;
                Ok(())
            })()
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn(Sample)"));
        })
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_impl_Fn_bool_Send_Sync_static_f22921bfbf64752f<
    'env,
    'v,
>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::objects::JObject<'v>,
) -> ::core::result::Result<impl Fn(bool) + Send + Sync + 'static, __JniErr> {
    Ok({
        use std::sync::Arc;
        let java_vm = Arc::new(
            env
                .get_java_vm()
                .map_err(|e| <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("Unable to retrieve JVM: {}", e)))?,
        );
        let callback_global_ref = env
            .new_global_ref(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to global-ref callback: {}", e)))?;
        let __invoke_class = env
            .get_object_class(&v)
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(
                format!("Unable to get callback class for {}: {}", "Fn(bool)", e),
            ))?;
        let __invoke_id = env
            .get_method_id(&__invoke_class, "run", "(Z)V")
            .map_err(|e| <__JniErr as ::core::convert::From<
                String,
            >>::from(format!("Unable to resolve run for {}: {}", "Fn(bool)", e)))?;
        Box::new(move |__cb_arg0: bool| {
            let _ = (|| -> ::core::result::Result<(), __JniErr> {
                let mut env = java_vm
                    .attach_current_thread_as_daemon()
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("Attach thread for {}: {}", "Fn(bool)", e)))?;
                env.push_local_frame(16)
                    .map_err(|e| <__JniErr as ::core::convert::From<
                        String,
                    >>::from(format!("push local frame for {}: {}", "Fn(bool)", e)))?;
                let __frame_res = (|| -> ::core::result::Result<(), __JniErr> {
                    let __cb0_enc = __jni_out_convert_bool_to_wire_3ee62077915d5228(
                        &mut env,
                        __cb_arg0,
                    )?;
                    let __call_res: ::core::result::Result<(), __JniErr> = unsafe {
                        env.call_method_unchecked(
                            &callback_global_ref,
                            __invoke_id,
                            jni::signature::ReturnType::Primitive(
                                jni::signature::Primitive::Void,
                            ),
                            &[jni::sys::jvalue { z: __cb0_enc }],
                        )
                    }
                        .map(|_| ())
                        .map_err(|e| {
                            let _ = env.exception_describe();
                            <__JniErr as ::core::convert::From<
                                String,
                            >>::from(e.to_string())
                        });
                    __call_res?;
                    Ok(())
                })();
                let _ = unsafe { env.pop_local_frame(&jni::objects::JObject::null()) };
                __frame_res?;
                Ok(())
            })()
                .map_err(|e| tracing::error!("{} callback error: {e}", "Fn(bool)"));
        })
    })
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_u16_fc24f387ddcec321<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jint,
) -> ::core::result::Result<u16, __JniErr> {
    Ok(
        ::core::primitive::u16::try_from(*v)
            .map_err(|_| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("u16 input out of range: {}", * v))
            })?,
    )
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_u16_to_wire_279d2392219844b4<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: u16,
) -> ::core::result::Result<jni::sys::jint, __JniErr> {
    Ok(v as jni::sys::jint)
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_in_convert_wire_to_u32_25dff6d476799035<'env, 'v>(
    env: &mut jni::JNIEnv<'env>,
    v: &jni::sys::jlong,
) -> ::core::result::Result<u32, __JniErr> {
    Ok(
        ::core::primitive::u32::try_from(*v)
            .map_err(|_| {
                <__JniErr as ::core::convert::From<
                    String,
                >>::from(format!("u32 input out of range: {}", * v))
            })?,
    )
}
#[allow(
    non_snake_case,
    unused_mut,
    unused_variables,
    unused_braces,
    unused_parens,
    dead_code,
    clippy::useless_conversion,
    clippy::needless_question_mark,
    clippy::let_and_return,
    clippy::nonminimal_bool,
    clippy::eq_op
)]
pub(crate) unsafe fn __jni_out_convert_u32_to_wire_b6376ae826304960<'a>(
    env: &mut jni::JNIEnv<'a>,
    v: u32,
) -> ::core::result::Result<jni::sys::jlong, __JniErr> {
    Ok(v as jni::sys::jlong)
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_advancedPublisherDeclareBackgroundMatchingListener<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    publisher: jni::sys::jlong,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let publisher = match __jni_in_convert_wire_to_AdvancedPublisher_jni_handle_codec_borrow_input_298801d5fc997bb9(
        &mut env,
        &publisher,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let callback = match __jni_in_convert_wire_to_impl_Fn_bool_Send_Sync_static_f22921bfbf64752f(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let on_close = match __jni_in_convert_wire_to_impl_Fn_Send_Sync_static_6d213555cb0eda7b(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::advanced_publisher_declare_background_matching_listener(
        &publisher,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_advancedPublisherDeclareMatchingListener<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    publisher: jni::sys::jlong,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let publisher = match __jni_in_convert_wire_to_AdvancedPublisher_jni_handle_codec_borrow_input_298801d5fc997bb9(
        &mut env,
        &publisher,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let callback = match __jni_in_convert_wire_to_impl_Fn_bool_Send_Sync_static_f22921bfbf64752f(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let on_close = match __jni_in_convert_wire_to_impl_Fn_Send_Sync_static_6d213555cb0eda7b(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::advanced_publisher_declare_matching_listener(
        &publisher,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_MatchingListener_jni_handle_codec_own_output_to_wire_7ca5e1ac9818f8d1(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_advancedPublisherDelete<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    publisher: jni::sys::jlong,
    attachment: jni::objects::JByteArray<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let publisher = match __jni_in_convert_wire_to_AdvancedPublisher_jni_handle_codec_borrow_input_298801d5fc997bb9(
        &mut env,
        &publisher,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_attachment = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::advanced_publisher_delete(
        &publisher,
        __folded_attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_advancedPublisherMatchingStatus<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    publisher: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jboolean {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let publisher = match __jni_in_convert_wire_to_AdvancedPublisher_jni_handle_codec_borrow_input_298801d5fc997bb9(
        &mut env,
        &publisher,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = match zenoh_flat::advanced_publisher_matching_status(&publisher) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jboolean;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jboolean;
        }
    };
    match __jni_out_convert_bool_to_wire_3ee62077915d5228(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jboolean
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_advancedPublisherPut<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    publisher: jni::sys::jlong,
    payload: jni::objects::JByteArray<'a>,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JByteArray<'a>,
    encoding_1: jni::sys::jlong,
    attachment: jni::objects::JByteArray<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let publisher = match __jni_in_convert_wire_to_AdvancedPublisher_jni_handle_codec_borrow_input_298801d5fc997bb9(
        &mut env,
        &publisher,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_payload = match __jni_in_convert_wire_to_Vec_u8_80984e9556387695(
        &mut env,
        &payload,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_payload = match ::core::result::Result::Ok(
        zenoh_flat::zbytes_new_from_vec(__exp_payload),
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &encoding_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_0: ::core::option::Option<u16> = match __jni_in_convert_wire_to_Option_u16_jni_optional_intermediate_input_gated_ff09dbf66b457369(
        &mut env,
        (encoding_0_0_present, encoding_0_0_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_1 = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_1 = match __jni_in_convert_wire_to_Option_Encoding_1ea8d6cdb046d07b(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_attachment = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::advanced_publisher_put(
        &publisher,
        __folded_payload,
        __folded_encoding.as_ref(),
        __folded_attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_advancedSubscriberDeclareBackgroundDetectPublishersSubscriber<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    subscriber: jni::sys::jlong,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    history_present: jni::sys::jboolean,
    history_value: jni::sys::jboolean,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let subscriber = match __jni_in_convert_wire_to_AdvancedSubscriber_jni_handle_codec_borrow_input_0931ab6ee88953e1(
        &mut env,
        &subscriber,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let callback = match __jni_in_convert_wire_to_impl_Fn_Sample_Send_Sync_static_de5b7c31ca7e8b9f(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let on_close = match __jni_in_convert_wire_to_impl_Fn_Send_Sync_static_6d213555cb0eda7b(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let history = match __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d(
        &mut env,
        (history_present, history_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::advanced_subscriber_declare_background_detect_publishers_subscriber(
        &subscriber,
        callback,
        on_close,
        history,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_advancedSubscriberDeclareBackgroundSampleMissListener<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    subscriber: jni::sys::jlong,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let subscriber = match __jni_in_convert_wire_to_AdvancedSubscriber_jni_handle_codec_borrow_input_0931ab6ee88953e1(
        &mut env,
        &subscriber,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let callback = match __jni_in_convert_wire_to_impl_Fn_Miss_Send_Sync_static_4ae3837246d447c7(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let on_close = match __jni_in_convert_wire_to_impl_Fn_Send_Sync_static_6d213555cb0eda7b(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::advanced_subscriber_declare_background_sample_miss_listener(
        &subscriber,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_advancedSubscriberDeclareDetectPublishersSubscriber<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    subscriber: jni::sys::jlong,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    history_present: jni::sys::jboolean,
    history_value: jni::sys::jboolean,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let subscriber = match __jni_in_convert_wire_to_AdvancedSubscriber_jni_handle_codec_borrow_input_0931ab6ee88953e1(
        &mut env,
        &subscriber,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let callback = match __jni_in_convert_wire_to_impl_Fn_Sample_Send_Sync_static_de5b7c31ca7e8b9f(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let on_close = match __jni_in_convert_wire_to_impl_Fn_Send_Sync_static_6d213555cb0eda7b(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let history = match __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d(
        &mut env,
        (history_present, history_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::advanced_subscriber_declare_detect_publishers_subscriber(
        &subscriber,
        callback,
        on_close,
        history,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_Subscriber_jni_handle_codec_own_output_to_wire_4552f6e215a6371c(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_advancedSubscriberDeclareSampleMissListener<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    subscriber: jni::sys::jlong,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let subscriber = match __jni_in_convert_wire_to_AdvancedSubscriber_jni_handle_codec_borrow_input_0931ab6ee88953e1(
        &mut env,
        &subscriber,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let callback = match __jni_in_convert_wire_to_impl_Fn_Miss_Send_Sync_static_4ae3837246d447c7(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let on_close = match __jni_in_convert_wire_to_impl_Fn_Send_Sync_static_6d213555cb0eda7b(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::advanced_subscriber_declare_sample_miss_listener(
        &subscriber,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_SampleMissListener_jni_handle_codec_own_output_to_wire_1b1d4f70a3effe1c(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_configGetJson<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    c: jni::sys::jlong,
    key: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let c = match __jni_in_convert_wire_to_Config_jni_handle_codec_borrow_input_77739071df1dbe47(
        &mut env,
        &c,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let key = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &key,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = match zenoh_flat::config_get_json(&c, &key) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return jni::objects::JObject::null().into();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return jni::objects::JObject::null().into();
        }
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_configInsertJson5<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    c: jni::sys::jlong,
    key: jni::objects::JString<'a>,
    value: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let mut c = match __jni_in_convert_wire_to_Config_jni_handle_codec_borrow_input_77739071df1dbe47(
        &mut env,
        &c,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let key = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &key,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let value = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &value,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::config_insert_json5(&mut c, &key, &value) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_configNewClone<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    c: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let c = match __jni_in_convert_wire_to_Config_jni_handle_codec_borrow_input_77739071df1dbe47(
        &mut env,
        &c,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::config_new_clone(&c);
    match __jni_out_convert_Config_jni_handle_codec_own_output_to_wire_3d8eeac173220d60(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_configNewDefault<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::config_new_default();
    match __jni_out_convert_Config_jni_handle_codec_own_output_to_wire_3d8eeac173220d60(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_configNewFromFile<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    path: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let path = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &path,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::config_new_from_file(&path) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_Config_jni_handle_codec_own_output_to_wire_3d8eeac173220d60(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_configNewFromJson5<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::config_new_from_json5(&s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_Config_jni_handle_codec_own_output_to_wire_3d8eeac173220d60(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_configNewFromYaml<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::config_new_from_yaml(&s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_Config_jni_handle_codec_own_output_to_wire_3d8eeac173220d60(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingGetId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match __jni_in_convert_wire_to_Encoding_jni_handle_codec_borrow_input_11ed5ebde83c779c(
        &mut env,
        &e,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::encoding_get_id(&e);
    match __jni_out_convert_u16_to_wire_279d2392219844b4(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingGetSchema<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JByteArray<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match __jni_in_convert_wire_to_Encoding_jni_handle_codec_borrow_input_11ed5ebde83c779c(
        &mut env,
        &e,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::encoding_get_schema(&e);
    match __jni_out_convert_Option_Vec_u8_jni_optional_intermediate_output_niche_to_wire_beaf597ddc121688(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingNewClone<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match __jni_in_convert_wire_to_Encoding_jni_handle_codec_borrow_input_11ed5ebde83c779c(
        &mut env,
        &e,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::encoding_new_clone(&e);
    match __jni_out_convert_Encoding_jni_handle_codec_own_output_to_wire_832b680de1995a2b(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingNewFromId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    id: jni::sys::jint,
    schema: jni::objects::JByteArray<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let id = match __jni_in_convert_wire_to_u16_fc24f387ddcec321(&mut env, &id) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let schema = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &schema,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::encoding_new_from_id(id, schema);
    match __jni_out_convert_Encoding_jni_handle_codec_own_output_to_wire_832b680de1995a2b(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingNewFromString<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::encoding_new_from_string(s);
    match __jni_out_convert_Encoding_jni_handle_codec_own_output_to_wire_832b680de1995a2b(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingNewWithSchema<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e_sel: jni::sys::jint,
    e_0_0_present: jni::sys::jboolean,
    e_0_0_value: jni::sys::jint,
    e_0_1: jni::objects::JByteArray<'a>,
    e_1: jni::sys::jlong,
    schema: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __exp_e_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &e_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_e_0_0: ::core::option::Option<u16> = match __jni_in_convert_wire_to_Option_u16_jni_optional_intermediate_input_gated_ff09dbf66b457369(
        &mut env,
        (e_0_0_present, e_0_0_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_e_0_1 = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &e_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_e_1 = match __jni_in_convert_wire_to_Option_Encoding_1ea8d6cdb046d07b(
        &mut env,
        &e_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_e = match {
        match __exp_e_sel {
            0i32 => {
                match __exp_e_0_0 {
                    ::core::option::Option::Some(__p0) => {
                        ::core::result::Result::Ok(
                            zenoh_flat::encoding_new_from_id(__p0, __exp_e_0_1),
                        )
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_e_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let schema = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &schema,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::encoding_new_with_schema(&__folded_e, schema);
    match __jni_out_convert_Encoding_jni_handle_codec_own_output_to_wire_832b680de1995a2b(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_encodingToString<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match __jni_in_convert_wire_to_Encoding_jni_handle_codec_borrow_input_11ed5ebde83c779c(
        &mut env,
        &e,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::encoding_to_string(&e);
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_helloGetLocators<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    h: jni::sys::jlong,
    __acc: jni::objects::JObject<'a>,
    __fold: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let h = match __jni_in_convert_wire_to_Hello_jni_handle_codec_borrow_input_21e25a9262ca3e93(
        &mut env,
        &h,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/StringFolder";
    const __CB_DESCR: &str = "(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;";
    let __vec = zenoh_flat::hello_get_locators(&h);
    let mut __acc = __acc;
    for __elem in __vec.into_iter() {
        let __enc = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
            &mut env,
            __elem,
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        let __obj: jni::objects::JObject = __enc.into();
        __acc = match __CB_MID
            .call_object(
                &mut env,
                __CB_FQN,
                "run",
                __CB_DESCR,
                &__fold,
                &[
                    jni::sys::jvalue {
                        l: __acc.as_raw(),
                    },
                    jni::sys::jvalue {
                        l: __obj.as_raw(),
                    },
                ],
            )
        {
            ::core::result::Result::Ok(__o) => __o,
            ::core::result::Result::Err(__e) => {
                let _ = env.exception_describe();
                let __e2 = <__JniErr as ::core::convert::From<
                    String,
                >>::from(__e.to_string());
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e2.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
    }
    __acc
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_helloGetWhatami<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    h: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let h = match __jni_in_convert_wire_to_Hello_jni_handle_codec_borrow_input_21e25a9262ca3e93(
        &mut env,
        &h,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::hello_get_whatami(&h);
    match __jni_out_convert_WhatAmI_to_wire_907d4a5187d437e6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_helloGetZid<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    h: jni::sys::jlong,
    __builder: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let h = match __jni_in_convert_wire_to_Hello_jni_handle_codec_borrow_input_21e25a9262ca3e93(
        &mut env,
        &h,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/config/ZenohIdBuilder";
    const __CB_DESCR: &str = "([B)Ljava/lang/Object;";
    let __out = zenoh_flat::hello_get_zid(&h);
    let (__chain_wire0,) = match __jni_out_convert_ZenohId_jni_product_intermediate_tuple_to_wire_c7bb0eb9058fd8ec(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__intermediate) => __intermediate,
        ::core::result::Result::Err(__chain_error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__chain_error.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __obj0: jni::objects::JObject = __chain_wire0.into();
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[
                jni::sys::jvalue {
                    l: __obj0.as_raw(),
                },
            ],
        )
    {
        ::core::result::Result::Ok(__o) => __o,
        ::core::result::Result::Err(__e) => {
            let _ = env.exception_describe();
            let __e2 = <__JniErr as ::core::convert::From<
                String,
            >>::from(__e.to_string());
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e2.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_initAndroidLogs<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    filter: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let filter = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &filter,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = zenoh_flat::init_android_logs(&filter);
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_initZenohLogsFromEnvOr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    fallback_filter: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let fallback_filter = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &fallback_filter,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = zenoh_flat::init_zenoh_logs_from_env_or(&fallback_filter);
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprAsStr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    ke: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let ke = match __jni_in_convert_wire_to_KeyExpr_jni_handle_codec_borrow_input_d9700f3a873cda93(
        &mut env,
        &ke,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::keyexpr_as_str(&ke);
    match __jni_out_convert_jni_text_codec_borrowed_to_wire_bc4fe45698de9c2e(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprIncludes<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    a: jni::sys::jlong,
    b_sel: jni::sys::jint,
    b_0: jni::objects::JString<'a>,
    b_1: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jboolean {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let a = match __jni_in_convert_wire_to_KeyExpr_jni_handle_codec_borrow_input_d9700f3a873cda93(
        &mut env,
        &a,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &b_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &b_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_1 = match __jni_in_convert_wire_to_Option_KeyExpr_75b6fc876caf57d7(
        &mut env,
        &b_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __folded_b = match {
        match __exp_b_sel {
            0i32 => {
                match __exp_b_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_b_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::keyexpr_includes(&a, &__folded_b);
    match __jni_out_convert_bool_to_wire_3ee62077915d5228(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jboolean
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprIntersects<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    a: jni::sys::jlong,
    b_sel: jni::sys::jint,
    b_0: jni::objects::JString<'a>,
    b_1: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jboolean {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let a = match __jni_in_convert_wire_to_KeyExpr_jni_handle_codec_borrow_input_d9700f3a873cda93(
        &mut env,
        &a,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &b_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &b_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __exp_b_1 = match __jni_in_convert_wire_to_Option_KeyExpr_75b6fc876caf57d7(
        &mut env,
        &b_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __folded_b = match {
        match __exp_b_sel {
            0i32 => {
                match __exp_b_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_b_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::keyexpr_intersects(&a, &__folded_b);
    match __jni_out_convert_bool_to_wire_3ee62077915d5228(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jboolean
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprNewAutocanonize<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::keyexpr_new_autocanonize(s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_KeyExpr_jni_handle_codec_own_output_to_wire_2022bfd454e7e1bc(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprNewClone<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    ke: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let ke = match __jni_in_convert_wire_to_KeyExpr_jni_handle_codec_borrow_input_d9700f3a873cda93(
        &mut env,
        &ke,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::keyexpr_new_clone(&ke);
    match __jni_out_convert_KeyExpr_jni_handle_codec_own_output_to_wire_2022bfd454e7e1bc(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprNewConcat<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    a_sel: jni::sys::jint,
    a_0: jni::objects::JString<'a>,
    a_1: jni::sys::jlong,
    b: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __exp_a_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &a_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_a_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &a_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_a_1 = match __jni_in_convert_wire_to_Option_KeyExpr_75b6fc876caf57d7(
        &mut env,
        &a_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_a = match {
        match __exp_a_sel {
            0i32 => {
                match __exp_a_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_a_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let b = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &b,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::keyexpr_new_concat(&__folded_a, b) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_KeyExpr_jni_handle_codec_own_output_to_wire_2022bfd454e7e1bc(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprNewJoin<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    a_sel: jni::sys::jint,
    a_0: jni::objects::JString<'a>,
    a_1: jni::sys::jlong,
    b: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __exp_a_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &a_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_a_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &a_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_a_1 = match __jni_in_convert_wire_to_Option_KeyExpr_75b6fc876caf57d7(
        &mut env,
        &a_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_a = match {
        match __exp_a_sel {
            0i32 => {
                match __exp_a_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_a_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let b = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &b,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::keyexpr_new_join(&__folded_a, b) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_KeyExpr_jni_handle_codec_own_output_to_wire_2022bfd454e7e1bc(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprNewTryFrom<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::keyexpr_new_try_from(s) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_KeyExpr_jni_handle_codec_own_output_to_wire_2022bfd454e7e1bc(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprRelationTo<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    a: jni::sys::jlong,
    b_sel: jni::sys::jint,
    b_0: jni::objects::JString<'a>,
    b_1: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let a = match __jni_in_convert_wire_to_KeyExpr_jni_handle_codec_borrow_input_d9700f3a873cda93(
        &mut env,
        &a,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __exp_b_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &b_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __exp_b_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &b_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __exp_b_1 = match __jni_in_convert_wire_to_Option_KeyExpr_75b6fc876caf57d7(
        &mut env,
        &b_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __folded_b = match {
        match __exp_b_sel {
            0i32 => {
                match __exp_b_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_b_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::keyexpr_relation_to(&a, &__folded_b);
    match __jni_out_convert_SetIntersectionLevel_to_wire_7d91a182acc0b634(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_keyexprToString<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    ke: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let ke = match __jni_in_convert_wire_to_KeyExpr_jni_handle_codec_borrow_input_d9700f3a873cda93(
        &mut env,
        &ke,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::keyexpr_to_string(&ke);
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_livelinessDeclareSubscriber<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    history: jni::sys::jboolean,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &key_expr_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match __jni_in_convert_wire_to_Option_KeyExpr_jni_optional_intermediate_input_niche_bbf13322cceb1d7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let history = match __jni_in_convert_wire_to_bool_1be2f6c32f925207(
        &mut env,
        &history,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let callback = match __jni_in_convert_wire_to_impl_Fn_Sample_Send_Sync_static_de5b7c31ca7e8b9f(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let on_close = match __jni_in_convert_wire_to_impl_Fn_Send_Sync_static_6d213555cb0eda7b(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::liveliness_declare_subscriber(
        &session,
        __folded_key_expr,
        history,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_Subscriber_jni_handle_codec_own_output_to_wire_4552f6e215a6371c(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_livelinessDeclareToken<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &key_expr_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match __jni_in_convert_wire_to_Option_KeyExpr_jni_optional_intermediate_input_niche_bbf13322cceb1d7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::liveliness_declare_token(&session, __folded_key_expr) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_LivelinessToken_jni_handle_codec_own_output_to_wire_9a3ca92ce7f5264b(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_livelinessGet<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    timeout_ms: jni::sys::jlong,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &key_expr_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_1 = match __jni_in_convert_wire_to_Option_KeyExpr_75b6fc876caf57d7(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let timeout_ms = match __jni_in_convert_wire_to_i64_da07d745d9e26f71(
        &mut env,
        &timeout_ms,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let callback = match __jni_in_convert_wire_to_impl_Fn_Reply_Send_Sync_static_8acaa3be44c06271(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let on_close = match __jni_in_convert_wire_to_impl_Fn_Send_Sync_static_6d213555cb0eda7b(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::liveliness_get(
        &session,
        &__folded_key_expr,
        timeout_ms,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_open<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    config: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let config = match __jni_in_convert_wire_to_Config_jni_handle_codec_consume_input_76dd1a8cf1a45c64(
        &mut env,
        &config,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::open(config) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_Session_jni_handle_codec_own_output_to_wire_cdc63163a43ff710(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_parametersContainsKey<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    k: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jboolean {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let k = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &k,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::parameters_contains_key(&s, &k);
    match __jni_out_convert_bool_to_wire_3ee62077915d5228(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jboolean
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_parametersExtend<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    other: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let other = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &other,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::parameters_extend(&s, &other);
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_parametersGet<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    k: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let k = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &k,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::parameters_get(&s, &k);
    match __jni_out_convert_Option_String_jni_optional_intermediate_output_niche_to_wire_72d3a6d24cbddfc8(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_parametersInsert<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    k: jni::objects::JString<'a>,
    v: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let k = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &k,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let v = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &v,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::parameters_insert(&s, &k, &v);
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_parametersIsWellFormed<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jboolean {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::parameters_is_well_formed(&s);
    match __jni_out_convert_bool_to_wire_3ee62077915d5228(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jboolean
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_parametersRemove<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    k: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let k = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &k,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::parameters_remove(&s, &k);
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_parametersValues<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::objects::JString<'a>,
    k: jni::objects::JString<'a>,
    __acc: jni::objects::JObject<'a>,
    __fold: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let k = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &k,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/StringFolder";
    const __CB_DESCR: &str = "(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;";
    let __vec = zenoh_flat::parameters_values(&s, &k);
    let mut __acc = __acc;
    for __elem in __vec.into_iter() {
        let __enc = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
            &mut env,
            __elem,
        ) {
            ::core::result::Result::Ok(__w) => __w,
            ::core::result::Result::Err(__e) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        let __obj: jni::objects::JObject = __enc.into();
        __acc = match __CB_MID
            .call_object(
                &mut env,
                __CB_FQN,
                "run",
                __CB_DESCR,
                &__fold,
                &[
                    jni::sys::jvalue {
                        l: __acc.as_raw(),
                    },
                    jni::sys::jvalue {
                        l: __obj.as_raw(),
                    },
                ],
            )
        {
            ::core::result::Result::Ok(__o) => __o,
            ::core::result::Result::Err(__e) => {
                let _ = env.exception_describe();
                let __e2 = <__JniErr as ::core::convert::From<
                    String,
                >>::from(__e.to_string());
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e2.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
    }
    __acc
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_publisherDelete<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    publisher: jni::sys::jlong,
    attachment: jni::objects::JByteArray<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let publisher = match __jni_in_convert_wire_to_Publisher_jni_handle_codec_borrow_input_de8fa64048befc2f(
        &mut env,
        &publisher,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_attachment = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::publisher_delete(&publisher, __folded_attachment) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_publisherPut<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    publisher: jni::sys::jlong,
    payload: jni::objects::JByteArray<'a>,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JByteArray<'a>,
    encoding_1: jni::sys::jlong,
    attachment: jni::objects::JByteArray<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let publisher = match __jni_in_convert_wire_to_Publisher_jni_handle_codec_borrow_input_de8fa64048befc2f(
        &mut env,
        &publisher,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_payload = match __jni_in_convert_wire_to_Vec_u8_80984e9556387695(
        &mut env,
        &payload,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_payload = match ::core::result::Result::Ok(
        zenoh_flat::zbytes_new_from_vec(__exp_payload),
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &encoding_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_0: ::core::option::Option<u16> = match __jni_in_convert_wire_to_Option_u16_jni_optional_intermediate_input_gated_ff09dbf66b457369(
        &mut env,
        (encoding_0_0_present, encoding_0_0_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_1 = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_1 = match __jni_in_convert_wire_to_Option_Encoding_1ea8d6cdb046d07b(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_attachment = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::publisher_put(
        &publisher,
        __folded_payload,
        __folded_encoding.as_ref(),
        __folded_attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_querierGet<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    querier: jni::sys::jlong,
    parameters: jni::objects::JString<'a>,
    payload: jni::objects::JByteArray<'a>,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JByteArray<'a>,
    encoding_1: jni::sys::jlong,
    attachment: jni::objects::JByteArray<'a>,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let querier = match __jni_in_convert_wire_to_Querier_jni_handle_codec_borrow_input_7d494263029d7b3c(
        &mut env,
        &querier,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let parameters = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &parameters,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_payload = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &payload,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_payload = match match __exp_payload {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &encoding_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_0: ::core::option::Option<u16> = match __jni_in_convert_wire_to_Option_u16_jni_optional_intermediate_input_gated_ff09dbf66b457369(
        &mut env,
        (encoding_0_0_present, encoding_0_0_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_1 = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_1 = match __jni_in_convert_wire_to_Option_Encoding_1ea8d6cdb046d07b(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_attachment = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let callback = match __jni_in_convert_wire_to_impl_Fn_Reply_Send_Sync_static_8acaa3be44c06271(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let on_close = match __jni_in_convert_wire_to_impl_Fn_Send_Sync_static_6d213555cb0eda7b(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::querier_get(
        &querier,
        parameters,
        __folded_payload,
        __folded_encoding.as_ref(),
        __folded_attachment,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryGetAcceptsReplies<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    q: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match __jni_in_convert_wire_to_Query_jni_handle_codec_borrow_input_0edb5f83bccc458b(
        &mut env,
        &q,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::query_get_accepts_replies(&q);
    match __jni_out_convert_ReplyKeyExpr_to_wire_fed1222b1d494b5e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryGetAttachment<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    q: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match __jni_in_convert_wire_to_Query_jni_handle_codec_borrow_input_0edb5f83bccc458b(
        &mut env,
        &q,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::query_get_attachment(&q);
    match __jni_out_convert_Option_ZBytes_jni_optional_intermediate_output_niche_to_wire_1562c928235871aa(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryGetEncoding<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    q: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match __jni_in_convert_wire_to_Query_jni_handle_codec_borrow_input_0edb5f83bccc458b(
        &mut env,
        &q,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::query_get_encoding(&q);
    match __jni_out_convert_Option_Encoding_jni_optional_intermediate_output_niche_to_wire_d5cdae9fd96149aa(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryGetKeyExpr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    q: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match __jni_in_convert_wire_to_Query_jni_handle_codec_borrow_input_0edb5f83bccc458b(
        &mut env,
        &q,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::query_get_key_expr(&q);
    match __jni_out_convert_KeyExpr_jni_handle_codec_clone_output_to_wire_76ff23f685cd5e3d(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryGetParameters<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    q: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match __jni_in_convert_wire_to_Query_jni_handle_codec_borrow_input_0edb5f83bccc458b(
        &mut env,
        &q,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::query_get_parameters(&q);
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryGetPayload<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    q: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let q = match __jni_in_convert_wire_to_Query_jni_handle_codec_borrow_input_0edb5f83bccc458b(
        &mut env,
        &q,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::query_get_payload(&q);
    match __jni_out_convert_Option_ZBytes_jni_optional_intermediate_output_niche_to_wire_1562c928235871aa(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryReplyDelete<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    query: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    timestamp_present: jni::sys::jboolean,
    timestamp_ntp64: jni::sys::jlong,
    timestamp_id: jni::objects::JByteArray<'a>,
    attachment: jni::objects::JByteArray<'a>,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let query = match __jni_in_convert_wire_to_Query_jni_handle_codec_borrow_input_0edb5f83bccc458b(
        &mut env,
        &query,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &key_expr_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_1 = match __jni_in_convert_wire_to_Option_KeyExpr_75b6fc876caf57d7(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let timestamp = match __jni_in_convert_wire_to_Option_Timestamp_jni_optional_intermediate_input_gated_0e1cf2a3b10b443d(
        &mut env,
        (timestamp_present, (timestamp_ntp64, timestamp_id)),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let __exp_attachment = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let express = match __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d(
        &mut env,
        (express_present, express_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::query_reply_delete(
        &query,
        &__folded_key_expr,
        timestamp,
        __folded_attachment,
        express,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryReplyError<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    query: jni::sys::jlong,
    payload: jni::objects::JByteArray<'a>,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JByteArray<'a>,
    encoding_1: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let query = match __jni_in_convert_wire_to_Query_jni_handle_codec_borrow_input_0edb5f83bccc458b(
        &mut env,
        &query,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_payload = match __jni_in_convert_wire_to_Vec_u8_80984e9556387695(
        &mut env,
        &payload,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_payload = match ::core::result::Result::Ok(
        zenoh_flat::zbytes_new_from_vec(__exp_payload),
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &encoding_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_0: ::core::option::Option<u16> = match __jni_in_convert_wire_to_Option_u16_jni_optional_intermediate_input_gated_ff09dbf66b457369(
        &mut env,
        (encoding_0_0_present, encoding_0_0_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_1 = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_1 = match __jni_in_convert_wire_to_Option_Encoding_1ea8d6cdb046d07b(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::query_reply_error(
        &query,
        __folded_payload,
        __folded_encoding.as_ref(),
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryReplySample<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    query: jni::sys::jlong,
    sample: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let query = match __jni_in_convert_wire_to_Query_jni_handle_codec_borrow_input_0edb5f83bccc458b(
        &mut env,
        &query,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let sample = match __jni_in_convert_wire_to_Sample_jni_handle_codec_consume_input_ca6bc567cd12e1fc(
        &mut env,
        &sample,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::query_reply_sample(&query, sample) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_queryReplySuccess<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    query: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    payload: jni::objects::JByteArray<'a>,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JByteArray<'a>,
    encoding_1: jni::sys::jlong,
    timestamp_present: jni::sys::jboolean,
    timestamp_ntp64: jni::sys::jlong,
    timestamp_id: jni::objects::JByteArray<'a>,
    attachment: jni::objects::JByteArray<'a>,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let query = match __jni_in_convert_wire_to_Query_jni_handle_codec_borrow_input_0edb5f83bccc458b(
        &mut env,
        &query,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &key_expr_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_1 = match __jni_in_convert_wire_to_Option_KeyExpr_75b6fc876caf57d7(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_payload = match __jni_in_convert_wire_to_Vec_u8_80984e9556387695(
        &mut env,
        &payload,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_payload = match ::core::result::Result::Ok(
        zenoh_flat::zbytes_new_from_vec(__exp_payload),
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &encoding_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_0: ::core::option::Option<u16> = match __jni_in_convert_wire_to_Option_u16_jni_optional_intermediate_input_gated_ff09dbf66b457369(
        &mut env,
        (encoding_0_0_present, encoding_0_0_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_1 = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_1 = match __jni_in_convert_wire_to_Option_Encoding_1ea8d6cdb046d07b(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let timestamp = match __jni_in_convert_wire_to_Option_Timestamp_jni_optional_intermediate_input_gated_0e1cf2a3b10b443d(
        &mut env,
        (timestamp_present, (timestamp_ntp64, timestamp_id)),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let __exp_attachment = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let express = match __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d(
        &mut env,
        (express_present, express_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::query_reply_success(
        &query,
        &__folded_key_expr,
        __folded_payload,
        __folded_encoding.as_ref(),
        timestamp,
        __folded_attachment,
        express,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_replyErrorGetEncoding<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match __jni_in_convert_wire_to_ReplyError_jni_handle_codec_borrow_input_622062d39f49b357(
        &mut env,
        &e,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::reply_error_get_encoding(&e);
    match __jni_out_convert_Encoding_jni_handle_codec_clone_output_to_wire_ec144ca078ed717e(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_replyErrorGetPayload<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match __jni_in_convert_wire_to_ReplyError_jni_handle_codec_borrow_input_622062d39f49b357(
        &mut env,
        &e,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::reply_error_get_payload(&e);
    match __jni_out_convert_ZBytes_jni_handle_codec_clone_output_to_wire_631cd85333755af2(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_replyErrorGetTimestampStack<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    e: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let e = match __jni_in_convert_wire_to_ReplyError_jni_handle_codec_borrow_input_622062d39f49b357(
        &mut env,
        &e,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::reply_error_get_timestamp_stack(&e);
    match __jni_out_convert_Option_TimestampStack_jni_optional_intermediate_output_niche_to_wire_9e52e4193d223461(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_replyGetErr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    r: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let r = match __jni_in_convert_wire_to_Reply_jni_handle_codec_borrow_input_7db518929d6a9b8f(
        &mut env,
        &r,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::reply_get_err(&r);
    match __jni_out_convert_Option_ReplyError_jni_optional_intermediate_output_niche_to_wire_bc4aafd0982f3eab(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_replyGetReplierId<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    r: jni::sys::jlong,
    __builder: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let r = match __jni_in_convert_wire_to_Reply_jni_handle_codec_borrow_input_7db518929d6a9b8f(
        &mut env,
        &r,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/pubsub/EntityGlobalIdBuilder";
    const __CB_DESCR: &str = "([BJ)Ljava/lang/Object;";
    let __out = zenoh_flat::reply_get_replier_id(&r);
    let (__chain_present, ((__chain_wire0,), __chain_wire1)) = match __jni_out_convert_Option_EntityGlobalId_jni_optional_intermediate_output_gated_to_wire_dd03e8279dedf2b8(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__intermediate) => __intermediate,
        ::core::result::Result::Err(__chain_error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__chain_error.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __obj0: jni::objects::JObject = __chain_wire0.into();
    let __obj1 = jni::sys::jvalue {
        j: __chain_wire1,
    };
    if __chain_present != 0 {
        match __CB_MID
            .call_object(
                &mut env,
                __CB_FQN,
                "run",
                __CB_DESCR,
                &__builder,
                &[
                    jni::sys::jvalue {
                        l: __obj0.as_raw(),
                    },
                    __obj1,
                ],
            )
        {
            ::core::result::Result::Ok(__o) => __o,
            ::core::result::Result::Err(__e) => {
                let _ = env.exception_describe();
                let __e2 = <__JniErr as ::core::convert::From<
                    String,
                >>::from(__e.to_string());
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e2.to_string(),
                );
                jni::objects::JObject::null().into()
            }
        }
    } else {
        jni::objects::JObject::null().into()
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_replyGetSample<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    r: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let r = match __jni_in_convert_wire_to_Reply_jni_handle_codec_borrow_input_7db518929d6a9b8f(
        &mut env,
        &r,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::reply_get_sample(&r);
    match __jni_out_convert_Option_Sample_jni_optional_intermediate_output_niche_to_wire_c6400b29b5a421b3(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_replyIsOk<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    r: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jboolean {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let r = match __jni_in_convert_wire_to_Reply_jni_handle_codec_borrow_input_7db518929d6a9b8f(
        &mut env,
        &r,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::reply_is_ok(&r);
    match __jni_out_convert_bool_to_wire_3ee62077915d5228(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jboolean
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetAttachment<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_Sample_jni_handle_codec_borrow_input_4939cf7fe8217cff(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_attachment(&s);
    match __jni_out_convert_Option_ZBytes_jni_optional_intermediate_output_niche_to_wire_1562c928235871aa(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetCongestionControl<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_Sample_jni_handle_codec_borrow_input_4939cf7fe8217cff(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::sample_get_congestion_control(&s);
    match __jni_out_convert_CongestionControl_to_wire_dd1f6047005395ca(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetEncoding<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_Sample_jni_handle_codec_borrow_input_4939cf7fe8217cff(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_encoding(&s);
    match __jni_out_convert_Encoding_jni_handle_codec_clone_output_to_wire_ec144ca078ed717e(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetExpress<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jboolean {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_Sample_jni_handle_codec_borrow_input_4939cf7fe8217cff(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jboolean;
        }
    };
    let __out = zenoh_flat::sample_get_express(&s);
    match __jni_out_convert_bool_to_wire_3ee62077915d5228(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jboolean
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetKeyExpr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_Sample_jni_handle_codec_borrow_input_4939cf7fe8217cff(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_key_expr(&s);
    match __jni_out_convert_KeyExpr_jni_handle_codec_clone_output_to_wire_76ff23f685cd5e3d(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetKind<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_Sample_jni_handle_codec_borrow_input_4939cf7fe8217cff(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::sample_get_kind(&s);
    match __jni_out_convert_SampleKind_to_wire_5850f5fbf98a86e6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetPayload<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_Sample_jni_handle_codec_borrow_input_4939cf7fe8217cff(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_payload(&s);
    match __jni_out_convert_ZBytes_jni_handle_codec_clone_output_to_wire_631cd85333755af2(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetPriority<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_Sample_jni_handle_codec_borrow_input_4939cf7fe8217cff(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::sample_get_priority(&s);
    match __jni_out_convert_Priority_to_wire_55b65fa623d4787e(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetReliability<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jint {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_Sample_jni_handle_codec_borrow_input_4939cf7fe8217cff(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jint;
        }
    };
    let __out = zenoh_flat::sample_get_reliability(&s);
    match __jni_out_convert_Reliability_to_wire_1173d3c71693bfba(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jint
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetSourceInfo<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __builder: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_Sample_jni_handle_codec_borrow_input_4939cf7fe8217cff(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/sample/SourceInfoBuilder";
    const __CB_DESCR: &str = "([BJJ)Ljava/lang/Object;";
    let __out = zenoh_flat::sample_get_source_info(&s);
    let (__chain_present, (((__chain_wire0,), __chain_wire1), __chain_wire2)) = match __jni_out_convert_Option_SourceInfo_jni_optional_intermediate_output_gated_to_wire_e84416f8e9b768c0(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__intermediate) => __intermediate,
        ::core::result::Result::Err(__chain_error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__chain_error.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __obj0: jni::objects::JObject = __chain_wire0.into();
    let __obj1 = jni::sys::jvalue {
        j: __chain_wire1,
    };
    let __obj2 = jni::sys::jvalue {
        j: __chain_wire2,
    };
    if __chain_present != 0 {
        match __CB_MID
            .call_object(
                &mut env,
                __CB_FQN,
                "run",
                __CB_DESCR,
                &__builder,
                &[
                    jni::sys::jvalue {
                        l: __obj0.as_raw(),
                    },
                    __obj1,
                    __obj2,
                ],
            )
        {
            ::core::result::Result::Ok(__o) => __o,
            ::core::result::Result::Err(__e) => {
                let _ = env.exception_describe();
                let __e2 = <__JniErr as ::core::convert::From<
                    String,
                >>::from(__e.to_string());
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e2.to_string(),
                );
                jni::objects::JObject::null().into()
            }
        }
    } else {
        jni::objects::JObject::null().into()
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetTimestamp<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __builder: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_Sample_jni_handle_codec_borrow_input_4939cf7fe8217cff(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/time/TimestampBuilderRaw";
    const __CB_DESCR: &str = "(J[B)Ljava/lang/Object;";
    let __out = zenoh_flat::sample_get_timestamp(&s);
    let (__chain_present, (__chain_wire0, __chain_wire1)) = match __jni_out_convert_Option_Timestamp_jni_optional_intermediate_output_gated_to_wire_20caf618dda4bcb7(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__intermediate) => __intermediate,
        ::core::result::Result::Err(__chain_error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__chain_error.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __obj0 = jni::sys::jvalue {
        j: __chain_wire0,
    };
    let __obj1: jni::objects::JObject = __chain_wire1.into();
    if __chain_present != 0 {
        match __CB_MID
            .call_object(
                &mut env,
                __CB_FQN,
                "run",
                __CB_DESCR,
                &__builder,
                &[
                    __obj0,
                    jni::sys::jvalue {
                        l: __obj1.as_raw(),
                    },
                ],
            )
        {
            ::core::result::Result::Ok(__o) => __o,
            ::core::result::Result::Err(__e) => {
                let _ = env.exception_describe();
                let __e2 = <__JniErr as ::core::convert::From<
                    String,
                >>::from(__e.to_string());
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e2.to_string(),
                );
                jni::objects::JObject::null().into()
            }
        }
    } else {
        jni::objects::JObject::null().into()
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleGetTimestampStack<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_Sample_jni_handle_codec_borrow_input_4939cf7fe8217cff(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::sample_get_timestamp_stack(&s);
    match __jni_out_convert_Option_TimestampStack_jni_optional_intermediate_output_niche_to_wire_9e52e4193d223461(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleNewDelete<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    timestamp_present: jni::sys::jboolean,
    timestamp_ntp64: jni::sys::jlong,
    timestamp_id: jni::objects::JByteArray<'a>,
    attachment: jni::objects::JByteArray<'a>,
    congestion_control: jni::sys::jint,
    priority: jni::sys::jint,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    reliability: jni::sys::jint,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __exp_key_expr_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &key_expr_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match __jni_in_convert_wire_to_Option_KeyExpr_jni_optional_intermediate_input_niche_bbf13322cceb1d7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let timestamp = match __jni_in_convert_wire_to_Option_Timestamp_jni_optional_intermediate_input_gated_0e1cf2a3b10b443d(
        &mut env,
        (timestamp_present, (timestamp_ntp64, timestamp_id)),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_attachment = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let congestion_control = match __jni_in_convert_wire_to_Option_CongestionControl_jni_optional_intermediate_input_niche_b0f8f28507150661(
        &mut env,
        &congestion_control,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let priority = match __jni_in_convert_wire_to_Option_Priority_jni_optional_intermediate_input_niche_6f17c8de5a824021(
        &mut env,
        &priority,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let express = match __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d(
        &mut env,
        (express_present, express_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let reliability = match __jni_in_convert_wire_to_Option_Reliability_jni_optional_intermediate_input_niche_5451244c94aef911(
        &mut env,
        &reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::sample_new_delete(
        __folded_key_expr,
        timestamp,
        __folded_attachment,
        congestion_control,
        priority,
        express,
        reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_Sample_jni_handle_codec_own_output_to_wire_c2cd4d4f2b134f88(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sampleNewPut<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    payload: jni::objects::JByteArray<'a>,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JByteArray<'a>,
    encoding_1: jni::sys::jlong,
    timestamp_present: jni::sys::jboolean,
    timestamp_ntp64: jni::sys::jlong,
    timestamp_id: jni::objects::JByteArray<'a>,
    attachment: jni::objects::JByteArray<'a>,
    congestion_control: jni::sys::jint,
    priority: jni::sys::jint,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    reliability: jni::sys::jint,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __exp_key_expr_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &key_expr_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match __jni_in_convert_wire_to_Option_KeyExpr_jni_optional_intermediate_input_niche_bbf13322cceb1d7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_payload = match __jni_in_convert_wire_to_Vec_u8_80984e9556387695(
        &mut env,
        &payload,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_payload = match ::core::result::Result::Ok(
        zenoh_flat::zbytes_new_from_vec(__exp_payload),
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_encoding_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &encoding_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_encoding_0_0: ::core::option::Option<u16> = match __jni_in_convert_wire_to_Option_u16_jni_optional_intermediate_input_gated_ff09dbf66b457369(
        &mut env,
        (encoding_0_0_present, encoding_0_0_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_encoding_0_1 = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_encoding_1 = match __jni_in_convert_wire_to_Option_Encoding_1ea8d6cdb046d07b(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let timestamp = match __jni_in_convert_wire_to_Option_Timestamp_jni_optional_intermediate_input_gated_0e1cf2a3b10b443d(
        &mut env,
        (timestamp_present, (timestamp_ntp64, timestamp_id)),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_attachment = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let congestion_control = match __jni_in_convert_wire_to_Option_CongestionControl_jni_optional_intermediate_input_niche_b0f8f28507150661(
        &mut env,
        &congestion_control,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let priority = match __jni_in_convert_wire_to_Option_Priority_jni_optional_intermediate_input_niche_6f17c8de5a824021(
        &mut env,
        &priority,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let express = match __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d(
        &mut env,
        (express_present, express_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let reliability = match __jni_in_convert_wire_to_Option_Reliability_jni_optional_intermediate_input_niche_5451244c94aef911(
        &mut env,
        &reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::sample_new_put(
        __folded_key_expr,
        __folded_payload,
        __folded_encoding.as_ref(),
        timestamp,
        __folded_attachment,
        congestion_control,
        priority,
        express,
        reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_Sample_jni_handle_codec_own_output_to_wire_c2cd4d4f2b134f88(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_scout<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    whatami: jni::sys::jint,
    config: jni::sys::jlong,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let whatami = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &whatami,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let config = match __jni_in_convert_wire_to_Option_Config_3d9d8ed7ea1ee21b(
        &mut env,
        &config,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let callback = match __jni_in_convert_wire_to_impl_Fn_Hello_Send_Sync_static_149e928a50c5a2b9(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let on_close = match __jni_in_convert_wire_to_impl_Fn_Send_Sync_static_6d213555cb0eda7b(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::scout(whatami, config.as_deref(), callback, on_close) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_Scout_jni_handle_codec_own_output_to_wire_9667aa43f25707f6(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionDeclareAdvancedPublisher<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JByteArray<'a>,
    encoding_1: jni::sys::jlong,
    congestion_control: jni::sys::jint,
    priority: jni::sys::jint,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    reliability: jni::sys::jint,
    sample_miss_detection_present: jni::sys::jboolean,
    sample_miss_detection_heartbeat: jni::sys::jlong,
    sample_miss_detection_sporadic: jni::sys::jboolean,
    publisher_detection_present: jni::sys::jboolean,
    publisher_detection_value: jni::sys::jboolean,
    cache_present: jni::sys::jboolean,
    cache_max_samples: jni::sys::jlong,
    cache_replies_config_priority: jni::sys::jint,
    cache_replies_config_congestion_control: jni::sys::jint,
    cache_replies_config_is_express: jni::sys::jboolean,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &key_expr_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match __jni_in_convert_wire_to_Option_KeyExpr_jni_optional_intermediate_input_niche_bbf13322cceb1d7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_encoding_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &encoding_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_encoding_0_0: ::core::option::Option<u16> = match __jni_in_convert_wire_to_Option_u16_jni_optional_intermediate_input_gated_ff09dbf66b457369(
        &mut env,
        (encoding_0_0_present, encoding_0_0_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_encoding_0_1 = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_encoding_1 = match __jni_in_convert_wire_to_Option_Encoding_1ea8d6cdb046d07b(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let congestion_control = match __jni_in_convert_wire_to_Option_CongestionControl_jni_optional_intermediate_input_niche_b0f8f28507150661(
        &mut env,
        &congestion_control,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let priority = match __jni_in_convert_wire_to_Option_Priority_jni_optional_intermediate_input_niche_6f17c8de5a824021(
        &mut env,
        &priority,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let express = match __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d(
        &mut env,
        (express_present, express_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let reliability = match __jni_in_convert_wire_to_Option_Reliability_jni_optional_intermediate_input_niche_5451244c94aef911(
        &mut env,
        &reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let sample_miss_detection = match __jni_in_convert_wire_to_Option_MissDetectionConfig_jni_optional_intermediate_input_gated_d4115213f6ae9aaa(
        &mut env,
        (
            sample_miss_detection_present,
            (sample_miss_detection_heartbeat, sample_miss_detection_sporadic),
        ),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let publisher_detection = match __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d(
        &mut env,
        (publisher_detection_present, publisher_detection_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let cache = match __jni_in_convert_wire_to_Option_CacheConfig_jni_optional_intermediate_input_gated_db098990eea705d9(
        &mut env,
        (
            cache_present,
            (
                cache_max_samples,
                (
                    cache_replies_config_priority,
                    cache_replies_config_congestion_control,
                    cache_replies_config_is_express,
                ),
            ),
        ),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::session_declare_advanced_publisher(
        &session,
        __folded_key_expr,
        __folded_encoding.as_ref(),
        congestion_control,
        priority,
        express,
        reliability,
        sample_miss_detection,
        publisher_detection,
        cache,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_AdvancedPublisher_jni_handle_codec_own_output_to_wire_2e93ba1588e21f36(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionDeclareAdvancedSubscriber<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    history_present: jni::sys::jboolean,
    history_detect_late_publishers: jni::sys::jboolean,
    history_max_samples_present: jni::sys::jboolean,
    history_max_samples_value: jni::sys::jlong,
    history_max_age_present: jni::sys::jboolean,
    history_max_age_value: jni::sys::jdouble,
    recovery_present: jni::sys::jboolean,
    recovery_mode: jni::objects::JObject<'a>,
    recovery_retention_period: jni::sys::jlong,
    query_timeout: jni::sys::jlong,
    subscriber_detection_present: jni::sys::jboolean,
    subscriber_detection_value: jni::sys::jboolean,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &key_expr_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match __jni_in_convert_wire_to_Option_KeyExpr_jni_optional_intermediate_input_niche_bbf13322cceb1d7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let callback = match __jni_in_convert_wire_to_impl_Fn_Sample_Send_Sync_static_de5b7c31ca7e8b9f(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let on_close = match __jni_in_convert_wire_to_impl_Fn_Send_Sync_static_6d213555cb0eda7b(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let history = match __jni_in_convert_wire_to_Option_HistoryConfig_jni_optional_intermediate_input_gated_9a07e6b3656908cb(
        &mut env,
        (
            history_present,
            (
                history_detect_late_publishers,
                (history_max_samples_present, history_max_samples_value),
                (history_max_age_present, history_max_age_value),
            ),
        ),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let recovery = match __jni_in_convert_wire_to_Option_RecoveryConfig_jni_optional_intermediate_input_gated_eff50792a3d16136(
        &mut env,
        (recovery_present, (recovery_mode, recovery_retention_period)),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let query_timeout = match __jni_in_convert_wire_to_Option_Duration_jni_optional_intermediate_input_niche_104bdfd3431d40b9(
        &mut env,
        &query_timeout,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let subscriber_detection = match __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d(
        &mut env,
        (subscriber_detection_present, subscriber_detection_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::session_declare_advanced_subscriber(
        &session,
        __folded_key_expr,
        callback,
        on_close,
        history,
        recovery,
        query_timeout,
        subscriber_detection,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_AdvancedSubscriber_jni_handle_codec_own_output_to_wire_b81d795d1d1470ce(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionDeclareKeyexpr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr: jni::objects::JString<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let key_expr = match __jni_in_convert_wire_to_jni_text_codec_owned_40915aa02cf8d9ce(
        &mut env,
        &key_expr,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::session_declare_keyexpr(&session, key_expr) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_KeyExpr_jni_handle_codec_own_output_to_wire_2022bfd454e7e1bc(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionDeclarePublisher<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JByteArray<'a>,
    encoding_1: jni::sys::jlong,
    congestion_control: jni::sys::jint,
    priority: jni::sys::jint,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    reliability: jni::sys::jint,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &key_expr_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match __jni_in_convert_wire_to_Option_KeyExpr_jni_optional_intermediate_input_niche_bbf13322cceb1d7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_encoding_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &encoding_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_encoding_0_0: ::core::option::Option<u16> = match __jni_in_convert_wire_to_Option_u16_jni_optional_intermediate_input_gated_ff09dbf66b457369(
        &mut env,
        (encoding_0_0_present, encoding_0_0_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_encoding_0_1 = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_encoding_1 = match __jni_in_convert_wire_to_Option_Encoding_1ea8d6cdb046d07b(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let congestion_control = match __jni_in_convert_wire_to_Option_CongestionControl_jni_optional_intermediate_input_niche_b0f8f28507150661(
        &mut env,
        &congestion_control,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let priority = match __jni_in_convert_wire_to_Option_Priority_jni_optional_intermediate_input_niche_6f17c8de5a824021(
        &mut env,
        &priority,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let express = match __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d(
        &mut env,
        (express_present, express_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let reliability = match __jni_in_convert_wire_to_Option_Reliability_jni_optional_intermediate_input_niche_5451244c94aef911(
        &mut env,
        &reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::session_declare_publisher(
        &session,
        __folded_key_expr,
        __folded_encoding.as_ref(),
        congestion_control,
        priority,
        express,
        reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_Publisher_jni_handle_codec_own_output_to_wire_94fe05baf6f4cff8(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionDeclareQuerier<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    target: jni::sys::jint,
    consolidation: jni::sys::jint,
    congestion_control: jni::sys::jint,
    priority: jni::sys::jint,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    timeout_ms_present: jni::sys::jboolean,
    timeout_ms_value: jni::sys::jlong,
    accept_replies: jni::sys::jint,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &key_expr_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match __jni_in_convert_wire_to_Option_KeyExpr_jni_optional_intermediate_input_niche_bbf13322cceb1d7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let target = match __jni_in_convert_wire_to_Option_QueryTarget_jni_optional_intermediate_input_niche_5f89727c78ac796e(
        &mut env,
        &target,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let consolidation = match __jni_in_convert_wire_to_Option_ConsolidationMode_jni_optional_intermediate_input_niche_cbd6120cd28a09b0(
        &mut env,
        &consolidation,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let congestion_control = match __jni_in_convert_wire_to_Option_CongestionControl_jni_optional_intermediate_input_niche_b0f8f28507150661(
        &mut env,
        &congestion_control,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let priority = match __jni_in_convert_wire_to_Option_Priority_jni_optional_intermediate_input_niche_6f17c8de5a824021(
        &mut env,
        &priority,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let express = match __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d(
        &mut env,
        (express_present, express_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let timeout_ms = match __jni_in_convert_wire_to_Option_i64_jni_optional_intermediate_input_gated_89d088af3f259362(
        &mut env,
        (timeout_ms_present, timeout_ms_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let accept_replies = match __jni_in_convert_wire_to_Option_ReplyKeyExpr_jni_optional_intermediate_input_niche_e3c3d07c93406d91(
        &mut env,
        &accept_replies,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::session_declare_querier(
        &session,
        __folded_key_expr,
        target,
        consolidation,
        congestion_control,
        priority,
        express,
        timeout_ms,
        accept_replies,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_Querier_jni_handle_codec_own_output_to_wire_26375716f15b424b(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionDeclareQueryable<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    complete_present: jni::sys::jboolean,
    complete_value: jni::sys::jboolean,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &key_expr_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match __jni_in_convert_wire_to_Option_KeyExpr_jni_optional_intermediate_input_niche_bbf13322cceb1d7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let complete = match __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d(
        &mut env,
        (complete_present, complete_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let callback = match __jni_in_convert_wire_to_impl_Fn_Query_Send_Sync_static_448bc7311098de85(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let on_close = match __jni_in_convert_wire_to_impl_Fn_Send_Sync_static_6d213555cb0eda7b(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::session_declare_queryable(
        &session,
        __folded_key_expr,
        complete,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_Queryable_jni_handle_codec_own_output_to_wire_3d471fba643e8930(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionDeclareSubscriber<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &key_expr_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __exp_key_expr_1 = match __jni_in_convert_wire_to_Option_KeyExpr_jni_optional_intermediate_input_niche_bbf13322cceb1d7d(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => ::core::result::Result::Ok(__v),
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let callback = match __jni_in_convert_wire_to_impl_Fn_Sample_Send_Sync_static_de5b7c31ca7e8b9f(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let on_close = match __jni_in_convert_wire_to_impl_Fn_Send_Sync_static_6d213555cb0eda7b(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = match zenoh_flat::session_declare_subscriber(
        &session,
        __folded_key_expr,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return 0 as jni::sys::jlong;
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return 0 as jni::sys::jlong;
        }
    };
    match __jni_out_convert_Subscriber_jni_handle_codec_own_output_to_wire_4552f6e215a6371c(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionDelete<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    congestion_control: jni::sys::jint,
    priority: jni::sys::jint,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    attachment: jni::objects::JByteArray<'a>,
    reliability: jni::sys::jint,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &key_expr_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_1 = match __jni_in_convert_wire_to_Option_KeyExpr_75b6fc876caf57d7(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let congestion_control = match __jni_in_convert_wire_to_Option_CongestionControl_jni_optional_intermediate_input_niche_b0f8f28507150661(
        &mut env,
        &congestion_control,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let priority = match __jni_in_convert_wire_to_Option_Priority_jni_optional_intermediate_input_niche_6f17c8de5a824021(
        &mut env,
        &priority,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let express = match __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d(
        &mut env,
        (express_present, express_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let __exp_attachment = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let reliability = match __jni_in_convert_wire_to_Option_Reliability_jni_optional_intermediate_input_niche_5451244c94aef911(
        &mut env,
        &reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::session_delete(
        &session,
        &__folded_key_expr,
        congestion_control,
        priority,
        express,
        __folded_attachment,
        reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionGet<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    selector_key_expr: jni::sys::jlong,
    selector_parameters: jni::objects::JString<'a>,
    timeout_ms_present: jni::sys::jboolean,
    timeout_ms_value: jni::sys::jlong,
    target: jni::sys::jint,
    consolidation: jni::sys::jint,
    accept_replies: jni::sys::jint,
    congestion_control: jni::sys::jint,
    priority: jni::sys::jint,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    payload: jni::objects::JByteArray<'a>,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JByteArray<'a>,
    encoding_1: jni::sys::jlong,
    attachment: jni::objects::JByteArray<'a>,
    callback: jni::objects::JObject<'a>,
    on_close: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let selector = match __jni_in_convert_wire_to_Selector_jni_product_intermediate_tuple_f642894fba00fd17(
        &mut env,
        (selector_key_expr, selector_parameters),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let timeout_ms = match __jni_in_convert_wire_to_Option_i64_jni_optional_intermediate_input_gated_89d088af3f259362(
        &mut env,
        (timeout_ms_present, timeout_ms_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let target = match __jni_in_convert_wire_to_Option_QueryTarget_jni_optional_intermediate_input_niche_5f89727c78ac796e(
        &mut env,
        &target,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let consolidation = match __jni_in_convert_wire_to_Option_ConsolidationMode_jni_optional_intermediate_input_niche_cbd6120cd28a09b0(
        &mut env,
        &consolidation,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let accept_replies = match __jni_in_convert_wire_to_Option_ReplyKeyExpr_jni_optional_intermediate_input_niche_e3c3d07c93406d91(
        &mut env,
        &accept_replies,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let congestion_control = match __jni_in_convert_wire_to_Option_CongestionControl_jni_optional_intermediate_input_niche_b0f8f28507150661(
        &mut env,
        &congestion_control,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let priority = match __jni_in_convert_wire_to_Option_Priority_jni_optional_intermediate_input_niche_6f17c8de5a824021(
        &mut env,
        &priority,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let express = match __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d(
        &mut env,
        (express_present, express_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let __exp_payload = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &payload,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_payload = match match __exp_payload {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &encoding_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_0: ::core::option::Option<u16> = match __jni_in_convert_wire_to_Option_u16_jni_optional_intermediate_input_gated_ff09dbf66b457369(
        &mut env,
        (encoding_0_0_present, encoding_0_0_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_1 = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_1 = match __jni_in_convert_wire_to_Option_Encoding_1ea8d6cdb046d07b(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_attachment = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let callback = match __jni_in_convert_wire_to_impl_Fn_Reply_Send_Sync_static_8acaa3be44c06271(
        &mut env,
        &callback,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let on_close = match __jni_in_convert_wire_to_impl_Fn_Send_Sync_static_6d213555cb0eda7b(
        &mut env,
        &on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::session_get(
        &session,
        selector,
        timeout_ms,
        target,
        consolidation,
        accept_replies,
        congestion_control,
        priority,
        express,
        __folded_payload,
        __folded_encoding.as_ref(),
        __folded_attachment,
        callback,
        on_close,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionGetPeersZid<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    __acc: jni::objects::JObject<'a>,
    __fold: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/config/ZenohIdFolderRaw";
    const __CB_DESCR: &str = "(Ljava/lang/Object;[B)Ljava/lang/Object;";
    let __vec = zenoh_flat::session_get_peers_zid(&session);
    let mut __acc = __acc;
    for __elem in __vec.into_iter() {
        let (__chain_wire0,) = match __jni_out_convert_ZenohId_jni_product_intermediate_tuple_to_wire_c7bb0eb9058fd8ec(
            &mut env,
            __elem,
        ) {
            ::core::result::Result::Ok(__intermediate) => __intermediate,
            ::core::result::Result::Err(__chain_error) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__chain_error.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        let __obj0: jni::objects::JObject = __chain_wire0.into();
        __acc = match __CB_MID
            .call_object(
                &mut env,
                __CB_FQN,
                "run",
                __CB_DESCR,
                &__fold,
                &[
                    jni::sys::jvalue {
                        l: __acc.as_raw(),
                    },
                    jni::sys::jvalue {
                        l: __obj0.as_raw(),
                    },
                ],
            )
        {
            ::core::result::Result::Ok(__o) => __o,
            ::core::result::Result::Err(__e) => {
                let _ = env.exception_describe();
                let __e2 = <__JniErr as ::core::convert::From<
                    String,
                >>::from(__e.to_string());
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e2.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
    }
    __acc
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionGetRoutersZid<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    __acc: jni::objects::JObject<'a>,
    __fold: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/config/ZenohIdFolderRaw";
    const __CB_DESCR: &str = "(Ljava/lang/Object;[B)Ljava/lang/Object;";
    let __vec = zenoh_flat::session_get_routers_zid(&session);
    let mut __acc = __acc;
    for __elem in __vec.into_iter() {
        let (__chain_wire0,) = match __jni_out_convert_ZenohId_jni_product_intermediate_tuple_to_wire_c7bb0eb9058fd8ec(
            &mut env,
            __elem,
        ) {
            ::core::result::Result::Ok(__intermediate) => __intermediate,
            ::core::result::Result::Err(__chain_error) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__chain_error.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        let __obj0: jni::objects::JObject = __chain_wire0.into();
        __acc = match __CB_MID
            .call_object(
                &mut env,
                __CB_FQN,
                "run",
                __CB_DESCR,
                &__fold,
                &[
                    jni::sys::jvalue {
                        l: __acc.as_raw(),
                    },
                    jni::sys::jvalue {
                        l: __obj0.as_raw(),
                    },
                ],
            )
        {
            ::core::result::Result::Ok(__o) => __o,
            ::core::result::Result::Err(__e) => {
                let _ = env.exception_describe();
                let __e2 = <__JniErr as ::core::convert::From<
                    String,
                >>::from(__e.to_string());
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e2.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
    }
    __acc
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionGetZid<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    __builder: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/config/ZenohIdBuilder";
    const __CB_DESCR: &str = "([B)Ljava/lang/Object;";
    let __out = zenoh_flat::session_get_zid(&session);
    let (__chain_wire0,) = match __jni_out_convert_ZenohId_jni_product_intermediate_tuple_to_wire_c7bb0eb9058fd8ec(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__intermediate) => __intermediate,
        ::core::result::Result::Err(__chain_error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__chain_error.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __obj0: jni::objects::JObject = __chain_wire0.into();
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[
                jni::sys::jvalue {
                    l: __obj0.as_raw(),
                },
            ],
        )
    {
        ::core::result::Result::Ok(__o) => __o,
        ::core::result::Result::Err(__e) => {
            let _ = env.exception_describe();
            let __e2 = <__JniErr as ::core::convert::From<
                String,
            >>::from(__e.to_string());
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e2.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionPut<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr_sel: jni::sys::jint,
    key_expr_0: jni::objects::JString<'a>,
    key_expr_1: jni::sys::jlong,
    payload: jni::objects::JByteArray<'a>,
    encoding_sel: jni::sys::jint,
    encoding_0_0_present: jni::sys::jboolean,
    encoding_0_0_value: jni::sys::jint,
    encoding_0_1: jni::objects::JByteArray<'a>,
    encoding_1: jni::sys::jlong,
    congestion_control: jni::sys::jint,
    priority: jni::sys::jint,
    express_present: jni::sys::jboolean,
    express_value: jni::sys::jboolean,
    attachment: jni::objects::JByteArray<'a>,
    reliability: jni::sys::jint,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &key_expr_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_0 = match __jni_in_convert_wire_to_Option_String_jni_optional_intermediate_input_niche_82a26a9b7e445442(
        &mut env,
        &key_expr_0,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_key_expr_1 = match __jni_in_convert_wire_to_Option_KeyExpr_75b6fc876caf57d7(
        &mut env,
        &key_expr_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_key_expr = match {
        match __exp_key_expr_sel {
            0i32 => {
                match __exp_key_expr_0 {
                    ::core::option::Option::Some(__p0) => {
                        zenoh_flat::keyexpr_new_try_from(__p0)
                            .map_err(|__e| ::std::format!("{}", __e))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from(
                                "constructor variant input missing",
                            ),
                        )
                    }
                }
            }
            1i32 => {
                match __exp_key_expr_1 {
                    ::core::option::Option::Some(__v) => {
                        ::core::result::Result::Ok(::core::clone::Clone::clone(&*__v))
                    }
                    ::core::option::Option::None => {
                        ::core::result::Result::Err(
                            ::std::string::String::from("identity variant value missing"),
                        )
                    }
                }
            }
            __sel => {
                ::core::result::Result::Err(
                    ::std::format!("invalid constructor selector: {}", __sel),
                )
            }
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_payload = match __jni_in_convert_wire_to_Vec_u8_80984e9556387695(
        &mut env,
        &payload,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_payload = match ::core::result::Result::Ok(
        zenoh_flat::zbytes_new_from_vec(__exp_payload),
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_sel = match __jni_in_convert_wire_to_i32_83b133e23cc76fc5(
        &mut env,
        &encoding_sel,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_0: ::core::option::Option<u16> = match __jni_in_convert_wire_to_Option_u16_jni_optional_intermediate_input_gated_ff09dbf66b457369(
        &mut env,
        (encoding_0_0_present, encoding_0_0_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_0_1 = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &encoding_0_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __exp_encoding_1 = match __jni_in_convert_wire_to_Option_Encoding_1ea8d6cdb046d07b(
        &mut env,
        &encoding_1,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_encoding = match if __exp_encoding_sel < 0 {
        ::core::result::Result::Ok(::core::option::Option::None)
    } else {
        ({
            match __exp_encoding_sel {
                0i32 => {
                    match __exp_encoding_0_0 {
                        ::core::option::Option::Some(__p0) => {
                            ::core::result::Result::Ok(
                                zenoh_flat::encoding_new_from_id(__p0, __exp_encoding_0_1),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "constructor variant input missing",
                                ),
                            )
                        }
                    }
                }
                1i32 => {
                    match __exp_encoding_1 {
                        ::core::option::Option::Some(__v) => {
                            ::core::result::Result::Ok(
                                ::core::clone::Clone::clone(&*__v),
                            )
                        }
                        ::core::option::Option::None => {
                            ::core::result::Result::Err(
                                ::std::string::String::from(
                                    "identity variant value missing",
                                ),
                            )
                        }
                    }
                }
                __sel => {
                    ::core::result::Result::Err(
                        ::std::format!("invalid constructor selector: {}", __sel),
                    )
                }
            }
        })
            .map(::core::option::Option::Some)
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let congestion_control = match __jni_in_convert_wire_to_Option_CongestionControl_jni_optional_intermediate_input_niche_b0f8f28507150661(
        &mut env,
        &congestion_control,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let priority = match __jni_in_convert_wire_to_Option_Priority_jni_optional_intermediate_input_niche_6f17c8de5a824021(
        &mut env,
        &priority,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let express = match __jni_in_convert_wire_to_Option_bool_jni_optional_intermediate_input_gated_3f6e7ec66fee944d(
        &mut env,
        (express_present, express_value),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return ();
        }
    };
    let __exp_attachment = match __jni_in_convert_wire_to_Option_Vec_u8_jni_optional_intermediate_input_niche_36da452e94b45d02(
        &mut env,
        &attachment,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __folded_attachment = match match __exp_attachment {
        ::core::option::Option::Some(__inner) => {
            (::core::result::Result::Ok(zenoh_flat::zbytes_new_from_vec(__inner)))
                .map(::core::option::Option::Some)
        }
        ::core::option::Option::None => {
            ::core::result::Result::Ok(::core::option::Option::None)
        }
    } {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            let __je = <__JniErr as ::core::convert::From<
                ::std::string::String,
            >>::from(__e);
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__je.to_string(),
            );
            return ();
        }
    };
    let reliability = match __jni_in_convert_wire_to_Option_Reliability_jni_optional_intermediate_input_niche_5451244c94aef911(
        &mut env,
        &reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::session_put(
        &session,
        &__folded_key_expr,
        __folded_payload,
        __folded_encoding.as_ref(),
        congestion_control,
        priority,
        express,
        __folded_attachment,
        reliability,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_sessionUndeclareKeyexpr<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    session: jni::sys::jlong,
    key_expr: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let session = match __jni_in_convert_wire_to_Session_jni_handle_codec_borrow_input_4cafb2fd6421beb7(
        &mut env,
        &session,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let key_expr = match __jni_in_convert_wire_to_KeyExpr_jni_handle_codec_consume_input_7fcb26b92cb14c18(
        &mut env,
        &key_expr,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return ();
        }
    };
    let __out = match zenoh_flat::session_undeclare_keyexpr(&session, key_expr) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return ();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return ();
        }
    };
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_timestampStackGetInstrumentation<
    'a,
>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __builder: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_TimestampStack_jni_handle_codec_borrow_input_0b5530c7da6cc169(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/time/TimestampInstrumentationBuilder";
    const __CB_DESCR: &str = "(ZZZ)Ljava/lang/Object;";
    let __out = zenoh_flat::timestamp_stack_get_instrumentation(&s);
    let (__chain_wire0, __chain_wire1, __chain_wire2) = match __jni_out_convert_TimestampInstrumentation_jni_product_intermediate_tuple_to_wire_2be420083815e28f(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__intermediate) => __intermediate,
        ::core::result::Result::Err(__chain_error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__chain_error.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __obj0 = jni::sys::jvalue {
        z: __chain_wire0,
    };
    let __obj1 = jni::sys::jvalue {
        z: __chain_wire1,
    };
    let __obj2 = jni::sys::jvalue {
        z: __chain_wire2,
    };
    match __CB_MID
        .call_object(
            &mut env,
            __CB_FQN,
            "run",
            __CB_DESCR,
            &__builder,
            &[__obj0, __obj1, __obj2],
        )
    {
        ::core::result::Result::Ok(__o) => __o,
        ::core::result::Result::Err(__e) => {
            let _ = env.exception_describe();
            let __e2 = <__JniErr as ::core::convert::From<
                String,
            >>::from(__e.to_string());
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e2.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_timestampStackGetRecords<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    s: jni::sys::jlong,
    __acc: jni::objects::JObject<'a>,
    __fold: jni::objects::JObject<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JObject<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let s = match __jni_in_convert_wire_to_TimestampStack_jni_handle_codec_borrow_input_0b5530c7da6cc169(
        &mut env,
        &s,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    #[allow(non_upper_case_globals)]
    static __CB_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __CB_FQN: &str = "io/zenoh/jni/time/TimestampStackRecordFolderRaw";
    const __CB_DESCR: &str = "(Ljava/lang/Object;IILio/zenoh/jni/time/Timestamp;[B)Ljava/lang/Object;";
    let __vec = zenoh_flat::timestamp_stack_get_records(&s);
    let mut __acc = __acc;
    for __elem in __vec.into_iter() {
        let (__chain_wire0, (__chain_wire1, (__chain_wire2,), (__chain_wire3,))) = match __jni_out_convert_TimestampStackRecord_jni_product_intermediate_tuple_to_wire_a61a345e64f503e6(
            &mut env,
            __elem,
        ) {
            ::core::result::Result::Ok(__intermediate) => __intermediate,
            ::core::result::Result::Err(__chain_error) => {
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__chain_error.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
        let __obj0 = jni::sys::jvalue {
            i: __chain_wire0,
        };
        let __obj1 = jni::sys::jvalue {
            i: __chain_wire1,
        };
        let __obj2: jni::objects::JObject = __chain_wire2;
        let __obj3: jni::objects::JObject = __chain_wire3.into();
        __acc = match __CB_MID
            .call_object(
                &mut env,
                __CB_FQN,
                "run",
                __CB_DESCR,
                &__fold,
                &[
                    jni::sys::jvalue {
                        l: __acc.as_raw(),
                    },
                    __obj0,
                    __obj1,
                    jni::sys::jvalue {
                        l: __obj2.as_raw(),
                    },
                    jni::sys::jvalue {
                        l: __obj3.as_raw(),
                    },
                ],
            )
        {
            ::core::result::Result::Ok(__o) => __o,
            ::core::result::Result::Err(__e) => {
                let _ = env.exception_describe();
                let __e2 = <__JniErr as ::core::convert::From<
                    String,
                >>::from(__e.to_string());
                signal_binding_error(
                    &mut env,
                    &__error_sink,
                    &__SINK_MID,
                    __SINK_FQN,
                    __SINK_DESCR,
                    &__e2.to_string(),
                );
                return jni::objects::JObject::null().into();
            }
        };
    }
    __acc
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_tryInitZenohLogsFromEnv<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> () {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let __out = zenoh_flat::try_init_zenoh_logs_from_env();
    match __jni_out_convert_unit_to_wire_9e1510fd173c1fd6(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            ()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zbytesNewClone<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    z: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let z = match __jni_in_convert_wire_to_ZBytes_jni_handle_codec_borrow_input_d849d0f26d3372f8(
        &mut env,
        &z,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::zbytes_new_clone(&z);
    match __jni_out_convert_ZBytes_jni_handle_codec_own_output_to_wire_1d6ceb9f821de6d7(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zbytesNewFromVec<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    bytes: jni::objects::JByteArray<'a>,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::sys::jlong {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let bytes = match __jni_in_convert_wire_to_Vec_u8_80984e9556387695(
        &mut env,
        &bytes,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return 0 as jni::sys::jlong;
        }
    };
    let __out = zenoh_flat::zbytes_new_from_vec(bytes);
    match __jni_out_convert_ZBytes_jni_handle_codec_own_output_to_wire_1d6ceb9f821de6d7(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            0 as jni::sys::jlong
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zbytesToBytes<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    z: jni::sys::jlong,
    __error_sink: jni::objects::JObject<'a>,
) -> jni::objects::JByteArray<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let z = match __jni_in_convert_wire_to_ZBytes_jni_handle_codec_borrow_input_d849d0f26d3372f8(
        &mut env,
        &z,
    ) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = zenoh_flat::zbytes_to_bytes(&z);
    match __jni_out_convert_Cow_u8_to_wire_eafa10ed25b05dd5(&mut env, __out) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
#[no_mangle]
#[allow(non_snake_case, unused_mut, unused_variables, dead_code)]
pub unsafe extern "C" fn Java_io_zenoh_jni_JNINative_zenohIdToString<'a>(
    mut env: jni::JNIEnv<'a>,
    _class: jni::objects::JClass<'a>,
    z_bytes: jni::objects::JByteArray<'a>,
    __error_sink: jni::objects::JObject<'a>,
    __domain_sink: jni::objects::JObject<'a>,
) -> jni::objects::JString<'a> {
    #[allow(non_upper_case_globals)]
    static __SINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __SINK_FQN: &str = "io/zenoh/jni/JniErrorHandler";
    const __SINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    #[allow(non_upper_case_globals)]
    static __DSINK_MID: ::prebindgen_jni_runtime::CachedIfaceMethod = ::prebindgen_jni_runtime::CachedIfaceMethod::new();
    const __DSINK_FQN: &str = "io/zenoh/jni/ErrorHandler";
    const __DSINK_DESCR: &str = "(Ljava/lang/String;)Ljava/lang/Object;";
    let z = match __jni_in_convert_wire_to_ZenohId_jni_product_intermediate_tuple_e59310f235d3e3e1(
        &mut env,
        (z_bytes,),
    ) {
        ::core::result::Result::Ok(__value) => __value,
        ::core::result::Result::Err(__error) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__error.to_string(),
            );
            return jni::objects::JObject::null().into();
        }
    };
    let __out = match zenoh_flat::zenoh_id_to_string(&z) {
        ::core::result::Result::Ok(__v) => __v,
        ::core::result::Result::Err(__de) => {
            let __eze0: jni::objects::JObject = {
                let __enc0 = match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
                    &mut env,
                    zenoh_flat::error_get_message(&__de),
                ) {
                    ::core::result::Result::Ok(__w) => __w,
                    ::core::result::Result::Err(__e) => {
                        signal_binding_error(
                            &mut env,
                            &__error_sink,
                            &__SINK_MID,
                            __SINK_FQN,
                            __SINK_DESCR,
                            &__e.to_string(),
                        );
                        return jni::objects::JObject::null().into();
                    }
                };
                __enc0.into()
            };
            signal_domain_error(
                &mut env,
                &__domain_sink,
                &__DSINK_MID,
                __DSINK_FQN,
                __DSINK_DESCR,
                &[
                    jni::sys::jvalue {
                        l: __eze0.as_raw(),
                    },
                ],
            );
            return jni::objects::JObject::null().into();
        }
    };
    match __jni_out_convert_jni_text_codec_owned_to_wire_1b6cdff0ec9adbcb(
        &mut env,
        __out,
    ) {
        ::core::result::Result::Ok(__w) => __w,
        ::core::result::Result::Err(__e) => {
            signal_binding_error(
                &mut env,
                &__error_sink,
                &__SINK_MID,
                __SINK_FQN,
                __SINK_DESCR,
                &__e.to_string(),
            );
            jni::objects::JObject::null().into()
        }
    }
}
const _: () = {
    konst::assertc_eq!(
        zenoh_flat::FEATURES,
        "zenoh-flat/auth_pubkey zenoh-flat/auth_usrpwd zenoh-flat/transport_compression zenoh-flat/transport_multilink zenoh-flat/transport_quic zenoh-flat/transport_quic_datagram zenoh-flat/transport_tcp zenoh-flat/transport_tls zenoh-flat/transport_udp zenoh-flat/transport_unixsock-stream zenoh-flat/transport_ws zenoh-flat/unstable",
        "prebindgen: features mismatch between source crate and prebindgen generated file.\n\
                        This usually happens if source crate is compiled with different feature set\n\
                        for build dependencies and for library usage. You may need to explicitly set\n\
                        the necessary features."
    );
};
