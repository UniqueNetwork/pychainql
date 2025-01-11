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

use nix::sys::signal::{signal, SigHandler, Signal};
use pyo3::PyResult;
use std::sync::Arc;
use std::sync::LazyLock;
use tokio::sync::Notify;

static RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .thread_name("chainql-tokio-runtime")
        .enable_all()
        .build()
        .unwrap()
});

static CANCELLATION_NOTIFIER: LazyLock<Arc<Notify>> = LazyLock::new(|| Arc::new(Notify::new()));

struct CancellationGuard(SigHandler);

impl CancellationGuard {
    fn setup() -> Self {
        Self(unsafe { signal(Signal::SIGINT, SigHandler::Handler(Self::ctrl_c_handler)).unwrap() })
    }

    extern "C" fn ctrl_c_handler(_signal: core::ffi::c_int) {
        CANCELLATION_NOTIFIER.notify_one();
    }
}

impl Drop for CancellationGuard {
    fn drop(&mut self) {
        unsafe { signal(Signal::SIGINT, self.0).unwrap() };
    }
}

#[inline]
#[track_caller]
pub fn execute_jsonnet<F, T>(f: F) -> F::Output
where
    F: FnOnce(Arc<Notify>) -> PyResult<T>,
{
    let _ctrl_c = CancellationGuard::setup();
    let _enter_guard = RUNTIME.enter();

    let cancel = Arc::clone(&CANCELLATION_NOTIFIER);
    f(cancel)
}
