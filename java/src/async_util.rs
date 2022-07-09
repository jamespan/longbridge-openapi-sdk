use std::future::Future;

use jni::{
    errors::Result,
    objects::{JObject, JValue},
    JNIEnv,
};
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("create tokio runtime")
});

use crate::{error::JniError, types::IntoJValue};

fn async_complete<'a, T, V>(env: &JNIEnv<'a>, callback: T, value: V) -> Result<()>
where
    T: Into<JObject<'a>>,
    V: Into<JObject<'a>>,
{
    env.call_method(
        callback,
        "callback",
        "(Ljava/lang/Object;Ljava/lang/Object;)V",
        &[JValue::from(JObject::null()), JValue::from(value)],
    )
    .map(|_| ())
}

fn async_error<'a, T, V>(env: &JNIEnv<'a>, callback: T, err: V) -> Result<()>
where
    T: Into<JObject<'a>>,
    V: Into<JObject<'a>>,
{
    env.call_method(
        callback,
        "callback",
        "(Ljava/lang/Object;Ljava/lang/Object;)V",
        &[JValue::from(err), JValue::from(JObject::null())],
    )
    .map(|_| ())
}

pub(crate) fn execute<'a, T, F>(env: &JNIEnv<'a>, callback: JObject, fut: F) -> Result<()>
where
    T: IntoJValue,
    F: Future<Output = ::std::result::Result<T, JniError>> + Send + 'static,
{
    let jvm = env.get_java_vm()?;
    let callback = env.new_global_ref(callback)?;

    let _guard = RUNTIME.enter();
    tokio::spawn(async move {
        let res = fut.await;
        let env = jvm.attach_current_thread().unwrap();
        match res {
            Ok(value) => {
                let _ = async_complete(
                    &env,
                    &callback,
                    value.into_jvalue(&env).unwrap().l().unwrap(),
                );
            }
            Err(err) => {
                let _ = async_error(&env, &callback, err.into_error_object(&env));
            }
        }
    });

    Ok(())
}
