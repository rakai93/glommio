use std::{io, ptr::NonNull};

/// A probe of the operations supported by this kernel version's io-uring
/// interface.
#[derive(Debug)]
pub struct Probe {
    probe: NonNull<liburing::io_uring_probe>,
}

impl Probe {
    pub fn new() -> io::Result<Probe> {
        unsafe {
            let probe = liburing::io_uring_get_probe();
            NonNull::new(probe)
                .ok_or_else(io::Error::last_os_error)
                .map(|probe| Probe { probe })
        }
    }

    pub(crate) fn for_ring(ring: *mut liburing::io_uring) -> io::Result<Probe> {
        unsafe {
            let probe = liburing::io_uring_get_probe_ring(ring);
            NonNull::new(probe)
                .ok_or_else(io::Error::last_os_error)
                .map(|probe| Probe { probe })
        }
    }

    pub fn supports(&self, op: liburing::io_uring_op) -> bool {
        unsafe { liburing::io_uring_opcode_supported(self.probe.as_ptr(), op as _) != 0 }
    }
}

impl Drop for Probe {
    fn drop(&mut self) {
        unsafe { libc::free(self.probe.as_ptr() as *mut _) }
    }
}
