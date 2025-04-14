use std::fs::File;
use std::error::Error;
use serde::Serialize;
use csv::Writer;

// This function will accept data as a vector of serializable rows (i.e., a collection of data)
pub fn write_to_csv<T>(data_buffer: &[T], filename: &str) -> Result<(), Box<dyn Error>>
where
    T: Serialize,
{
    // Create a file with the given filename
    let file = File::create(filename)?;
    let mut writer = Writer::from_writer(file);

    // Write each row of the buffer to the CSV
    for row in data_buffer {
        writer.serialize(row)?;
    }

    // Flush the writer to ensure data is written to the file
    writer.flush()?;

    println!("Log file saved to {}", filename);

    Ok(())
}
