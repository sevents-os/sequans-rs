use atat::atat_derive::AtatEnum;

/// The CME error reporting methods.
#[derive(Clone, Debug, PartialEq, AtatEnum)]
#[at_enum(u8)]
pub enum CMEErrorReports {
    Off = 0,
    Numeric = 1,
    Verbose = 2,
}

/// The CEREG unsolicited reporting methods.
#[derive(Clone, Debug, PartialEq, AtatEnum)]
#[at_enum(u8)]
pub enum CEREGReports {
    Off = 0,
    Enabled = 1,
    EnabledWithLocation = 2,
    EnabledWithLocationEmmCause = 3,
    EnabledUePsmWithLocation = 4,
    EnabledUePsmWithLocationEmmCause = 5,
}
