use async_trait::async_trait;

use crate::durable_host::DurableWorkerCtx;
use crate::metrics::wasm::record_host_function_call;
use crate::workerctx::WorkerCtx;
use wasmtime_wasi::preview2::bindings::wasi::cli::exit::Host;

#[async_trait]
impl<Ctx: WorkerCtx> Host for DurableWorkerCtx<Ctx> {
    fn exit(&mut self, status: Result<(), ()>) -> anyhow::Result<()> {
        record_host_function_call("cli::exit", "exit");
        Host::exit(&mut self.as_wasi_view(), status)
    }
}