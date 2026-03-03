mod sv39;
mod sv48;

#[cfg(feature = "sv39")]
pub type SvPageConfig = sv39::Sv39Config;

#[cfg(feature = "sv48")]
pub type SvPageConfig = sv48::Sv48Config;

#[cfg(not(any(feature = "sv39", feature = "sv48")))]
compile_error!("Must enable either 'sv39' or 'sv48' feature");
