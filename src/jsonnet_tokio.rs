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
use nix::sys::signal::{signal, SigHandler, Signal};
use pyo3::PyResult;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::LazyLock;
use tokio::sync::Notify;
use tokio::task::block_in_place;

static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .thread_name("chainql-tokio-runtime")
        .enable_all()
        .build()
        .unwrap()
});

static CANCELLATION_NOTIFIER: LazyLock<Arc<Notify>> = LazyLock::new(|| Arc::new(Notify::new()));

#[track_caller]
#[inline(always)]
pub fn execute_jsonnet<F, T>(f: F) -> F::Output
where
    F: FnOnce(Arc<Notify>) -> PyResult<T>,
{
    let pyhandler = unsafe { signal(Signal::SIGINT, SigHandler::Handler(ctrl_c_handler)).unwrap() };

    // TODO: Remove when chainql_core migrates to tracings
    let use_logger = crate::ENABLE_LOGGER.load(Ordering::SeqCst);
    let _gag = (!use_logger).then(|| (Gag::stdout().unwrap(), Gag::stderr().unwrap()));

    let _enter_guard = RUNTIME.enter();
    let result = block_in_place(|| f(Arc::clone(&CANCELLATION_NOTIFIER)));
    
    // TODO: Defer this
    unsafe { signal(Signal::SIGINT, pyhandler).unwrap() };

    result
}

extern "C" fn ctrl_c_handler(_signal: core::ffi::c_int) {
    CANCELLATION_NOTIFIER.notify_last();
}
