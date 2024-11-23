use tokio::task::block_in_place;

pub(crate) fn init() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .thread_name("chainql-tokio-runtime")
        .enable_all()
        .build()
        .unwrap();

    crate::RUNTIME.set(runtime).unwrap()
}

#[inline(always)]
pub(crate) fn execute_in_tokio<F: FnOnce() -> T, T>(f: F) -> T {
    let runtime = crate::RUNTIME.get().unwrap();
    let _enter_guard = runtime.enter();
    block_in_place(f)
}
