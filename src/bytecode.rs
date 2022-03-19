
type Instruction = (u8, Box<[isize]>);
type ByteCode = Box<[Instruction]>;
