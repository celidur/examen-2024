use std::simd::u8x64;

#[cfg(test)]
mod tests;

// This is an example of what it should do, you need to do it faster with SIMD.
pub fn naive_xor_chunks(a: [u8; 4096], b: [u8; 4096]) -> [u8; 4096] {
    let mut c = [0; 4096];
    for ((a, b), c) in a.into_iter().zip(b.into_iter()).zip(c.iter_mut()) {
        *c = a ^ b;
    }
    c
}

pub fn xor_chunks(a: [u8; 4096], b: [u8; 4096]) -> [u8; 4096] {
    let mut result = [0u8; 4096];
    for ((chunk_a, chunk_b), chunk_result) in a
        .array_chunks::<64>()
        .zip(b.array_chunks::<64>())
        .zip(result.array_chunks_mut::<64>())
    {
        *chunk_result = (u8x64::from_array(*chunk_a) ^ u8x64::from_array(*chunk_b)).to_array();
    }

    result
}
