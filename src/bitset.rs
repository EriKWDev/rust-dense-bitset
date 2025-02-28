/// Trait to define the basic functions of a `BitSet`
pub trait BitSet {
    /// Sets the value of the bit at position `position` to `value`
    fn set_bit(&mut self, position: usize, value: bool);

    /// Gets the value of the bit at position `position`
    fn get_bit(&self, position: usize) -> bool;

    /// Returns the bitset's Hamming weight
    fn get_weight(&self) -> u32;

    /// Resets the bitset
    fn reset(&mut self);

    /// Produces a string representation of the bitset (little endian), aligned with 64 bits and with leading zeroes
    fn to_string(self) -> String;
}

/// Trait to define operations between bitsets such as is_subset_of, is_superset_of and more
pub trait BitsetOps<O: BitSet> {
    fn is_subset_of(&self, other: &O) -> bool;
}
