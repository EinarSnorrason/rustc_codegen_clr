#![recursion_limit = "1024"]
    #![feature(custom_mir, core_intrinsics, const_hash)]
    #![allow(unused_parens, unused_assignments, overflowing_literals,internal_features)]
    extern crate core;
    use core::intrinsics::mir::*;

    use std::ffi::{c_char, c_int};

    extern "C" {
        fn printf(fmt: *const c_char, ...) -> c_int;
    }
    trait PrintFDebug{
        unsafe fn printf_debug(&self);
    }
    impl<T:PrintFDebug> PrintFDebug for *const T{
        unsafe fn printf_debug(&self){
            unsafe{(**self).printf_debug()};
        }
    }
    impl<T:PrintFDebug> PrintFDebug for *mut T{
        unsafe fn printf_debug(&self){
            unsafe{(**self).printf_debug()};
        }
    }
    impl<T:PrintFDebug> PrintFDebug for &T{
        unsafe fn printf_debug(&self){
            (**self).printf_debug();
        }
    }
    impl<T:PrintFDebug> PrintFDebug for &mut T{
        unsafe fn printf_debug(&self){
            (**self).printf_debug();
        }
    }
    impl PrintFDebug for i8{
        unsafe fn printf_debug(&self){
            printf("%i\0".as_ptr() as *const c_char,*self as i8 as c_int);
        }
    }
    impl PrintFDebug for u8{
        unsafe fn printf_debug(&self){
            printf("%u\0".as_ptr() as *const c_char,*self as u8 as c_int);
        }
    } 
    impl PrintFDebug for i16{
        unsafe fn printf_debug(&self){
            printf("%i\0".as_ptr() as *const c_char,*self as i16 as c_int);
        }
    }
    impl PrintFDebug for u16{
        unsafe fn printf_debug(&self){
            printf("%u\0".as_ptr() as *const c_char,*self as u16 as c_int);
        }
    } 
    impl PrintFDebug for i32{
        unsafe fn printf_debug(&self){
            printf("%i\0".as_ptr() as *const c_char,*self);
        }
    }
    impl PrintFDebug for f32{
        unsafe fn printf_debug(&self){
            printf("%f\0".as_ptr() as *const c_char,*self as core::ffi::c_double);
        }
    }
    impl PrintFDebug for f64{
        unsafe fn printf_debug(&self){
            printf("%f\0".as_ptr() as *const c_char,*self as core::ffi::c_double);
        }
    }
    impl<T:PrintFDebug,const N:usize> PrintFDebug for [T;N]{
        unsafe fn printf_debug(&self){
            printf("[\0".as_ptr() as *const c_char);
            for b in self{
                b.printf_debug();
                printf(",\0".as_ptr() as *const c_char);
            }
            printf("]\0".as_ptr() as *const c_char);
        }
    }
    impl PrintFDebug for u32{
        unsafe fn printf_debug(&self){
            printf("%u\0".as_ptr() as *const c_char,*self);
        }
    } 
    impl PrintFDebug for char{
        unsafe fn printf_debug(&self){
            printf("%u\0".as_ptr() as *const c_char,*self as u64);
        }
    } 
    impl PrintFDebug for i64{
        unsafe fn printf_debug(&self){
            printf("%li\0".as_ptr() as *const c_char,*self);
        }
    }
    impl PrintFDebug for u64{
        unsafe fn printf_debug(&self){
            printf("%lu\0".as_ptr() as *const c_char,*self);
        }
    } 
    impl PrintFDebug for i128{
        unsafe fn printf_debug(&self){
            u128::printf_debug(&(*self as u128));
        }
    } 
    impl PrintFDebug for u128{
        unsafe fn printf_debug(&self){
            printf("%lx%lx\0".as_ptr() as *const c_char, (*self >> 64) as u64,*self as u64);
        }
    } 
    impl PrintFDebug for isize{
        unsafe fn printf_debug(&self){
            printf("%li\0".as_ptr() as *const c_char,*self as isize);
        }
    }
    impl PrintFDebug for usize{
        unsafe fn printf_debug(&self){
            printf("%lu\0".as_ptr() as *const c_char,*self as usize);
        }
    } 
    impl PrintFDebug for bool{
        unsafe fn printf_debug(&self){
            if *self{
                printf("true\0".as_ptr() as *const c_char);
            }
            else{
                printf("false\0".as_ptr() as *const c_char);
            }
        }
    } 
    impl PrintFDebug for (){
        unsafe fn printf_debug(&self){
            printf("()\0".as_ptr() as *const c_char);
        }
    } 
    impl<A:PrintFDebug> PrintFDebug for (A,){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",)\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug> PrintFDebug for (A,B){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug> PrintFDebug for (A,B,C){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug> PrintFDebug for (A,B,C,D){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug> PrintFDebug for (A,B,C,D,E){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug> PrintFDebug for (A,B,C,D,E,F){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.5.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.5.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.6.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.5.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.6.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.7.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.5.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.6.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.7.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.8.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.5.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.6.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.7.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.8.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.9.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug,K:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J,K){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.5.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.6.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.7.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.8.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.9.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.10.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    impl<A:PrintFDebug,B:PrintFDebug,C:PrintFDebug,D:PrintFDebug,E:PrintFDebug,F:PrintFDebug,G:PrintFDebug,H:PrintFDebug,I:PrintFDebug,J:PrintFDebug,K:PrintFDebug,L:PrintFDebug> PrintFDebug for (A,B,C,D,E,F,G,H,I,J,K,L){
        unsafe fn printf_debug(&self){
            printf("(\0".as_ptr() as *const c_char);
            self.0.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.1.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.2.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.3.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.4.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.5.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.6.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.7.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.8.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.9.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.10.printf_debug();
            printf(",\0".as_ptr() as *const c_char);
            self.11.printf_debug();
            printf(")\0".as_ptr() as *const c_char);
        }
    }
    #[inline(never)]
    fn dump_var(
        f: usize,
        var0: usize, val0: impl PrintFDebug,
        var1: usize, val1: impl PrintFDebug,
        var2: usize, val2: impl PrintFDebug,
        var3: usize, val3: impl PrintFDebug,
    ) {
        unsafe{
            printf("fn%u:_%u = \0".as_ptr() as *const c_char,f,var0);
            val0.printf_debug();
            printf("\n_%u = \0".as_ptr() as *const c_char,var1);
            val1.printf_debug();
            printf("\n_%u = \0".as_ptr() as *const c_char,var2);
            val2.printf_debug();
            printf("\n_%u = \0".as_ptr() as *const c_char,var3);
            val3.printf_debug();
            printf("\n\0".as_ptr() as *const c_char);
        }
    }
    #[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn0(mut _1: u32,mut _2: isize) -> char {
