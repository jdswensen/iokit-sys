// exports from <IOKit/i2c/IOI2CInterface.h>

use libc::{c_char, c_int};
use types::{IOOptionBits, io_service_t};
use io_return::IOReturn;
use mach::vm_types::vm_address_t;

pub type IOI2CRequestRef = *mut IOI2CRequest;
pub type IOI2CRequestCompletion = extern fn(request: IOI2CRequestRef);

pub const kIOI2CUseSubAddressCommFlag: c_int = 0x0000_0002;


#[doc(hidden)]
#[allow(dead_code)]
#[repr(C, align(4))] // Is this the correct way to deal with byte alignment?
pub struct IOI2CRequest {
    send_transaction_type: IOOptionBits,
    reply_transaction_type: IOOptionBits,
    send_address: u32,
    reply_address: u32,
    send_sub_address: u8,
    reply_sub_address: u8,
    __reserved_a: [u8; 2],
    min_reply_delay: u64,
    result: IOReturn,
    comm_flags: IOOptionBits,

    #[cfg(target_pointer_width = "64")]
    __pad_a: u32,

    #[cfg(target_pointer_width = "32")]
    send_buffer: vm_address_t,

    send_bytes: u32,
    __reserved_b: [u32; 2],

    #[cfg(target_pointer_width = "64")]
    __pad_b: u32,

    #[cfg(target_pointer_width = "32")]
    reply_buffer: vm_address_t,

    reply_bytes: u32,
    completion: IOI2CRequestCompletion,

    #[cfg(target_pointer_width = "32")]
    __pad_c: [u32; 5],

    #[cfg(target_pointer_width = "64")]
    send_buffer: vm_address_t,

    #[cfg(target_pointer_width = "64")]
    reply_buffer: vm_address_t,


    __reserved_c: [u32; 10],
}

pub fn kIOI2CSupportedCommFlagsKey() -> *const c_char {
    b"IOI2CSupportedCommFlags\0".as_ptr() as *const c_char
}


// Opaque, see: https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
#[repr(C)] pub struct IOI2CConnect { _private: [u8; 0] }
pub type IOI2CConnectRef = *mut IOI2CConnect;

extern "C" {
    pub fn IOI2CInterfaceOpen(interface: io_service_t, options: IOOptionBits, connect: IOI2CConnectRef) -> IOReturn;
    pub fn IOI2CInterfaceClose(connect: IOI2CConnectRef, options: IOOptionBits) -> IOReturn;
    pub fn IOI2CSendRequest(connect: IOI2CConnectRef, options: IOOptionBits, request: IOI2CRequestRef) -> IOReturn;
}