use std::io::{ErrorKind, Seek, SeekFrom};

pub struct Args {
    pub file_in: String,
    pub file_out: String,
    pub mode: String,
}

pub fn parse_args(raw_args: &Vec<String>) -> std::io::Result<Args> {
    if raw_args.len() != 4 {
        return Err(std::io::Error::new(
            ErrorKind::Other,
            "Not enough actual parameters.",
        ));
    }

    let mode = &raw_args[1];

    if mode != "compress" && mode != "decompress" {
        return Err(std::io::Error::new(ErrorKind::Other, "Unknown mode."));
    }

    let file_in = &raw_args[2];
    let file_out = &raw_args[3];

    let args = Args {
        file_in: file_in.to_owned(),
        file_out: file_out.to_owned(),
        mode: mode.to_owned(),
    };

    Ok(args)
}

pub fn stream_current_position<T: Seek>(stream: &mut T) -> std::io::Result<u64> {
    stream.seek(SeekFrom::Current(0))
}

pub fn stream_length<T: Seek>(stream: &mut T) -> std::io::Result<u64> {
    let old_pos = stream_current_position(stream)?;
    let len = stream.seek(SeekFrom::End(0))?;
    stream.seek(SeekFrom::Start(old_pos))?;

    Ok(len)
}
