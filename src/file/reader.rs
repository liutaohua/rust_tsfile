use std::fs::File;
use std::io::{Cursor, Read};
use std::sync::Arc;

use crate::chunk::reader::PageHeader;
use crate::encoding::decoder::Field;
use crate::error::Result;
use crate::file::metadata::{
    ChunkMetadata, MetadataIndexNodeType, TimeseriesMetadata, TsFileMetadata,
};
use crate::utils::io::FileSource;

pub trait Length {
    fn len(&self) -> u64;
}

pub trait TryClone: Sized {
    fn try_clone(&self) -> std::io::Result<Self>;
}

pub trait SectionReader: Length {
    type T: Read;
    fn get_read(&self, start: u64, len: usize) -> Result<Self::T>;
    fn get_cursor(&self, start: u64, len: usize) -> Result<Cursor<Vec<u8>>>;
}

pub trait FileReader {
    fn metadata(&self) -> &TsFileMetadata;
    fn device_meta_iter(&self) -> Box<dyn DeviceMetadataIter<Item=MetadataIndexNodeType>>;
    fn get_device_reader();
    fn sensor_meta_iter(
        &self,
        device: &str,
    ) -> Box<dyn SensorMetadataIter<Item=TimeseriesMetadata>>;

    fn get_sensor_reader(&self, device: &str, sensor: &str) -> Option<Box<dyn SensorReader>>;
}

pub trait DeviceMetadataIter: Iterator {}

pub trait SensorMetadataIter: Iterator {}

pub trait DeviceReader {
    fn metadata(&self) -> Vec<TimeseriesMetadata>;

    fn get_sensor_reader(&self, i: usize) -> Result<Box<dyn SensorReader>>;
}

pub trait SensorReader {
    fn metadata(&self) -> &Vec<ChunkMetadata>;

    fn number_of_chunks(&self) -> usize;

    fn get_chunk_reader(
        &self,
        i: usize,
    ) -> Result<Box<dyn ChunkReader<Item=Box<dyn PageReader>>>>;
}

pub trait ChunkReader: Iterator {
}

pub trait PageReader {
    fn header(&self) -> &PageHeader;
    fn data(&self) -> (Vec<Field>, Vec<Field>);
}

pub struct RowIter {
    current_row_group: usize,
    num_row_groups: usize,
    iters: Vec<Box<dyn PageReader>>,
}

impl RowIter {
    pub fn new(iters: Vec<Box<dyn PageReader>>) -> Self {
        Self {
            current_row_group: 0,
            num_row_groups: 0,
            iters,
        }
    }
}

impl Length for File {
    fn len(&self) -> u64 {
        self.metadata().map(|m| m.len()).unwrap_or(0u64)
    }
}

impl SectionReader for File {
    type T = FileSource<File>;

    fn get_read(&self, start: u64, length: usize) -> Result<Self::T> {
        Ok(FileSource::new(self, start, length))
    }

    fn get_cursor(&self, start: u64, len: usize) -> Result<Cursor<Vec<u8>>> {
        match self.get_read(start, len) {
            Ok(mut reader) => {
                let mut data = vec![0; len];
                reader.read_exact(&mut data);
                Ok(Cursor::new(data))
            }
            Err(e) => Err(e),
        }
    }
}

impl TryClone for File {
    fn try_clone(&self) -> std::io::Result<Self> {
        self.try_clone()
    }
}
