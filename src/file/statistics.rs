use std::borrow::BorrowMut;
use std::convert::TryFrom;
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};
use varint::VarintRead;

use crate::error::TsFileError;
use crate::utils::io::BigEndianReader;

#[derive(Debug)]
pub enum Statistic {
    Boolean(BooleanStatistics),
    Int32(IntegerStatistics),
    Int64(LongStatistics),
    FLOAT(FloatStatistics),
    DOUBLE(DoubleStatistics),
    TEXT(BinaryStatistics),
}

#[derive(Debug)]
pub struct StatisticHeader {
    is_empty: bool,
    count: i32,
    start_time: i64,
    end_time: i64,
}

#[derive(Debug)]
pub struct BinaryStatistics {
    header: StatisticHeader,
    first_value: String,
    last_value: String,
}

#[derive(Debug)]
pub struct BooleanStatistics {
    header: StatisticHeader,
    first_value: bool,
    last_value: bool,
    sum_value: i64,
}

#[derive(Debug)]
pub struct IntegerStatistics {
    header: StatisticHeader,
    min_value: i32,
    max_value: i32,
    first_value: i32,
    last_value: i32,
    sum_value: i64,
}

#[derive(Debug)]
pub struct LongStatistics {
    header: StatisticHeader,
    min_value: i64,
    max_value: i64,
    first_value: i64,
    last_value: i64,
    sum_value: f64,
}

#[derive(Debug)]
pub struct DoubleStatistics {
    header: StatisticHeader,
    min_value: f64,
    max_value: f64,
    first_value: f64,
    last_value: f64,
    sum_value: f64,
}

#[derive(Debug)]
pub struct FloatStatistics {
    header: StatisticHeader,
    min_value: f32,
    max_value: f32,
    first_value: f32,
    last_value: f32,
    sum_value: f64,
}

impl TryFrom<&mut Cursor<Vec<u8>>> for StatisticHeader {
    type Error = TsFileError;

    fn try_from(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, Self::Error> {
        Ok(Self {
            count: cursor.read_unsigned_varint_32()? as i32,
            start_time: cursor.read_big_endian_i64(),
            end_time: cursor.read_big_endian_i64(),
            is_empty: false,
        })
    }
}

impl TryFrom<&mut Cursor<Vec<u8>>> for BooleanStatistics {
    type Error = TsFileError;

    fn try_from(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, Self::Error> {
        Ok(Self {
            header: StatisticHeader::try_from(cursor.borrow_mut())?,
            first_value: cursor.read_bool(),
            last_value: cursor.read_bool(),
            sum_value: cursor.read_big_endian_i64(),
        })
    }
}

impl TryFrom<&'_ mut Cursor<Vec<u8>>> for IntegerStatistics {
    type Error = TsFileError;

    fn try_from(cursor: &'_ mut Cursor<Vec<u8>>) -> Result<Self, Self::Error> {
        Ok(Self {
            header: StatisticHeader::try_from(cursor.borrow_mut())?,
            min_value: cursor.read_big_endian_i32(),
            max_value: cursor.read_big_endian_i32(),
            first_value: cursor.read_big_endian_i32(),
            last_value: cursor.read_big_endian_i32(),
            sum_value: cursor.read_big_endian_i64(),
        })
    }
}

impl TryFrom<&'_ mut Cursor<Vec<u8>>> for FloatStatistics {
    type Error = TsFileError;

    fn try_from(cursor: &'_ mut Cursor<Vec<u8>>) -> Result<Self, Self::Error> {
        Ok(Self {
            header: StatisticHeader::try_from(cursor.borrow_mut())?,
            min_value: cursor.read_f32::<BigEndian>()?,
            max_value: cursor.read_f32::<BigEndian>()?,
            first_value: cursor.read_f32::<BigEndian>()?,
            last_value: cursor.read_f32::<BigEndian>()?,
            sum_value: cursor.read_f64::<BigEndian>()?,
        })
    }
}

impl TryFrom<&'_ mut Cursor<Vec<u8>>> for DoubleStatistics {
    type Error = TsFileError;

    fn try_from(cursor: &'_ mut Cursor<Vec<u8>>) -> Result<Self, Self::Error> {
        Ok(Self {
            header: StatisticHeader::try_from(cursor.borrow_mut())?,
            min_value: cursor.read_f64::<BigEndian>()?,
            max_value: cursor.read_f64::<BigEndian>()?,
            first_value: cursor.read_f64::<BigEndian>()?,
            last_value: cursor.read_f64::<BigEndian>()?,
            sum_value: cursor.read_f64::<BigEndian>()?,
        })
    }
}

impl TryFrom<&'_ mut Cursor<Vec<u8>>> for LongStatistics {
    type Error = TsFileError;

    fn try_from(cursor: &'_ mut Cursor<Vec<u8>>) -> Result<Self, Self::Error> {
        Ok(Self {
            header: StatisticHeader::try_from(cursor.borrow_mut())?,
            min_value: cursor.read_i64::<BigEndian>()?,
            max_value: cursor.read_i64::<BigEndian>()?,
            first_value: cursor.read_i64::<BigEndian>()?,
            last_value: cursor.read_i64::<BigEndian>()?,
            sum_value: cursor.read_f64::<BigEndian>()?,
        })
    }
}

impl TryFrom<&'_ mut Cursor<Vec<u8>>> for BinaryStatistics {
    type Error = TsFileError;

    fn try_from(value: &'_ mut Cursor<Vec<u8>>) -> Result<Self, Self::Error> {
        todo!()
    }
}
