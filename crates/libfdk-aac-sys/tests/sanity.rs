use libfdk_aac_sys::{aacDecoder_Close, aacDecoder_Open, TRANSPORT_TYPE_TT_MP4_ADTS};

// Test checking that the library is correctly linking.
#[test]
fn sanity() {
    let decoder = unsafe { aacDecoder_Open(TRANSPORT_TYPE_TT_MP4_ADTS, 4) };
    unsafe { aacDecoder_Close(decoder) }
}
