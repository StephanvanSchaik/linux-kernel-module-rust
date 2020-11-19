#[derive(Clone, Copy, Debug)]
pub enum Ioctl {
    None(u32, u32, u32),
    Read(u32, u32, u32),
    Write(u32, u32, u32),
    ReadWrite(u32, u32, u32),
}

impl Ioctl {
    const READ: u32 = 2;
    const WRITE: u32 = 1;
    const READ_WRITE: u32 = Ioctl::READ | Ioctl::WRITE;

    const NUM_BITS: usize = 8;
    const TYPE_BITS: usize = 8;
    const SIZE_BITS: usize = 14;
    const DIR_BITS: usize = 2;

    const NUM_SHIFT: usize = 0;
    const TYPE_SHIFT: usize = Ioctl::NUM_SHIFT + Ioctl::NUM_BITS;
    const SIZE_SHIFT: usize = Ioctl::TYPE_SHIFT + Ioctl::TYPE_BITS;
    const DIR_SHIFT: usize = Ioctl::SIZE_SHIFT + Ioctl::SIZE_BITS;

    const NUM_MASK: u32 = (1 << Ioctl::NUM_BITS) - 1;
    const TYPE_MASK: u32 = (1 << Ioctl::TYPE_BITS) - 1;
    const SIZE_MASK: u32 = (1 << Ioctl::SIZE_BITS) - 1;
    const DIR_MASK: u32 = (1 << Ioctl::DIR_BITS) - 1;

    pub fn size(self) -> u32 {
        match self {
            Ioctl::None(_, _, size) => size,
            Ioctl::Read(_, _, size) => size,
            Ioctl::Write(_, _, size) => size,
            Ioctl::ReadWrite(_, _, size) => size,
        }
    }
}

impl From<u32> for Ioctl {
    fn from(ioctl: u32) -> Self {
        let num = (ioctl >> Ioctl::NUM_SHIFT) & Ioctl::NUM_MASK;
        let ty = (ioctl >> Ioctl::TYPE_SHIFT) & Ioctl::TYPE_MASK;
        let size = (ioctl >> Ioctl::SIZE_SHIFT) & Ioctl::SIZE_MASK;
        let dir = (ioctl >> Ioctl::DIR_SHIFT) & Ioctl::DIR_MASK;

        match dir {
            Ioctl::READ => Ioctl::Read(ty, num, size),
            Ioctl::WRITE => Ioctl::Write(ty, num, size),
            Ioctl::READ_WRITE => Ioctl::ReadWrite(ty, num, size),
            _ => Ioctl::None(ty, num, size),
        }
    }
}
