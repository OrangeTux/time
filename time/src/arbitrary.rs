use crate::{
    duration,
    time::{Hours, Minutes, Nanoseconds, Seconds},
    Date, Duration, PrimitiveDateTime, Time,
};
use arbitrary::{Arbitrary, Result, Unstructured};

impl<'a> Arbitrary<'a> for Date {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let day = u.int_in_range(Self::MIN.to_julian_day()..=Self::MAX.to_julian_day())?;
        Result::Ok(Self::from_julian_day_unchecked(day))
    }
}

impl<'a> Arbitrary<'a> for Duration {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let random =
            u.int_in_range(duration::Nanoseconds::MIN.get()..=duration::Nanoseconds::MAX.get())?;
        // SAFETY:
        // The call to `duration::Nanoseconds::new_cheched()` is fine.
        //
        // `duration::Nanoseconds` is an alias to `deranged::RangedI32`. The docs of `RangedI32::new_unchecked()`
        // state:
        //
        // "The value must be within the range MIN..=MAX."
        let nanoseconds = unsafe { duration::Nanoseconds::new_unchecked(random) };
        Result::Ok(Self::new_ranged(<_>::arbitrary(u)?, nanoseconds))
    }
}

impl<'a> Arbitrary<'a> for Time {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let random = u.int_in_range(Hours::MIN.get()..=Hours::MAX.get())?;

        // SAFETY:
        // The call to `Hours::new_cheched()` is fine.
        //
        // `Hours` is an alias to `deranged::RangedI8`. The docs of `RangedI8::new_unchecked()`
        // state:
        //
        // "The value must be within the range MIN..=MAX."
        let hours = unsafe { Hours::new_unchecked(random) };

        let random = u.int_in_range(Minutes::MIN.get()..=Minutes::MAX.get())?;
        // SAFETY:
        // The call to `Minutes::new_cheched()` is fine.
        //
        // `Minutes` is an alias to `deranged::RangedI8`. The docs of `RangedI8::new_unchecked()`
        // state:
        //
        // "The value must be within the range MIN..=MAX."
        let minutes = unsafe { Minutes::new_unchecked(random) };

        let random = u.int_in_range(Seconds::MIN.get()..=Seconds::MAX.get())?;
        // SAFETY:
        // The call to `Seconds::new_cheched()` is fine.
        //
        // `Seconds` is an alias to `deranged::RangedI8`. The docs of `RangedI8::new_unchecked()`
        // state:
        //
        // "The value must be within the range MIN..=MAX."
        let seconds = unsafe { Seconds::new_unchecked(random) };

        let random = u.int_in_range(Nanoseconds::MIN.get()..=Nanoseconds::MAX.get())?;
        // SAFETY:
        // The call to `Nanoseconds::new_cheched()` is fine.
        //
        // `Nanoseconds` is an alias to `deranged::RangedI32`. The docs of `RangedI32::new_unchecked()`
        // state:
        //
        // "The value must be within the range MIN..=MAX."
        let nanoseconds = unsafe { Nanoseconds::new_unchecked(random) };
        Ok(Self::from_hms_nanos_ranged(
            hours,
            minutes,
            seconds,
            nanoseconds,
        ))
    }
}

//impl<'a> Arbitrary<'a> for PrimitiveDateTime {
//fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
//let random =
//u.int_in_range(duration::Nanoseconds::MIN.get()..=duration::Nanoseconds::MAX.get())?;
//// SAFETY:
//// The call to `duration::Nanoseconds::new_cheched()` is fine.
////
//// `duration::Nanoseconds` is an alias to `deranged::RangedI32`. The docs of `RangedI32::new_unchecked()`
//// state:
////
//// "The value must be within the range MIN..=MAX."
//let nanoseconds = unsafe { duration::Nanoseconds::new_unchecked(random) };
//Result::Ok(Self::new_ranged(<_>::arbitrary(u)?, nanoseconds))
//}
//}
