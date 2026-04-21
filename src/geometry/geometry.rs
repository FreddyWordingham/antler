use crate::geometry::{Bounded, Traceable};

pub trait Geometry: Bounded + Traceable {}
impl<T: Bounded + Traceable> Geometry for T {}
