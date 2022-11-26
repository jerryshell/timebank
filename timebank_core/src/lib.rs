use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Record {
    #[serde(rename = "date")]
    pub date: String,
    #[serde(rename = "timeIndexBegin")]
    pub time_index_begin: u32,
    #[serde(rename = "timeIndexEnd")]
    pub time_index_end: u32,
    #[serde(rename = "type")]
    pub type_str: String,
    #[serde(rename = "remark")]
    pub remark: String,
}

pub fn hhmm_to_time_index(hhmm: &str) -> Result<u32, String> {
    let hhmm_split: Vec<&str> = hhmm.split(':').collect();
    if hhmm_split.len() != 2 {
        return Err(format!("invalid(1) hhmm: {}", hhmm));
    }

    let Some(hh) = hhmm_split.first() else {
        return Err(format!("invalid(2) hhmm: {}", hhmm));
    };

    let Ok(hh_u32) = hh.parse::<u32>() else {
        return Err(format!("invalid(3) hhmm: {}", hhmm));
    };

    let mut time_index = hh_u32 * 2;

    let Some(mm) = hhmm_split.last() else {
        return Err(format!("invalid(4) hhmm: {}", hhmm));
    };

    if *mm == "30" {
        time_index += 1;
    }

    Ok(time_index)
}

pub fn hhmmhhmm_to_time_index_range(hhmmhhmm: &str) -> Result<(u32, u32), String> {
    let hhmmhhmm_split: Vec<&str> = hhmmhhmm.split('-').collect();
    if hhmmhhmm_split.len() != 2 {
        return Err(format!("invalid(1) hhmmhhmm: {}", hhmmhhmm));
    }

    let Some(hhmm1) = hhmmhhmm_split.first() else {
        return Err(format!("invalid(2) hhmmhhmm: {}", hhmmhhmm));
    };
    // println!("hhmm1 {:#?}", hhmm1);

    let Ok(hhmm1_time_index) = hhmm_to_time_index(hhmm1) else {
        return Err(format!("invalid(3) hhmmhhmm: {}", hhmmhhmm));
    };
    // println!("hhmm1_time_index {:#?}", hhmm1_time_index);

    let Some(hhmm2) = hhmmhhmm_split.last() else {
        return Err(format!("invalid(4) hhmmhhmm: {}", hhmmhhmm));
    };
    // println!("hhmm2 {:#?}", hhmm2);

    let Ok(hhmm2_time_index) = hhmm_to_time_index(hhmm2) else {
        return Err(format!("invalid(5) hhmmhhmm: {}", hhmmhhmm));
    };
    // println!("hhmm2_time_index {:#?}", hhmm2_time_index);

    Ok((hhmm1_time_index, hhmm2_time_index))
}
