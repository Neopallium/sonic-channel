use super::StreamCommand;
use crate::result::*;

#[derive(Debug, Default)]
pub struct QuitCommand;

impl StreamCommand for QuitCommand {
    type Response = bool;

    fn message(&self) -> String {
        String::from("QUIT\r\n")
    }

    fn receive(&self, message: String) -> Result<Self::Response> {
        log::debug!("{}", &message);
        if message.starts_with("ENDED ") {
            Ok(true)
        } else {
            log::error!("{}", &message);
            Err(Error::new(ErrorKind::WrongSonicResponse))
        }
    }
}
