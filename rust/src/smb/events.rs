/* Copyright (C) 2018 Open Information Security Foundation
 *
 * You can copy, redistribute or modify this Program under the terms of
 * the GNU General Public License version 2 as published by the Free
 * Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * version 2 along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301, USA.
 */

use crate::core::*;
use crate::smb::smb::*;

#[repr(u32)]
pub enum SMBEvent {
    InternalError = 0,
    MalformedData = 1,
    RecordOverflow = 2,
    MalformedNtlmsspRequest = 3,
    MalformedNtlmsspResponse = 4,
    DuplicateNegotiate = 5,
    NegotiateMalformedDialects = 6,
    FileOverlap = 7,
    RequestToClient = 8,
    ResponseToServer = 9,

    /// Negotiated max sizes exceed our limit
    NegotiateMaxReadSizeTooLarge = 10,
    NegotiateMaxWriteSizeTooLarge = 11,

    /// READ request asking for more than `max_read_size`
    ReadRequestTooLarge = 12,
    /// READ response bigger than `max_read_size`
    ReadResponseTooLarge = 13,
    ReadQueueSizeExceeded = 14,
    ReadQueueCntExceeded = 15,
    /// WRITE request for more than `max_write_size`
    WriteRequestTooLarge = 16,
    WriteQueueSizeExceeded = 17,
    WriteQueueCntExceeded = 18,
}

impl SMBEvent {
    pub fn from_i32(value: i32) -> Option<SMBEvent> {
        match value {
            0 => Some(SMBEvent::InternalError),
            1 => Some(SMBEvent::MalformedData),
            2 => Some(SMBEvent::RecordOverflow),
            3 => Some(SMBEvent::MalformedNtlmsspRequest),
            4 => Some(SMBEvent::MalformedNtlmsspResponse),
            5 => Some(SMBEvent::DuplicateNegotiate),
            6 => Some(SMBEvent::NegotiateMalformedDialects),
            7 => Some(SMBEvent::FileOverlap),
            8 => Some(SMBEvent::RequestToClient),
            9 => Some(SMBEvent::ResponseToServer),
            10 => Some(SMBEvent::NegotiateMaxReadSizeTooLarge),
            11 => Some(SMBEvent::NegotiateMaxWriteSizeTooLarge),
            12 => Some(SMBEvent::ReadRequestTooLarge),
            13 => Some(SMBEvent::ReadResponseTooLarge),
            14 => Some(SMBEvent::ReadQueueSizeExceeded),
            15 => Some(SMBEvent::ReadQueueCntExceeded),
            16 => Some(SMBEvent::WriteRequestTooLarge),
            17 => Some(SMBEvent::WriteQueueSizeExceeded),
            18 => Some(SMBEvent::WriteQueueCntExceeded),
            _ => None,
        }
    }
}

pub fn smb_str_to_event(instr: &str) -> i32 {
    SCLogDebug!("checking {}", instr);
    match instr {
        "internal_error"                     => SMBEvent::InternalError as i32,
        "malformed_data"                     => SMBEvent::MalformedData as i32,
        "record_overflow"                    => SMBEvent::RecordOverflow as i32,
        "malformed_ntlmssp_request"          => SMBEvent::MalformedNtlmsspRequest as i32,
        "malformed_ntlmssp_response"         => SMBEvent::MalformedNtlmsspResponse as i32,
        "duplicate_negotiate"                => SMBEvent::DuplicateNegotiate as i32,
        "negotiate_malformed_dialects"       => SMBEvent::NegotiateMalformedDialects as i32,
        "file_overlap"                       => SMBEvent::FileOverlap as i32,
        "request_to_client"                  => SMBEvent::RequestToClient as i32,
        "response_to_server"                 => SMBEvent::ResponseToServer as i32,
        "negotiate_max_read_size_too_large"  => SMBEvent::NegotiateMaxReadSizeTooLarge as i32,
        "negotiate_max_write_size_too_large" => SMBEvent::NegotiateMaxWriteSizeTooLarge as i32,
        "read_request_too_large"             => SMBEvent::ReadRequestTooLarge as i32,
        "read_response_too_large"            => SMBEvent::ReadResponseTooLarge as i32,
        "read_queue_size_exceeded"           => SMBEvent::ReadQueueSizeExceeded as i32,
        "read_queue_cnt_exceeded"            => SMBEvent::ReadQueueCntExceeded as i32,
        "write_request_too_large"            => SMBEvent::WriteRequestTooLarge as i32,
        "write_queue_size_exceeded"          => SMBEvent::WriteQueueSizeExceeded as i32,
        "write_queue_cnt_exceeded"           => SMBEvent::WriteQueueCntExceeded as i32,
        _                                    => -1,
    }
}

impl SMBTransaction {
    /// Set event.
    pub fn set_event(&mut self, e: SMBEvent) {
        sc_app_layer_decoder_events_set_event_raw(&mut self.events, e as u8);
    }

    /// Set events from vector of events.
    pub fn set_events(&mut self, events: Vec<SMBEvent>) {
        for e in events {
            sc_app_layer_decoder_events_set_event_raw(&mut self.events, e as u8);
        }
    }
}

impl SMBState {
    /// Set an event. The event is set on the most recent transaction.
    pub fn set_event(&mut self, event: SMBEvent) {
        let len = self.transactions.len();
        if len == 0 {
            return;
        }

        let tx = &mut self.transactions[len - 1];
        tx.set_event(event);
        //sc_app_layer_decoder_events_set_event_raw(&mut tx.events, event as u8);
    }
}
