pub mod convert;
pub mod json;
pub mod keys;
pub mod psl;
pub mod singbox;

pub fn decrypt_link(_s: &str) -> Result<String, String> { Err("stub".into()) }
pub fn encrypt_happ(_s: &str, _m: HappMode) -> Result<String, String> { Err("stub".into()) }
pub fn encrypt_v2(_s: &str, _k: V2Key) -> Result<String, String> { Err("stub".into()) }
pub fn encrypt_crypt5_legacy(_s: &str) -> Result<String, String> { Err("stub".into()) }
pub fn inspect(_s: &str) -> Result<String, String> { Err("stub".into()) }
pub fn unwrap_link(_s: &str) -> Result<String, String> { Err("stub".into()) }
pub fn convert(_ops: ConvertOps) -> Result<String, String> { Err("stub".into()) }

#[derive(Clone, Copy)]
pub enum HappMode { Crypt5, Crypt4, Crypt3, Crypt2, Crypt }
#[derive(Clone, Copy)]
pub enum V2Key { Crypt3, Crypt4, Key3 }
#[derive(Clone, Copy)]
pub enum LinkKind { Happ, V2RayTun }
pub struct ConvertOps;
