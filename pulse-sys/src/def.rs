// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language linking library.
//
// This library is free software; you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation; either version
// 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this library;
// if not, see <http://www.gnu.org/licenses/>.

//! Global definitions

use std::os::raw::c_void;
use libc::timeval;
use crate::sample::pa_usec_t;

/// An invalid index
pub const PA_INVALID_INDEX: u32 = std::u32::MAX;

pub type pa_free_cb_t = Option<extern "C" fn(p: *mut c_void)>;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum pa_device_type_t {
    Sink,
    Source,
}

pub const PA_DEVICE_TYPE_SINK: pa_device_type_t = pa_device_type_t::Sink;
pub const PA_DEVICE_TYPE_SOURCE: pa_device_type_t = pa_device_type_t::Source;

#[repr(C)]
pub struct pa_buffer_attr {
    pub maxlength: u32,
    pub tlength: u32,
    pub prebuf: u32,
    pub minreq: u32,
    pub fragsize: u32,
}

#[repr(C)]
pub struct pa_timing_info {
    pub timestamp: timeval,
    pub synchronized_clocks: i32,
    pub sink_usec: pa_usec_t,
    pub source_usec: pa_usec_t,
    pub transport_usec: pa_usec_t,
    pub playing: i32,
    pub write_index_corrupt: i32,
    pub write_index: i64,
    pub read_index_corrupt: i32,
    pub read_index: i64,
    pub configured_sink_usec: pa_usec_t,
    pub configured_source_usec: pa_usec_t,
    pub since_underrun: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pa_spawn_api {
    pub prefork: Option<extern "C" fn()>,
    pub postfork: Option<extern "C" fn()>,
    pub atfork: Option<extern "C" fn()>,
}

pub type pa_sink_flags_t = u32;

pub use self::sink_flags::*;

/// Special sink flags.
pub mod sink_flags {
    use super::pa_sink_flags_t;

    pub const PA_SINK_NOFLAGS: pa_sink_flags_t = 0x0;
    pub const PA_SINK_HW_VOLUME_CTRL: pa_sink_flags_t = 0x1;
    pub const PA_SINK_LATENCY: pa_sink_flags_t = 0x2;
    pub const PA_SINK_HARDWARE: pa_sink_flags_t = 0x4;
    pub const PA_SINK_NETWORK: pa_sink_flags_t = 0x8;
    pub const PA_SINK_HW_MUTE_CTRL: pa_sink_flags_t = 0x10;
    pub const PA_SINK_DECIBEL_VOLUME: pa_sink_flags_t = 0x20;
    pub const PA_SINK_FLAT_VOLUME: pa_sink_flags_t = 0x40;
    pub const PA_SINK_DYNAMIC_LATENCY: pa_sink_flags_t = 0x80;
    pub const PA_SINK_SET_FORMATS: pa_sink_flags_t = 0x100;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum pa_sink_state_t {
    Invalid = -1,
    Running = 0,
    Idle = 1,
    Suspended = 2,
}

pub const PA_SINK_INVALID_STATE: pa_sink_state_t = pa_sink_state_t::Invalid;
pub const PA_SINK_RUNNING: pa_sink_state_t = pa_sink_state_t::Running;
pub const PA_SINK_IDLE: pa_sink_state_t = pa_sink_state_t::Idle;
pub const PA_SINK_SUSPENDED: pa_sink_state_t = pa_sink_state_t::Suspended;

/// Returns `true` if sink is playing: running or idle.
#[inline(always)]
pub fn pa_sink_is_opened(state: pa_sink_state_t) -> bool {
    state == pa_sink_state_t::Running ||
    state == pa_sink_state_t::Idle
}

/// Returns `true` if sink is running.
#[inline(always)]
pub fn pa_sink_is_running(state: pa_sink_state_t) -> bool {
    state == pa_sink_state_t::Running
}

pub type pa_source_flags_t = u32;

pub use self::source_flags::*;

/// Special source flags.
pub mod source_flags {
    use super::pa_source_flags_t;

    pub const PA_SOURCE_NOFLAGS: pa_source_flags_t = 0x0;
    pub const PA_SOURCE_HW_VOLUME_CTRL: pa_source_flags_t = 0x1;
    pub const PA_SOURCE_LATENCY: pa_source_flags_t = 0x2;
    pub const PA_SOURCE_HARDWARE: pa_source_flags_t = 0x4;
    pub const PA_SOURCE_NETWORK: pa_source_flags_t = 0x8;
    pub const PA_SOURCE_HW_MUTE_CTRL: pa_source_flags_t = 0x10;
    pub const PA_SOURCE_DECIBEL_VOLUME: pa_source_flags_t = 0x20;
    pub const PA_SOURCE_DYNAMIC_LATENCY: pa_source_flags_t = 0x42;
    pub const PA_SOURCE_FLAT_VOLUME: pa_source_flags_t = 0x80;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum pa_source_state_t {
    Invalid = -1,
    Running = 0,
    Idle = 1,
    Suspended = 2,
}

pub const PA_SOURCE_INVALID_STATE: pa_source_state_t = pa_source_state_t::Invalid;
pub const PA_SOURCE_RUNNING: pa_source_state_t = pa_source_state_t::Running;
pub const PA_SOURCE_IDLE: pa_source_state_t = pa_source_state_t::Idle;
pub const PA_SOURCE_SUSPENDED: pa_source_state_t = pa_source_state_t::Suspended;

#[inline(always)]
pub fn pa_source_is_opened(state: pa_source_state_t) -> bool {
    state == pa_source_state_t::Running ||
    state == pa_source_state_t::Idle
}

#[inline(always)]
pub fn pa_source_is_running(state: pa_source_state_t) -> bool {
    state == pa_source_state_t::Running
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum pa_port_available_t {
    /// This port does not support jack detection
    Unknown = 0,
    /// This port is not available, likely because the jack is not plugged in.
    No = 1,
    /// This port is available, likely because the jack is plugged in.
    Yes = 2,
}

pub const PA_PORT_AVAILABLE_UNKNOWN: pa_port_available_t = pa_port_available_t::Unknown;
pub const PA_PORT_AVAILABLE_NO: pa_port_available_t = pa_port_available_t::No;
pub const PA_PORT_AVAILABLE_YES: pa_port_available_t = pa_port_available_t::Yes;

