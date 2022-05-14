use anyhow::Result;
use std::ffi::CString;

use llvm_sys::{
    core::{LLVMAddFunction, LLVMGetParam},
    prelude::LLVMValueRef,
};

use crate::{basic_block::LLBasicBlock, context::LLContext};

use super::{module::LLModule, types::LLFunctionType};

//------------------------------------------------------------------------------
// Type Definitions
//------------------------------------------------------------------------------

/// For ypes that are LLVMValueRef.
pub trait LLValue {
    unsafe fn value_ref(&self) -> LLVMValueRef;
}

/// LLVM Function wrapper.
///
/// # Safety
/// It is unsafe to use the reference of `LLFunctionType` because its params can be independently freed.
/// Holding an `Rc` to it ensures that that does not happen.
///
/// WARNING: This is safe only if we can only create a Function from a Module.
///
/// # Ownership
/// - Owned by an LLVM Module.
/// - Owns the basic blocks and arguments added to it.
///
/// - https://llvm.org/doxygen/Function_8cpp_source.html#l00409
/// - https://llvm.org/doxygen/Function_8cpp_source.html#l00509
#[derive(Debug)]
pub struct LLFunction(LLVMValueRef);

/// LLVM Param wrapper.
///
/// TODO(appcypher): Add note on safety and ownership.
pub struct LLParam(LLVMValueRef);

/// A wrapper for LLVM `store` instruction.
///
/// TODO(appcypher): Add note on safety and ownership.
pub struct LLStore(LLVMValueRef);

/// A wrapper for LLVM `ret` instruction.
///
/// TODO(appcypher): Add note on safety and ownership.
pub struct LLUnreachable(LLVMValueRef);

/// A wrapper for LLVM `alloca` instruction.
///
/// TODO(appcypher): Add note on safety and ownership.
pub struct LLAlloca(LLVMValueRef);

/// A wrapper for LLVM `ret` instruction.
///
/// TODO(appcypher): Add note on safety and ownership.
pub struct LLRet(LLVMValueRef);

/// A wrapper for LLVM `ret void` instruction.
///
/// TODO(appcypher): Add note on safety and ownership.
pub struct LLRetVoid(LLVMValueRef);

/// A wrapper for LLVM `br` instruction.
///
/// TODO(appcypher): Add note on safety and ownership.
pub struct LLBr(LLVMValueRef);

/// A wrapper for LLVM CondBr instruction.
///
/// TODO(appcypher): Add note on safety and ownership.
pub struct LLCondBr(LLVMValueRef);

//------------------------------------------------------------------------------
// Implementations
//------------------------------------------------------------------------------

impl LLFunction {
    /// Creates a new LLVM Function.
    ///
    /// This is the only way to create an LLFunction, ensuring it has an associated Module.
    /// Therefore a Module must already exist to dispose it.
    ///
    /// # Safety
    /// - Module can only be created from a Context that frees it.
    /// - TODO(appcypher): It is unclear if the LLVM depends on `name` pointer.
    ///
    /// ### References
    /// - https://llvm.org/doxygen/Twine_8h_source.html#l00271
    /// - https://llvm.org/doxygen/Twine_8h_source.html#l00477
    /// - https://llvm.org/doxygen/Value_8cpp_source.html#l00315
    /// - https://llvm.org/doxygen/StringRef_8h_source.html#l00107
    pub(super) fn new(
        name: &str,
        module: &mut LLModule,
        function_type: &LLFunctionType,
    ) -> Result<Self> {
        Ok(Self(unsafe {
            LLVMAddFunction(
                module.as_ptr(),
                CString::new(name)?.as_ptr(),
                function_type.as_ptr(),
            )
        }))
    }

    /// Creates a new LLVM Basic Block.
    pub fn create_basic_block(&self, name: &str, context: &LLContext) -> Result<LLBasicBlock> {
        LLBasicBlock::new(name, self, context)
    }

    /// Gets the param at the given index.
    pub fn get_param(&self, index: u32) -> LLParam {
        LLParam::from_ptr(unsafe { LLVMGetParam(self.as_ptr(), index) })
    }

    pub(crate) unsafe fn as_ptr(&self) -> LLVMValueRef {
        self.0
    }
}

impl LLValue for LLFunction {
    unsafe fn value_ref(&self) -> LLVMValueRef {
        self.0
    }
}

macro_rules! impl_value {
    ($t:ty) => {
        impl $t {
            pub(super) fn from_ptr(ptr: LLVMValueRef) -> Self {
                Self(ptr)
            }

            pub(crate) unsafe fn as_ptr(&self) -> LLVMValueRef {
                self.0
            }
        }

        impl LLValue for $t {
            unsafe fn value_ref(&self) -> LLVMValueRef {
                self.0
            }
        }
    };
}

impl_value!(LLParam);
impl_value!(LLUnreachable);
impl_value!(LLAlloca);
impl_value!(LLStore);
impl_value!(LLRet);
impl_value!(LLRetVoid);
impl_value!(LLBr);
impl_value!(LLCondBr);