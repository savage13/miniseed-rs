//! MiniSEED Library for rust
//!
//! This is an wrapper around the IRIS libmseed library at
//!   https://github.com/iris-edu/libmseed
//!
//! Currently, it can read miniseed records from a file or parse from
//! a in memory buffer. The underlying data and timing information can
//! be obtained from a ms_record.
//!
//! ```
//! use miniseed::ms_record;
//! let file = "tests/sample.miniseed";
//! let rec = ms_record::read(file);
//! assert_eq!(rec.to_string(), "PN_PPNAF_00_HHZ, 1, D, 512, 206 samples, 100 Hz, 2016-10-30 18:02:58.230 UTC");
//! ```
//!
//! ```
//! use miniseed::ms_record;
//! use std::fs::File;
//! use std::io::Read;
//!
//! let mut file = File::open("tests/sample.miniseed").unwrap();
//! let mut buf = vec![];
//! let _ = file.read_to_end(&mut buf).unwrap();
//!
//! let rec = ms_record::parse(&buf);
//! assert_eq!(rec.to_string(), "PN_PPNAF_00_HHZ, 1, D, 512, 206 samples, 100 Hz, 2016-10-30 18:02:58.230 UTC");
//! ```
//!
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate chrono;
extern crate libc;
extern crate num;

use chrono::Utc;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::DateTime;
use chrono::Duration;
//use chrono::Timelike;

use std::ffi::CString;
use std::path::Path;
use libc::c_char;

extern crate glob;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

unsafe impl Send for ms_record {}
unsafe impl Sync for ms_record {}

// unsafe impl Send for ms_group {}
// unsafe impl Sync for ms_group {}

// unsafe impl Send for ms_trace {}
// unsafe impl Sync for ms_trace {}

/// MiniSEED Record
#[derive(Debug)]
pub struct ms_record(*mut MSRecord);
// #[derive(Debug)]
// pub struct ms_group(*mut MSTraceGroup);
// #[derive(Debug)]
// pub struct ms_trace(*mut MSTrace);


// macro_rules! trace_loop {
//     ($name:ident, $tfunc:ident, $vfunc:ident, $t:ty) => {
//         pub fn $name(&self) -> Option<$t> {
//             let mut t = match self.trace() {
//                 None    => return None,
//                 Some(x) => Some(x),
//             };
//             let mut vals = vec![];
//             while let Some(tp) = t {
//                 vals.push( tp.$tfunc() );
//                 t = tp.next();
//             }
//             Some($vfunc(&vals))
//         }
//     };
// }

macro_rules! cast {
    ($x:ident, $t:ty) => { ($x as *mut _) as *mut $t };
    (ptr, $x:ident, $t:ty) => { ((&mut $x) as *mut _) as *mut *mut $t };
}

pub fn fmin<T: num::Float>(v: &[T]) -> T {
    let mut vf = v[0];
    for vi in v { if *vi < vf { vf = *vi; }  }
    vf
}
pub fn fmax<T: num::Float>(v: &[T]) -> T {
    let mut vf = v[0];
    for vi in v { if *vi > vf { vf = *vi; }  }
    vf
}
// fn tmin(v: &[DateTime<Utc>]) -> DateTime<Utc> {
//     let mut vf = v[0];
//     for vi in v { if *vi < vf { vf = *vi; }  }
//     vf
// }
// fn tmax(v: &[DateTime<Utc>]) -> DateTime<Utc> {
//     let mut vf = v[0];
//     for vi in v { if *vi > vf { vf = *vi; }  }
//     vf
// }
/// Convert DateTime<Utc> to seconds from epoch
pub fn utc_to_f64(t: &DateTime<Utc>) -> f64 {
    t.timestamp() as f64 + t.timestamp_subsec_nanos() as f64 / 1e9
}
/// Convert seconds from epoch to DateTime<Utc>
pub fn f64_to_utc(t: f64) -> DateTime<Utc> {
    let i = t.trunc() as i64;
    let f = (t.fract() * 1e9) as u32;
    let t = NaiveDateTime::from_timestamp(i,f);
    DateTime::<Utc>::from_utc(t,Utc)
}

