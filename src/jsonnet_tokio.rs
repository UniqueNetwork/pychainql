// Copyright 2024 Valery Klachkov
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use gag::Gag;
use std::sync::atomic::Ordering;
use tokio::task::block_in_place;

pub fn init() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .thread_name("chainql-tokio-runtime")
        .enable_all()
        .build()
        .unwrap();

    crate::RUNTIME.set(runtime).unwrap()
}

#[inline(always)]
pub fn execute_jsonnet<F: FnOnce() -> T, T>(f: F) -> T {
    let use_logger = crate::ENABLE_LOGGER.load(Ordering::SeqCst);
    let _gag = (!use_logger).then(|| (Gag::stdout().unwrap(), Gag::stderr().unwrap()));

    let runtime = crate::RUNTIME.get().unwrap();
    let _enter_guard = runtime.enter();

    block_in_place(f)
}
