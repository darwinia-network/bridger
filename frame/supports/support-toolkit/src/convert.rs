/// Smart codec types mapper
#[cfg(feature = "codec")]
pub struct SmartCodecMapper;

#[cfg(feature = "codec")]
impl SmartCodecMapper {
    /// map an encodeable types to new decodeable types
    pub fn map_to<S, T>(source: &S) -> Result<T, codec::Error>
    where
        S: codec::Encode,
        T: codec::Decode,
    {
        let decoded = codec::Encode::encode(source);
        T::decode(&mut decoded.as_slice())
    }
}