// fn tmax_to_f64(v: &[DateTime<Utc>]) -> f64 {
//     utc_to_f64(&tmax(v))
// }
// fn tmin_to_f64(v: &[DateTime<Utc>]) -> f64 {
//     utc_to_f64(&tmin(v))
// }

// impl ms_trace {
//     fn ptr(&self) -> MSTrace {
//         unsafe {*self.0}
//     }
//     pub fn npts(&self) -> i64 {
//         self.ptr().numsamples
//     }
//     pub fn delta(&self) -> f64 {
//         1.0 / self.ptr().samprate
//     }
//     pub fn id(&self) -> String {
//         let m = self.ptr();
//         let net = i8_to_string(&m.network);
//         let sta = i8_to_string(&m.station);
//         let loc = i8_to_string(&m.location);
//         let cha = i8_to_string(&m.channel);
//         format!("{}_{}_{}_{}", net, sta, loc, cha)
//     }
//     pub fn into_f64(&self) -> i32 {
//         unsafe { mst_convertsamples(self.0, 'd' as i8, 1) }
//     }
//     pub fn data_type(&self) -> char {
//         self.ptr().sampletype as u8 as char
//     }
//     pub fn time_as_f64(&self) -> Vec<f64> {
//         self.time().iter().map(|x| utc_to_f64(x)).collect()
//     }
//     pub fn time(&self) -> Vec<DateTime<Utc>> {
//         let n = self.npts();
//         let b = self.start();
//         let dt = self.delta();
//         (0..n)
//             .map(|i| (i as f64) * dt * 1e6)
//             .map(|i| b + Duration::microseconds(i as i64))
//             .collect::<Vec<DateTime<Utc>>>()
//     }
//     pub fn data_as_f64(&self) -> Vec<f64> {
//         match self.data_type() {
//             'i' => self.data_i32().iter().map(|&x| x as f64).collect(),
//             'f' => self.data_f32().iter().map(|&x| x as f64).collect(),
//             'd' => self.data_f64().iter().map(|&x| x as f64).collect(),
//             'a' => panic!("attempt to take min of ascii data"),
//             _ => panic!("unknown data type: {}", self.data_type()),
//         }
//     }

//     pub fn data_f64(&self) -> &[f64] {
//         use std::slice::from_raw_parts_mut;
//         let n = self.npts() as usize;
//         let p = self.ptr().datasamples;
//         unsafe { from_raw_parts_mut(p as *mut f64, n) }
//     }
//     pub fn data_f32(&self) -> &[f32] {
//         use std::slice::from_raw_parts_mut;
//         let n = self.npts() as usize;
//         let p = self.ptr().datasamples;
//         unsafe { from_raw_parts_mut(p as *mut f32, n) }
//     }
//     pub fn data_i32(&self) -> &[i32] {
//         use std::slice::from_raw_parts_mut;
//         let n = self.npts() as usize;
//         let p = self.ptr().datasamples;
//         unsafe { from_raw_parts_mut(p as *mut i32, n) }
//     }
//     pub fn start(&self) -> DateTime<Utc> {
//         let b = self.ptr().starttime;
//         let mut t = BTime::zero();
//         unsafe { ms_hptime2btime(b, t.as_mut_ptr()) };
//         t.to_datetime()
//     }
//     pub fn end(&self) -> DateTime<Utc> {
//         let b = self.ptr().endtime;
//         let mut t = BTime::zero();
//         unsafe { ms_hptime2btime(b, t.as_mut_ptr()) };
//         t.to_datetime()
//     }
//     pub fn next(&self) -> Option<ms_trace> {
//         if self.is_null() {
//             return None;
//         }
//         let p = self.ptr().next;
//         if p.is_null() {
//             return None;
//         }
//         Some( ms_trace( p ) )
//     }
//     pub fn to_vec(self) -> Vec<ms_trace> {
//         let mut out = vec![];
//         let mut p = self.next();
//         out.push(self);
//         while let Some(pt) = p {
//             let next = pt.next();
//             out.push(pt);
//             p = next;
//         }
//         out
//     }
//     pub fn is_null(&self) -> bool {
//         self.0.is_null()
//     }
//     pub fn min(&self) -> f64 {
//         match self.data_type() {
//             'i' => *self.data_i32().iter().min().unwrap()  as f64,
//             'f' => fmin(self.data_f32()) as f64,
//             'd' => fmin(self.data_f64()) as f64,
//             'a' => panic!("attempt to take min of ascii data"),
//             _ => panic!("unknown data type: {}", self.data_type()),
//         }
//     }
//     pub fn max(&self) -> f64 {
//         match self.data_type() {
//             'i' => *self.data_i32().iter().max().unwrap()  as f64,
//             'f' => fmax(self.data_f32()) as f64,
//             'd' => fmax(self.data_f64()) as f64,
//             'a' => panic!("attempt to take min of ascii data"),
//             _ => panic!("unknown data type: {}", self.data_type()),
//         }
//     }
// }


