// this file defines the binary value of each register, relation to other registers,
// and register object
use paste::paste;
pub type RegisterLength = u8;

pub const NUL: RegisterLength = 0b_0000_0000;
pub const R1: RegisterLength = 0b_0001_0000;
pub const R2: RegisterLength = 0b_0010_0000;
pub const R3: RegisterLength = 0b_0011_0000;
pub const R4: RegisterLength = 0b_0100_0000;
pub const R5: RegisterLength = 0b_0101_0000;
pub const R6: RegisterLength = 0b_0110_0000;
pub const R7: RegisterLength = 0b_0111_0000;
pub const R8: RegisterLength = 0b_1000_0000;
pub const R9: RegisterLength = 0b_1001_0000;
pub const R10: RegisterLength = 0b_1010_0000;
pub const R11: RegisterLength = 0b_1011_0000;
pub const R12: RegisterLength = 0b_1100_0000;
pub const PC: RegisterLength = 0b_1101_0000;
pub const SP: RegisterLength = 0b_1110_0000;
pub const FP: RegisterLength = 0b_1111_0000;

pub const B1: RegisterLength = 0b_0000_0001;
pub const B2: RegisterLength = 0b_0000_0010;
pub const B3: RegisterLength = 0b_0000_0011;
pub const B4: RegisterLength = 0b_0000_0100;
pub const B5: RegisterLength = 0b_0000_0101;
pub const B6: RegisterLength = 0b_0000_0110;
pub const B7: RegisterLength = 0b_0000_0111;
pub const B8: RegisterLength = 0b_0000_1000;
pub const Q1: RegisterLength = 0b_0000_1001;
pub const Q2: RegisterLength = 0b_0000_1010;
pub const Q3: RegisterLength = 0b_0000_1011;
pub const Q4: RegisterLength = 0b_0000_1100;
pub const L: RegisterLength = 0b_0000_1101;
pub const H: RegisterLength = 0b_0000_1110;

/// denotes second tier registers (bad name)
pub const SEC: RegisterLength = 0b_0000_1111;

/// denotes unitialized memory (FP|SEC)
pub const UNINIT: RegisterLength = 0xff;

macro_rules! define_register {
    ($n:literal) => {
        paste! {
            pub const [<R $n B1>]: RegisterLength = [<R $n>] | B1;
            pub const [<R $n B2>]: RegisterLength = [<R $n>] | B2;
            pub const [<R $n B3>]: RegisterLength = [<R $n>] | B3;
            pub const [<R $n B4>]: RegisterLength = [<R $n>] | B4;
            pub const [<R $n B5>]: RegisterLength = [<R $n>] | B5;
            pub const [<R $n B6>]: RegisterLength = [<R $n>] | B6;
            pub const [<R $n B7>]: RegisterLength = [<R $n>] | B7;
            pub const [<R $n B8>]: RegisterLength = [<R $n>] | B8;

            pub const [<R $n Q1>]: RegisterLength = [<R $n>] | Q1;
            pub const [<R $n Q2>]: RegisterLength = [<R $n>] | Q2;
            pub const [<R $n Q3>]: RegisterLength = [<R $n>] | Q3;
            pub const [<R $n Q4>]: RegisterLength = [<R $n>] | Q4;

            pub const [<R $n L>]: RegisterLength = [<R $n>] | L;
            pub const [<R $n H>]: RegisterLength = [<R $n>] | H;
        }
    };
}

define_register!(1);
define_register!(2);
define_register!(3);
define_register!(4);
define_register!(5);
define_register!(6);
define_register!(7);
define_register!(8);
define_register!(9);
define_register!(10);
define_register!(11);
define_register!(12);
