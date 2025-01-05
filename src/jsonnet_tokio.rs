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
use pyo3::exceptions::PyChildProcessError;
use pyo3::{PyResult, Python};
use std::sync::atomic::Ordering;
use jsonnet::interop::threading::{jrsonnet_exit_thread, jrsonnet_reenter_thread};

#[repr(transparent)]
struct ForcedlySendable<T>(T);

unsafe impl<T> Send for ForcedlySendable<T> {}

impl<T> ForcedlySendable<T> {
    /// Hack the borrow checker.
    ///
    /// For some reason, if you access T in the thread closure,
    /// compiler will show error 'T is not Send'.
    #[inline(always)]
    fn into_inner(self) -> T {
        self.0
    }
}

// #[inline(always)]
// pub fn execute_jsonnet<F: FnOnce() -> T, T>(py: Python<'_>, f: F) -> T {
//     let use_logger = crate::ENABLE_LOGGER.load(Ordering::SeqCst);
//     let _gag = (!use_logger).then(|| (Gag::stdout().unwrap(), Gag::stderr().unwrap()));
//
//     let _enter_guard = crate::RUNTIME.enter();
//
//     tokio::task::block_in_place(f)
// }

#[track_caller]
pub fn execute_jsonnet<F, T>(py: Python<'_>, f: F) -> PyResult<T>
where
    F: for<'env> FnOnce() -> PyResult<T>,
{
    let jsonnet_ctx = unsafe { jrsonnet_exit_thread() };

    // Chainql actively uses println!(...) for logging without option to disable it.
    // As a workaround, the output is redirected to /dev/null using the Gag crate.
    let use_logger = crate::ENABLE_LOGGER.load(Ordering::SeqCst);
    let _gag = (!use_logger).then(|| (Gag::stdout().unwrap(), Gag::stderr().unwrap()));

    // Chainql methods are called in a separate thread to avoid blocking signal handling.
    let worker = {
        // Wrappers for chainql methods often capture jsonnet objects that
        // do not implement Send due to the non-atomic reference counter within them.
        //
        // To work around that "limitation", a hack was necessary to allow
        // passing the wrapper to the thread and the result of execution
        // from the thread.
        //
        // This is safe because we block the main thread until
        // the function execution is complete.
        let f = ForcedlySendable(f);

        || {
            // chainql_core uses jsonrpsee with websocket feature, which requires the Tokio Runtime.
            let _enter_guard = crate::RUNTIME.enter();

            // TODO
            unsafe { jrsonnet_reenter_thread(jsonnet_ctx) };
            let result = f.into_inner()();
            let jsonnet_ctx = unsafe { jrsonnet_exit_thread() };

            (ForcedlySendable(result), jsonnet_ctx)
        }
    };

    // SAFETY: The lifetime of a thread is less than the lifetime of a function.
    let jrsonnet_thread = unsafe {
        std::thread::Builder::new()
            .name("jrsonnet-thread".to_string())
            .spawn_unchecked(worker)
            .expect("failed to spawn thread for jrsonnet")
    };

    let (result, jsonnet_ctx) = loop {
        // Giving Python the ability to check sigint (Ctrl+C) or any other signals.
        py.check_signals()?;

        if !jrsonnet_thread.is_finished() {
            std::thread::yield_now();
            continue;
        }

        break match jrsonnet_thread.join() {
            Ok((ForcedlySendable(result), ctx)) => (result, ctx),
            Err(err) => {
                // const DEFAULT_MESSAGE: &str =
                //     "unknown error, please enable logs by chainql.enable_logs() for more details";
                //
                // let err_message = err
                //     .downcast::<String>()
                //     .map(|err_message| *err_message)
                //     .unwrap_or(DEFAULT_MESSAGE.to_owned());
                //
                // Err(PyChildProcessError::new_err(
                //     format!("jrsonnet unexpectedly panicked with message \"{err_message}\". This is bug, please report it to https://github.com/UniqueNetwork/pychainql/issues"),
                // ))
                todo!()
            }
        };
    };

    unsafe { jrsonnet_reenter_thread(jsonnet_ctx) };

    result
}
