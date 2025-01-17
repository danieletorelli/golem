// Generated by `wit-bindgen` 0.21.0. DO NOT EDIT!
// Options used:
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_run_cabi<T: Guest>() -> *mut u8 {
    let result0 = T::run();
    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    let (t2_0, t2_1, t2_2, t2_3, t2_4) = result0;
    match t2_0 {
        Some(e) => {
            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
            let vec3 = (e.into_bytes()).into_boxed_slice();
            let ptr3 = vec3.as_ptr().cast::<u8>();
            let len3 = vec3.len();
            ::core::mem::forget(vec3);
            *ptr1.add(8).cast::<usize>() = len3;
            *ptr1.add(4).cast::<*mut u8>() = ptr3.cast_mut();
        }
        None => {
            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
        }
    };
    match t2_1 {
        Some(e) => {
            *ptr1.add(12).cast::<u8>() = (1i32) as u8;
            let vec4 = (e.into_bytes()).into_boxed_slice();
            let ptr4 = vec4.as_ptr().cast::<u8>();
            let len4 = vec4.len();
            ::core::mem::forget(vec4);
            *ptr1.add(20).cast::<usize>() = len4;
            *ptr1.add(16).cast::<*mut u8>() = ptr4.cast_mut();
        }
        None => {
            *ptr1.add(12).cast::<u8>() = (0i32) as u8;
        }
    };
    match t2_2 {
        Some(e) => {
            *ptr1.add(24).cast::<u8>() = (1i32) as u8;
            let vec5 = (e.into_bytes()).into_boxed_slice();
            let ptr5 = vec5.as_ptr().cast::<u8>();
            let len5 = vec5.len();
            ::core::mem::forget(vec5);
            *ptr1.add(32).cast::<usize>() = len5;
            *ptr1.add(28).cast::<*mut u8>() = ptr5.cast_mut();
        }
        None => {
            *ptr1.add(24).cast::<u8>() = (0i32) as u8;
        }
    };
    match t2_3 {
        Some(e) => {
            *ptr1.add(36).cast::<u8>() = (1i32) as u8;
            let vec6 = (e.into_bytes()).into_boxed_slice();
            let ptr6 = vec6.as_ptr().cast::<u8>();
            let len6 = vec6.len();
            ::core::mem::forget(vec6);
            *ptr1.add(44).cast::<usize>() = len6;
            *ptr1.add(40).cast::<*mut u8>() = ptr6.cast_mut();
        }
        None => {
            *ptr1.add(36).cast::<u8>() = (0i32) as u8;
        }
    };
    match t2_4 {
        Some(e) => {
            *ptr1.add(48).cast::<u8>() = (1i32) as u8;
            let vec7 = (e.into_bytes()).into_boxed_slice();
            let ptr7 = vec7.as_ptr().cast::<u8>();
            let len7 = vec7.len();
            ::core::mem::forget(vec7);
            *ptr1.add(56).cast::<usize>() = len7;
            *ptr1.add(52).cast::<*mut u8>() = ptr7.cast_mut();
        }
        None => {
            *ptr1.add(48).cast::<u8>() = (0i32) as u8;
        }
    };
    ptr1
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_run<T: Guest>(arg0: *mut u8) {
    let l0 = i32::from(*arg0.add(0).cast::<u8>());
    match l0 {
        0 => (),
        _ => {
            let l1 = *arg0.add(4).cast::<*mut u8>();
            let l2 = *arg0.add(8).cast::<usize>();
            _rt::cabi_dealloc(l1, l2, 1);
        }
    }
    let l3 = i32::from(*arg0.add(12).cast::<u8>());
    match l3 {
        0 => (),
        _ => {
            let l4 = *arg0.add(16).cast::<*mut u8>();
            let l5 = *arg0.add(20).cast::<usize>();
            _rt::cabi_dealloc(l4, l5, 1);
        }
    }
    let l6 = i32::from(*arg0.add(24).cast::<u8>());
    match l6 {
        0 => (),
        _ => {
            let l7 = *arg0.add(28).cast::<*mut u8>();
            let l8 = *arg0.add(32).cast::<usize>();
            _rt::cabi_dealloc(l7, l8, 1);
        }
    }
    let l9 = i32::from(*arg0.add(36).cast::<u8>());
    match l9 {
        0 => (),
        _ => {
            let l10 = *arg0.add(40).cast::<*mut u8>();
            let l11 = *arg0.add(44).cast::<usize>();
            _rt::cabi_dealloc(l10, l11, 1);
        }
    }
    let l12 = i32::from(*arg0.add(48).cast::<u8>());
    match l12 {
        0 => (),
        _ => {
            let l13 = *arg0.add(52).cast::<*mut u8>();
            let l14 = *arg0.add(56).cast::<usize>();
            _rt::cabi_dealloc(l13, l14, 1);
        }
    }
}
pub trait Guest {
    fn run() -> (
        Option<_rt::String>,
        Option<_rt::String>,
        Option<_rt::String>,
        Option<_rt::String>,
        Option<_rt::String>,
    );
}
#[doc(hidden)]

macro_rules! __export_world_file_write_read_delete_cabi{
  ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

    #[export_name = "run"]
    unsafe extern "C" fn export_run() -> *mut u8 {
      $($path_to_types)*::_export_run_cabi::<$ty>()
    }
    #[export_name = "cabi_post_run"]
    unsafe extern "C" fn _post_return_run(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_run::<$ty>(arg0)
    }
  };);
}
#[doc(hidden)]
pub(crate) use __export_world_file_write_read_delete_cabi;
#[repr(align(4))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 60]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 60]);
mod _rt {
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }
    pub use alloc_crate::alloc;
    pub use alloc_crate::string::String;
    extern crate alloc as alloc_crate;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_file_write_read_delete_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::__export_world_file_write_read_delete_cabi!($ty with_types_in $($path_to_types_root)*);
  )
}
#[doc(inline)]
pub(crate) use __export_file_write_read_delete_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.21.0:file-write-read-delete:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 206] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07B\x01A\x02\x01A\x04\x01\
ks\x01o\x05\0\0\0\0\0\x01@\0\0\x01\x04\0\x03run\x01\x02\x04\x01\x1fgolem:it/file\
-write-read-delete\x04\0\x0b\x1c\x01\0\x16file-write-read-delete\x03\0\0\0G\x09p\
roducers\x01\x0cprocessed-by\x02\x0dwit-component\x070.201.0\x10wit-bindgen-rust\
\x060.21.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
