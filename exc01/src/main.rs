use std::{io, fs, fmt, str};
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug, Copy, Clone)]
struct Object {
    id: u8,
    message: u8,
    crc: u8,
}

/// Creates Object based on input array of '0' and '1'
///
/// # Example
/// ['1','1','1','1','1','1','1','1'] -> Object {id: 15, message: 7, crc: 1}
///
impl From<&[u8]> for Object {
    fn from(array: &[u8]) -> Self {
        let id = str::from_utf8(&array[0..4]).unwrap();
        let message = str::from_utf8(&array[4..7]).unwrap();
        let crc = array[7] as char;
        Object {
            id: u8::from_str_radix(id, 2).unwrap(),
            message: u8::from_str_radix(message, 2).unwrap(),
            crc: crc.to_string().parse().unwrap(),
        }
    }
}

/// "Converts" Object to binary String
///
/// # Example
/// Object {id: 15, message: 7, crc: 1} -> "11111111"
///
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>4b}{:0>3b}{:b}", self.id, self.message, self.crc)
    }
}

/// Reads and parses file to a Vec<Object>
///
/// # Errors
/// Returns `Err` if file contains any character other than '0' or '1',
/// or length of the file is not divisible by 8.
///
fn read_file(input_path: String) -> Result<Vec<Object>, io::Error> {
    let mut file = fs::File::open(input_path)?;
    let mut v_buffer = Vec::new();
    file.read_to_end(&mut v_buffer)?;

    if v_buffer.len() % 8 != 0 {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Input data length not divisible by 8"));
    }

    if v_buffer.iter().any(|c| *c != 48 && *c != 49) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Input data contains forbidden characters"));
    }

    // create Vec<Object> from vector of chars
    let v_object = v_buffer.chunks(8)
        .into_iter()
        .map(|val| Object::from(val))
        .collect::<Vec<Object>>();
    Ok(v_object)
}

/// Writes output file based on input Vec<Object>
///
fn write_file(output_path: String, v_object: Vec<Object>) -> Result<(), io::Error> {
    let mut file = File::create(output_path)?;
    let v_object_len = v_object.len();

    // filtering v_object into vector of only valid Objects
    let v_valid_obj = v_object
        .into_iter()
        .filter(|obj| {
            println!("{:?} {:?}", obj, (obj.id % 2 == obj.crc) && (obj.message != 0));
            (obj.id % 2 == obj.crc) && (obj.message != 0)
        })
        .collect::<Vec<Object>>();

    write!(file, "{}\n", v_object_len)?;
    write!(file, "{}\n", v_object_len - v_valid_obj.len())?;
    write!(file, "{}", v_valid_obj.into_iter().map(|obj| format!("{}", obj)).collect::<Vec<String>>().join(""))?;
    Ok(())
}

fn main() -> io::Result<()> {
    let v_object = read_file("input.txt".to_string())?;
    write_file("output.txt".to_string(), v_object)?;
    Ok(())
}
