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

use wl_clipboard_rs::{copy, paste};

use crate::common::{ClipboardProvider, Result};

pub struct Clipboard {
}

pub struct Primary {
}

/// Create new clipboard from a raw display pointer.
///
/// # Safety
///
/// Since the type of the display is a raw pointer, it's the responsibility of the callee to make
/// sure that the passed pointer is a valid Wayland display.
pub unsafe fn create_clipboards_from_external(display: *mut c_void) -> (Primary, Clipboard) {
    (Primary {}, Clipboard {})
}

impl ClipboardProvider for Clipboard {
    fn get_contents(&mut self) -> Result<String> {
        let result = paste::get_contents(paste::ClipboardType::Regular, paste::Seat::Unspecified, paste::MimeType::Text);
        match result {
            Ok((mut pipe, _)) => {
                let mut contents = String::new();
                pipe.read_to_string(&mut contents)?;
                Ok(contents)
            }
            Err(paste::Error::NoSeats) | Err(paste::Error::ClipboardEmpty) | Err(paste::Error::NoMimeType) => {
                Ok("".to_owned())
            }
            Err(err) => Err("get_contents returned error".into())
        }
    }

    fn set_contents(&mut self, data: String) -> Result<()> {
        let mut opts = copy::Options::new();
        opts.clipboard(copy::ClipboardType::Regular);
        opts.copy(copy::Source::Bytes(data.into_bytes().into()), copy::MimeType::Text);

        Ok(())
    }
}

impl ClipboardProvider for Primary {
    fn get_contents(&mut self) -> Result<String> {
        Ok("".to_owned())
    }

    fn set_contents(&mut self, data: String) -> Result<()> {
        Ok(())
    }
}
