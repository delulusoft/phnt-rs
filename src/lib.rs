// MIT License
//
// Copyright (c) 2024 oberrich <oberrich.llvm@proton.me>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![allow(
   warnings,
   unused,
   non_snake_case,
   non_camel_case_types,
   non_upper_case_globals
)]

/// Bindings for `phnt` (nightly) generated by `bindgen`
pub mod ffi {
   // use vendored bindings (only available for `x86_64` arch)
   #[cfg_attr(docsrs, doc(cfg(not(feature = "regenerate"))))]
   #[cfg(all(not(feature = "regenerate"), target_arch = "x86"))]
   include!("ffi/i686_bindgen.rs");
   #[cfg_attr(docsrs, doc(cfg(not(feature = "regenerate"))))]
   #[cfg(all(not(feature = "regenerate"), target_arch = "x86_64"))]
   include!("ffi/x86_64_bindgen.rs");
   #[cfg_attr(docsrs, doc(cfg(not(feature = "regenerate"))))]
   #[cfg(all(not(feature = "regenerate"), target_arch = "aarch64"))]
   include!("ffi/aarch64_bindgen.rs");

   // use re-generated bindings
   #[cfg_attr(docsrs, doc(cfg(feature = "regenerate")))]
   #[cfg(all(feature = "regenerate", target_arch = "x86"))]
   include!("ffi/i686_bindgen.rs");
   #[cfg_attr(docsrs, doc(cfg(feature = "regenerate")))]
   #[cfg(all(feature = "regenerate", target_arch = "x86_64"))]
   include!("ffi/x86_64_bindgen.rs");
   #[cfg_attr(docsrs, doc(cfg(feature = "regenerate")))]
   #[cfg(all(feature = "regenerate", target_arch = "aarch64"))]
   include!("ffi/aarch64_bindgen.rs");
}

/// Extensions to the bindings (useful functions, macros, etc.)
pub mod ext {
   use crate::ffi::*;
   use core::{arch::asm, mem, ptr};

   #[inline]
   #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
   pub unsafe fn __readfsdword(offset: u32) -> usize {
      let out: usize;
      asm!(
          "mov {}, fs:[{}]",
          lateout(reg) out, in(reg) offset,
          options(nostack, pure, readonly),
      );
      out
   }

   #[inline]
   #[cfg(target_arch = "x86_64")]
   pub unsafe fn __readgsqword(offset: u32) -> usize {
      let out: usize;
      asm!(
          "mov {}, gs:[{}]",
          lateout(reg) out, in(reg) offset,
          options(nostack, pure, readonly),
      );
      out
   }

   #[inline]
   #[cfg(target_arch = "aarch64")]
   pub unsafe fn __readgsqword(offset: u32) -> usize {
      0usize
   }

   #[inline]
   pub unsafe fn NtCurrentTeb() -> *mut TEB {
      const TEB_OFFSET: u32 = mem::offset_of!(NT_TIB, Self_) as u32;
      if cfg!(target_arch = "x86_64") {
         __readgsqword(TEB_OFFSET) as *mut TEB
      } else if cfg!(target_arch = "x86") {
         __readfsdword(TEB_OFFSET) as *mut TEB
      } else if cfg!(target_arch = "aarch64") {
         let p: *mut TEB = core::ptr::null_mut();
         p
      } else {
         unimplemented!("target architecture not implemented yet")
      }
   }

   #[cfg(test)]
   mod tests {
      use super::*;
      use windows_sys::Win32::System::Threading::GetCurrentThreadId;

      #[test]
      fn test_teb() {
         let cur_thread = unsafe { (*NtCurrentTeb()).ClientId.UniqueThread as isize };
         let cur_thread_sys = unsafe { GetCurrentThreadId() as isize };
         assert_eq!(cur_thread, cur_thread_sys);
      }
   }
}
