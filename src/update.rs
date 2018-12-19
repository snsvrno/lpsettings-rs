/// useful functions for updating?

use failure::Error;
use chrono;

use get_value;
use set_value;
use Type;

pub fn check_if_should_update(app_name : &str) -> bool {

    // getting the update frequency, defaults to 1 day but can be overwritten
    // in the config / settings
    let mut frequency : i64 = 1;
    if let Ok(Some(result)) = get_value(&format!("{}.{}",app_name,"update.freq")) {
        match result {
            Type::Int(int) => { frequency = int as i64; },
            Type::Float(float) => { frequency = float as i64; },
            _ => { }
        }
    }

    let now = chrono::Utc::now();

    // checks if should update based on the frequency
    if let Ok(Some(last_check)) = get_value(&format!("{}.{}",app_name,"update.last_check")) {
        if let Type::Text(text_date) = last_check {
            match chrono::DateTime::parse_from_rfc3339(&text_date){
                Err(error) => { error!("{}",error); return false; },
                Ok(date) => {
                    if now.signed_duration_since(date).num_days() < frequency { return false; }
                },
            }
        }
    }

    true
}

pub fn set_last_update_as_now(app_name : &str) -> Result<(),Error> {
    let now = chrono::Utc::now();
    match set_value(&format!("{}.{}",app_name,"update.last_check"),&now.to_rfc3339()) {
        Err(error) => Err(error),
        Ok(_) => Ok(())
    }
}