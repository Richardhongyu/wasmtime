#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod api {
    //! Define the Rust interface a backend must implement in order to be used by
    //! this crate. the `Box<dyn ...>` types returned by these interfaces allow
    //! implementations to maintain backend-specific state between calls.
    use crate::witx::types::{ExecutionTarget, GraphBuilderArray, Tensor};
    use thiserror::Error;
    use wiggle::GuestError;
    /// A [Backend] contains the necessary state to load [BackendGraph]s.
    pub(crate) trait Backend: Send + Sync {
        fn name(&self) -> &str;
        fn load(
            &mut self,
            builders: &GraphBuilderArray<'_>,
            target: ExecutionTarget,
        ) -> Result<Box<dyn BackendGraph>, BackendError>;
    }
    /// A [BackendGraph] can create [BackendExecutionContext]s; this is the backing
    /// implementation for a [crate::witx::types::Graph].
    pub(crate) trait BackendGraph: Send + Sync {
        fn init_execution_context(
            &mut self,
        ) -> Result<Box<dyn BackendExecutionContext>, BackendError>;
    }
    /// A [BackendExecutionContext] performs the actual inference; this is the
    /// backing implementation for a [crate::witx::types::GraphExecutionContext].
    pub(crate) trait BackendExecutionContext: Send + Sync {
        fn set_input(
            &mut self,
            index: u32,
            tensor: &Tensor<'_>,
        ) -> Result<(), BackendError>;
        fn compute(&mut self) -> Result<(), BackendError>;
        fn get_output(
            &mut self,
            index: u32,
            destination: &mut [u8],
        ) -> Result<u32, BackendError>;
    }
    /// Errors returned by a backend; [BackendError::BackendAccess] is a catch-all
    /// for failures interacting with the ML library.
    pub enum BackendError {
        #[error("Failed while accessing backend")]
        BackendAccess(#[from] anyhow::Error),
        #[error("Failed while accessing guest module")]
        GuestAccess(#[from] GuestError),
        #[error("The backend expects {0} buffers, passed {1}")]
        InvalidNumberOfBuilders(u32, u32),
        #[error("Not enough memory to copy tensor data of size: {0}")]
        NotEnoughMemory(usize),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BackendError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                BackendError::BackendAccess(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BackendAccess",
                        &__self_0,
                    )
                }
                BackendError::GuestAccess(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GuestAccess",
                        &__self_0,
                    )
                }
                BackendError::InvalidNumberOfBuilders(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "InvalidNumberOfBuilders",
                        __self_0,
                        &__self_1,
                    )
                }
                BackendError::NotEnoughMemory(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "NotEnoughMemory",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for BackendError {
        fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
            use thiserror::private::AsDynError;
            #[allow(deprecated)]
            match self {
                BackendError::BackendAccess { 0: source, .. } => {
                    std::option::Option::Some(source.as_dyn_error())
                }
                BackendError::GuestAccess { 0: source, .. } => {
                    std::option::Option::Some(source.as_dyn_error())
                }
                BackendError::InvalidNumberOfBuilders { .. } => std::option::Option::None,
                BackendError::NotEnoughMemory { .. } => std::option::Option::None,
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::fmt::Display for BackendError {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_imports)]
            use thiserror::private::{DisplayAsDisplay, PathAsDisplay};
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                BackendError::BackendAccess(_0) => {
                    __formatter.write_fmt(format_args!("Failed while accessing backend"))
                }
                BackendError::GuestAccess(_0) => {
                    __formatter
                        .write_fmt(format_args!("Failed while accessing guest module"))
                }
                BackendError::InvalidNumberOfBuilders(_0, _1) => {
                    __formatter
                        .write_fmt(
                            format_args!(
                                "The backend expects {0} buffers, passed {1}", _0
                                .as_display(), _1.as_display()
                            ),
                        )
                }
                BackendError::NotEnoughMemory(_0) => {
                    __formatter
                        .write_fmt(
                            format_args!(
                                "Not enough memory to copy tensor data of size: {0}", _0
                                .as_display()
                            ),
                        )
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<anyhow::Error> for BackendError {
        #[allow(deprecated)]
        fn from(source: anyhow::Error) -> Self {
            BackendError::BackendAccess {
                0: source,
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<GuestError> for BackendError {
        #[allow(deprecated)]
        fn from(source: GuestError) -> Self {
            BackendError::GuestAccess {
                0: source,
            }
        }
    }
}
mod ctx {
    //! Implements the base structure (i.e. [WasiNnCtx]) that will provide the
    //! implementation of the wasi-nn API.
    use crate::api::{Backend, BackendError, BackendExecutionContext, BackendGraph};
    use crate::openvino::OpenvinoBackend;
    use crate::r#impl::UsageError;
    use crate::witx::types::{Graph, GraphEncoding, GraphExecutionContext};
    use std::collections::HashMap;
    use std::hash::Hash;
    use thiserror::Error;
    use wiggle::GuestError;
    /// Capture the state necessary for calling into the backend ML libraries.
    pub struct WasiNnCtx {
        pub(crate) backends: HashMap<u8, Box<dyn Backend>>,
        pub(crate) graphs: Table<Graph, Box<dyn BackendGraph>>,
        pub(crate) executions: Table<
            GraphExecutionContext,
            Box<dyn BackendExecutionContext>,
        >,
    }
    impl WasiNnCtx {
        /// Make a new context from the default state.
        pub fn new() -> WasiNnResult<Self> {
            let mut backends = HashMap::new();
            backends
                .insert(
                    GraphEncoding::Openvino.into(),
                    Box::new(OpenvinoBackend::default()) as Box<dyn Backend>,
                );
            Ok(Self {
                backends,
                graphs: Table::default(),
                executions: Table::default(),
            })
        }
    }
    /// Possible errors while interacting with [WasiNnCtx].
    pub enum WasiNnError {
        #[error("backend error")]
        BackendError(#[from] BackendError),
        #[error("guest error")]
        GuestError(#[from] GuestError),
        #[error("usage error")]
        UsageError(#[from] UsageError),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for WasiNnError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                WasiNnError::BackendError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "BackendError",
                        &__self_0,
                    )
                }
                WasiNnError::GuestError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "GuestError",
                        &__self_0,
                    )
                }
                WasiNnError::UsageError(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "UsageError",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for WasiNnError {
        fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
            use thiserror::private::AsDynError;
            #[allow(deprecated)]
            match self {
                WasiNnError::BackendError { 0: source, .. } => {
                    std::option::Option::Some(source.as_dyn_error())
                }
                WasiNnError::GuestError { 0: source, .. } => {
                    std::option::Option::Some(source.as_dyn_error())
                }
                WasiNnError::UsageError { 0: source, .. } => {
                    std::option::Option::Some(source.as_dyn_error())
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::fmt::Display for WasiNnError {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                WasiNnError::BackendError(_0) => {
                    __formatter.write_fmt(format_args!("backend error"))
                }
                WasiNnError::GuestError(_0) => {
                    __formatter.write_fmt(format_args!("guest error"))
                }
                WasiNnError::UsageError(_0) => {
                    __formatter.write_fmt(format_args!("usage error"))
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<BackendError> for WasiNnError {
        #[allow(deprecated)]
        fn from(source: BackendError) -> Self {
            WasiNnError::BackendError {
                0: source,
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<GuestError> for WasiNnError {
        #[allow(deprecated)]
        fn from(source: GuestError) -> Self {
            WasiNnError::GuestError {
                0: source,
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<UsageError> for WasiNnError {
        #[allow(deprecated)]
        fn from(source: UsageError) -> Self {
            WasiNnError::UsageError {
                0: source,
            }
        }
    }
    pub(crate) type WasiNnResult<T> = std::result::Result<T, WasiNnError>;
    /// Record handle entries in a table.
    pub struct Table<K, V> {
        entries: HashMap<K, V>,
        next_key: u32,
    }
    impl<K, V> Default for Table<K, V> {
        fn default() -> Self {
            Self {
                entries: HashMap::new(),
                next_key: 0,
            }
        }
    }
    impl<K, V> Table<K, V>
    where
        K: Eq + Hash + From<u32> + Copy,
    {
        pub fn insert(&mut self, value: V) -> K {
            let key = self.use_next_key();
            self.entries.insert(key, value);
            key
        }
        pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
            self.entries.get_mut(&key)
        }
        fn use_next_key(&mut self) -> K {
            let current = self.next_key;
            self.next_key += 1;
            K::from(current)
        }
    }
}
mod r#impl {
    //! Implements the wasi-nn API.
    use crate::ctx::WasiNnResult as Result;
    use crate::witx::types::{
        ExecutionTarget, Graph, GraphBuilderArray, GraphEncoding, GraphExecutionContext,
        Tensor,
    };
    use crate::witx::wasi_ephemeral_nn::WasiEphemeralNn;
    use crate::WasiNnCtx;
    use thiserror::Error;
    use wiggle::GuestPtr;
    pub enum UsageError {
        #[error("Invalid context; has the load function been called?")]
        InvalidContext,
        #[error("Only OpenVINO's IR is currently supported, passed encoding: {0:?}")]
        InvalidEncoding(GraphEncoding),
        #[error("OpenVINO expects only two buffers (i.e. [ir, weights]), passed: {0}")]
        InvalidNumberOfBuilders(u32),
        #[error("Invalid graph handle; has it been loaded?")]
        InvalidGraphHandle,
        #[error("Invalid execution context handle; has it been initialized?")]
        InvalidExecutionContextHandle,
        #[error("Not enough memory to copy tensor data of size: {0}")]
        NotEnoughMemory(u32),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UsageError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                UsageError::InvalidContext => {
                    ::core::fmt::Formatter::write_str(f, "InvalidContext")
                }
                UsageError::InvalidEncoding(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidEncoding",
                        &__self_0,
                    )
                }
                UsageError::InvalidNumberOfBuilders(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "InvalidNumberOfBuilders",
                        &__self_0,
                    )
                }
                UsageError::InvalidGraphHandle => {
                    ::core::fmt::Formatter::write_str(f, "InvalidGraphHandle")
                }
                UsageError::InvalidExecutionContextHandle => {
                    ::core::fmt::Formatter::write_str(f, "InvalidExecutionContextHandle")
                }
                UsageError::NotEnoughMemory(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "NotEnoughMemory",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for UsageError {}
    #[allow(unused_qualifications)]
    impl std::fmt::Display for UsageError {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_imports)]
            use thiserror::private::{DisplayAsDisplay, PathAsDisplay};
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                UsageError::InvalidContext {} => {
                    __formatter
                        .write_fmt(
                            format_args!(
                                "Invalid context; has the load function been called?"
                            ),
                        )
                }
                UsageError::InvalidEncoding(_0) => {
                    __formatter
                        .write_fmt(
                            format_args!(
                                "Only OpenVINO\'s IR is currently supported, passed encoding: {0:?}",
                                _0
                            ),
                        )
                }
                UsageError::InvalidNumberOfBuilders(_0) => {
                    __formatter
                        .write_fmt(
                            format_args!(
                                "OpenVINO expects only two buffers (i.e. [ir, weights]), passed: {0}",
                                _0.as_display()
                            ),
                        )
                }
                UsageError::InvalidGraphHandle {} => {
                    __formatter
                        .write_fmt(
                            format_args!("Invalid graph handle; has it been loaded?"),
                        )
                }
                UsageError::InvalidExecutionContextHandle {} => {
                    __formatter
                        .write_fmt(
                            format_args!(
                                "Invalid execution context handle; has it been initialized?"
                            ),
                        )
                }
                UsageError::NotEnoughMemory(_0) => {
                    __formatter
                        .write_fmt(
                            format_args!(
                                "Not enough memory to copy tensor data of size: {0}", _0
                                .as_display()
                            ),
                        )
                }
            }
        }
    }
    impl<'a> WasiEphemeralNn for WasiNnCtx {
        fn load<'b>(
            &mut self,
            builders: &GraphBuilderArray<'_>,
            encoding: GraphEncoding,
            target: ExecutionTarget,
        ) -> Result<Graph> {
            let encoding_id: u8 = encoding.into();
            let graph = if let Some(backend) = self.backends.get_mut(&encoding_id) {
                backend.load(builders, target)?
            } else {
                return Err(UsageError::InvalidEncoding(encoding).into());
            };
            let graph_id = self.graphs.insert(graph);
            Ok(graph_id)
        }
        fn init_execution_context(
            &mut self,
            graph_id: Graph,
        ) -> Result<GraphExecutionContext> {
            let exec_context = if let Some(graph) = self.graphs.get_mut(graph_id) {
                graph.init_execution_context()?
            } else {
                return Err(UsageError::InvalidGraphHandle.into());
            };
            let exec_context_id = self.executions.insert(exec_context);
            Ok(exec_context_id)
        }
        fn set_input<'b>(
            &mut self,
            exec_context_id: GraphExecutionContext,
            index: u32,
            tensor: &Tensor<'b>,
        ) -> Result<()> {
            if let Some(exec_context) = self.executions.get_mut(exec_context_id) {
                Ok(exec_context.set_input(index, tensor)?)
            } else {
                Err(UsageError::InvalidGraphHandle.into())
            }
        }
        fn compute(&mut self, exec_context_id: GraphExecutionContext) -> Result<()> {
            if let Some(exec_context) = self.executions.get_mut(exec_context_id) {
                Ok(exec_context.compute()?)
            } else {
                Err(UsageError::InvalidExecutionContextHandle.into())
            }
        }
        fn get_output<'b>(
            &mut self,
            exec_context_id: GraphExecutionContext,
            index: u32,
            out_buffer: &GuestPtr<'_, u8>,
            out_buffer_max_size: u32,
        ) -> Result<u32> {
            if let Some(exec_context) = self.executions.get_mut(exec_context_id) {
                let mut destination = out_buffer
                    .as_array(out_buffer_max_size)
                    .as_slice_mut()?
                    .expect(
                        "cannot use with shared memories; see https://github.com/bytecodealliance/wasmtime/issues/5235 (TODO)",
                    );
                Ok(exec_context.get_output(index, &mut destination)?)
            } else {
                Err(UsageError::InvalidGraphHandle.into())
            }
        }
    }
}
mod openvino {
    //! Implements the wasi-nn API.
    use crate::api::{Backend, BackendError, BackendExecutionContext, BackendGraph};
    use crate::witx::types::{ExecutionTarget, GraphBuilderArray, Tensor, TensorType};
    use openvino::{InferenceError, Layout, Precision, SetupError, TensorDesc};
    use std::sync::Arc;
    pub(crate) struct OpenvinoBackend(Option<openvino::Core>);
    #[automatically_derived]
    impl ::core::default::Default for OpenvinoBackend {
        #[inline]
        fn default() -> OpenvinoBackend {
            OpenvinoBackend(::core::default::Default::default())
        }
    }
    unsafe impl Send for OpenvinoBackend {}
    unsafe impl Sync for OpenvinoBackend {}
    impl Backend for OpenvinoBackend {
        fn name(&self) -> &str {
            "openvino"
        }
        fn load(
            &mut self,
            builders: &GraphBuilderArray<'_>,
            target: ExecutionTarget,
        ) -> Result<Box<dyn BackendGraph>, BackendError> {
            if builders.len() != 2 {
                return Err(
                    BackendError::InvalidNumberOfBuilders(2, builders.len()).into(),
                );
            }
            if self.0.is_none() {
                self.0.replace(openvino::Core::new(None)?);
            }
            let builders = builders.as_ptr();
            let xml = builders
                .read()?
                .as_slice()?
                .expect(
                    "cannot use with shared memories; see https://github.com/bytecodealliance/wasmtime/issues/5235 (TODO)",
                );
            let weights = builders
                .add(1)?
                .read()?
                .as_slice()?
                .expect(
                    "cannot use with shared memories; see https://github.com/bytecodealliance/wasmtime/issues/5235 (TODO)",
                );
            let core = self
                .0
                .as_mut()
                .expect("openvino::Core was previously constructed");
            let mut cnn_network = core.read_network_from_buffer(&xml, &weights)?;
            for i in 0..cnn_network.get_inputs_len()? {
                let name = cnn_network.get_input_name(i)?;
                cnn_network.set_input_layout(&name, Layout::NHWC)?;
            }
            let exec_network = core
                .load_network(&cnn_network, map_execution_target_to_string(target))?;
            Ok(Box::new(OpenvinoGraph(Arc::new(cnn_network), exec_network)))
        }
    }
    struct OpenvinoGraph(Arc<openvino::CNNNetwork>, openvino::ExecutableNetwork);
    unsafe impl Send for OpenvinoGraph {}
    unsafe impl Sync for OpenvinoGraph {}
    impl BackendGraph for OpenvinoGraph {
        fn init_execution_context(
            &mut self,
        ) -> Result<Box<dyn BackendExecutionContext>, BackendError> {
            let infer_request = self.1.create_infer_request()?;
            Ok(Box::new(OpenvinoExecutionContext(self.0.clone(), infer_request)))
        }
    }
    struct OpenvinoExecutionContext(Arc<openvino::CNNNetwork>, openvino::InferRequest);
    impl BackendExecutionContext for OpenvinoExecutionContext {
        fn set_input(
            &mut self,
            index: u32,
            tensor: &Tensor<'_>,
        ) -> Result<(), BackendError> {
            let input_name = self.0.get_input_name(index as usize)?;
            let dimensions = tensor
                .dimensions
                .as_slice()?
                .expect(
                    "cannot use with shared memories; see https://github.com/bytecodealliance/wasmtime/issues/5235 (TODO)",
                )
                .iter()
                .map(|d| *d as usize)
                .collect::<Vec<_>>();
            let precision = map_tensor_type_to_precision(tensor.type_);
            let desc = TensorDesc::new(Layout::NHWC, &dimensions, precision);
            let data = tensor
                .data
                .as_slice()?
                .expect(
                    "cannot use with shared memories; see https://github.com/bytecodealliance/wasmtime/issues/5235 (TODO)",
                );
            let blob = openvino::Blob::new(&desc, &data)?;
            self.1.set_blob(&input_name, &blob)?;
            Ok(())
        }
        fn compute(&mut self) -> Result<(), BackendError> {
            self.1.infer()?;
            Ok(())
        }
        fn get_output(
            &mut self,
            index: u32,
            destination: &mut [u8],
        ) -> Result<u32, BackendError> {
            let output_name = self.0.get_output_name(index as usize)?;
            let mut blob = self.1.get_blob(&output_name)?;
            let blob_size = blob.byte_len()?;
            if blob_size > destination.len() {
                return Err(BackendError::NotEnoughMemory(blob_size));
            }
            destination[..blob_size].copy_from_slice(blob.buffer()?);
            Ok(blob_size as u32)
        }
    }
    impl From<InferenceError> for BackendError {
        fn from(e: InferenceError) -> Self {
            BackendError::BackendAccess(anyhow::Error::new(e))
        }
    }
    impl From<SetupError> for BackendError {
        fn from(e: SetupError) -> Self {
            BackendError::BackendAccess(anyhow::Error::new(e))
        }
    }
    /// Return the execution target string expected by OpenVINO from the
    /// `ExecutionTarget` enum provided by wasi-nn.
    fn map_execution_target_to_string(target: ExecutionTarget) -> &'static str {
        match target {
            ExecutionTarget::Cpu => "CPU",
            ExecutionTarget::Gpu => "GPU",
            ExecutionTarget::Tpu => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "not implemented: {0}",
                        format_args!("OpenVINO does not support TPU execution targets")
                    ),
                )
            }
        }
    }
    /// Return OpenVINO's precision type for the `TensorType` enum provided by
    /// wasi-nn.
    fn map_tensor_type_to_precision(tensor_type: TensorType) -> openvino::Precision {
        match tensor_type {
            TensorType::F16 => Precision::FP16,
            TensorType::F32 => Precision::FP32,
            TensorType::U8 => Precision::U8,
            TensorType::I32 => Precision::I32,
        }
    }
}
mod witx {
    //! Contains the macro-generated implementation of wasi-nn from the its witx definition file.
    use crate::ctx::WasiNnCtx;
    use crate::ctx::WasiNnError;
    use anyhow::Result;
    pub mod types {
        use std::convert::TryFrom;
        pub type BufferSize = u32;
        pub enum NnErrno {
            Success,
            InvalidArgument,
            MissingMemory,
            Busy,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for NnErrno {
            #[inline]
            fn clone(&self) -> NnErrno {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for NnErrno {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        NnErrno::Success => "Success",
                        NnErrno::InvalidArgument => "InvalidArgument",
                        NnErrno::MissingMemory => "MissingMemory",
                        NnErrno::Busy => "Busy",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for NnErrno {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for NnErrno {
            #[inline]
            fn eq(&self, other: &NnErrno) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for NnErrno {}
        impl TryFrom<u16> for NnErrno {
            type Error = wiggle::GuestError;
            #[inline]
            fn try_from(value: u16) -> Result<NnErrno, wiggle::GuestError> {
                match value {
                    0 => Ok(NnErrno::Success),
                    1 => Ok(NnErrno::InvalidArgument),
                    2 => Ok(NnErrno::MissingMemory),
                    3 => Ok(NnErrno::Busy),
                    _ => Err(wiggle::GuestError::InvalidEnumValue("NnErrno")),
                }
            }
        }
        impl TryFrom<i32> for NnErrno {
            type Error = wiggle::GuestError;
            #[inline]
            fn try_from(value: i32) -> Result<NnErrno, wiggle::GuestError> {
                NnErrno::try_from(u16::try_from(value)?)
            }
        }
        impl From<NnErrno> for u16 {
            #[inline]
            fn from(v: NnErrno) -> u16 {
                match v {
                    NnErrno::Success => 0,
                    NnErrno::InvalidArgument => 1,
                    NnErrno::MissingMemory => 2,
                    NnErrno::Busy => 3,
                }
            }
        }
        impl<'a> wiggle::GuestType<'a> for NnErrno {
            #[inline]
            fn guest_size() -> u32 {
                2u32
            }
            #[inline]
            fn guest_align() -> usize {
                2usize
            }
            fn read(
                location: &wiggle::GuestPtr<'a, Self>,
            ) -> Result<Self, wiggle::GuestError> {
                let tag = location.cast::<u16>().read()?;
                match tag {
                    0 => Ok(NnErrno::Success),
                    1 => Ok(NnErrno::InvalidArgument),
                    2 => Ok(NnErrno::MissingMemory),
                    3 => Ok(NnErrno::Busy),
                    _ => Err(wiggle::GuestError::InvalidEnumValue("NnErrno")),
                }
            }
            fn write(
                location: &wiggle::GuestPtr<'_, Self>,
                val: Self,
            ) -> Result<(), wiggle::GuestError> {
                match val {
                    NnErrno::Success => {
                        location.cast().write(0usize as u16)?;
                    }
                    NnErrno::InvalidArgument => {
                        location.cast().write(1usize as u16)?;
                    }
                    NnErrno::MissingMemory => {
                        location.cast().write(2usize as u16)?;
                    }
                    NnErrno::Busy => {
                        location.cast().write(3usize as u16)?;
                    }
                }
                Ok(())
            }
        }
        pub type TensorDimensions<'a> = wiggle::GuestPtr<'a, [u32]>;
        pub enum TensorType {
            F16,
            F32,
            U8,
            I32,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TensorType {
            #[inline]
            fn clone(&self) -> TensorType {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TensorType {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        TensorType::F16 => "F16",
                        TensorType::F32 => "F32",
                        TensorType::U8 => "U8",
                        TensorType::I32 => "I32",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TensorType {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TensorType {
            #[inline]
            fn eq(&self, other: &TensorType) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for TensorType {}
        impl TryFrom<u8> for TensorType {
            type Error = wiggle::GuestError;
            #[inline]
            fn try_from(value: u8) -> Result<TensorType, wiggle::GuestError> {
                match value {
                    0 => Ok(TensorType::F16),
                    1 => Ok(TensorType::F32),
                    2 => Ok(TensorType::U8),
                    3 => Ok(TensorType::I32),
                    _ => Err(wiggle::GuestError::InvalidEnumValue("TensorType")),
                }
            }
        }
        impl TryFrom<i32> for TensorType {
            type Error = wiggle::GuestError;
            #[inline]
            fn try_from(value: i32) -> Result<TensorType, wiggle::GuestError> {
                TensorType::try_from(u8::try_from(value)?)
            }
        }
        impl From<TensorType> for u8 {
            #[inline]
            fn from(v: TensorType) -> u8 {
                match v {
                    TensorType::F16 => 0,
                    TensorType::F32 => 1,
                    TensorType::U8 => 2,
                    TensorType::I32 => 3,
                }
            }
        }
        impl<'a> wiggle::GuestType<'a> for TensorType {
            #[inline]
            fn guest_size() -> u32 {
                1u32
            }
            #[inline]
            fn guest_align() -> usize {
                1usize
            }
            fn read(
                location: &wiggle::GuestPtr<'a, Self>,
            ) -> Result<Self, wiggle::GuestError> {
                let tag = location.cast::<u8>().read()?;
                match tag {
                    0 => Ok(TensorType::F16),
                    1 => Ok(TensorType::F32),
                    2 => Ok(TensorType::U8),
                    3 => Ok(TensorType::I32),
                    _ => Err(wiggle::GuestError::InvalidEnumValue("TensorType")),
                }
            }
            fn write(
                location: &wiggle::GuestPtr<'_, Self>,
                val: Self,
            ) -> Result<(), wiggle::GuestError> {
                match val {
                    TensorType::F16 => {
                        location.cast().write(0usize as u8)?;
                    }
                    TensorType::F32 => {
                        location.cast().write(1usize as u8)?;
                    }
                    TensorType::U8 => {
                        location.cast().write(2usize as u8)?;
                    }
                    TensorType::I32 => {
                        location.cast().write(3usize as u8)?;
                    }
                }
                Ok(())
            }
        }
        pub type TensorData<'a> = wiggle::GuestPtr<'a, [u8]>;
        pub struct Tensor<'a> {
            pub dimensions: TensorDimensions<'a>,
            pub type_: TensorType,
            pub data: TensorData<'a>,
        }
        #[automatically_derived]
        impl<'a> ::core::clone::Clone for Tensor<'a> {
            #[inline]
            fn clone(&self) -> Tensor<'a> {
                Tensor {
                    dimensions: ::core::clone::Clone::clone(&self.dimensions),
                    type_: ::core::clone::Clone::clone(&self.type_),
                    data: ::core::clone::Clone::clone(&self.data),
                }
            }
        }
        #[automatically_derived]
        impl<'a> ::core::fmt::Debug for Tensor<'a> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Tensor",
                    "dimensions",
                    &self.dimensions,
                    "type_",
                    &self.type_,
                    "data",
                    &&self.data,
                )
            }
        }
        impl<'a> wiggle::GuestType<'a> for Tensor<'a> {
            #[inline]
            fn guest_size() -> u32 {
                20u32
            }
            #[inline]
            fn guest_align() -> usize {
                4usize
            }
            fn read(
                location: &wiggle::GuestPtr<'a, Self>,
            ) -> Result<Self, wiggle::GuestError> {
                let dimensions = <TensorDimensions as wiggle::GuestType>::read(
                    &location.cast::<u8>().add(0u32)?.cast(),
                )?;
                let type_ = <TensorType as wiggle::GuestType>::read(
                    &location.cast::<u8>().add(8u32)?.cast(),
                )?;
                let data = <TensorData as wiggle::GuestType>::read(
                    &location.cast::<u8>().add(12u32)?.cast(),
                )?;
                Ok(Tensor { dimensions, type_, data })
            }
            fn write(
                location: &wiggle::GuestPtr<'_, Self>,
                val: Self,
            ) -> Result<(), wiggle::GuestError> {
                wiggle::GuestType::write(
                    &location.cast::<u8>().add(0u32)?.cast(),
                    val.dimensions,
                )?;
                wiggle::GuestType::write(
                    &location.cast::<u8>().add(8u32)?.cast(),
                    val.type_,
                )?;
                wiggle::GuestType::write(
                    &location.cast::<u8>().add(12u32)?.cast(),
                    val.data,
                )?;
                Ok(())
            }
        }
        pub type GraphBuilder<'a> = wiggle::GuestPtr<'a, [u8]>;
        pub type GraphBuilderArray<'a> = wiggle::GuestPtr<'a, [GraphBuilder<'a>]>;
        #[repr(transparent)]
        pub struct Graph(u32);
        #[automatically_derived]
        impl ::core::marker::Copy for Graph {}
        #[automatically_derived]
        impl ::core::clone::Clone for Graph {
            #[inline]
            fn clone(&self) -> Graph {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Graph {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Graph", &&self.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Graph {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Graph {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Graph {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Graph {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Graph {
            #[inline]
            fn eq(&self, other: &Graph) -> bool {
                self.0 == other.0
            }
        }
        impl Graph {
            #[inline]
            pub unsafe fn inner(&self) -> u32 {
                self.0
            }
        }
        impl From<Graph> for u32 {
            #[inline]
            fn from(e: Graph) -> u32 {
                e.0
            }
        }
        impl From<Graph> for i32 {
            #[inline]
            fn from(e: Graph) -> i32 {
                e.0 as i32
            }
        }
        impl From<u32> for Graph {
            #[inline]
            fn from(e: u32) -> Graph {
                Graph(e)
            }
        }
        impl From<i32> for Graph {
            #[inline]
            fn from(e: i32) -> Graph {
                Graph(e as u32)
            }
        }
        impl ::std::fmt::Display for Graph {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0}({1})", "Graph", self.0))
            }
        }
        impl<'a> wiggle::GuestType<'a> for Graph {
            #[inline]
            fn guest_size() -> u32 {
                4u32
            }
            #[inline]
            fn guest_align() -> usize {
                4usize
            }
            #[inline]
            fn read(
                location: &wiggle::GuestPtr<'a, Graph>,
            ) -> Result<Graph, wiggle::GuestError> {
                Ok(Graph(u32::read(&location.cast())?))
            }
            #[inline]
            fn write(
                location: &wiggle::GuestPtr<'_, Self>,
                val: Self,
            ) -> Result<(), wiggle::GuestError> {
                u32::write(&location.cast(), val.0)
            }
        }
        pub enum GraphEncoding {
            Openvino,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for GraphEncoding {
            #[inline]
            fn clone(&self) -> GraphEncoding {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for GraphEncoding {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "Openvino")
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for GraphEncoding {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for GraphEncoding {
            #[inline]
            fn eq(&self, other: &GraphEncoding) -> bool {
                true
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for GraphEncoding {}
        impl TryFrom<u8> for GraphEncoding {
            type Error = wiggle::GuestError;
            #[inline]
            fn try_from(value: u8) -> Result<GraphEncoding, wiggle::GuestError> {
                match value {
                    0 => Ok(GraphEncoding::Openvino),
                    _ => Err(wiggle::GuestError::InvalidEnumValue("GraphEncoding")),
                }
            }
        }
        impl TryFrom<i32> for GraphEncoding {
            type Error = wiggle::GuestError;
            #[inline]
            fn try_from(value: i32) -> Result<GraphEncoding, wiggle::GuestError> {
                GraphEncoding::try_from(u8::try_from(value)?)
            }
        }
        impl From<GraphEncoding> for u8 {
            #[inline]
            fn from(v: GraphEncoding) -> u8 {
                match v {
                    GraphEncoding::Openvino => 0,
                }
            }
        }
        impl<'a> wiggle::GuestType<'a> for GraphEncoding {
            #[inline]
            fn guest_size() -> u32 {
                1u32
            }
            #[inline]
            fn guest_align() -> usize {
                1usize
            }
            fn read(
                location: &wiggle::GuestPtr<'a, Self>,
            ) -> Result<Self, wiggle::GuestError> {
                let tag = location.cast::<u8>().read()?;
                match tag {
                    0 => Ok(GraphEncoding::Openvino),
                    _ => Err(wiggle::GuestError::InvalidEnumValue("GraphEncoding")),
                }
            }
            fn write(
                location: &wiggle::GuestPtr<'_, Self>,
                val: Self,
            ) -> Result<(), wiggle::GuestError> {
                match val {
                    GraphEncoding::Openvino => {
                        location.cast().write(0usize as u8)?;
                    }
                }
                Ok(())
            }
        }
        pub enum ExecutionTarget {
            Cpu,
            Gpu,
            Tpu,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for ExecutionTarget {
            #[inline]
            fn clone(&self) -> ExecutionTarget {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ExecutionTarget {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        ExecutionTarget::Cpu => "Cpu",
                        ExecutionTarget::Gpu => "Gpu",
                        ExecutionTarget::Tpu => "Tpu",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for ExecutionTarget {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ExecutionTarget {
            #[inline]
            fn eq(&self, other: &ExecutionTarget) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for ExecutionTarget {}
        impl TryFrom<u8> for ExecutionTarget {
            type Error = wiggle::GuestError;
            #[inline]
            fn try_from(value: u8) -> Result<ExecutionTarget, wiggle::GuestError> {
                match value {
                    0 => Ok(ExecutionTarget::Cpu),
                    1 => Ok(ExecutionTarget::Gpu),
                    2 => Ok(ExecutionTarget::Tpu),
                    _ => Err(wiggle::GuestError::InvalidEnumValue("ExecutionTarget")),
                }
            }
        }
        impl TryFrom<i32> for ExecutionTarget {
            type Error = wiggle::GuestError;
            #[inline]
            fn try_from(value: i32) -> Result<ExecutionTarget, wiggle::GuestError> {
                ExecutionTarget::try_from(u8::try_from(value)?)
            }
        }
        impl From<ExecutionTarget> for u8 {
            #[inline]
            fn from(v: ExecutionTarget) -> u8 {
                match v {
                    ExecutionTarget::Cpu => 0,
                    ExecutionTarget::Gpu => 1,
                    ExecutionTarget::Tpu => 2,
                }
            }
        }
        impl<'a> wiggle::GuestType<'a> for ExecutionTarget {
            #[inline]
            fn guest_size() -> u32 {
                1u32
            }
            #[inline]
            fn guest_align() -> usize {
                1usize
            }
            fn read(
                location: &wiggle::GuestPtr<'a, Self>,
            ) -> Result<Self, wiggle::GuestError> {
                let tag = location.cast::<u8>().read()?;
                match tag {
                    0 => Ok(ExecutionTarget::Cpu),
                    1 => Ok(ExecutionTarget::Gpu),
                    2 => Ok(ExecutionTarget::Tpu),
                    _ => Err(wiggle::GuestError::InvalidEnumValue("ExecutionTarget")),
                }
            }
            fn write(
                location: &wiggle::GuestPtr<'_, Self>,
                val: Self,
            ) -> Result<(), wiggle::GuestError> {
                match val {
                    ExecutionTarget::Cpu => {
                        location.cast().write(0usize as u8)?;
                    }
                    ExecutionTarget::Gpu => {
                        location.cast().write(1usize as u8)?;
                    }
                    ExecutionTarget::Tpu => {
                        location.cast().write(2usize as u8)?;
                    }
                }
                Ok(())
            }
        }
        #[repr(transparent)]
        pub struct GraphExecutionContext(u32);
        #[automatically_derived]
        impl ::core::marker::Copy for GraphExecutionContext {}
        #[automatically_derived]
        impl ::core::clone::Clone for GraphExecutionContext {
            #[inline]
            fn clone(&self) -> GraphExecutionContext {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for GraphExecutionContext {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(
                    f,
                    "GraphExecutionContext",
                    &&self.0,
                )
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for GraphExecutionContext {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for GraphExecutionContext {}
        #[automatically_derived]
        impl ::core::cmp::Eq for GraphExecutionContext {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for GraphExecutionContext {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for GraphExecutionContext {
            #[inline]
            fn eq(&self, other: &GraphExecutionContext) -> bool {
                self.0 == other.0
            }
        }
        impl GraphExecutionContext {
            #[inline]
            pub unsafe fn inner(&self) -> u32 {
                self.0
            }
        }
        impl From<GraphExecutionContext> for u32 {
            #[inline]
            fn from(e: GraphExecutionContext) -> u32 {
                e.0
            }
        }
        impl From<GraphExecutionContext> for i32 {
            #[inline]
            fn from(e: GraphExecutionContext) -> i32 {
                e.0 as i32
            }
        }
        impl From<u32> for GraphExecutionContext {
            #[inline]
            fn from(e: u32) -> GraphExecutionContext {
                GraphExecutionContext(e)
            }
        }
        impl From<i32> for GraphExecutionContext {
            #[inline]
            fn from(e: i32) -> GraphExecutionContext {
                GraphExecutionContext(e as u32)
            }
        }
        impl ::std::fmt::Display for GraphExecutionContext {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0}({1})", "GraphExecutionContext", self.0))
            }
        }
        impl<'a> wiggle::GuestType<'a> for GraphExecutionContext {
            #[inline]
            fn guest_size() -> u32 {
                4u32
            }
            #[inline]
            fn guest_align() -> usize {
                4usize
            }
            #[inline]
            fn read(
                location: &wiggle::GuestPtr<'a, GraphExecutionContext>,
            ) -> Result<GraphExecutionContext, wiggle::GuestError> {
                Ok(GraphExecutionContext(u32::read(&location.cast())?))
            }
            #[inline]
            fn write(
                location: &wiggle::GuestPtr<'_, Self>,
                val: Self,
            ) -> Result<(), wiggle::GuestError> {
                u32::write(&location.cast(), val.0)
            }
        }
        pub trait UserErrorConversion {
            fn nn_errno_from_wasi_nn_error(
                &mut self,
                e: super::WasiNnError,
            ) -> wiggle::anyhow::Result<NnErrno>;
        }
    }
    pub mod wasi_ephemeral_nn {
        use super::types::*;
        pub use super::types::UserErrorConversion;
        #[allow(unreachable_code)]
        pub fn load<'a>(
            ctx: &'a mut (impl WasiEphemeralNn + UserErrorConversion),
            memory: &dyn wiggle::GuestMemory,
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
        ) -> wiggle::anyhow::Result<i32> {
            use std::convert::TryFrom as _;
            let _span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "wiggle abi",
                            "wasmtime_wasi_nn::witx::wasi_ephemeral_nn",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-nn/src/witx.rs"),
                            Some(7u32),
                            Some("wasmtime_wasi_nn::witx::wasi_ephemeral_nn"),
                            ::tracing_core::field::FieldSet::new(
                                &["module", "function"],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    } && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(
                        meta,
                        &{
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = meta.fields().iter();
                            meta.fields()
                                .value_set(
                                    &[
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"wasi_ephemeral_nn" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"load" as &Value),
                                        ),
                                    ],
                                )
                        },
                    )
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            _span
                .in_scope(|| {
                    let builder = wiggle::GuestPtr::<
                        [GraphBuilder<'_>],
                    >::new(memory, (arg0 as u32, arg1 as u32));
                    let encoding = GraphEncoding::try_from(arg2)
                        .map_err(|e| {
                            wiggle::GuestError::InFunc {
                                modulename: "wasi_ephemeral_nn",
                                funcname: "load",
                                location: "convert GraphEncoding",
                                err: Box::new(wiggle::GuestError::from(e)),
                            }
                        })?;
                    let target = ExecutionTarget::try_from(arg3)
                        .map_err(|e| {
                            wiggle::GuestError::InFunc {
                                modulename: "wasi_ephemeral_nn",
                                funcname: "load",
                                location: "convert ExecutionTarget",
                                err: Box::new(wiggle::GuestError::from(e)),
                            }
                        })?;
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-nn/src/witx.rs:7",
                                    "wasmtime_wasi_nn::witx::wasi_ephemeral_nn",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-nn/src/witx.rs"),
                                    Some(7u32),
                                    Some("wasmtime_wasi_nn::witx::wasi_ephemeral_nn"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["builder", "encoding", "target"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let enabled = wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && wiggle::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never() && CALLSITE.is_enabled(interest)
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&builder) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&encoding) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&target) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let ret = WasiEphemeralNn::load(ctx, &builder, encoding, target);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-nn/src/witx.rs:7",
                                    "wasmtime_wasi_nn::witx::wasi_ephemeral_nn",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-nn/src/witx.rs"),
                                    Some(7u32),
                                    Some("wasmtime_wasi_nn::witx::wasi_ephemeral_nn"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["result"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let enabled = wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && wiggle::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never() && CALLSITE.is_enabled(interest)
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&ret) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    return Ok(
                        match ret {
                            Ok(e) => {
                                wiggle::GuestPtr::<Graph>::new(memory, arg4 as u32)
                                    .write(e)
                                    .map_err(|e| {
                                        wiggle::GuestError::InFunc {
                                            modulename: "wasi_ephemeral_nn",
                                            funcname: "load",
                                            location: "write graph",
                                            err: Box::new(wiggle::GuestError::from(e)),
                                        }
                                    })?;
                                <NnErrno as wiggle::GuestErrorType>::success() as i32
                            }
                            Err(e) => {
                                UserErrorConversion::nn_errno_from_wasi_nn_error(ctx, e)?
                                    as i32
                            }
                        },
                    );
                })
        }
        #[allow(unreachable_code)]
        pub fn init_execution_context<'a>(
            ctx: &'a mut (impl WasiEphemeralNn + UserErrorConversion),
            memory: &dyn wiggle::GuestMemory,
            arg0: i32,
            arg1: i32,
        ) -> wiggle::anyhow::Result<i32> {
            use std::convert::TryFrom as _;
            let _span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "wiggle abi",
                            "wasmtime_wasi_nn::witx::wasi_ephemeral_nn",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-nn/src/witx.rs"),
                            Some(7u32),
                            Some("wasmtime_wasi_nn::witx::wasi_ephemeral_nn"),
                            ::tracing_core::field::FieldSet::new(
                                &["module", "function"],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    } && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(
                        meta,
                        &{
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = meta.fields().iter();
                            meta.fields()
                                .value_set(
                                    &[
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"wasi_ephemeral_nn" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"init_execution_context" as &Value),
                                        ),
                                    ],
                                )
                        },
                    )
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            _span
                .in_scope(|| {
                    let graph = Graph::from(arg0);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-nn/src/witx.rs:7",
                                    "wasmtime_wasi_nn::witx::wasi_ephemeral_nn",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-nn/src/witx.rs"),
                                    Some(7u32),
                                    Some("wasmtime_wasi_nn::witx::wasi_ephemeral_nn"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["graph"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let enabled = wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && wiggle::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never() && CALLSITE.is_enabled(interest)
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::display(&graph) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let ret = WasiEphemeralNn::init_execution_context(ctx, graph);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-nn/src/witx.rs:7",
                                    "wasmtime_wasi_nn::witx::wasi_ephemeral_nn",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-nn/src/witx.rs"),
                                    Some(7u32),
                                    Some("wasmtime_wasi_nn::witx::wasi_ephemeral_nn"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["result"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let enabled = wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && wiggle::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never() && CALLSITE.is_enabled(interest)
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&ret) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    return Ok(
                        match ret {
                            Ok(e) => {
                                wiggle::GuestPtr::<
                                    GraphExecutionContext,
                                >::new(memory, arg1 as u32)
                                    .write(e)
                                    .map_err(|e| {
                                        wiggle::GuestError::InFunc {
                                            modulename: "wasi_ephemeral_nn",
                                            funcname: "init_execution_context",
                                            location: "write graph_execution_context",
                                            err: Box::new(wiggle::GuestError::from(e)),
                                        }
                                    })?;
                                <NnErrno as wiggle::GuestErrorType>::success() as i32
                            }
                            Err(e) => {
                                UserErrorConversion::nn_errno_from_wasi_nn_error(ctx, e)?
                                    as i32
                            }
                        },
                    );
                })
        }
        #[allow(unreachable_code)]
        pub fn set_input<'a>(
            ctx: &'a mut (impl WasiEphemeralNn + UserErrorConversion),
            memory: &dyn wiggle::GuestMemory,
            arg0: i32,
            arg1: i32,
            arg2: i32,
        ) -> wiggle::anyhow::Result<i32> {
            use std::convert::TryFrom as _;
            let _span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "wiggle abi",
                            "wasmtime_wasi_nn::witx::wasi_ephemeral_nn",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-nn/src/witx.rs"),
                            Some(7u32),
                            Some("wasmtime_wasi_nn::witx::wasi_ephemeral_nn"),
                            ::tracing_core::field::FieldSet::new(
                                &["module", "function"],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    } && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(
                        meta,
                        &{
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = meta.fields().iter();
                            meta.fields()
                                .value_set(
                                    &[
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"wasi_ephemeral_nn" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"set_input" as &Value),
                                        ),
                                    ],
                                )
                        },
                    )
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            _span
                .in_scope(|| {
                    let context = GraphExecutionContext::from(arg0);
                    let index = arg1 as u32;
                    let tensor = wiggle::GuestPtr::<Tensor>::new(memory, arg2 as u32)
                        .read()
                        .map_err(|e| {
                            wiggle::GuestError::InFunc {
                                modulename: "wasi_ephemeral_nn",
                                funcname: "set_input",
                                location: "read tensor",
                                err: Box::new(wiggle::GuestError::from(e)),
                            }
                        })?;
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-nn/src/witx.rs:7",
                                    "wasmtime_wasi_nn::witx::wasi_ephemeral_nn",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-nn/src/witx.rs"),
                                    Some(7u32),
                                    Some("wasmtime_wasi_nn::witx::wasi_ephemeral_nn"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["context", "index", "tensor"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let enabled = wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && wiggle::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never() && CALLSITE.is_enabled(interest)
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::display(&context) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::display(&index) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&tensor) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let ret = WasiEphemeralNn::set_input(ctx, context, index, &tensor);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-nn/src/witx.rs:7",
                                    "wasmtime_wasi_nn::witx::wasi_ephemeral_nn",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-nn/src/witx.rs"),
                                    Some(7u32),
                                    Some("wasmtime_wasi_nn::witx::wasi_ephemeral_nn"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["result"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let enabled = wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && wiggle::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never() && CALLSITE.is_enabled(interest)
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&ret) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    return Ok(
                        match ret {
                            Ok(e) => {
                                <NnErrno as wiggle::GuestErrorType>::success() as i32
                            }
                            Err(e) => {
                                UserErrorConversion::nn_errno_from_wasi_nn_error(ctx, e)?
                                    as i32
                            }
                        },
                    );
                })
        }
        #[allow(unreachable_code)]
        pub fn get_output<'a>(
            ctx: &'a mut (impl WasiEphemeralNn + UserErrorConversion),
            memory: &dyn wiggle::GuestMemory,
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
        ) -> wiggle::anyhow::Result<i32> {
            use std::convert::TryFrom as _;
            let _span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "wiggle abi",
                            "wasmtime_wasi_nn::witx::wasi_ephemeral_nn",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-nn/src/witx.rs"),
                            Some(7u32),
                            Some("wasmtime_wasi_nn::witx::wasi_ephemeral_nn"),
                            ::tracing_core::field::FieldSet::new(
                                &["module", "function"],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    } && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(
                        meta,
                        &{
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = meta.fields().iter();
                            meta.fields()
                                .value_set(
                                    &[
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"wasi_ephemeral_nn" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"get_output" as &Value),
                                        ),
                                    ],
                                )
                        },
                    )
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            _span
                .in_scope(|| {
                    let context = GraphExecutionContext::from(arg0);
                    let index = arg1 as u32;
                    let out_buffer = wiggle::GuestPtr::<u8>::new(memory, arg2 as u32);
                    let out_buffer_max_size = arg3 as u32;
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-nn/src/witx.rs:7",
                                    "wasmtime_wasi_nn::witx::wasi_ephemeral_nn",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-nn/src/witx.rs"),
                                    Some(7u32),
                                    Some("wasmtime_wasi_nn::witx::wasi_ephemeral_nn"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["context", "index", "out_buffer", "out_buffer_max_size"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let enabled = wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && wiggle::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never() && CALLSITE.is_enabled(interest)
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::display(&context) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::display(&index) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&out_buffer) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(
                                                    &wiggle::tracing::field::display(&out_buffer_max_size)
                                                        as &Value,
                                                ),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let ret = WasiEphemeralNn::get_output(
                        ctx,
                        context,
                        index,
                        &out_buffer,
                        out_buffer_max_size,
                    );
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-nn/src/witx.rs:7",
                                    "wasmtime_wasi_nn::witx::wasi_ephemeral_nn",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-nn/src/witx.rs"),
                                    Some(7u32),
                                    Some("wasmtime_wasi_nn::witx::wasi_ephemeral_nn"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["result"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let enabled = wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && wiggle::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never() && CALLSITE.is_enabled(interest)
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&ret) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    return Ok(
                        match ret {
                            Ok(e) => {
                                wiggle::GuestPtr::<BufferSize>::new(memory, arg4 as u32)
                                    .write(e)
                                    .map_err(|e| {
                                        wiggle::GuestError::InFunc {
                                            modulename: "wasi_ephemeral_nn",
                                            funcname: "get_output",
                                            location: "write buffer_size",
                                            err: Box::new(wiggle::GuestError::from(e)),
                                        }
                                    })?;
                                <NnErrno as wiggle::GuestErrorType>::success() as i32
                            }
                            Err(e) => {
                                UserErrorConversion::nn_errno_from_wasi_nn_error(ctx, e)?
                                    as i32
                            }
                        },
                    );
                })
        }
        #[allow(unreachable_code)]
        pub fn compute<'a>(
            ctx: &'a mut (impl WasiEphemeralNn + UserErrorConversion),
            memory: &dyn wiggle::GuestMemory,
            arg0: i32,
        ) -> wiggle::anyhow::Result<i32> {
            use std::convert::TryFrom as _;
            let _span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "wiggle abi",
                            "wasmtime_wasi_nn::witx::wasi_ephemeral_nn",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-nn/src/witx.rs"),
                            Some(7u32),
                            Some("wasmtime_wasi_nn::witx::wasi_ephemeral_nn"),
                            ::tracing_core::field::FieldSet::new(
                                &["module", "function"],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    MacroCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    } && CALLSITE.is_enabled(interest)
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(
                        meta,
                        &{
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = meta.fields().iter();
                            meta.fields()
                                .value_set(
                                    &[
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"wasi_ephemeral_nn" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"compute" as &Value),
                                        ),
                                    ],
                                )
                        },
                    )
                } else {
                    let span = CALLSITE.disabled_span();
                    {};
                    span
                }
            };
            _span
                .in_scope(|| {
                    let context = GraphExecutionContext::from(arg0);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-nn/src/witx.rs:7",
                                    "wasmtime_wasi_nn::witx::wasi_ephemeral_nn",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-nn/src/witx.rs"),
                                    Some(7u32),
                                    Some("wasmtime_wasi_nn::witx::wasi_ephemeral_nn"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["context"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let enabled = wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && wiggle::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never() && CALLSITE.is_enabled(interest)
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::display(&context) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let ret = WasiEphemeralNn::compute(ctx, context);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-nn/src/witx.rs:7",
                                    "wasmtime_wasi_nn::witx::wasi_ephemeral_nn",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-nn/src/witx.rs"),
                                    Some(7u32),
                                    Some("wasmtime_wasi_nn::witx::wasi_ephemeral_nn"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["result"],
                                        ::tracing_core::callsite::Identifier(&CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            MacroCallsite::new(&META)
                        };
                        let enabled = wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && wiggle::tracing::Level::TRACE
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = CALLSITE.interest();
                                !interest.is_never() && CALLSITE.is_enabled(interest)
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = CALLSITE.metadata().fields().iter();
                                CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&ret) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    return Ok(
                        match ret {
                            Ok(e) => {
                                <NnErrno as wiggle::GuestErrorType>::success() as i32
                            }
                            Err(e) => {
                                UserErrorConversion::nn_errno_from_wasi_nn_error(ctx, e)?
                                    as i32
                            }
                        },
                    );
                })
        }
        pub trait WasiEphemeralNn {
            fn load<'a>(
                &mut self,
                builder: &GraphBuilderArray<'a>,
                encoding: GraphEncoding,
                target: ExecutionTarget,
            ) -> Result<Graph, super::WasiNnError>;
            fn init_execution_context(
                &mut self,
                graph: Graph,
            ) -> Result<GraphExecutionContext, super::WasiNnError>;
            fn set_input<'a>(
                &mut self,
                context: GraphExecutionContext,
                index: u32,
                tensor: &Tensor<'a>,
            ) -> Result<(), super::WasiNnError>;
            fn get_output<'a>(
                &mut self,
                context: GraphExecutionContext,
                index: u32,
                out_buffer: &wiggle::GuestPtr<'a, u8>,
                out_buffer_max_size: BufferSize,
            ) -> Result<BufferSize, super::WasiNnError>;
            fn compute(
                &mut self,
                context: GraphExecutionContext,
            ) -> Result<(), super::WasiNnError>;
        }
        /// Adds all instance items to the specified `Linker`.
        pub fn add_to_linker<T, U>(
            linker: &mut wiggle::wasmtime_crate::Linker<T>,
            get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
        ) -> wiggle::anyhow::Result<()>
        where
            U: WasiEphemeralNn + UserErrorConversion,
        {
            linker
                .func_wrap(
                    "wasi_ephemeral_nn",
                    "load",
                    move |
                        mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                        arg0: i32,
                        arg1: i32,
                        arg2: i32,
                        arg3: i32,
                        arg4: i32,
                    | -> wiggle::anyhow::Result<i32> {
                        let export = caller.get_export("memory");
                        let (mem, ctx) = match &export {
                            Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                let ctx = get_cx(ctx);
                                (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                            }
                            Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                let ctx = get_cx(caller.data_mut());
                                (
                                    wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                    ctx,
                                )
                            }
                            _ => {
                                return ::anyhow::__private::Err({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("missing required memory export"),
                                    );
                                    error
                                });
                            }
                        };
                        Ok(<i32>::from(load(ctx, &mem, arg0, arg1, arg2, arg3, arg4)?))
                    },
                )?;
            linker
                .func_wrap(
                    "wasi_ephemeral_nn",
                    "init_execution_context",
                    move |
                        mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                        arg0: i32,
                        arg1: i32,
                    | -> wiggle::anyhow::Result<i32> {
                        let export = caller.get_export("memory");
                        let (mem, ctx) = match &export {
                            Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                let ctx = get_cx(ctx);
                                (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                            }
                            Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                let ctx = get_cx(caller.data_mut());
                                (
                                    wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                    ctx,
                                )
                            }
                            _ => {
                                return ::anyhow::__private::Err({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("missing required memory export"),
                                    );
                                    error
                                });
                            }
                        };
                        Ok(<i32>::from(init_execution_context(ctx, &mem, arg0, arg1)?))
                    },
                )?;
            linker
                .func_wrap(
                    "wasi_ephemeral_nn",
                    "set_input",
                    move |
                        mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                        arg0: i32,
                        arg1: i32,
                        arg2: i32,
                    | -> wiggle::anyhow::Result<i32> {
                        let export = caller.get_export("memory");
                        let (mem, ctx) = match &export {
                            Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                let ctx = get_cx(ctx);
                                (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                            }
                            Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                let ctx = get_cx(caller.data_mut());
                                (
                                    wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                    ctx,
                                )
                            }
                            _ => {
                                return ::anyhow::__private::Err({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("missing required memory export"),
                                    );
                                    error
                                });
                            }
                        };
                        Ok(<i32>::from(set_input(ctx, &mem, arg0, arg1, arg2)?))
                    },
                )?;
            linker
                .func_wrap(
                    "wasi_ephemeral_nn",
                    "get_output",
                    move |
                        mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                        arg0: i32,
                        arg1: i32,
                        arg2: i32,
                        arg3: i32,
                        arg4: i32,
                    | -> wiggle::anyhow::Result<i32> {
                        let export = caller.get_export("memory");
                        let (mem, ctx) = match &export {
                            Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                let ctx = get_cx(ctx);
                                (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                            }
                            Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                let ctx = get_cx(caller.data_mut());
                                (
                                    wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                    ctx,
                                )
                            }
                            _ => {
                                return ::anyhow::__private::Err({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("missing required memory export"),
                                    );
                                    error
                                });
                            }
                        };
                        Ok(
                            <i32>::from(
                                get_output(ctx, &mem, arg0, arg1, arg2, arg3, arg4)?,
                            ),
                        )
                    },
                )?;
            linker
                .func_wrap(
                    "wasi_ephemeral_nn",
                    "compute",
                    move |
                        mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                        arg0: i32,
                    | -> wiggle::anyhow::Result<i32> {
                        let export = caller.get_export("memory");
                        let (mem, ctx) = match &export {
                            Some(wiggle::wasmtime_crate::Extern::Memory(m)) => {
                                let (mem, ctx) = m.data_and_store_mut(&mut caller);
                                let ctx = get_cx(ctx);
                                (wiggle::wasmtime::WasmtimeGuestMemory::new(mem), ctx)
                            }
                            Some(wiggle::wasmtime_crate::Extern::SharedMemory(m)) => {
                                let ctx = get_cx(caller.data_mut());
                                (
                                    wiggle::wasmtime::WasmtimeGuestMemory::shared(m.data()),
                                    ctx,
                                )
                            }
                            _ => {
                                return ::anyhow::__private::Err({
                                    let error = ::anyhow::__private::format_err(
                                        format_args!("missing required memory export"),
                                    );
                                    error
                                });
                            }
                        };
                        Ok(<i32>::from(compute(ctx, &mem, arg0)?))
                    },
                )?;
            Ok(())
        }
    }
    use types::NnErrno;
    impl<'a> types::UserErrorConversion for WasiNnCtx {
        fn nn_errno_from_wasi_nn_error(&mut self, e: WasiNnError) -> Result<NnErrno> {
            {
                ::std::io::_eprint(format_args!("Host error: {0:?}\n", e));
            };
            match e {
                WasiNnError::BackendError(_) => {
                    ::core::panicking::panic("not implemented")
                }
                WasiNnError::GuestError(_) => ::core::panicking::panic("not implemented"),
                WasiNnError::UsageError(_) => ::core::panicking::panic("not implemented"),
            }
        }
    }
    /// Additionally, we must let Wiggle know which of our error codes represents a successful operation.
    impl wiggle::GuestErrorType for NnErrno {
        fn success() -> Self {
            Self::Success
        }
    }
}
pub use ctx::WasiNnCtx;
pub use witx::wasi_ephemeral_nn::add_to_linker;
