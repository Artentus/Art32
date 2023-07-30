#[macro_use]
extern crate static_assertions;

mod cpu;
mod memory;
mod system;

use system::Art32;

trait Ashr<Rhs = Self> {
    type Output;

    fn ashr(self, rhs: Rhs) -> Self::Output;
}

impl Ashr for u32 {
    type Output = Self;

    #[inline]
    fn ashr(self, rhs: Self) -> Self::Output {
        ((self as i32) >> rhs) as u32
    }
}

macro_rules! shuffle_bits {
    ($input:ident { [$src_end:literal : $src_start:literal] => [$dst_end:literal : $dst_start:literal] $(,)? }) => {{
        const_assert!($src_start >= 0);
        const_assert!($dst_start >= 0);
        const_assert!($src_end >= $src_start);
        const_assert!($dst_end >= $dst_start);
        const_assert_eq!($src_end - $src_start, $dst_end - $dst_start);

        let mask = !((!0) << ($src_end - $src_start + 1));
        (($input >> $src_start) & mask) << $dst_start
    }};
    ($input:ident { [$src:literal] => [$dst:literal] $(,)? }) => {{
        const_assert!($src >= 0);
        const_assert!($dst >= 0);

        (($input >> $src) & 0x1) << $dst
    }};
    ($input:ident { sign [$src:literal] => [$dst:literal] $(,)? }) => {{
        const_assert!($src >= 0);
        const_assert!($dst >= 0);

        let bit = ($input >> $src) & 0x1;
        let sign = (!bit).wrapping_add(1);
        sign << $dst
    }};
    ($input:ident { [$src_end:literal : $src_start:literal] => [$dst_end:literal : $dst_start:literal], $($t:tt)+ }) => {
        $crate::shuffle_bits!($input { [$src_end : $src_start] => [$dst_end : $dst_start] })
        | $crate::shuffle_bits!($input { $($t)+ })
    };
    ($input:ident { [$src:literal] => [$dst:literal], $($t:tt)+ }) => {
        $crate::shuffle_bits!($input { [$src] => [$dst] })
        | $crate::shuffle_bits!($input { $($t)+ })
    };
    ($input:ident { sign [$src:literal] => [$dst:literal], $($t:tt)+ }) => {
        $crate::shuffle_bits!($input { sign [$src] => [$dst] })
        | $crate::shuffle_bits!($input { $($t)+ })
    };
}

use shuffle_bits;

fn main() {
    let mut art32 = Art32::new();
    art32.reset();
    art32.step();
}