mir! {
type RET = char;
let _3: usize;
let _4: u128;
let _5: [i128; 7];
let _6: (*mut i8, i8, *mut i8, char, f32, *mut i128);
let _7: isize;
let _8: Adt55;
let _9: bool;
let _10: i8;
let _11: bool;
let _12: f32;
let _13: [i64; 1];
let _14: [i32; 3];
let _15: Adt54;
let _16: char;
let _17: &'static i8;
let _18: u64;
let _19: [i64; 1];
let _20: bool;
let _21: isize;
let _22: bool;
let _23: [i64; 1];
let _24: ();
let _25: ();
{
_2 = (-9223372036854775808_isize);
RET = '\u{56559}';
_1 = !1933766663_u32;
_2 = -(-9223372036854775808_isize);
_1 = 56796_u16 as u32;
_1 = 3419834267_u32;
_2 = !9223372036854775807_isize;
_2 = (-9223372036854775808_isize);
_2 = 9736152453652277768_u64 as isize;
_2 = RET as isize;
_1 = 2018014287_u32 ^ 3734689788_u32;
_4 = 5524235832208386799_u64 as u128;
_1 = RET as u32;
RET = '\u{21bfb}';
_2 = (-3_isize) >> _4;
_3 = 4_usize + 3457534971913447257_usize;
_4 = 195644134294653421032840915750075678063_u128 | 106187246695824782681940087648228639772_u128;
_4 = 40489814198003391809937146647687003050_u128;
_1 = 4246670321_u32;
_3 = !9236680015907976719_usize;
match _4 {
0 => bb1,
1 => bb2,
2 => bb3,
40489814198003391809937146647687003050 => bb5,
_ => bb4
}
}
bb1 = {
Return()
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
Return()
}
bb5 = {
RET = '\u{74b54}';
_4 = 213135784557330641767027057489470732094_u128 | 174264854635560452049066591777377412555_u128;
_1 = 2649549135_u32 + 1809475932_u32;
RET = '\u{e3dbd}';
_1 = 1237264596_u32 - 2239212099_u32;
RET = '\u{64809}';
_1 = (-668692550_i32) as u32;
_5 = [37192364892284958286742356972721252354_i128,109199277080829010328060502834578986873_i128,(-124014178428230494819422889293107000282_i128),109614763543913941334889953381102905333_i128,(-94789503391665030643843186689298701547_i128),13993819077245644882138600192645255459_i128,124966450672690207259732906378027725121_i128];
_1 = (-19508952_i32) as u32;
Call(_4 = fn1(_5, _2, _5, _5, _5, _2, _5, _2, _5, _5), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_3 = 882190465560846840_usize * 2572416798705244911_usize;
_6.2 = core::ptr::addr_of_mut!(_6.1);
_3 = 3980_i16 as usize;
_6.4 = _3 as f32;
_2 = (-28421_i16) as isize;
_6.1 = -117_i8;
_6.2 = core::ptr::addr_of_mut!(_6.1);
RET = '\u{ccb34}';
_6.4 = _4 as f32;
_6.0 = core::ptr::addr_of_mut!(_6.1);
_6.3 = RET;
_7 = _6.3 as isize;
_3 = 10443952480299321827_usize;
_10 = _6.1;
_6.0 = _6.2;
_6.2 = core::ptr::addr_of_mut!(_10);
_6.3 = RET;
_7 = _6.4 as isize;
_2 = _7 | _7;
_5 = [(-74445796575394896086167770399014648494_i128),138150493862248107860184006970609287391_i128,4273946260599119909133206049372211837_i128,82130908694289884481136945361608961263_i128,3086452035186098774448633089683021843_i128,91084222088055812409794758192278735898_i128,52066171013259957013271425260540808596_i128];
_11 = !false;
_11 = false ^ false;
_6.3 = RET;
_7 = (-6314878005777613449_i64) as isize;
_10 = -_6.1;
Goto(bb7)
}
bb7 = {
_11 = _6.3 <= _6.3;
_2 = _7 - _7;
_4 = 296481168681490096973052692053479965794_u128;
_10 = (-4761559634293179363_i64) as i8;
_5 = [160121867400017563363142915546524872874_i128,71918751807217464098324437261111809399_i128,(-43810641703547580364447339065272940036_i128),83232914028334296093123192129419206065_i128,(-162587366607442282397871914646207058456_i128),(-28217100253102861239074966798436006728_i128),(-150689465803261866204163959285870083174_i128)];
_4 = 277635200864491531084541858930911389034_u128 + 305664091799066117177944658343208455147_u128;
_9 = _10 <= _6.1;
_3 = 4594952243975932477_i64 as usize;
_6.4 = 100851320820231744592578096380756348072_i128 as f32;
_6.2 = _6.0;
_6.0 = core::ptr::addr_of_mut!(_6.1);
_9 = _11;
_11 = _9;
Goto(bb8)
}
bb8 = {
_13 = [(-5772664711866314736_i64)];
_6.3 = RET;
_4 = !210395819904967665183705746108117476094_u128;
_9 = _10 <= _10;
_10 = _6.1 << _2;
_6.1 = _10 >> _1;
_13 = [(-2034792284935338931_i64)];
_14 = [(-304027534_i32),(-1053870770_i32),1581769183_i32];
RET = _6.3;
_1 = _6.3 as u32;
_1 = 15575_u16 as u32;
_1 = 1547457330_u32;
_13 = [(-4915247145417198297_i64)];
_15.fld0 = [_1,_1,_1,_1,_1,_1];
_6.1 = _10;
_16 = _6.3;
_6.2 = _6.0;
RET = _16;
_9 = !_11;
RET = _6.3;
RET = _6.3;
_14 = [(-69801675_i32),(-922720530_i32),(-668023596_i32)];
_12 = _6.4;
Goto(bb9)
}
bb9 = {
_6.0 = core::ptr::addr_of_mut!(_6.1);
_2 = _7;
_9 = _11 | _11;
_9 = !_11;
_13 = [411273445842632799_i64];
_6.4 = 13335_u16 as f32;
_6.1 = !_10;
_2 = _7;
_11 = !_9;
_9 = _11;
_6.3 = RET;
RET = _6.3;
_10 = 10487391722639589907_u64 as i8;
_9 = !_11;
_15.fld1 = [(-67214822498956807413093975486026370371_i128),(-120946219575074712692868194670634758611_i128),93550813164241532859943343470753198791_i128,9828011909034991954216999984015200790_i128,(-36803521294032063332828332925228139722_i128),24169720794677789623853235187857293419_i128,(-15275020885914243918187425714421015901_i128),116167585304354340034980902437343995490_i128];
_5 = [(-118761247597629508714016897047663415269_i128),(-123410932025297919515700859027693388441_i128),(-67677639389616323458994761462483737159_i128),(-75690518599510899485780782687660124943_i128),22106504634793135240900778902144157840_i128,3377675877802942918308863227523834115_i128,30435555214471849300896154773308508419_i128];
_18 = 9030884089861683610_u64;
_19 = [(-3737230327174671863_i64)];
_14 = [(-440992017_i32),(-309580229_i32),(-906699143_i32)];
_2 = !_7;
_11 = !_9;
RET = _16;
match _18 {
0 => bb7,
1 => bb8,
2 => bb3,
9030884089861683610 => bb11,
_ => bb10
}
}
bb10 = {
_11 = _6.3 <= _6.3;
_2 = _7 - _7;
_4 = 296481168681490096973052692053479965794_u128;
_10 = (-4761559634293179363_i64) as i8;
_5 = [160121867400017563363142915546524872874_i128,71918751807217464098324437261111809399_i128,(-43810641703547580364447339065272940036_i128),83232914028334296093123192129419206065_i128,(-162587366607442282397871914646207058456_i128),(-28217100253102861239074966798436006728_i128),(-150689465803261866204163959285870083174_i128)];
_4 = 277635200864491531084541858930911389034_u128 + 305664091799066117177944658343208455147_u128;
_9 = _10 <= _6.1;
_3 = 4594952243975932477_i64 as usize;
_6.4 = 100851320820231744592578096380756348072_i128 as f32;
_6.2 = _6.0;
_6.0 = core::ptr::addr_of_mut!(_6.1);
_9 = _11;
_11 = _9;
Goto(bb8)
}
bb11 = {
_15.fld0 = [_1,_1,_1,_1,_1,_1];
_5 = [(-125892704871924254862404950802124811687_i128),(-6395984953042416066117540364491096798_i128),124153284854040267297651353058294621087_i128,(-21023204844217937404649919847072571387_i128),(-87397001723474963017206565328978576509_i128),91981562008493954792399300873088802179_i128,(-25647026726123827296443466660936362139_i128)];
_5 = [(-6397415917814304375082076824270909096_i128),(-22540840886533219117318083743239295293_i128),156336646025284440540191372386427473848_i128,86722605379813244203121076266944420288_i128,111308333177687520688393902265354189258_i128,81110352997362362259223678986246993208_i128,125581949363662465782981856914510509553_i128];
_6.4 = _12;
_15.fld1 = [24421885465100960358166055608289579245_i128,(-119348353384696418013517034122850290793_i128),(-47204724955046073297423934432948149030_i128),167372585203284646316716094946965231619_i128,(-39547110730131015398142827371092625078_i128),64613400176559711609814707487282813899_i128,133745359964533428015553506724302757424_i128,(-116045054419525826285726809004356704220_i128)];
RET = _6.3;
_6.1 = _10 | _10;
_3 = 6_usize * 14630731355690690719_usize;
Goto(bb12)
}
bb12 = {
_15.fld1 = [89299048758601457821346631692593022628_i128,(-161915806867503923543155453776345990566_i128),145479557554657227409335669452734834985_i128,142138697011678149238504714383093375317_i128,141089676509760673656281667115787286560_i128,93428360058274493246916433265493622110_i128,40128194527036254809612088145735271620_i128,11652314159760541517870492852793114725_i128];
_18 = !12041540239228790681_u64;
RET = _16;
_2 = _12 as isize;
_5 = [142129373091723786209448257964051326966_i128,(-5645591294118203122868604831734840371_i128),12371801943301742200707711986619665891_i128,(-56011137141487268361617606138862911667_i128),47113395580527893499025092804857777495_i128,20991722529657019390141158608352556936_i128,122996048839331922184677215164327466705_i128];
_10 = _6.1 & _6.1;
RET = _6.3;
_15.fld0 = [_1,_1,_1,_1,_1,_1];
_5 = [25301520141678419170272804374519640830_i128,(-2800805231674063124132725151445501376_i128),114749202572814570031537214020720536379_i128,(-114415988374247545162577912887810205723_i128),(-144529659645130985122032150326215779364_i128),(-101828546582296500301793060661222251617_i128),51710129559269189072908888918399722883_i128];
_6.1 = _10 ^ _10;
match _1 {
0 => bb3,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
1547457330 => bb18,
_ => bb17
}
}
bb13 = {
_15.fld0 = [_1,_1,_1,_1,_1,_1];
_5 = [(-125892704871924254862404950802124811687_i128),(-6395984953042416066117540364491096798_i128),124153284854040267297651353058294621087_i128,(-21023204844217937404649919847072571387_i128),(-87397001723474963017206565328978576509_i128),91981562008493954792399300873088802179_i128,(-25647026726123827296443466660936362139_i128)];
_5 = [(-6397415917814304375082076824270909096_i128),(-22540840886533219117318083743239295293_i128),156336646025284440540191372386427473848_i128,86722605379813244203121076266944420288_i128,111308333177687520688393902265354189258_i128,81110352997362362259223678986246993208_i128,125581949363662465782981856914510509553_i128];
_6.4 = _12;
_15.fld1 = [24421885465100960358166055608289579245_i128,(-119348353384696418013517034122850290793_i128),(-47204724955046073297423934432948149030_i128),167372585203284646316716094946965231619_i128,(-39547110730131015398142827371092625078_i128),64613400176559711609814707487282813899_i128,133745359964533428015553506724302757424_i128,(-116045054419525826285726809004356704220_i128)];
RET = _6.3;
_6.1 = _10 | _10;
_3 = 6_usize * 14630731355690690719_usize;
Goto(bb12)
}
bb14 = {
Return()
}
bb15 = {
_6.0 = core::ptr::addr_of_mut!(_6.1);
_2 = _7;
_9 = _11 | _11;
_9 = !_11;
_13 = [411273445842632799_i64];
_6.4 = 13335_u16 as f32;
_6.1 = !_10;
_2 = _7;
_11 = !_9;
_9 = _11;
_6.3 = RET;
RET = _6.3;
_10 = 10487391722639589907_u64 as i8;
_9 = !_11;
_15.fld1 = [(-67214822498956807413093975486026370371_i128),(-120946219575074712692868194670634758611_i128),93550813164241532859943343470753198791_i128,9828011909034991954216999984015200790_i128,(-36803521294032063332828332925228139722_i128),24169720794677789623853235187857293419_i128,(-15275020885914243918187425714421015901_i128),116167585304354340034980902437343995490_i128];
_5 = [(-118761247597629508714016897047663415269_i128),(-123410932025297919515700859027693388441_i128),(-67677639389616323458994761462483737159_i128),(-75690518599510899485780782687660124943_i128),22106504634793135240900778902144157840_i128,3377675877802942918308863227523834115_i128,30435555214471849300896154773308508419_i128];
_18 = 9030884089861683610_u64;
_19 = [(-3737230327174671863_i64)];
_14 = [(-440992017_i32),(-309580229_i32),(-906699143_i32)];
_2 = !_7;
_11 = !_9;
RET = _16;
match _18 {
0 => bb7,
1 => bb8,
2 => bb3,
9030884089861683610 => bb11,
_ => bb10
}
}
bb16 = {
_13 = [(-5772664711866314736_i64)];
_6.3 = RET;
_4 = !210395819904967665183705746108117476094_u128;
_9 = _10 <= _10;
_10 = _6.1 << _2;
_6.1 = _10 >> _1;
_13 = [(-2034792284935338931_i64)];
_14 = [(-304027534_i32),(-1053870770_i32),1581769183_i32];
RET = _6.3;
_1 = _6.3 as u32;
_1 = 15575_u16 as u32;
_1 = 1547457330_u32;
_13 = [(-4915247145417198297_i64)];
_15.fld0 = [_1,_1,_1,_1,_1,_1];
_6.1 = _10;
_16 = _6.3;
_6.2 = _6.0;
RET = _16;
_9 = !_11;
RET = _6.3;
RET = _6.3;
_14 = [(-69801675_i32),(-922720530_i32),(-668023596_i32)];
_12 = _6.4;
Goto(bb9)
}
bb17 = {
Return()
}
bb18 = {
RET = _16;
_6.2 = core::ptr::addr_of_mut!(_6.1);
_10 = _6.1 - _6.1;
_6.2 = core::ptr::addr_of_mut!(_10);
_20 = !_9;
_6.4 = 59538_u16 as f32;
_20 = !_11;
_13 = [(-267323335115554514_i64)];
_21 = _2 | _2;
_4 = 82940947263042655855533337379400401901_u128 | 58046012974194629908497455153857297575_u128;
_12 = -_6.4;
_7 = _21 + _21;
_17 = &_10;
_3 = !5_usize;
_22 = _11 & _9;
_19 = [4596143697528394391_i64];
_5 = [(-106993533340359626581844616763022723681_i128),148844762521780191627071539407954315473_i128,76214082736521980894312076997445359694_i128,140517112014110194856196548522407485171_i128,161381384998442259085478755058661547920_i128,(-100149767882566937393288851233662393097_i128),165798155793783388943534210417041480166_i128];
_14 = [1862573658_i32,2002011133_i32,(-703920637_i32)];
_22 = !_11;
_6.1 = (*_17);
Goto(bb19)
}
bb19 = {
Call(_24 = dump_var(0_usize, 13_usize, Move(_13), 11_usize, Move(_11), 4_usize, Move(_4), 18_usize, Move(_18)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_24 = dump_var(0_usize, 10_usize, Move(_10), 16_usize, Move(_16), 14_usize, Move(_14), 21_usize, Move(_21)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn1(mut _1: [i128; 7],mut _2: isize,mut _3: [i128; 7],mut _4: [i128; 7],mut _5: [i128; 7],mut _6: isize,mut _7: [i128; 7],mut _8: isize,mut _9: [i128; 7],mut _10: [i128; 7]) -> u128 {
mir! {
type RET = u128;
let _11: f32;
let _12: Adt48;
let _13: [i32; 3];
let _14: (char, &'static i8);
let _15: [u8; 5];
let _16: [i64; 1];
let _17: (char, &'static i8);
let _18: [u32; 6];
let _19: Adt54;
let _20: [u32; 6];
let _21: isize;
let _22: [i64; 1];
let _23: bool;
let _24: [i32; 3];
let _25: [char; 8];
let _26: Adt47;
let _27: ();
let _28: ();
{
_10 = _5;
_2 = _6;
_10 = [98621999514435568587750489463690809727_i128,(-99245204282492281352598485044921700800_i128),123195247916306299244457914930308910574_i128,19845929679645459463505575652358129251_i128,(-141760426707784508069497637174375976386_i128),101183376256116863038602516309706961143_i128,132647004514283564756462535461246461239_i128];
_1 = [138254615933452060812096280309605468304_i128,(-133633049242647957696325755872051070803_i128),3766167520531379586191867289170822292_i128,(-51760351926057939886237418372521329843_i128),(-72618122055624897004602573492862472544_i128),(-11889034004240813612076477092058644655_i128),123324238681414731338226892776770691391_i128];
_6 = 26558_i16 as isize;
_5 = _3;
RET = 177172832799329865529447721623523952651_u128 ^ 100367110376853147221977916282796851181_u128;
_6 = (-54338230798853928926249841342568197546_i128) as isize;
_5 = [(-50095950254242908882299196009591517878_i128),(-79248896501056121114249380260045498218_i128),66647121692309847484428916211530731387_i128,(-88173144349143314836230005097178443823_i128),(-149465066397439674150911601603995228638_i128),(-143063899696498381894845222826087018356_i128),(-165778052787116937350795691064637416139_i128)];
_7 = [(-91623060471046345353102978635580189220_i128),119371192092526916885754725677005829555_i128,35706749113781143156282461073551283743_i128,(-114918640495556764694827757750225555579_i128),(-33833910422502012956903736582048751992_i128),83294926674369756042677854119764532346_i128,8992538495841723480737819825492079704_i128];
RET = !243705312732981070263756076616091800086_u128;
_6 = _8 & _2;
_5 = _1;
_10 = [(-113336739518816161861067821435767616765_i128),(-140371396581651121124525361892071160003_i128),(-160955361624460913185543629667023274084_i128),(-107623342940341820124504707553094106158_i128),(-134489089313481023763631068120624234362_i128),116467975957957814539196790864150153669_i128,(-61758624266894231105154261429031449570_i128)];
_8 = !_6;
_11 = (-1677889587_i32) as f32;
_9 = [56341683117032544053356901767941843676_i128,(-34450811636344725960604916663567877958_i128),14508579098516075084898904004246279892_i128,(-168101363828303838434174226007397134578_i128),146513171207057382103727685770665256158_i128,(-35149380116696335152984097215635334197_i128),14626865581900475191892996982500482342_i128];
_12.fld3.0 = [(-87406026446458379822936452368998746820_i128),(-150434200850214970645493701163651653598_i128),(-138940835408910389356622650681325359327_i128),32369633596629915681799080668328876921_i128,(-86294479918788349825162286000306339734_i128),29174394332011406011875779144882087942_i128,(-121393698574571717651130347277438674688_i128)];
Call(_12.fld3.2 = fn2(_9, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_12.fld2 = 6174413133582565306_u64 ^ 14643928960927392247_u64;
_12.fld3.4 = false as u8;
_12.fld0.4 = !165094527868959229950259135902775518812_i128;
_12.fld3.1.5 = core::ptr::addr_of_mut!(_12.fld0.4);
_12.fld3.1.3 = '\u{7de3b}';
_12.fld3.3 = [3545_u16,6804_u16];
_12.fld5 = (-732814552_i32) ^ (-1767965097_i32);
RET = (-35976233200490624_i64) as u128;
_12.fld3.1.4 = _11 - _11;
_12.fld3.0 = _7;
_12.fld3.1.3 = '\u{32851}';
_12.fld0.2 = core::ptr::addr_of_mut!(_1);
_12.fld0.1 = 27992_u16;
_1 = [_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4];
_12.fld3.1.1 = (-78_i8) & 47_i8;
_1 = _10;
_12.fld3.3 = [_12.fld0.1,_12.fld0.1];
_12.fld3.1.0 = core::ptr::addr_of_mut!(_12.fld3.1.1);
_12.fld0.4 = _12.fld5 as i128;
_12.fld2 = _8 as u64;
_12.fld4 = _12.fld0.1;
_14.1 = &_12.fld3.1.1;
_12.fld3.1.5 = core::ptr::addr_of_mut!(_12.fld0.4);
_12.fld0.3 = 958915471666444194_i64 as i32;
_12.fld3.1.2 = core::ptr::addr_of_mut!(_12.fld3.1.1);
_12.fld3.0 = [_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4];
_6 = true as isize;
Goto(bb2)
}
bb2 = {
_12.fld3.1.3 = '\u{f5625}';
_12.fld3.1.0 = core::ptr::addr_of_mut!(_12.fld3.1.1);
_12.fld3.2 = 19204_i16;
Goto(bb3)
}
bb3 = {
_10 = _12.fld3.0;
_12.fld3.1.3 = '\u{7d6c1}';
_12.fld1 = [_12.fld3.1.3,_12.fld3.1.3,_12.fld3.1.3,_12.fld3.1.3,_12.fld3.1.3,_12.fld3.1.3,_12.fld3.1.3,_12.fld3.1.3];
match _12.fld0.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
27992 => bb9,
_ => bb8
}
}
bb4 = {
_12.fld3.1.3 = '\u{f5625}';
_12.fld3.1.0 = core::ptr::addr_of_mut!(_12.fld3.1.1);
_12.fld3.2 = 19204_i16;
Goto(bb3)
}
bb5 = {
_12.fld2 = 6174413133582565306_u64 ^ 14643928960927392247_u64;
_12.fld3.4 = false as u8;
_12.fld0.4 = !165094527868959229950259135902775518812_i128;
_12.fld3.1.5 = core::ptr::addr_of_mut!(_12.fld0.4);
_12.fld3.1.3 = '\u{7de3b}';
_12.fld3.3 = [3545_u16,6804_u16];
_12.fld5 = (-732814552_i32) ^ (-1767965097_i32);
RET = (-35976233200490624_i64) as u128;
_12.fld3.1.4 = _11 - _11;
_12.fld3.0 = _7;
_12.fld3.1.3 = '\u{32851}';
_12.fld0.2 = core::ptr::addr_of_mut!(_1);
_12.fld0.1 = 27992_u16;
_1 = [_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4];
_12.fld3.1.1 = (-78_i8) & 47_i8;
_1 = _10;
_12.fld3.3 = [_12.fld0.1,_12.fld0.1];
_12.fld3.1.0 = core::ptr::addr_of_mut!(_12.fld3.1.1);
_12.fld0.4 = _12.fld5 as i128;
_12.fld2 = _8 as u64;
_12.fld4 = _12.fld0.1;
_14.1 = &_12.fld3.1.1;
_12.fld3.1.5 = core::ptr::addr_of_mut!(_12.fld0.4);
_12.fld0.3 = 958915471666444194_i64 as i32;
_12.fld3.1.2 = core::ptr::addr_of_mut!(_12.fld3.1.1);
_12.fld3.0 = [_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4];
_6 = true as isize;
Goto(bb2)
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
_14.0 = _12.fld3.1.3;
_12.fld3.1.4 = RET as f32;
_12.fld2 = !4778268864649560148_u64;
_2 = _6;
RET = 109968012443966144274416538491445052284_u128;
_12.fld3.1.2 = _12.fld3.1.0;
match RET {
0 => bb1,
1 => bb10,
2 => bb11,
109968012443966144274416538491445052284 => bb13,
_ => bb12
}
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_12.fld3.1.3 = '\u{f5625}';
_12.fld3.1.0 = core::ptr::addr_of_mut!(_12.fld3.1.1);
_12.fld3.2 = 19204_i16;
Goto(bb3)
}
bb13 = {
_12.fld3.1.5 = core::ptr::addr_of_mut!(_12.fld0.4);
_15 = [_12.fld3.4,_12.fld3.4,_12.fld3.4,_12.fld3.4,_12.fld3.4];
RET = !205963246152721928132990380003755384668_u128;
_12.fld3.3 = [_12.fld0.1,_12.fld4];
_12.fld4 = _12.fld0.1;
_6 = -_8;
_12.fld0.1 = _12.fld4;
_12.fld0.2 = core::ptr::addr_of_mut!(_1);
_12.fld0.4 = 34899066394252630210948251277756170961_i128 + (-139077705081555076596749957471770701759_i128);
_12.fld0.1 = _12.fld4 & _12.fld4;
_12.fld3.3 = [_12.fld4,_12.fld4];
_15 = [_12.fld3.4,_12.fld3.4,_12.fld3.4,_12.fld3.4,_12.fld3.4];
_15 = [_12.fld3.4,_12.fld3.4,_12.fld3.4,_12.fld3.4,_12.fld3.4];
_12.fld3.1.4 = _11;
_17 = Move(_14);
_13 = [_12.fld0.3,_12.fld5,_12.fld5];
_1 = [_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4];
_21 = -_6;
_14.1 = &_12.fld3.1.1;
_12.fld0.4 = RET as i128;
_11 = _12.fld3.1.4 + _12.fld3.1.4;
Goto(bb14)
}
bb14 = {
_8 = _21;
_12.fld3.4 = !180_u8;
_14.0 = _12.fld3.1.3;
_22 = [(-1972788923937126603_i64)];
_17.0 = _12.fld3.1.3;
_11 = _12.fld3.2 as f32;
_6 = _21;
_12.fld1 = [_17.0,_12.fld3.1.3,_17.0,_14.0,_12.fld3.1.3,_14.0,_12.fld3.1.3,_17.0];
_1 = [_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4];
_8 = _6;
_12.fld3.1.4 = _11;
_12.fld3.1.5 = core::ptr::addr_of_mut!(_12.fld0.4);
_14 = (_17.0, Move(_17.1));
_12.fld3.1.3 = _14.0;
_4 = [_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4,_12.fld0.4];
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(1_usize, 13_usize, Move(_13), 2_usize, Move(_2), 3_usize, Move(_3), 10_usize, Move(_10)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(1_usize, 8_usize, Move(_8), 1_usize, Move(_1), 21_usize, Move(_21), 28_usize, _28), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn2(mut _1: [i128; 7],mut _2: [i128; 7]) -> i16 {
mir! {
type RET = i16;
let _3: u8;
let _4: [u16; 2];
let _5: [u16; 2];
let _6: usize;
let _7: [char; 8];
let _8: f64;
let _9: [i128; 8];
let _10: Adt54;
let _11: *mut i8;
let _12: Adt53;
let _13: f64;
let _14: Adt40;
let _15: Adt53;
let _16: isize;
let _17: [i32; 3];
let _18: i32;
let _19: bool;
let _20: [i32; 3];
let _21: f32;
let _22: u64;
let _23: [u8; 2];
let _24: f32;
let _25: [char; 8];
let _26: [i128; 7];
let _27: bool;
let _28: u32;
let _29: u8;
let _30: i32;
let _31: [u8; 5];
let _32: i16;
let _33: [char; 8];
let _34: f64;
let _35: [u32; 6];
let _36: i64;
let _37: isize;
let _38: ();
let _39: ();
{
RET = '\u{7bd3c}' as i16;
_1 = [13226023099769190958906394134288700873_i128,(-141311747677285804216516867491660986694_i128),(-32156007423699930422548571702321558478_i128),(-100566664068907858689360421824782721555_i128),(-130804940570242719759294138504772069607_i128),154372234280212517293830617332063128350_i128,56444834983962727137374699378191122636_i128];
Call(_1 = fn3(_2, _2, _2, _2, _2, _2, _2, _2, _2, _2, _2, _2, RET, _2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = (-25124_i16) & (-30416_i16);
RET = (-27839_i16);
_2 = _1;
RET = 2211731217106391671_usize as i16;
RET = 9338_i16 << (-123_isize);
RET = 19534_i16 & (-6963_i16);
RET = -(-26490_i16);
RET = false as i16;
RET = (-30246_i16);
_2 = [(-80544086093690795071789568357897906566_i128),(-104621805171486750265635406667522266449_i128),(-52928311413448162538977534234880170481_i128),86686936418170661816822113636334852503_i128,(-82753847132728654557894395535573269477_i128),146545551979975626656960730632868083878_i128,82176468448083758008724998917089032359_i128];
_1 = [119513233448514101907267952493837154498_i128,(-135309096543224147196115487151563367826_i128),(-62953937223177300713953490221856044231_i128),64236655703476984415166113316644590553_i128,31572517804855480835622299941684035760_i128,60017166011288442348571095739360487458_i128,(-57187409541545887165256880414784539060_i128)];
_1 = [121989790760203469293356157888307785439_i128,86514229731868344784392345334023977581_i128,(-160295653638237692291384254044956407825_i128),(-97733368493107899290957840264427027825_i128),90728990131335047150453661763285043671_i128,(-121889949479426817224409930955350353238_i128),(-26859991042959526879913637754931662949_i128)];
_4 = [8800_u16,47233_u16];
RET = 28234_i16;
_3 = 58_u8 >> RET;
_3 = 230_u8 + 217_u8;
RET = !(-25868_i16);
_2 = _1;
RET = '\u{305c1}' as i16;
_2 = [2772355079849586697113484295611108134_i128,34475198163450655645635000452343527434_i128,142077477567521516227460146161082422420_i128,(-152656839500131113549902109474914929740_i128),(-37681595413968470012981095734509796276_i128),20569142931927891918205254642740592732_i128,(-14357159853038638609512565275668028786_i128)];
_8 = 1986426271255076159_i64 as f64;
_3 = 225_u8 << RET;
Goto(bb2)
}
bb2 = {
_5 = _4;
_8 = 9223372036854775807_isize as f64;
_8 = 2972735407781532848_u64 as f64;
_5 = _4;
_9 = [(-139607514985933501121735837987622402063_i128),(-169027150547741546773404987178389994636_i128),(-29097284502960067425887713403033028093_i128),142743258679939860400741754150507849769_i128,38953406804619889546857386090699125276_i128,(-9403647115896798611612728016517320595_i128),40339045847106649629877316278545329287_i128,88204721786162663529408839187213544100_i128];
_5 = _4;
_6 = (-144238661900289697108991527432586293818_i128) as usize;
Goto(bb3)
}
bb3 = {
_4 = [56661_u16,2225_u16];
_7 = ['\u{cf1d6}','\u{acde8}','\u{9b09c}','\u{9eb1c}','\u{936e8}','\u{c0f27}','\u{f63ce}','\u{36374}'];
_8 = 13073969860437234481_u64 as f64;
_5 = _4;
_5 = _4;
_8 = RET as f64;
_7 = ['\u{85bad}','\u{704be}','\u{943c6}','\u{df959}','\u{3f00e}','\u{8a9bb}','\u{108728}','\u{915f2}'];
Goto(bb4)
}
bb4 = {
_1 = _2;
_4 = [61881_u16,12503_u16];
_6 = 7_usize + 5591550653115561257_usize;
_3 = 92_u8 | 252_u8;
_6 = !6966541556312260474_usize;
_6 = 839785703068355850_usize;
_2 = [(-71264491572954350086408605668050710979_i128),(-80579618325532163751782953448887563354_i128),155708383284798109943535284485840609696_i128,(-55199237165863581474574139168932965045_i128),146814255288540864047215437142012128600_i128,(-62473518991086370275726638495178921105_i128),(-55715548129151961571424867668704803993_i128)];
_1 = _2;
_4 = _5;
_3 = 6583686892137401270_u64 as u8;
_6 = _8 as usize;
_4 = [37180_u16,24851_u16];
Goto(bb5)
}
bb5 = {
_1 = [(-85285737365091219843400648057403579394_i128),(-80403391260448564285773762333685293412_i128),(-1301611187886122199103953875349942181_i128),151838089125575682453974725697574168547_i128,(-75261581044941628796812255402503040379_i128),(-161798901329698463943920989505099940279_i128),161335713441696183396404389314264379653_i128];
_1 = [30014230010173694620053935839571923302_i128,46501239787031420834801290255354264601_i128,112366150195840062905603567421957686401_i128,11579395235671272375460707456221274949_i128,(-160428297187297230437471578606319497914_i128),88744641688434456507564994344325421522_i128,37819098565855530255352688077980939888_i128];
_12 = Adt53::Variant2 { fld0: 16370042587035732528_u64 };
_7 = ['\u{10b421}','\u{58823}','\u{979fb}','\u{97219}','\u{61aef}','\u{34647}','\u{931c2}','\u{d6197}'];
_10.fld0 = [2912616941_u32,4231692513_u32,1214625730_u32,111820083_u32,1089656303_u32,2897692443_u32];
_4 = [2929_u16,24727_u16];
_13 = _6 as f64;
_5 = [25979_u16,26934_u16];
_8 = _13 + _13;
place!(Field::<u64>(Variant(_12, 2), 0)) = !15182958828518764281_u64;
_12 = Adt53::Variant2 { fld0: 9499791735848244739_u64 };
_10.fld0 = [1556668608_u32,2761627797_u32,817279783_u32,617050859_u32,554983676_u32,2133972495_u32];
_1 = _2;
_12 = Adt53::Variant2 { fld0: 6853330335147352593_u64 };
_8 = -_13;
_1 = [169944986338992750444170014999662302235_i128,(-115653882883495933786516240263484693306_i128),(-156601843724959738234314447051854236540_i128),(-105639018200184578835563562840462833557_i128),8960775853025175376917503078742452703_i128,(-130106918337259112718712930016688817597_i128),95476906699049649483105710330483870544_i128];
_3 = 204_u8;
RET = 18652_i16;
Goto(bb6)
}
bb6 = {
place!(Field::<u64>(Variant(_12, 2), 0)) = 13269635573282722460_u64;
_19 = true;
_10.fld0 = [591192915_u32,2067519659_u32,2764834526_u32,3758092851_u32,117879144_u32,2117805843_u32];
_12 = Adt53::Variant3 { fld0: '\u{e3f97}' };
_2 = [(-169668875762869807755425767826858606683_i128),(-68679260345316021684711480190624031365_i128),(-113347890093263255974932312525487512316_i128),(-107191396762334444286363619949975828809_i128),(-154370530380351698686201281118983966638_i128),(-123039271387321406257738311873708762287_i128),84893612695715449783407527938944436697_i128];
_2 = [(-76959735640202346912903356814458685041_i128),114015690763643240548425211176795244511_i128,1097063567229661230042767335658705834_i128,(-21283710271591303812157750161628935331_i128),(-120632654778087777091619770595985764278_i128),4359306906805941516871727675368197039_i128,37938304640369135489864851625668005116_i128];
_13 = _8 + _8;
_4 = _5;
_10.fld1 = _9;
_2 = _1;
_16 = (-127_isize) ^ 46_isize;
_4 = [33398_u16,32955_u16];
_10.fld0 = [1397563780_u32,2463663730_u32,2076707718_u32,466329874_u32,3569420701_u32,2003280255_u32];
_2 = [27964920322515550877041600186218665548_i128,77207150966544224292179321040864591504_i128,88338174102368908348693192476712821634_i128,49352086194002493500852848192502585301_i128,158270664817459007884717970840079576305_i128,86271145625784907320522035010186445296_i128,72628784336070983359419481947588141516_i128];
_15 = Adt53::Variant3 { fld0: '\u{3447a}' };
_1 = _2;
_20 = [(-1118219247_i32),1210716454_i32,(-848367554_i32)];
_13 = (-5376785260825477512_i64) as f64;
_13 = _8 * _8;
_17 = [(-1447572387_i32),(-918470635_i32),1046555715_i32];
_12 = Adt53::Variant2 { fld0: 8274840088538969295_u64 };
Goto(bb7)
}
bb7 = {
_22 = !11073688153538288012_u64;
RET = 2565_i16 * (-1544_i16);
RET = 26915_i16;
place!(Field::<u64>(Variant(_12, 2), 0)) = '\u{62737}' as u64;
_8 = _6 as f64;
_17 = [851745630_i32,(-1245785021_i32),(-1031949017_i32)];
_19 = !false;
_1 = _2;
_17 = [(-1330773132_i32),(-690892268_i32),786206462_i32];
_19 = !true;
_7 = ['\u{5e9c8}','\u{89418}','\u{cb5ef}','\u{a2019}','\u{14305}','\u{10a010}','\u{c45d2}','\u{27797}'];
_15 = Adt53::Variant2 { fld0: Field::<u64>(Variant(_12, 2), 0) };
_24 = RET as f32;
_1 = _2;
_25 = ['\u{340b5}','\u{4e1c4}','\u{7ed54}','\u{10714b}','\u{b5022}','\u{7473e}','\u{ca30e}','\u{adf5}'];
_15 = Move(_12);
_16 = -(-9223372036854775808_isize);
_21 = _24 + _24;
Goto(bb8)
}
bb8 = {
_25 = ['\u{9000}','\u{70a78}','\u{86e13}','\u{9ff19}','\u{ada61}','\u{10100e}','\u{6da09}','\u{80f8c}'];
_13 = _8;
_1 = [(-66098909654717185827377605157850274536_i128),125460067109033893102908590197141187746_i128,(-45662370732925530795194761968154577898_i128),(-5088946963255737843809693429563272095_i128),(-40105100476321736341754054315272555099_i128),(-65798064314630764399052378600891273541_i128),168777846698011030672227221273063519627_i128];
_26 = [145669461493152131441486830024843865254_i128,(-111303403558564787702120474754533229430_i128),(-66240862969588687138350271909866154285_i128),164233204384909074973760525774855234540_i128,(-6804969631112983355137338399815084831_i128),145045215559606580192346725329712439889_i128,124539217501499258012643632983533327618_i128];
place!(Field::<u64>(Variant(_15, 2), 0)) = _22;
_25 = ['\u{7a9ec}','\u{e3b1b}','\u{fe4b}','\u{105726}','\u{aed40}','\u{7deaf}','\u{1003a8}','\u{94b37}'];
Goto(bb9)
}
bb9 = {
_12 = Adt53::Variant2 { fld0: _22 };
_5 = _4;
_23 = [_3,_3];
_27 = !_19;
_18 = 1663463744_i32;
_20 = [_18,_18,_18];
Goto(bb10)
}
bb10 = {
_23 = [_3,_3];
_10.fld0 = [2542982702_u32,3804867802_u32,3023674580_u32,4017566526_u32,459722777_u32,1156629999_u32];
Goto(bb11)
}
bb11 = {
_28 = 1301389976_u32 & 686676446_u32;
_1 = [(-131286250681131682931385996983174632405_i128),52530563940101238954917286789697824911_i128,(-166303203699416533796353542593055670591_i128),24552098109330542373828703045688259585_i128,(-109601147712530857497915036829951922638_i128),(-99552579950159292490954112716209014117_i128),97704625978397311197515255007756887739_i128];
_18 = 857295092_i32 * 1403351325_i32;
_29 = !_3;
RET = (-28227_i16);
_20 = _17;
_9 = [93648065884692530669855616838798891697_i128,1126940300304835100677166984052401455_i128,32683230388312045468178007466385024692_i128,(-117091456678593086180480226527464396582_i128),(-57073522462776266447833168406505687331_i128),(-15050849264809775903397879121989330354_i128),127709171413974672549659081478169675643_i128,14541617507903227617853851210416559163_i128];
_18 = (-1858840748_i32);
_23 = [_3,_3];
RET = _22 as i16;
_27 = _19;
_8 = -_13;
_26 = [75228761216732122843928965914452161174_i128,(-2650119370181602066166638353341147706_i128),149684548268163391116938136482978103044_i128,18768338779537926250110234858425358685_i128,(-85670759880211221788678019514132907827_i128),8963446885329327094391521962756583664_i128,(-106713244487327393723334626345079636801_i128)];
_27 = _6 < _6;
_22 = (-3425783870929511876_i64) as u64;
place!(Field::<u64>(Variant(_15, 2), 0)) = 27803976544565319593796602464499176201_i128 as u64;
_15 = Move(_12);
_9 = _10.fld1;
_3 = _29 ^ _29;
Goto(bb12)
}
bb12 = {
_30 = _18 >> _3;
Call(_26 = core::intrinsics::transmute(_1), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
_5 = [31462_u16,34070_u16];
_3 = _29 << RET;
_31 = [_3,_3,_3,_29,_3];
SetDiscriminant(_15, 0);
_28 = 2011012384_u32;
_24 = _21 * _21;
_31 = [_3,_3,_3,_29,_3];
place!(Field::<*mut [u32; 6]>(Variant(_15, 0), 0)) = core::ptr::addr_of_mut!(_10.fld0);
place!(Field::<Adt41>(Variant(_15, 0), 1)).fld0.0 = core::ptr::addr_of_mut!(place!(Field::<Adt41>(Variant(_15, 0), 1)).fld2.1.2);
RET = 28869_i16;
Goto(bb14)
}
bb14 = {
place!(Field::<Adt41>(Variant(_15, 0), 1)).fld1 = 326585890657064514707390727895716326387_u128 as i64;
place!(Field::<Adt41>(Variant(_15, 0), 1)).fld2.3 = [56053_u16,30072_u16];
place!(Field::<Adt41>(Variant(_15, 0), 1)).fld2.1.4 = RET as f32;
_21 = Field::<Adt41>(Variant(_15, 0), 1).fld2.1.4 + _24;
_12 = Adt53::Variant3 { fld0: '\u{e5c8e}' };
place!(Field::<Adt41>(Variant(_15, 0), 1)).fld2.2 = RET;
_27 = _19;
place!(Field::<Adt41>(Variant(_15, 0), 1)).fld2.4 = !_3;
_28 = 1456381103_u32 - 3268844809_u32;
place!(Field::<Adt41>(Variant(_15, 0), 1)).fld2.1.1 = (-90_i8) & (-11_i8);
_26 = [144350161827247472595948128445433306420_i128,(-112648247060168919719941671103414249706_i128),123321708088000622561724747968574432217_i128,(-10651546320205774583863478375201945682_i128),110592420853096922421623089048710263719_i128,41542497033607544709431143229006801480_i128,36519391570649475969214000409193041190_i128];
_8 = _13 + _13;
_13 = _8 - _8;
_16 = !(-9223372036854775808_isize);
_1 = _2;
place!(Field::<char>(Variant(_12, 3), 0)) = '\u{9b6d0}';
_9 = _10.fld1;
_10.fld1 = _9;
_31 = [Field::<Adt41>(Variant(_15, 0), 1).fld2.4,Field::<Adt41>(Variant(_15, 0), 1).fld2.4,Field::<Adt41>(Variant(_15, 0), 1).fld2.4,_3,Field::<Adt41>(Variant(_15, 0), 1).fld2.4];
place!(Field::<*mut [u32; 6]>(Variant(_15, 0), 0)) = core::ptr::addr_of_mut!(_10.fld0);
_4 = _5;
SetDiscriminant(_12, 0);
place!(Field::<Adt41>(Variant(_12, 0), 1)).fld0 = (Field::<Adt41>(Variant(_15, 0), 1).fld0.0,);
place!(Field::<Adt41>(Variant(_15, 0), 1)).fld2.1.2 = core::ptr::addr_of_mut!(place!(Field::<Adt41>(Variant(_12, 0), 1)).fld2.1.1);
place!(Field::<Adt41>(Variant(_12, 0), 1)).fld2.0 = [73032311921605134082232551775417892135_i128,(-30942615494487728173198360625311433343_i128),(-35998550985234599140920598935987897531_i128),(-65253228267956872824086326376812834302_i128),(-98978909562517407975801038916621216405_i128),(-101014999858255044022895244677724467688_i128),32378856133947422494365536614440890503_i128];
place!(Field::<Adt41>(Variant(_15, 0), 1)).fld2.1.1 = -29_i8;
place!(Field::<Adt41>(Variant(_12, 0), 1)).fld1 = Field::<Adt41>(Variant(_15, 0), 1).fld1 - Field::<Adt41>(Variant(_15, 0), 1).fld1;
Goto(bb15)
}
bb15 = {
Call(_38 = dump_var(2_usize, 1_usize, Move(_1), 2_usize, Move(_2), 31_usize, Move(_31), 28_usize, Move(_28)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_38 = dump_var(2_usize, 6_usize, Move(_6), 27_usize, Move(_27), 7_usize, Move(_7), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_38 = dump_var(2_usize, 26_usize, Move(_26), 18_usize, Move(_18), 16_usize, Move(_16), 39_usize, _39), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn3(mut _1: [i128; 7],mut _2: [i128; 7],mut _3: [i128; 7],mut _4: [i128; 7],mut _5: [i128; 7],mut _6: [i128; 7],mut _7: [i128; 7],mut _8: [i128; 7],mut _9: [i128; 7],mut _10: [i128; 7],mut _11: [i128; 7],mut _12: [i128; 7],mut _13: i16,mut _14: [i128; 7]) -> [i128; 7] {
mir! {
type RET = [i128; 7];
let _15: Adt54;
let _16: ([char; 8], u16, *mut [i128; 7], i32, i128);
let _17: *mut [i128; 7];
let _18: f64;
let _19: [i64; 1];
let _20: isize;
let _21: [i32; 3];
let _22: Adt41;
let _23: Adt48;
let _24: &'static i8;
let _25: u16;
let _26: [i32; 3];
let _27: *const f64;
let _28: ();
let _29: ();
{
_10 = [(-154953999695531979164203871626620823798_i128),13953507495017645235285342557071454423_i128,149466654869332872080531430597659040527_i128,3613190615971583028847142254280764207_i128,(-137183474882278380161134173678458867526_i128),(-37816687511025024205930351073993600308_i128),75557143222509934542215870335000272941_i128];
_6 = [3175765284153388357519437452995065151_i128,(-79316607698509082222329418723988950583_i128),(-6445556394721579919724368729581885613_i128),(-142849956885224080142261474867032667257_i128),(-41523289483259604893445815451175495154_i128),(-19204609881412278568309504371280317211_i128),34286472433289344525264237301733825827_i128];
_4 = [51066123825183284922671498479367607993_i128,133327997986441306946199422411483657057_i128,(-59436958980150996532288810419905086540_i128),14869065586543432441916887037184501794_i128,(-43060015387089739175846180104805906333_i128),85228473865706882799450627949452268437_i128,(-139099797311769055432349199560655258158_i128)];
_1 = [(-161504197430214065296810474692831154355_i128),(-135270883691410368180489843899453795129_i128),(-71047802745138347964650569821546967406_i128),(-167769436725298930728540510263002556863_i128),(-35359555810669262796907367884358702001_i128),(-25279248054739903857812729089685122853_i128),26062295638372963243826678096123822859_i128];
Call(_13 = fn4(_10, _12, _1, _4, _14, _11, _7, _12, _8, _12, _12, _3), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
RET = [(-12318850678954051472625985663163069735_i128),162852300898888529477051501446776906472_i128,(-151109760111577125113548860501543289094_i128),78119995620900923225673132828060441973_i128,40236142107446269335060562059138991307_i128,29698082072502647435887843729053940365_i128,111825097072113900997163696489717808487_i128];
RET = [133547385986625840939897227946147693429_i128,57731969638595399188413174448488510828_i128,541904027278056054207001471642490709_i128,(-24007396892210057107823526481923590104_i128),81034526512003305315309832818272235234_i128,132884514701478196677966357140683976402_i128,21692812929992376447383626214656093281_i128];
_15.fld1 = [29188594790456750770447796800296462718_i128,(-125345836955925114975841230279980629428_i128),(-136981260368969961483124156535219579709_i128),42272411692848794651949491861963167189_i128,69428590271074091756725942438207251499_i128,(-81767806005397491459415017373620379915_i128),(-4418693771713261860063036695053905634_i128),(-6032798785615904644756091974542674746_i128)];
_1 = _5;
_13 = 164_u8 as i16;
_5 = [42214534647464683904219765358004128053_i128,(-78643021082956318352433664251211271111_i128),25994380057214547171128106240894052838_i128,(-65559793885368036909665969423165014109_i128),11640312343463351888248473350904917971_i128,59149698194967445541253990431843578041_i128,102499817550948409174767387594354284717_i128];
Goto(bb2)
}
bb2 = {
RET = [(-53148841584664867997680662303627997915_i128),33117251010518448762281287441095738654_i128,(-94246171988541779210563557936501396381_i128),143589570845813687182835774773478681115_i128,(-113850085564720227352575081100955967527_i128),(-23506103539330966363133554495700628023_i128),19773697458855643009162153373707360777_i128];
_16.3 = (-654971093_i32);
_16.1 = '\u{10ac0}' as u16;
_15.fld1 = [122863032840453515135216382451073087899_i128,13580985826367343301345179054622687139_i128,(-46614692770828044026125156756772959737_i128),(-15064789672934043759162769059301856626_i128),154273914161175039954284270141717664688_i128,133439528239674652544614409371643912168_i128,(-68011064348815720794001557091344181332_i128),164749458612299785047497383022986587386_i128];
_4 = [43738192558231035195734729855347180284_i128,96772354721663647050097068390335678344_i128,51759569347828923504518978013501549977_i128,114032527017956707902677672949561466479_i128,(-25869784539809844511143819033049744813_i128),(-94843578692912151027822760689829329550_i128),45362957043370460949688651365798187375_i128];
_4 = _8;
_1 = [(-48860309451909354341637589403161407640_i128),(-96118658424430410838928206696574784618_i128),(-8826317195672476984095258338261678566_i128),136255731617145782347926239865941984152_i128,104878880426759231743811134354126722851_i128,(-107926779794685317373395946219317572920_i128),37888316355830043113600196020579149920_i128];
_17 = core::ptr::addr_of_mut!(_10);
RET = (*_17);
_16.2 = _17;
_16.4 = 12683120870262825536794389541229245055_i128;
(*_17) = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
RET = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
match _16.4 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
12683120870262825536794389541229245055 => bb11,
_ => bb10
}
}
bb3 = {
RET = [(-12318850678954051472625985663163069735_i128),162852300898888529477051501446776906472_i128,(-151109760111577125113548860501543289094_i128),78119995620900923225673132828060441973_i128,40236142107446269335060562059138991307_i128,29698082072502647435887843729053940365_i128,111825097072113900997163696489717808487_i128];
RET = [133547385986625840939897227946147693429_i128,57731969638595399188413174448488510828_i128,541904027278056054207001471642490709_i128,(-24007396892210057107823526481923590104_i128),81034526512003305315309832818272235234_i128,132884514701478196677966357140683976402_i128,21692812929992376447383626214656093281_i128];
_15.fld1 = [29188594790456750770447796800296462718_i128,(-125345836955925114975841230279980629428_i128),(-136981260368969961483124156535219579709_i128),42272411692848794651949491861963167189_i128,69428590271074091756725942438207251499_i128,(-81767806005397491459415017373620379915_i128),(-4418693771713261860063036695053905634_i128),(-6032798785615904644756091974542674746_i128)];
_1 = _5;
_13 = 164_u8 as i16;
_5 = [42214534647464683904219765358004128053_i128,(-78643021082956318352433664251211271111_i128),25994380057214547171128106240894052838_i128,(-65559793885368036909665969423165014109_i128),11640312343463351888248473350904917971_i128,59149698194967445541253990431843578041_i128,102499817550948409174767387594354284717_i128];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_16.0 = ['\u{b87b3}','\u{ee54}','\u{cd6d}','\u{c280c}','\u{f0bc1}','\u{4bb36}','\u{4ff56}','\u{2329c}'];
_15.fld1 = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
_15.fld0 = [1081396060_u32,1041638147_u32,1174124120_u32,1073258965_u32,1078733696_u32,3050662879_u32];
_18 = _16.4 as f64;
_18 = 2_usize as f64;
_3 = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
_15.fld1 = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
_19 = [4654104934250729214_i64];
_10 = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
_3 = _2;
_16.1 = !23865_u16;
_15.fld0 = [27399559_u32,1354203063_u32,2594365963_u32,1355295132_u32,2606284611_u32,3021238629_u32];
_8 = _2;
_16.1 = 1977_u16;
_12 = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
_20 = _16.1 as isize;
_17 = core::ptr::addr_of_mut!(_8);
_16.0 = ['\u{10c944}','\u{6baa5}','\u{fdb29}','\u{c23f}','\u{2a0f1}','\u{b34e1}','\u{ab78f}','\u{e03da}'];
_21 = [_16.3,_16.3,_16.3];
_16.2 = core::ptr::addr_of_mut!(_6);
_22.fld2.1.4 = (-80_i8) as f32;
_22.fld2.1.0 = core::ptr::addr_of_mut!(_22.fld2.1.1);
_19 = [6765181416526909906_i64];
_10 = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
_11 = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
_4 = _10;
_22.fld2.1.0 = core::ptr::addr_of_mut!(_22.fld2.1.1);
match _16.4 {
12683120870262825536794389541229245055 => bb13,
_ => bb12
}
}
bb12 = {
RET = [(-53148841584664867997680662303627997915_i128),33117251010518448762281287441095738654_i128,(-94246171988541779210563557936501396381_i128),143589570845813687182835774773478681115_i128,(-113850085564720227352575081100955967527_i128),(-23506103539330966363133554495700628023_i128),19773697458855643009162153373707360777_i128];
_16.3 = (-654971093_i32);
_16.1 = '\u{10ac0}' as u16;
_15.fld1 = [122863032840453515135216382451073087899_i128,13580985826367343301345179054622687139_i128,(-46614692770828044026125156756772959737_i128),(-15064789672934043759162769059301856626_i128),154273914161175039954284270141717664688_i128,133439528239674652544614409371643912168_i128,(-68011064348815720794001557091344181332_i128),164749458612299785047497383022986587386_i128];
_4 = [43738192558231035195734729855347180284_i128,96772354721663647050097068390335678344_i128,51759569347828923504518978013501549977_i128,114032527017956707902677672949561466479_i128,(-25869784539809844511143819033049744813_i128),(-94843578692912151027822760689829329550_i128),45362957043370460949688651365798187375_i128];
_4 = _8;
_1 = [(-48860309451909354341637589403161407640_i128),(-96118658424430410838928206696574784618_i128),(-8826317195672476984095258338261678566_i128),136255731617145782347926239865941984152_i128,104878880426759231743811134354126722851_i128,(-107926779794685317373395946219317572920_i128),37888316355830043113600196020579149920_i128];
_17 = core::ptr::addr_of_mut!(_10);
RET = (*_17);
_16.2 = _17;
_16.4 = 12683120870262825536794389541229245055_i128;
(*_17) = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
RET = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
match _16.4 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
12683120870262825536794389541229245055 => bb11,
_ => bb10
}
}
bb13 = {
_22.fld1 = 9213347496473384138_i64;
_22.fld0.0 = core::ptr::addr_of_mut!(_23.fld3.1.2);
_14 = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
_11 = _4;
_7 = _2;
_23.fld4 = _16.1;
_23.fld0.0 = ['\u{513d8}','\u{b6eba}','\u{7eee9}','\u{24c58}','\u{20c27}','\u{ee54e}','\u{11721}','\u{6fa8a}'];
_22.fld2.1.0 = core::ptr::addr_of_mut!(_23.fld3.1.1);
_24 = &_22.fld2.1.1;
(*_17) = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
_23.fld3.0 = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
_25 = _23.fld4;
_22.fld2.3 = [_23.fld4,_25];
_6 = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
_7 = [_16.4,_16.4,_16.4,_16.4,_16.4,_16.4,_16.4];
match _23.fld4 {
0 => bb1,
1 => bb6,
2 => bb9,
3 => bb10,
4 => bb12,
5 => bb14,
1977 => bb16,
_ => bb15
}
}
bb14 = {
Return()
}
bb15 = {
RET = [(-12318850678954051472625985663163069735_i128),162852300898888529477051501446776906472_i128,(-151109760111577125113548860501543289094_i128),78119995620900923225673132828060441973_i128,40236142107446269335060562059138991307_i128,29698082072502647435887843729053940365_i128,111825097072113900997163696489717808487_i128];
RET = [133547385986625840939897227946147693429_i128,57731969638595399188413174448488510828_i128,541904027278056054207001471642490709_i128,(-24007396892210057107823526481923590104_i128),81034526512003305315309832818272235234_i128,132884514701478196677966357140683976402_i128,21692812929992376447383626214656093281_i128];
_15.fld1 = [29188594790456750770447796800296462718_i128,(-125345836955925114975841230279980629428_i128),(-136981260368969961483124156535219579709_i128),42272411692848794651949491861963167189_i128,69428590271074091756725942438207251499_i128,(-81767806005397491459415017373620379915_i128),(-4418693771713261860063036695053905634_i128),(-6032798785615904644756091974542674746_i128)];
_1 = _5;
_13 = 164_u8 as i16;
_5 = [42214534647464683904219765358004128053_i128,(-78643021082956318352433664251211271111_i128),25994380057214547171128106240894052838_i128,(-65559793885368036909665969423165014109_i128),11640312343463351888248473350904917971_i128,59149698194967445541253990431843578041_i128,102499817550948409174767387594354284717_i128];
Goto(bb2)
}
bb16 = {
_23.fld3.4 = 23_u8 | 137_u8;
_23.fld3.1.2 = core::ptr::addr_of_mut!((*_24));
_25 = _23.fld4 >> _16.1;
_16.4 = (-16058912015936008964721881544036242955_i128) ^ 60425574330683038579726716705546817478_i128;
Goto(bb17)
}
bb17 = {
Call(_28 = dump_var(3_usize, 14_usize, Move(_14), 20_usize, Move(_20), 12_usize, Move(_12), 5_usize, Move(_5)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_28 = dump_var(3_usize, 9_usize, Move(_9), 4_usize, Move(_4), 8_usize, Move(_8), 1_usize, Move(_1)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_28 = dump_var(3_usize, 10_usize, Move(_10), 29_usize, _29, 29_usize, _29, 29_usize, _29), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn4(mut _1: [i128; 7],mut _2: [i128; 7],mut _3: [i128; 7],mut _4: [i128; 7],mut _5: [i128; 7],mut _6: [i128; 7],mut _7: [i128; 7],mut _8: [i128; 7],mut _9: [i128; 7],mut _10: [i128; 7],mut _11: [i128; 7],mut _12: [i128; 7]) -> i16 {
mir! {
type RET = i16;
let _13: [char; 8];
let _14: isize;
let _15: [u32; 6];
let _16: Adt54;
let _17: [i64; 1];
let _18: u32;
let _19: usize;
let _20: Adt46;
let _21: bool;
let _22: *mut [u32; 6];
let _23: isize;
let _24: isize;
let _25: f64;
let _26: u64;
let _27: [i32; 3];
let _28: u32;
let _29: [i128; 7];
let _30: bool;
let _31: isize;
let _32: isize;
let _33: *mut char;
let _34: &'static i8;
let _35: [u8; 2];
let _36: Adt54;
let _37: [i32; 3];
let _38: (*mut i8, i8, *mut i8, char, f32, *mut i128);
let _39: (bool, &'static i8);
let _40: ();
let _41: ();
{
_13 = ['\u{23a8}','\u{a1eec}','\u{103285}','\u{1810c}','\u{2e58a}','\u{2f51f}','\u{a4820}','\u{1bc9d}'];
_10 = [(-9763897049655977597990590029603952553_i128),(-40053284076575796539977059053131883483_i128),(-80229864191291180341843557220678294906_i128),(-41074785221263529702000501913468716316_i128),(-127063126070265606369038998315427435182_i128),(-58075954345756554562542928300802972066_i128),(-131226784042176172188272994437812127098_i128)];
RET = true as i16;
RET = (-29087_i16) ^ (-24115_i16);
RET = 4024_i16;
_14 = (-9223372036854775808_isize) >> RET;
_13 = ['\u{aea9a}','\u{936f4}','\u{c389d}','\u{62346}','\u{b89c8}','\u{cec6c}','\u{f7346}','\u{20d2c}'];
_1 = [138421371953984518730576974935761026746_i128,39225354230260277707219246179846256053_i128,82672521209037417229780906454271509326_i128,(-163221545252070596696713459034694729220_i128),(-57886808147522268562545477436980264305_i128),99408879150934904396116172455219480725_i128,(-18331397964016458133411365666371236039_i128)];
_1 = [(-7851621553904376879995408661082451686_i128),96061562604113244959551158008398217831_i128,(-68037076131524732199718756031220541211_i128),(-134798542015528306523970651131657805301_i128),(-6161917723144775230581917668974376016_i128),(-87034950495602794431356460159291117985_i128),(-111929806198361004190789246353983144334_i128)];
_7 = [126268404483453341920568906489861966881_i128,(-16857192050297691558303772590946082123_i128),50052719360235967077908198311170893837_i128,(-120741578340710643011642940578530665896_i128),(-53521985478339974283574634937308601755_i128),109170833446053182395837390580577414593_i128,6585971075083348685223339070018242777_i128];
_14 = 5_usize as isize;
_16.fld0 = [1764797354_u32,2121684799_u32,2480660077_u32,2480839498_u32,706249627_u32,2940909871_u32];
_13 = ['\u{e7b86}','\u{b4784}','\u{7eadd}','\u{87a55}','\u{94253}','\u{a0927}','\u{cbb26}','\u{3c9d}'];
_12 = [156514579726484444111836624474556497726_i128,7326922181207229160285463854574689059_i128,(-64070232489804188051442004449215791521_i128),(-30680693720273956778700635052098385199_i128),21921286487379661056509317032870935665_i128,38526980137652045561246096590193837916_i128,68269365677278309835863765361758171799_i128];
_5 = [(-136987832076100775503810727083387542296_i128),(-168847498483123567141935162011123578973_i128),(-90124728602505234219286620196075641327_i128),88023163660726811605773555461027092099_i128,(-100312771065197488797781645224715562936_i128),13325231250251176105835696628686545218_i128,83647970493484825238212558204632130262_i128];
_16.fld1 = [(-137098667652076236075349652395467513422_i128),143423405220622665840833088940748601086_i128,123540919350143736481194586753377563041_i128,118910127015313735069254103625530515565_i128,131400202318583134390790701713123902644_i128,64693725319680789953819878907808876429_i128,126326719451690181383078637682565307702_i128,(-4176082902264850342087200181331815132_i128)];
_10 = _6;
_11 = [(-119125830005603368103242051316158589599_i128),(-83191795839993674395561051286444549818_i128),(-32544054164918298602069314023108497195_i128),(-108967136104859337458423053241899417648_i128),(-85034061671987003115752395305586286766_i128),(-140171528032561507711644008552740690047_i128),(-35751274576291305031725152483036678728_i128)];
_17 = [(-9050665866354185583_i64)];
RET = !(-31276_i16);
_11 = [36698277206292547654063115903933725479_i128,(-84783382380557862517801294934986743284_i128),(-20013136770681304224886507503706079163_i128),(-149629425335384302274420700196594815219_i128),(-124269678790035235824681008754313293279_i128),(-133653977820143607035997036429930356047_i128),124961939549306285692093894633919678215_i128];
_15 = [2293909202_u32,2256409803_u32,3448379356_u32,2406701915_u32,618936458_u32,252813141_u32];
Call(_11 = fn5(_13, _14), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6 = [7791460105791840013697433037696118119_i128,(-77106221414278557338304039329040805669_i128),137327288669878679920723000317293808300_i128,146121655218484904244314057508158325977_i128,(-50516705406013755168144532301724993263_i128),99217974504405116645356469890561881762_i128,(-263846374746434542678259408387651608_i128)];
_18 = 3157662963_u32 * 675435496_u32;
_4 = _6;
_18 = 1167853146_u32 | 1461002358_u32;
_2 = [11452544267274823191676632973164154619_i128,61305885644545364961329488909036898218_i128,(-163134934987676063264687830974332728913_i128),(-148988412935505763908423393724858332750_i128),103277754343466665401136294685203255267_i128,154750657555757961140437882125812081920_i128,81684756439115940488861458780930645756_i128];
Goto(bb2)
}
bb2 = {
_2 = [43474846114800003930736960494781831901_i128,48615387664061642129828883584834216664_i128,(-21851097414813975734645862192524200907_i128),(-72005993468792367704479041592519738631_i128),(-22159347248886799310965594801250475715_i128),(-23516751555967742676421999252706679060_i128),(-152442330868635499836876085900302615423_i128)];
_4 = [(-143785840551671819198385254006695376039_i128),(-161524875146111209055437467225532003460_i128),160642854236850572474596358775047316976_i128,18676015192209350406961601020116841419_i128,110592372159719474652745432949014668559_i128,(-168959780900092333137623789179382745824_i128),(-7892301753981656428824462500381918240_i128)];
RET = (-12858_i16);
_12 = [140850044408597968313336741406116340501_i128,51762579760372073036290119114008498399_i128,36726154298085761926836824495704637402_i128,7584015038426244232278464709589159752_i128,(-73130502192623696897032762454730359259_i128),(-87723068703369659957586395755511458350_i128),98040182339961993130601430854837649107_i128];
_16.fld1 = [12854794025337795308418527101269136163_i128,(-96987182160621622153409879691883951583_i128),(-120819433996294851985773634791801726809_i128),131027457252966977010001548336700024405_i128,103895920444498098184809684229152432127_i128,85704017107390334897081542570975040914_i128,(-10920723666909384013760557677069146163_i128),117750774255777154725343568168560447400_i128];
_13 = ['\u{ec5da}','\u{e6e91}','\u{e1ba1}','\u{fdce2}','\u{177a6}','\u{37d72}','\u{aa0af}','\u{7f98d}'];
_15 = [_18,_18,_18,_18,_18,_18];
_4 = [90932834135780490808805165621094479082_i128,(-104938662426111820356945519020958818655_i128),44185896159327015979294545927005611513_i128,1797877243432336096982526449738206967_i128,110347489704636226464298641105451269347_i128,(-96061060462720074147389920424864919222_i128),51832074441042308508453264541970269328_i128];
_8 = _12;
_4 = [(-3813993563486981942556536951162676877_i128),(-88205643120506712288638888348290273530_i128),73419486248424250331489106927811906861_i128,(-52297349303894623902692892130750484923_i128),(-117096339804032701219698543842403446806_i128),(-132899769317903429929626447119381333597_i128),(-49313089051735053476824366938361808018_i128)];
_12 = [(-19516926739476210849619189269305215232_i128),32780897218050901550762384959827723563_i128,119341592975264770819359288002842928932_i128,105055641665480398569437012598490803207_i128,(-95889571647872919563667456076459629185_i128),(-125182242612918368821559167653910817576_i128),87391208257462123228457875597493091617_i128];
_5 = [73652214906398949783160345372632837205_i128,26390001788635400572817423247696396157_i128,1573992965896732834211682118050927681_i128,80912913680281826485882173697688108446_i128,(-197732168473449945657009771177539570_i128),(-120014158986263686693718356473879047523_i128),132007702676488142591866316840715889549_i128];
_11 = [(-156836450408133860846081482078303947810_i128),146033538949849605734168899932309920088_i128,(-87317979779381585554833432765175689530_i128),(-34916953956443787310005041231589117641_i128),(-143218074609743013683141661733998264514_i128),131144708494026143137092404381042294568_i128,(-35767170336578988334035372570102853569_i128)];
_7 = [(-32756866392741658179186821157716052415_i128),(-126220360456648323864446706886486778310_i128),(-74827257776017389392241382523914768545_i128),28598774880516995901222940646234235205_i128,8005082295542615222300766420103647440_i128,94673792979368927722244431543313220776_i128,160127699189202352127918130951151628548_i128];
_17 = [(-5897808559123725712_i64)];
_2 = [(-8157116883458953220187297155814916640_i128),(-55114739902416462323052234905744729466_i128),(-72802820998578415330188540331101503430_i128),166651545337851321556080410027718961349_i128,(-55200960124360362702271886870367079937_i128),(-28534361672416831311861660855105814756_i128),(-10811374414618637840834693181033264771_i128)];
_16.fld1 = [100417537145106581999399563761393674051_i128,(-8764672108355693209288533848123129112_i128),12755999144698707479795764024286325366_i128,(-121149450034237140707496105667316921219_i128),(-132856778128644184549642184302141914388_i128),16540706130804224971132987853851894659_i128,(-68940056744658477091444179911836448696_i128),(-12010729160597655611220358297192119639_i128)];
_20 = Adt46::Variant2 { fld0: true };
_17 = [(-715548502710869279_i64)];
_15 = [_18,_18,_18,_18,_18,_18];
RET = !(-16407_i16);
RET = (-10851_i16);
_11 = _9;
Goto(bb3)
}
bb3 = {
_20 = Adt46::Variant3 { fld0: _16.fld1 };
match RET {
0 => bb1,
340282366920938463463374607431768200605 => bb5,
_ => bb4
}
}
bb4 = {
_6 = [7791460105791840013697433037696118119_i128,(-77106221414278557338304039329040805669_i128),137327288669878679920723000317293808300_i128,146121655218484904244314057508158325977_i128,(-50516705406013755168144532301724993263_i128),99217974504405116645356469890561881762_i128,(-263846374746434542678259408387651608_i128)];
_18 = 3157662963_u32 * 675435496_u32;
_4 = _6;
_18 = 1167853146_u32 | 1461002358_u32;
_2 = [11452544267274823191676632973164154619_i128,61305885644545364961329488909036898218_i128,(-163134934987676063264687830974332728913_i128),(-148988412935505763908423393724858332750_i128),103277754343466665401136294685203255267_i128,154750657555757961140437882125812081920_i128,81684756439115940488861458780930645756_i128];
Goto(bb2)
}
bb5 = {
_11 = [70249844572521740797031660867935892479_i128,162252752911062808947041072503913030821_i128,(-70564249009047964184499475398267143177_i128),(-15996882137125759470047681676389797619_i128),156989154376070668285310087264766058690_i128,(-84574873642511734993003545448141853341_i128),50826157672490876361189841465871399301_i128];
_10 = _4;
RET = (-29351_i16) >> _14;
_19 = !14332642267994073744_usize;
_7 = [115997794021257555764873052495714263761_i128,(-71735809161889523740633774157278092233_i128),(-36437663344555679455619153545553460574_i128),108150534118867482933801208382845292549_i128,24907497513126396010901333072687549619_i128,(-80971222081239420202009706857756776820_i128),(-100891751879558931938869559443659104629_i128)];
_16.fld0 = [_18,_18,_18,_18,_18,_18];
_21 = false ^ true;
_17 = [2060522492267579835_i64];
_11 = _4;
_3 = [90472399037043555673070824636493852950_i128,136819921152585726466660769725189646612_i128,(-123957384072284100452617974083851140259_i128),(-83220907164309458921054074016102286466_i128),(-44226615240714469918666033250874163981_i128),108448787555160967784488537419090488606_i128,(-159850145471993672988441738498955379108_i128)];
RET = 3780_i16 | 1577_i16;
_19 = 14984395399499358875_usize;
_22 = core::ptr::addr_of_mut!(_16.fld0);
(*_22) = _15;
_16.fld1 = [(-34766781438344090792047497676446006531_i128),(-101423229703651102576581058909819562728_i128),62030388001867740549659770289256134661_i128,105129595451367106121554632825849722363_i128,(-24862423116444954713007594803608521548_i128),139596812459924597891744508290915801814_i128,21817347082069341958765479133603097750_i128,168096504179526401121537399485644319737_i128];
_13 = ['\u{93194}','\u{9eaf7}','\u{cf6b}','\u{2bc66}','\u{b331a}','\u{3b526}','\u{bebe8}','\u{ba0d8}'];
match _19 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
14984395399499358875 => bb11,
_ => bb10
}
}
bb6 = {
_6 = [7791460105791840013697433037696118119_i128,(-77106221414278557338304039329040805669_i128),137327288669878679920723000317293808300_i128,146121655218484904244314057508158325977_i128,(-50516705406013755168144532301724993263_i128),99217974504405116645356469890561881762_i128,(-263846374746434542678259408387651608_i128)];
_18 = 3157662963_u32 * 675435496_u32;
_4 = _6;
_18 = 1167853146_u32 | 1461002358_u32;
_2 = [11452544267274823191676632973164154619_i128,61305885644545364961329488909036898218_i128,(-163134934987676063264687830974332728913_i128),(-148988412935505763908423393724858332750_i128),103277754343466665401136294685203255267_i128,154750657555757961140437882125812081920_i128,81684756439115940488861458780930645756_i128];
Goto(bb2)
}
bb7 = {
_20 = Adt46::Variant3 { fld0: _16.fld1 };
match RET {
0 => bb1,
340282366920938463463374607431768200605 => bb5,
_ => bb4
}
}
bb8 = {
_2 = [43474846114800003930736960494781831901_i128,48615387664061642129828883584834216664_i128,(-21851097414813975734645862192524200907_i128),(-72005993468792367704479041592519738631_i128),(-22159347248886799310965594801250475715_i128),(-23516751555967742676421999252706679060_i128),(-152442330868635499836876085900302615423_i128)];
_4 = [(-143785840551671819198385254006695376039_i128),(-161524875146111209055437467225532003460_i128),160642854236850572474596358775047316976_i128,18676015192209350406961601020116841419_i128,110592372159719474652745432949014668559_i128,(-168959780900092333137623789179382745824_i128),(-7892301753981656428824462500381918240_i128)];
RET = (-12858_i16);
_12 = [140850044408597968313336741406116340501_i128,51762579760372073036290119114008498399_i128,36726154298085761926836824495704637402_i128,7584015038426244232278464709589159752_i128,(-73130502192623696897032762454730359259_i128),(-87723068703369659957586395755511458350_i128),98040182339961993130601430854837649107_i128];
_16.fld1 = [12854794025337795308418527101269136163_i128,(-96987182160621622153409879691883951583_i128),(-120819433996294851985773634791801726809_i128),131027457252966977010001548336700024405_i128,103895920444498098184809684229152432127_i128,85704017107390334897081542570975040914_i128,(-10920723666909384013760557677069146163_i128),117750774255777154725343568168560447400_i128];
_13 = ['\u{ec5da}','\u{e6e91}','\u{e1ba1}','\u{fdce2}','\u{177a6}','\u{37d72}','\u{aa0af}','\u{7f98d}'];
_15 = [_18,_18,_18,_18,_18,_18];
_4 = [90932834135780490808805165621094479082_i128,(-104938662426111820356945519020958818655_i128),44185896159327015979294545927005611513_i128,1797877243432336096982526449738206967_i128,110347489704636226464298641105451269347_i128,(-96061060462720074147389920424864919222_i128),51832074441042308508453264541970269328_i128];
_8 = _12;
_4 = [(-3813993563486981942556536951162676877_i128),(-88205643120506712288638888348290273530_i128),73419486248424250331489106927811906861_i128,(-52297349303894623902692892130750484923_i128),(-117096339804032701219698543842403446806_i128),(-132899769317903429929626447119381333597_i128),(-49313089051735053476824366938361808018_i128)];
_12 = [(-19516926739476210849619189269305215232_i128),32780897218050901550762384959827723563_i128,119341592975264770819359288002842928932_i128,105055641665480398569437012598490803207_i128,(-95889571647872919563667456076459629185_i128),(-125182242612918368821559167653910817576_i128),87391208257462123228457875597493091617_i128];
_5 = [73652214906398949783160345372632837205_i128,26390001788635400572817423247696396157_i128,1573992965896732834211682118050927681_i128,80912913680281826485882173697688108446_i128,(-197732168473449945657009771177539570_i128),(-120014158986263686693718356473879047523_i128),132007702676488142591866316840715889549_i128];
_11 = [(-156836450408133860846081482078303947810_i128),146033538949849605734168899932309920088_i128,(-87317979779381585554833432765175689530_i128),(-34916953956443787310005041231589117641_i128),(-143218074609743013683141661733998264514_i128),131144708494026143137092404381042294568_i128,(-35767170336578988334035372570102853569_i128)];
_7 = [(-32756866392741658179186821157716052415_i128),(-126220360456648323864446706886486778310_i128),(-74827257776017389392241382523914768545_i128),28598774880516995901222940646234235205_i128,8005082295542615222300766420103647440_i128,94673792979368927722244431543313220776_i128,160127699189202352127918130951151628548_i128];
_17 = [(-5897808559123725712_i64)];
_2 = [(-8157116883458953220187297155814916640_i128),(-55114739902416462323052234905744729466_i128),(-72802820998578415330188540331101503430_i128),166651545337851321556080410027718961349_i128,(-55200960124360362702271886870367079937_i128),(-28534361672416831311861660855105814756_i128),(-10811374414618637840834693181033264771_i128)];
_16.fld1 = [100417537145106581999399563761393674051_i128,(-8764672108355693209288533848123129112_i128),12755999144698707479795764024286325366_i128,(-121149450034237140707496105667316921219_i128),(-132856778128644184549642184302141914388_i128),16540706130804224971132987853851894659_i128,(-68940056744658477091444179911836448696_i128),(-12010729160597655611220358297192119639_i128)];
_20 = Adt46::Variant2 { fld0: true };
_17 = [(-715548502710869279_i64)];
_15 = [_18,_18,_18,_18,_18,_18];
RET = !(-16407_i16);
RET = (-10851_i16);
_11 = _9;
Goto(bb3)
}
bb9 = {
_6 = [7791460105791840013697433037696118119_i128,(-77106221414278557338304039329040805669_i128),137327288669878679920723000317293808300_i128,146121655218484904244314057508158325977_i128,(-50516705406013755168144532301724993263_i128),99217974504405116645356469890561881762_i128,(-263846374746434542678259408387651608_i128)];
_18 = 3157662963_u32 * 675435496_u32;
_4 = _6;
_18 = 1167853146_u32 | 1461002358_u32;
_2 = [11452544267274823191676632973164154619_i128,61305885644545364961329488909036898218_i128,(-163134934987676063264687830974332728913_i128),(-148988412935505763908423393724858332750_i128),103277754343466665401136294685203255267_i128,154750657555757961140437882125812081920_i128,81684756439115940488861458780930645756_i128];
Goto(bb2)
}
bb10 = {
Return()
}
bb11 = {
_16.fld0 = [_18,_18,_18,_18,_18,_18];
(*_22) = [_18,_18,_18,_18,_18,_18];
_25 = _14 as f64;
_2 = _9;
_11 = [95734137805758825564841561795097859625_i128,47501746242864401913501974366078122562_i128,119900680251707623845810806441206119502_i128,148464074587490932617482615334379149550_i128,21065949132440781361536330938495024921_i128,(-165943513833495170211239900625172742300_i128),(-21951550961237777233856920102329615463_i128)];
_23 = !_14;
_18 = 3460939650_u32;
place!(Field::<[i128; 8]>(Variant(_20, 3), 0)) = _16.fld1;
_11 = _5;
_25 = (-1447967604_i32) as f64;
_11 = [75033101965844477343159931314866851758_i128,(-102899100553937840977160380929147907266_i128),147148121299084486914761077711003525629_i128,(-84554386716913685026079394821536017240_i128),(-6146632993554623159473165914758982547_i128),(-34291084917653347886970558377814249173_i128),(-33681452812423904747400463007825108737_i128)];
_17 = [6538513729601788479_i64];
_26 = 15601195997475085955_u64;
_25 = _26 as f64;
SetDiscriminant(_20, 2);
_2 = [(-36413483198562680033700207372683207916_i128),(-170130227165559005080289498325527236631_i128),(-66248328118803011605983813543462700879_i128),5088397014338343313449226121820062093_i128,(-74127834092872584196000911321502005350_i128),81019727126085624585955240424324265704_i128,104171387083002540174826378705776459465_i128];
_25 = 174_u8 as f64;
_23 = _14 + _14;
(*_22) = [_18,_18,_18,_18,_18,_18];
_12 = _5;
_23 = _14 & _14;
_1 = _6;
RET = _18 as i16;
Goto(bb12)
}
bb12 = {
_29 = _9;
_25 = 18912_u16 as f64;
_31 = _23 << _18;
_7 = _5;
_10 = [(-124902047841839086956106197754536840992_i128),(-17342225816917330916236669179478063184_i128),16631377400147135324013760085273688975_i128,100196490659752810787503268756095533131_i128,58878262912291274068774300535867909274_i128,(-161845466519165485171399087414311039155_i128),75718274697080885829659754112686393724_i128];
_20 = Adt46::Variant3 { fld0: _16.fld1 };
_13 = ['\u{d494e}','\u{b3f0f}','\u{7c374}','\u{d858f}','\u{10160f}','\u{faa1c}','\u{f10ac}','\u{6cad8}'];
(*_22) = [_18,_18,_18,_18,_18,_18];
_16 = Adt54 { fld0: _15,fld1: Field::<[i128; 8]>(Variant(_20, 3), 0) };
_16.fld1 = [149098107671419378262915164316139416872_i128,91710788891994477519603426032271955833_i128,(-93264316270280753935560074982204429994_i128),151685122098541446485198125877046205223_i128,(-130492952214591237920379376731966844821_i128),(-109687479944246661746583852761206749744_i128),(-116123236827178630259214084382554070201_i128),(-96771822155287456177784579164004520113_i128)];
RET = _26 as i16;
_16 = Adt54 { fld0: _15,fld1: Field::<[i128; 8]>(Variant(_20, 3), 0) };
_5 = [(-113610323966806189748434224686574977417_i128),102791746125900717242443878305432787481_i128,27142393909981111653218952056458564698_i128,(-57871425196221002187251628483057732539_i128),145419367419624701478812413763407310324_i128,(-30764825151353366221634301078703993879_i128),(-3679411279747019412364953825136550663_i128)];
_28 = 5896192682430429340_i64 as u32;
_32 = _31 & _31;
(*_22) = [_18,_18,_18,_28,_28,_28];
_21 = !true;
_30 = !_21;
_22 = core::ptr::addr_of_mut!(_15);
_15 = [_18,_28,_18,_28,_28,_18];
_35 = [77_u8,197_u8];
_13 = ['\u{2a581}','\u{50661}','\u{c368c}','\u{c1a58}','\u{109fd2}','\u{ec234}','\u{30593}','\u{42d5f}'];
match _26 {
0 => bb13,
1 => bb14,
2 => bb15,
3 => bb16,
4 => bb17,
5 => bb18,
15601195997475085955 => bb20,
_ => bb19
}
}
bb13 = {
_16.fld0 = [_18,_18,_18,_18,_18,_18];
(*_22) = [_18,_18,_18,_18,_18,_18];
_25 = _14 as f64;
_2 = _9;
_11 = [95734137805758825564841561795097859625_i128,47501746242864401913501974366078122562_i128,119900680251707623845810806441206119502_i128,148464074587490932617482615334379149550_i128,21065949132440781361536330938495024921_i128,(-165943513833495170211239900625172742300_i128),(-21951550961237777233856920102329615463_i128)];
_23 = !_14;
_18 = 3460939650_u32;
place!(Field::<[i128; 8]>(Variant(_20, 3), 0)) = _16.fld1;
_11 = _5;
_25 = (-1447967604_i32) as f64;
_11 = [75033101965844477343159931314866851758_i128,(-102899100553937840977160380929147907266_i128),147148121299084486914761077711003525629_i128,(-84554386716913685026079394821536017240_i128),(-6146632993554623159473165914758982547_i128),(-34291084917653347886970558377814249173_i128),(-33681452812423904747400463007825108737_i128)];
_17 = [6538513729601788479_i64];
_26 = 15601195997475085955_u64;
_25 = _26 as f64;
SetDiscriminant(_20, 2);
_2 = [(-36413483198562680033700207372683207916_i128),(-170130227165559005080289498325527236631_i128),(-66248328118803011605983813543462700879_i128),5088397014338343313449226121820062093_i128,(-74127834092872584196000911321502005350_i128),81019727126085624585955240424324265704_i128,104171387083002540174826378705776459465_i128];
_25 = 174_u8 as f64;
_23 = _14 + _14;
(*_22) = [_18,_18,_18,_18,_18,_18];
_12 = _5;
_23 = _14 & _14;
_1 = _6;
RET = _18 as i16;
Goto(bb12)
}
bb14 = {
Return()
}
bb15 = {
_6 = [7791460105791840013697433037696118119_i128,(-77106221414278557338304039329040805669_i128),137327288669878679920723000317293808300_i128,146121655218484904244314057508158325977_i128,(-50516705406013755168144532301724993263_i128),99217974504405116645356469890561881762_i128,(-263846374746434542678259408387651608_i128)];
_18 = 3157662963_u32 * 675435496_u32;
_4 = _6;
_18 = 1167853146_u32 | 1461002358_u32;
_2 = [11452544267274823191676632973164154619_i128,61305885644545364961329488909036898218_i128,(-163134934987676063264687830974332728913_i128),(-148988412935505763908423393724858332750_i128),103277754343466665401136294685203255267_i128,154750657555757961140437882125812081920_i128,81684756439115940488861458780930645756_i128];
Goto(bb2)
}
bb16 = {
_6 = [7791460105791840013697433037696118119_i128,(-77106221414278557338304039329040805669_i128),137327288669878679920723000317293808300_i128,146121655218484904244314057508158325977_i128,(-50516705406013755168144532301724993263_i128),99217974504405116645356469890561881762_i128,(-263846374746434542678259408387651608_i128)];
_18 = 3157662963_u32 * 675435496_u32;
_4 = _6;
_18 = 1167853146_u32 | 1461002358_u32;
_2 = [11452544267274823191676632973164154619_i128,61305885644545364961329488909036898218_i128,(-163134934987676063264687830974332728913_i128),(-148988412935505763908423393724858332750_i128),103277754343466665401136294685203255267_i128,154750657555757961140437882125812081920_i128,81684756439115940488861458780930645756_i128];
Goto(bb2)
}
bb17 = {
_20 = Adt46::Variant3 { fld0: _16.fld1 };
match RET {
0 => bb1,
340282366920938463463374607431768200605 => bb5,
_ => bb4
}
}
bb18 = {
_6 = [7791460105791840013697433037696118119_i128,(-77106221414278557338304039329040805669_i128),137327288669878679920723000317293808300_i128,146121655218484904244314057508158325977_i128,(-50516705406013755168144532301724993263_i128),99217974504405116645356469890561881762_i128,(-263846374746434542678259408387651608_i128)];
_18 = 3157662963_u32 * 675435496_u32;
_4 = _6;
_18 = 1167853146_u32 | 1461002358_u32;
_2 = [11452544267274823191676632973164154619_i128,61305885644545364961329488909036898218_i128,(-163134934987676063264687830974332728913_i128),(-148988412935505763908423393724858332750_i128),103277754343466665401136294685203255267_i128,154750657555757961140437882125812081920_i128,81684756439115940488861458780930645756_i128];
Goto(bb2)
}
bb19 = {
_11 = [70249844572521740797031660867935892479_i128,162252752911062808947041072503913030821_i128,(-70564249009047964184499475398267143177_i128),(-15996882137125759470047681676389797619_i128),156989154376070668285310087264766058690_i128,(-84574873642511734993003545448141853341_i128),50826157672490876361189841465871399301_i128];
_10 = _4;
RET = (-29351_i16) >> _14;
_19 = !14332642267994073744_usize;
_7 = [115997794021257555764873052495714263761_i128,(-71735809161889523740633774157278092233_i128),(-36437663344555679455619153545553460574_i128),108150534118867482933801208382845292549_i128,24907497513126396010901333072687549619_i128,(-80971222081239420202009706857756776820_i128),(-100891751879558931938869559443659104629_i128)];
_16.fld0 = [_18,_18,_18,_18,_18,_18];
_21 = false ^ true;
_17 = [2060522492267579835_i64];
_11 = _4;
_3 = [90472399037043555673070824636493852950_i128,136819921152585726466660769725189646612_i128,(-123957384072284100452617974083851140259_i128),(-83220907164309458921054074016102286466_i128),(-44226615240714469918666033250874163981_i128),108448787555160967784488537419090488606_i128,(-159850145471993672988441738498955379108_i128)];
RET = 3780_i16 | 1577_i16;
_19 = 14984395399499358875_usize;
_22 = core::ptr::addr_of_mut!(_16.fld0);
(*_22) = _15;
_16.fld1 = [(-34766781438344090792047497676446006531_i128),(-101423229703651102576581058909819562728_i128),62030388001867740549659770289256134661_i128,105129595451367106121554632825849722363_i128,(-24862423116444954713007594803608521548_i128),139596812459924597891744508290915801814_i128,21817347082069341958765479133603097750_i128,168096504179526401121537399485644319737_i128];
_13 = ['\u{93194}','\u{9eaf7}','\u{cf6b}','\u{2bc66}','\u{b331a}','\u{3b526}','\u{bebe8}','\u{ba0d8}'];
match _19 {
0 => bb4,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
14984395399499358875 => bb11,
_ => bb10
}
}
bb20 = {
_10 = _9;
_31 = _32;
_7 = _2;
_6 = _3;
_36.fld1 = Field::<[i128; 8]>(Variant(_20, 3), 0);
_4 = _8;
_14 = RET as isize;
_23 = _32;
_36.fld0 = [_28,_28,_18,_18,_28,_28];
_19 = !2_usize;
_27 = [1998300258_i32,1410844992_i32,(-1650936755_i32)];
SetDiscriminant(_20, 3);
place!(Field::<[i128; 8]>(Variant(_20, 3), 0)) = [(-115632302261064458258439734800443109420_i128),118618611044710631200365159089609336678_i128,(-161745397629106371082348928660613779947_i128),77760478707779633019545870664481873958_i128,39251482915462598801118418405834354026_i128,39881586371093955671081630074440587916_i128,(-165959860687925326147898139065847062305_i128),153780194126873378832869220667392677875_i128];
_23 = -_31;
_28 = _18;
_14 = _23 + _31;
_2 = _1;
_31 = -_14;
_15 = [_18,_18,_28,_18,_18,_18];
_10 = [(-151001245232212881437599837399803783499_i128),(-13608630832397300172315516946249411124_i128),(-28840654365510081350783426147917966983_i128),25108420932750270839069248280365271695_i128,(-55534078706380011668123020387912500742_i128),64205589744149896276657377496568812396_i128,(-139365189531833417428043248470617064336_i128)];
_3 = [78508892039471302927415037303029203390_i128,43820818952768530124119874375351110695_i128,(-15020608712850297578201383353155286576_i128),113255813048405035061107941788544051288_i128,(-96469575787474517295068135940826801366_i128),(-84803520524500780758899178595160545347_i128),(-61704571998163397571697837803239099675_i128)];
_2 = [(-32479997313591358011292904869972903519_i128),140559825327662161054903263343548057694_i128,40863469114419276286205827526733175107_i128,(-31655440740516978001903038372308905049_i128),(-110517807331983416590975386555555721019_i128),(-167466442030309028850978951650817057645_i128),95915331183858424812150281473674780730_i128];
_14 = _26 as isize;
_16.fld1 = _36.fld1;
SetDiscriminant(_20, 0);
Goto(bb21)
}
bb21 = {
Call(_40 = dump_var(4_usize, 35_usize, Move(_35), 17_usize, Move(_17), 13_usize, Move(_13), 1_usize, Move(_1)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_40 = dump_var(4_usize, 30_usize, Move(_30), 28_usize, Move(_28), 12_usize, Move(_12), 11_usize, Move(_11)), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Call(_40 = dump_var(4_usize, 4_usize, Move(_4), 32_usize, Move(_32), 21_usize, Move(_21), 5_usize, Move(_5)), ReturnTo(bb24), UnwindUnreachable())
}
bb24 = {
Call(_40 = dump_var(4_usize, 7_usize, Move(_7), 19_usize, Move(_19), 41_usize, _41, 41_usize, _41), ReturnTo(bb25), UnwindUnreachable())
}
bb25 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn5(mut _1: [char; 8],mut _2: isize) -> [i128; 7] {
mir! {
type RET = [i128; 7];
let _3: f32;
let _4: f64;
let _5: u32;
let _6: [char; 8];
let _7: isize;
let _8: isize;
let _9: [i64; 1];
let _10: [u16; 2];
let _11: f64;
let _12: i8;
let _13: Adt42;
let _14: [char; 8];
let _15: [i64; 1];
let _16: [u8; 2];
let _17: isize;
let _18: i64;
let _19: Adt54;
let _20: *mut [i128; 7];
let _21: ();
let _22: ();
{
RET = [(-107543213464232874203265935220589308081_i128),(-102270466466046441996631066099904715153_i128),(-161120202054523443071317896477181570115_i128),82324120516724621830320854341170899012_i128,139772429232297447538382460679324618807_i128,79280482708263796291651207910219864529_i128,(-5137421336958516057611667025222314059_i128)];
_1 = ['\u{3fc0}','\u{f7776}','\u{6ed3c}','\u{4db55}','\u{7b998}','\u{108193}','\u{d65e4}','\u{d5bce}'];
_1 = ['\u{b1fb0}','\u{be10}','\u{7e79e}','\u{73c0b}','\u{97f53}','\u{c1eb5}','\u{714ba}','\u{bad06}'];
RET = [(-29009148742884073706190581189160737509_i128),(-9054869375662924252187191576665998898_i128),(-100287350474556347397029202621040624332_i128),51870650623237126863063746697079039281_i128,(-22387079281103863419601180199093402750_i128),159920873747819788233673041563216621559_i128,(-116327037994654134698090603086626161_i128)];
_4 = 17454143376573163338_u64 as f64;
_4 = _2 as f64;
_3 = 115_u8 as f32;
_1 = ['\u{6d314}','\u{5d6e6}','\u{70f57}','\u{47bb7}','\u{2ff94}','\u{c6cc5}','\u{77dcc}','\u{eb821}'];
RET = [14325153496894657220725250133268284259_i128,(-95058794632212109543469339303491546140_i128),(-60069555866992696346291334412170911455_i128),131934813696078933559371459530073331859_i128,117523060612530682275566542235820831340_i128,143857842715935992467219724223092098786_i128,(-109281726290278313981137532871615940232_i128)];
RET = [102092706187866445791332530910441475230_i128,(-103903694674951410060910437672542079805_i128),(-101185482035197030306511621717566343171_i128),(-73947731780550117633934424036993757976_i128),(-105904289445099790501307111423093455670_i128),(-163776008935349875667829418720840154603_i128),(-144644874086939986764178026139603369312_i128)];
_1 = ['\u{7c90a}','\u{e97b5}','\u{10fdba}','\u{79f82}','\u{65b69}','\u{fbc07}','\u{8d266}','\u{3c5f6}'];
Call(_4 = fn6(_1, RET, _1, _3, _1, RET, _2, _1, RET), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_2 = -(-9223372036854775808_isize);
RET = [(-91936444577133808056446592065547084541_i128),8786874420941867569557557273220024116_i128,(-40902718809412143712465020536686663080_i128),(-6611406363600539386703880389340671093_i128),(-86640807744785703689933947146948215545_i128),60394666137749451073744101168568231343_i128,(-66422077894146135690390158139808281319_i128)];
RET = [20074592019930053186501983865348265094_i128,121726307029173673966646636328989665200_i128,(-164305792145529631694312217296398866093_i128),4051288970106061537998884095871442722_i128,(-21206156414454851184405371720699421056_i128),84573057106636816613388041573760858054_i128,121022893352974418794144275617572446421_i128];
_3 = 8020_u16 as f32;
_2 = 19339_u16 as isize;
_1 = ['\u{a6ad5}','\u{10779f}','\u{b2d59}','\u{f58e7}','\u{18639}','\u{86334}','\u{1f139}','\u{c8f19}'];
_2 = 9223372036854775807_isize;
match _2 {
9223372036854775807 => bb3,
_ => bb2
}
}
bb2 = {
Return()
}
bb3 = {
_4 = 10633452867625081686_u64 as f64;
_4 = 1681279901_u32 as f64;
_2 = (-9223372036854775808_isize) >> (-5266761856522847398_i64);
Goto(bb4)
}
bb4 = {
_2 = 4_usize as isize;
_2 = -(-69_isize);
_4 = (-18_i8) as f64;
_5 = 3694147171_u32;
_5 = !2676036537_u32;
RET = [(-146386206359428687753893753479320570503_i128),127091888701797111378585738591369297683_i128,(-117438222615783962041755287312783426603_i128),(-16356147624922369387736985331291188040_i128),(-111164658098293080251102414355735464153_i128),13808983122380227153716040639687852581_i128,71308478286919806428568352761533872952_i128];
_2 = (-117_isize);
_6 = ['\u{a2e16}','\u{49902}','\u{4e43c}','\u{2c384}','\u{fc475}','\u{b20d6}','\u{5951a}','\u{10e224}'];
RET = [135488278007732659336825923530039806057_i128,(-92961441882704473193989544755369504262_i128),89991853099274948507585737788776315432_i128,(-67773263539491637822955203682168941558_i128),(-43401123939874618241891206358131048552_i128),(-108484066515666872215330026930560318301_i128),167971369638345732439809171294811620702_i128];
_4 = 57907_u16 as f64;
_4 = _2 as f64;
RET = [(-129020078386301773927823972420805641136_i128),(-54832450729498377663412413892551856383_i128),(-122770647295197351556851540974453253163_i128),(-149528830341177821489686980127972717968_i128),(-33500308862899253702311082486209596773_i128),88001038454701433175579877872892473432_i128,66598860911836953178708810977337159544_i128];
_6 = ['\u{15adb}','\u{42bf9}','\u{cf43}','\u{82475}','\u{10a27d}','\u{69422}','\u{8e51d}','\u{d350a}'];
_8 = _2;
_5 = !3179698867_u32;
_7 = _5 as isize;
RET = [(-66173452908262901594778814585575969434_i128),(-121240500694120157918885073934856160109_i128),(-84837611319161626433662695083421266607_i128),148052291277554952543328693828751004422_i128,(-103251871530998339423046664675968294978_i128),(-153980895766643455357278201228935966694_i128),122885114316637880036530563612066032131_i128];
_3 = _2 as f32;
RET = [(-76119482587864449445363459323436505725_i128),(-143551326093831288937365201649757500496_i128),(-99992445626743453502139320120624021560_i128),(-157991389939600448535640361552287226558_i128),(-43444794064922921865568533122777558153_i128),(-32830288211538137986521607279134115398_i128),26627964693043848631015333085090715_i128];
_5 = 1207390605_u32;
Call(_8 = core::intrinsics::bswap(_2), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = [149903203729277164345052139223146471900_i128,(-162171925131116267400235797439806168900_i128),85312281048209454536394339155585211172_i128,76003239359953485728253633031585855735_i128,141274097932937666996432579042726608187_i128,67660364064345883660880329607019471844_i128,168668077461717721473937668605212890341_i128];
_1 = _6;
Call(_6 = fn7(_1, _1, _5, _1, _1, RET, RET, _2, _7, _1, RET, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb6 = {
_3 = _7 as f32;
_8 = _7 >> _7;
_1 = _6;
_9 = [(-87087874372253928_i64)];
_1 = ['\u{8c2f1}','\u{d00a0}','\u{e1fd}','\u{b95b}','\u{d4d11}','\u{35f7b}','\u{ba6e}','\u{2bd18}'];
_9 = [(-6422096116344340885_i64)];
_4 = 46512_u16 as f64;
RET = [152455642901577667024479167600519568585_i128,119611720016814196469185838672573747182_i128,143804186039154484356514055392197997691_i128,(-41164678330781521625822613013450822183_i128),137407858573837768047499794219851256899_i128,(-17731832053310937166290481314683694133_i128),36436427009034369041467021399303735905_i128];
RET = [(-48071655766767016462615873159011763117_i128),(-117159339385473687806200080416834489737_i128),(-104946614848673022335712943559997838029_i128),(-52235882610131694292127675423188859962_i128),98510265409501557108715914811683337293_i128,148814897602911907553521398224422213969_i128,(-60668787688812156193236492400071407818_i128)];
_6 = ['\u{a12a3}','\u{c67dd}','\u{845e4}','\u{f64c5}','\u{7c3b0}','\u{b2237}','\u{edbd8}','\u{59db7}'];
_8 = !_7;
_11 = _4;
RET = [(-81399543304842592097465955012206560419_i128),102531360027389060947905770299886912225_i128,146885784895396949952721996928846534304_i128,(-90575716646385568722353264220133829084_i128),118112463538553602328463666003968940102_i128,154743071299479944992994926653335099846_i128,(-144900256917745367099588073408062276586_i128)];
_3 = 30029_u16 as f32;
_7 = 121_i8 as isize;
RET = [(-139838077162416067686162205612838633410_i128),86333645119595356576132229543054059957_i128,133457167663799144081796918571770079312_i128,(-23781199832047189497044335650704345182_i128),(-109762963998033812517783300037053148938_i128),(-119545786003547393596746760912733828117_i128),(-110125227028444547632161190843580522_i128)];
_6 = _1;
RET = [57755046664811432929881685636594251914_i128,93637794062197706033207734350028220629_i128,(-74606940712239518315526907008624943882_i128),81798300929670817779805208852652178113_i128,69895309562266952043014544332981544631_i128,132356751317901455431607355913665171964_i128,(-142667856015203670902998116100503561667_i128)];
_1 = _6;
RET = [29539616257342736547620199749839730966_i128,6348519910106563135098412327999007149_i128,40368613724177452907694277130738556355_i128,(-143952172243712292960164576607057514222_i128),(-160324872229472864961432175798642762283_i128),13175475842343435214723912278204927999_i128,26462628923263700717640188885753653107_i128];
Goto(bb7)
}
bb7 = {
_6 = ['\u{5a2e6}','\u{e4b5}','\u{33bdb}','\u{6252f}','\u{d2815}','\u{e75f7}','\u{ad7af}','\u{e70fa}'];
_8 = !_7;
_8 = _7 >> _2;
_2 = _8;
_9 = [5808089943937872432_i64];
_1 = ['\u{5269c}','\u{b85a5}','\u{10e2da}','\u{1034e5}','\u{c283b}','\u{c4ac6}','\u{eed57}','\u{8cba9}'];
_9 = [(-6158814580605936046_i64)];
_10 = [58309_u16,26503_u16];
RET = [(-165031850019422528918792097413482625868_i128),46823668414240728874540387625785985942_i128,(-107918702830643005356289249034690016142_i128),(-69921243772701250647234498267676364686_i128),142843508929845305249019531874419689663_i128,137409335140085040189474791082785431226_i128,(-37304771507610089047008871599658727821_i128)];
_11 = -_4;
_7 = _11 as isize;
_6 = ['\u{404d2}','\u{c0e13}','\u{7cd2d}','\u{7eaa2}','\u{d0c38}','\u{999a8}','\u{f152e}','\u{aaf0a}'];
_11 = _4 + _4;
_10 = [32793_u16,62408_u16];
_12 = (-9_i8);
_9 = [(-5256717474499718283_i64)];
_6 = _1;
_5 = !2080702368_u32;
_11 = _4;
_2 = _8 >> _5;
_11 = _4 - _4;
_4 = -_11;
_10 = [36446_u16,50112_u16];
_8 = _2;
_7 = _2 - _8;
_14 = ['\u{fe5d7}','\u{49dee}','\u{c7bd2}','\u{103044}','\u{a82c6}','\u{3a1fd}','\u{6cf3f}','\u{da27b}'];
_10 = [58922_u16,60702_u16];
_7 = 31557_u16 as isize;
_7 = -_2;
Goto(bb8)
}
bb8 = {
_4 = -_11;
_4 = 230_u8 as f64;
_4 = -_11;
_4 = _11 - _11;
_15 = [(-7717030645222699582_i64)];
_11 = _4 - _4;
RET = [111841182491394159029182565233848459179_i128,26815742735894700585697909411888812444_i128,102781681802420569068375623480983386102_i128,6723662721721032506738896837871505211_i128,(-84963740904203998071303048320874952335_i128),27293481118013560357867864104746905367_i128,(-8058513138382424628366415811821117863_i128)];
match _12 {
0 => bb9,
1 => bb10,
2 => bb11,
3 => bb12,
4 => bb13,
5 => bb14,
340282366920938463463374607431768211447 => bb16,
_ => bb15
}
}
bb9 = {
_6 = ['\u{5a2e6}','\u{e4b5}','\u{33bdb}','\u{6252f}','\u{d2815}','\u{e75f7}','\u{ad7af}','\u{e70fa}'];
_8 = !_7;
_8 = _7 >> _2;
_2 = _8;
_9 = [5808089943937872432_i64];
_1 = ['\u{5269c}','\u{b85a5}','\u{10e2da}','\u{1034e5}','\u{c283b}','\u{c4ac6}','\u{eed57}','\u{8cba9}'];
_9 = [(-6158814580605936046_i64)];
_10 = [58309_u16,26503_u16];
RET = [(-165031850019422528918792097413482625868_i128),46823668414240728874540387625785985942_i128,(-107918702830643005356289249034690016142_i128),(-69921243772701250647234498267676364686_i128),142843508929845305249019531874419689663_i128,137409335140085040189474791082785431226_i128,(-37304771507610089047008871599658727821_i128)];
_11 = -_4;
_7 = _11 as isize;
_6 = ['\u{404d2}','\u{c0e13}','\u{7cd2d}','\u{7eaa2}','\u{d0c38}','\u{999a8}','\u{f152e}','\u{aaf0a}'];
_11 = _4 + _4;
_10 = [32793_u16,62408_u16];
_12 = (-9_i8);
_9 = [(-5256717474499718283_i64)];
_6 = _1;
_5 = !2080702368_u32;
_11 = _4;
_2 = _8 >> _5;
_11 = _4 - _4;
_4 = -_11;
_10 = [36446_u16,50112_u16];
_8 = _2;
_7 = _2 - _8;
_14 = ['\u{fe5d7}','\u{49dee}','\u{c7bd2}','\u{103044}','\u{a82c6}','\u{3a1fd}','\u{6cf3f}','\u{da27b}'];
_10 = [58922_u16,60702_u16];
_7 = 31557_u16 as isize;
_7 = -_2;
Goto(bb8)
}
bb10 = {
_3 = _7 as f32;
_8 = _7 >> _7;
_1 = _6;
_9 = [(-87087874372253928_i64)];
_1 = ['\u{8c2f1}','\u{d00a0}','\u{e1fd}','\u{b95b}','\u{d4d11}','\u{35f7b}','\u{ba6e}','\u{2bd18}'];
_9 = [(-6422096116344340885_i64)];
_4 = 46512_u16 as f64;
RET = [152455642901577667024479167600519568585_i128,119611720016814196469185838672573747182_i128,143804186039154484356514055392197997691_i128,(-41164678330781521625822613013450822183_i128),137407858573837768047499794219851256899_i128,(-17731832053310937166290481314683694133_i128),36436427009034369041467021399303735905_i128];
RET = [(-48071655766767016462615873159011763117_i128),(-117159339385473687806200080416834489737_i128),(-104946614848673022335712943559997838029_i128),(-52235882610131694292127675423188859962_i128),98510265409501557108715914811683337293_i128,148814897602911907553521398224422213969_i128,(-60668787688812156193236492400071407818_i128)];
_6 = ['\u{a12a3}','\u{c67dd}','\u{845e4}','\u{f64c5}','\u{7c3b0}','\u{b2237}','\u{edbd8}','\u{59db7}'];
_8 = !_7;
_11 = _4;
RET = [(-81399543304842592097465955012206560419_i128),102531360027389060947905770299886912225_i128,146885784895396949952721996928846534304_i128,(-90575716646385568722353264220133829084_i128),118112463538553602328463666003968940102_i128,154743071299479944992994926653335099846_i128,(-144900256917745367099588073408062276586_i128)];
_3 = 30029_u16 as f32;
_7 = 121_i8 as isize;
RET = [(-139838077162416067686162205612838633410_i128),86333645119595356576132229543054059957_i128,133457167663799144081796918571770079312_i128,(-23781199832047189497044335650704345182_i128),(-109762963998033812517783300037053148938_i128),(-119545786003547393596746760912733828117_i128),(-110125227028444547632161190843580522_i128)];
_6 = _1;
RET = [57755046664811432929881685636594251914_i128,93637794062197706033207734350028220629_i128,(-74606940712239518315526907008624943882_i128),81798300929670817779805208852652178113_i128,69895309562266952043014544332981544631_i128,132356751317901455431607355913665171964_i128,(-142667856015203670902998116100503561667_i128)];
_1 = _6;
RET = [29539616257342736547620199749839730966_i128,6348519910106563135098412327999007149_i128,40368613724177452907694277130738556355_i128,(-143952172243712292960164576607057514222_i128),(-160324872229472864961432175798642762283_i128),13175475842343435214723912278204927999_i128,26462628923263700717640188885753653107_i128];
Goto(bb7)
}
bb11 = {
RET = [149903203729277164345052139223146471900_i128,(-162171925131116267400235797439806168900_i128),85312281048209454536394339155585211172_i128,76003239359953485728253633031585855735_i128,141274097932937666996432579042726608187_i128,67660364064345883660880329607019471844_i128,168668077461717721473937668605212890341_i128];
_1 = _6;
Call(_6 = fn7(_1, _1, _5, _1, _1, RET, RET, _2, _7, _1, RET, _1), ReturnTo(bb6), UnwindUnreachable())
}
bb12 = {
_2 = 4_usize as isize;
_2 = -(-69_isize);
_4 = (-18_i8) as f64;
_5 = 3694147171_u32;
_5 = !2676036537_u32;
RET = [(-146386206359428687753893753479320570503_i128),127091888701797111378585738591369297683_i128,(-117438222615783962041755287312783426603_i128),(-16356147624922369387736985331291188040_i128),(-111164658098293080251102414355735464153_i128),13808983122380227153716040639687852581_i128,71308478286919806428568352761533872952_i128];
_2 = (-117_isize);
_6 = ['\u{a2e16}','\u{49902}','\u{4e43c}','\u{2c384}','\u{fc475}','\u{b20d6}','\u{5951a}','\u{10e224}'];
RET = [135488278007732659336825923530039806057_i128,(-92961441882704473193989544755369504262_i128),89991853099274948507585737788776315432_i128,(-67773263539491637822955203682168941558_i128),(-43401123939874618241891206358131048552_i128),(-108484066515666872215330026930560318301_i128),167971369638345732439809171294811620702_i128];
_4 = 57907_u16 as f64;
_4 = _2 as f64;
RET = [(-129020078386301773927823972420805641136_i128),(-54832450729498377663412413892551856383_i128),(-122770647295197351556851540974453253163_i128),(-149528830341177821489686980127972717968_i128),(-33500308862899253702311082486209596773_i128),88001038454701433175579877872892473432_i128,66598860911836953178708810977337159544_i128];
_6 = ['\u{15adb}','\u{42bf9}','\u{cf43}','\u{82475}','\u{10a27d}','\u{69422}','\u{8e51d}','\u{d350a}'];
_8 = _2;
_5 = !3179698867_u32;
_7 = _5 as isize;
RET = [(-66173452908262901594778814585575969434_i128),(-121240500694120157918885073934856160109_i128),(-84837611319161626433662695083421266607_i128),148052291277554952543328693828751004422_i128,(-103251871530998339423046664675968294978_i128),(-153980895766643455357278201228935966694_i128),122885114316637880036530563612066032131_i128];
_3 = _2 as f32;
RET = [(-76119482587864449445363459323436505725_i128),(-143551326093831288937365201649757500496_i128),(-99992445626743453502139320120624021560_i128),(-157991389939600448535640361552287226558_i128),(-43444794064922921865568533122777558153_i128),(-32830288211538137986521607279134115398_i128),26627964693043848631015333085090715_i128];
_5 = 1207390605_u32;
Call(_8 = core::intrinsics::bswap(_2), ReturnTo(bb5), UnwindUnreachable())
}
bb13 = {
_4 = 10633452867625081686_u64 as f64;
_4 = 1681279901_u32 as f64;
_2 = (-9223372036854775808_isize) >> (-5266761856522847398_i64);
Goto(bb4)
}
bb14 = {
Return()
}
bb15 = {
_2 = -(-9223372036854775808_isize);
RET = [(-91936444577133808056446592065547084541_i128),8786874420941867569557557273220024116_i128,(-40902718809412143712465020536686663080_i128),(-6611406363600539386703880389340671093_i128),(-86640807744785703689933947146948215545_i128),60394666137749451073744101168568231343_i128,(-66422077894146135690390158139808281319_i128)];
RET = [20074592019930053186501983865348265094_i128,121726307029173673966646636328989665200_i128,(-164305792145529631694312217296398866093_i128),4051288970106061537998884095871442722_i128,(-21206156414454851184405371720699421056_i128),84573057106636816613388041573760858054_i128,121022893352974418794144275617572446421_i128];
_3 = 8020_u16 as f32;
_2 = 19339_u16 as isize;
_1 = ['\u{a6ad5}','\u{10779f}','\u{b2d59}','\u{f58e7}','\u{18639}','\u{86334}','\u{1f139}','\u{c8f19}'];
_2 = 9223372036854775807_isize;
match _2 {
9223372036854775807 => bb3,
_ => bb2
}
}
bb16 = {
_8 = _2;
_8 = _2;
_15 = [7824741655136079907_i64];
_16 = [179_u8,198_u8];
_14 = ['\u{9de2f}','\u{a7ecb}','\u{27f40}','\u{5fff9}','\u{39871}','\u{d2d5f}','\u{a4bee}','\u{2bc36}'];
_17 = _7 ^ _2;
_14 = ['\u{92845}','\u{9261c}','\u{2d610}','\u{1b7f}','\u{d5e38}','\u{ff7cb}','\u{791b3}','\u{61210}'];
_4 = _11;
_6 = ['\u{b6c00}','\u{621d1}','\u{8ecdf}','\u{3d750}','\u{3e734}','\u{15c99}','\u{f9573}','\u{b131f}'];
_2 = (-2083535654_i32) as isize;
_9 = _15;
_15 = _9;
_20 = core::ptr::addr_of_mut!(RET);
(*_20) = [(-91677172851298401668231069894492881723_i128),72381369080872754291497846899246466305_i128,161555292280700487469236402472872659461_i128,(-114101532107759428607789289428884918353_i128),20821837810647940893661977587135991430_i128,83830995621756208170561425242077679825_i128,(-60809409551323882353929500962990948490_i128)];
_3 = 160004535909171020654431423202564048118_i128 as f32;
_19.fld0 = [_5,_5,_5,_5,_5,_5];
Goto(bb17)
}
bb17 = {
Call(_21 = dump_var(5_usize, 12_usize, Move(_12), 2_usize, Move(_2), 5_usize, Move(_5), 17_usize, Move(_17)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_21 = dump_var(5_usize, 9_usize, Move(_9), 16_usize, Move(_16), 22_usize, _22, 22_usize, _22), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn6(mut _1: [char; 8],mut _2: [i128; 7],mut _3: [char; 8],mut _4: f32,mut _5: [char; 8],mut _6: [i128; 7],mut _7: isize,mut _8: [char; 8],mut _9: [i128; 7]) -> f64 {
mir! {
type RET = f64;
let _10: i128;
let _11: i64;
let _12: u32;
let _13: i8;
let _14: [u32; 6];
let _15: i64;
let _16: char;
let _17: *mut [u32; 6];
let _18: (*mut i8, i8, *mut i8, char, f32, *mut i128);
let _19: usize;
let _20: [u32; 6];
let _21: i32;
let _22: *mut i8;
let _23: f32;
let _24: char;
let _25: [u32; 6];
let _26: [u8; 2];
let _27: ();
let _28: ();
{
RET = (-10033_i16) as f64;
_7 = (-93156327066417932422822454383462700476_i128) as isize;
_1 = ['\u{12f29}','\u{ab0a0}','\u{8b9a}','\u{9d3ca}','\u{5b5b}','\u{c2d36}','\u{4072b}','\u{8ae43}'];
_3 = ['\u{5078f}','\u{120a}','\u{8f965}','\u{bd58}','\u{cbf4f}','\u{a9800}','\u{998d}','\u{e8cfe}'];
_1 = _8;
Goto(bb1)
}
bb1 = {
_7 = (-1874790293_i32) as isize;
RET = 208790187_i32 as f64;
_6 = [(-43960483100159636000039652273953865426_i128),(-21215856555333857129010926091179547669_i128),147675194969599915351943741448738589329_i128,70058029673664293099454260147223007781_i128,98336771055050510562656428356640972663_i128,143964831545983375559000248803482772399_i128,22902620468050193665472821393256829335_i128];
_5 = ['\u{5530f}','\u{bdced}','\u{3b49f}','\u{9c43f}','\u{eb33d}','\u{b8eae}','\u{c9f1f}','\u{1f5f2}'];
RET = 7139431484150956484_i64 as f64;
_3 = ['\u{9fee4}','\u{4e37a}','\u{2b31e}','\u{41727}','\u{33eac}','\u{e5b2b}','\u{da4cf}','\u{28a79}'];
_10 = 883513390_i32 as i128;
_5 = ['\u{3aecd}','\u{40f94}','\u{296d2}','\u{6b6a9}','\u{ca4e1}','\u{50a81}','\u{a00ff}','\u{a0033}'];
_1 = ['\u{21857}','\u{c7ce5}','\u{bd262}','\u{e27d8}','\u{e1f5f}','\u{b70d9}','\u{dd693}','\u{b0fa0}'];
_4 = 51_i8 as f32;
RET = 74_i8 as f64;
RET = 3988992752272833433_u64 as f64;
_8 = ['\u{9cc60}','\u{f6fa9}','\u{b8443}','\u{d8e2d}','\u{8e0d1}','\u{8bd01}','\u{92c92}','\u{2ab68}'];
_3 = ['\u{10b7c5}','\u{baa38}','\u{c8208}','\u{1424d}','\u{52c14}','\u{f95c1}','\u{27b05}','\u{561e}'];
_5 = _8;
_7 = !9223372036854775807_isize;
_12 = 3032787498_u32 << _10;
_11 = 112_i8 as i64;
Goto(bb2)
}
bb2 = {
_5 = ['\u{10e0d9}','\u{83858}','\u{acf17}','\u{222ec}','\u{87a7a}','\u{8dda7}','\u{123fe}','\u{5630f}'];
_4 = RET as f32;
_4 = (-23_i8) as f32;
_4 = 20459_i16 as f32;
_3 = _5;
_13 = -(-28_i8);
_10 = 149446505506482478533477502743583849319_i128;
_12 = 1509501615_u32 * 4022697987_u32;
_16 = '\u{6d9f}';
_8 = [_16,_16,_16,_16,_16,_16,_16,_16];
_9 = _6;
_13 = -(-77_i8);
_8 = _5;
_16 = '\u{af19a}';
_13 = 51_u8 as i8;
_3 = _1;
_4 = _11 as f32;
_3 = [_16,_16,_16,_16,_16,_16,_16,_16];
_10 = 20831493839392783985636518633877643284_i128;
_9 = [_10,_10,_10,_10,_10,_10,_10];
_8 = _5;
match _10 {
0 => bb3,
1 => bb4,
2 => bb5,
3 => bb6,
4 => bb7,
5 => bb8,
6 => bb9,
20831493839392783985636518633877643284 => bb11,
_ => bb10
}
}
bb3 = {
_7 = (-1874790293_i32) as isize;
RET = 208790187_i32 as f64;
_6 = [(-43960483100159636000039652273953865426_i128),(-21215856555333857129010926091179547669_i128),147675194969599915351943741448738589329_i128,70058029673664293099454260147223007781_i128,98336771055050510562656428356640972663_i128,143964831545983375559000248803482772399_i128,22902620468050193665472821393256829335_i128];
_5 = ['\u{5530f}','\u{bdced}','\u{3b49f}','\u{9c43f}','\u{eb33d}','\u{b8eae}','\u{c9f1f}','\u{1f5f2}'];
RET = 7139431484150956484_i64 as f64;
_3 = ['\u{9fee4}','\u{4e37a}','\u{2b31e}','\u{41727}','\u{33eac}','\u{e5b2b}','\u{da4cf}','\u{28a79}'];
_10 = 883513390_i32 as i128;
_5 = ['\u{3aecd}','\u{40f94}','\u{296d2}','\u{6b6a9}','\u{ca4e1}','\u{50a81}','\u{a00ff}','\u{a0033}'];
_1 = ['\u{21857}','\u{c7ce5}','\u{bd262}','\u{e27d8}','\u{e1f5f}','\u{b70d9}','\u{dd693}','\u{b0fa0}'];
_4 = 51_i8 as f32;
RET = 74_i8 as f64;
RET = 3988992752272833433_u64 as f64;
_8 = ['\u{9cc60}','\u{f6fa9}','\u{b8443}','\u{d8e2d}','\u{8e0d1}','\u{8bd01}','\u{92c92}','\u{2ab68}'];
_3 = ['\u{10b7c5}','\u{baa38}','\u{c8208}','\u{1424d}','\u{52c14}','\u{f95c1}','\u{27b05}','\u{561e}'];
_5 = _8;
_7 = !9223372036854775807_isize;
_12 = 3032787498_u32 << _10;
_11 = 112_i8 as i64;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
Return()
}
bb11 = {
_17 = core::ptr::addr_of_mut!(_14);
_20 = [_12,_12,_12,_12,_12,_12];
_18.2 = core::ptr::addr_of_mut!(_18.1);
_18.2 = core::ptr::addr_of_mut!(_18.1);
_11 = (-3250842137862194547_i64) & 6134432387680288997_i64;
_18.3 = _16;
_12 = _11 as u32;
_21 = 689380871_i32;
_19 = 26100_u16 as usize;
_8 = [_18.3,_16,_18.3,_16,_18.3,_18.3,_18.3,_18.3];
_2 = [_10,_10,_10,_10,_10,_10,_10];
_20 = [_12,_12,_12,_12,_12,_12];
RET = 162604437642957110407749277521721358424_u128 as f64;
_18.3 = _16;
_1 = [_16,_16,_16,_16,_16,_16,_16,_18.3];
_9 = [_10,_10,_10,_10,_10,_10,_10];
RET = 16568211388542905619_u64 as f64;
Call(_2 = core::intrinsics::transmute(_6), ReturnTo(bb12), UnwindUnreachable())
}
bb12 = {
_18.0 = core::ptr::addr_of_mut!(_18.1);
_18.0 = core::ptr::addr_of_mut!(_18.1);
_20 = [_12,_12,_12,_12,_12,_12];
_13 = (-35_i8);
(*_17) = [_12,_12,_12,_12,_12,_12];
_13 = 77_i8;
_20 = _14;
_21 = !1991478693_i32;
_15 = _11 + _11;
_18.3 = _16;
_15 = _11;
_15 = -_11;
_18.2 = core::ptr::addr_of_mut!(_13);
_5 = _1;
match _10 {
0 => bb1,
1 => bb8,
20831493839392783985636518633877643284 => bb14,
_ => bb13
}
}
bb13 = {
Return()
}
bb14 = {
_14 = [_12,_12,_12,_12,_12,_12];
_9 = [_10,_10,_10,_10,_10,_10,_10];
_17 = core::ptr::addr_of_mut!(_20);
_18.1 = _13;
_17 = core::ptr::addr_of_mut!(_25);
_18.4 = -_4;
_24 = _18.3;
(*_17) = _14;
Goto(bb15)
}
bb15 = {
Call(_27 = dump_var(6_usize, 10_usize, Move(_10), 11_usize, Move(_11), 5_usize, Move(_5), 19_usize, Move(_19)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_27 = dump_var(6_usize, 2_usize, Move(_2), 20_usize, Move(_20), 21_usize, Move(_21), 6_usize, Move(_6)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_27 = dump_var(6_usize, 12_usize, Move(_12), 3_usize, Move(_3), 28_usize, _28, 28_usize, _28), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn7(mut _1: [char; 8],mut _2: [char; 8],mut _3: u32,mut _4: [char; 8],mut _5: [char; 8],mut _6: [i128; 7],mut _7: [i128; 7],mut _8: isize,mut _9: isize,mut _10: [char; 8],mut _11: [i128; 7],mut _12: [char; 8]) -> [char; 8] {
mir! {
type RET = [char; 8];
let _13: bool;
let _14: i32;
let _15: Adt54;
let _16: i32;
let _17: i64;
let _18: i128;
let _19: isize;
let _20: Adt52;
let _21: i16;
let _22: Adt54;
let _23: Adt55;
let _24: char;
let _25: [u16; 2];
let _26: u128;
let _27: isize;
let _28: Adt54;
let _29: i16;
let _30: ();
let _31: ();
{
_7 = [128928187737348162254516584527392279155_i128,(-82514223194473662156288494529408000509_i128),124007552606869493849459388755533566894_i128,149549748808473765347263067570830566938_i128,(-146787018482890262728225245461630998971_i128),50731915978119337191433190570316624886_i128,121053615738001615201203724593957616338_i128];
_7 = [(-89726768085451096723007995997942008397_i128),899857365377367938108358632507516290_i128,(-87827024197721736873508840817578568757_i128),(-32467294995515532110177916174927619722_i128),(-70463025697969881967541974749091588757_i128),4740147890151617368645452684512503752_i128,(-133712884408918689234675118037357981014_i128)];
_10 = ['\u{1315d}','\u{16418}','\u{85984}','\u{103b2a}','\u{3550d}','\u{b387a}','\u{6165c}','\u{2c7a4}'];
RET = _12;
_3 = 3020144884_u32;
_3 = !1293871582_u32;
_10 = RET;
_11 = [(-151653056962223028752529261918083980711_i128),(-154787867396367910941329072633726886320_i128),(-102676546225292646739923552036062005647_i128),(-16920751421395685095428749784285858545_i128),(-49949645325068252086427377574104174925_i128),8409245270785249999369214378794614707_i128,89345440624091073552784930032978274314_i128];
_8 = -_9;
_5 = ['\u{105574}','\u{102bbb}','\u{d4460}','\u{3ed35}','\u{3223f}','\u{36a8c}','\u{84435}','\u{7499d}'];
Goto(bb1)
}
bb1 = {
RET = _12;
_13 = !false;
RET = ['\u{c7f85}','\u{38dcd}','\u{248fb}','\u{b34c1}','\u{d7d59}','\u{a76ac}','\u{c191c}','\u{54c46}'];
_15.fld0 = [_3,_3,_3,_3,_3,_3];
_3 = 3273096548_u32 | 3713401098_u32;
RET = _12;
_11 = [(-71562683265690257324798248207242448944_i128),(-40429814349251184772666426040528823709_i128),109264904606893634566195846723581178455_i128,(-107779932218944901189406700212197998004_i128),137142932131265365522437070463663140132_i128,145984116973639175766383304219669177320_i128,(-45804523717449644788435607078937163819_i128)];
_6 = [(-65020908557436959826644833545879658503_i128),119484353908054892854881780108610449420_i128,155650688546452810305966913162927836336_i128,91188581564441573657855021560036070027_i128,(-19536139649177039822525803176179082273_i128),59113418321661472197242090260958636180_i128,44200114419978131722660117719909505740_i128];
_15.fld0 = [_3,_3,_3,_3,_3,_3];
Goto(bb2)
}
bb2 = {
_2 = _5;
_11 = _6;
_11 = [138400499194665857578693800747587976020_i128,(-128324796392791046276459519808216496497_i128),81727059994328133813727047357307894997_i128,(-50526290062574510568890093620914931983_i128),117932058356596700971556856718197684510_i128,130756001896763071592496206604682513269_i128,14652202818739917425267816011753507207_i128];
_16 = !(-1711905212_i32);
_15.fld0 = [_3,_3,_3,_3,_3,_3];
_7 = [(-119150083060129110999686858964449144038_i128),108981026402656776790076038805263990589_i128,(-146665897741804516767151417736785501503_i128),(-152554927883610041379482763746833515084_i128),(-64102424186491255646206923677697540855_i128),90115904957186155932266919464065057831_i128,142208927417199697698863749641213009213_i128];
_6 = [(-27355924107207628124709294495533833189_i128),133020447027978055248720765122151745497_i128,140367117517198346033341148820661363175_i128,38843410109781365048473638257250959015_i128,102820491016874426251977453431770012533_i128,64869705234855839245359280709318140944_i128,(-89660263770556623369892964039587419774_i128)];
_14 = _16 << _8;
_15.fld0 = [_3,_3,_3,_3,_3,_3];
_10 = ['\u{10568e}','\u{100b49}','\u{2822a}','\u{a0ed1}','\u{3d060}','\u{59bcb}','\u{44f9e}','\u{69749}'];
_10 = ['\u{aa4eb}','\u{107437}','\u{1a8ae}','\u{3abce}','\u{cff91}','\u{b9fb0}','\u{dc740}','\u{2f5a3}'];
_15.fld1 = [166887615264541764002802772437075925462_i128,30558166311463010270838941883704924671_i128,(-98572922508472476377418069756270058952_i128),59029490043469025053367525656423134117_i128,85219381847267547800218384993311699380_i128,(-82482182355530595533171286365199577082_i128),135286111688000497155958456082178960902_i128,96394426118865573858152940334931279833_i128];
_8 = _9 & _9;
_3 = 3714444013_u32 - 1806195199_u32;
_15.fld0 = [_3,_3,_3,_3,_3,_3];
_4 = ['\u{79991}','\u{b0d6d}','\u{896d1}','\u{f80}','\u{63f01}','\u{c442b}','\u{a8806}','\u{36d2c}'];
_14 = _8 as i32;
_7 = [(-139703860531873274738814887560599317285_i128),159969456511111119390504697533751383295_i128,(-35913131484241539685940503069776877381_i128),(-31399580236762990728882644456406382668_i128),(-27902990374184951001455553641850202818_i128),44173900271877584404419306189748387928_i128,(-61690776324692391035044853366606171042_i128)];
_4 = _5;
_10 = ['\u{799dc}','\u{50404}','\u{5bec6}','\u{e01f5}','\u{ce9eb}','\u{513e4}','\u{bb9e}','\u{546d5}'];
_9 = _8;
Goto(bb3)
}
bb3 = {
_16 = _3 as i32;
_6 = _11;
_6 = [(-108284999487106406145385244482198339781_i128),(-19810339076972912591916879978875185199_i128),57319438278405157862040802980426095700_i128,69031848067788655594154841796402143660_i128,(-54257115206062224229272173616688225405_i128),(-62618629085214222861701972796055308684_i128),23997224000915463087899830211591810764_i128];
Goto(bb4)
}
bb4 = {
_4 = ['\u{d693f}','\u{bb5a7}','\u{1067fd}','\u{b159e}','\u{17eac}','\u{b4a0d}','\u{59351}','\u{99fa7}'];
_8 = 85_u8 as isize;
_17 = (-803053416710692707_i64);
RET = _4;
_15.fld1 = [164071096490066114605396532586486121039_i128,(-39229275473682525000409736905046746925_i128),48350533821905997286917624888382439142_i128,137651435561712791417647764517723962180_i128,(-130275516747265805611751968863276113530_i128),(-140349849373449446506892592460965513866_i128),84345785861562247747299203263240586893_i128,26371432657000225569826878418575580452_i128];
_8 = _9;
_18 = _13 as i128;
RET = ['\u{5a62e}','\u{4ae28}','\u{f2bf3}','\u{1a890}','\u{764e7}','\u{1bc4d}','\u{7ca38}','\u{e419d}'];
_10 = ['\u{f4da0}','\u{b3ec7}','\u{53df5}','\u{fab14}','\u{ed8c7}','\u{13055}','\u{57033}','\u{3701c}'];
_9 = _8 | _8;
_14 = !_16;
_15.fld1 = [_18,_18,_18,_18,_18,_18,_18,_18];
_2 = ['\u{aca36}','\u{6d265}','\u{18ad0}','\u{51abd}','\u{88bd2}','\u{fe44b}','\u{87d6b}','\u{28560}'];
_18 = 16913810078819228461_u64 as i128;
_9 = _8 & _8;
_18 = 23246755333236520304034123763242815046_i128 ^ (-58267998587689592262517841012555713679_i128);
RET = _5;
_17 = (-6065511944356385239_i64);
Call(_14 = fn8(_9, _5, _11), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
RET = ['\u{e4138}','\u{d1cec}','\u{730f7}','\u{5074a}','\u{fbd7d}','\u{1358a}','\u{68fec}','\u{77762}'];
_18 = 141708015606494032447126937374632662540_i128 * (-79676759260946066671132634274348781777_i128);
_19 = _8;
_21 = 35091_u16 as i16;
_20 = Adt52::Variant0 { fld0: 238678576782576421612235126527243394574_u128 };
_14 = _16;
_16 = 2_usize as i32;
_12 = _5;
_11 = [_18,_18,_18,_18,_18,_18,_18];
place!(Field::<u128>(Variant(_20, 0), 0)) = _17 as u128;
_3 = Field::<u128>(Variant(_20, 0), 0) as u32;
_10 = ['\u{9923b}','\u{873db}','\u{c5bb0}','\u{c1931}','\u{9e2d}','\u{712e}','\u{b3207}','\u{35370}'];
_15.fld0 = [_3,_3,_3,_3,_3,_3];
_16 = '\u{5a761}' as i32;
_19 = _8 ^ _8;
_4 = _2;
RET = ['\u{b7c31}','\u{9fd75}','\u{fdf2b}','\u{e4240}','\u{6e95c}','\u{79a12}','\u{dc7dc}','\u{f10f7}'];
_8 = _13 as isize;
Goto(bb6)
}
bb6 = {
_9 = _19;
SetDiscriminant(_20, 1);
_10 = ['\u{402cf}','\u{3ef3a}','\u{10e6de}','\u{104ef7}','\u{3779f}','\u{191db}','\u{7d490}','\u{f4dc5}'];
place!(Field::<bool>(Variant(_20, 1), 0)) = _13 ^ _13;
Goto(bb7)
}
bb7 = {
_15.fld1 = [_18,_18,_18,_18,_18,_18,_18,_18];
_1 = _2;
_4 = _12;
_18 = 54702358263906803088777317190657543788_i128;
match _18 {
0 => bb5,
1 => bb2,
2 => bb3,
3 => bb8,
4 => bb9,
5 => bb10,
54702358263906803088777317190657543788 => bb12,
_ => bb11
}
}
bb8 = {
_9 = _19;
SetDiscriminant(_20, 1);
_10 = ['\u{402cf}','\u{3ef3a}','\u{10e6de}','\u{104ef7}','\u{3779f}','\u{191db}','\u{7d490}','\u{f4dc5}'];
place!(Field::<bool>(Variant(_20, 1), 0)) = _13 ^ _13;
Goto(bb7)
}
bb9 = {
RET = _12;
_13 = !false;
RET = ['\u{c7f85}','\u{38dcd}','\u{248fb}','\u{b34c1}','\u{d7d59}','\u{a76ac}','\u{c191c}','\u{54c46}'];
_15.fld0 = [_3,_3,_3,_3,_3,_3];
_3 = 3273096548_u32 | 3713401098_u32;
RET = _12;
_11 = [(-71562683265690257324798248207242448944_i128),(-40429814349251184772666426040528823709_i128),109264904606893634566195846723581178455_i128,(-107779932218944901189406700212197998004_i128),137142932131265365522437070463663140132_i128,145984116973639175766383304219669177320_i128,(-45804523717449644788435607078937163819_i128)];
_6 = [(-65020908557436959826644833545879658503_i128),119484353908054892854881780108610449420_i128,155650688546452810305966913162927836336_i128,91188581564441573657855021560036070027_i128,(-19536139649177039822525803176179082273_i128),59113418321661472197242090260958636180_i128,44200114419978131722660117719909505740_i128];
_15.fld0 = [_3,_3,_3,_3,_3,_3];
Goto(bb2)
}
bb10 = {
_4 = ['\u{d693f}','\u{bb5a7}','\u{1067fd}','\u{b159e}','\u{17eac}','\u{b4a0d}','\u{59351}','\u{99fa7}'];
_8 = 85_u8 as isize;
_17 = (-803053416710692707_i64);
RET = _4;
_15.fld1 = [164071096490066114605396532586486121039_i128,(-39229275473682525000409736905046746925_i128),48350533821905997286917624888382439142_i128,137651435561712791417647764517723962180_i128,(-130275516747265805611751968863276113530_i128),(-140349849373449446506892592460965513866_i128),84345785861562247747299203263240586893_i128,26371432657000225569826878418575580452_i128];
_8 = _9;
_18 = _13 as i128;
RET = ['\u{5a62e}','\u{4ae28}','\u{f2bf3}','\u{1a890}','\u{764e7}','\u{1bc4d}','\u{7ca38}','\u{e419d}'];
_10 = ['\u{f4da0}','\u{b3ec7}','\u{53df5}','\u{fab14}','\u{ed8c7}','\u{13055}','\u{57033}','\u{3701c}'];
_9 = _8 | _8;
_14 = !_16;
_15.fld1 = [_18,_18,_18,_18,_18,_18,_18,_18];
_2 = ['\u{aca36}','\u{6d265}','\u{18ad0}','\u{51abd}','\u{88bd2}','\u{fe44b}','\u{87d6b}','\u{28560}'];
_18 = 16913810078819228461_u64 as i128;
_9 = _8 & _8;
_18 = 23246755333236520304034123763242815046_i128 ^ (-58267998587689592262517841012555713679_i128);
RET = _5;
_17 = (-6065511944356385239_i64);
Call(_14 = fn8(_9, _5, _11), ReturnTo(bb5), UnwindUnreachable())
}
bb11 = {
_16 = _3 as i32;
_6 = _11;
_6 = [(-108284999487106406145385244482198339781_i128),(-19810339076972912591916879978875185199_i128),57319438278405157862040802980426095700_i128,69031848067788655594154841796402143660_i128,(-54257115206062224229272173616688225405_i128),(-62618629085214222861701972796055308684_i128),23997224000915463087899830211591810764_i128];
Goto(bb4)
}
bb12 = {
_22.fld0 = [_3,_3,_3,_3,_3,_3];
Call(_3 = fn9(_10, _7, _1, _7, _18, RET, _9, _4, _12), ReturnTo(bb13), UnwindUnreachable())
}
bb13 = {
place!(Field::<bool>(Variant(_20, 1), 0)) = _13;
_15.fld0 = _22.fld0;
_18 = -(-143489576196886252732813211528043511168_i128);
place!(Field::<[i128; 8]>(Variant(_20, 1), 5)) = [_18,_18,_18,_18,_18,_18,_18,_18];
_15 = Adt54 { fld0: _22.fld0,fld1: Field::<[i128; 8]>(Variant(_20, 1), 5) };
place!(Field::<[u8; 2]>(Variant(_20, 1), 4)) = [91_u8,21_u8];
place!(Field::<[u8; 2]>(Variant(_20, 1), 4)) = [11_u8,167_u8];
_11 = [_18,_18,_18,_18,_18,_18,_18];
_21 = 5_usize as i16;
place!(Field::<Adt46>(Variant(_20, 1), 6)) = Adt46::Variant2 { fld0: Field::<bool>(Variant(_20, 1), 0) };
_17 = -3481827730718946218_i64;
_11 = [_18,_18,_18,_18,_18,_18,_18];
_17 = -5762433986089732231_i64;
_18 = 124879748049095823035556579136867193111_i128 * 5736050060905753630601570557632359506_i128;
_22.fld1 = Field::<[i128; 8]>(Variant(_20, 1), 5);
_9 = _19;
place!(Field::<bool>(Variant(place!(Field::<Adt46>(Variant(_20, 1), 6)), 2), 0)) = _14 <= _16;
_13 = !Field::<bool>(Variant(Field::<Adt46>(Variant(_20, 1), 6), 2), 0);
_21 = !9134_i16;
place!(Field::<[u32; 6]>(Variant(_20, 1), 3)) = _22.fld0;
_15.fld0 = [_3,_3,_3,_3,_3,_3];
_6 = _7;
_15.fld1 = [_18,_18,_18,_18,_18,_18,_18,_18];
Call(_11 = core::intrinsics::transmute(_6), ReturnTo(bb14), UnwindUnreachable())
}
bb14 = {
place!(Field::<[u8; 2]>(Variant(_20, 1), 4)) = [153_u8,233_u8];
_15.fld0 = _22.fld0;
_10 = ['\u{108593}','\u{4b5a9}','\u{472dd}','\u{9b21b}','\u{4aa12}','\u{ab843}','\u{4f0ab}','\u{103d7e}'];
_21 = -(-95_i16);
place!(Field::<[u32; 6]>(Variant(_20, 1), 3)) = [_3,_3,_3,_3,_3,_3];
_5 = ['\u{22226}','\u{10adfb}','\u{ad714}','\u{638c6}','\u{f5736}','\u{9e971}','\u{fb7ff}','\u{45b0b}'];
_16 = _14;
_1 = ['\u{f844}','\u{5930b}','\u{441fa}','\u{cf0c3}','\u{68560}','\u{d72e8}','\u{71cdc}','\u{6f69e}'];
_8 = -_19;
place!(Field::<[u8; 2]>(Variant(_20, 1), 4)) = [126_u8,217_u8];
RET = ['\u{d5787}','\u{88280}','\u{459f9}','\u{ee262}','\u{b7439}','\u{d9c90}','\u{9f7d5}','\u{56565}'];
_24 = '\u{eacb3}';
_11 = [_18,_18,_18,_18,_18,_18,_18];
_22 = _15;
_22.fld0 = [_3,_3,_3,_3,_3,_3];
_15 = Adt54 { fld0: Field::<[u32; 6]>(Variant(_20, 1), 3),fld1: _22.fld1 };
_22.fld0 = [_3,_3,_3,_3,_3,_3];
place!(Field::<[u32; 6]>(Variant(_20, 1), 3)) = [_3,_3,_3,_3,_3,_3];
_27 = -_19;
place!(Field::<[u8; 2]>(Variant(_20, 1), 4)) = [247_u8,57_u8];
_16 = _14 * _14;
_26 = !123156365499875976046232425866353956757_u128;
_10 = _5;
RET = [_24,_24,_24,_24,_24,_24,_24,_24];
_11 = [_18,_18,_18,_18,_18,_18,_18];
_25 = [58668_u16,45290_u16];
_6 = _7;
Goto(bb15)
}
bb15 = {
Call(_30 = dump_var(7_usize, 1_usize, Move(_1), 6_usize, Move(_6), 3_usize, Move(_3), 7_usize, Move(_7)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_30 = dump_var(7_usize, 17_usize, Move(_17), 5_usize, Move(_5), 4_usize, Move(_4), 21_usize, Move(_21)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_30 = dump_var(7_usize, 19_usize, Move(_19), 10_usize, Move(_10), 13_usize, Move(_13), 31_usize, _31), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn8(mut _1: isize,mut _2: [char; 8],mut _3: [i128; 7]) -> i32 {
mir! {
type RET = i32;
let _4: u16;
let _5: u32;
let _6: usize;
let _7: *mut char;
let _8: isize;
let _9: isize;
let _10: u16;
let _11: [u32; 6];
let _12: (char, &'static i8);
let _13: [u32; 6];
let _14: Adt45;
let _15: [u8; 5];
let _16: *mut [i128; 7];
let _17: Adt46;
let _18: [u16; 2];
let _19: f32;
let _20: isize;
let _21: i16;
let _22: [char; 8];
let _23: Adt54;
let _24: i32;
let _25: Adt54;
let _26: [u8; 5];
let _27: (*mut *mut i8,);
let _28: (bool, &'static i8);
let _29: i128;
let _30: f32;
let _31: [u8; 5];
let _32: [u32; 6];
let _33: [u8; 5];
let _34: isize;
let _35: [i128; 7];
let _36: isize;
let _37: Adt55;
let _38: Adt54;
let _39: [u8; 5];
let _40: [u8; 5];
let _41: [i64; 1];
let _42: Adt44;
let _43: bool;
let _44: u16;
let _45: isize;
let _46: [i64; 1];
let _47: usize;
let _48: isize;
let _49: u64;
let _50: ();
let _51: ();
{
RET = 4817402662347011022_i64 as i32;
RET = 1_i8 as i32;
_1 = false as isize;
RET = 527915604_i32;
RET = (-937566883_i32);
_3 = [147430112761980635972712036441339434570_i128,(-138862857134966083931211285133884920274_i128),150073449810252580946680468825918814235_i128,(-124060297753619628395714068694582909028_i128),32934274864660486470727229515901970789_i128,107111605353015684127784879668778912457_i128,89769831800794486869714541887831339415_i128];
_3 = [53594589331548145623601189542207936400_i128,114134395918962764751147781774173976130_i128,142906778492613911433548845026488807422_i128,157204154042043816102873100159089559128_i128,(-130063730453261488545058627505224613303_i128),(-119022528026748435936357190486161979807_i128),19927375966773300820255404247615963871_i128];
RET = (-1611042843_i32);
_1 = !108_isize;
RET = -(-1151163645_i32);
_3 = [(-106329688273157474525327821004603139087_i128),52067888947240145917275140344261755976_i128,101052370273058001372232275283076906617_i128,(-45520210211975473607323610469355114121_i128),106516298768113163797442248105148041111_i128,(-115693919612406502600326461907366590568_i128),(-139923578895911690575311227551255160204_i128)];
RET = 1880176294_i32 << _1;
RET = 579786150_i32 ^ (-1860242105_i32);
RET = (-2139613541_i32);
_2 = ['\u{aa565}','\u{94a23}','\u{43482}','\u{ad7fa}','\u{880ae}','\u{21653}','\u{648e2}','\u{9373e}'];
_2 = ['\u{14215}','\u{96d0e}','\u{b596d}','\u{7a822}','\u{2ac43}','\u{5e4b3}','\u{a30c4}','\u{23a7d}'];
_4 = !40547_u16;
RET = (-1048059990_i32) << _4;
Goto(bb1)
}
bb1 = {
RET = -(-1128729554_i32);
_2 = ['\u{180aa}','\u{dfa40}','\u{e987a}','\u{533ba}','\u{ea8ab}','\u{aefd0}','\u{648b9}','\u{fc409}'];
_1 = (-32_isize);
_5 = !3810592130_u32;
Goto(bb2)
}
bb2 = {
_4 = !58134_u16;
RET = 79_i8 as i32;
_5 = _1 as u32;
_3 = [(-39652976806071289364322034110041474534_i128),(-164576149144126571951307661707688523270_i128),102188144859731008222023805047885781813_i128,119538562732242079107019601377218862722_i128,21520026628307855731313475117506662555_i128,(-89481284724292844567411062630825907899_i128),(-167889717535901992890096411615424343922_i128)];
RET = !940194035_i32;
_5 = !2102480486_u32;
_3 = [120280668845323047806738563398645224847_i128,(-37962339219299387526227707723110438436_i128),162986332705417009488846244838655898491_i128,160017504512643398169559326949870087215_i128,105393140569125470167115067781727069616_i128,92495196316300854043062441586544484729_i128,153230576476712005615151381332952668486_i128];
_3 = [101214494411854175068707345154291547495_i128,139545362140889948623058158495900753565_i128,720688622383000369970240987278034856_i128,28066640777929643271384688740345366036_i128,(-131612312477154755158400031049366331907_i128),145113442774264257020264426925116740376_i128,(-52992456621618744934960614656220400748_i128)];
RET = (-1740721453_i32) * 545528173_i32;
_6 = 13530214923886997807_usize - 3_usize;
RET = _1 as i32;
_6 = !8700288720290048649_usize;
_4 = !37029_u16;
_5 = 206671278_u32 + 2168679569_u32;
_3 = [(-36793864273406511532698045666777492154_i128),5741186544812593729286718548804840251_i128,(-136840430750840053408508672647244475658_i128),(-76167859371736302505120917371191606863_i128),(-156189613135520579693207036640027615566_i128),(-48616081487881266632432894078169427281_i128),(-91478945843163166777979255302236170085_i128)];
_4 = 31637_u16 - 16492_u16;
_2 = ['\u{1225f}','\u{79c1c}','\u{d2dcb}','\u{7f2fe}','\u{113dc}','\u{9b745}','\u{9e4b2}','\u{62783}'];
_2 = ['\u{31c19}','\u{5295f}','\u{3cd3d}','\u{dc8e3}','\u{c7fdf}','\u{3effd}','\u{6f6ec}','\u{d514c}'];
Call(_9 = core::intrinsics::bswap(_1), ReturnTo(bb3), UnwindUnreachable())
}
bb3 = {
RET = (-2131400965_i32) << _4;
_3 = [153630769778887551365715658582170117756_i128,(-109276464745972069854560150802455974766_i128),60530550128103511707806739627586919651_i128,21616660333034672241118868877195014703_i128,(-73835705172066099824799774478688429034_i128),(-6301697795849892884460002966143594771_i128),(-45637320650477789708935172231588350463_i128)];
_8 = -_1;
_2 = ['\u{c664c}','\u{fe90a}','\u{17e3f}','\u{2797}','\u{39766}','\u{a6c56}','\u{33868}','\u{d3af5}'];
_10 = _4 & _4;
RET = 1852603573_i32 << _5;
_3 = [20734064838055536428415480283684399947_i128,(-75243564657319504692606754691722450512_i128),(-113363144135699021284727207315923313683_i128),65555403879307508704982448886446337105_i128,150400347727872560129225389586834373498_i128,97415862307130526622986584490980594610_i128,(-30915645367797680604489217302585278759_i128)];
_8 = _1;
_10 = !_4;
_1 = _5 as isize;
RET = (-184537492_i32);
RET = (-2136019165_i32);
RET = 188_u8 as i32;
_7 = core::ptr::addr_of_mut!(_12.0);
(*_7) = '\u{3ce4c}';
_3 = [(-113180463522267859552514766358379085875_i128),95308979111151128878107363345641259729_i128,(-133691380753631634560222202144184721586_i128),(-19415756898449612398496952705503837213_i128),118063553549425473023282433535340069035_i128,(-95860379903144433294701952708606292699_i128),(-2770211440684580753637524971748601588_i128)];
_12.0 = '\u{10f22b}';
(*_7) = '\u{f6c3a}';
RET = (-2088124582_i32);
_7 = core::ptr::addr_of_mut!(_12.0);
_8 = _1 << _10;
_4 = _10 ^ _10;
Goto(bb4)
}
bb4 = {
RET = !(-1202613563_i32);
_15 = [217_u8,103_u8,146_u8,141_u8,229_u8];
_12.0 = '\u{fb90d}';
_17 = Adt46::Variant2 { fld0: true };
_6 = 6_usize - 7873356729324632918_usize;
_6 = !7_usize;
_13 = [_5,_5,_5,_5,_5,_5];
place!(Field::<bool>(Variant(_17, 2), 0)) = !false;
_7 = core::ptr::addr_of_mut!(_12.0);
RET = 449093489_i32;
RET = (-59649727_i32) & (-1925895808_i32);
_20 = _6 as isize;
_19 = 9050281103829233800401504899168139304_u128 as f32;
SetDiscriminant(_17, 2);
_6 = 13333250207130852217_u64 as usize;
_15 = [222_u8,140_u8,57_u8,162_u8,52_u8];
_18 = [_4,_4];
_13 = [_5,_5,_5,_5,_5,_5];
Goto(bb5)
}
bb5 = {
_19 = RET as f32;
_13 = [_5,_5,_5,_5,_5,_5];
_12.0 = '\u{109c50}';
(*_7) = '\u{84869}';
_6 = 0_usize;
_13 = [_5,_5,_5,_5,_5,_5];
_23.fld0 = _13;
_13[_6] = _23.fld0[_6];
_11 = _23.fld0;
RET = !1402694298_i32;
_11 = [_13[_6],_23.fld0[_6],_13[_6],_13[_6],_23.fld0[_6],_13[_6]];
_16 = core::ptr::addr_of_mut!(_3);
_22 = [_2[_6],_12.0,_2[_6],_2[_6],_2[_6],_2[_6],(*_7),_2[_6]];
_21 = 15062_i16;
_23.fld1 = [(*_16)[_6],_3[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6]];
_13[_6] = _11[_6] + _5;
_11 = [_5,_13[_6],_23.fld0[_6],_13[_6],_23.fld0[_6],_5];
_13[_6] = _11[_6] & _23.fld0[_6];
_22[_6] = (*_7);
_7 = core::ptr::addr_of_mut!((*_7));
_22 = [_2[_6],_2[_6],_2[_6],(*_7),_2[_6],_2[_6],_2[_6],_2[_6]];
_2 = [_12.0,_12.0,(*_7),_22[_6],_22[_6],_12.0,_22[_6],_22[_6]];
_2[_6] = _22[_6];
_23.fld1 = [(*_16)[_6],(*_16)[_6],(*_16)[_6],_3[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6],_3[_6]];
match (*_16)[_6] {
0 => bb2,
227101903398670603910859841073389125581 => bb7,
_ => bb6
}
}
bb6 = {
RET = !(-1202613563_i32);
_15 = [217_u8,103_u8,146_u8,141_u8,229_u8];
_12.0 = '\u{fb90d}';
_17 = Adt46::Variant2 { fld0: true };
_6 = 6_usize - 7873356729324632918_usize;
_6 = !7_usize;
_13 = [_5,_5,_5,_5,_5,_5];
place!(Field::<bool>(Variant(_17, 2), 0)) = !false;
_7 = core::ptr::addr_of_mut!(_12.0);
RET = 449093489_i32;
RET = (-59649727_i32) & (-1925895808_i32);
_20 = _6 as isize;
_19 = 9050281103829233800401504899168139304_u128 as f32;
SetDiscriminant(_17, 2);
_6 = 13333250207130852217_u64 as usize;
_15 = [222_u8,140_u8,57_u8,162_u8,52_u8];
_18 = [_4,_4];
_13 = [_5,_5,_5,_5,_5,_5];
Goto(bb5)
}
bb7 = {
(*_16)[_6] = _23.fld1[_6] | _23.fld1[_6];
_29 = -(*_16)[_6];
(*_16) = [_23.fld1[_6],_29,_29,_29,_23.fld1[_6],_29,_29];
_23.fld0[_6] = _5;
_25.fld1[_6] = -_3[_6];
_13[_6] = RET as u32;
_24 = -RET;
(*_7) = _22[_6];
_1 = _8;
_15[_6] = 5064271546821021260_u64 as u8;
_26 = [_15[_6],_15[_6],_15[_6],_15[_6],_15[_6]];
_28.0 = !false;
_11 = [_13[_6],_23.fld0[_6],_13[_6],_13[_6],_5,_5];
_24 = RET - RET;
place!(Field::<bool>(Variant(_17, 2), 0)) = !_28.0;
match _21 {
0 => bb5,
1 => bb8,
2 => bb9,
15062 => bb11,
_ => bb10
}
}
bb8 = {
RET = -(-1128729554_i32);
_2 = ['\u{180aa}','\u{dfa40}','\u{e987a}','\u{533ba}','\u{ea8ab}','\u{aefd0}','\u{648b9}','\u{fc409}'];
_1 = (-32_isize);
_5 = !3810592130_u32;
Goto(bb2)
}
bb9 = {
_19 = RET as f32;
_13 = [_5,_5,_5,_5,_5,_5];
_12.0 = '\u{109c50}';
(*_7) = '\u{84869}';
_6 = 0_usize;
_13 = [_5,_5,_5,_5,_5,_5];
_23.fld0 = _13;
_13[_6] = _23.fld0[_6];
_11 = _23.fld0;
RET = !1402694298_i32;
_11 = [_13[_6],_23.fld0[_6],_13[_6],_13[_6],_23.fld0[_6],_13[_6]];
_16 = core::ptr::addr_of_mut!(_3);
_22 = [_2[_6],_12.0,_2[_6],_2[_6],_2[_6],_2[_6],(*_7),_2[_6]];
_21 = 15062_i16;
_23.fld1 = [(*_16)[_6],_3[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6]];
_13[_6] = _11[_6] + _5;
_11 = [_5,_13[_6],_23.fld0[_6],_13[_6],_23.fld0[_6],_5];
_13[_6] = _11[_6] & _23.fld0[_6];
_22[_6] = (*_7);
_7 = core::ptr::addr_of_mut!((*_7));
_22 = [_2[_6],_2[_6],_2[_6],(*_7),_2[_6],_2[_6],_2[_6],_2[_6]];
_2 = [_12.0,_12.0,(*_7),_22[_6],_22[_6],_12.0,_22[_6],_22[_6]];
_2[_6] = _22[_6];
_23.fld1 = [(*_16)[_6],(*_16)[_6],(*_16)[_6],_3[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6],_3[_6]];
match (*_16)[_6] {
0 => bb2,
227101903398670603910859841073389125581 => bb7,
_ => bb6
}
}
bb10 = {
RET = (-2131400965_i32) << _4;
_3 = [153630769778887551365715658582170117756_i128,(-109276464745972069854560150802455974766_i128),60530550128103511707806739627586919651_i128,21616660333034672241118868877195014703_i128,(-73835705172066099824799774478688429034_i128),(-6301697795849892884460002966143594771_i128),(-45637320650477789708935172231588350463_i128)];
_8 = -_1;
_2 = ['\u{c664c}','\u{fe90a}','\u{17e3f}','\u{2797}','\u{39766}','\u{a6c56}','\u{33868}','\u{d3af5}'];
_10 = _4 & _4;
RET = 1852603573_i32 << _5;
_3 = [20734064838055536428415480283684399947_i128,(-75243564657319504692606754691722450512_i128),(-113363144135699021284727207315923313683_i128),65555403879307508704982448886446337105_i128,150400347727872560129225389586834373498_i128,97415862307130526622986584490980594610_i128,(-30915645367797680604489217302585278759_i128)];
_8 = _1;
_10 = !_4;
_1 = _5 as isize;
RET = (-184537492_i32);
RET = (-2136019165_i32);
RET = 188_u8 as i32;
_7 = core::ptr::addr_of_mut!(_12.0);
(*_7) = '\u{3ce4c}';
_3 = [(-113180463522267859552514766358379085875_i128),95308979111151128878107363345641259729_i128,(-133691380753631634560222202144184721586_i128),(-19415756898449612398496952705503837213_i128),118063553549425473023282433535340069035_i128,(-95860379903144433294701952708606292699_i128),(-2770211440684580753637524971748601588_i128)];
_12.0 = '\u{10f22b}';
(*_7) = '\u{f6c3a}';
RET = (-2088124582_i32);
_7 = core::ptr::addr_of_mut!(_12.0);
_8 = _1 << _10;
_4 = _10 ^ _10;
Goto(bb4)
}
bb11 = {
_30 = _23.fld1[_6] as f32;
_2[_6] = _12.0;
_33 = [_15[_6],_15[_6],_26[_6],_15[_6],_15[_6]];
_4 = !_10;
_18[_6] = _10;
(*_7) = _22[_6];
_13[_6] = _5;
_13 = [_5,_11[_6],_23.fld0[_6],_23.fld0[_6],_5,_23.fld0[_6]];
_25.fld1[_6] = _3[_6];
_29 = (*_16)[_6];
_32 = _13;
_15[_6] = Field::<bool>(Variant(_17, 2), 0) as u8;
_32 = [_5,_13[_6],_23.fld0[_6],_5,_5,_13[_6]];
_25.fld1 = [_29,_29,(*_16)[_6],_23.fld1[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6],_23.fld1[_6]];
_22[_6] = _12.0;
_25.fld0[_6] = _23.fld0[_6] ^ _13[_6];
(*_7) = _2[_6];
_24 = (*_16)[_6] as i32;
(*_7) = _2[_6];
_10 = _4;
_28.0 = !Field::<bool>(Variant(_17, 2), 0);
_25 = Adt54 { fld0: _13,fld1: _23.fld1 };
(*_7) = _22[_6];
_7 = core::ptr::addr_of_mut!(_2[_6]);
_22 = [_2[_6],_2[_6],_12.0,_12.0,(*_7),(*_7),_12.0,_12.0];
_11 = _13;
_30 = _19;
Goto(bb12)
}
bb12 = {
_1 = _19 as isize;
_31 = [_15[_6],_26[_6],_33[_6],_33[_6],_15[_6]];
_15 = [_26[_6],_31[_6],_26[_6],_33[_6],_31[_6]];
_30 = _19 + _19;
SetDiscriminant(_17, 3);
_23.fld1[_6] = (*_16)[_6] ^ _3[_6];
_13[_6] = _32[_6];
_35[_6] = _29 >> _25.fld1[_6];
_35 = [_29,(*_16)[_6],_3[_6],_29,(*_16)[_6],_3[_6],(*_16)[_6]];
place!(Field::<[i128; 8]>(Variant(_17, 3), 0))[_6] = _25.fld1[_6];
_4 = _10;
_25.fld1[_6] = !Field::<[i128; 8]>(Variant(_17, 3), 0)[_6];
_30 = _19 * _19;
_5 = _11[_6];
_7 = core::ptr::addr_of_mut!(_22[_6]);
_9 = !_8;
Goto(bb13)
}
bb13 = {
_39 = [_26[_6],_33[_6],_15[_6],_31[_6],_31[_6]];
_33 = _39;
_32 = _25.fld0;
_23.fld0 = [_32[_6],_5,_32[_6],_32[_6],_25.fld0[_6],_32[_6]];
_4 = 330705895669345627126781639476687075081_u128 as u16;
_41[_6] = (-7861106941985204580_i64) * (-8131289456637341619_i64);
_19 = _30;
match (*_16)[_6] {
0 => bb10,
1 => bb9,
2 => bb14,
227101903398670603910859841073389125581 => bb16,
_ => bb15
}
}
bb14 = {
RET = !(-1202613563_i32);
_15 = [217_u8,103_u8,146_u8,141_u8,229_u8];
_12.0 = '\u{fb90d}';
_17 = Adt46::Variant2 { fld0: true };
_6 = 6_usize - 7873356729324632918_usize;
_6 = !7_usize;
_13 = [_5,_5,_5,_5,_5,_5];
place!(Field::<bool>(Variant(_17, 2), 0)) = !false;
_7 = core::ptr::addr_of_mut!(_12.0);
RET = 449093489_i32;
RET = (-59649727_i32) & (-1925895808_i32);
_20 = _6 as isize;
_19 = 9050281103829233800401504899168139304_u128 as f32;
SetDiscriminant(_17, 2);
_6 = 13333250207130852217_u64 as usize;
_15 = [222_u8,140_u8,57_u8,162_u8,52_u8];
_18 = [_4,_4];
_13 = [_5,_5,_5,_5,_5,_5];
Goto(bb5)
}
bb15 = {
_19 = RET as f32;
_13 = [_5,_5,_5,_5,_5,_5];
_12.0 = '\u{109c50}';
(*_7) = '\u{84869}';
_6 = 0_usize;
_13 = [_5,_5,_5,_5,_5,_5];
_23.fld0 = _13;
_13[_6] = _23.fld0[_6];
_11 = _23.fld0;
RET = !1402694298_i32;
_11 = [_13[_6],_23.fld0[_6],_13[_6],_13[_6],_23.fld0[_6],_13[_6]];
_16 = core::ptr::addr_of_mut!(_3);
_22 = [_2[_6],_12.0,_2[_6],_2[_6],_2[_6],_2[_6],(*_7),_2[_6]];
_21 = 15062_i16;
_23.fld1 = [(*_16)[_6],_3[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6]];
_13[_6] = _11[_6] + _5;
_11 = [_5,_13[_6],_23.fld0[_6],_13[_6],_23.fld0[_6],_5];
_13[_6] = _11[_6] & _23.fld0[_6];
_22[_6] = (*_7);
_7 = core::ptr::addr_of_mut!((*_7));
_22 = [_2[_6],_2[_6],_2[_6],(*_7),_2[_6],_2[_6],_2[_6],_2[_6]];
_2 = [_12.0,_12.0,(*_7),_22[_6],_22[_6],_12.0,_22[_6],_22[_6]];
_2[_6] = _22[_6];
_23.fld1 = [(*_16)[_6],(*_16)[_6],(*_16)[_6],_3[_6],(*_16)[_6],(*_16)[_6],(*_16)[_6],_3[_6]];
match (*_16)[_6] {
0 => bb2,
227101903398670603910859841073389125581 => bb7,
_ => bb6
}
}
bb16 = {
_40[_6] = _31[_6] ^ _39[_6];
_32 = [_11[_6],_11[_6],_25.fld0[_6],_5,_13[_6],_13[_6]];
_26 = _15;
_36 = _30 as isize;
_44 = _18[_6] ^ _10;
_17 = Adt46::Variant3 { fld0: _23.fld1 };
_18[_6] = !_10;
_40[_6] = _41[_6] as u8;
(*_7) = _2[_6];
_46[_6] = _41[_6];
_23.fld1[_6] = -_35[_6];
_45 = !_8;
_15[_6] = _40[_6] - _40[_6];
_25.fld0 = [_5,_23.fld0[_6],_5,_23.fld0[_6],_23.fld0[_6],_5];
_46[_6] = !_41[_6];
_45 = _8;
_44 = _10;
_39 = [_40[_6],_40[_6],_15[_6],_40[_6],_33[_6]];
_35[_6] = -_23.fld1[_6];
_31[_6] = !_15[_6];
_38.fld1 = [_29,(*_16)[_6],_29,_3[_6],_23.fld1[_6],Field::<[i128; 8]>(Variant(_17, 3), 0)[_6],Field::<[i128; 8]>(Variant(_17, 3), 0)[_6],_3[_6]];
Goto(bb17)
}
bb17 = {
Call(_50 = dump_var(8_usize, 21_usize, Move(_21), 13_usize, Move(_13), 26_usize, Move(_26), 29_usize, Move(_29)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_50 = dump_var(8_usize, 36_usize, Move(_36), 6_usize, Move(_6), 44_usize, Move(_44), 33_usize, Move(_33)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_50 = dump_var(8_usize, 32_usize, Move(_32), 31_usize, Move(_31), 15_usize, Move(_15), 3_usize, Move(_3)), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Call(_50 = dump_var(8_usize, 10_usize, Move(_10), 8_usize, Move(_8), 51_usize, _51, 51_usize, _51), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn9(mut _1: [char; 8],mut _2: [i128; 7],mut _3: [char; 8],mut _4: [i128; 7],mut _5: i128,mut _6: [char; 8],mut _7: isize,mut _8: [char; 8],mut _9: [char; 8]) -> u32 {
mir! {
type RET = u32;
let _10: [u8; 2];
let _11: [i128; 7];
let _12: u8;
let _13: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8);
let _14: *mut [i128; 7];
let _15: [u8; 5];
let _16: [i128; 7];
let _17: isize;
let _18: Adt41;
let _19: bool;
let _20: [i128; 8];
let _21: char;
let _22: [i32; 3];
let _23: (char, &'static i8);
let _24: i64;
let _25: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8);
let _26: i128;
let _27: i8;
let _28: Adt56;
let _29: f64;
let _30: *const f64;
let _31: [char; 8];
let _32: i8;
let _33: [i128; 8];
let _34: *mut i8;
let _35: [i128; 8];
let _36: ();
let _37: ();
{
_10 = [14_u8,205_u8];
_5 = (-99656020733585225612012333114689464971_i128) ^ (-45287133513805959936956024102160408493_i128);
_9 = ['\u{58cae}','\u{732c2}','\u{876aa}','\u{1054b6}','\u{10ef93}','\u{7bc20}','\u{d9541}','\u{72de}'];
_3 = ['\u{9f261}','\u{22b8d}','\u{f24c5}','\u{99d31}','\u{60b6c}','\u{b1dee}','\u{9c6b}','\u{67aec}'];
RET = 3968658683_u32;
_10 = [177_u8,127_u8];
RET = !4256979218_u32;
_2 = [_5,_5,_5,_5,_5,_5,_5];
_4 = [_5,_5,_5,_5,_5,_5,_5];
_1 = ['\u{82fb6}','\u{a5b6b}','\u{cb52a}','\u{449bf}','\u{54231}','\u{db26c}','\u{4774f}','\u{9279e}'];
_5 = 26881659677717937820252269119605222126_i128 << _7;
_1 = ['\u{2993c}','\u{edc43}','\u{30512}','\u{94bf}','\u{2293e}','\u{5840d}','\u{9f5f2}','\u{77d5b}'];
_1 = ['\u{7622c}','\u{4057d}','\u{ea78d}','\u{45f7d}','\u{ed265}','\u{721c7}','\u{5fa92}','\u{bed82}'];
_10 = [90_u8,157_u8];
_1 = _9;
RET = 4265129075_u32;
_2 = _4;
RET = (-15_i8) as u32;
_1 = ['\u{ba5fb}','\u{10ea42}','\u{df987}','\u{7f49d}','\u{f4f18}','\u{e83f2}','\u{98a85}','\u{c1ec1}'];
_1 = _8;
_12 = _7 as u8;
Goto(bb1)
}
bb1 = {
_8 = _1;
_12 = 205_u8;
RET = !230187744_u32;
_11 = [_5,_5,_5,_5,_5,_5,_5];
_12 = _5 as u8;
_3 = _1;
_3 = ['\u{26016}','\u{6583e}','\u{83cf}','\u{1069d8}','\u{1a626}','\u{7667f}','\u{47a47}','\u{1ef}'];
_11 = [_5,_5,_5,_5,_5,_5,_5];
_9 = ['\u{12c36}','\u{10427d}','\u{39822}','\u{a1ef6}','\u{d0788}','\u{103bdb}','\u{bc7d8}','\u{b9e3e}'];
_4 = [_5,_5,_5,_5,_5,_5,_5];
_13.1.3 = '\u{107ff2}';
_13.2 = 28257_u16 as i16;
_13.1.0 = core::ptr::addr_of_mut!(_13.1.1);
_13.1.5 = core::ptr::addr_of_mut!(_5);
_3 = [_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3];
_13.1.1 = (-27_i8) + (-19_i8);
_13.0 = [_5,_5,_5,_5,_5,_5,_5];
_4 = [_5,_5,_5,_5,_5,_5,_5];
Call(_13 = fn10(_8, _9, _9, _6, _5, _6, _11, _11, _5, _12, _7, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_14 = core::ptr::addr_of_mut!(_2);
_15 = [_12,_13.4,_13.4,_12,_13.4];
_13.1.5 = core::ptr::addr_of_mut!(_5);
_13.1.5 = core::ptr::addr_of_mut!(_5);
_13.1.5 = core::ptr::addr_of_mut!(_5);
_6 = [_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3];
_11 = [_5,_5,_5,_5,_5,_5,_5];
_16 = _11;
_17 = _7 << _13.1.1;
Goto(bb3)
}
bb3 = {
_13.1.0 = core::ptr::addr_of_mut!(_13.1.1);
_13.1.0 = core::ptr::addr_of_mut!(_18.fld2.1.1);
_8 = _6;
_19 = true;
_18.fld1 = 12791194933698930723_u64 as i64;
_18.fld2.1.5 = core::ptr::addr_of_mut!(_5);
_2 = [_5,_5,_5,_5,_5,_5,_5];
_13.0 = [_5,_5,_5,_5,_5,_5,_5];
_18.fld2.2 = _13.2 + _13.2;
RET = 2519005662_u32;
_18.fld2.1 = _13.1;
_20 = [_5,_5,_5,_5,_5,_5,_5,_5];
_18.fld2 = _13;
match _18.fld2.1.1 {
0 => bb1,
1 => bb2,
2 => bb4,
3 => bb5,
340282366920938463463374607431768211455 => bb7,
_ => bb6
}
}
bb4 = {
_14 = core::ptr::addr_of_mut!(_2);
_15 = [_12,_13.4,_13.4,_12,_13.4];
_13.1.5 = core::ptr::addr_of_mut!(_5);
_13.1.5 = core::ptr::addr_of_mut!(_5);
_13.1.5 = core::ptr::addr_of_mut!(_5);
_6 = [_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3];
_11 = [_5,_5,_5,_5,_5,_5,_5];
_16 = _11;
_17 = _7 << _13.1.1;
Goto(bb3)
}
bb5 = {
_8 = _1;
_12 = 205_u8;
RET = !230187744_u32;
_11 = [_5,_5,_5,_5,_5,_5,_5];
_12 = _5 as u8;
_3 = _1;
_3 = ['\u{26016}','\u{6583e}','\u{83cf}','\u{1069d8}','\u{1a626}','\u{7667f}','\u{47a47}','\u{1ef}'];
_11 = [_5,_5,_5,_5,_5,_5,_5];
_9 = ['\u{12c36}','\u{10427d}','\u{39822}','\u{a1ef6}','\u{d0788}','\u{103bdb}','\u{bc7d8}','\u{b9e3e}'];
_4 = [_5,_5,_5,_5,_5,_5,_5];
_13.1.3 = '\u{107ff2}';
_13.2 = 28257_u16 as i16;
_13.1.0 = core::ptr::addr_of_mut!(_13.1.1);
_13.1.5 = core::ptr::addr_of_mut!(_5);
_3 = [_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3];
_13.1.1 = (-27_i8) + (-19_i8);
_13.0 = [_5,_5,_5,_5,_5,_5,_5];
_4 = [_5,_5,_5,_5,_5,_5,_5];
Call(_13 = fn10(_8, _9, _9, _6, _5, _6, _11, _11, _5, _12, _7, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
Return()
}
bb7 = {
_13.2 = -_18.fld2.2;
_18.fld0.0 = core::ptr::addr_of_mut!(_18.fld2.1.2);
_25.1.1 = _13.1.1 | _13.1.1;
_18.fld2.1.5 = _13.1.5;
_8 = _6;
_5 = -168524922278910544134935052001361372978_i128;
_18.fld2.1 = (_13.1.0, _25.1.1, _13.1.0, _13.1.3, _13.1.4, _13.1.5);
_21 = _18.fld2.1.3;
_25.1.4 = 258389371682460073181311262080239858908_u128 as f32;
_27 = _25.1.1;
_25 = _13;
_13.1.4 = 190965385693733054046887359230770575771_u128 as f32;
_1 = _6;
_18.fld0.0 = core::ptr::addr_of_mut!(_18.fld2.1.2);
_25.2 = -_13.2;
_23.1 = &_27;
_25.4 = !_12;
_25.0 = [_5,_5,_5,_5,_5,_5,_5];
_10 = [_18.fld2.4,_18.fld2.4];
Call(_18.fld2.0 = fn16(_25, _18.fld2.1.0, _18.fld2.1.3, _18.fld0, _18.fld0.0, _18.fld0.0, _13.1.0, _18.fld2.1.0, Move(_23.1), _13.1.0, _18.fld2.1.1, _18.fld2.1.2), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_18.fld2.1.3 = _21;
_18.fld2.1.0 = core::ptr::addr_of_mut!(_27);
_7 = _17 + _17;
_15 = [_25.4,_12,_25.4,_13.4,_13.4];
_25.1.2 = _18.fld2.1.0;
_25.1.0 = _13.1.0;
_24 = _18.fld1;
_22 = [(-1609154143_i32),2133748796_i32,201201263_i32];
_30 = core::ptr::addr_of!(_29);
_17 = !_7;
_7 = !_17;
_25.1.0 = _13.1.2;
RET = 1073879657_u32;
_13.1.5 = _18.fld2.1.5;
_29 = 337815875105251553798720603287345082463_u128 as f64;
RET = !1185515959_u32;
_13.1.3 = _21;
Goto(bb9)
}
bb9 = {
_22 = [1670082997_i32,(-788702644_i32),(-1125137431_i32)];
Goto(bb10)
}
bb10 = {
_13.1 = _18.fld2.1;
(*_14) = _4;
_25.1.2 = core::ptr::addr_of_mut!(_27);
_31 = [_25.1.3,_21,_25.1.3,_21,_25.1.3,_25.1.3,_18.fld2.1.3,_18.fld2.1.3];
_18.fld2.1.4 = 6_usize as f32;
Call(_25.4 = fn17(_27, _13.1, _18.fld2.1, _21, _18.fld0, _18.fld0, _18.fld0, _25.1, _18.fld2, _18.fld2.1.3), ReturnTo(bb11), UnwindUnreachable())
}
bb11 = {
_16 = [_5,_5,_5,_5,_5,_5,_5];
_19 = _25.4 != _25.4;
_25.3 = [23138_u16,34534_u16];
_25.3 = [15563_u16,43132_u16];
_23.0 = _18.fld2.1.3;
_26 = _5;
_25.1.3 = _21;
_18.fld2.0 = [_5,_5,_5,_26,_5,_5,_26];
_26 = !_5;
_30 = core::ptr::addr_of!(_29);
_18.fld2.1.3 = _13.1.3;
_30 = core::ptr::addr_of!((*_30));
_14 = core::ptr::addr_of_mut!(_2);
_25.1.5 = _13.1.5;
match _25.1.1 {
0 => bb12,
1 => bb13,
2 => bb14,
3 => bb15,
4 => bb16,
5 => bb17,
340282366920938463463374607431768211455 => bb19,
_ => bb18
}
}
bb12 = {
_13.1 = _18.fld2.1;
(*_14) = _4;
_25.1.2 = core::ptr::addr_of_mut!(_27);
_31 = [_25.1.3,_21,_25.1.3,_21,_25.1.3,_25.1.3,_18.fld2.1.3,_18.fld2.1.3];
_18.fld2.1.4 = 6_usize as f32;
Call(_25.4 = fn17(_27, _13.1, _18.fld2.1, _21, _18.fld0, _18.fld0, _18.fld0, _25.1, _18.fld2, _18.fld2.1.3), ReturnTo(bb11), UnwindUnreachable())
}
bb13 = {
_22 = [1670082997_i32,(-788702644_i32),(-1125137431_i32)];
Goto(bb10)
}
bb14 = {
_18.fld2.1.3 = _21;
_18.fld2.1.0 = core::ptr::addr_of_mut!(_27);
_7 = _17 + _17;
_15 = [_25.4,_12,_25.4,_13.4,_13.4];
_25.1.2 = _18.fld2.1.0;
_25.1.0 = _13.1.0;
_24 = _18.fld1;
_22 = [(-1609154143_i32),2133748796_i32,201201263_i32];
_30 = core::ptr::addr_of!(_29);
_17 = !_7;
_7 = !_17;
_25.1.0 = _13.1.2;
RET = 1073879657_u32;
_13.1.5 = _18.fld2.1.5;
_29 = 337815875105251553798720603287345082463_u128 as f64;
RET = !1185515959_u32;
_13.1.3 = _21;
Goto(bb9)
}
bb15 = {
_13.2 = -_18.fld2.2;
_18.fld0.0 = core::ptr::addr_of_mut!(_18.fld2.1.2);
_25.1.1 = _13.1.1 | _13.1.1;
_18.fld2.1.5 = _13.1.5;
_8 = _6;
_5 = -168524922278910544134935052001361372978_i128;
_18.fld2.1 = (_13.1.0, _25.1.1, _13.1.0, _13.1.3, _13.1.4, _13.1.5);
_21 = _18.fld2.1.3;
_25.1.4 = 258389371682460073181311262080239858908_u128 as f32;
_27 = _25.1.1;
_25 = _13;
_13.1.4 = 190965385693733054046887359230770575771_u128 as f32;
_1 = _6;
_18.fld0.0 = core::ptr::addr_of_mut!(_18.fld2.1.2);
_25.2 = -_13.2;
_23.1 = &_27;
_25.4 = !_12;
_25.0 = [_5,_5,_5,_5,_5,_5,_5];
_10 = [_18.fld2.4,_18.fld2.4];
Call(_18.fld2.0 = fn16(_25, _18.fld2.1.0, _18.fld2.1.3, _18.fld0, _18.fld0.0, _18.fld0.0, _13.1.0, _18.fld2.1.0, Move(_23.1), _13.1.0, _18.fld2.1.1, _18.fld2.1.2), ReturnTo(bb8), UnwindUnreachable())
}
bb16 = {
_8 = _1;
_12 = 205_u8;
RET = !230187744_u32;
_11 = [_5,_5,_5,_5,_5,_5,_5];
_12 = _5 as u8;
_3 = _1;
_3 = ['\u{26016}','\u{6583e}','\u{83cf}','\u{1069d8}','\u{1a626}','\u{7667f}','\u{47a47}','\u{1ef}'];
_11 = [_5,_5,_5,_5,_5,_5,_5];
_9 = ['\u{12c36}','\u{10427d}','\u{39822}','\u{a1ef6}','\u{d0788}','\u{103bdb}','\u{bc7d8}','\u{b9e3e}'];
_4 = [_5,_5,_5,_5,_5,_5,_5];
_13.1.3 = '\u{107ff2}';
_13.2 = 28257_u16 as i16;
_13.1.0 = core::ptr::addr_of_mut!(_13.1.1);
_13.1.5 = core::ptr::addr_of_mut!(_5);
_3 = [_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3];
_13.1.1 = (-27_i8) + (-19_i8);
_13.0 = [_5,_5,_5,_5,_5,_5,_5];
_4 = [_5,_5,_5,_5,_5,_5,_5];
Call(_13 = fn10(_8, _9, _9, _6, _5, _6, _11, _11, _5, _12, _7, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb17 = {
_8 = _1;
_12 = 205_u8;
RET = !230187744_u32;
_11 = [_5,_5,_5,_5,_5,_5,_5];
_12 = _5 as u8;
_3 = _1;
_3 = ['\u{26016}','\u{6583e}','\u{83cf}','\u{1069d8}','\u{1a626}','\u{7667f}','\u{47a47}','\u{1ef}'];
_11 = [_5,_5,_5,_5,_5,_5,_5];
_9 = ['\u{12c36}','\u{10427d}','\u{39822}','\u{a1ef6}','\u{d0788}','\u{103bdb}','\u{bc7d8}','\u{b9e3e}'];
_4 = [_5,_5,_5,_5,_5,_5,_5];
_13.1.3 = '\u{107ff2}';
_13.2 = 28257_u16 as i16;
_13.1.0 = core::ptr::addr_of_mut!(_13.1.1);
_13.1.5 = core::ptr::addr_of_mut!(_5);
_3 = [_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3];
_13.1.1 = (-27_i8) + (-19_i8);
_13.0 = [_5,_5,_5,_5,_5,_5,_5];
_4 = [_5,_5,_5,_5,_5,_5,_5];
Call(_13 = fn10(_8, _9, _9, _6, _5, _6, _11, _11, _5, _12, _7, _6), ReturnTo(bb2), UnwindUnreachable())
}
bb18 = {
_14 = core::ptr::addr_of_mut!(_2);
_15 = [_12,_13.4,_13.4,_12,_13.4];
_13.1.5 = core::ptr::addr_of_mut!(_5);
_13.1.5 = core::ptr::addr_of_mut!(_5);
_13.1.5 = core::ptr::addr_of_mut!(_5);
_6 = [_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3,_13.1.3];
_11 = [_5,_5,_5,_5,_5,_5,_5];
_16 = _11;
_17 = _7 << _13.1.1;
Goto(bb3)
}
bb19 = {
_2 = [_26,_5,_26,_5,_5,_5,_5];
_13.2 = _29 as i16;
_13.1.5 = core::ptr::addr_of_mut!(_26);
_13.1.4 = _5 as f32;
_30 = core::ptr::addr_of!((*_30));
_18.fld2.1.3 = _25.1.3;
_25.1.5 = core::ptr::addr_of_mut!(_26);
_10 = [_25.4,_25.4];
Goto(bb20)
}
bb20 = {
Call(_36 = dump_var(9_usize, 20_usize, Move(_20), 3_usize, Move(_3), 26_usize, Move(_26), 31_usize, Move(_31)), ReturnTo(bb21), UnwindUnreachable())
}
bb21 = {
Call(_36 = dump_var(9_usize, 24_usize, Move(_24), 21_usize, Move(_21), 9_usize, Move(_9), 17_usize, Move(_17)), ReturnTo(bb22), UnwindUnreachable())
}
bb22 = {
Call(_36 = dump_var(9_usize, 5_usize, Move(_5), 27_usize, Move(_27), 16_usize, Move(_16), 37_usize, _37), ReturnTo(bb23), UnwindUnreachable())
}
bb23 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn10(mut _1: [char; 8],mut _2: [char; 8],mut _3: [char; 8],mut _4: [char; 8],mut _5: i128,mut _6: [char; 8],mut _7: [i128; 7],mut _8: [i128; 7],mut _9: i128,mut _10: u8,mut _11: isize,mut _12: [char; 8]) -> ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8) {
mir! {
type RET = ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8);
let _13: Adt50;
let _14: isize;
let _15: Adt47;
let _16: i16;
let _17: bool;
let _18: f32;
let _19: (*mut i8, i8, *mut i8, char, f32, *mut i128);
let _20: ();
let _21: ();
{
RET.3 = [41130_u16,20854_u16];
RET.0 = [_5,_5,_5,_9,_5,_9,_5];
RET.3 = [18315_u16,3114_u16];
_13.fld1.fld3.1.0 = core::ptr::addr_of_mut!(_13.fld1.fld3.1.1);
_13.fld1.fld3.1.5 = core::ptr::addr_of_mut!(_13.fld1.fld0.4);
RET.1.0 = core::ptr::addr_of_mut!(_13.fld1.fld3.1.1);
_13.fld1.fld3.0 = [_9,_9,_9,_5,_5,_5,_5];
_13.fld1.fld0.3 = 1633217460_i32 * (-155988606_i32);
RET.1.1 = -119_i8;
RET.4 = !_10;
RET.1.5 = core::ptr::addr_of_mut!(_9);
_13.fld1.fld3.1.4 = _13.fld1.fld0.3 as f32;
Call(_13.fld1.fld3.1.0 = fn11(RET.1.5, _5, _1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_13.fld1.fld0.2 = core::ptr::addr_of_mut!(_8);
_13.fld1.fld3.1.2 = _13.fld1.fld3.1.0;
RET.1.2 = _13.fld1.fld3.1.2;
_13.fld1.fld0.3 = -502506048_i32;
RET.1.0 = _13.fld1.fld3.1.2;
_5 = _9 - _9;
_14 = _11;
_13.fld1.fld3.1.5 = core::ptr::addr_of_mut!(_13.fld1.fld0.4);
Call(RET.1.1 = fn15(_2, _2, _8, RET.1.5, _5, RET.1.5, _13.fld1.fld3.1.0, _1, _13.fld1.fld3.1.0, RET.1.0, _13.fld1.fld3.1.2, _3, RET.1.5, RET.1.5, _13.fld1.fld0.2), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_13.fld1.fld3.1.5 = RET.1.5;
_13.fld2.0 = core::ptr::addr_of_mut!(RET.1.2);
RET.1.0 = core::ptr::addr_of_mut!(_13.fld1.fld3.1.1);
_13.fld1.fld0.2 = core::ptr::addr_of_mut!(_13.fld1.fld3.0);
RET.1 = (_13.fld1.fld3.1.2, (-95_i8), _13.fld1.fld3.1.2, '\u{b8f3b}', _13.fld1.fld3.1.4, _13.fld1.fld3.1.5);
RET.1 = (_13.fld1.fld3.1.2, (-1_i8), _13.fld1.fld3.1.0, '\u{1065dd}', _13.fld1.fld3.1.4, _13.fld1.fld3.1.5);
RET.2 = !(-26960_i16);
_14 = 193425842499659084783454593591781413586_u128 as isize;
RET.1.4 = -_13.fld1.fld3.1.4;
RET.1.5 = _13.fld1.fld3.1.5;
Goto(bb3)
}
bb3 = {
Call(_20 = dump_var(10_usize, 10_usize, Move(_10), 8_usize, Move(_8), 1_usize, Move(_1), 9_usize, Move(_9)), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Call(_20 = dump_var(10_usize, 14_usize, Move(_14), 5_usize, Move(_5), 21_usize, _21, 21_usize, _21), ReturnTo(bb5), UnwindUnreachable())
}
bb5 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn11(mut _1: *mut i128,mut _2: i128,mut _3: [char; 8]) -> *mut i8 {
mir! {
type RET = *mut i8;
let _4: [i32; 3];
let _5: i128;
let _6: [u32; 6];
let _7: [i128; 8];
let _8: Adt41;
let _9: f32;
let _10: bool;
let _11: i64;
let _12: f64;
let _13: isize;
let _14: [u8; 2];
let _15: i32;
let _16: [i32; 3];
let _17: [i64; 1];
let _18: [u32; 6];
let _19: char;
let _20: isize;
let _21: f64;
let _22: u8;
let _23: [i32; 3];
let _24: u8;
let _25: (bool, &'static i8);
let _26: [u16; 2];
let _27: *mut i128;
let _28: [u8; 5];
let _29: [char; 8];
let _30: *mut i128;
let _31: *mut [i128; 7];
let _32: u8;
let _33: Adt56;
let _34: [char; 8];
let _35: ();
let _36: ();
{
_3 = ['\u{6f44c}','\u{19754}','\u{dfa71}','\u{473a5}','\u{18cd7}','\u{ed32f}','\u{f211b}','\u{2ffd3}'];
(*_1) = _2 + _2;
_3 = ['\u{f014c}','\u{84ab0}','\u{6d9ac}','\u{4e48b}','\u{f72f}','\u{b2f70}','\u{7007}','\u{103bd5}'];
(*_1) = -_2;
_4 = [204060366_i32,(-1284846287_i32),1880737984_i32];
(*_1) = _2;
_4 = [(-1608770419_i32),(-1938137145_i32),2112368653_i32];
_3 = ['\u{dfe37}','\u{8f18c}','\u{200bf}','\u{b1664}','\u{bc823}','\u{ea075}','\u{5f850}','\u{47bb3}'];
_4 = [1140147510_i32,(-33082112_i32),1578119099_i32];
Goto(bb1)
}
bb1 = {
_5 = (*_1) ^ (*_1);
(*_1) = 1309737123_u32 as i128;
_2 = false as i128;
(*_1) = 4051467650_u32 as i128;
_5 = _2;
(*_1) = 2657330737311408395_u64 as i128;
_2 = (*_1) + (*_1);
_4 = [1434636459_i32,(-109825111_i32),492665267_i32];
_6 = [348712779_u32,1353962566_u32,767135888_u32,3224698984_u32,3173920042_u32,3072715528_u32];
_4 = [(-2125274953_i32),2030479217_i32,176745691_i32];
_6 = [1796434970_u32,911583519_u32,1305138728_u32,2097534208_u32,1280731247_u32,3278103552_u32];
_7 = [(*_1),_2,(*_1),_5,(*_1),_2,_2,(*_1)];
_2 = !(*_1);
_3 = ['\u{d2720}','\u{b40f6}','\u{b64f1}','\u{bfadd}','\u{d49d4}','\u{bece5}','\u{d9b85}','\u{10d575}'];
(*_1) = _2;
_3 = ['\u{11c48}','\u{70fe4}','\u{645a2}','\u{3a939}','\u{85a36}','\u{a889f}','\u{10434}','\u{7e5cf}'];
Goto(bb2)
}
bb2 = {
(*_1) = -_5;
_2 = -(*_1);
(*_1) = !_2;
_8.fld2.3 = [58767_u16,30498_u16];
_7 = [(*_1),_5,_2,_2,_5,(*_1),_5,_5];
_8.fld2.1.2 = core::ptr::addr_of_mut!(_8.fld2.1.1);
_8.fld2.1.5 = _1;
_8.fld2.1.0 = core::ptr::addr_of_mut!(_8.fld2.1.1);
RET = core::ptr::addr_of_mut!(_8.fld2.1.1);
_7 = [(*_1),(*_1),(*_1),_2,(*_1),(*_1),_5,_5];
_8.fld2.1.4 = (-83_i8) as f32;
_8.fld0.0 = core::ptr::addr_of_mut!(RET);
_10 = false;
(*RET) = 55_i8;
_8.fld2.3 = [6641_u16,57360_u16];
_8.fld2.4 = 254_u8 ^ 162_u8;
(*_1) = _2;
_3 = ['\u{acdad}','\u{10f4d1}','\u{5c550}','\u{5a67b}','\u{103403}','\u{bc558}','\u{1daef}','\u{c1687}'];
_4 = [551445949_i32,206846890_i32,(-1145253563_i32)];
_11 = (-7773998117519269109_i64);
_1 = core::ptr::addr_of_mut!(_5);
_11 = _8.fld2.4 as i64;
_8.fld2.0 = [(*_1),(*_1),(*_1),_2,(*_1),(*_1),_5];
match (*RET) {
0 => bb3,
1 => bb4,
55 => bb6,
_ => bb5
}
}
bb3 = {
_5 = (*_1) ^ (*_1);
(*_1) = 1309737123_u32 as i128;
_2 = false as i128;
(*_1) = 4051467650_u32 as i128;
_5 = _2;
(*_1) = 2657330737311408395_u64 as i128;
_2 = (*_1) + (*_1);
_4 = [1434636459_i32,(-109825111_i32),492665267_i32];
_6 = [348712779_u32,1353962566_u32,767135888_u32,3224698984_u32,3173920042_u32,3072715528_u32];
_4 = [(-2125274953_i32),2030479217_i32,176745691_i32];
_6 = [1796434970_u32,911583519_u32,1305138728_u32,2097534208_u32,1280731247_u32,3278103552_u32];
_7 = [(*_1),_2,(*_1),_5,(*_1),_2,_2,(*_1)];
_2 = !(*_1);
_3 = ['\u{d2720}','\u{b40f6}','\u{b64f1}','\u{bfadd}','\u{d49d4}','\u{bece5}','\u{d9b85}','\u{10d575}'];
(*_1) = _2;
_3 = ['\u{11c48}','\u{70fe4}','\u{645a2}','\u{3a939}','\u{85a36}','\u{a889f}','\u{10434}','\u{7e5cf}'];
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
_8.fld1 = _11;
_5 = _2;
_8.fld2.4 = 4683_u16 as u8;
_8.fld2.1.0 = core::ptr::addr_of_mut!((*RET));
_8.fld2.2 = (-31598_i16) & 3206_i16;
(*_1) = 6_usize as i128;
_8.fld2.1.4 = 67280401521803716480240225626107322428_u128 as f32;
_8.fld2.3 = [17225_u16,56067_u16];
_8.fld2.1.2 = RET;
_8.fld2.1.2 = core::ptr::addr_of_mut!((*RET));
_4 = [1863925611_i32,(-1000612066_i32),51079849_i32];
_8.fld2.1.0 = _8.fld2.1.2;
RET = _8.fld2.1.2;
_8.fld2.1.4 = (*RET) as f32;
(*RET) = _10 as i8;
_11 = _8.fld1 >> _8.fld2.1.1;
_15 = 13375607725663719566_u64 as i32;
_8.fld2.1.3 = '\u{46d17}';
_8.fld2.1.0 = _8.fld2.1.2;
_14 = [_8.fld2.4,_8.fld2.4];
_10 = !false;
_12 = _8.fld2.4 as f64;
Goto(bb7)
}
bb7 = {
_17 = [_11];
(*RET) = _15 as i8;
_8.fld1 = !_11;
_12 = _11 as f64;
Call(_5 = fn12(_2, RET, _8, _3, _4, _8.fld2, _6, _8.fld2.1.1, _4, _15, _6), ReturnTo(bb8), UnwindUnreachable())
}
bb8 = {
_19 = _8.fld2.1.3;
_4 = [_15,_15,_15];
_3 = [_8.fld2.1.3,_19,_8.fld2.1.3,_19,_8.fld2.1.3,_8.fld2.1.3,_19,_8.fld2.1.3];
_12 = (*RET) as f64;
_22 = _8.fld2.2 as u8;
_9 = _8.fld2.1.4 - _8.fld2.1.4;
(*RET) = 93_i8;
_14 = [_22,_22];
(*_1) = _2 - _2;
_4 = [_15,_15,_15];
_17 = [_8.fld1];
_2 = (*_1) << _8.fld1;
_12 = _5 as f64;
_4 = [_15,_15,_15];
_18 = [646980810_u32,1041355731_u32,3730094178_u32,2887322347_u32,1551368019_u32,1284230692_u32];
_1 = _8.fld2.1.5;
_8.fld0.0 = core::ptr::addr_of_mut!(_8.fld2.1.0);
match _8.fld2.1.1 {
0 => bb5,
1 => bb2,
2 => bb6,
3 => bb9,
93 => bb11,
_ => bb10
}
}
bb9 = {
(*_1) = -_5;
_2 = -(*_1);
(*_1) = !_2;
_8.fld2.3 = [58767_u16,30498_u16];
_7 = [(*_1),_5,_2,_2,_5,(*_1),_5,_5];
_8.fld2.1.2 = core::ptr::addr_of_mut!(_8.fld2.1.1);
_8.fld2.1.5 = _1;
_8.fld2.1.0 = core::ptr::addr_of_mut!(_8.fld2.1.1);
RET = core::ptr::addr_of_mut!(_8.fld2.1.1);
_7 = [(*_1),(*_1),(*_1),_2,(*_1),(*_1),_5,_5];
_8.fld2.1.4 = (-83_i8) as f32;
_8.fld0.0 = core::ptr::addr_of_mut!(RET);
_10 = false;
(*RET) = 55_i8;
_8.fld2.3 = [6641_u16,57360_u16];
_8.fld2.4 = 254_u8 ^ 162_u8;
(*_1) = _2;
_3 = ['\u{acdad}','\u{10f4d1}','\u{5c550}','\u{5a67b}','\u{103403}','\u{bc558}','\u{1daef}','\u{c1687}'];
_4 = [551445949_i32,206846890_i32,(-1145253563_i32)];
_11 = (-7773998117519269109_i64);
_1 = core::ptr::addr_of_mut!(_5);
_11 = _8.fld2.4 as i64;
_8.fld2.0 = [(*_1),(*_1),(*_1),_2,(*_1),(*_1),_5];
match (*RET) {
0 => bb3,
1 => bb4,
55 => bb6,
_ => bb5
}
}
bb10 = {
Return()
}
bb11 = {
_19 = _8.fld2.1.3;
_12 = 15310229970944144012_u64 as f64;
_8.fld0.0 = core::ptr::addr_of_mut!(_8.fld2.1.2);
_8.fld1 = _11;
_25.1 = &(*RET);
_22 = _8.fld2.4;
_8.fld2.0 = [_2,_5,_5,_5,_5,_5,_5];
_24 = _8.fld2.4;
_22 = _12 as u8;
_28 = [_8.fld2.4,_22,_22,_22,_8.fld2.4];
_10 = true;
(*RET) = (-76_i8);
_23 = [_15,_15,_15];
_8.fld0.0 = core::ptr::addr_of_mut!(_8.fld2.1.2);
_22 = _8.fld2.4 | _8.fld2.4;
_5 = !_2;
_14 = [_22,_8.fld2.4];
_5 = _2 ^ (*_1);
_8.fld2.2 = !(-22002_i16);
_8.fld2.1.2 = core::ptr::addr_of_mut!(_8.fld2.1.1);
_17 = [_8.fld1];
_8.fld2.2 = !19079_i16;
_20 = _12 as isize;
_25.1 = &(*RET);
_1 = _8.fld2.1.5;
_29 = _3;
Goto(bb12)
}
bb12 = {
_11 = _8.fld1 * _8.fld1;
_27 = _1;
_6 = _18;
(*_27) = _15 as i128;
_8.fld2.1.3 = _19;
(*_27) = _20 as i128;
_29 = [_19,_8.fld2.1.3,_8.fld2.1.3,_8.fld2.1.3,_19,_8.fld2.1.3,_19,_19];
_23 = _4;
_23 = [_15,_15,_15];
_16 = [_15,_15,_15];
_11 = _8.fld1 << _5;
_19 = _8.fld2.1.3;
(*RET) = (-73_i8) + (-28_i8);
RET = _8.fld2.1.2;
_8.fld2.4 = _22 << _2;
_8.fld2.1.3 = _19;
_8.fld2.1.3 = _19;
(*_27) = -_5;
_8.fld2.1.2 = core::ptr::addr_of_mut!((*RET));
_8.fld2.1.5 = core::ptr::addr_of_mut!((*_1));
RET = core::ptr::addr_of_mut!((*RET));
_7 = [_5,(*_27),_2,(*_27),(*_27),_5,(*_1),(*_27)];
_31 = core::ptr::addr_of_mut!(_8.fld2.0);
_8.fld2.1.4 = 194600662532517140300273391562924503613_u128 as f32;
_21 = _12;
_29 = [_8.fld2.1.3,_19,_8.fld2.1.3,_19,_8.fld2.1.3,_19,_8.fld2.1.3,_8.fld2.1.3];
_28 = [_24,_8.fld2.4,_8.fld2.4,_8.fld2.4,_8.fld2.4];
Goto(bb13)
}
bb13 = {
_4 = _23;
_12 = _21;
(*_1) = _5 >> _11;
_30 = core::ptr::addr_of_mut!(_2);
_8.fld0.0 = core::ptr::addr_of_mut!(_8.fld2.1.2);
_25.1 = &(*RET);
(*_1) = _2 - _5;
_3 = [_19,_19,_19,_8.fld2.1.3,_8.fld2.1.3,_19,_8.fld2.1.3,_8.fld2.1.3];
(*_30) = (*_27) | (*_1);
_8.fld1 = (*_27) as i64;
Goto(bb14)
}
bb14 = {
_17 = [_8.fld1];
_8.fld2.1.2 = core::ptr::addr_of_mut!((*RET));
_5 = _12 as i128;
(*RET) = 68_i8;
(*_31) = [(*_30),(*_30),_2,(*_30),_5,(*_30),(*_27)];
_8.fld2.1 = (RET, (-83_i8), RET, _19, _9, _1);
Goto(bb15)
}
bb15 = {
Call(_35 = dump_var(11_usize, 10_usize, Move(_10), 14_usize, Move(_14), 3_usize, Move(_3), 22_usize, Move(_22)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_35 = dump_var(11_usize, 11_usize, Move(_11), 18_usize, Move(_18), 29_usize, Move(_29), 4_usize, Move(_4)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_35 = dump_var(11_usize, 5_usize, Move(_5), 17_usize, Move(_17), 36_usize, _36, 36_usize, _36), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn12(mut _1: i128,mut _2: *mut i8,mut _3: Adt41,mut _4: [char; 8],mut _5: [i32; 3],mut _6: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8),mut _7: [u32; 6],mut _8: i8,mut _9: [i32; 3],mut _10: i32,mut _11: [u32; 6]) -> i128 {
mir! {
type RET = i128;
let _12: [i64; 1];
let _13: (bool, &'static i8);
let _14: *mut i128;
let _15: [i32; 3];
let _16: (char, &'static i8);
let _17: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8);
let _18: Adt46;
let _19: u8;
let _20: isize;
let _21: u64;
let _22: [char; 8];
let _23: [i128; 8];
let _24: usize;
let _25: u32;
let _26: u64;
let _27: [i128; 8];
let _28: ();
let _29: ();
{
(*_2) = -_8;
_3.fld2 = (_6.0, _6.1, _6.2, _6.3, _6.4);
_12 = [_3.fld1];
_3.fld2.2 = _6.2;
(*_2) = _3.fld2.1.1 | _6.1.1;
RET = _3.fld2.4 as i128;
_3.fld2.1.2 = _6.1.2;
_13.0 = false & false;
_3.fld2.2 = _6.2 + _6.2;
_8 = _6.1.1;
Call(_3.fld2.1 = fn13(_3.fld2.2, _8, _6, _7, _6.1.3, _2, _3.fld0.0, _6.2, _7, _4, _3.fld0.0, _5), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_6.1.5 = _3.fld2.1.5;
_13.0 = true;
_3.fld2 = (_6.0, _6.1, _6.2, _6.3, _6.4);
RET = _1 & _1;
_9 = _5;
_11 = [2598672444_u32,2473142542_u32,2797721596_u32,3061675379_u32,54718568_u32,2350325325_u32];
_6.1.1 = (*_2) & (*_2);
_2 = _6.1.0;
_6.4 = _3.fld2.4 + _3.fld2.4;
_6.1.4 = _3.fld2.1.4 - _3.fld2.1.4;
_8 = _6.1.1 * _3.fld2.1.1;
_13.1 = &(*_2);
_3.fld2.1 = (_2, _8, _2, _6.1.3, _6.1.4, _6.1.5);
_14 = _6.1.5;
_6.2 = _3.fld2.2;
Call(_6.4 = fn14(Move(_13.1), _6.1.1, _2, _2, _6.1.2, _3.fld2.1, _6.1.2, _6.1.1, _3.fld2, _3.fld2.1, _6.1.0, _3.fld2, _3.fld0.0), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_3.fld2.1.1 = _10 as i8;
Goto(bb3)
}
bb3 = {
(*_2) = -_8;
RET = _10 as i128;
_6.1.4 = _3.fld2.1.4;
RET = _1 & _1;
_16.0 = _6.1.3;
_17.1.2 = _3.fld2.1.2;
_17 = (_6.0, _6.1, _6.2, _6.3, _3.fld2.4);
_16.1 = &(*_2);
_3.fld2.4 = _17.4 & _17.4;
_3.fld1 = (-8499500668542627463_i64);
_17.1.3 = _6.1.3;
_17.1.2 = core::ptr::addr_of_mut!((*_2));
_3.fld2.1.4 = -_17.1.4;
(*_2) = -_8;
_11 = [1642734481_u32,281421866_u32,2587988545_u32,1874316811_u32,4288984685_u32,4110259494_u32];
_3.fld2.3 = _17.3;
_17.1.4 = 1789981341_u32 as f32;
_3.fld2.4 = _17.4;
_17.4 = 3538877951_u32 as u8;
Goto(bb4)
}
bb4 = {
_3.fld2.1.2 = _6.1.2;
_5 = _9;
_13.0 = _6.1.1 <= (*_2);
_17.1 = (_3.fld2.1.0, _8, _3.fld2.1.2, _3.fld2.1.3, _6.1.4, _3.fld2.1.5);
_4 = [_16.0,_3.fld2.1.3,_6.1.3,_16.0,_6.1.3,_17.1.3,_6.1.3,_3.fld2.1.3];
RET = _1 & _1;
_6.1.2 = core::ptr::addr_of_mut!(_8);
_17.1.5 = _6.1.5;
RET = _1 >> (*_2);
_15 = [_10,_10,_10];
_6.4 = !_17.4;
_14 = core::ptr::addr_of_mut!(_1);
(*_2) = _3.fld2.1.3 as i8;
_4 = [_16.0,_16.0,_16.0,_6.1.3,_16.0,_6.1.3,_3.fld2.1.3,_3.fld2.1.3];
_3.fld2.1.0 = _6.1.2;
_17.2 = _3.fld2.2 >> _17.1.1;
_6.1.5 = _17.1.5;
_3.fld2.1.5 = core::ptr::addr_of_mut!(_1);
_3.fld2.2 = _17.2;
RET = _1;
_16.0 = _6.1.3;
_3.fld2.1.5 = core::ptr::addr_of_mut!((*_14));
match _3.fld1 {
0 => bb5,
340282366920938463454875106763225583993 => bb7,
_ => bb6
}
}
bb5 = {
_6.1.5 = _3.fld2.1.5;
_13.0 = true;
_3.fld2 = (_6.0, _6.1, _6.2, _6.3, _6.4);
RET = _1 & _1;
_9 = _5;
_11 = [2598672444_u32,2473142542_u32,2797721596_u32,3061675379_u32,54718568_u32,2350325325_u32];
_6.1.1 = (*_2) & (*_2);
_2 = _6.1.0;
_6.4 = _3.fld2.4 + _3.fld2.4;
_6.1.4 = _3.fld2.1.4 - _3.fld2.1.4;
_8 = _6.1.1 * _3.fld2.1.1;
_13.1 = &(*_2);
_3.fld2.1 = (_2, _8, _2, _6.1.3, _6.1.4, _6.1.5);
_14 = _6.1.5;
_6.2 = _3.fld2.2;
Call(_6.4 = fn14(Move(_13.1), _6.1.1, _2, _2, _6.1.2, _3.fld2.1, _6.1.2, _6.1.1, _3.fld2, _3.fld2.1, _6.1.0, _3.fld2, _3.fld0.0), ReturnTo(bb2), UnwindUnreachable())
}
bb6 = {
_3.fld2.1.1 = _10 as i8;
Goto(bb3)
}
bb7 = {
_3.fld2.1.4 = _6.1.4;
(*_2) = _6.1.1;
_6.1 = _3.fld2.1;
_3.fld2.1.2 = _6.1.0;
_3.fld2.2 = _17.2 * _17.2;
_4 = [_6.1.3,_6.1.3,_3.fld2.1.3,_6.1.3,_17.1.3,_16.0,_3.fld2.1.3,_17.1.3];
_3.fld2.1.2 = core::ptr::addr_of_mut!(_6.1.1);
_19 = _3.fld2.4 * _6.4;
_8 = (*_2) << _17.1.1;
_18 = Adt46::Variant2 { fld0: _13.0 };
_16.1 = &(*_2);
_13.1 = Move(_16.1);
_17.4 = _6.4;
_14 = core::ptr::addr_of_mut!((*_14));
_19 = _6.4 + _3.fld2.4;
_6.1 = (_3.fld2.1.0, (*_2), _3.fld2.1.0, _3.fld2.1.3, _3.fld2.1.4, _3.fld2.1.5);
place!(Field::<bool>(Variant(_18, 2), 0)) = !_13.0;
_16 = (_3.fld2.1.3, Move(_13.1));
_17.2 = _3.fld2.2 + _3.fld2.2;
RET = (*_14);
_20 = (-9223372036854775808_isize);
_6.1.0 = _17.1.0;
_7 = [3427529997_u32,3303195198_u32,3354996353_u32,2131235056_u32,2199135929_u32,1943025514_u32];
_6.2 = !_17.2;
_17.4 = !_3.fld2.4;
_17.2 = _6.2;
_3.fld0.0 = core::ptr::addr_of_mut!(_17.1.2);
_3.fld2.1.4 = _17.1.4 + _6.1.4;
match _3.fld1 {
0 => bb6,
1 => bb2,
2 => bb8,
3 => bb9,
4 => bb10,
340282366920938463454875106763225583993 => bb12,
_ => bb11
}
}
bb8 = {
_3.fld2.1.1 = _10 as i8;
Goto(bb3)
}
bb9 = {
_6.1.5 = _3.fld2.1.5;
_13.0 = true;
_3.fld2 = (_6.0, _6.1, _6.2, _6.3, _6.4);
RET = _1 & _1;
_9 = _5;
_11 = [2598672444_u32,2473142542_u32,2797721596_u32,3061675379_u32,54718568_u32,2350325325_u32];
_6.1.1 = (*_2) & (*_2);
_2 = _6.1.0;
_6.4 = _3.fld2.4 + _3.fld2.4;
_6.1.4 = _3.fld2.1.4 - _3.fld2.1.4;
_8 = _6.1.1 * _3.fld2.1.1;
_13.1 = &(*_2);
_3.fld2.1 = (_2, _8, _2, _6.1.3, _6.1.4, _6.1.5);
_14 = _6.1.5;
_6.2 = _3.fld2.2;
Call(_6.4 = fn14(Move(_13.1), _6.1.1, _2, _2, _6.1.2, _3.fld2.1, _6.1.2, _6.1.1, _3.fld2, _3.fld2.1, _6.1.0, _3.fld2, _3.fld0.0), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
_6.1.5 = _3.fld2.1.5;
_13.0 = true;
_3.fld2 = (_6.0, _6.1, _6.2, _6.3, _6.4);
RET = _1 & _1;
_9 = _5;
_11 = [2598672444_u32,2473142542_u32,2797721596_u32,3061675379_u32,54718568_u32,2350325325_u32];
_6.1.1 = (*_2) & (*_2);
_2 = _6.1.0;
_6.4 = _3.fld2.4 + _3.fld2.4;
_6.1.4 = _3.fld2.1.4 - _3.fld2.1.4;
_8 = _6.1.1 * _3.fld2.1.1;
_13.1 = &(*_2);
_3.fld2.1 = (_2, _8, _2, _6.1.3, _6.1.4, _6.1.5);
_14 = _6.1.5;
_6.2 = _3.fld2.2;
Call(_6.4 = fn14(Move(_13.1), _6.1.1, _2, _2, _6.1.2, _3.fld2.1, _6.1.2, _6.1.1, _3.fld2, _3.fld2.1, _6.1.0, _3.fld2, _3.fld0.0), ReturnTo(bb2), UnwindUnreachable())
}
bb11 = {
_3.fld2.1.1 = _10 as i8;
Goto(bb3)
}
bb12 = {
_17.1.0 = _3.fld2.1.0;
_17.1.4 = 63930_u16 as f32;
_3.fld2.4 = _13.0 as u8;
_13.1 = Move(_16.1);
_6.0 = [RET,RET,(*_14),(*_14),_1,_1,(*_14)];
_21 = !15388915483984877382_u64;
RET = (*_14);
SetDiscriminant(_18, 1);
_17.1.1 = (*_2) & (*_2);
_17.0 = _6.0;
_3.fld1 = -3444344303982304664_i64;
match _20 {
0 => bb10,
1 => bb6,
2 => bb8,
3 => bb13,
340282366920938463454151235394913435648 => bb15,
_ => bb14
}
}
bb13 = {
_3.fld2.1.2 = _6.1.2;
_5 = _9;
_13.0 = _6.1.1 <= (*_2);
_17.1 = (_3.fld2.1.0, _8, _3.fld2.1.2, _3.fld2.1.3, _6.1.4, _3.fld2.1.5);
_4 = [_16.0,_3.fld2.1.3,_6.1.3,_16.0,_6.1.3,_17.1.3,_6.1.3,_3.fld2.1.3];
RET = _1 & _1;
_6.1.2 = core::ptr::addr_of_mut!(_8);
_17.1.5 = _6.1.5;
RET = _1 >> (*_2);
_15 = [_10,_10,_10];
_6.4 = !_17.4;
_14 = core::ptr::addr_of_mut!(_1);
(*_2) = _3.fld2.1.3 as i8;
_4 = [_16.0,_16.0,_16.0,_6.1.3,_16.0,_6.1.3,_3.fld2.1.3,_3.fld2.1.3];
_3.fld2.1.0 = _6.1.2;
_17.2 = _3.fld2.2 >> _17.1.1;
_6.1.5 = _17.1.5;
_3.fld2.1.5 = core::ptr::addr_of_mut!(_1);
_3.fld2.2 = _17.2;
RET = _1;
_16.0 = _6.1.3;
_3.fld2.1.5 = core::ptr::addr_of_mut!((*_14));
match _3.fld1 {
0 => bb5,
340282366920938463454875106763225583993 => bb7,
_ => bb6
}
}
bb14 = {
_3.fld2.1.4 = _6.1.4;
(*_2) = _6.1.1;
_6.1 = _3.fld2.1;
_3.fld2.1.2 = _6.1.0;
_3.fld2.2 = _17.2 * _17.2;
_4 = [_6.1.3,_6.1.3,_3.fld2.1.3,_6.1.3,_17.1.3,_16.0,_3.fld2.1.3,_17.1.3];
_3.fld2.1.2 = core::ptr::addr_of_mut!(_6.1.1);
_19 = _3.fld2.4 * _6.4;
_8 = (*_2) << _17.1.1;
_18 = Adt46::Variant2 { fld0: _13.0 };
_16.1 = &(*_2);
_13.1 = Move(_16.1);
_17.4 = _6.4;
_14 = core::ptr::addr_of_mut!((*_14));
_19 = _6.4 + _3.fld2.4;
_6.1 = (_3.fld2.1.0, (*_2), _3.fld2.1.0, _3.fld2.1.3, _3.fld2.1.4, _3.fld2.1.5);
place!(Field::<bool>(Variant(_18, 2), 0)) = !_13.0;
_16 = (_3.fld2.1.3, Move(_13.1));
_17.2 = _3.fld2.2 + _3.fld2.2;
RET = (*_14);
_20 = (-9223372036854775808_isize);
_6.1.0 = _17.1.0;
_7 = [3427529997_u32,3303195198_u32,3354996353_u32,2131235056_u32,2199135929_u32,1943025514_u32];
_6.2 = !_17.2;
_17.4 = !_3.fld2.4;
_17.2 = _6.2;
_3.fld0.0 = core::ptr::addr_of_mut!(_17.1.2);
_3.fld2.1.4 = _17.1.4 + _6.1.4;
match _3.fld1 {
0 => bb6,
1 => bb2,
2 => bb8,
3 => bb9,
4 => bb10,
340282366920938463454875106763225583993 => bb12,
_ => bb11
}
}
bb15 = {
_22 = [_3.fld2.1.3,_16.0,_16.0,_16.0,_3.fld2.1.3,_17.1.3,_6.1.3,_6.1.3];
_19 = _3.fld2.4;
RET = (*_14) & (*_14);
(*_14) = RET * RET;
_16 = (_17.1.3, Move(_13.1));
_3.fld2.2 = -_17.2;
_3.fld2.1.5 = core::ptr::addr_of_mut!(_1);
place!(Field::<[i64; 1]>(Variant(_18, 1), 4)) = [_3.fld1];
_3.fld2.1.4 = -_6.1.4;
_3.fld2.1.5 = _6.1.5;
_16.0 = _17.1.3;
_3.fld2.2 = !_17.2;
Goto(bb16)
}
bb16 = {
Call(_28 = dump_var(12_usize, 9_usize, Move(_9), 12_usize, Move(_12), 21_usize, Move(_21), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_28 = dump_var(12_usize, 19_usize, Move(_19), 8_usize, Move(_8), 15_usize, Move(_15), 29_usize, _29), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn13(mut _1: i16,mut _2: i8,mut _3: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8),mut _4: [u32; 6],mut _5: char,mut _6: *mut i8,mut _7: *mut *mut i8,mut _8: i16,mut _9: [u32; 6],mut _10: [char; 8],mut _11: *mut *mut i8,mut _12: [i32; 3]) -> (*mut i8, i8, *mut i8, char, f32, *mut i128) {
mir! {
type RET = (*mut i8, i8, *mut i8, char, f32, *mut i128);
let _13: usize;
let _14: i8;
let _15: i128;
let _16: i64;
let _17: [i128; 8];
let _18: *mut char;
let _19: Adt52;
let _20: [u8; 2];
let _21: f32;
let _22: Adt54;
let _23: [u16; 2];
let _24: (char, &'static i8);
let _25: [u16; 2];
let _26: isize;
let _27: Adt56;
let _28: u32;
let _29: [u8; 5];
let _30: [i32; 3];
let _31: f64;
let _32: Adt54;
let _33: [i128; 8];
let _34: ();
let _35: ();
{
RET.1 = (*_6);
RET.1 = -(*_6);
(*_6) = !RET.1;
RET = ((*_11), _2, _3.1.0, _3.1.3, _3.1.4, _3.1.5);
_3.1.5 = RET.5;
_3.1 = ((*_11), _2, _6, _5, RET.4, RET.5);
_3.1 = ((*_11), (*_6), (*_7), _5, RET.4, RET.5);
Goto(bb1)
}
bb1 = {
_10 = [_3.1.3,RET.3,RET.3,_3.1.3,RET.3,_5,_5,RET.3];
_8 = RET.1 as i16;
_3.1.5 = RET.5;
_12 = [1194300157_i32,862708180_i32,(-1910329692_i32)];
(*_7) = core::ptr::addr_of_mut!((*_6));
_12 = [(-1670037987_i32),(-319609283_i32),(-1885491766_i32)];
(*_7) = _3.1.0;
_6 = (*_7);
_3.0 = [(-44372393774808491368402845774594049633_i128),40334769184041784285602985326831226715_i128,(-2732687088091571231613192294789463911_i128),(-71193441727656059354607603604431361541_i128),(-52219124154599085686188622641887809300_i128),(-156235113866079802760228250832598513168_i128),46616710346279877454860100201684017236_i128];
RET.1 = 9223372036854775807_isize as i8;
_16 = -1225484497444424217_i64;
RET.2 = core::ptr::addr_of_mut!(_3.1.1);
_3.4 = 228_u8;
_15 = !(-4526466284661341293899114881090038165_i128);
RET.4 = _3.1.4;
_18 = core::ptr::addr_of_mut!(_5);
_8 = !_3.2;
_3.1.0 = _3.1.2;
match _3.4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
228 => bb10,
_ => bb9
}
}
bb2 = {
Return()
}
bb3 = {
Return()
}
bb4 = {
Return()
}
bb5 = {
Return()
}
bb6 = {
Return()
}
bb7 = {
Return()
}
bb8 = {
Return()
}
bb9 = {
Return()
}
bb10 = {
_9 = _4;
RET.0 = core::ptr::addr_of_mut!(_14);
_3.1.0 = (*_11);
_4 = _9;
_18 = core::ptr::addr_of_mut!(_3.1.3);
_3.1.1 = (*_6) >> _1;
_13 = !4_usize;
_3.1.2 = RET.2;
RET.0 = _3.1.2;
_14 = -(*_6);
RET.3 = _5;
(*_11) = core::ptr::addr_of_mut!(RET.1);
RET = (_3.1.0, _3.1.1, _3.1.2, _5, _3.1.4, _3.1.5);
_17 = [_15,_15,_15,_15,_15,_15,_15,_15];
_17 = [_15,_15,_15,_15,_15,_15,_15,_15];
RET.0 = core::ptr::addr_of_mut!((*_6));
_22.fld0 = [1163966640_u32,1990087702_u32,3188709790_u32,3880981594_u32,2205940089_u32,1905047915_u32];
_18 = core::ptr::addr_of_mut!((*_18));
RET = (_3.1.2, _3.1.1, (*_7), (*_18), _3.1.4, _3.1.5);
(*_18) = RET.3;
_14 = _3.1.1;
_22.fld1 = [_15,_15,_15,_15,_15,_15,_15,_15];
_10 = [_5,(*_18),(*_18),(*_18),_3.1.3,_5,_5,_3.1.3];
Goto(bb11)
}
bb11 = {
_10 = [_5,(*_18),_5,_3.1.3,_3.1.3,RET.3,_3.1.3,_3.1.3];
_5 = RET.3;
_24.1 = &_2;
(*_6) = -RET.1;
RET.3 = _3.1.3;
_9 = [853339453_u32,69955090_u32,4074655145_u32,3203287589_u32,1019175335_u32,3356224645_u32];
_25 = [57839_u16,24977_u16];
(*_6) = _14;
_3.1.2 = core::ptr::addr_of_mut!(RET.1);
_11 = _7;
_24.0 = _5;
RET.3 = (*_18);
RET.3 = (*_18);
RET.0 = core::ptr::addr_of_mut!(_2);
_5 = RET.3;
_7 = _11;
_5 = (*_18);
_24.1 = &_3.1.1;
_3.1.0 = core::ptr::addr_of_mut!((*_6));
Goto(bb12)
}
bb12 = {
(*_7) = _6;
RET.5 = core::ptr::addr_of_mut!(_15);
_30 = [(-668050605_i32),(-1365076299_i32),(-616585070_i32)];
_23 = [51261_u16,32991_u16];
_23 = [28454_u16,58872_u16];
_29 = [_3.4,_3.4,_3.4,_3.4,_3.4];
RET.0 = core::ptr::addr_of_mut!(_3.1.1);
_11 = core::ptr::addr_of_mut!((*_7));
RET = ((*_11), _3.1.1, _6, _5, _3.1.4, _3.1.5);
_1 = 4179456471_u32 as i16;
(*_11) = core::ptr::addr_of_mut!(_14);
_3.4 = 113_u8;
RET.2 = (*_11);
_3.1.1 = (*_6);
_11 = core::ptr::addr_of_mut!(RET.0);
RET.3 = (*_18);
_6 = _3.1.2;
_21 = _3.1.4 * RET.4;
match _3.4 {
0 => bb10,
1 => bb2,
2 => bb3,
3 => bb6,
4 => bb13,
5 => bb14,
113 => bb16,
_ => bb15
}
}
bb13 = {
Return()
}
bb14 = {
Return()
}
bb15 = {
_10 = [_3.1.3,RET.3,RET.3,_3.1.3,RET.3,_5,_5,RET.3];
_8 = RET.1 as i16;
_3.1.5 = RET.5;
_12 = [1194300157_i32,862708180_i32,(-1910329692_i32)];
(*_7) = core::ptr::addr_of_mut!((*_6));
_12 = [(-1670037987_i32),(-319609283_i32),(-1885491766_i32)];
(*_7) = _3.1.0;
_6 = (*_7);
_3.0 = [(-44372393774808491368402845774594049633_i128),40334769184041784285602985326831226715_i128,(-2732687088091571231613192294789463911_i128),(-71193441727656059354607603604431361541_i128),(-52219124154599085686188622641887809300_i128),(-156235113866079802760228250832598513168_i128),46616710346279877454860100201684017236_i128];
RET.1 = 9223372036854775807_isize as i8;
_16 = -1225484497444424217_i64;
RET.2 = core::ptr::addr_of_mut!(_3.1.1);
_3.4 = 228_u8;
_15 = !(-4526466284661341293899114881090038165_i128);
RET.4 = _3.1.4;
_18 = core::ptr::addr_of_mut!(_5);
_8 = !_3.2;
_3.1.0 = _3.1.2;
match _3.4 {
0 => bb2,
1 => bb3,
2 => bb4,
3 => bb5,
4 => bb6,
5 => bb7,
6 => bb8,
228 => bb10,
_ => bb9
}
}
bb16 = {
_3.1.4 = RET.4 + _21;
RET.1 = !_14;
_3.1.0 = core::ptr::addr_of_mut!(RET.1);
RET.5 = core::ptr::addr_of_mut!(_15);
_31 = _16 as f64;
_32.fld0 = _22.fld0;
_26 = _3.4 as isize;
_2 = 33728_u16 as i8;
RET.5 = core::ptr::addr_of_mut!(_15);
_20 = [_3.4,_3.4];
(*_6) = _14;
Goto(bb17)
}
bb17 = {
Call(_34 = dump_var(13_usize, 17_usize, Move(_17), 2_usize, Move(_2), 30_usize, Move(_30), 9_usize, Move(_9)), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Call(_34 = dump_var(13_usize, 4_usize, Move(_4), 10_usize, Move(_10), 15_usize, Move(_15), 25_usize, Move(_25)), ReturnTo(bb19), UnwindUnreachable())
}
bb19 = {
Call(_34 = dump_var(13_usize, 14_usize, Move(_14), 35_usize, _35, 35_usize, _35, 35_usize, _35), ReturnTo(bb20), UnwindUnreachable())
}
bb20 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
fn fn14(mut _1: &'static i8,mut _2: i8,mut _3: *mut i8,mut _4: *mut i8,mut _5: *mut i8,mut _6: (*mut i8, i8, *mut i8, char, f32, *mut i128),mut _7: *mut i8,mut _8: i8,mut _9: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8),mut _10: (*mut i8, i8, *mut i8, char, f32, *mut i128),mut _11: *mut i8,mut _12: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8),mut _13: *mut *mut i8) -> u8 {
mir! {
type RET = u8;
let _14: [char; 8];
let _15: Adt55;
let _16: (bool, &'static i8);
let _17: f32;
let _18: u32;
let _19: Adt50;
let _20: (*mut *mut i8,);
let _21: [u32; 6];
let _22: bool;
let _23: [i128; 7];
let _24: (char, &'static i8);
let _25: f64;
let _26: u16;
let _27: i16;
let _28: Adt47;
let _29: isize;
let _30: Adt45;
let _31: ();
let _32: ();
{
_12.1.1 = _10.1;
_12.3 = [13759_u16,42632_u16];
_10.3 = _6.3;
_6.4 = 3697058863_u32 as f32;
_9.1 = _6;
_9.1.5 = _6.5;
_2 = 8393393406318765355_usize as i8;
_10.0 = core::ptr::addr_of_mut!((*_11));
_3 = _7;
_2 = (*_4) | _10.1;
_12.1.4 = (-1325332930639388094321875478018038366_i128) as f32;
_10.5 = _9.1.5;
(*_13) = _7;
Call(_12.2 = core::intrinsics::bswap(_9.2), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
(*_13) = core::ptr::addr_of_mut!((*_11));
RET = 10297063018050248771_u64 as u8;
_12.1.1 = _9.4 as i8;
_9.1.5 = _10.5;
_4 = _12.1.2;
_9.1.1 = _6.1;
_12.1.1 = _8;
RET = 17909901646123650002_u64 as u8;
_12.1.5 = _10.5;
_12.1.4 = -_10.4;
_9.1.5 = _10.5;
_12.2 = _9.2 & _9.2;
_6.3 = _10.3;
_11 = core::ptr::addr_of_mut!((*_4));
_10.4 = _6.4 * _12.1.4;
_12.1 = (_3, (*_11), _6.0, _10.3, _6.4, _10.5);
_6.4 = _10.4 + _9.1.4;
_9.2 = _12.2;
_10.4 = -_9.1.4;
_16.1 = &_9.1.1;
_19.fld1.fld4 = !21246_u16;
_6 = (_10.2, _8, _3, _9.1.3, _10.4, _12.1.5);
_12.1.0 = core::ptr::addr_of_mut!((*_5));
Call(_19.fld1.fld3.4 = core::intrinsics::bswap(_12.4), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
_19.fld1.fld2 = !4086832318223291404_u64;
_19.fld1.fld2 = 14664286979690602697_u64;
_3 = _9.1.0;
_19.fld1.fld3.1.3 = _9.1.3;
_19.fld1.fld5 = _12.2 as i32;
_18 = !2390198429_u32;
_10.4 = _18 as f32;
_19.fld2.0 = _13;
_6.0 = _9.1.2;
_12.1.0 = core::ptr::addr_of_mut!((*_11));
_10.0 = _4;
_19.fld1.fld3.1 = _9.1;
_6.3 = _19.fld1.fld3.1.3;
_24.1 = &(*_1);
_17 = _19.fld1.fld3.1.4;
_23 = [58736111976185146963734215538074012431_i128,42976748608513148947231105226361997564_i128,(-73855386704185335141231782137467541820_i128),(-41349417451264730155412496493549781870_i128),154494976468717114245933948567947588506_i128,90250515485451861814434984685854953857_i128,5888904408764519213850901398641556483_i128];
_19.fld1.fld0.3 = 234970830059143574438098597929122869162_u128 as i32;
_9.1.2 = _12.1.2;
_14 = [_6.3,_12.1.3,_12.1.3,_9.1.3,_12.1.3,_19.fld1.fld3.1.3,_9.1.3,_12.1.3];
_10 = (_5, (*_4), _9.1.2, _12.1.3, _19.fld1.fld3.1.4, _9.1.5);
_5 = core::ptr::addr_of_mut!((*_11));
_12.1.5 = core::ptr::addr_of_mut!(_19.fld1.fld0.4);
_19.fld1.fld3.1.1 = (*_5) & _2;
_9.4 = 284496075924835638532818943166160228784_u128 as u8;
match _19.fld1.fld2 {
0 => bb1,
1 => bb3,
14664286979690602697 => bb5,
_ => bb4
}
}
bb3 = {
(*_13) = core::ptr::addr_of_mut!((*_11));
RET = 10297063018050248771_u64 as u8;
_12.1.1 = _9.4 as i8;
_9.1.5 = _10.5;
_4 = _12.1.2;
_9.1.1 = _6.1;
_12.1.1 = _8;
RET = 17909901646123650002_u64 as u8;
_12.1.5 = _10.5;
_12.1.4 = -_10.4;
_9.1.5 = _10.5;
_12.2 = _9.2 & _9.2;
_6.3 = _10.3;
_11 = core::ptr::addr_of_mut!((*_4));
_10.4 = _6.4 * _12.1.4;
_12.1 = (_3, (*_11), _6.0, _10.3, _6.4, _10.5);
_6.4 = _10.4 + _9.1.4;
_9.2 = _12.2;
_10.4 = -_9.1.4;
_16.1 = &_9.1.1;
_19.fld1.fld4 = !21246_u16;
_6 = (_10.2, _8, _3, _9.1.3, _10.4, _12.1.5);
_12.1.0 = core::ptr::addr_of_mut!((*_5));
Call(_19.fld1.fld3.4 = core::intrinsics::bswap(_12.4), ReturnTo(bb2), UnwindUnreachable())
}
bb4 = {
Return()
}
bb5 = {
_19.fld1.fld0.1 = !_19.fld1.fld4;
_19.fld1.fld1 = [_10.3,_19.fld1.fld3.1.3,_19.fld1.fld3.1.3,_9.1.3,_9.1.3,_12.1.3,_9.1.3,_19.fld1.fld3.1.3];
_16 = (true, Move(_1));
_6.5 = _19.fld1.fld3.1.5;
_9.0 = [(-51071008322988120819628012666825702255_i128),165374793728664623536875476700499699176_i128,157447150937147543809556962624259387697_i128,130632457269162617001209260942267460763_i128,(-1581277305839998687720107303890815920_i128),122133051629988978217539412826094588906_i128,169124798198214278944957264251149559768_i128];
match _19.fld1.fld2 {
0 => bb3,
1 => bb2,
2 => bb6,
3 => bb7,
4 => bb8,
5 => bb9,
6 => bb10,
14664286979690602697 => bb12,
_ => bb11
}
}
bb6 = {
Return()
}
bb7 = {
(*_13) = core::ptr::addr_of_mut!((*_11));
RET = 10297063018050248771_u64 as u8;
_12.1.1 = _9.4 as i8;
_9.1.5 = _10.5;
_4 = _12.1.2;
_9.1.1 = _6.1;
_12.1.1 = _8;
RET = 17909901646123650002_u64 as u8;
_12.1.5 = _10.5;
_12.1.4 = -_10.4;
_9.1.5 = _10.5;
_12.2 = _9.2 & _9.2;
_6.3 = _10.3;
_11 = core::ptr::addr_of_mut!((*_4));
_10.4 = _6.4 * _12.1.4;
_12.1 = (_3, (*_11), _6.0, _10.3, _6.4, _10.5);
_6.4 = _10.4 + _9.1.4;
_9.2 = _12.2;
_10.4 = -_9.1.4;
_16.1 = &_9.1.1;
_19.fld1.fld4 = !21246_u16;
_6 = (_10.2, _8, _3, _9.1.3, _10.4, _12.1.5);
_12.1.0 = core::ptr::addr_of_mut!((*_5));
Call(_19.fld1.fld3.4 = core::intrinsics::bswap(_12.4), ReturnTo(bb2), UnwindUnreachable())
}
bb8 = {
_19.fld1.fld2 = !4086832318223291404_u64;
_19.fld1.fld2 = 14664286979690602697_u64;
_3 = _9.1.0;
_19.fld1.fld3.1.3 = _9.1.3;
_19.fld1.fld5 = _12.2 as i32;
_18 = !2390198429_u32;
_10.4 = _18 as f32;
_19.fld2.0 = _13;
_6.0 = _9.1.2;
_12.1.0 = core::ptr::addr_of_mut!((*_11));
_10.0 = _4;
_19.fld1.fld3.1 = _9.1;
_6.3 = _19.fld1.fld3.1.3;
_24.1 = &(*_1);
_17 = _19.fld1.fld3.1.4;
_23 = [58736111976185146963734215538074012431_i128,42976748608513148947231105226361997564_i128,(-73855386704185335141231782137467541820_i128),(-41349417451264730155412496493549781870_i128),154494976468717114245933948567947588506_i128,90250515485451861814434984685854953857_i128,5888904408764519213850901398641556483_i128];
_19.fld1.fld0.3 = 234970830059143574438098597929122869162_u128 as i32;
_9.1.2 = _12.1.2;
_14 = [_6.3,_12.1.3,_12.1.3,_9.1.3,_12.1.3,_19.fld1.fld3.1.3,_9.1.3,_12.1.3];
_10 = (_5, (*_4), _9.1.2, _12.1.3, _19.fld1.fld3.1.4, _9.1.5);
_5 = core::ptr::addr_of_mut!((*_11));
_12.1.5 = core::ptr::addr_of_mut!(_19.fld1.fld0.4);
_19.fld1.fld3.1.1 = (*_5) & _2;
_9.4 = 284496075924835638532818943166160228784_u128 as u8;
match _19.fld1.fld2 {
0 => bb1,
1 => bb3,
14664286979690602697 => bb5,
_ => bb4
}
}
bb9 = {
(*_13) = core::ptr::addr_of_mut!((*_11));
RET = 10297063018050248771_u64 as u8;
_12.1.1 = _9.4 as i8;
_9.1.5 = _10.5;
_4 = _12.1.2;
_9.1.1 = _6.1;
_12.1.1 = _8;
RET = 17909901646123650002_u64 as u8;
_12.1.5 = _10.5;
_12.1.4 = -_10.4;
_9.1.5 = _10.5;
_12.2 = _9.2 & _9.2;
_6.3 = _10.3;
_11 = core::ptr::addr_of_mut!((*_4));
_10.4 = _6.4 * _12.1.4;
_12.1 = (_3, (*_11), _6.0, _10.3, _6.4, _10.5);
_6.4 = _10.4 + _9.1.4;
_9.2 = _12.2;
_10.4 = -_9.1.4;
_16.1 = &_9.1.1;
_19.fld1.fld4 = !21246_u16;
_6 = (_10.2, _8, _3, _9.1.3, _10.4, _12.1.5);
_12.1.0 = core::ptr::addr_of_mut!((*_5));
Call(_19.fld1.fld3.4 = core::intrinsics::bswap(_12.4), ReturnTo(bb2), UnwindUnreachable())
}
bb10 = {
Return()
}
bb11 = {
Return()
}
bb12 = {
_20.0 = _13;
_14 = [_10.3,_19.fld1.fld3.1.3,_19.fld1.fld3.1.3,_10.3,_10.3,_6.3,_19.fld1.fld3.1.3,_9.1.3];
_14 = _19.fld1.fld1;
RET = _19.fld1.fld4 as u8;
_19.fld1.fld3.1 = (_3, (*_7), _6.2, _9.1.3, _17, _10.5);
_19.fld1.fld3.2 = _12.2 * _9.2;
Goto(bb13)
}
bb13 = {
_16.1 = &(*_5);
_19.fld1.fld3.1.3 = _6.3;
(*_7) = _9.1.4 as i8;
_16.0 = !false;
_19.fld1.fld3.4 = _19.fld1.fld3.2 as u8;
match _19.fld1.fld2 {
0 => bb1,
1 => bb2,
2 => bb14,
14664286979690602697 => bb16,
_ => bb15
}
}
bb14 = {
(*_13) = core::ptr::addr_of_mut!((*_11));
RET = 10297063018050248771_u64 as u8;
_12.1.1 = _9.4 as i8;
_9.1.5 = _10.5;
_4 = _12.1.2;
_9.1.1 = _6.1;
_12.1.1 = _8;
RET = 17909901646123650002_u64 as u8;
_12.1.5 = _10.5;
_12.1.4 = -_10.4;
_9.1.5 = _10.5;
_12.2 = _9.2 & _9.2;
_6.3 = _10.3;
_11 = core::ptr::addr_of_mut!((*_4));
_10.4 = _6.4 * _12.1.4;
_12.1 = (_3, (*_11), _6.0, _10.3, _6.4, _10.5);
_6.4 = _10.4 + _9.1.4;
_9.2 = _12.2;
_10.4 = -_9.1.4;
_16.1 = &_9.1.1;
_19.fld1.fld4 = !21246_u16;
_6 = (_10.2, _8, _3, _9.1.3, _10.4, _12.1.5);
_12.1.0 = core::ptr::addr_of_mut!((*_5));
Call(_19.fld1.fld3.4 = core::intrinsics::bswap(_12.4), ReturnTo(bb2), UnwindUnreachable())
}
bb15 = {
_19.fld1.fld2 = !4086832318223291404_u64;
_19.fld1.fld2 = 14664286979690602697_u64;
_3 = _9.1.0;
_19.fld1.fld3.1.3 = _9.1.3;
_19.fld1.fld5 = _12.2 as i32;
_18 = !2390198429_u32;
_10.4 = _18 as f32;
_19.fld2.0 = _13;
_6.0 = _9.1.2;
_12.1.0 = core::ptr::addr_of_mut!((*_11));
_10.0 = _4;
_19.fld1.fld3.1 = _9.1;
_6.3 = _19.fld1.fld3.1.3;
_24.1 = &(*_1);
_17 = _19.fld1.fld3.1.4;
_23 = [58736111976185146963734215538074012431_i128,42976748608513148947231105226361997564_i128,(-73855386704185335141231782137467541820_i128),(-41349417451264730155412496493549781870_i128),154494976468717114245933948567947588506_i128,90250515485451861814434984685854953857_i128,5888904408764519213850901398641556483_i128];
_19.fld1.fld0.3 = 234970830059143574438098597929122869162_u128 as i32;
_9.1.2 = _12.1.2;
_14 = [_6.3,_12.1.3,_12.1.3,_9.1.3,_12.1.3,_19.fld1.fld3.1.3,_9.1.3,_12.1.3];
_10 = (_5, (*_4), _9.1.2, _12.1.3, _19.fld1.fld3.1.4, _9.1.5);
_5 = core::ptr::addr_of_mut!((*_11));
_12.1.5 = core::ptr::addr_of_mut!(_19.fld1.fld0.4);
_19.fld1.fld3.1.1 = (*_5) & _2;
_9.4 = 284496075924835638532818943166160228784_u128 as u8;
match _19.fld1.fld2 {
0 => bb1,
1 => bb3,
14664286979690602697 => bb5,
_ => bb4
}
}
bb16 = {
Goto(bb17)
}
bb17 = {
Call(_31 = dump_var(14_usize, 14_usize, Move(_14), 2_usize, Move(_2), 32_usize, _32, 32_usize, _32), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn15(mut _1: [char; 8],mut _2: [char; 8],mut _3: [i128; 7],mut _4: *mut i128,mut _5: i128,mut _6: *mut i128,mut _7: *mut i8,mut _8: [char; 8],mut _9: *mut i8,mut _10: *mut i8,mut _11: *mut i8,mut _12: [char; 8],mut _13: *mut i128,mut _14: *mut i128,mut _15: *mut [i128; 7]) -> i8 {
mir! {
type RET = i8;
let _16: Adt46;
let _17: ();
let _18: ();
{
_10 = _11;
(*_13) = _5;
(*_15) = [(*_13),(*_4),_5,(*_14),(*_6),(*_13),(*_13)];
RET = 123_i8;
RET = !14_i8;
_15 = core::ptr::addr_of_mut!((*_15));
_14 = core::ptr::addr_of_mut!((*_6));
_5 = (*_4) | (*_6);
RET = (*_14) as i8;
Goto(bb1)
}
bb1 = {
Call(_17 = dump_var(15_usize, 5_usize, Move(_5), 3_usize, Move(_3), 2_usize, Move(_2), 18_usize, _18), ReturnTo(bb2), UnwindUnreachable())
}
bb2 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn16(mut _1: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8),mut _2: *mut i8,mut _3: char,mut _4: (*mut *mut i8,),mut _5: *mut *mut i8,mut _6: *mut *mut i8,mut _7: *mut i8,mut _8: *mut i8,mut _9: &'static i8,mut _10: *mut i8,mut _11: i8,mut _12: *mut i8) -> [i128; 7] {
mir! {
type RET = [i128; 7];
let _13: [u16; 2];
let _14: [u8; 5];
let _15: [u32; 6];
let _16: isize;
let _17: u128;
let _18: bool;
let _19: isize;
let _20: Adt55;
let _21: [i128; 7];
let _22: isize;
let _23: i64;
let _24: isize;
let _25: i64;
let _26: Adt49;
let _27: [i32; 3];
let _28: i8;
let _29: [u8; 5];
let _30: *mut i128;
let _31: u128;
let _32: f32;
let _33: [u8; 5];
let _34: [u16; 2];
let _35: isize;
let _36: Adt54;
let _37: u32;
let _38: Adt55;
let _39: bool;
let _40: char;
let _41: f64;
let _42: i128;
let _43: char;
let _44: ();
let _45: ();
{
RET = [(-45574991381132833219715437659113427156_i128),127800279528374730847995866890679752388_i128,156776314100684221386001202210922918912_i128,53385260744162636851653745850319870111_i128,165676529823348233039805709562371479345_i128,(-137212440826049462458571471913544598375_i128),(-92090142000511041686012181364005625905_i128)];
(*_5) = _8;
_11 = 5_usize as i8;
(*_5) = core::ptr::addr_of_mut!(_11);
_10 = _8;
(*_2) = 179128729921860760533603990252678976325_u128 as i8;
(*_2) = _1.1.1 >> (*_9);
Goto(bb1)
}
bb1 = {
_1.1.1 = (*_10) << (*_9);
(*_7) = (*_9);
_5 = core::ptr::addr_of_mut!((*_5));
(*_8) = (*_9);
_14 = [_1.4,_1.4,_1.4,_1.4,_1.4];
(*_2) = !(*_9);
_12 = _8;
(*_2) = !(*_9);
_4 = (_5,);
RET = [119949708370723419420287427948224032607_i128,(-41492146990526196899802198670480837265_i128),139446241450097343918008868153850234678_i128,99217615980029983071872178162839480451_i128,44970663482830645672144768610664031230_i128,141104815560010428619796816130656162962_i128,(-16518050413507579667312090637396919630_i128)];
_5 = _6;
_18 = false;
_4.0 = _6;
Goto(bb2)
}
bb2 = {
(*_6) = _12;
_17 = 306057988493181154745863908119606003117_u128;
_1.1.3 = _3;
_15 = [2348162228_u32,435125325_u32,174471367_u32,2299859114_u32,1198689657_u32,2932707584_u32];
(*_5) = _2;
_1.3 = [4231_u16,31017_u16];
_13 = _1.3;
(*_5) = core::ptr::addr_of_mut!((*_9));
(*_5) = _1.1.0;
_3 = _1.1.3;
(*_2) = (*_9);
_1.0 = RET;
match _17 {
0 => bb3,
306057988493181154745863908119606003117 => bb5,
_ => bb4
}
}
bb3 = {
_1.1.1 = (*_10) << (*_9);
(*_7) = (*_9);
_5 = core::ptr::addr_of_mut!((*_5));
(*_8) = (*_9);
_14 = [_1.4,_1.4,_1.4,_1.4,_1.4];
(*_2) = !(*_9);
_12 = _8;
(*_2) = !(*_9);
_4 = (_5,);
RET = [119949708370723419420287427948224032607_i128,(-41492146990526196899802198670480837265_i128),139446241450097343918008868153850234678_i128,99217615980029983071872178162839480451_i128,44970663482830645672144768610664031230_i128,141104815560010428619796816130656162962_i128,(-16518050413507579667312090637396919630_i128)];
_5 = _6;
_18 = false;
_4.0 = _6;
Goto(bb2)
}
bb4 = {
Return()
}
bb5 = {
(*_7) = (*_9);
(*_2) = _1.1.1;
_1.0 = RET;
(*_5) = _1.1.0;
(*_12) = (-156110276121490727132398149712224354673_i128) as i8;
(*_10) = -_1.1.1;
_19 = 9223372036854775807_isize;
_17 = _1.2 as u128;
_1.1.2 = _8;
_19 = 9223372036854775807_isize | (-9223372036854775808_isize);
_1.0 = [(-138040884119406376687704640637983111430_i128),65797868295412682656202456478711078594_i128,43261239949355750836979920953094602985_i128,134460258950313265687886730305467040975_i128,19260975295363004914872051527767706016_i128,(-163866967281135185770916492313116512801_i128),142200231732020594350913671113135078401_i128];
RET = [(-19849869293631668283984564245618060183_i128),19009060706860156551016520432239714950_i128,112636357026857570091271689968281013536_i128,64128448257283683154143231133513589753_i128,75675447201734760900095591947399027339_i128,(-160247945493452662934258805869172804843_i128),150031467181862505590737113440721541619_i128];
_1.1.2 = _7;
Goto(bb6)
}
bb6 = {
_14 = [_1.4,_1.4,_1.4,_1.4,_1.4];
(*_10) = _1.1.1;
_22 = (-4825409117065056370198130320928657764_i128) as isize;
(*_5) = core::ptr::addr_of_mut!(_1.1.1);
_23 = (-7465018374807692497_i64) ^ 3050541629037955844_i64;
Goto(bb7)
}
bb7 = {
_1.0 = [107870801504026586963013330292464354707_i128,52601068542791667512437130703727006483_i128,(-49761547470431056535210183258817364322_i128),(-20818129043471485183569126758747137525_i128),31004011370186011043482392867544373951_i128,106172425467638937936229289231977668231_i128,87809854762043607371512560165860682476_i128];
_1.2 = 13852_i16 * (-16752_i16);
_5 = core::ptr::addr_of_mut!(_10);
_3 = _1.1.3;
(*_2) = !(*_9);
_1.3 = _13;
_1.0 = [(-75449471580177706452431008551509373434_i128),22292393150785542006134710660004160713_i128,(-140444941861602819345562522911287613455_i128),(-51964586369962452483418190187842557849_i128),49471015564916417300948995708041397597_i128,(-4244803161200680467361626458251333396_i128),(-103172111209274141820762304952106049417_i128)];
(*_10) = !(*_9);
_12 = core::ptr::addr_of_mut!((*_7));
_1.4 = !121_u8;
(*_5) = (*_6);
_5 = core::ptr::addr_of_mut!(_8);
_1.1.1 = (*_8) ^ (*_2);
_25 = _23;
(*_10) = 3321341941_u32 as i8;
_15 = [3294067061_u32,986965886_u32,1642878521_u32,1695366854_u32,1002653558_u32,3849418003_u32];
(*_10) = (*_7) ^ (*_9);
_13 = [44669_u16,21814_u16];
_1.4 = 34_u8 << _1.1.1;
(*_12) = (*_10);
_24 = !_22;
_2 = core::ptr::addr_of_mut!((*_10));
(*_5) = core::ptr::addr_of_mut!((*_10));
RET = [37542170961519410739633466854195945575_i128,47943951079590476757881713894076428427_i128,68637909039037651131166942984915613514_i128,(-128555004902440869322989807180092971687_i128),66298208049226690895313686098293048149_i128,(-152485633302972751783739893778037067779_i128),27575847434592871183468802063335067476_i128];
(*_7) = (*_2);
Goto(bb8)
}
bb8 = {
_16 = _1.1.4 as isize;
_10 = (*_5);
_1.3 = [3822_u16,29486_u16];
Goto(bb9)
}
bb9 = {
_15 = [3819998835_u32,3349679642_u32,2690905206_u32,2028389837_u32,3418210604_u32,218901041_u32];
_1.3 = _13;
_19 = _16;
_1.1.1 = 492041657_i32 as i8;
_1.1.3 = _3;
_4 = (_5,);
_14 = [_1.4,_1.4,_1.4,_1.4,_1.4];
_12 = core::ptr::addr_of_mut!(_1.1.1);
_22 = _1.2 as isize;
(*_5) = _1.1.0;
_35 = _1.1.4 as isize;
(*_2) = !(*_7);
_34 = _13;
_34 = [60212_u16,13464_u16];
Goto(bb10)
}
bb10 = {
_8 = (*_6);
_33 = [_1.4,_1.4,_1.4,_1.4,_1.4];
_36.fld1 = [147603452509680887072390967217453644224_i128,12798710464883003872335433339192766574_i128,(-146926751632619156756262711304844453087_i128),114951359339634947438035107548819148441_i128,(-94058265908103372582184953291650804391_i128),(-85629492801497421043518338793119191278_i128),145347047970085960937534726923668560630_i128,(-124185373456495003860593290601261719308_i128)];
(*_7) = _25 as i8;
Goto(bb11)
}
bb11 = {
_37 = 3011126281_u32 >> (*_12);
_25 = _23;
Goto(bb12)
}
bb12 = {
_1.0 = [(-98440740863841440810984350617939472802_i128),7133732875846678707950184014705992373_i128,(-26662058021494017492354948898478391763_i128),(-24892880983020991425035634306859726098_i128),(-168691393105969069930865927330188615829_i128),95829637887123412233561764618993480742_i128,(-43819882007262958238643059011447113502_i128)];
_35 = 11504_u16 as isize;
(*_7) = (*_2) | (*_12);
_23 = _25 | _25;
_35 = _18 as isize;
(*_5) = _10;
_33 = [_1.4,_1.4,_1.4,_1.4,_1.4];
_33 = [_1.4,_1.4,_1.4,_1.4,_1.4];
_15 = [_37,_37,_37,_37,_37,_37];
_25 = _23 | _23;
_2 = core::ptr::addr_of_mut!((*_9));
(*_12) = 5002153258438686649_u64 as i8;
_35 = _19 >> _17;
_36.fld1 = [(-21934034699215571254920252229774001456_i128),57219944644406388498352072846816001231_i128,(-34415455546110404274035173239507838903_i128),82225627544702761270826577960964931388_i128,(-124202688247199327866842964928241898452_i128),(-40167002587087382063319888678051824804_i128),78465518162941409376765275269370385316_i128,114852735677237702970056352237996579642_i128];
_29 = [_1.4,_1.4,_1.4,_1.4,_1.4];
_27 = [(-1075089745_i32),(-2128821921_i32),1452972974_i32];
_30 = _1.1.5;
_34 = [57441_u16,27016_u16];
(*_6) = _2;
_9 = &_11;
Goto(bb13)
}
bb13 = {
_43 = _1.1.3;
(*_6) = core::ptr::addr_of_mut!((*_8));
(*_12) = (*_2);
Goto(bb14)
}
bb14 = {
_4 = (_5,);
_31 = (-1483554400_i32) as u128;
(*_10) = (*_2);
_24 = _1.2 as isize;
Goto(bb15)
}
bb15 = {
Call(_44 = dump_var(16_usize, 29_usize, Move(_29), 3_usize, Move(_3), 25_usize, Move(_25), 37_usize, Move(_37)), ReturnTo(bb16), UnwindUnreachable())
}
bb16 = {
Call(_44 = dump_var(16_usize, 23_usize, Move(_23), 11_usize, Move(_11), 35_usize, Move(_35), 22_usize, Move(_22)), ReturnTo(bb17), UnwindUnreachable())
}
bb17 = {
Call(_44 = dump_var(16_usize, 43_usize, Move(_43), 17_usize, Move(_17), 45_usize, _45, 45_usize, _45), ReturnTo(bb18), UnwindUnreachable())
}
bb18 = {
Return()
}

}
}
#[custom_mir(dialect = "runtime", phase = "initial")]
pub fn fn17(mut _1: i8,mut _2: (*mut i8, i8, *mut i8, char, f32, *mut i128),mut _3: (*mut i8, i8, *mut i8, char, f32, *mut i128),mut _4: char,mut _5: (*mut *mut i8,),mut _6: (*mut *mut i8,),mut _7: (*mut *mut i8,),mut _8: (*mut i8, i8, *mut i8, char, f32, *mut i128),mut _9: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8),mut _10: char) -> u8 {
mir! {
type RET = u8;
let _11: Adt54;
let _12: Adt44;
let _13: Adt56;
let _14: [u8; 5];
let _15: [u8; 2];
let _16: [u16; 2];
let _17: [char; 8];
let _18: ();
let _19: ();
{
_2 = _3;
_4 = _3.3;
_9.1.5 = _3.5;
RET = _9.4;
_9.2 = (-20591_i16) << _2.1;
RET = _9.4 - _9.4;
_7 = _5;
_9.4 = 209365815769817956517721200586300753202_u128 as u8;
_2.0 = _3.0;
_8.3 = _10;
_9.1 = (_8.2, _1, _2.0, _2.3, _3.4, _2.5);
_8.2 = core::ptr::addr_of_mut!(_8.1);
_9.4 = RET;
_3.3 = _8.3;
_9.4 = 4_usize as u8;
_2.1 = _9.1.1 >> _9.1.1;
_2.2 = core::ptr::addr_of_mut!(_3.1);
_11.fld1 = [49017282752672707406262030864758439297_i128,34285275243451145761415276928116031830_i128,(-91328804322197975153069139954314439816_i128),27199142027968518293575627757125501433_i128,(-21367373718767905383721575176929007970_i128),(-108128042483378585936090793017012805931_i128),29581405844318001855578938620380888046_i128,12958070309375106447608346380151126616_i128];
_4 = _2.3;
Call(_3.1 = core::intrinsics::bswap(_2.1), ReturnTo(bb1), UnwindUnreachable())
}
bb1 = {
_9.1.4 = _3.4 - _3.4;
_9.4 = !RET;
Goto(bb2)
}
bb2 = {
_9.1.4 = _8.4 - _8.4;
_11.fld0 = [3931631641_u32,3246772338_u32,1354787583_u32,2045129511_u32,3987860340_u32,501541656_u32];
_5 = (_7.0,);
_9.1.5 = _3.5;
_9.3 = [29312_u16,7626_u16];
_3.1 = _1 + _1;
_4 = _2.3;
RET = _9.1.4 as u8;
_8.5 = _3.5;
RET = _9.4;
_8.0 = core::ptr::addr_of_mut!(_2.1);
RET = _9.4 - _9.4;
_9.1.2 = _8.2;
Goto(bb3)
}
bb3 = {
Call(_18 = dump_var(17_usize, 1_usize, Move(_1), 19_usize, _19, 19_usize, _19, 19_usize, _19), ReturnTo(bb4), UnwindUnreachable())
}
bb4 = {
Return()
}

}
}
pub fn main() {
                fn0(std::hint::black_box(4106568299_u32), std::hint::black_box(9223372036854775807_isize));
                
            }
impl PrintFDebug for Adt40{
	unsafe fn printf_debug(&self){unsafe{printf("Adt40::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt40 {
Variant0{
fld0: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8),
fld1: *mut [i128; 7],
fld2: i128,

},
Variant1{
fld0: *mut i128,
fld1: (*mut *mut i8,),
fld2: u8,
fld3: [i128; 8],
fld4: *mut [i128; 7],
fld5: i32,
fld6: (*mut i8, i8, *mut i8, char, f32, *mut i128),
fld7: [i64; 1],

},
Variant2{
fld0: ([char; 8], u16, *mut [i128; 7], i32, i128),
fld1: char,
fld2: isize,
fld3: u64,
fld4: u128,
fld5: *const f64,
fld6: i64,
fld7: *mut *mut i8,

},
Variant3{
fld0: (*mut i8, i8, *mut i8, char, f32, *mut i128),
fld1: f32,
fld2: u16,
fld3: [u8; 2],
fld4: *mut char,
fld5: *mut i128,
fld6: [i128; 7],
fld7: i128,

}}
impl PrintFDebug for Adt41{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt41{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt41 {
fld0: (*mut *mut i8,),
fld1: i64,
fld2: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8),
}
impl PrintFDebug for Adt42{
	unsafe fn printf_debug(&self){unsafe{printf("Adt42::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt42 {
Variant0{
fld0: (*mut *mut i8,),
fld1: Adt41,
fld2: ([char; 8], u16, *mut [i128; 7], i32, i128),
fld3: i8,
fld4: u16,

},
Variant1{
fld0: *mut i8,
fld1: u8,
fld2: [u32; 6],

},
Variant2{
fld0: u16,
fld1: i64,
fld2: (*mut i8, i8, *mut i8, char, f32, *mut i128),

},
Variant3{
fld0: [i128; 8],
fld1: u16,
fld2: i16,
fld3: *mut i128,

}}
impl PrintFDebug for Adt43{
	unsafe fn printf_debug(&self){unsafe{printf("Adt43::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt43 {
Variant0{
fld0: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8),
fld1: [char; 8],

},
Variant1{
fld0: *mut i8,
fld1: char,
fld2: ([char; 8], u16, *mut [i128; 7], i32, i128),
fld3: [i64; 1],
fld4: (*mut *mut i8,),

},
Variant2{
fld0: *mut [u32; 6],
fld1: *mut [i128; 7],

}}
impl PrintFDebug for Adt44{
	unsafe fn printf_debug(&self){unsafe{printf("Adt44::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt44 {
Variant0{
fld0: Adt43,
fld1: *mut [u32; 6],
fld2: isize,
fld3: usize,
fld4: *const f64,
fld5: i32,
fld6: i64,
fld7: *mut [i128; 7],

},
Variant1{
fld0: u64,
fld1: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8),
fld2: ([char; 8], u16, *mut [i128; 7], i32, i128),
fld3: i8,
fld4: i16,
fld5: *mut char,

}}
impl PrintFDebug for Adt45{
	unsafe fn printf_debug(&self){unsafe{printf("Adt45::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt45 {
Variant0{
fld0: *mut [i128; 7],
fld1: u8,
fld2: [u8; 2],
fld3: f32,
fld4: usize,
fld5: [i64; 1],
fld6: (*mut i8, i8, *mut i8, char, f32, *mut i128),
fld7: *mut char,

},
Variant1{
fld0: [u16; 2],
fld1: char,
fld2: u8,
fld3: *mut i128,
fld4: *mut i8,
fld5: u64,
fld6: i64,
fld7: i128,

},
Variant2{
fld0: u64,
fld1: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8),
fld2: Adt42,
fld3: *const f64,
fld4: u32,
fld5: u128,
fld6: [u32; 6],
fld7: i128,

}}
impl PrintFDebug for Adt46{
	unsafe fn printf_debug(&self){unsafe{printf("Adt46::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt46 {
Variant0{
fld0: i64,
fld1: char,
fld2: *mut i128,
fld3: u8,
fld4: ([char; 8], u16, *mut [i128; 7], i32, i128),
fld5: *mut *mut i8,

},
Variant1{
fld0: bool,
fld1: Adt45,
fld2: f32,
fld3: *const f64,
fld4: [i64; 1],

},
Variant2{
fld0: bool,

},
Variant3{
fld0: [i128; 8],

}}
impl PrintFDebug for Adt47{
	unsafe fn printf_debug(&self){unsafe{printf("Adt47::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt47 {
Variant0{
fld0: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8),

},
Variant1{
fld0: u64,
fld1: i128,

},
Variant2{
fld0: [u8; 5],
fld1: *mut i128,
fld2: i32,
fld3: [u16; 2],
fld4: [i128; 7],

},
Variant3{
fld0: [i64; 1],
fld1: char,
fld2: isize,
fld3: *mut [u32; 6],
fld4: usize,

}}
impl PrintFDebug for Adt48{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt48{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt48 {
fld0: ([char; 8], u16, *mut [i128; 7], i32, i128),
fld1: [char; 8],
fld2: u64,
fld3: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8),
fld4: u16,
fld5: i32,
}
impl PrintFDebug for Adt49{
	unsafe fn printf_debug(&self){unsafe{printf("Adt49::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,fld5,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt49 {
Variant0{
fld0: Adt41,
fld1: u8,
fld2: *const f64,
fld3: *mut i8,
fld4: Adt40,
fld5: i32,

},
Variant1{
fld0: u16,
fld1: Adt44,
fld2: isize,

},
Variant2{
fld0: *mut *mut i8,
fld1: f32,
fld2: Adt42,

},
Variant3{
fld0: Adt40,
fld1: u128,
fld2: Adt43,
fld3: [u8; 5],
fld4: [u32; 6],

}}
impl PrintFDebug for Adt50{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt50{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt50 {
fld0: Adt47,
fld1: Adt48,
fld2: (*mut *mut i8,),
}
impl PrintFDebug for Adt51{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt51{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt51 {
fld0: Adt40,
fld1: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8),
fld2: isize,
fld3: u128,
fld4: [i64; 1],
fld5: [i128; 7],
fld6: Adt41,
fld7: i128,
}
impl PrintFDebug for Adt52{
	unsafe fn printf_debug(&self){unsafe{printf("Adt52::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt52 {
Variant0{
fld0: u128,

},
Variant1{
fld0: bool,
fld1: Adt42,
fld2: Adt43,
fld3: [u32; 6],
fld4: [u8; 2],
fld5: [i128; 8],
fld6: Adt46,
fld7: i128,

}}
impl PrintFDebug for Adt53{
	unsafe fn printf_debug(&self){unsafe{printf("Adt53::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt53 {
Variant0{
fld0: *mut [u32; 6],
fld1: Adt41,
fld2: [i32; 3],

},
Variant1{
fld0: (*mut *mut i8,),
fld1: i32,
fld2: *mut i128,
fld3: ([char; 8], u16, *mut [i128; 7], i32, i128),
fld4: Adt44,

},
Variant2{
fld0: u64,

},
Variant3{
fld0: char,

}}
impl PrintFDebug for Adt54{
	unsafe fn printf_debug(&self){
	unsafe{printf("Adt54{ ".as_ptr()  as *const c_char)};
	unsafe{printf("}\0".as_ptr() as *const c_char)};}
}
#[derive(Copy,Clone)]pub struct Adt54 {
fld0: [u32; 6],
fld1: [i128; 8],
}
impl PrintFDebug for Adt55{
	unsafe fn printf_debug(&self){unsafe{printf("Adt55::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,fld4,fld5,fld6,fld7,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld7:\0".as_ptr() as *const c_char)};
		fld7.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant2{fld0,fld1,fld2,fld3,fld4,fld5,fld6,}=>{
unsafe{printf("Variant2{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld5:\0".as_ptr() as *const c_char)};
		fld5.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld6:\0".as_ptr() as *const c_char)};
		fld6.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant3{fld0,fld1,}=>{
unsafe{printf("Variant3{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt55 {
Variant0{
fld0: ([i128; 7], (*mut i8, i8, *mut i8, char, f32, *mut i128), i16, [u16; 2], u8),
fld1: [u32; 6],
fld2: [char; 8],

},
Variant1{
fld0: bool,
fld1: Adt45,
fld2: *mut i128,
fld3: u32,
fld4: u64,
fld5: Adt49,
fld6: Adt51,
fld7: *mut i8,

},
Variant2{
fld0: [u16; 2],
fld1: Adt47,
fld2: *mut [u32; 6],
fld3: f32,
fld4: (*mut *mut i8,),
fld5: Adt42,
fld6: [u8; 2],

},
Variant3{
fld0: *mut *mut i8,
fld1: Adt45,

}}
impl PrintFDebug for Adt56{
	unsafe fn printf_debug(&self){unsafe{printf("Adt56::\0".as_ptr()  as *const c_char)};match self{
	Self::Variant0{fld0,fld1,fld2,fld3,fld4,}=>{
unsafe{printf("Variant0{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld4:\0".as_ptr() as *const c_char)};
		fld4.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
	Self::Variant1{fld0,fld1,fld2,fld3,}=>{
unsafe{printf("Variant1{\0".as_ptr() as *const c_char)};
		unsafe{printf("fld0:\0".as_ptr() as *const c_char)};
		fld0.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld1:\0".as_ptr() as *const c_char)};
		fld1.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld2:\0".as_ptr() as *const c_char)};
		fld2.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
		unsafe{printf("fld3:\0".as_ptr() as *const c_char)};
		fld3.printf_debug();
unsafe{printf(",\0".as_ptr() as *const c_char)};
},
		}
unsafe{printf("\0}".as_ptr() as *const c_char)};
	}
}
#[derive(Copy,Clone)]pub enum Adt56 {
Variant0{
fld0: [char; 8],
fld1: Adt50,
fld2: u32,
fld3: Adt42,
fld4: *mut i8,

},
Variant1{
fld0: usize,
fld1: u8,
fld2: Adt46,
fld3: i8,

}}