// impl ms_group {
//     pub fn new() -> ms_group {
//         let null = std::ptr::null_mut() as *mut MSTraceGroup;
//         let mstg = unsafe { mst_initgroup( null ) };
//         ms_group( mstg )
//     }
//     pub fn add_record(&self, msr: ms_record) -> bool {
//         let timetol     = -1.0;
//         let sampratetol = -1.0;
//         let dataquality = 0;
//         let mst = unsafe { mst_addmsrtogroup(self.0, msr.0,
//                                              dataquality, timetol, sampratetol ) };
//         self.heal();

//         return ! mst.is_null();
//     }
//     pub fn from_ms_records(mrs: Vec<ms_record>) -> ms_group {
//         let g = ms_group::new();
//         for r in mrs {
//             g.add_record(r);
//         }
//         g
//     }
//     pub fn heal(&self) -> i32 {
//         let timetol     = -1.0;
//         let sampratetol = -1.0;
//         unsafe { mst_groupheal(self.0, timetol, sampratetol) }
//     }
//     fn ptr(&self) -> MSTraceGroup {
//         unsafe {*self.0}
//     }
//     pub fn numtraces(&self) -> i32 {
//         self.ptr().numtraces
//     }
//     pub fn trace(&self) -> Option<ms_trace> {
//         let ptr = self.ptr().traces;
//         if ptr.is_null() {
//             return None
//         }
//         Some( ms_trace( ptr ) )
//     }
//     pub fn to_vec(&self) -> Vec<ms_trace> {
//         match self.trace() {
//             None => vec![],
//             Some(x) => x.to_vec()
//         }
//     }
//     pub fn sort(&self) -> i32{
//         let quality = 0;
//         unsafe { mst_groupsort(self.0, quality) }
//     }
//     pub fn gaps(&self, id: &str) -> Option<Vec<[DateTime<Utc>;2]>>{
//         let avail = match self.available(id) {
//             Some(x) => x,
//             None => return None,
//         };
//         let mut gaps = vec![];
//         for i in 1 .. avail.len() {
//             let t = [ avail[i-1][1], avail[i][0] ];
//             gaps.push(t)
//         }
//         Some(gaps)
//     }
//     pub fn available(&self, id: &str) -> Option<Vec<[DateTime<Utc>;2]>> {
//         self.sort();
//         let mut t = match self.trace() {
//             Some(x) => Some(x),
//             None => return None,
//         };
//         let mut avail = vec![];
//         while let Some(tp) = t {
//             t = tp.next();
//             if tp.id() == id { // Check delta
//                 avail.push( [ tp.start(), tp.end() ]);
//             }
//         }
//         Some(avail)
//     }
//     pub fn gaps_in_window(&self, id:&str, window: &[DateTime<Utc>;2]) -> Option<Vec<[DateTime<Utc>;2]>> {
//         let avail = match self.available(id) {
//             Some(x) => x,
//             None => return None,
//         };
//         //println!("WINDOW: {:?}", window);
//         //println!("AVAIL: {:?}", avail);
//         let n = avail.len();
//         let mut gaps = vec![];
//         if window[0] < avail[0][0] {
//             let t = [ window[0], std::cmp::min(avail[0][0],window[1])];
//             //println!("PRE-GAP: {:?}", t);
//             gaps.push( t );
//         }
//         for i in 1..n {
//             let (b,e) = (avail[i-1][1], avail[i][0]);
//             if window[0] < e && window[1] > b { // Overlap
//                 let t = [std::cmp::max(b,window[0]), std::cmp::min(e,window[1])];
//                 //println!("MID-GAP: {:?}", t);
//                 gaps.push( t );
//             }
//         }
//         if window[1] > avail[n-1][1] {
//             let t = [ std::cmp::max(avail[n-1][1],window[0]), window[1]];
//             //println!("POS-GAP: {:?}", t);
//             gaps.push( t );
//         }
//         Some(gaps)
//     }
//     trace_loop!(max,  max, fmax, f64);
//     trace_loop!(min,  min, fmin, f64);
//     trace_loop!(tmin, start, tmin_to_f64, f64);
//     trace_loop!(tmax, end,   tmax_to_f64, f64);
// }

