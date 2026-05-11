// Local Procedure Call (LPC)

#[derive(Debug)]
pub struct PortObject {
    pub connection_port: *mut PortObject,
    pub server_process: usize,
    pub message_queue: crate::ke::ListEntry,
}

pub fn lpc_create_port(name: &str) -> *mut PortObject {
    core::ptr::null_mut()
}

pub fn lpc_send_request(port: *mut PortObject, message: &[u8]) -> i32 {
    0
}
