pub fn show_line(data: &[u8], offset: usize) -> String
{   
    if offset % 16 != 0 {
        println!("There is no offset of {}. The offset number must be a multiple of 16.", offset);
        return String::new();
    }

    if offset >= data.len() {
        println!("Offset is out of bounds.");
        return String::new();
    } else {
        let start = offset;
        let end = std::cmp::min(start + 16, data.len());
        let mut output = String::new();
        // the hex part of the line.
        let hex_parts: Vec<String> = data[start..end]
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect();

        let hex_line = hex_parts.join(" ");
        let hex_padded = format!("{:<48}", hex_line);

        // the ascii part of the line.
        let ascii_part = data[start..end]
        .iter()
        .map(|&b| {
            if b >= 32 && b <= 126 {
                b as char
            } else {
                '.'
            }
        })
        .collect::<String>();

        output.push_str(&format!("{:08x} {} {}\n", start, hex_padded, ascii_part));
        output
    }
}

pub fn show_lines(data: &[u8],
    offset: usize,
    count: usize)
    -> String
{
    let mut output = String::new();
    for i in 0..count {
        let current_offset = offset + i * 16;
        if current_offset >= data.len() {
            break;
        }
        output.push_str(&show_line(data, current_offset));
    }
    output
}