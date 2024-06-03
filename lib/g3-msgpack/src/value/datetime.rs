/*
 * Copyright 2023 ByteDance and/or its affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use rmpv::ValueRef;

pub fn as_rfc3339_datetime(value: &ValueRef) -> anyhow::Result<DateTime<Utc>> {
    match value {
        ValueRef::String(s) => match s.as_str() {
            Some(s) => {
                let datetime = DateTime::parse_from_rfc3339(s)
                    .map_err(|e| anyhow!("invalid rfc3339 datetime string: {e}"))?;
                Ok(datetime.with_timezone(&Utc))
            }
            None => Err(anyhow!("invalid utf-8 string")),
        },
        _ => Err(anyhow!(
            "yaml value type for 'rfc3339 datetime' should be string"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmpv::Utf8StringRef;

    #[test]
    fn utc_tz() {
        let value = ValueRef::String(Utf8StringRef::from("2019-05-23T17:38:00Z"));
        let dt = as_rfc3339_datetime(&value).unwrap();
        assert_eq!(dt.to_rfc3339(), "2019-05-23T17:38:00+00:00");
    }

    #[test]
    fn t_error() {
        let value = ValueRef::String(Utf8StringRef::from("2019-05-23 17:38:00"));
        assert!(as_rfc3339_datetime(&value).is_err());

        let value = ValueRef::F32(1.0);
        assert!(as_rfc3339_datetime(&value).is_err());
    }
}
