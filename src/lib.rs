mod functions;
mod notifications;
pub use functions::*;
pub use notifications::*;

use jack::*;

pub fn as_slice_mut<'a>(p: &'a Port<AudioIn>, ps: &'a ProcessScope) -> &'a mut [f32] {
    assert_eq!(p.client_ptr(), ps.client_ptr());
    unsafe {
        std::slice::from_raw_parts_mut(p.buffer(ps.n_frames()) as *mut f32, ps.n_frames() as usize)
    }
}
