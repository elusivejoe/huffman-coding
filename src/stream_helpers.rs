use std::io::{Seek, SeekFrom};

pub fn stream_current_position<T: Seek>(stream: &mut T) -> std::io::Result<u64> {
    stream.seek(SeekFrom::Current(0))
}

pub fn stream_length<T: Seek>(stream: &mut T) -> std::io::Result<u64> {
    let old_pos = stream_current_position(stream)?;
    let len = stream.seek(SeekFrom::End(0))?;
    stream.seek(SeekFrom::Start(old_pos))?;

    Ok(len)
}
