use std::os::raw::c_void;

#[repr(C)]
pub struct CGlobalVars {
    pub realtime: f32,
    pub frame_count: i32,
    pub absolute_frame_time: f32,
    pub absolute_frame_start: f32,
    pub cur_time: f32,
    pub frame_time: f32,
    pub max_clients: i32,
    pub tick_count: i32,
    pub interval_per_tick: f32,
    pub interpolation_amount: f32,
    pub sim_ticks: i32,
    pub network_protocol: i32,
    pub save_data: *const c_void,
    pub client: bool,
    pub remote_client: bool,
}