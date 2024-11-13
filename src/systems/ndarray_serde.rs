//! Helper module for ndarray serialization

pub mod array4 {
    use ndarray::Array4;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S, T>(array: &Array4<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize + Clone,
    {
        let shape = array.shape();
        let data: Vec<_> = array.iter().cloned().collect();
        (shape, data).serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Array4<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de> + Clone,
    {
        let (shape, data): ([usize; 4], Vec<T>) = Deserialize::deserialize(deserializer)?;
        Ok(Array4::from_shape_vec(shape, data).unwrap())
    }
}
