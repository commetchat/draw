use std::io::{self, Read};

pub fn read_u8<R: Read>(file: &mut R) -> io::Result<u8> {
    let mut buffer = [0u8; 1];
    file.read_exact(&mut buffer)?;
    Ok(buffer[0])
}

pub fn read_u16<R: Read>(file: &mut R) -> io::Result<u16> {
    let mut buffer = [0u8; 2];
    file.read_exact(&mut buffer)?;
    Ok(u16::from_le_bytes(buffer))
}

pub fn read_u32<R: Read>(file: &mut R) -> io::Result<u32> {
    let mut buffer = [0u8; 4];
    file.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

pub fn read_u64<R: Read>(file: &mut R) -> io::Result<u64> {
    let mut buffer = [0u8; 8];
    file.read_exact(&mut buffer)?;
    Ok(u64::from_le_bytes(buffer))
}

pub fn read_f32<R: Read>(file: &mut R) -> io::Result<f32> {
    let mut buffer = [0u8; 4];
    file.read_exact(&mut buffer)?;
    Ok(f32::from_le_bytes(buffer))
}

pub fn read_f64<R: Read>(file: &mut R) -> io::Result<f64> {
    let mut buffer = [0u8; 8];
    file.read_exact(&mut buffer)?;
    Ok(f64::from_le_bytes(buffer))
}

pub fn read_string<R: Read>(file: &mut R) -> io::Result<String> {
    let _ = read_u8(file)?;

    let mut buffer = Vec::new();
    loop {
        let byte = read_u8(file)?;
        if byte == 0 {
            break;
        }
        buffer.push(byte);
    }
    Ok(String::from_utf8_lossy(&buffer).to_string())
}
