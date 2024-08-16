pub fn slice_to_i32(data: &[u8]) -> i32 {
    let mut d: [u8; 4] = [0; 4];
    d.clone_from_slice(data);
    
    i32::from_be_bytes(d)
}
pub fn slice_to_u32(data: &[u8]) -> u32 {
    let mut d: [u8; 4] = [0; 4];
    d.clone_from_slice(data);
    
    u32::from_be_bytes(d)
}