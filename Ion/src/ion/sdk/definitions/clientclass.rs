use std::os::raw::c_char;

use crate::ion::sdk::definitions::recvprop;

type CreateClientClassFn = unsafe extern "system" fn(ent: i32, serial: i32);
type CreateEventFn = unsafe extern "system" fn();

#[derive(Clone)]
#[repr(C)]
pub struct ClientClass {
    create_client_class: CreateClientClassFn,
    create_event: CreateEventFn,
    network_name: *mut c_char,
    pub recv_table: *mut recvprop::CRecvTable,
    pub next: *mut usize,
    pub class_id: i32,
}