#[cfg_attr(feature = "mcxa132", path = "devices/mcxa132/device.rs")]
#[cfg_attr(feature = "mcxa133", path = "devices/mcxa133/device.rs")]
#[cfg_attr(feature = "mcxa142", path = "devices/mcxa142/device.rs")]
#[cfg_attr(feature = "mcxa143", path = "devices/mcxa143/device.rs")]
#[cfg_attr(feature = "mcxa152", path = "devices/mcxa152/device.rs")]
#[cfg_attr(feature = "mcxa153", path = "devices/mcxa153/device.rs")]
#[cfg_attr(feature = "mcxa144", path = "devices/mcxa144/device.rs")]
#[cfg_attr(feature = "mcxa145", path = "devices/mcxa145/device.rs")]
#[cfg_attr(feature = "mcxa146", path = "devices/mcxa146/device.rs")]
#[cfg_attr(feature = "mcxa154", path = "devices/mcxa154/device.rs")]
#[cfg_attr(feature = "mcxa155", path = "devices/mcxa155/device.rs")]
#[cfg_attr(feature = "mcxa156", path = "devices/mcxa156/device.rs")]
#[cfg_attr(feature = "mcxa165", path = "devices/mcxa165/device.rs")]
#[cfg_attr(feature = "mcxa166", path = "devices/mcxa166/device.rs")]
#[cfg_attr(feature = "mcxa175", path = "devices/mcxa175/device.rs")]
#[cfg_attr(feature = "mcxa176", path = "devices/mcxa176/device.rs")]
#[cfg_attr(feature = "mcxa255", path = "devices/mcxa255/device.rs")]
#[cfg_attr(feature = "mcxa256", path = "devices/mcxa256/device.rs")]
#[cfg_attr(feature = "mcxa275", path = "devices/mcxa275/device.rs")]
#[cfg_attr(feature = "mcxa276", path = "devices/mcxa276/device.rs")]
#[cfg_attr(
    feature = "mcxn546_cm33_core0",
    path = "devices/mcxn546_cm33_core0/device.rs"
)]
#[cfg_attr(
    feature = "mcxn546_cm33_core1",
    path = "devices/mcxn546_cm33_core1/device.rs"
)]
#[cfg_attr(
    feature = "mcxn547_cm33_core0",
    path = "devices/mcxn547_cm33_core0/device.rs"
)]
#[cfg_attr(
    feature = "mcxn547_cm33_core1",
    path = "devices/mcxn547_cm33_core1/device.rs"
)]
#[cfg_attr(
    feature = "mcxn946_cm33_core0",
    path = "devices/mcxn946_cm33_core0/device.rs"
)]
#[cfg_attr(
    feature = "mcxn946_cm33_core1",
    path = "devices/mcxn946_cm33_core1/device.rs"
)]
#[cfg_attr(
    feature = "mcxn947_cm33_core0",
    path = "devices/mcxn947_cm33_core0/device.rs"
)]
#[cfg_attr(
    feature = "mcxn947_cm33_core1",
    path = "devices/mcxn947_cm33_core1/device.rs"
)]
#[cfg_attr(feature = "mcxn235", path = "devices/mcxn235/device.rs")]
#[cfg_attr(feature = "mcxn236", path = "devices/mcxn236/device.rs")]
#[cfg(feature = "_device_selected")]
mod _device;
#[cfg(feature = "_device_selected")]
pub use _device::*;
