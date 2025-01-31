use std::io;


fn _handle_out<H>(msg: String, mut handler: H)
where
    H: std::io::Write,
{
    handler.write_all(msg.as_bytes()).unwrap();
    handler.flush().unwrap();
}

pub fn println<M>(msg: M)
where
    M: Into<String>,
{
    _handle_out(msg.into() + "\n".as_ref(), io::stdout());
}
pub fn eprintln<M>(msg: M)
where
    M: Into<String>,
{
    _handle_out(msg.into() + "\n".as_ref(), io::stderr());
}
