use std::fs;
use std::io;

// Write on the file.
pub fn write_bytes_to_file(path: &str,
    line: usize,
    offset: usize,
    size_of_data: usize,
    data: &[u8]) 
    -> Result<(), io::Error>{

        let mut bytes = fs::read(path)?;
        let line_offset = (line - 1) * 16;
        let offset_to_edit = line_offset + offset - 1;

        if data.len() != size_of_data {
            println!("Data length ({}) doesn't match the expected size ({}). Aborting write.", data.len(), size_of_data);
            return Ok(())
        } 
        
        if offset_to_edit + data.len () > bytes.len() {
            println!("Write would go out of bounds of the file. Aborting.");
            return Ok(());
        }


        for (i, &byte) in data.iter().enumerate(){
            bytes[offset_to_edit + i] = byte;
        }
        fs::write(path, bytes)?;
        println!("Successfully wrote {} bytes at offset 0x{:x} (line {}, offset {}.)", data.len(), offset_to_edit, line, offset);
        Ok(())
}