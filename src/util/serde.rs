use enumset::{EnumSet, EnumSetType};
use serde::{Serialize, Serializer};

pub fn ser_enumset<T, S>(set: &EnumSet<T>, ser: S) -> Result<S::Ok, S::Error>
where
    T: EnumSetType + Serialize,
    S: Serializer,
{
    ser.collect_seq(set.iter())
}
