// Copyright 2017 Avraham Weinstock
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::ffi::c_void;
use std::io::Read;
use std::io::Write;
use std::process::Stdio;

use wl_clipboard_rs::paste;

use crate::common::{ClipboardProvider, Result};

pub struct WaylandClipboardContext {
    clipboard: paste::ClipboardType,
}

/// Create new clipboard from a raw display pointer.
///
/// # Safety
///
/// Since the type of the display is a raw pointer, it's the responsibility of the callee to make
/// sure that the passed pointer is a valid Wayland display.
pub unsafe fn create_clipboards_from_external(_display: *mut c_void) -> (WaylandClipboardContext, WaylandClipboardContext) {
    (
        WaylandClipboardContext { clipboard: paste::ClipboardType::Primary },
        WaylandClipboardContext { clipboard: paste::ClipboardType::Regular }
    )
}

impl ClipboardProvider for WaylandClipboardContext   {
    fn get_contents(&mut self) -> Result<String> {
        let result = paste::get_contents(self.clipboard, paste::Seat::Unspecified, paste::MimeType::Text);
        match result {
            Ok((mut pipe, _)) => {
                let mut contents = String::new();
                pipe.read_to_string(&mut contents)?;
                Ok(contents)
            }
            Err(paste::Error::NoSeats) | Err(paste::Error::ClipboardEmpty) | Err(paste::Error::NoMimeType) => {
                Ok("".to_owned())
            }
            // TODO: show error
            Err(_) => Err("get_contents returned error".into())
        }
    }

    fn set_contents(&mut self, data: String) -> Result<()> {
        let mut command = std::process::Command::new("wl-copy");
        if self.clipboard == paste::ClipboardType::Primary {
            command.arg("--primary");
        }
        command.stdin(Stdio::piped());

        let mut child = command.spawn()?;
        child.stdin.take().unwrap().write_all(data.as_bytes())?;
        child.wait()?;
        Ok(())
    }
}
