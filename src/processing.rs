use std::fs;

/// Stores content of JDSave file and extracted information
pub(crate) struct FileInfo {
    pub(crate) count: usize,
    pub(crate) data: Vec<u8>,
    pub(crate) webm: Vec<u8>,
    pub(crate) images: Vec<Vec<u8>>,
    pub(crate) additional: Vec<Vec<u8>>
}

/// Extract additional information from JDSave file.
/// Extracts everything not 0 von start to first JFIF header
///
pub(crate) fn extract_additional(file_info: &mut FileInfo) {
    let mut end: bool = true;
    let mut part: Vec<u8> = Vec::new();

    for (i, byte) in file_info.data.iter().enumerate() {
        let byte = *byte;
        if byte == 0u8 {
            if !end {
                file_info.additional.push(part);
                part = Vec::new();
            }
            end = true;
        } else {

            end = false;
            part.push(byte);
        }

        // check if matched jfif tag
        if byte == 0xFF && file_info.data[i + 1] == 0xD8 {
            break;
        }
    }
}


/// Extract all images from JDSave file.
/// Images are in JFIF format and get exported as output_NUMBER_SUBNUMBER.jfif
///
pub(crate) fn extract_images(file_info: &mut FileInfo) {
    let mut start_pos: usize = 0;

    for (i, byte) in file_info.data.iter().enumerate() {
        let next_byte = file_info.data[i + 1];
        let byte = *byte;

        if byte == 0xFF && next_byte == 0xD8 {
            start_pos = i;
        }

        if byte == 0xFF && next_byte == 0xD9 {
            if start_pos == 0 {
                println!("Not possible. Found JFIF END BYTE without beginning");
                continue;
            }

            file_info
                .images
                .push(Vec::from(&file_info.data[start_pos..i + 2]));
            start_pos = 0;
        }

        if byte == 0x1A
            && next_byte == 0x45
            && file_info.data[i + 2] == 0xDF
            && file_info.data[i + 3] == 0xA3
        {
            break;
        }
    }
}

pub(crate) fn extract_video(file_info: &mut FileInfo) {
    const NEEDLE: [u8; 4] = [0x1A, 0x45, 0xDF, 0xA3];

    let mut first = 0;
    let mut last = file_info.data.len() - 1;

    if let Some(found) = file_info
        .data
        .windows(NEEDLE.len())
        .position(|window| window == NEEDLE)
    {
        first = found;
        println!("found magic num");
    } else {
        println!("did not found magic num");
    }

    // find end of webm
    loop {
        if file_info.data[last] == 0 {
            last -= 1;
        } else {
            break;
        }
    }

    file_info.webm = Vec::from(&file_info.data[first..=last]);
}
