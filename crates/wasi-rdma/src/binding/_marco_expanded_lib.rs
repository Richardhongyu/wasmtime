mod witx {
    pub mod types {
        use std::convert::TryFrom;
        pub enum RdmaError {
            Success,
            RuntimeError,
            IoError,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RdmaError {
            #[inline]
            fn clone(&self) -> RdmaError {
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RdmaError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(
                    f,
                    match self {
                        RdmaError::Success => "Success",
                        RdmaError::RuntimeError => "RuntimeError",
                        RdmaError::IoError => "IoError",
                    },
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for RdmaError {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for RdmaError {
            #[inline]
            fn eq(&self, other: &RdmaError) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for RdmaError {}
        impl TryFrom<u16> for RdmaError {
            type Error = wiggle::GuestError;
            #[inline]
            fn try_from(value: u16) -> Result<RdmaError, wiggle::GuestError> {
                match value {
                    0 => Ok(RdmaError::Success),
                    1 => Ok(RdmaError::RuntimeError),
                    2 => Ok(RdmaError::IoError),
                    _ => Err(wiggle::GuestError::InvalidEnumValue("RdmaError")),
                }
            }
        }
        impl TryFrom<i32> for RdmaError {
            type Error = wiggle::GuestError;
            #[inline]
            fn try_from(value: i32) -> Result<RdmaError, wiggle::GuestError> {
                RdmaError::try_from(u16::try_from(value)?)
            }
        }
        impl From<RdmaError> for u16 {
            #[inline]
            fn from(v: RdmaError) -> u16 {
                match v {
                    RdmaError::Success => 0,
                    RdmaError::RuntimeError => 1,
                    RdmaError::IoError => 2,
                }
            }
        }
        impl<'a> wiggle::GuestType<'a> for RdmaError {
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
                    0 => Ok(RdmaError::Success),
                    1 => Ok(RdmaError::RuntimeError),
                    2 => Ok(RdmaError::IoError),
                    _ => Err(wiggle::GuestError::InvalidEnumValue("RdmaError")),
                }
            }
            fn write(
                location: &wiggle::GuestPtr<'_, Self>,
                val: Self,
            ) -> Result<(), wiggle::GuestError> {
                match val {
                    RdmaError::Success => {
                        location.cast().write(0usize as u16)?;
                    }
                    RdmaError::RuntimeError => {
                        location.cast().write(1usize as u16)?;
                    }
                    RdmaError::IoError => {
                        location.cast().write(2usize as u16)?;
                    }
                }
                Ok(())
            }
        }
        #[repr(transparent)]
        pub struct IbvMr(u32);
        #[automatically_derived]
        impl ::core::marker::Copy for IbvMr {}
        #[automatically_derived]
        impl ::core::clone::Clone for IbvMr {
            #[inline]
            fn clone(&self) -> IbvMr {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IbvMr {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "IbvMr", &&self.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for IbvMr {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for IbvMr {}
        #[automatically_derived]
        impl ::core::cmp::Eq for IbvMr {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for IbvMr {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for IbvMr {
            #[inline]
            fn eq(&self, other: &IbvMr) -> bool {
                self.0 == other.0
            }
        }
        impl IbvMr {
            #[inline]
            pub unsafe fn inner(&self) -> u32 {
                self.0
            }
        }
        impl From<IbvMr> for u32 {
            #[inline]
            fn from(e: IbvMr) -> u32 {
                e.0
            }
        }
        impl From<IbvMr> for i32 {
            #[inline]
            fn from(e: IbvMr) -> i32 {
                e.0 as i32
            }
        }
        impl From<u32> for IbvMr {
            #[inline]
            fn from(e: u32) -> IbvMr {
                IbvMr(e)
            }
        }
        impl From<i32> for IbvMr {
            #[inline]
            fn from(e: i32) -> IbvMr {
                IbvMr(e as u32)
            }
        }
        impl ::std::fmt::Display for IbvMr {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0}({1})", "IbvMr", self.0))
            }
        }
        impl<'a> wiggle::GuestType<'a> for IbvMr {
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
                location: &wiggle::GuestPtr<'a, IbvMr>,
            ) -> Result<IbvMr, wiggle::GuestError> {
                Ok(IbvMr(u32::read(&location.cast())?))
            }
            #[inline]
            fn write(
                location: &wiggle::GuestPtr<'_, Self>,
                val: Self,
            ) -> Result<(), wiggle::GuestError> {
                u32::write(&location.cast(), val.0)
            }
        }
        #[repr(transparent)]
        pub struct IbvWc(u32);
        #[automatically_derived]
        impl ::core::marker::Copy for IbvWc {}
        #[automatically_derived]
        impl ::core::clone::Clone for IbvWc {
            #[inline]
            fn clone(&self) -> IbvWc {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IbvWc {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "IbvWc", &&self.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for IbvWc {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for IbvWc {}
        #[automatically_derived]
        impl ::core::cmp::Eq for IbvWc {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for IbvWc {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for IbvWc {
            #[inline]
            fn eq(&self, other: &IbvWc) -> bool {
                self.0 == other.0
            }
        }
        impl IbvWc {
            #[inline]
            pub unsafe fn inner(&self) -> u32 {
                self.0
            }
        }
        impl From<IbvWc> for u32 {
            #[inline]
            fn from(e: IbvWc) -> u32 {
                e.0
            }
        }
        impl From<IbvWc> for i32 {
            #[inline]
            fn from(e: IbvWc) -> i32 {
                e.0 as i32
            }
        }
        impl From<u32> for IbvWc {
            #[inline]
            fn from(e: u32) -> IbvWc {
                IbvWc(e)
            }
        }
        impl From<i32> for IbvWc {
            #[inline]
            fn from(e: i32) -> IbvWc {
                IbvWc(e as u32)
            }
        }
        impl ::std::fmt::Display for IbvWc {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0}({1})", "IbvWc", self.0))
            }
        }
        impl<'a> wiggle::GuestType<'a> for IbvWc {
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
                location: &wiggle::GuestPtr<'a, IbvWc>,
            ) -> Result<IbvWc, wiggle::GuestError> {
                Ok(IbvWc(u32::read(&location.cast())?))
            }
            #[inline]
            fn write(
                location: &wiggle::GuestPtr<'_, Self>,
                val: Self,
            ) -> Result<(), wiggle::GuestError> {
                u32::write(&location.cast(), val.0)
            }
        }
        pub struct RdmaAddrinfoStruct {
            pub flags: i32,
            pub port_space: i32,
            pub family: i32,
            pub qp_type: i32,
            pub src_len: u32,
            pub dst_len: u32,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RdmaAddrinfoStruct {
            #[inline]
            fn clone(&self) -> RdmaAddrinfoStruct {
                RdmaAddrinfoStruct {
                    flags: ::core::clone::Clone::clone(&self.flags),
                    port_space: ::core::clone::Clone::clone(&self.port_space),
                    family: ::core::clone::Clone::clone(&self.family),
                    qp_type: ::core::clone::Clone::clone(&self.qp_type),
                    src_len: ::core::clone::Clone::clone(&self.src_len),
                    dst_len: ::core::clone::Clone::clone(&self.dst_len),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RdmaAddrinfoStruct {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "flags",
                    "port_space",
                    "family",
                    "qp_type",
                    "src_len",
                    "dst_len",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.flags,
                    &self.port_space,
                    &self.family,
                    &self.qp_type,
                    &self.src_len,
                    &&self.dst_len,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "RdmaAddrinfoStruct",
                    names,
                    values,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for RdmaAddrinfoStruct {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for RdmaAddrinfoStruct {
            #[inline]
            fn eq(&self, other: &RdmaAddrinfoStruct) -> bool {
                self.flags == other.flags && self.port_space == other.port_space
                    && self.family == other.family && self.qp_type == other.qp_type
                    && self.src_len == other.src_len && self.dst_len == other.dst_len
            }
        }
        impl<'a> wiggle::GuestType<'a> for RdmaAddrinfoStruct {
            #[inline]
            fn guest_size() -> u32 {
                24u32
            }
            #[inline]
            fn guest_align() -> usize {
                4usize
            }
            fn read(
                location: &wiggle::GuestPtr<'a, Self>,
            ) -> Result<Self, wiggle::GuestError> {
                let flags = <i32 as wiggle::GuestType>::read(
                    &location.cast::<u8>().add(0u32)?.cast(),
                )?;
                let port_space = <i32 as wiggle::GuestType>::read(
                    &location.cast::<u8>().add(4u32)?.cast(),
                )?;
                let family = <i32 as wiggle::GuestType>::read(
                    &location.cast::<u8>().add(8u32)?.cast(),
                )?;
                let qp_type = <i32 as wiggle::GuestType>::read(
                    &location.cast::<u8>().add(12u32)?.cast(),
                )?;
                let src_len = <u32 as wiggle::GuestType>::read(
                    &location.cast::<u8>().add(16u32)?.cast(),
                )?;
                let dst_len = <u32 as wiggle::GuestType>::read(
                    &location.cast::<u8>().add(20u32)?.cast(),
                )?;
                Ok(RdmaAddrinfoStruct {
                    flags,
                    port_space,
                    family,
                    qp_type,
                    src_len,
                    dst_len,
                })
            }
            fn write(
                location: &wiggle::GuestPtr<'_, Self>,
                val: Self,
            ) -> Result<(), wiggle::GuestError> {
                wiggle::GuestType::write(
                    &location.cast::<u8>().add(0u32)?.cast(),
                    val.flags,
                )?;
                wiggle::GuestType::write(
                    &location.cast::<u8>().add(4u32)?.cast(),
                    val.port_space,
                )?;
                wiggle::GuestType::write(
                    &location.cast::<u8>().add(8u32)?.cast(),
                    val.family,
                )?;
                wiggle::GuestType::write(
                    &location.cast::<u8>().add(12u32)?.cast(),
                    val.qp_type,
                )?;
                wiggle::GuestType::write(
                    &location.cast::<u8>().add(16u32)?.cast(),
                    val.src_len,
                )?;
                wiggle::GuestType::write(
                    &location.cast::<u8>().add(20u32)?.cast(),
                    val.dst_len,
                )?;
                Ok(())
            }
        }
        pub struct IbvQpCap {
            pub max_send_wr: u32,
            pub max_recv_wr: u32,
            pub max_send_sge: u32,
            pub max_recv_sge: u32,
            pub max_inline_data: u32,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for IbvQpCap {
            #[inline]
            fn clone(&self) -> IbvQpCap {
                IbvQpCap {
                    max_send_wr: ::core::clone::Clone::clone(&self.max_send_wr),
                    max_recv_wr: ::core::clone::Clone::clone(&self.max_recv_wr),
                    max_send_sge: ::core::clone::Clone::clone(&self.max_send_sge),
                    max_recv_sge: ::core::clone::Clone::clone(&self.max_recv_sge),
                    max_inline_data: ::core::clone::Clone::clone(&self.max_inline_data),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IbvQpCap {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "IbvQpCap",
                    "max_send_wr",
                    &self.max_send_wr,
                    "max_recv_wr",
                    &self.max_recv_wr,
                    "max_send_sge",
                    &self.max_send_sge,
                    "max_recv_sge",
                    &self.max_recv_sge,
                    "max_inline_data",
                    &&self.max_inline_data,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for IbvQpCap {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for IbvQpCap {
            #[inline]
            fn eq(&self, other: &IbvQpCap) -> bool {
                self.max_send_wr == other.max_send_wr
                    && self.max_recv_wr == other.max_recv_wr
                    && self.max_send_sge == other.max_send_sge
                    && self.max_recv_sge == other.max_recv_sge
                    && self.max_inline_data == other.max_inline_data
            }
        }
        impl<'a> wiggle::GuestType<'a> for IbvQpCap {
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
                let max_send_wr = <u32 as wiggle::GuestType>::read(
                    &location.cast::<u8>().add(0u32)?.cast(),
                )?;
                let max_recv_wr = <u32 as wiggle::GuestType>::read(
                    &location.cast::<u8>().add(4u32)?.cast(),
                )?;
                let max_send_sge = <u32 as wiggle::GuestType>::read(
                    &location.cast::<u8>().add(8u32)?.cast(),
                )?;
                let max_recv_sge = <u32 as wiggle::GuestType>::read(
                    &location.cast::<u8>().add(12u32)?.cast(),
                )?;
                let max_inline_data = <u32 as wiggle::GuestType>::read(
                    &location.cast::<u8>().add(16u32)?.cast(),
                )?;
                Ok(IbvQpCap {
                    max_send_wr,
                    max_recv_wr,
                    max_send_sge,
                    max_recv_sge,
                    max_inline_data,
                })
            }
            fn write(
                location: &wiggle::GuestPtr<'_, Self>,
                val: Self,
            ) -> Result<(), wiggle::GuestError> {
                wiggle::GuestType::write(
                    &location.cast::<u8>().add(0u32)?.cast(),
                    val.max_send_wr,
                )?;
                wiggle::GuestType::write(
                    &location.cast::<u8>().add(4u32)?.cast(),
                    val.max_recv_wr,
                )?;
                wiggle::GuestType::write(
                    &location.cast::<u8>().add(8u32)?.cast(),
                    val.max_send_sge,
                )?;
                wiggle::GuestType::write(
                    &location.cast::<u8>().add(12u32)?.cast(),
                    val.max_recv_sge,
                )?;
                wiggle::GuestType::write(
                    &location.cast::<u8>().add(16u32)?.cast(),
                    val.max_inline_data,
                )?;
                Ok(())
            }
        }
        #[repr(transparent)]
        pub struct Rdma(u32);
        #[automatically_derived]
        impl ::core::marker::Copy for Rdma {}
        #[automatically_derived]
        impl ::core::clone::Clone for Rdma {
            #[inline]
            fn clone(&self) -> Rdma {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Rdma {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Rdma", &&self.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for Rdma {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for Rdma {}
        #[automatically_derived]
        impl ::core::cmp::Eq for Rdma {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Rdma {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Rdma {
            #[inline]
            fn eq(&self, other: &Rdma) -> bool {
                self.0 == other.0
            }
        }
        impl Rdma {
            #[inline]
            pub unsafe fn inner(&self) -> u32 {
                self.0
            }
        }
        impl From<Rdma> for u32 {
            #[inline]
            fn from(e: Rdma) -> u32 {
                e.0
            }
        }
        impl From<Rdma> for i32 {
            #[inline]
            fn from(e: Rdma) -> i32 {
                e.0 as i32
            }
        }
        impl From<u32> for Rdma {
            #[inline]
            fn from(e: u32) -> Rdma {
                Rdma(e)
            }
        }
        impl From<i32> for Rdma {
            #[inline]
            fn from(e: i32) -> Rdma {
                Rdma(e as u32)
            }
        }
        impl ::std::fmt::Display for Rdma {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0}({1})", "Rdma", self.0))
            }
        }
        impl<'a> wiggle::GuestType<'a> for Rdma {
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
                location: &wiggle::GuestPtr<'a, Rdma>,
            ) -> Result<Rdma, wiggle::GuestError> {
                Ok(Rdma(u32::read(&location.cast())?))
            }
            #[inline]
            fn write(
                location: &wiggle::GuestPtr<'_, Self>,
                val: Self,
            ) -> Result<(), wiggle::GuestError> {
                u32::write(&location.cast(), val.0)
            }
        }
        #[repr(transparent)]
        pub struct RdmaCq(u32);
        #[automatically_derived]
        impl ::core::marker::Copy for RdmaCq {}
        #[automatically_derived]
        impl ::core::clone::Clone for RdmaCq {
            #[inline]
            fn clone(&self) -> RdmaCq {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RdmaCq {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "RdmaCq", &&self.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for RdmaCq {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for RdmaCq {}
        #[automatically_derived]
        impl ::core::cmp::Eq for RdmaCq {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for RdmaCq {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for RdmaCq {
            #[inline]
            fn eq(&self, other: &RdmaCq) -> bool {
                self.0 == other.0
            }
        }
        impl RdmaCq {
            #[inline]
            pub unsafe fn inner(&self) -> u32 {
                self.0
            }
        }
        impl From<RdmaCq> for u32 {
            #[inline]
            fn from(e: RdmaCq) -> u32 {
                e.0
            }
        }
        impl From<RdmaCq> for i32 {
            #[inline]
            fn from(e: RdmaCq) -> i32 {
                e.0 as i32
            }
        }
        impl From<u32> for RdmaCq {
            #[inline]
            fn from(e: u32) -> RdmaCq {
                RdmaCq(e)
            }
        }
        impl From<i32> for RdmaCq {
            #[inline]
            fn from(e: i32) -> RdmaCq {
                RdmaCq(e as u32)
            }
        }
        impl ::std::fmt::Display for RdmaCq {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0}({1})", "RdmaCq", self.0))
            }
        }
        impl<'a> wiggle::GuestType<'a> for RdmaCq {
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
                location: &wiggle::GuestPtr<'a, RdmaCq>,
            ) -> Result<RdmaCq, wiggle::GuestError> {
                Ok(RdmaCq(u32::read(&location.cast())?))
            }
            #[inline]
            fn write(
                location: &wiggle::GuestPtr<'_, Self>,
                val: Self,
            ) -> Result<(), wiggle::GuestError> {
                u32::write(&location.cast(), val.0)
            }
        }
        #[repr(transparent)]
        pub struct EpPd(u32);
        #[automatically_derived]
        impl ::core::marker::Copy for EpPd {}
        #[automatically_derived]
        impl ::core::clone::Clone for EpPd {
            #[inline]
            fn clone(&self) -> EpPd {
                let _: ::core::clone::AssertParamIsClone<u32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for EpPd {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "EpPd", &&self.0)
            }
        }
        #[automatically_derived]
        impl ::core::hash::Hash for EpPd {
            fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                ::core::hash::Hash::hash(&self.0, state)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralEq for EpPd {}
        #[automatically_derived]
        impl ::core::cmp::Eq for EpPd {
            #[inline]
            #[doc(hidden)]
            #[no_coverage]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u32>;
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for EpPd {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for EpPd {
            #[inline]
            fn eq(&self, other: &EpPd) -> bool {
                self.0 == other.0
            }
        }
        impl EpPd {
            #[inline]
            pub unsafe fn inner(&self) -> u32 {
                self.0
            }
        }
        impl From<EpPd> for u32 {
            #[inline]
            fn from(e: EpPd) -> u32 {
                e.0
            }
        }
        impl From<EpPd> for i32 {
            #[inline]
            fn from(e: EpPd) -> i32 {
                e.0 as i32
            }
        }
        impl From<u32> for EpPd {
            #[inline]
            fn from(e: u32) -> EpPd {
                EpPd(e)
            }
        }
        impl From<i32> for EpPd {
            #[inline]
            fn from(e: i32) -> EpPd {
                EpPd(e as u32)
            }
        }
        impl ::std::fmt::Display for EpPd {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{0}({1})", "EpPd", self.0))
            }
        }
        impl<'a> wiggle::GuestType<'a> for EpPd {
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
                location: &wiggle::GuestPtr<'a, EpPd>,
            ) -> Result<EpPd, wiggle::GuestError> {
                Ok(EpPd(u32::read(&location.cast())?))
            }
            #[inline]
            fn write(
                location: &wiggle::GuestPtr<'_, Self>,
                val: Self,
            ) -> Result<(), wiggle::GuestError> {
                u32::write(&location.cast(), val.0)
            }
        }
        pub trait UserErrorConversion {}
    }
    pub mod wasi_ephemeral_rdma {
        use super::types::*;
        pub use super::types::UserErrorConversion;
        #[allow(unreachable_code)]
        pub fn rdma_init<'a>(
            ctx: &'a mut (impl WasiEphemeralRdma),
            memory: &dyn wiggle::GuestMemory,
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
            arg4: i32,
            arg5: i32,
            arg6: i32,
            arg7: i32,
        ) -> wiggle::anyhow::Result<i32> {
            use std::convert::TryFrom as _;
            let _span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "wiggle abi",
                            "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-rdma/src/witx.rs"),
                            Some(1u32),
                            Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                            Some(&"wasi_ephemeral_rdma" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"rdma_init" as &Value),
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
                    let node = wiggle::GuestPtr::<
                        str,
                    >::new(memory, (arg0 as u32, arg1 as u32));
                    let service = wiggle::GuestPtr::<
                        str,
                    >::new(memory, (arg2 as u32, arg3 as u32));
                    let hints = wiggle::GuestPtr::<
                        RdmaAddrinfoStruct,
                    >::new(memory, arg4 as u32)
                        .read()
                        .map_err(|e| {
                            wiggle::GuestError::InFunc {
                                modulename: "wasi_ephemeral_rdma",
                                funcname: "rdma_init",
                                location: "read rdma_addrinfo_struct",
                                err: Box::new(wiggle::GuestError::from(e)),
                            }
                        })?;
                    let cap = wiggle::GuestPtr::<IbvQpCap>::new(memory, arg5 as u32)
                        .read()
                        .map_err(|e| {
                            wiggle::GuestError::InFunc {
                                modulename: "wasi_ephemeral_rdma",
                                funcname: "rdma_init",
                                location: "read ibv_qp_cap",
                                err: Box::new(wiggle::GuestError::from(e)),
                            }
                        })?;
                    let is_server = u8::try_from(arg6)
                        .map_err(|e| {
                            wiggle::GuestError::InFunc {
                                modulename: "wasi_ephemeral_rdma",
                                funcname: "rdma_init",
                                location: "convert u8",
                                err: Box::new(wiggle::GuestError::from(e)),
                            }
                        })?;
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["node", "service", "hints", "cap", "is_server"],
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
                                                Some(&wiggle::tracing::field::debug(&node) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&service) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&hints) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&cap) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::display(&is_server) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let ret = WasiEphemeralRdma::rdma_init(
                        ctx,
                        &node,
                        &service,
                        &hints,
                        &cap,
                        is_server,
                    );
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                wiggle::GuestPtr::<Rdma>::new(memory, arg7 as u32)
                                    .write(e)
                                    .map_err(|e| {
                                        wiggle::GuestError::InFunc {
                                            modulename: "wasi_ephemeral_rdma",
                                            funcname: "rdma_init",
                                            location: "write rdma",
                                            err: Box::new(wiggle::GuestError::from(e)),
                                        }
                                    })?;
                                <RdmaError as wiggle::GuestErrorType>::success() as i32
                            }
                            Err(e) => e as i32,
                        },
                    );
                })
        }
        #[allow(unreachable_code)]
        pub fn rdma_connect<'a>(
            ctx: &'a mut (impl WasiEphemeralRdma),
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
                            "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-rdma/src/witx.rs"),
                            Some(1u32),
                            Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                            Some(&"wasi_ephemeral_rdma" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"rdma_connect" as &Value),
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
                    let rdma = Rdma::from(arg0);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["rdma"],
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
                                                Some(&wiggle::tracing::field::display(&rdma) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let ret = WasiEphemeralRdma::rdma_connect(ctx, rdma);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                <RdmaError as wiggle::GuestErrorType>::success() as i32
                            }
                            Err(e) => e as i32,
                        },
                    );
                })
        }
        #[allow(unreachable_code)]
        pub fn rdma_disconnect<'a>(
            ctx: &'a mut (impl WasiEphemeralRdma),
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
                            "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-rdma/src/witx.rs"),
                            Some(1u32),
                            Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                            Some(&"wasi_ephemeral_rdma" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"rdma_disconnect" as &Value),
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
                    let rdma = Rdma::from(arg0);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["rdma"],
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
                                                Some(&wiggle::tracing::field::display(&rdma) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let ret = WasiEphemeralRdma::rdma_disconnect(ctx, rdma);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                <RdmaError as wiggle::GuestErrorType>::success() as i32
                            }
                            Err(e) => e as i32,
                        },
                    );
                })
        }
        #[allow(unreachable_code)]
        pub fn rdma_get_send_comp<'a>(
            ctx: &'a mut (impl WasiEphemeralRdma),
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
                            "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-rdma/src/witx.rs"),
                            Some(1u32),
                            Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                            Some(&"wasi_ephemeral_rdma" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"rdma_get_send_comp" as &Value),
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
                    let rdma = Rdma::from(arg0);
                    let wc = IbvWc::from(arg1);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["rdma", "wc"],
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
                                                Some(&wiggle::tracing::field::display(&rdma) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::display(&wc) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let ret = WasiEphemeralRdma::rdma_get_send_comp(ctx, rdma, wc);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                wiggle::GuestPtr::<IbvWc>::new(memory, arg2 as u32)
                                    .write(e)
                                    .map_err(|e| {
                                        wiggle::GuestError::InFunc {
                                            modulename: "wasi_ephemeral_rdma",
                                            funcname: "rdma_get_send_comp",
                                            location: "write ibv_wc",
                                            err: Box::new(wiggle::GuestError::from(e)),
                                        }
                                    })?;
                                <RdmaError as wiggle::GuestErrorType>::success() as i32
                            }
                            Err(e) => e as i32,
                        },
                    );
                })
        }
        #[allow(unreachable_code)]
        pub fn rdma_get_recv_comp<'a>(
            ctx: &'a mut (impl WasiEphemeralRdma),
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
                            "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-rdma/src/witx.rs"),
                            Some(1u32),
                            Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                            Some(&"wasi_ephemeral_rdma" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"rdma_get_recv_comp" as &Value),
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
                    let rdma = Rdma::from(arg0);
                    let wc = IbvWc::from(arg1);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["rdma", "wc"],
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
                                                Some(&wiggle::tracing::field::display(&rdma) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::display(&wc) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let ret = WasiEphemeralRdma::rdma_get_recv_comp(ctx, rdma, wc);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                wiggle::GuestPtr::<IbvWc>::new(memory, arg2 as u32)
                                    .write(e)
                                    .map_err(|e| {
                                        wiggle::GuestError::InFunc {
                                            modulename: "wasi_ephemeral_rdma",
                                            funcname: "rdma_get_recv_comp",
                                            location: "write ibv_wc",
                                            err: Box::new(wiggle::GuestError::from(e)),
                                        }
                                    })?;
                                <RdmaError as wiggle::GuestErrorType>::success() as i32
                            }
                            Err(e) => e as i32,
                        },
                    );
                })
        }
        #[allow(unreachable_code)]
        pub fn rdma_reg_msgs<'a>(
            ctx: &'a mut (impl WasiEphemeralRdma),
            memory: &dyn wiggle::GuestMemory,
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
        ) -> wiggle::anyhow::Result<i32> {
            use std::convert::TryFrom as _;
            let _span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "wiggle abi",
                            "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-rdma/src/witx.rs"),
                            Some(1u32),
                            Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                            Some(&"wasi_ephemeral_rdma" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"rdma_reg_msgs" as &Value),
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
                    let rdma = Rdma::from(arg0);
                    let addr = wiggle::GuestPtr::<u8>::new(memory, arg1 as u32);
                    let size = arg2 as u32;
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["rdma", "addr", "size"],
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
                                                Some(&wiggle::tracing::field::display(&rdma) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&addr) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::display(&size) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let ret = WasiEphemeralRdma::rdma_reg_msgs(ctx, rdma, &addr, size);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                wiggle::GuestPtr::<IbvMr>::new(memory, arg3 as u32)
                                    .write(e)
                                    .map_err(|e| {
                                        wiggle::GuestError::InFunc {
                                            modulename: "wasi_ephemeral_rdma",
                                            funcname: "rdma_reg_msgs",
                                            location: "write ibv_mr",
                                            err: Box::new(wiggle::GuestError::from(e)),
                                        }
                                    })?;
                                <RdmaError as wiggle::GuestErrorType>::success() as i32
                            }
                            Err(e) => e as i32,
                        },
                    );
                })
        }
        #[allow(unreachable_code)]
        pub fn rdma_dereg_mr<'a>(
            ctx: &'a mut (impl WasiEphemeralRdma),
            memory: &dyn wiggle::GuestMemory,
            arg0: i32,
        ) -> wiggle::anyhow::Result<()> {
            use std::convert::TryFrom as _;
            let _span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "wiggle abi",
                            "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-rdma/src/witx.rs"),
                            Some(1u32),
                            Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                            Some(&"wasi_ephemeral_rdma" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"rdma_dereg_mr" as &Value),
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
                    let ibv_mr = IbvMr::from(arg0);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["ibv_mr"],
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
                                                Some(&wiggle::tracing::field::display(&ibv_mr) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let ret = WasiEphemeralRdma::rdma_dereg_mr(ctx, ibv_mr);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                    return Ok(());
                })
        }
        #[allow(unreachable_code)]
        pub fn rdma_post_send<'a>(
            ctx: &'a mut (impl WasiEphemeralRdma),
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
                            "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-rdma/src/witx.rs"),
                            Some(1u32),
                            Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                            Some(&"wasi_ephemeral_rdma" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"rdma_post_send" as &Value),
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
                    let rdma = Rdma::from(arg0);
                    let addr = wiggle::GuestPtr::<u8>::new(memory, arg1 as u32);
                    let size = arg2 as u32;
                    let send_mr = IbvMr::from(arg3);
                    let flags = arg4 as u32;
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["rdma", "addr", "size", "send_mr", "flags"],
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
                                                Some(&wiggle::tracing::field::display(&rdma) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&addr) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::display(&size) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::display(&send_mr) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::display(&flags) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let ret = WasiEphemeralRdma::rdma_post_send(
                        ctx,
                        rdma,
                        &addr,
                        size,
                        send_mr,
                        flags,
                    );
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                <RdmaError as wiggle::GuestErrorType>::success() as i32
                            }
                            Err(e) => e as i32,
                        },
                    );
                })
        }
        #[allow(unreachable_code)]
        pub fn rdma_post_recv<'a>(
            ctx: &'a mut (impl WasiEphemeralRdma),
            memory: &dyn wiggle::GuestMemory,
            arg0: i32,
            arg1: i32,
            arg2: i32,
            arg3: i32,
        ) -> wiggle::anyhow::Result<i32> {
            use std::convert::TryFrom as _;
            let _span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "wiggle abi",
                            "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-rdma/src/witx.rs"),
                            Some(1u32),
                            Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                            Some(&"wasi_ephemeral_rdma" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"rdma_post_recv" as &Value),
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
                    let rdma = Rdma::from(arg0);
                    let addr = wiggle::GuestPtr::<u8>::new(memory, arg1 as u32);
                    let size = arg2 as u32;
                    let recv_mr = IbvMr::from(arg3);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["rdma", "addr", "size", "recv_mr"],
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
                                                Some(&wiggle::tracing::field::display(&rdma) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::debug(&addr) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::display(&size) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(&wiggle::tracing::field::display(&recv_mr) as &Value),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let ret = WasiEphemeralRdma::rdma_post_recv(
                        ctx,
                        rdma,
                        &addr,
                        size,
                        recv_mr,
                    );
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                <RdmaError as wiggle::GuestErrorType>::success() as i32
                            }
                            Err(e) => e as i32,
                        },
                    );
                })
        }
        #[allow(unreachable_code)]
        pub fn ibv_query_qp<'a>(
            ctx: &'a mut (impl WasiEphemeralRdma),
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
                            "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-rdma/src/witx.rs"),
                            Some(1u32),
                            Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                            Some(&"wasi_ephemeral_rdma" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"ibv_query_qp" as &Value),
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
                    let rdma = Rdma::from(arg0);
                    let ibv_qp_attrmask = arg1 as u32;
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
                                    ::tracing_core::field::FieldSet::new(
                                        &["rdma", "ibv_qp_attrmask"],
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
                                                Some(&wiggle::tracing::field::display(&rdma) as &Value),
                                            ),
                                            (
                                                &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                                Some(
                                                    &wiggle::tracing::field::display(&ibv_qp_attrmask) as &Value,
                                                ),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                        }
                    };
                    let ret = WasiEphemeralRdma::ibv_query_qp(
                        ctx,
                        rdma,
                        ibv_qp_attrmask,
                    );
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                <RdmaError as wiggle::GuestErrorType>::success() as i32
                            }
                            Err(e) => e as i32,
                        },
                    );
                })
        }
        #[allow(unreachable_code)]
        pub fn print_hello_world<'a>(
            ctx: &'a mut (impl WasiEphemeralRdma),
            memory: &dyn wiggle::GuestMemory,
        ) -> wiggle::anyhow::Result<()> {
            use std::convert::TryFrom as _;
            let _span = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                    use ::tracing::__macro_support::MacroCallsite;
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "wiggle abi",
                            "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                            wiggle::tracing::Level::TRACE,
                            Some("crates/wasi-rdma/src/witx.rs"),
                            Some(1u32),
                            Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                                            Some(&"wasi_ephemeral_rdma" as &Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&"print_hello_world" as &Value),
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
                    let ret = WasiEphemeralRdma::print_hello_world(ctx);
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static CALLSITE: ::tracing::__macro_support::MacroCallsite = {
                            use ::tracing::__macro_support::MacroCallsite;
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/wasi-rdma/src/witx.rs:1",
                                    "wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma",
                                    wiggle::tracing::Level::TRACE,
                                    Some("crates/wasi-rdma/src/witx.rs"),
                                    Some(1u32),
                                    Some("wasmtime_wasi_rdma::witx::wasi_ephemeral_rdma"),
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
                    return Ok(());
                })
        }
        pub trait WasiEphemeralRdma {
            fn rdma_init<'a>(
                &mut self,
                node: &wiggle::GuestPtr<'a, str>,
                service: &wiggle::GuestPtr<'a, str>,
                hints: &RdmaAddrinfoStruct,
                cap: &IbvQpCap,
                is_server: u8,
            ) -> Result<Rdma, RdmaError>;
            fn rdma_connect(&mut self, rdma: Rdma) -> Result<(), RdmaError>;
            fn rdma_disconnect(&mut self, rdma: Rdma) -> Result<(), RdmaError>;
            fn rdma_get_send_comp(
                &mut self,
                rdma: Rdma,
                wc: IbvWc,
            ) -> Result<IbvWc, RdmaError>;
            fn rdma_get_recv_comp(
                &mut self,
                rdma: Rdma,
                wc: IbvWc,
            ) -> Result<IbvWc, RdmaError>;
            fn rdma_reg_msgs<'a>(
                &mut self,
                rdma: Rdma,
                addr: &wiggle::GuestPtr<'a, u8>,
                size: u32,
            ) -> Result<IbvMr, RdmaError>;
            fn rdma_dereg_mr(&mut self, ibv_mr: IbvMr) -> ();
            fn rdma_post_send<'a>(
                &mut self,
                rdma: Rdma,
                addr: &wiggle::GuestPtr<'a, u8>,
                size: u32,
                send_mr: IbvMr,
                flags: u32,
            ) -> Result<(), RdmaError>;
            fn rdma_post_recv<'a>(
                &mut self,
                rdma: Rdma,
                addr: &wiggle::GuestPtr<'a, u8>,
                size: u32,
                recv_mr: IbvMr,
            ) -> Result<(), RdmaError>;
            fn ibv_query_qp(
                &mut self,
                rdma: Rdma,
                ibv_qp_attrmask: u32,
            ) -> Result<(), RdmaError>;
            fn print_hello_world(&mut self) -> ();
        }
        /// Adds all instance items to the specified `Linker`.
        pub fn add_to_linker<T, U>(
            linker: &mut wiggle::wasmtime_crate::Linker<T>,
            get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
        ) -> wiggle::anyhow::Result<()>
        where
            U: WasiEphemeralRdma,
        {
            linker
                .func_wrap(
                    "wasi_ephemeral_rdma",
                    "rdma_init",
                    move |
                        mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                        arg0: i32,
                        arg1: i32,
                        arg2: i32,
                        arg3: i32,
                        arg4: i32,
                        arg5: i32,
                        arg6: i32,
                        arg7: i32,
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
                                rdma_init(
                                    ctx,
                                    &mem,
                                    arg0,
                                    arg1,
                                    arg2,
                                    arg3,
                                    arg4,
                                    arg5,
                                    arg6,
                                    arg7,
                                )?,
                            ),
                        )
                    },
                )?;
            linker
                .func_wrap(
                    "wasi_ephemeral_rdma",
                    "rdma_connect",
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
                        Ok(<i32>::from(rdma_connect(ctx, &mem, arg0)?))
                    },
                )?;
            linker
                .func_wrap(
                    "wasi_ephemeral_rdma",
                    "rdma_disconnect",
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
                        Ok(<i32>::from(rdma_disconnect(ctx, &mem, arg0)?))
                    },
                )?;
            linker
                .func_wrap(
                    "wasi_ephemeral_rdma",
                    "rdma_get_send_comp",
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
                        Ok(<i32>::from(rdma_get_send_comp(ctx, &mem, arg0, arg1, arg2)?))
                    },
                )?;
            linker
                .func_wrap(
                    "wasi_ephemeral_rdma",
                    "rdma_get_recv_comp",
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
                        Ok(<i32>::from(rdma_get_recv_comp(ctx, &mem, arg0, arg1, arg2)?))
                    },
                )?;
            linker
                .func_wrap(
                    "wasi_ephemeral_rdma",
                    "rdma_reg_msgs",
                    move |
                        mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                        arg0: i32,
                        arg1: i32,
                        arg2: i32,
                        arg3: i32,
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
                                rdma_reg_msgs(ctx, &mem, arg0, arg1, arg2, arg3)?,
                            ),
                        )
                    },
                )?;
            linker
                .func_wrap(
                    "wasi_ephemeral_rdma",
                    "rdma_dereg_mr",
                    move |
                        mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                        arg0: i32,
                    | -> wiggle::anyhow::Result<()> {
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
                        Ok(<()>::from(rdma_dereg_mr(ctx, &mem, arg0)?))
                    },
                )?;
            linker
                .func_wrap(
                    "wasi_ephemeral_rdma",
                    "rdma_post_send",
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
                                rdma_post_send(ctx, &mem, arg0, arg1, arg2, arg3, arg4)?,
                            ),
                        )
                    },
                )?;
            linker
                .func_wrap(
                    "wasi_ephemeral_rdma",
                    "rdma_post_recv",
                    move |
                        mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                        arg0: i32,
                        arg1: i32,
                        arg2: i32,
                        arg3: i32,
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
                                rdma_post_recv(ctx, &mem, arg0, arg1, arg2, arg3)?,
                            ),
                        )
                    },
                )?;
            linker
                .func_wrap(
                    "wasi_ephemeral_rdma",
                    "ibv_query_qp",
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
                        Ok(<i32>::from(ibv_query_qp(ctx, &mem, arg0, arg1)?))
                    },
                )?;
            linker
                .func_wrap(
                    "wasi_ephemeral_rdma",
                    "print_hello_world",
                    move |
                        mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                    | -> wiggle::anyhow::Result<()> {
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
                        Ok(<()>::from(print_hello_world(ctx, &mem)?))
                    },
                )?;
            Ok(())
        }
    }
    impl wiggle::GuestErrorType for types::RdmaError {
        fn success() -> Self {
            Self::Success
        }
    }
}
