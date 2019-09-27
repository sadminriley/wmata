use std::error;
use std::fmt;

pub mod responses;
pub mod tests;

const NEXT_TRAINS: &'static str = "https://api.wmata.com/StationPrediction.svc/json/GetPrediction";
const INFORMATION: &'static str = "https://api.wmata.com/Rail.svc/json/jStationInfo";
const PARKING_INFORMATION: &'static str = "https://api.wmata.com/Rail.svc/json/jStationParking";
const PATH: &'static str = "https://api.wmata.com/Rail.svc/json/jPath";
const TIMINGS: &'static str = "https://api.wmata.com/Rail.svc/json/jStationTimes";
pub const STATION_TO_STATION: &'static str =
    "https://api.wmata.com/Rail.svc/json/jSrcStationToDstStationInfo";

pub struct Station<'a> {
    pub api_key: &'a str,
    pub station_code: StationCode,
}

pub enum StationCode {
    A01,
    A02,
    A03,
    A04,
    A05,
    A06,
    A07,
    A08,
    A09,
    A10,
    A11,
    A12,
    A13,
    A14,
    A15,
    B01,
    B02,
    B03,
    B04,
    B05,
    B06,
    B07,
    B08,
    B09,
    B10,
    B11,
    B35,
    C01,
    C02,
    C03,
    C04,
    C05,
    C06,
    C07,
    C08,
    C09,
    C10,
    C12,
    C13,
    C14,
    C15,
    D01,
    D02,
    D03,
    D04,
    D05,
    D06,
    D07,
    D08,
    D09,
    D10,
    D11,
    D12,
    D13,
    E01,
    E02,
    E03,
    E04,
    E05,
    E06,
    E07,
    E08,
    E09,
    E10,
    F01,
    F02,
    F03,
    F04,
    F05,
    F06,
    F07,
    F08,
    F09,
    F10,
    F11,
    G01,
    G02,
    G03,
    G04,
    G05,
    J02,
    J03,
    K01,
    K02,
    K03,
    K04,
    K05,
    K06,
    K07,
    K08,
    N01,
    N02,
    N03,
    N04,
    N06,
}

impl StationCode {
    pub fn to_string<'a>(&self) -> &'a str {
        match &self {
            StationCode::A01 => "A01",
            StationCode::A02 => "A02",
            StationCode::A03 => "A03",
            StationCode::A04 => "A04",
            StationCode::A05 => "A05",
            StationCode::A06 => "A06",
            StationCode::A07 => "A07",
            StationCode::A08 => "A08",
            StationCode::A09 => "A09",
            StationCode::A10 => "A10",
            StationCode::A11 => "A11",
            StationCode::A12 => "A12",
            StationCode::A13 => "A13",
            StationCode::A14 => "A14",
            StationCode::A15 => "A15",
            StationCode::B01 => "B01",
            StationCode::B02 => "B02",
            StationCode::B03 => "B03",
            StationCode::B04 => "B04",
            StationCode::B05 => "B05",
            StationCode::B06 => "B06",
            StationCode::B07 => "B07",
            StationCode::B08 => "B08",
            StationCode::B09 => "B09",
            StationCode::B10 => "B10",
            StationCode::B11 => "B11",
            StationCode::B35 => "B35",
            StationCode::C01 => "C01",
            StationCode::C02 => "C02",
            StationCode::C03 => "C03",
            StationCode::C04 => "C04",
            StationCode::C05 => "C05",
            StationCode::C06 => "C06",
            StationCode::C07 => "C07",
            StationCode::C08 => "C08",
            StationCode::C09 => "C09",
            StationCode::C10 => "C10",
            StationCode::C12 => "C12",
            StationCode::C13 => "C13",
            StationCode::C14 => "C14",
            StationCode::C15 => "C15",
            StationCode::D01 => "D01",
            StationCode::D02 => "D02",
            StationCode::D03 => "D03",
            StationCode::D04 => "D04",
            StationCode::D05 => "D05",
            StationCode::D06 => "D06",
            StationCode::D07 => "D07",
            StationCode::D08 => "D08",
            StationCode::D09 => "D09",
            StationCode::D10 => "D10",
            StationCode::D11 => "D11",
            StationCode::D12 => "D12",
            StationCode::D13 => "D13",
            StationCode::E01 => "E01",
            StationCode::E02 => "E02",
            StationCode::E03 => "E03",
            StationCode::E04 => "E04",
            StationCode::E05 => "E05",
            StationCode::E06 => "E06",
            StationCode::E07 => "E07",
            StationCode::E08 => "E08",
            StationCode::E09 => "E09",
            StationCode::E10 => "E10",
            StationCode::F01 => "F01",
            StationCode::F02 => "F02",
            StationCode::F03 => "F03",
            StationCode::F04 => "F04",
            StationCode::F05 => "F05",
            StationCode::F06 => "F06",
            StationCode::F07 => "F07",
            StationCode::F08 => "F08",
            StationCode::F09 => "F09",
            StationCode::F10 => "F10",
            StationCode::F11 => "F11",
            StationCode::G01 => "G01",
            StationCode::G02 => "G02",
            StationCode::G03 => "G03",
            StationCode::G04 => "G04",
            StationCode::G05 => "G05",
            StationCode::J02 => "J02",
            StationCode::J03 => "J03",
            StationCode::K01 => "K01",
            StationCode::K02 => "K02",
            StationCode::K03 => "K03",
            StationCode::K04 => "K04",
            StationCode::K05 => "K05",
            StationCode::K06 => "K06",
            StationCode::K07 => "K07",
            StationCode::K08 => "K08",
            StationCode::N01 => "N01",
            StationCode::N02 => "N02",
            StationCode::N03 => "N03",
            StationCode::N04 => "N04",
            StationCode::N06 => "N06",
        }
    }

