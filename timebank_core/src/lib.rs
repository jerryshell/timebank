use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Record {
    #[serde(rename = "date")]
    pub date: NaiveDate,
    #[serde(rename = "timeIndexBegin")]
    pub time_index_begin: usize,
    #[serde(rename = "timeIndexEnd")]
    pub time_index_end: usize,
    #[serde(rename = "type")]
    pub type_str: String,
    #[serde(rename = "remark")]
    pub remark: String,
}

pub fn hhmm_to_time_index(hhmm: &str) -> Result<usize, String> {
    let hhmm_split: Vec<&str> = hhmm.split(':').collect();
    if hhmm_split.len() != 2 {
        return Err(format!("invalid(1) hhmm: {}", hhmm));
    }

    let Some(hh) = hhmm_split.first() else {
        return Err(format!("invalid(2) hhmm: {}", hhmm));
    };

    let Ok(hh_usize) = hh.parse::<usize>() else {
        return Err(format!("invalid(3) hhmm: {}", hhmm));
    };

    let mut time_index = hh_usize * 2;

    let Some(mm) = hhmm_split.last() else {
        return Err(format!("invalid(4) hhmm: {}", hhmm));
    };

    if *mm == "30" {
        time_index += 1;
    }

    Ok(time_index)
}

pub fn hhmmhhmm_to_time_index_range(hhmmhhmm: &str) -> Result<(usize, usize), String> {
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
