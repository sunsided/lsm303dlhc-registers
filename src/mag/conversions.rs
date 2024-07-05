use crate::mag::*;
use core::ops::Add;

impl Add<OutXHighM> for OutXLowM {
    type Output = i16;

    fn add(self, hi: OutXHighM) -> Self::Output {
        (hi.bits() as i16) << 8 | (self.bits() as i16)
    }
}

impl Add<OutXLowM> for OutXHighM {
    type Output = i16;

    fn add(self, lo: OutXLowM) -> Self::Output {
        lo.add(self)
    }
}

impl Add<OutYHighM> for OutYLowM {
    type Output = i16;

    fn add(self, hi: OutYHighM) -> Self::Output {
        (hi.bits() as i16) << 8 | (self.bits() as i16)
    }
}

impl Add<OutYLowM> for OutYHighM {
    type Output = i16;

    fn add(self, lo: OutYLowM) -> Self::Output {
        lo.add(self)
    }
}

impl Add<OutZHighM> for OutZLowM {
    type Output = i16;

    fn add(self, hi: OutZHighM) -> Self::Output {
        (hi.bits() as i16) << 8 | (self.bits() as i16)
    }
}

impl Add<OutZLowM> for OutZHighM {
    type Output = i16;

    fn add(self, lo: OutZLowM) -> Self::Output {
        lo.add(self)
    }
}
