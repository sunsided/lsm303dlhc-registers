use crate::accel::*;
use core::ops::Add;

impl Add<OutXHighA> for OutXLowA {
    type Output = i16;

    fn add(self, hi: OutXHighA) -> Self::Output {
        (hi.bits() as i16) << 8 | (self.bits() as i16)
    }
}

impl Add<OutXLowA> for OutXHighA {
    type Output = i16;

    fn add(self, lo: OutXLowA) -> Self::Output {
        lo.add(self)
    }
}

impl Add<OutYHighA> for OutYLowA {
    type Output = i16;

    fn add(self, hi: OutYHighA) -> Self::Output {
        (hi.bits() as i16) << 8 | (self.bits() as i16)
    }
}

impl Add<OutYLowA> for OutYHighA {
    type Output = i16;

    fn add(self, lo: OutYLowA) -> Self::Output {
        lo.add(self)
    }
}

impl Add<OutZHighA> for OutZLowA {
    type Output = i16;

    fn add(self, hi: OutZHighA) -> Self::Output {
        (hi.bits() as i16) << 8 | (self.bits() as i16)
    }
}

impl Add<OutZLowA> for OutZHighA {
    type Output = i16;

    fn add(self, lo: OutZLowA) -> Self::Output {
        lo.add(self)
    }
}
