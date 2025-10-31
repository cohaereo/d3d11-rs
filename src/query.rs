use crate::{impl_device_child, verify_ffi_struct};
use d3d11_ffi::Direct3D11::*;

pub use d3d11_ffi::Direct3D11::D3D11_QUERY_DATA_TIMESTAMP_DISJOINT;

pub trait Asynchronous: Sized {
    fn to_ffi_async(&self) -> ID3D11Asynchronous;
    fn from_ffi_async(resource: ID3D11Asynchronous) -> Option<Self>;
}

#[macro_export]
macro_rules! impl_asynchronous {
    ($name:ident) => {
        impl_device_child!($name);
        impl $crate::query::Asynchronous for $name {
            fn to_ffi_async(&self) -> ID3D11Asynchronous {
                self.0.clone().into()
            }

            fn from_ffi_async(resource: ID3D11Asynchronous) -> Option<Self> {
                use d3d11_ffi::core::Interface;
                resource.cast().ok().map(Self)
            }
        }
    };
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Query(pub(crate) ID3D11Query);
impl_asynchronous!(Query);

#[repr(transparent)]
#[derive(Clone)]
pub struct Predicate(pub(crate) ID3D11Predicate);
impl_asynchronous!(Predicate);

#[repr(transparent)]
#[derive(Clone)]
pub struct Counter(pub(crate) ID3D11Counter);
impl_asynchronous!(Counter);

#[repr(C)]
#[derive(Clone, Debug)]
pub struct QueryDesc {
    pub query: QueryType,
    pub misc_flags: u32, // TODO: Bitflags
}
verify_ffi_struct!(QueryDesc, D3D11_QUERY_DESC);

impl QueryDesc {
    pub fn new(query: QueryType, misc_flags: u32) -> Self {
        Self { query, misc_flags }
    }

    pub fn event() -> Self {
        Self::new(QueryType::Event, 0)
    }

    pub fn timestamp() -> Self {
        Self::new(QueryType::Timestamp, 0)
    }

    pub fn timestamp_disjoint() -> Self {
        Self::new(QueryType::TimestampDisjoint, 0)
    }

    pub fn occlusion() -> Self {
        Self::new(QueryType::Occlusion, 0)
    }

    /// # Parameters
    /// - `hint`: If true, tells the hardware that if it is not yet sure if something is hidden or not to draw it anyway. Note that predication data cannot be returned to your application via [`DeviceContext::get_data`] when using this flag.
    pub fn occlusion_predicate(hint: bool) -> Self {
        Self::new(
            QueryType::OcclusionPredicate,
            if hint {
                D3D11_QUERY_MISC_PREDICATEHINT.0 as u32
            } else {
                0
            },
        )
    }
}

#[repr(i32)]
#[derive(Clone, Debug)]
pub enum QueryType {
    Event = D3D11_QUERY_EVENT.0,
    Occlusion = D3D11_QUERY_OCCLUSION.0,
    OcclusionPredicate = D3D11_QUERY_OCCLUSION_PREDICATE.0,
    PipelineStatistics = D3D11_QUERY_PIPELINE_STATISTICS.0,
    SoOverflowPredicate = D3D11_QUERY_SO_OVERFLOW_PREDICATE.0,
    SoOverflowPredicateStream0 = D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM0.0,
    SoOverflowPredicateStream1 = D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM1.0,
    SoOverflowPredicateStream2 = D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM2.0,
    SoOverflowPredicateStream3 = D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM3.0,
    SoStatistics = D3D11_QUERY_SO_STATISTICS.0,
    SoStatisticsStream0 = D3D11_QUERY_SO_STATISTICS_STREAM0.0,
    SoStatisticsStream1 = D3D11_QUERY_SO_STATISTICS_STREAM1.0,
    SoStatisticsStream2 = D3D11_QUERY_SO_STATISTICS_STREAM2.0,
    SoStatisticsStream3 = D3D11_QUERY_SO_STATISTICS_STREAM3.0,
    Timestamp = D3D11_QUERY_TIMESTAMP.0,
    TimestampDisjoint = D3D11_QUERY_TIMESTAMP_DISJOINT.0,
}
