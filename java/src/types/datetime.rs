use std::borrow::Cow;

use jni::{
    errors::Result,
    objects::{JValue, JValueOwned},
    JNIEnv,
};
use time::{Date, Month, OffsetDateTime, Time};

use crate::{
    init::{
        TIME_INSTANT_CLASS, TIME_LOCALDATE_CLASS, TIME_LOCALTIME_CLASS, TIME_OFFSETDATETIME_CLASS,
        TIME_ZONE_ID,
    },
    types::{ClassLoader, FromJValue, IntoJValue, JSignature},
};

impl ClassLoader for OffsetDateTime {
    fn init(_env: &mut JNIEnv) {}

    fn class_ref() -> jni::objects::GlobalRef {
        TIME_OFFSETDATETIME_CLASS.get().cloned().unwrap()
    }
}

impl JSignature for OffsetDateTime {
    fn signature() -> Cow<'static, str> {
        "Ljava/time/OffsetDateTime;".into()
    }
}

impl FromJValue for OffsetDateTime {
    fn from_jvalue(env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        let obj = value.l()?;
        let value = env.call_method(obj, "toEpochSecond", "()J", &[])?.j()?;
        Ok(OffsetDateTime::from_unix_timestamp(value).unwrap())
    }
}

impl IntoJValue for OffsetDateTime {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        let instant = env.call_static_method(
            TIME_INSTANT_CLASS.get().unwrap(),
            "ofEpochSecond",
            "(J)Ljava/time/Instant;",
            &[JValue::from(self.unix_timestamp())],
        )?;

        env.call_static_method(
            TIME_OFFSETDATETIME_CLASS.get().unwrap(),
            "ofInstant",
            "(Ljava/time/Instant;Ljava/time/ZoneId;)Ljava/time/OffsetDateTime;",
            &[
                instant.borrow(),
                JValue::from(TIME_ZONE_ID.get().unwrap().as_obj()),
            ],
        )
    }
}

impl ClassLoader for Date {
    fn init(_env: &mut JNIEnv) {}

    fn class_ref() -> jni::objects::GlobalRef {
        TIME_LOCALDATE_CLASS.get().cloned().unwrap()
    }
}

impl JSignature for Date {
    fn signature() -> Cow<'static, str> {
        "Ljava/time/LocalDate;".into()
    }
}

impl FromJValue for Date {
    fn from_jvalue(env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        let obj = value.l()?;
        let year = env.call_method(&obj, "getYear", "()I", &[])?.i()?;
        let month = env.call_method(&obj, "getMonthValue", "()I", &[])?.i()?;
        let day = env.call_method(&obj, "getDayOfMonth", "()I", &[])?.i()?;
        Ok(
            Date::from_calendar_date(year, Month::try_from(month as u8).unwrap(), day as u8)
                .unwrap(),
        )
    }
}

impl IntoJValue for Date {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        env.call_static_method(
            TIME_LOCALDATE_CLASS.get().unwrap(),
            "of",
            "(III)Ljava/time/LocalDate;",
            &[
                JValue::from(self.year()),
                JValue::from(self.month() as i32),
                JValue::from(self.day() as i32),
            ],
        )
    }
}

impl ClassLoader for Time {
    fn init(_env: &mut JNIEnv) {}

    fn class_ref() -> jni::objects::GlobalRef {
        TIME_LOCALTIME_CLASS.get().cloned().unwrap()
    }
}

impl JSignature for Time {
    fn signature() -> Cow<'static, str> {
        "Ljava/time/LocalTime;".into()
    }
}

impl FromJValue for Time {
    fn from_jvalue(env: &mut JNIEnv, value: JValueOwned) -> Result<Self> {
        let obj = value.l()?;
        let hour = env.call_method(&obj, "getHour", "()I", &[])?.i()?;
        let minute = env.call_method(&obj, "getMinute", "()I", &[])?.i()?;
        let second = env.call_method(&obj, "getSecond", "()I", &[])?.i()?;
        Ok(Time::from_hms(hour as u8, minute as u8, second as u8).unwrap())
    }
}

impl IntoJValue for Time {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        env.call_static_method(
            TIME_LOCALTIME_CLASS.get().unwrap(),
            "of",
            "(III)Ljava/time/LocalTime;",
            &[
                JValue::from(self.hour() as i32),
                JValue::from(self.minute() as i32),
                JValue::from(self.second() as i32),
            ],
        )
    }
}