    pub fn from(string: &str) -> Result<StationCode, StringIsNotStationCodeError> {
        match string {
            "A01" => Ok(StationCode::A01),
            "A02" => Ok(StationCode::A02),
            "A03" => Ok(StationCode::A03),
            "A04" => Ok(StationCode::A04),
            "A05" => Ok(StationCode::A05),
            "A06" => Ok(StationCode::A06),
            "A07" => Ok(StationCode::A07),
            "A08" => Ok(StationCode::A08),
            "A09" => Ok(StationCode::A09),
            "A10" => Ok(StationCode::A10),
            "A11" => Ok(StationCode::A11),
            "A12" => Ok(StationCode::A12),
            "A13" => Ok(StationCode::A13),
            "A14" => Ok(StationCode::A14),
            "A15" => Ok(StationCode::A15),
            "B01" => Ok(StationCode::B01),
            "B02" => Ok(StationCode::B02),
            "B03" => Ok(StationCode::B03),
            "B04" => Ok(StationCode::B04),
            "B05" => Ok(StationCode::B05),
            "B06" => Ok(StationCode::B06),
            "B07" => Ok(StationCode::B07),
            "B08" => Ok(StationCode::B08),
            "B09" => Ok(StationCode::B09),
            "B10" => Ok(StationCode::B10),
            "B11" => Ok(StationCode::B11),
            "B35" => Ok(StationCode::B35),
            "C01" => Ok(StationCode::C01),
            "C02" => Ok(StationCode::C02),
            "C03" => Ok(StationCode::C03),
            "C04" => Ok(StationCode::C04),
            "C05" => Ok(StationCode::C05),
            "C06" => Ok(StationCode::C06),
            "C07" => Ok(StationCode::C07),
            "C08" => Ok(StationCode::C08),
            "C09" => Ok(StationCode::C09),
            "C10" => Ok(StationCode::C10),
            "C12" => Ok(StationCode::C12),
            "C13" => Ok(StationCode::C13),
            "C14" => Ok(StationCode::C14),
            "C15" => Ok(StationCode::C15),
            "D01" => Ok(StationCode::D01),
            "D02" => Ok(StationCode::D02),
            "D03" => Ok(StationCode::D03),
            "D04" => Ok(StationCode::D04),
            "D05" => Ok(StationCode::D05),
            "D06" => Ok(StationCode::D06),
            "D07" => Ok(StationCode::D07),
            "D08" => Ok(StationCode::D08),
            "D09" => Ok(StationCode::D09),
            "D10" => Ok(StationCode::D10),
            "D11" => Ok(StationCode::D11),
            "D12" => Ok(StationCode::D12),
            "D13" => Ok(StationCode::D13),
            "E01" => Ok(StationCode::E01),
            "E02" => Ok(StationCode::E02),
            "E03" => Ok(StationCode::E03),
            "E04" => Ok(StationCode::E04),
            "E05" => Ok(StationCode::E05),
            "E06" => Ok(StationCode::E06),
            "E07" => Ok(StationCode::E07),
            "E08" => Ok(StationCode::E08),
            "E09" => Ok(StationCode::E09),
            "E10" => Ok(StationCode::E10),
            "F01" => Ok(StationCode::F01),
            "F02" => Ok(StationCode::F02),
            "F03" => Ok(StationCode::F03),
            "F04" => Ok(StationCode::F04),
            "F05" => Ok(StationCode::F05),
            "F06" => Ok(StationCode::F06),
            "F07" => Ok(StationCode::F07),
            "F08" => Ok(StationCode::F08),
            "F09" => Ok(StationCode::F09),
            "F10" => Ok(StationCode::F10),
            "F11" => Ok(StationCode::F11),
            "G01" => Ok(StationCode::G01),
            "G02" => Ok(StationCode::G02),
            "G03" => Ok(StationCode::G03),
            "G04" => Ok(StationCode::G04),
            "G05" => Ok(StationCode::G05),
            "J02" => Ok(StationCode::J02),
            "J03" => Ok(StationCode::J03),
            "K01" => Ok(StationCode::K01),
            "K02" => Ok(StationCode::K02),
            "K03" => Ok(StationCode::K03),
            "K04" => Ok(StationCode::K04),
            "K05" => Ok(StationCode::K05),
            "K06" => Ok(StationCode::K06),
            "K07" => Ok(StationCode::K07),
            "K08" => Ok(StationCode::K08),
            "N01" => Ok(StationCode::N01),
            "N02" => Ok(StationCode::N02),
            "N03" => Ok(StationCode::N03),
            "N04" => Ok(StationCode::N04),
            "N06" => Ok(StationCode::N06),
            _ => Err(StringIsNotStationCodeError),
        }
    }
}

pub trait ToStationCode {
    fn to_station_code(&self) -> Result<StationCode, StringIsNotStationCodeError>;
}

impl ToStationCode for &str {
    fn to_station_code(&self) -> Result<StationCode, StringIsNotStationCodeError> {
        StationCode::from(&self)
    }
}

#[derive(Debug, Clone)]
pub struct StringIsNotStationCodeError;

impl fmt::Display for StringIsNotStationCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Provided string is not a valid station code.")
    }
}

impl error::Error for StringIsNotStationCodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
