use mycelium_bitfield::bitfield;



bitfield! {
    pub struct VirtAddr<usize> {
        pub const PAGE_OFFSET = 12;
        pub const LVL_0_IDX = 9;
        pub const LVL_1_IDX = 9;
        pub const LVL_2_IDX = 9;
        const _RESERVED = 24;
    }
}
    impl VirtAddr {
    pub fn lvl_idx(&self, lvl: usize) -> usize {
        match lvl {
            0 => self.get(Self::LVL_0_IDX),
            1 => self.get(Self::LVL_1_IDX),
            2 => self.get(Self::LVL_2_IDX),
            _ => unreachable!()
        }
    }
}
