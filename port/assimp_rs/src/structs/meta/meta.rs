extern crate libc;
use crate::env::INT_MAX;
use libc::{c_void};

#[derive(Clone, Debug, Copy)]
enum MetadataType {
    AI_BOOL       = 0,
    AI_INT32      = 1,
    AI_UINT64     = 2,
    AI_FLOAT      = 3,
    AI_DOUBLE     = 4,
    AI_AISTRING   = 5,
    AI_AIVECTOR3D = 6,
    AI_META_MAX   = 7
    // Need to define `macro_rules!` for the swig
    // FORCE_32BIT = INT_MAX
}

#[derive(Clone, Debug, Copy)]
struct MetadataEntry {
    m_type: MetadataType,
    m_data: *mut c_void
}

use crate::structs::Str;

#[derive(Clone, Debug, Copy)]
struct Metadata {
    m_num_properties: usize,
    m_keys: &Str,
    m_values: &MetadataEntry
}


