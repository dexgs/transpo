pub const PORT: usize = 8080;

// restrict the maximum time a file will live
pub const MAX_DAYS: u32 = 6;
pub const MAX_HOURS: u32 = 23;
pub const MAX_MINUTES: u32 = 59;

// 500MB
pub const MAX_UPLOAD_SIZE: usize = 500000000;

// 5GB
pub const MAX_STORAGE_CAPACITY: usize = 5000000000;

pub const MAX_DOWNLOAD_LIMIT: u32 = 999;

// path at which data is saved. is relative to the dir from
// which transpo is run by default, but can be made absolute
pub const STORAGE_PATH: &'static str = "./storage";

// length of random file names. set to a length where you will
// not run out of names. names are random alphanumeric ASCII, 
// so number of possible names is 62 raised to NAME_LENGTH
pub const NAME_LENGTH: usize = 20;
