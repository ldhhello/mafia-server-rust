use super::binarydata::BinaryData;
use super::utils;
use std::error::Error;
use std::mem::swap;

#[derive(Clone)]
pub struct Packet {
    method: i32,
    data: Vec<BinaryData>
}

impl Packet {
    pub fn new() -> Self {
        Self {
            method: 0, 
            data: vec![]
        }
    }
    pub fn from_data(method: i32, data: Vec<BinaryData>) -> Self {
        Self { method, data }
    }

    pub fn from_binary(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let mut packet = Self::new();

        if data.len() < 8 {
            return Err(Box::from("Data is too short"));
        }

        packet.method = utils::slice_to_i32(&data[0..4]);
        let data_cnt = utils::slice_to_u32(&data[4..8]);
        let mut offset: usize = 8;

        for _ in 0..data_cnt {
            if offset + 4 > data.len() {
                return Err(Box::from("Invalid packet"));
            }

            let len = utils::slice_to_u32(&data[offset..offset+4]);
            offset += 4;

            if offset + len as usize > data.len() {
                return Err(Box::from("Invalid packet"));
            }

            let now_data = data[offset..offset+len as usize].to_vec();
            packet.data.push(BinaryData::from_vec(now_data));

            offset += len as usize;
        }

        return Ok(packet);
    }
}

impl Packet {
    pub fn to_binary(mut self, has_packet_size: bool) -> Vec<u8> {
        let mut res = Vec::new();

        let mut len = match has_packet_size {
            true => 12,
            false => 8
        };

        for d in &self.data {
            len += d.vec.len() + 4;
        }

        res.reserve_exact(len);

        if has_packet_size {
            res.append(&mut (len as u32).to_be_bytes().to_vec());
        }
        res.append(&mut self.method.to_be_bytes().to_vec());
        res.append(&mut (self.data.len() as i32).to_be_bytes().to_vec());

        for d in &mut self.data {
            res.append(&mut (d.vec.len() as i32).to_be_bytes().to_vec());
            res.append(&mut d.vec);
        }

        return res;
    }
    pub fn get<'a>(&'a self, idx: usize) -> &'a BinaryData {
        &self.data[idx]
    }
    pub fn get_move(&mut self, idx: usize) -> BinaryData { // 그 위치에는 빈 값을 채워 줌
        let mut data = BinaryData::from_vec(vec![]);
        swap(&mut data, &mut self.data[idx]);

        return data;
    }
    pub fn get_method(&self) -> i32 {
        self.method
    }
    pub fn get_mut<'a>(&'a mut self, idx: usize) -> &'a mut BinaryData {
        &mut self.data[idx]
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn push(&mut self, data: BinaryData) {
        self.data.push(data);
    }
    pub fn set_method(&mut self, method: i32) {
        self.method = method;
    }
    pub fn get_vec(&self) -> &Vec<BinaryData> {
        &self.data
    }
    pub fn get_vec_mut(&mut self) -> &mut Vec<BinaryData> {
        &mut self.data
    }
    pub fn get_vec_move(self) -> Vec<BinaryData> {
        self.data
    }
}