impl BTime {
    pub fn as_mut_ptr(&mut self) -> *mut BTime {
        self as *mut BTime
    }
    pub fn zero() ->  BTime {
        BTime { year:0,day:0,hour:0,min:0,sec:0,fract:0,unused:0}
    }
    pub fn to_datetime(&self) -> DateTime<Utc> {
        // Convert Year/DayOfYear and Hour/Minute/Second/MicroSecond to DateTime
        let d = NaiveDate::from_yo(self.year as i32, self.day as u32)
            .and_hms_micro(self.hour as u32, self.min as u32,
                           self.sec as u32, self.fract as u32 *100);
        // Convert to UTC DateTime
        DateTime::<Utc>::from_utc(d, Utc)
    }
}

pub enum Data<'a> {
    Int(&'a [i32]),
    Float(&'a [f32]),
    Double(&'a [f64]),
    Ascii(&'a [u8]),
}

impl <'a> Data<'a> {
    pub fn to_f64(&self) -> Vec<f64> {
        match self {
            &Data::Int(y) => y.iter().map(|&i| i as f64).collect(),
            &Data::Float(y) => y.iter().map(|&i| i as f64).collect(),
            &Data::Double(y) => y.iter().map(|&i| i).collect(),
            &Data::Ascii(_y) => vec![],
        }
    }
}


impl ms_record {
    /// Get pointer to wrapped MSRecord value
    pub fn ptr(&self) -> MSRecord {
        unsafe { *self.0 }
    }
    /// Create a null pointer as a MSRecord
    pub fn null() -> *mut MSRecord {
        let p = unsafe { msr_init(std::ptr::null_mut()) } as *mut MSRecord;
        p
    }
    /// Read a file and return a ms_record
    ///
    /// ```
    /// # use miniseed::ms_record;
    /// let file = "tests/sample.miniseed";
    /// let rec = ms_record::read(file);
    /// assert_eq!(rec.to_string(), "PN_PPNAF_00_HHZ, 1, D, 512, 206 samples, 100 Hz, 2016-10-30 18:02:58.230 UTC");
    /// ```
    pub fn read<S>(file: S) -> ms_record
        where S: AsRef<Path>
    {
        let sfile : String = file.as_ref().to_string_lossy().into_owned();
        let cfile = CString::new(sfile).unwrap();

        let verbose     : flag = 1;
        let dataflag    : flag = 1;
        let skipnotdata : flag = 1;
        let mut pmsr = ms_record::null();
        let mut pmsfp = std::ptr::null_mut() as *mut MSFileParam;
        let retcode = unsafe{
            // WTF: https://github.com/rust-lang/rust/issues/17417
            ms_readmsr_r ( ((&mut pmsfp) as *mut _) as *mut *mut MSFileParam,
                              ((&mut pmsr) as *mut _) as *mut *mut MSRecord,
                              cfile.as_ptr(),
                              0,
                              std::ptr::null_mut(), // fpos
                              std::ptr::null_mut(), // last
                              skipnotdata,
                              dataflag,
                              verbose)
        };
        if retcode != MS_NOERROR as i32 {
            println!("retcode: {}", retcode);
        }
        ms_record ( pmsr )

    }
    /// Return the MiniSEED Record FSDH Header,
    ///   this is typically used internally
    ///
    /// ```
    /// # use miniseed::ms_record;
    /// let file = "tests/sample.miniseed";
    /// let rec = ms_record::read(file);
    /// let hdr = rec.header();
    /// ```
    pub fn header(&self) -> fsdh_s {
        let m = self.ptr();
        unsafe { * (m.fsdh as *mut fsdh_s) as  fsdh_s }
    }
    /// Return the start time
    ///
    /// ```
    /// # use miniseed::ms_record;
    /// let file = "tests/sample.miniseed";
    /// let rec = ms_record::read(file);
    /// assert_eq!(rec.start().to_string(), "2016-10-30 18:02:58.230 UTC");
    /// ```
    pub fn start(&self) -> DateTime<Utc> {
        self.header().start_time.to_datetime()
    }
    /// Return the end time
    ///
    /// ```
    /// # use miniseed::ms_record;
    /// let file = "tests/sample.miniseed";
    /// let rec = ms_record::read(file);
    /// assert_eq!(rec.end().to_string(), "2016-10-30 18:03:00.279999 UTC");
    /// ```
    pub fn end(&self) -> DateTime<Utc> {
        let n = self.npts();
        let b = self.start();
        let dt = self.delta();
        b + Duration::microseconds( ( (n-1) as f64 * dt * 1e6) as i64 )
    }
    /// Return the time of the next sample beyond the record
    ///   assuming a constant sample rate
    ///
    /// ```
    /// # use miniseed::ms_record;
    /// let file = "tests/sample.miniseed";
    /// let rec = ms_record::read(file);
    /// assert_eq!(rec.end1().to_string(), "2016-10-30 18:03:00.290 UTC");
    /// ```
    pub fn end1(&self) -> DateTime<Utc> {
        let n = self.npts();
        let b = self.start();
        let dt = self.delta();
        b + Duration::microseconds( ( n as f64 * dt * 1e6) as i64 )
    }
    /// Return the sample rate
    ///
    /// ```
    /// # use miniseed::ms_record;
    /// let file = "tests/sample.miniseed";
    /// let rec = ms_record::read(file);
    /// assert_eq!(rec.delta(), 0.01);
    /// ```
    pub fn delta(&self) -> f64 {
        let m = self.ptr();
        1.0 / m.samprate
    }
    /// Return the data sample type
    ///
    /// - c - Character data
    /// - i - i32 data
    /// - f - f32 data
    /// - d - f64 data
    ///
    /// ```
    /// # use miniseed::ms_record;
    /// let file = "tests/sample.miniseed";
    /// let rec = ms_record::read(file);
    /// assert_eq!(rec.data_type(), 'i');
    /// ```
    pub fn data_type(&self) -> char {
        let m = self.ptr();
        m.sampletype as u8 as char
    }
    /// Return the data sample type
    ///
    /// see data_type()
    pub fn dtype(&self) -> char {
        let m = self.ptr();
        m.sampletype as u8 as char
    }
    /// Return the number of points or samples
    ///
    /// ```
    /// # use miniseed::ms_record;
    /// let file = "tests/sample.miniseed";
    /// let rec = ms_record::read(file);
    /// assert_eq!(rec.npts(), 206);
    /// ```
    pub fn npts(&self) -> usize {
        let m = self.ptr();
        m.numsamples as usize
    }
    /// Return the timing of each sample
    ///
    /// ```
    /// # use miniseed::ms_record;
    /// let file = "tests/sample.miniseed";
    /// let rec = ms_record::read(file);
    /// println!("{:?}", rec.time());
    /// ```
    pub fn time(&self) -> Vec<DateTime<Utc>> {
        let n = self.npts();
        let b = self.start();
        let dt = self.delta();
        (0..n)
            .map(|i| (i as f64) * dt * 1e6)
            .map(|i| b + Duration::microseconds(i as i64))
            .collect::<Vec<DateTime<Utc>>>()
    }

    fn data(&self) -> Option<Data> {
        use std::slice::from_raw_parts_mut;
        let m = self.ptr();
        let p = m.datasamples;
        let n = self.npts();
        let y = match self.dtype() {
            'i' => {
                let y : &[i32] = unsafe {from_raw_parts_mut(p as *mut i32, n) };
                Data::Int(y)
            },
            'f' => {
                let y : &[f32] = unsafe { from_raw_parts_mut(p as *mut f32, n) };
                Data::Float(y)
            },
            'd' => {
                let y : &[f64] = unsafe { from_raw_parts_mut(p as *mut f64, n) };
                Data::Double(y)
            },
            'a' => {
                let y : &[u8] = unsafe { from_raw_parts_mut(p as *mut u8, n) };
                Data::Ascii(y)
            }
            _ => {
                println!("Unknown data type: {}", self.dtype());
                return None;
            },
        };
        Some(y)
    }
    fn check_data_type(&self, want: char) {
        if self.dtype() != want {
            panic!("Incorrect data type: requested: '{}, current: '{}'",
                   want, self.dtype());
        }
    }

    /// Return the data as f64
    ///
    /// ```panic
    /// use miniseed::ms_record;
    /// let file = "tests/sample.miniseed";
    /// let rec = ms_record::read(file);
    /// assert_eq!(rec.data_f64()[0], 1.0);
    /// ```
    pub fn data_f64(&self) -> &[f64] {
        use std::slice::from_raw_parts_mut;
        self.check_data_type('d');
        let n = self.npts() as usize;
        let p = self.ptr().datasamples;
        unsafe { from_raw_parts_mut(p as *mut f64, n) }
    }
    /// Return the data as f32
    ///
    /// ```panic
    /// use miniseed::ms_record;
    /// let file = "tests/sample.miniseed";
    /// let rec = ms_record::read(file);
    /// assert_eq!(rec.data_f32()[0], 4.75878e-12);
    /// ```
    pub fn data_f32(&self) -> &[f32] {
        use std::slice::from_raw_parts_mut;
        self.check_data_type('f');
        let n = self.npts() as usize;
        let p = self.ptr().datasamples;
        unsafe { from_raw_parts_mut(p as *mut f32, n) }
    }
    /// Return the data as i32
    ///
    /// ```
    /// # use miniseed::ms_record;
    /// let file = "tests/sample.miniseed";
    /// let rec = ms_record::read(file);
    /// assert_eq!(rec.data_i32()[0], 339598);
    /// ```
    pub fn data_i32(&self) -> &[i32] {
        use std::slice::from_raw_parts_mut;
        self.check_data_type('i');
        let n = self.npts() as usize;
        let p = self.ptr().datasamples;
        unsafe { from_raw_parts_mut(p as *mut i32, n) }
    }
    /// Return the minimum data value
    ///
    /// ```
    /// # use miniseed::ms_record;
    /// let file = "tests/sample.miniseed";
    /// let rec = ms_record::read(file);
    /// assert_eq!(rec.min(), 333405.0);
    /// ```
    pub fn min(&self) -> f64 {
        match self.data_type() {
            'i' => *self.data_i32().iter().min().unwrap()  as f64,
            'f' => fmin(self.data_f32()) as f64,
            'd' => fmin(self.data_f64()) as f64,
            'a' => panic!("attempt to take min of ascii data"),
            _ => panic!("unknown data type: {}", self.data_type()),
        }
    }
    /// Return the maximum data value
    ///
    /// ```
    /// # use miniseed::ms_record;
    /// let file = "tests/sample.miniseed";
    /// let rec = ms_record::read(file);
    /// assert_eq!(rec.max(), 342105.0);
    /// ```
    pub fn max(&self) -> f64 {
        match self.data_type() {
            'i' => *self.data_i32().iter().max().unwrap()  as f64,
            'f' => fmax(self.data_f32()) as f64,
            'd' => fmax(self.data_f64()) as f64,
            'a' => panic!("attempt to take min of ascii data"),
            _ => panic!("unknown data type: {}", self.data_type()),
        }
    }
    /// Return the unique record identifier or ID
    ///
    /// ```
    /// # use miniseed::ms_record;
    /// let file = "tests/sample.miniseed";
    /// let rec = ms_record::read(file);
    /// assert_eq!(rec.id(), "PN_PPNAF_00_HHZ");
    /// ```
    pub fn id(&self) -> String {
        let m = self.ptr();
        let net = i8_to_string(&m.network);
        let sta = i8_to_string(&m.station);
        let loc = i8_to_string(&m.location);
        let cha = i8_to_string(&m.channel);
        format!("{}_{}_{}_{}", net, sta, loc, cha)
    }
    /// Parse a SeedLink data buffer and return a ms_record
    ///
    /// ```
    /// # use miniseed::ms_record;
    /// use std::fs::File;
    /// use std::io::Read;
    ///
    /// let mut file = File::open("tests/sample.miniseed").unwrap();
    /// let mut buf = vec![];
    /// let _ = file.read_to_end(&mut buf).unwrap();
    ///
    /// let rec = ms_record::parse(&buf);
    /// assert_eq!(rec.to_string(), "PN_PPNAF_00_HHZ, 1, D, 512, 206 samples, 100 Hz, 2016-10-30 18:02:58.230 UTC");
    /// ```
    pub fn parse(record: &[u8]) -> ms_record {
        let verbose  = 1;
        let data     = 1;

        // Copy Data
        let mut rec = record.to_vec();
        // Get Pointer to memory slice
        let prec = &mut rec[..];
        // Convert to char * (C Type)
        let c_rec = cast!(prec, c_char);

        let mut pmsr = ms_record::null();
        let ppmsr = cast!(ptr, pmsr, MSRecord);

        let ret = unsafe { msr_parse(c_rec, record.len() as i32, ppmsr, 0, data, verbose) };

        if ret != MS_NOERROR as i32 {
            println!("retcode: {}", ret);
        }
        ms_record( pmsr )
    }

    // Get Character data, if available
    pub fn as_string(&self) -> Option<String> {
        match self.data() {
            Some(Data::Ascii(x)) => Some(String::from_utf8(x.to_vec()).unwrap()),
            _ => None
        }
    }

}

fn i8_to_string(vin: &[i8]) -> String {
    let v : Vec<u8> = vin.iter()
        .map(|x| *x as u8)      // cast i8 as u8
        .filter(|x| *x != 0u8) // remove null terminators
        .collect();
    String::from_utf8(v).unwrap()  // convert to  string
}
use std::fmt;
impl fmt::Display for ms_record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = self.ptr();
        let s = self.id();
        write!(f, "{}, {}, {}, {}, {} samples, {} Hz, {}", s, v.sequence_number,
               v.dataquality as u8 as char, v.reclen, v.samplecnt,
               v.samprate, self.start())
    }
}
impl Drop for ms_record {
    fn drop(&mut self) {
        //println!("Dropping ms_record: {:?}!", self.ptr);
        unsafe {
            ms_readmsr (&mut (self.0 as *mut _),
                        std::ptr::null_mut(),
                        0,
                        std::ptr::null_mut(),
                        std::ptr::null_mut(),
                        0, 0, 0);
        }
    }
}

#[cfg(test)]
mod tests {

}

