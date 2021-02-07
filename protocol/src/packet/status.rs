use crate::DecodeError;
use crate::Decoder;
use std::io::Read;
use minecraft_protocol_derive::Packet;


pub enum StatusServerBoundPacket {
    StatusRequest,
    PingRequest(PingRequest)
}

impl StatusServerBoundPacket {
    pub fn get_type_id(&self) -> u8 {
        match self {
            Self::StatusRequest => 0x00,
            Self::PingRequest(_) => 0x01
        }
    }

    pub fn decode<R: Read>(type_id: u8, reader: &mut R) -> Result<Self, DecodeError> {
        match type_id {
            0x00 => {
                Ok(Self::StatusRequest)
            }
            0x01 => {
                let ping_request = PingRequest::decode(reader)?;

                Ok(Self::PingRequest(ping_request))
            }
            _ => Err(DecodeError::UnknownPacketType { type_id })
        }
    }

    pub fn status_request() -> Self {
        Self::StatusRequest
    }

    pub fn ping_request(time: i64) -> Self {
        let ping_request = PingRequest {
            time
        };

        Self::PingRequest(ping_request)
    }
}
pub enum StatusClientBoundPacket {
    StatusResponse(StatusResponse),
    PingResponse(PingResponse)
}

impl StatusClientBoundPacket {
    pub fn get_type_id(&self) -> u8 {
        match self {
            Self::StatusResponse(_) => 0x00,
            Self::PingResponse(_) => 0x01
        }
    }

    pub fn decode<R: Read>(type_id: u8, reader: &mut R) -> Result<Self, DecodeError> {
        match type_id {
            0x00 => {
                let status_response = StatusResponse::decode(reader)?;

                Ok(Self::StatusResponse(status_response))
            }
            0x01 => {
                let ping_response = PingResponse::decode(reader)?;

                Ok(Self::PingResponse(ping_response))
            }
            _ => Err(DecodeError::UnknownPacketType { type_id })
        }
    }

    pub fn status_response(response: String) -> Self {
        let status_response = StatusResponse {
            response
        };

        Self::StatusResponse(status_response)
    }

    pub fn ping_response(time: i64) -> Self {
        let ping_response = PingResponse {
            time
        };

        Self::PingResponse(ping_response)
    }
}
#[derive(Packet, Debug)]
pub struct PingRequest {
    pub time: i64
}


#[derive(Packet, Debug)]
pub struct StatusResponse {
    pub response: String
}

#[derive(Packet, Debug)]
pub struct PingResponse {
    pub time: i64
}

