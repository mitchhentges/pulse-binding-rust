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

//! Connection contexts for asynchronous communication with a server.
//! 
//! A `pa_context` object wraps a connection to a PulseAudio server using its native protocol.

pub use ext_device_manager::*;
pub use ext_device_restore::*;
pub use ext_stream_restore::*;
pub use introspect::*;
pub use scache::*;
pub use subscribe::*;

pub mod ext_device_manager;
pub mod ext_device_restore;
pub mod ext_stream_restore;
pub mod introspect;
pub mod scache;
pub mod subscribe;

use std::os::raw::{c_char, c_void};
use crate::mainloop::api::{pa_time_event, pa_time_event_cb_t, pa_mainloop_api};
use crate::{operation::pa_operation, def::pa_spawn_api};
use crate::proplist::{pa_proplist, pa_update_mode_t};
use crate::sample::{pa_usec_t, pa_sample_spec};

/// An opaque connection context to a daemon
pub enum pa_context {}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum pa_context_state_t {
    Unconnected,
    Connecting,
    Authorizing,
    SettingName,
    Ready,
    Failed,
    Terminated,
}

pub const PA_CONTEXT_UNCONNECTED: pa_context_state_t = pa_context_state_t::Unconnected;
pub const PA_CONTEXT_CONNECTING: pa_context_state_t = pa_context_state_t::Connecting;
pub const PA_CONTEXT_AUTHORIZING: pa_context_state_t = pa_context_state_t::Authorizing;
pub const PA_CONTEXT_SETTING_NAME: pa_context_state_t = pa_context_state_t::SettingName;
pub const PA_CONTEXT_READY: pa_context_state_t = pa_context_state_t::Ready;
pub const PA_CONTEXT_FAILED: pa_context_state_t = pa_context_state_t::Failed;
pub const PA_CONTEXT_TERMINATED: pa_context_state_t = pa_context_state_t::Terminated;

/// Returns `true` if the passed state is one of the connected states.
#[inline(always)]
pub fn pa_context_is_good(state: pa_context_state_t) -> bool {
    state == pa_context_state_t::Connecting ||
    state == pa_context_state_t::Authorizing ||
    state == pa_context_state_t::SettingName ||
    state == pa_context_state_t::Ready
}

pub type pa_context_flags_t = u32;

pub use self::flags::*;

/// Some special flags for contexts.
pub mod flags {
    use super::pa_context_flags_t;

    pub const PA_CONTEXT_NOFLAGS: pa_context_flags_t = 0x0;
    pub const PA_CONTEXT_NOAUTOSPAWN: pa_context_flags_t = 0x1;
    pub const PA_CONTEXT_NOFAIL: pa_context_flags_t = 0x2;
}

pub type pa_context_notify_cb_t = Option<extern "C" fn(c: *mut pa_context, userdata: *mut c_void)>;

pub type pa_context_success_cb_t = Option<extern "C" fn(c: *mut pa_context, success: i32, userdata: *mut c_void)>;

pub type pa_context_event_cb_t = Option<extern "C" fn(c: *mut pa_context, name: *const c_char, p: *mut pa_proplist, userdata: *mut c_void)>;

#[link(name="pulse")]
extern "C" {
    pub fn pa_context_new(mainloop: *const pa_mainloop_api, name: *const c_char) -> *mut pa_context;
    pub fn pa_context_new_with_proplist(mainloop: *const pa_mainloop_api, name: *const c_char, proplist: *const pa_proplist) -> *mut pa_context;
    pub fn pa_context_unref(c: *mut pa_context);
    pub fn pa_context_ref(c: *mut pa_context) -> *mut pa_context;
    pub fn pa_context_set_state_callback(c: *mut pa_context, cb: pa_context_notify_cb_t, userdata: *mut c_void);
    pub fn pa_context_set_event_callback(p: *mut pa_context, cb: pa_context_event_cb_t, userdata: *mut c_void);
    pub fn pa_context_errno(c: *const pa_context) -> i32;
    pub fn pa_context_is_pending(c: *const pa_context) -> i32;
    pub fn pa_context_get_state(c: *const pa_context) -> pa_context_state_t;
    pub fn pa_context_connect(c: *mut pa_context, server: *const c_char, flags: pa_context_flags_t, api: *const pa_spawn_api) -> i32;
    pub fn pa_context_disconnect(c: *mut pa_context);
    pub fn pa_context_drain(c: *mut pa_context, cb: pa_context_notify_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_context_exit_daemon(c: *mut pa_context, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_context_set_default_sink(c: *mut pa_context, name: *const c_char, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_context_set_default_source(c: *mut pa_context, name: *const c_char, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_context_is_local(c: *const pa_context) -> i32;
    pub fn pa_context_set_name(c: *mut pa_context, name: *const c_char, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_context_get_server(c: *const pa_context) -> *const c_char;
    pub fn pa_context_get_protocol_version(c: *const pa_context) -> u32;
    pub fn pa_context_get_server_protocol_version(c: *const pa_context) -> u32;
    pub fn pa_context_proplist_update(c: *mut pa_context, mode: pa_update_mode_t, p: *const pa_proplist, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_context_proplist_remove(c: *mut pa_context, keys: *const *const c_char, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut pa_operation;
    pub fn pa_context_get_index(s: *const pa_context) -> u32;
    pub fn pa_context_rttime_new(c: *const pa_context, usec: pa_usec_t, cb: pa_time_event_cb_t, userdata: *mut c_void) -> *mut pa_time_event;
    pub fn pa_context_rttime_restart(c: *const pa_context, e: *mut pa_time_event, usec: pa_usec_t);
    pub fn pa_context_get_tile_size(c: *const pa_context, ss: *const pa_sample_spec) -> usize;
    pub fn pa_context_load_cookie_from_file(c: *mut pa_context, cookie_file_path: *const c_char) -> i32;
}
