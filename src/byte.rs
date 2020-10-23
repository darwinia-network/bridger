#![macro_use]

/// Doc with expr
#[macro_export]
macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

/// Convert bytes to hex
#[macro_export]
macro_rules! hex {
    ($bytes:expr) => {{
        let mut s = String::new();
        for i in $bytes {
            s.push_str(&format!("{:02x}", i));
        }
        s
    }};
}

/// Convert hex string to `Vec<u8>` or `[u8; n]`
#[macro_export]
macro_rules! bytes {
    // Convert hex to Vec<u8>
    ($hex:expr) => {{
        let mut h = $hex;
        if h.starts_with("0x") {
            h = &h[2..];
        }

        (0..h.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&h[i..i + 2], 16))
            .collect::<Result<Vec<u8>, _>>()
            .unwrap_or_default()
    }};

    // Convert hex to [u8; $bits]
    ($hex:expr, $bits:expr) => {{
        let mut hash = [0_u8; $bits];
        hash.copy_from_slice(&bytes!($hex));
        hash
    }};
}

/// Implement serde for big array
#[macro_export]
macro_rules! serde_array {
    ($len:expr) => {
        impl<'de, T> BigArray<'de> for [T; $len]
        where
            T: Default + Copy + Serialize + Deserialize<'de>,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut seq = serializer.serialize_tuple(self.len())?;
                for elem in &self[..] {
                    seq.serialize_element(elem)?;
                }
                seq.end()
            }

            fn deserialize<D>(deserializer: D) -> Result<[T; $len], D::Error>
            where
                D: Deserializer<'de>,
            {
                struct ArrayVisitor<T> {
                    element: PhantomData<T>,
                }

                impl<'de, T> Visitor<'de> for ArrayVisitor<T>
                where
                    T: Default + Copy + Deserialize<'de>,
                {
                    type Value = [T; $len];

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str(concat!("an array of length ", $len))
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<[T; $len], A::Error>
                    where
                        A: SeqAccess<'de>,
                    {
                        let mut arr = [T::default(); $len];
                        for i in 0..$len {
                            arr[i] = seq
                                .next_element()?
                                .ok_or_else(|| Error::invalid_length(i, &self))?;
                        }
                        Ok(arr)
                    }
                }

                let visitor = ArrayVisitor {
                    element: PhantomData,
                };
                deserializer.deserialize_tuple($len, visitor)
            }
        }
    };
}

/// Construct hash bytes
#[macro_export]
macro_rules! construct_hash_bytes {
    ( $(#[$attr:meta])* $visibility:vis struct $name:ident ( $len:tt ); ) => {
        serde_array!($len);

        doc_comment!{
            concat!("The ", stringify!($len), "-bit hash type."),
            $(#[$attr])*
            #[derive(Decode, Encode, Serialize, Deserialize)]
            $visibility struct $name (
                #[serde(with = "BigArray")]
                pub [u8; $len]
            );
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str(&hex!(self.0.as_ref()))
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.debug_list().entries(self.0.iter()).finish()
            }
        }

        impl Default for $name {
            fn default() -> $name {
                $name([0; $len])
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                for i in 0..self.0.len() {
                    if self.0[i] != other.0[i] {
                        return false;
                    }
                }
                true
            }
        }

        impl Eq for $name {}
    };
}
