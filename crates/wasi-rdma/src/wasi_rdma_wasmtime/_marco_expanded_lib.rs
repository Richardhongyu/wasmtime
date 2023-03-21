#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use wiggle::from_witx;
use wiggle::GuestPtr;
use crate::rdma::Rdma;
struct WiggleCtx;
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
    pub enum AiFlags {
        RaiPassive,
        RaiNumerichost,
        RaiNoroute,
        RaiFamily,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AiFlags {
        #[inline]
        fn clone(&self) -> AiFlags {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AiFlags {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    AiFlags::RaiPassive => "RaiPassive",
                    AiFlags::RaiNumerichost => "RaiNumerichost",
                    AiFlags::RaiNoroute => "RaiNoroute",
                    AiFlags::RaiFamily => "RaiFamily",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AiFlags {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AiFlags {
        #[inline]
        fn eq(&self, other: &AiFlags) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AiFlags {}
    impl TryFrom<u8> for AiFlags {
        type Error = wiggle::GuestError;
        #[inline]
        fn try_from(value: u8) -> Result<AiFlags, wiggle::GuestError> {
            match value {
                0 => Ok(AiFlags::RaiPassive),
                1 => Ok(AiFlags::RaiNumerichost),
                2 => Ok(AiFlags::RaiNoroute),
                3 => Ok(AiFlags::RaiFamily),
                _ => Err(wiggle::GuestError::InvalidEnumValue("AiFlags")),
            }
        }
    }
    impl TryFrom<i32> for AiFlags {
        type Error = wiggle::GuestError;
        #[inline]
        fn try_from(value: i32) -> Result<AiFlags, wiggle::GuestError> {
            AiFlags::try_from(u8::try_from(value)?)
        }
    }
    impl From<AiFlags> for u8 {
        #[inline]
        fn from(v: AiFlags) -> u8 {
            match v {
                AiFlags::RaiPassive => 0,
                AiFlags::RaiNumerichost => 1,
                AiFlags::RaiNoroute => 2,
                AiFlags::RaiFamily => 3,
            }
        }
    }
    impl<'a> wiggle::GuestType<'a> for AiFlags {
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
                0 => Ok(AiFlags::RaiPassive),
                1 => Ok(AiFlags::RaiNumerichost),
                2 => Ok(AiFlags::RaiNoroute),
                3 => Ok(AiFlags::RaiFamily),
                _ => Err(wiggle::GuestError::InvalidEnumValue("AiFlags")),
            }
        }
        fn write(
            location: &wiggle::GuestPtr<'_, Self>,
            val: Self,
        ) -> Result<(), wiggle::GuestError> {
            match val {
                AiFlags::RaiPassive => {
                    location.cast().write(0usize as u8)?;
                }
                AiFlags::RaiNumerichost => {
                    location.cast().write(1usize as u8)?;
                }
                AiFlags::RaiNoroute => {
                    location.cast().write(2usize as u8)?;
                }
                AiFlags::RaiFamily => {
                    location.cast().write(3usize as u8)?;
                }
            }
            Ok(())
        }
    }
    pub enum AiPortSpace {
        RdmaPsUdp,
        RdmaPsTcp,
        RdmaPsIb,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AiPortSpace {
        #[inline]
        fn clone(&self) -> AiPortSpace {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AiPortSpace {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    AiPortSpace::RdmaPsUdp => "RdmaPsUdp",
                    AiPortSpace::RdmaPsTcp => "RdmaPsTcp",
                    AiPortSpace::RdmaPsIb => "RdmaPsIb",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AiPortSpace {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AiPortSpace {
        #[inline]
        fn eq(&self, other: &AiPortSpace) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AiPortSpace {}
    impl TryFrom<u8> for AiPortSpace {
        type Error = wiggle::GuestError;
        #[inline]
        fn try_from(value: u8) -> Result<AiPortSpace, wiggle::GuestError> {
            match value {
                0 => Ok(AiPortSpace::RdmaPsUdp),
                1 => Ok(AiPortSpace::RdmaPsTcp),
                2 => Ok(AiPortSpace::RdmaPsIb),
                _ => Err(wiggle::GuestError::InvalidEnumValue("AiPortSpace")),
            }
        }
    }
    impl TryFrom<i32> for AiPortSpace {
        type Error = wiggle::GuestError;
        #[inline]
        fn try_from(value: i32) -> Result<AiPortSpace, wiggle::GuestError> {
            AiPortSpace::try_from(u8::try_from(value)?)
        }
    }
    impl From<AiPortSpace> for u8 {
        #[inline]
        fn from(v: AiPortSpace) -> u8 {
            match v {
                AiPortSpace::RdmaPsUdp => 0,
                AiPortSpace::RdmaPsTcp => 1,
                AiPortSpace::RdmaPsIb => 2,
            }
        }
    }
    impl<'a> wiggle::GuestType<'a> for AiPortSpace {
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
                0 => Ok(AiPortSpace::RdmaPsUdp),
                1 => Ok(AiPortSpace::RdmaPsTcp),
                2 => Ok(AiPortSpace::RdmaPsIb),
                _ => Err(wiggle::GuestError::InvalidEnumValue("AiPortSpace")),
            }
        }
        fn write(
            location: &wiggle::GuestPtr<'_, Self>,
            val: Self,
        ) -> Result<(), wiggle::GuestError> {
            match val {
                AiPortSpace::RdmaPsUdp => {
                    location.cast().write(0usize as u8)?;
                }
                AiPortSpace::RdmaPsTcp => {
                    location.cast().write(1usize as u8)?;
                }
                AiPortSpace::RdmaPsIb => {
                    location.cast().write(2usize as u8)?;
                }
            }
            Ok(())
        }
    }
    pub enum AiFamliy {
        AfIb,
        AfInet,
        AfInet6,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AiFamliy {
        #[inline]
        fn clone(&self) -> AiFamliy {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AiFamliy {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    AiFamliy::AfIb => "AfIb",
                    AiFamliy::AfInet => "AfInet",
                    AiFamliy::AfInet6 => "AfInet6",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AiFamliy {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AiFamliy {
        #[inline]
        fn eq(&self, other: &AiFamliy) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AiFamliy {}
    impl TryFrom<u8> for AiFamliy {
        type Error = wiggle::GuestError;
        #[inline]
        fn try_from(value: u8) -> Result<AiFamliy, wiggle::GuestError> {
            match value {
                0 => Ok(AiFamliy::AfIb),
                1 => Ok(AiFamliy::AfInet),
                2 => Ok(AiFamliy::AfInet6),
                _ => Err(wiggle::GuestError::InvalidEnumValue("AiFamliy")),
            }
        }
    }
    impl TryFrom<i32> for AiFamliy {
        type Error = wiggle::GuestError;
        #[inline]
        fn try_from(value: i32) -> Result<AiFamliy, wiggle::GuestError> {
            AiFamliy::try_from(u8::try_from(value)?)
        }
    }
    impl From<AiFamliy> for u8 {
        #[inline]
        fn from(v: AiFamliy) -> u8 {
            match v {
                AiFamliy::AfIb => 0,
                AiFamliy::AfInet => 1,
                AiFamliy::AfInet6 => 2,
            }
        }
    }
    impl<'a> wiggle::GuestType<'a> for AiFamliy {
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
                0 => Ok(AiFamliy::AfIb),
                1 => Ok(AiFamliy::AfInet),
                2 => Ok(AiFamliy::AfInet6),
                _ => Err(wiggle::GuestError::InvalidEnumValue("AiFamliy")),
            }
        }
        fn write(
            location: &wiggle::GuestPtr<'_, Self>,
            val: Self,
        ) -> Result<(), wiggle::GuestError> {
            match val {
                AiFamliy::AfIb => {
                    location.cast().write(0usize as u8)?;
                }
                AiFamliy::AfInet => {
                    location.cast().write(1usize as u8)?;
                }
                AiFamliy::AfInet6 => {
                    location.cast().write(2usize as u8)?;
                }
            }
            Ok(())
        }
    }
    pub enum AiQpType {
        IbvUd,
        IbvRc,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AiQpType {
        #[inline]
        fn clone(&self) -> AiQpType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AiQpType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    AiQpType::IbvUd => "IbvUd",
                    AiQpType::IbvRc => "IbvRc",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AiQpType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AiQpType {
        #[inline]
        fn eq(&self, other: &AiQpType) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for AiQpType {}
    impl TryFrom<u8> for AiQpType {
        type Error = wiggle::GuestError;
        #[inline]
        fn try_from(value: u8) -> Result<AiQpType, wiggle::GuestError> {
            match value {
                0 => Ok(AiQpType::IbvUd),
                1 => Ok(AiQpType::IbvRc),
                _ => Err(wiggle::GuestError::InvalidEnumValue("AiQpType")),
            }
        }
    }
    impl TryFrom<i32> for AiQpType {
        type Error = wiggle::GuestError;
        #[inline]
        fn try_from(value: i32) -> Result<AiQpType, wiggle::GuestError> {
            AiQpType::try_from(u8::try_from(value)?)
        }
    }
    impl From<AiQpType> for u8 {
        #[inline]
        fn from(v: AiQpType) -> u8 {
            match v {
                AiQpType::IbvUd => 0,
                AiQpType::IbvRc => 1,
            }
        }
    }
    impl<'a> wiggle::GuestType<'a> for AiQpType {
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
                0 => Ok(AiQpType::IbvUd),
                1 => Ok(AiQpType::IbvRc),
                _ => Err(wiggle::GuestError::InvalidEnumValue("AiQpType")),
            }
        }
        fn write(
            location: &wiggle::GuestPtr<'_, Self>,
            val: Self,
        ) -> Result<(), wiggle::GuestError> {
            match val {
                AiQpType::IbvUd => {
                    location.cast().write(0usize as u8)?;
                }
                AiQpType::IbvRc => {
                    location.cast().write(1usize as u8)?;
                }
            }
            Ok(())
        }
    }
    pub enum IbvQpType {
        IbvQptRc,
        IbvQptUc,
        IbvQptUd,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for IbvQpType {
        #[inline]
        fn clone(&self) -> IbvQpType {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for IbvQpType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    IbvQpType::IbvQptRc => "IbvQptRc",
                    IbvQpType::IbvQptUc => "IbvQptUc",
                    IbvQpType::IbvQptUd => "IbvQptUd",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for IbvQpType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for IbvQpType {
        #[inline]
        fn eq(&self, other: &IbvQpType) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for IbvQpType {}
    impl TryFrom<u8> for IbvQpType {
        type Error = wiggle::GuestError;
        #[inline]
        fn try_from(value: u8) -> Result<IbvQpType, wiggle::GuestError> {
            match value {
                0 => Ok(IbvQpType::IbvQptRc),
                1 => Ok(IbvQpType::IbvQptUc),
                2 => Ok(IbvQpType::IbvQptUd),
                _ => Err(wiggle::GuestError::InvalidEnumValue("IbvQpType")),
            }
        }
    }
    impl TryFrom<i32> for IbvQpType {
        type Error = wiggle::GuestError;
        #[inline]
        fn try_from(value: i32) -> Result<IbvQpType, wiggle::GuestError> {
            IbvQpType::try_from(u8::try_from(value)?)
        }
    }
    impl From<IbvQpType> for u8 {
        #[inline]
        fn from(v: IbvQpType) -> u8 {
            match v {
                IbvQpType::IbvQptRc => 0,
                IbvQpType::IbvQptUc => 1,
                IbvQpType::IbvQptUd => 2,
            }
        }
    }
    impl<'a> wiggle::GuestType<'a> for IbvQpType {
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
                0 => Ok(IbvQpType::IbvQptRc),
                1 => Ok(IbvQpType::IbvQptUc),
                2 => Ok(IbvQpType::IbvQptUd),
                _ => Err(wiggle::GuestError::InvalidEnumValue("IbvQpType")),
            }
        }
        fn write(
            location: &wiggle::GuestPtr<'_, Self>,
            val: Self,
        ) -> Result<(), wiggle::GuestError> {
            match val {
                IbvQpType::IbvQptRc => {
                    location.cast().write(0usize as u8)?;
                }
                IbvQpType::IbvQptUc => {
                    location.cast().write(1usize as u8)?;
                }
                IbvQpType::IbvQptUd => {
                    location.cast().write(2usize as u8)?;
                }
            }
            Ok(())
        }
    }
    #[repr(transparent)]
    pub struct Id(u32);
    #[automatically_derived]
    impl ::core::marker::Copy for Id {}
    #[automatically_derived]
    impl ::core::clone::Clone for Id {
        #[inline]
        fn clone(&self) -> Id {
            let _: ::core::clone::AssertParamIsClone<u32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Id {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Id", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Id {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Id {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Id {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Id {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Id {
        #[inline]
        fn eq(&self, other: &Id) -> bool {
            self.0 == other.0
        }
    }
    impl Id {
        #[inline]
        pub unsafe fn inner(&self) -> u32 {
            self.0
        }
    }
    impl From<Id> for u32 {
        #[inline]
        fn from(e: Id) -> u32 {
            e.0
        }
    }
    impl From<Id> for i32 {
        #[inline]
        fn from(e: Id) -> i32 {
            e.0 as i32
        }
    }
    impl From<u32> for Id {
        #[inline]
        fn from(e: u32) -> Id {
            Id(e)
        }
    }
    impl From<i32> for Id {
        #[inline]
        fn from(e: i32) -> Id {
            Id(e as u32)
        }
    }
    impl ::std::fmt::Display for Id {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{0}({1})", "Id", self.0))
        }
    }
    impl<'a> wiggle::GuestType<'a> for Id {
        #[inline]
        fn guest_size() -> u32 {
            4u32
        }
        #[inline]
        fn guest_align() -> usize {
            4usize
        }
        #[inline]
        fn read(location: &wiggle::GuestPtr<'a, Id>) -> Result<Id, wiggle::GuestError> {
            Ok(Id(u32::read(&location.cast())?))
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
    #[repr(transparent)]
    pub struct RdmaAddrinfo(u32);
    #[automatically_derived]
    impl ::core::marker::Copy for RdmaAddrinfo {}
    #[automatically_derived]
    impl ::core::clone::Clone for RdmaAddrinfo {
        #[inline]
        fn clone(&self) -> RdmaAddrinfo {
            let _: ::core::clone::AssertParamIsClone<u32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for RdmaAddrinfo {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(
                f,
                "RdmaAddrinfo",
                &&self.0,
            )
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for RdmaAddrinfo {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for RdmaAddrinfo {}
    #[automatically_derived]
    impl ::core::cmp::Eq for RdmaAddrinfo {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RdmaAddrinfo {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RdmaAddrinfo {
        #[inline]
        fn eq(&self, other: &RdmaAddrinfo) -> bool {
            self.0 == other.0
        }
    }
    impl RdmaAddrinfo {
        #[inline]
        pub unsafe fn inner(&self) -> u32 {
            self.0
        }
    }
    impl From<RdmaAddrinfo> for u32 {
        #[inline]
        fn from(e: RdmaAddrinfo) -> u32 {
            e.0
        }
    }
    impl From<RdmaAddrinfo> for i32 {
        #[inline]
        fn from(e: RdmaAddrinfo) -> i32 {
            e.0 as i32
        }
    }
    impl From<u32> for RdmaAddrinfo {
        #[inline]
        fn from(e: u32) -> RdmaAddrinfo {
            RdmaAddrinfo(e)
        }
    }
    impl From<i32> for RdmaAddrinfo {
        #[inline]
        fn from(e: i32) -> RdmaAddrinfo {
            RdmaAddrinfo(e as u32)
        }
    }
    impl ::std::fmt::Display for RdmaAddrinfo {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{0}({1})", "RdmaAddrinfo", self.0))
        }
    }
    impl<'a> wiggle::GuestType<'a> for RdmaAddrinfo {
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
            location: &wiggle::GuestPtr<'a, RdmaAddrinfo>,
        ) -> Result<RdmaAddrinfo, wiggle::GuestError> {
            Ok(RdmaAddrinfo(u32::read(&location.cast())?))
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
    pub struct Context(u32);
    #[automatically_derived]
    impl ::core::marker::Copy for Context {}
    #[automatically_derived]
    impl ::core::clone::Clone for Context {
        #[inline]
        fn clone(&self) -> Context {
            let _: ::core::clone::AssertParamIsClone<u32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Context {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Context", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for Context {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Context {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Context {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Context {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Context {
        #[inline]
        fn eq(&self, other: &Context) -> bool {
            self.0 == other.0
        }
    }
    impl Context {
        #[inline]
        pub unsafe fn inner(&self) -> u32 {
            self.0
        }
    }
    impl From<Context> for u32 {
        #[inline]
        fn from(e: Context) -> u32 {
            e.0
        }
    }
    impl From<Context> for i32 {
        #[inline]
        fn from(e: Context) -> i32 {
            e.0 as i32
        }
    }
    impl From<u32> for Context {
        #[inline]
        fn from(e: u32) -> Context {
            Context(e)
        }
    }
    impl From<i32> for Context {
        #[inline]
        fn from(e: i32) -> Context {
            Context(e as u32)
        }
    }
    impl ::std::fmt::Display for Context {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{0}({1})", "Context", self.0))
        }
    }
    impl<'a> wiggle::GuestType<'a> for Context {
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
            location: &wiggle::GuestPtr<'a, Context>,
        ) -> Result<Context, wiggle::GuestError> {
            Ok(Context(u32::read(&location.cast())?))
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
    pub struct ConnParam(u32);
    #[automatically_derived]
    impl ::core::marker::Copy for ConnParam {}
    #[automatically_derived]
    impl ::core::clone::Clone for ConnParam {
        #[inline]
        fn clone(&self) -> ConnParam {
            let _: ::core::clone::AssertParamIsClone<u32>;
            *self
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ConnParam {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "ConnParam", &&self.0)
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for ConnParam {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.0, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for ConnParam {}
    #[automatically_derived]
    impl ::core::cmp::Eq for ConnParam {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<u32>;
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for ConnParam {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for ConnParam {
        #[inline]
        fn eq(&self, other: &ConnParam) -> bool {
            self.0 == other.0
        }
    }
    impl ConnParam {
        #[inline]
        pub unsafe fn inner(&self) -> u32 {
            self.0
        }
    }
    impl From<ConnParam> for u32 {
        #[inline]
        fn from(e: ConnParam) -> u32 {
            e.0
        }
    }
    impl From<ConnParam> for i32 {
        #[inline]
        fn from(e: ConnParam) -> i32 {
            e.0 as i32
        }
    }
    impl From<u32> for ConnParam {
        #[inline]
        fn from(e: u32) -> ConnParam {
            ConnParam(e)
        }
    }
    impl From<i32> for ConnParam {
        #[inline]
        fn from(e: i32) -> ConnParam {
            ConnParam(e as u32)
        }
    }
    impl ::std::fmt::Display for ConnParam {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            f.write_fmt(format_args!("{0}({1})", "ConnParam", self.0))
        }
    }
    impl<'a> wiggle::GuestType<'a> for ConnParam {
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
            location: &wiggle::GuestPtr<'a, ConnParam>,
        ) -> Result<ConnParam, wiggle::GuestError> {
            Ok(ConnParam(u32::read(&location.cast())?))
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
        pub flags: AiFlags,
        pub port_space: AiPortSpace,
        pub family: AiFamliy,
        pub qp_type: AiQpType,
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
            12u32
        }
        #[inline]
        fn guest_align() -> usize {
            4usize
        }
        fn read(
            location: &wiggle::GuestPtr<'a, Self>,
        ) -> Result<Self, wiggle::GuestError> {
            let flags = <AiFlags as wiggle::GuestType>::read(
                &location.cast::<u8>().add(0u32)?.cast(),
            )?;
            let port_space = <AiPortSpace as wiggle::GuestType>::read(
                &location.cast::<u8>().add(1u32)?.cast(),
            )?;
            let family = <AiFamliy as wiggle::GuestType>::read(
                &location.cast::<u8>().add(2u32)?.cast(),
            )?;
            let qp_type = <AiQpType as wiggle::GuestType>::read(
                &location.cast::<u8>().add(3u32)?.cast(),
            )?;
            let src_len = <u32 as wiggle::GuestType>::read(
                &location.cast::<u8>().add(4u32)?.cast(),
            )?;
            let dst_len = <u32 as wiggle::GuestType>::read(
                &location.cast::<u8>().add(8u32)?.cast(),
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
                &location.cast::<u8>().add(1u32)?.cast(),
                val.port_space,
            )?;
            wiggle::GuestType::write(
                &location.cast::<u8>().add(2u32)?.cast(),
                val.family,
            )?;
            wiggle::GuestType::write(
                &location.cast::<u8>().add(3u32)?.cast(),
                val.qp_type,
            )?;
            wiggle::GuestType::write(
                &location.cast::<u8>().add(4u32)?.cast(),
                val.src_len,
            )?;
            wiggle::GuestType::write(
                &location.cast::<u8>().add(8u32)?.cast(),
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
    pub struct IbvQpInitAttr {
        pub qp_context: Id,
        pub send_cq: RdmaCq,
        pub recv_cq: RdmaCq,
        pub cap: IbvQpCap,
        pub qp_type: IbvQpType,
        pub sq_sig_all: u8,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for IbvQpInitAttr {
        #[inline]
        fn clone(&self) -> IbvQpInitAttr {
            IbvQpInitAttr {
                qp_context: ::core::clone::Clone::clone(&self.qp_context),
                send_cq: ::core::clone::Clone::clone(&self.send_cq),
                recv_cq: ::core::clone::Clone::clone(&self.recv_cq),
                cap: ::core::clone::Clone::clone(&self.cap),
                qp_type: ::core::clone::Clone::clone(&self.qp_type),
                sq_sig_all: ::core::clone::Clone::clone(&self.sq_sig_all),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for IbvQpInitAttr {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "qp_context",
                "send_cq",
                "recv_cq",
                "cap",
                "qp_type",
                "sq_sig_all",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.qp_context,
                &self.send_cq,
                &self.recv_cq,
                &self.cap,
                &self.qp_type,
                &&self.sq_sig_all,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "IbvQpInitAttr",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for IbvQpInitAttr {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for IbvQpInitAttr {
        #[inline]
        fn eq(&self, other: &IbvQpInitAttr) -> bool {
            self.qp_context == other.qp_context && self.send_cq == other.send_cq
                && self.recv_cq == other.recv_cq && self.cap == other.cap
                && self.qp_type == other.qp_type && self.sq_sig_all == other.sq_sig_all
        }
    }
    impl<'a> wiggle::GuestType<'a> for IbvQpInitAttr {
        #[inline]
        fn guest_size() -> u32 {
            36u32
        }
        #[inline]
        fn guest_align() -> usize {
            4usize
        }
        fn read(
            location: &wiggle::GuestPtr<'a, Self>,
        ) -> Result<Self, wiggle::GuestError> {
            let qp_context = <Id as wiggle::GuestType>::read(
                &location.cast::<u8>().add(0u32)?.cast(),
            )?;
            let send_cq = <RdmaCq as wiggle::GuestType>::read(
                &location.cast::<u8>().add(4u32)?.cast(),
            )?;
            let recv_cq = <RdmaCq as wiggle::GuestType>::read(
                &location.cast::<u8>().add(8u32)?.cast(),
            )?;
            let cap = <IbvQpCap as wiggle::GuestType>::read(
                &location.cast::<u8>().add(12u32)?.cast(),
            )?;
            let qp_type = <IbvQpType as wiggle::GuestType>::read(
                &location.cast::<u8>().add(32u32)?.cast(),
            )?;
            let sq_sig_all = <u8 as wiggle::GuestType>::read(
                &location.cast::<u8>().add(33u32)?.cast(),
            )?;
            Ok(IbvQpInitAttr {
                qp_context,
                send_cq,
                recv_cq,
                cap,
                qp_type,
                sq_sig_all,
            })
        }
        fn write(
            location: &wiggle::GuestPtr<'_, Self>,
            val: Self,
        ) -> Result<(), wiggle::GuestError> {
            wiggle::GuestType::write(
                &location.cast::<u8>().add(0u32)?.cast(),
                val.qp_context,
            )?;
            wiggle::GuestType::write(
                &location.cast::<u8>().add(4u32)?.cast(),
                val.send_cq,
            )?;
            wiggle::GuestType::write(
                &location.cast::<u8>().add(8u32)?.cast(),
                val.recv_cq,
            )?;
            wiggle::GuestType::write(
                &location.cast::<u8>().add(12u32)?.cast(),
                val.cap,
            )?;
            wiggle::GuestType::write(
                &location.cast::<u8>().add(32u32)?.cast(),
                val.qp_type,
            )?;
            wiggle::GuestType::write(
                &location.cast::<u8>().add(33u32)?.cast(),
                val.sq_sig_all,
            )?;
            Ok(())
        }
    }
    pub trait UserErrorConversion {}
}
pub mod rdma {
    use super::types::*;
    pub use super::types::UserErrorConversion;
    #[allow(unreachable_code)]
    pub fn rdma_getaddrinfo<'a>(
        ctx: &'a mut (impl Rdma),
        memory: &dyn wiggle::GuestMemory,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
        arg5: i32,
    ) -> wiggle::anyhow::Result<i32> {
        use std::convert::TryFrom as _;
        let _span = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "wiggle abi",
                        "untitled4::rdma",
                        wiggle::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(6u32),
                        Some("untitled4::rdma"),
                        ::tracing_core::field::FieldSet::new(
                            &["module", "function"],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if wiggle::tracing::Level::TRACE
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && ::tracing::__macro_support::__is_enabled(
                    CALLSITE.metadata(),
                    interest,
                )
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
                                        Some(&"rdma" as &dyn Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"rdma_getaddrinfo" as &dyn Value),
                                    ),
                                ],
                            )
                    },
                )
            } else {
                let span = ::tracing::__macro_support::__disabled_span(
                    CALLSITE.metadata(),
                );
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
                            modulename: "rdma",
                            funcname: "rdma_getaddrinfo",
                            location: "read rdma_addrinfo_struct",
                            err: Box::new(wiggle::GuestError::from(e)),
                        }
                    })?;
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["node", "service", "hints"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::debug(&node) as &dyn Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&wiggle::tracing::field::debug(&service) as &dyn Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&wiggle::tracing::field::debug(&hints) as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                let ret = Rdma::rdma_getaddrinfo(ctx, &node, &service, &hints);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["result"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::debug(&ret) as &dyn Value),
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
                            wiggle::GuestPtr::<RdmaAddrinfo>::new(memory, arg5 as u32)
                                .write(e)
                                .map_err(|e| {
                                    wiggle::GuestError::InFunc {
                                        modulename: "rdma",
                                        funcname: "rdma_getaddrinfo",
                                        location: "write rdma_addrinfo",
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
    pub fn rdma_create_ep<'a>(
        ctx: &'a mut (impl Rdma),
        memory: &dyn wiggle::GuestMemory,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) -> wiggle::anyhow::Result<i32> {
        use std::convert::TryFrom as _;
        let _span = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "wiggle abi",
                        "untitled4::rdma",
                        wiggle::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(6u32),
                        Some("untitled4::rdma"),
                        ::tracing_core::field::FieldSet::new(
                            &["module", "function"],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if wiggle::tracing::Level::TRACE
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && ::tracing::__macro_support::__is_enabled(
                    CALLSITE.metadata(),
                    interest,
                )
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
                                        Some(&"rdma" as &dyn Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"rdma_create_ep" as &dyn Value),
                                    ),
                                ],
                            )
                    },
                )
            } else {
                let span = ::tracing::__macro_support::__disabled_span(
                    CALLSITE.metadata(),
                );
                {};
                span
            }
        };
        _span
            .in_scope(|| {
                let id = Id::from(arg0);
                let res = RdmaAddrinfo::from(arg1);
                let pd = EpPd::from(arg2);
                let qp_init_attr = wiggle::GuestPtr::<
                    IbvQpInitAttr,
                >::new(memory, arg3 as u32)
                    .read()
                    .map_err(|e| {
                        wiggle::GuestError::InFunc {
                            modulename: "rdma",
                            funcname: "rdma_create_ep",
                            location: "read ibv_qp_init_attr",
                            err: Box::new(wiggle::GuestError::from(e)),
                        }
                    })?;
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["id", "res", "pd", "qp_init_attr"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::display(&id) as &dyn Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&wiggle::tracing::field::display(&res) as &dyn Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&wiggle::tracing::field::display(&pd) as &dyn Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(
                                                &wiggle::tracing::field::debug(&qp_init_attr) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                let ret = Rdma::rdma_create_ep(ctx, id, res, pd, &qp_init_attr);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["result"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::debug(&ret) as &dyn Value),
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
    pub fn rdma_freeaddrinfo<'a>(
        ctx: &'a mut (impl Rdma),
        memory: &dyn wiggle::GuestMemory,
        arg0: i32,
    ) -> wiggle::anyhow::Result<i32> {
        use std::convert::TryFrom as _;
        let _span = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "wiggle abi",
                        "untitled4::rdma",
                        wiggle::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(6u32),
                        Some("untitled4::rdma"),
                        ::tracing_core::field::FieldSet::new(
                            &["module", "function"],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if wiggle::tracing::Level::TRACE
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && ::tracing::__macro_support::__is_enabled(
                    CALLSITE.metadata(),
                    interest,
                )
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
                                        Some(&"rdma" as &dyn Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"rdma_freeaddrinfo" as &dyn Value),
                                    ),
                                ],
                            )
                    },
                )
            } else {
                let span = ::tracing::__macro_support::__disabled_span(
                    CALLSITE.metadata(),
                );
                {};
                span
            }
        };
        _span
            .in_scope(|| {
                let res = RdmaAddrinfo::from(arg0);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["res"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::display(&res) as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                let ret = Rdma::rdma_freeaddrinfo(ctx, res);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["result"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::debug(&ret) as &dyn Value),
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
    pub fn rdma_reg_msgs<'a>(
        ctx: &'a mut (impl Rdma),
        memory: &dyn wiggle::GuestMemory,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> wiggle::anyhow::Result<i32> {
        use std::convert::TryFrom as _;
        let _span = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "wiggle abi",
                        "untitled4::rdma",
                        wiggle::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(6u32),
                        Some("untitled4::rdma"),
                        ::tracing_core::field::FieldSet::new(
                            &["module", "function"],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if wiggle::tracing::Level::TRACE
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && ::tracing::__macro_support::__is_enabled(
                    CALLSITE.metadata(),
                    interest,
                )
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
                                        Some(&"rdma" as &dyn Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"rdma_reg_msgs" as &dyn Value),
                                    ),
                                ],
                            )
                    },
                )
            } else {
                let span = ::tracing::__macro_support::__disabled_span(
                    CALLSITE.metadata(),
                );
                {};
                span
            }
        };
        _span
            .in_scope(|| {
                let id = Id::from(arg0);
                let addr = wiggle::GuestPtr::<u8>::new(memory, arg1 as u32);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["id", "addr"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::display(&id) as &dyn Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&wiggle::tracing::field::debug(&addr) as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                let ret = Rdma::rdma_reg_msgs(ctx, id, &addr);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["result"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::debug(&ret) as &dyn Value),
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
                            wiggle::GuestPtr::<IbvMr>::new(memory, arg2 as u32)
                                .write(e)
                                .map_err(|e| {
                                    wiggle::GuestError::InFunc {
                                        modulename: "rdma",
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
        ctx: &'a mut (impl Rdma),
        memory: &dyn wiggle::GuestMemory,
        arg0: i32,
    ) -> wiggle::anyhow::Result<i32> {
        use std::convert::TryFrom as _;
        let _span = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "wiggle abi",
                        "untitled4::rdma",
                        wiggle::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(6u32),
                        Some("untitled4::rdma"),
                        ::tracing_core::field::FieldSet::new(
                            &["module", "function"],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if wiggle::tracing::Level::TRACE
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && ::tracing::__macro_support::__is_enabled(
                    CALLSITE.metadata(),
                    interest,
                )
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
                                        Some(&"rdma" as &dyn Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"rdma_dereg_mr" as &dyn Value),
                                    ),
                                ],
                            )
                    },
                )
            } else {
                let span = ::tracing::__macro_support::__disabled_span(
                    CALLSITE.metadata(),
                );
                {};
                span
            }
        };
        _span
            .in_scope(|| {
                let mr = IbvMr::from(arg0);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["mr"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::display(&mr) as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                let ret = Rdma::rdma_dereg_mr(ctx, mr);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["result"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::debug(&ret) as &dyn Value),
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
    pub fn rdma_post_send<'a>(
        ctx: &'a mut (impl Rdma),
        memory: &dyn wiggle::GuestMemory,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
        arg4: i32,
    ) -> wiggle::anyhow::Result<()> {
        use std::convert::TryFrom as _;
        let _span = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "wiggle abi",
                        "untitled4::rdma",
                        wiggle::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(6u32),
                        Some("untitled4::rdma"),
                        ::tracing_core::field::FieldSet::new(
                            &["module", "function"],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if wiggle::tracing::Level::TRACE
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && ::tracing::__macro_support::__is_enabled(
                    CALLSITE.metadata(),
                    interest,
                )
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
                                        Some(&"rdma" as &dyn Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"rdma_post_send" as &dyn Value),
                                    ),
                                ],
                            )
                    },
                )
            } else {
                let span = ::tracing::__macro_support::__disabled_span(
                    CALLSITE.metadata(),
                );
                {};
                span
            }
        };
        _span
            .in_scope(|| {
                let id = Id::from(arg0);
                let context = Context::from(arg1);
                let addr = wiggle::GuestPtr::<u8>::new(memory, arg2 as u32);
                let send_mr = IbvMr::from(arg3);
                let flags = arg4 as u32;
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["id", "context", "addr", "send_mr", "flags"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::display(&id) as &dyn Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(
                                                &wiggle::tracing::field::display(&context) as &dyn Value,
                                            ),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&wiggle::tracing::field::debug(&addr) as &dyn Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(
                                                &wiggle::tracing::field::display(&send_mr) as &dyn Value,
                                            ),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&wiggle::tracing::field::display(&flags) as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                let ret = Rdma::rdma_post_send(ctx, id, context, &addr, send_mr, flags);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["result"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::debug(&ret) as &dyn Value),
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
    pub fn rdma_destroy_ep<'a>(
        ctx: &'a mut (impl Rdma),
        memory: &dyn wiggle::GuestMemory,
        arg0: i32,
    ) -> wiggle::anyhow::Result<i32> {
        use std::convert::TryFrom as _;
        let _span = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "wiggle abi",
                        "untitled4::rdma",
                        wiggle::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(6u32),
                        Some("untitled4::rdma"),
                        ::tracing_core::field::FieldSet::new(
                            &["module", "function"],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if wiggle::tracing::Level::TRACE
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && ::tracing::__macro_support::__is_enabled(
                    CALLSITE.metadata(),
                    interest,
                )
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
                                        Some(&"rdma" as &dyn Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"rdma_destroy_ep" as &dyn Value),
                                    ),
                                ],
                            )
                    },
                )
            } else {
                let span = ::tracing::__macro_support::__disabled_span(
                    CALLSITE.metadata(),
                );
                {};
                span
            }
        };
        _span
            .in_scope(|| {
                let id = Id::from(arg0);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["id"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::display(&id) as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                let ret = Rdma::rdma_destroy_ep(ctx, id);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["result"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::debug(&ret) as &dyn Value),
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
        ctx: &'a mut (impl Rdma),
        memory: &dyn wiggle::GuestMemory,
        arg0: i32,
        arg1: i32,
        arg2: i32,
        arg3: i32,
    ) -> wiggle::anyhow::Result<()> {
        use std::convert::TryFrom as _;
        let _span = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "wiggle abi",
                        "untitled4::rdma",
                        wiggle::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(6u32),
                        Some("untitled4::rdma"),
                        ::tracing_core::field::FieldSet::new(
                            &["module", "function"],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if wiggle::tracing::Level::TRACE
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && ::tracing::__macro_support::__is_enabled(
                    CALLSITE.metadata(),
                    interest,
                )
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
                                        Some(&"rdma" as &dyn Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"rdma_post_recv" as &dyn Value),
                                    ),
                                ],
                            )
                    },
                )
            } else {
                let span = ::tracing::__macro_support::__disabled_span(
                    CALLSITE.metadata(),
                );
                {};
                span
            }
        };
        _span
            .in_scope(|| {
                let id = Id::from(arg0);
                let context = Context::from(arg1);
                let addr = wiggle::GuestPtr::<u8>::new(memory, arg2 as u32);
                let recv_mr = IbvMr::from(arg3);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["id", "context", "addr", "recv_mr"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::display(&id) as &dyn Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(
                                                &wiggle::tracing::field::display(&context) as &dyn Value,
                                            ),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&wiggle::tracing::field::debug(&addr) as &dyn Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(
                                                &wiggle::tracing::field::display(&recv_mr) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                let ret = Rdma::rdma_post_recv(ctx, id, context, &addr, recv_mr);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["result"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::debug(&ret) as &dyn Value),
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
    pub fn rdma_get_send_comp<'a>(
        ctx: &'a mut (impl Rdma),
        memory: &dyn wiggle::GuestMemory,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> wiggle::anyhow::Result<i32> {
        use std::convert::TryFrom as _;
        let _span = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "wiggle abi",
                        "untitled4::rdma",
                        wiggle::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(6u32),
                        Some("untitled4::rdma"),
                        ::tracing_core::field::FieldSet::new(
                            &["module", "function"],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if wiggle::tracing::Level::TRACE
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && ::tracing::__macro_support::__is_enabled(
                    CALLSITE.metadata(),
                    interest,
                )
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
                                        Some(&"rdma" as &dyn Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"rdma_get_send_comp" as &dyn Value),
                                    ),
                                ],
                            )
                    },
                )
            } else {
                let span = ::tracing::__macro_support::__disabled_span(
                    CALLSITE.metadata(),
                );
                {};
                span
            }
        };
        _span
            .in_scope(|| {
                let id = Id::from(arg0);
                let wc = IbvWc::from(arg1);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["id", "wc"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::display(&id) as &dyn Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&wiggle::tracing::field::display(&wc) as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                let ret = Rdma::rdma_get_send_comp(ctx, id, wc);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["result"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::debug(&ret) as &dyn Value),
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
                                        modulename: "rdma",
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
        ctx: &'a mut (impl Rdma),
        memory: &dyn wiggle::GuestMemory,
        arg0: i32,
        arg1: i32,
        arg2: i32,
    ) -> wiggle::anyhow::Result<i32> {
        use std::convert::TryFrom as _;
        let _span = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "wiggle abi",
                        "untitled4::rdma",
                        wiggle::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(6u32),
                        Some("untitled4::rdma"),
                        ::tracing_core::field::FieldSet::new(
                            &["module", "function"],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if wiggle::tracing::Level::TRACE
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && ::tracing::__macro_support::__is_enabled(
                    CALLSITE.metadata(),
                    interest,
                )
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
                                        Some(&"rdma" as &dyn Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"rdma_get_recv_comp" as &dyn Value),
                                    ),
                                ],
                            )
                    },
                )
            } else {
                let span = ::tracing::__macro_support::__disabled_span(
                    CALLSITE.metadata(),
                );
                {};
                span
            }
        };
        _span
            .in_scope(|| {
                let id = Id::from(arg0);
                let wc = IbvWc::from(arg1);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["id", "wc"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::display(&id) as &dyn Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(&wiggle::tracing::field::display(&wc) as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                let ret = Rdma::rdma_get_recv_comp(ctx, id, wc);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["result"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::debug(&ret) as &dyn Value),
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
                                        modulename: "rdma",
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
    pub fn rdma_connect<'a>(
        ctx: &'a mut (impl Rdma),
        memory: &dyn wiggle::GuestMemory,
        arg0: i32,
        arg1: i32,
    ) -> wiggle::anyhow::Result<()> {
        use std::convert::TryFrom as _;
        let _span = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "wiggle abi",
                        "untitled4::rdma",
                        wiggle::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(6u32),
                        Some("untitled4::rdma"),
                        ::tracing_core::field::FieldSet::new(
                            &["module", "function"],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if wiggle::tracing::Level::TRACE
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && ::tracing::__macro_support::__is_enabled(
                    CALLSITE.metadata(),
                    interest,
                )
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
                                        Some(&"rdma" as &dyn Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"rdma_connect" as &dyn Value),
                                    ),
                                ],
                            )
                    },
                )
            } else {
                let span = ::tracing::__macro_support::__disabled_span(
                    CALLSITE.metadata(),
                );
                {};
                span
            }
        };
        _span
            .in_scope(|| {
                let id = Id::from(arg0);
                let conn_param = ConnParam::from(arg1);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["id", "conn_param"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::display(&id) as &dyn Value),
                                        ),
                                        (
                                            &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                            Some(
                                                &wiggle::tracing::field::display(&conn_param) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                let ret = Rdma::rdma_connect(ctx, id, conn_param);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["result"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::debug(&ret) as &dyn Value),
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
    pub fn rdma_disconnect<'a>(
        ctx: &'a mut (impl Rdma),
        memory: &dyn wiggle::GuestMemory,
        arg0: i32,
    ) -> wiggle::anyhow::Result<()> {
        use std::convert::TryFrom as _;
        let _span = {
            use ::tracing::__macro_support::Callsite as _;
            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                static META: ::tracing::Metadata<'static> = {
                    ::tracing_core::metadata::Metadata::new(
                        "wiggle abi",
                        "untitled4::rdma",
                        wiggle::tracing::Level::TRACE,
                        Some("src/lib.rs"),
                        Some(6u32),
                        Some("untitled4::rdma"),
                        ::tracing_core::field::FieldSet::new(
                            &["module", "function"],
                            ::tracing_core::callsite::Identifier(&CALLSITE),
                        ),
                        ::tracing::metadata::Kind::SPAN,
                    )
                };
                ::tracing::callsite::DefaultCallsite::new(&META)
            };
            let mut interest = ::tracing::subscriber::Interest::never();
            if wiggle::tracing::Level::TRACE
                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                && wiggle::tracing::Level::TRACE
                    <= ::tracing::level_filters::LevelFilter::current()
                && {
                    interest = CALLSITE.interest();
                    !interest.is_never()
                }
                && ::tracing::__macro_support::__is_enabled(
                    CALLSITE.metadata(),
                    interest,
                )
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
                                        Some(&"rdma" as &dyn Value),
                                    ),
                                    (
                                        &iter.next().expect("FieldSet corrupted (this is a bug)"),
                                        Some(&"rdma_disconnect" as &dyn Value),
                                    ),
                                ],
                            )
                    },
                )
            } else {
                let span = ::tracing::__macro_support::__disabled_span(
                    CALLSITE.metadata(),
                );
                {};
                span
            }
        };
        _span
            .in_scope(|| {
                let id = Id::from(arg0);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["id"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::display(&id) as &dyn Value),
                                        ),
                                    ],
                                )
                        });
                    } else {
                    }
                };
                let ret = Rdma::rdma_disconnect(ctx, id);
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event src/lib.rs:6",
                                "untitled4::rdma",
                                wiggle::tracing::Level::TRACE,
                                Some("src/lib.rs"),
                                Some(6u32),
                                Some("untitled4::rdma"),
                                ::tracing_core::field::FieldSet::new(
                                    &["result"],
                                    ::tracing_core::callsite::Identifier(&CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = wiggle::tracing::Level::TRACE
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && wiggle::tracing::Level::TRACE
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
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
                                            Some(&wiggle::tracing::field::debug(&ret) as &dyn Value),
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
    pub trait Rdma {
        fn rdma_getaddrinfo<'a>(
            &mut self,
            node: &wiggle::GuestPtr<'a, str>,
            service: &wiggle::GuestPtr<'a, str>,
            hints: &RdmaAddrinfoStruct,
        ) -> Result<RdmaAddrinfo, RdmaError>;
        fn rdma_create_ep(
            &mut self,
            id: Id,
            res: RdmaAddrinfo,
            pd: EpPd,
            qp_init_attr: &IbvQpInitAttr,
        ) -> Result<(), RdmaError>;
        fn rdma_freeaddrinfo(&mut self, res: RdmaAddrinfo) -> Result<(), RdmaError>;
        fn rdma_reg_msgs<'a>(
            &mut self,
            id: Id,
            addr: &wiggle::GuestPtr<'a, u8>,
        ) -> Result<IbvMr, RdmaError>;
        fn rdma_dereg_mr(&mut self, mr: IbvMr) -> Result<(), RdmaError>;
        fn rdma_post_send<'a>(
            &mut self,
            id: Id,
            context: Context,
            addr: &wiggle::GuestPtr<'a, u8>,
            send_mr: IbvMr,
            flags: u32,
        ) -> ();
        fn rdma_destroy_ep(&mut self, id: Id) -> Result<(), RdmaError>;
        fn rdma_post_recv<'a>(
            &mut self,
            id: Id,
            context: Context,
            addr: &wiggle::GuestPtr<'a, u8>,
            recv_mr: IbvMr,
        ) -> ();
        fn rdma_get_send_comp(&mut self, id: Id, wc: IbvWc) -> Result<IbvWc, RdmaError>;
        fn rdma_get_recv_comp(&mut self, id: Id, wc: IbvWc) -> Result<IbvWc, RdmaError>;
        fn rdma_connect(&mut self, id: Id, conn_param: ConnParam) -> ();
        fn rdma_disconnect(&mut self, id: Id) -> ();
    }
    /// Adds all instance items to the specified `Linker`.
    pub fn add_to_linker<T, U>(
        linker: &mut wiggle::wasmtime_crate::Linker<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> wiggle::anyhow::Result<()>
    where
        U: Rdma,
    {
        linker
            .func_wrap(
                "rdma",
                "rdma_getaddrinfo",
                move |
                    mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                    arg0: i32,
                    arg1: i32,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
                    arg5: i32,
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
                            rdma_getaddrinfo(
                                ctx,
                                &mem,
                                arg0,
                                arg1,
                                arg2,
                                arg3,
                                arg4,
                                arg5,
                            )?,
                        ),
                    )
                },
            )?;
        linker
            .func_wrap(
                "rdma",
                "rdma_create_ep",
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
                    Ok(<i32>::from(rdma_create_ep(ctx, &mem, arg0, arg1, arg2, arg3)?))
                },
            )?;
        linker
            .func_wrap(
                "rdma",
                "rdma_freeaddrinfo",
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
                    Ok(<i32>::from(rdma_freeaddrinfo(ctx, &mem, arg0)?))
                },
            )?;
        linker
            .func_wrap(
                "rdma",
                "rdma_reg_msgs",
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
                    Ok(<i32>::from(rdma_reg_msgs(ctx, &mem, arg0, arg1, arg2)?))
                },
            )?;
        linker
            .func_wrap(
                "rdma",
                "rdma_dereg_mr",
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
                    Ok(<i32>::from(rdma_dereg_mr(ctx, &mem, arg0)?))
                },
            )?;
        linker
            .func_wrap(
                "rdma",
                "rdma_post_send",
                move |
                    mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                    arg0: i32,
                    arg1: i32,
                    arg2: i32,
                    arg3: i32,
                    arg4: i32,
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
                    Ok(
                        <()>::from(
                            rdma_post_send(ctx, &mem, arg0, arg1, arg2, arg3, arg4)?,
                        ),
                    )
                },
            )?;
        linker
            .func_wrap(
                "rdma",
                "rdma_destroy_ep",
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
                    Ok(<i32>::from(rdma_destroy_ep(ctx, &mem, arg0)?))
                },
            )?;
        linker
            .func_wrap(
                "rdma",
                "rdma_post_recv",
                move |
                    mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                    arg0: i32,
                    arg1: i32,
                    arg2: i32,
                    arg3: i32,
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
                    Ok(<()>::from(rdma_post_recv(ctx, &mem, arg0, arg1, arg2, arg3)?))
                },
            )?;
        linker
            .func_wrap(
                "rdma",
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
                "rdma",
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
                "rdma",
                "rdma_connect",
                move |
                    mut caller: wiggle::wasmtime_crate::Caller<'_, T>,
                    arg0: i32,
                    arg1: i32,
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
                    Ok(<()>::from(rdma_connect(ctx, &mem, arg0, arg1)?))
                },
            )?;
        linker
            .func_wrap(
                "rdma",
                "rdma_disconnect",
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
                    Ok(<()>::from(rdma_disconnect(ctx, &mem, arg0)?))
                },
            )?;
        Ok(())
    }
}
pub mod metadata {
    pub const DOC_TEXT: &str = "(typename $rdma_error (enum (@witx tag u16) $success $runtime_error $io_error))\n(typename $ai_flags (enum (@witx tag u8) $RAI_PASSIVE $RAI_NUMERICHOST $RAI_NOROUTE $RAI_FAMILY))\n(typename $ai_port_space (enum (@witx tag u8) $RDMA_PS_UDP $RDMA_PS_TCP $RDMA_PS_IB))\n(typename $ai_famliy (enum (@witx tag u8) $AF_IB $AF_INET $AF_INET6))\n(typename $ai_qp_type (enum (@witx tag u8) $IBV_UD $IBV_RC))\n(typename $ibv_qp_type (enum (@witx tag u8) $IBV_QPT_RC $IBV_QPT_UC $IBV_QPT_UD))\n(typename $id (handle))\n(typename $ibv_mr (handle))\n(typename $ibv_wc (handle))\n(typename $rdma_addrinfo (handle))\n(typename $context (handle))\n(typename $conn_param (handle))\n(typename $rdma_addrinfo_struct (record (field $flags $ai_flags) (field $port_space $ai_port_space) (field $family $ai_famliy) (field $qp_type $ai_qp_type) (field $src_len u32) (field $dst_len u32)))\n(typename $ibv_qp_cap (record (field $max_send_wr u32) (field $max_recv_wr u32) (field $max_send_sge u32) (field $max_recv_sge u32) (field $max_inline_data u32)))\n(typename $rdma_cq (handle))\n(typename $ep_pd (handle))\n(typename $ibv_qp_init_attr (record (field $qp_context $id) (field $send_cq $rdma_cq) (field $recv_cq $rdma_cq) (;; (field $srq (handle))\n ;) (field $cap $ibv_qp_cap) (field $qp_type $ibv_qp_type) (field $sq_sig_all u8)))\n(module $rdma (import \"memory\" (memory)) (@interface func (export \"rdma_getaddrinfo\") (param $node (list char)) (param $service (list char)) (param $hints $rdma_addrinfo_struct) (result $error (variant (@witx tag u32) (case $ok $rdma_addrinfo) (case $err $rdma_error)))) (@interface func (export \"rdma_create_ep\") (param $id $id) (param $res $rdma_addrinfo) (param $pd $ep_pd) (param $qp_init_attr $ibv_qp_init_attr) (result $error (variant (@witx tag u32) (case $ok) (case $err $rdma_error)))) (@interface func (export \"rdma_freeaddrinfo\") (param $res $rdma_addrinfo) (result $error (variant (@witx tag u32) (case $ok) (case $err $rdma_error)))) (@interface func (export \"rdma_reg_msgs\") (param $id $id) (;; Register the memory region for the message buffer\n ;) (param $addr (@witx pointer u8)) (result $error (variant (@witx tag u32) (case $ok $ibv_mr) (case $err $rdma_error)))) (@interface func (export \"rdma_dereg_mr\") (param $mr $ibv_mr) (result $error (variant (@witx tag u32) (case $ok) (case $err $rdma_error)))) (@interface func (export \"rdma_post_send\") (param $id $id) (param $context $context) (param $addr (@witx pointer u8)) (param $send_mr $ibv_mr) (param $flags u32)) (@interface func (export \"rdma_destroy_ep\") (param $id $id) (result $error (variant (@witx tag u32) (case $ok) (case $err $rdma_error)))) (@interface func (export \"rdma_post_recv\") (param $id $id) (param $context $context) (param $addr (@witx pointer u8)) (param $recv_mr $ibv_mr)) (@interface func (export \"rdma_get_send_comp\") (param $id $id) (param $wc $ibv_wc) (result $error (variant (@witx tag u32) (case $ok $ibv_wc) (case $err $rdma_error)))) (@interface func (export \"rdma_get_recv_comp\") (param $id $id) (param $wc $ibv_wc) (result $error (variant (@witx tag u32) (case $ok $ibv_wc) (case $err $rdma_error)))) (@interface func (export \"rdma_connect\") (param $id $id) (param $conn_param $conn_param)) (@interface func (export \"rdma_disconnect\") (param $id $id)))\n";
    pub fn document() -> wiggle::witx::Document {
        wiggle::witx::parse(DOC_TEXT).unwrap()
    }
}
impl Rdma for WiggleCtx {}
fn main() {}
