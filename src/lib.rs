#[test]
fn it_works() {
    assert_eq!(murmur3_32([], 0), 0);
    assert_eq!(murmur3_32([], 1), 0x514E28B7);
    assert_eq!(murmur3_32([], 0xffffffff), 0x81F16F39);
    assert_eq!(murmur3_32([0,0,0,0], 0), 0x2362F9DE);
    assert_eq!(murmur3_32([0x21,0x43,0x65,0x87], 0x5082EDEE), 0x2362F9DE);
    assert_eq!(murmur3_32([0x21,0x43,0x65], 0), 0x7E4A8634);
    assert_eq!(murmur3_32([0x21,0x43], 0), 0xA0F7B07A);
    assert_eq!(murmur3_32([0x21], 0), 0x72661CF4);
}


pub const fn murmur3_32<const T: usize>(data: [u8; T], seed: u32) -> u32 {
    let c1: u32 = 0xcc9e2d51;
    let c2: u32 = 0x1b873593;
    let r2 = 13;
    let m = 5;
    let n: u32 = 0xe6546b64;

    let mut hash = seed;
    
    let mut i = 0;
    let iterator = T/4;
    while i < iterator {
        let data = [data[i*4], data[i*4+1], data[i*4+2], data[i*4+3]];
        let mut k = u32::from_le_bytes(data);
        k = k.wrapping_mul(c1);
        k = k.rotate_left(15);
        k = k.wrapping_mul(c2);
        
        hash = hash ^ k;
        hash = hash << r2;
        hash = hash.wrapping_mul(m).wrapping_add(n);

        i+=1;
    }
    match T%4 {
        0 => (),
        1 => {
            let data = [data[i*4], 0, 0, 0];
            let k = mix_bytes_32(data);
            hash = hash ^ k;
        },
        2 => {
            let data = [data[i*4], data[i*4+1], 0, 0];
            let k = mix_bytes_32(data);
            hash = hash ^ k;
        },
        3 => {
            let data = [data[i*4], data[i*4+1], data[i*4+2], 0];
            let k = mix_bytes_32(data);
            hash = hash ^ k;
            
        }
        _ => unimplemented!()
    }

    hash = hash ^ T as u32;
    hash = hash ^ (hash.wrapping_shr(16));
    hash = hash.wrapping_mul(0x85ebca6b);
    hash = hash ^ (hash.wrapping_shr(13));
    hash = hash.wrapping_mul(0xc2b2ae35);
    hash = hash ^ (hash.wrapping_shr(16));


    hash
}

const fn mix_bytes_32(data: [u8; 4]) -> u32 {
    let r1 = 15;
    let c1: u32 = 0xcc9e2d51;
    let c2: u32 = 0x1b873593;
    let mut k = u32::from_le_bytes(data);
    k = k.wrapping_mul(c1);
    k = k.rotate_left(r1);
    k = k.wrapping_mul(c2);
    return k
}