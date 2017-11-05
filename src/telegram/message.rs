use std::fmt;


pub enum MessageBody {
    Error { content: String },
    FileWrited { path: String, content: String },
    FileRemoved { path: String },
}
impl fmt::Display for MessageBody {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match *self {
            MessageBody::Error { ref content } => {
                format!("Internal error: `{}`\nLogram stopped", content)
            }
            MessageBody::FileWrited {
                ref path,
                ref content,
            } => format!("*{}*\n`{}`", path, content),
            MessageBody::FileRemoved { ref path } => {
                format!("File *{}* removed.\nWatching then stopped", path)
            }
        };
        write!(f, "{}", message)
    }
}

pub struct Message {
    pub chat_id: Option<i64>,
    pub body: MessageBody,
}
