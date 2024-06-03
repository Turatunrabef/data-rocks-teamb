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

use std::net::{IpAddr, SocketAddr};

use chrono::{DateTime, Utc};

use g3_types::net::{SocketBufferConfig, UpstreamAddr};

pub(crate) struct UdpConnectTaskNotes {
    pub(crate) buf_conf: SocketBufferConfig,
    pub(crate) upstream: Option<UpstreamAddr>,
    pub(crate) escaper: String,
    pub(crate) bind: Option<IpAddr>,
    pub(crate) next: Option<SocketAddr>,
    pub(crate) local: Option<SocketAddr>,
    pub(crate) expire: Option<DateTime<Utc>>,
}

impl UdpConnectTaskNotes {
    pub(crate) fn empty(buf_conf: SocketBufferConfig) -> Self {
        UdpConnectTaskNotes {
            buf_conf,
            upstream: None,
            escaper: String::new(),
            bind: None,
            next: None,
            local: None,
            expire: None,
        }
    }

    #[allow(unused)]
    pub(crate) fn new(upstream: UpstreamAddr, buf_conf: SocketBufferConfig) -> Self {
        UdpConnectTaskNotes {
            buf_conf,
            upstream: Some(upstream),
            escaper: String::new(),
            bind: None,
            next: None,
            local: None,
            expire: None,
        }
    }
}
