use super::StreamCommand;
use crate::result::*;

#[derive(Debug, Default)]
pub struct PingCommand;

impl StreamCommand for PingCommand {
    type Response = bool;

    fn message(&self) -> String {
        String::from("PING\r\n")
    }

    fn receive(&self, message: String) -> Result<Self::Response> {
        log::debug!("{}", &message);
        if message == "PONG\r\n" {
            Ok(true)
        } else {
            log::error!("{}", &message);
            Err(Error::new(ErrorKind::WrongSonicResponse))
        }
    }
}
