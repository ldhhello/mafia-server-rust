use super::utils;

#[derive(Clone)]
pub struct BinaryData {
    pub vec: Vec<u8>
}

impl BinaryData {
    pub fn from_vec(vec: Vec<u8>) -> BinaryData {
        BinaryData { vec }
    }
    pub fn from_i32(num: i32) -> BinaryData {
        BinaryData {
            vec: num.to_be_bytes().to_vec()
        }
    }
    pub fn from_string(str: String) -> BinaryData {
        BinaryData {
            vec: str.into_bytes()
        }
    }
    pub fn from_string_ref(str: &mut String) -> BinaryData {
        BinaryData {
            vec: str.as_bytes().to_vec()
        }
    }
    pub fn to_i32(&self) -> i32 {
        return utils::slice_to_i32(&self.vec[..]);
    }
    pub fn to_bool(&self) -> bool {
        return self.to_i32() != 0;
    }
    pub fn as_string(self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(String::from_utf8(self.vec)?)
    }
}