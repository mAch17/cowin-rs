extern crate ureq;
extern crate serde_json;

const SERVER: &str = "https://cdn-api.co-vin.in/api";
const GET_STATES_ENDPOINT: &str = "/v2/admin/location/states";
const GET_DISTRICTS_ENDPOINT: &str ="/v2/admin/location/districts";
const GET_SLOTS_BY_PIN_ENDPOINT: &str = "/v2/appointment/sessions/public/findByPin";
const GET_SLOTS_BY_DISTRICT_ENDPOINT: &str ="/v2/appointment/sessions/public/findByDistrict";
const GET_1WEEK_SLOTS_BY_PIN_ENDPOINT: &str = "/v2/appointment/sessions/public/calendarByPin";
const GET_1WEEK_SLOTS_BY_DISTRICT_ENDPOINT: &str = "/v2/appointment/sessions/public/calendarByDistrict";

pub enum RequestOutput{
	Works(serde_json::Value),
	ReadError(String),
	NetworkError(String)
}

pub fn get_india_states() -> RequestOutput {
	let states_api = [SERVER , GET_STATES_ENDPOINT].join("");

	match ureq::get(&states_api[..]).call() {
		Ok(response) => 
		match response.into_json() {
			Ok(json_response) => RequestOutput::Works(json_response),
			Err(e) => RequestOutput::ReadError(e.to_string()),
		},
		Err(e) => RequestOutput::NetworkError(e.to_string()) ,
	}
}

pub fn get_state_districts(state_id: String) -> RequestOutput {
    let districts_api = [SERVER,GET_DISTRICTS_ENDPOINT,"/"].join("");
    let mut url = districts_api.to_string();
    url += &state_id;
    match ureq::get(&url).call() {
    	Ok(response) => match response.into_json() {
    		Ok(json_response) => RequestOutput::Works(json_response),
    		Err(e) => RequestOutput::ReadError(e.to_string()),
    	}
    	Err(e) => RequestOutput::NetworkError(e.to_string())
    }

}

pub fn district_vaccination_schedule(district_id : String, dateinformat: String) -> RequestOutput {
	let district_vaccination_schedule_api = [SERVER, GET_1WEEK_SLOTS_BY_DISTRICT_ENDPOINT].join("");
	match ureq::get(&district_vaccination_schedule_api).set("Accept-Language", "hi_IN").set("accept", "application/json").query("district_id",&district_id).query("date",&dateinformat).call() {
	    	Ok(response) => match response.into_json() {
    		Ok(json_response) => RequestOutput::Works(json_response),
    		Err(e) => RequestOutput::ReadError(e.to_string()),
    	}
    	Err(e) => RequestOutput::NetworkError(e.to_string())	
	}
}

pub fn pincode_vaccination_schedule(pincode:String,dateinformat:String) -> RequestOutput {
	let pincode_vaccination_schedule_api = [SERVER,GET_1WEEK_SLOTS_BY_PIN_ENDPOINT].join("");
	match ureq::get(&pincode_vaccination_schedule_api).set("Accept-Language", "hi_IN").set("accept", "application/json").query("pincode",&pincode).query("date",&dateinformat).call() {
			Ok(response) => match response.into_json() {
    		Ok(json_response) => RequestOutput::Works(json_response),
    		Err(e) => RequestOutput::ReadError(e.to_string()),
    	}
    	Err(e) => RequestOutput::NetworkError(e.to_string())	
    }
}


pub fn get_required_state_data(json:serde_json::Value , state:String) -> std::result::Result<String,String>{
	if let serde_json::Value::Object(state_data) = json {
		if let Some(serde_json::Value::Array(state_data_array)) = state_data.get("states"){
			for state_data in state_data_array{
				if let serde_json::Value::Object(state_map) = state_data{
					if let Some(serde_json::Value::Number(state_id)) = state_map.get("state_id"){
						if let Some(serde_json::Value::String(state_name)) = state_map.get("state_name"){
							if state.eq(state_name){
								// Got state being mentioned
								println!("{:?} is state id {:?}",state,state_id );
								return Ok(state_id.to_string());
							} else {
								continue;
							}
						}
					} else {
						println!("No state_id seen, it seems REST API format might have changed, please check");
						continue;
					}
				} else {
					println!("No state objects seen, it seems REST API format might have changed, please check");
					continue;
				}
			}
			return Err(format!("{:?} not seen in COWIN APIs, available states {:?}",state,state_data_array ));
		}
		else {
			Err(format!("Cannot parse array {:?}",state_data))
		}
	} else {
		Err(String::from("Cannot Parse State Data"))
	}	
}

pub fn get_required_district_data(json:serde_json::Value , district:String)  -> std::result::Result<String,String>{
	if let serde_json::Value::Object(district_data) = json {
		if let Some(serde_json::Value::Array(district_data_array)) = district_data.get("districts"){
			for district_data in district_data_array{
				if let serde_json::Value::Object(district_map) = district_data{
					if let Some(serde_json::Value::Number(district_id)) = district_map.get("district_id"){
						if let Some(serde_json::Value::String(district_name)) = district_map.get("district_name"){
							if district.eq(district_name){
								// Got state being mentioned
								println!("{:?} is district id {:?}",district,district_id );
								return Ok(district_id.to_string());
							} else {
								continue;
							}
						}
					} else {
						println!("No district_id seen, it seems REST API format might have changed, please check");
						continue;
					}
				} else {
					println!("No district objects seen, it seems REST API format might have changed, please check");
					continue;
				}
			}
			return Err(format!("{:?} not seen in COWIN APIs, available districts {:?}",district,district_data_array ));
		}
		else {
			Err(format!("Cannot parse array {:?}",district_data))
		}
	} else {
		Err(String::from("Cannot Parse districts Data"))
	}		
}

pub fn get_data_for_state_district(state:String, district:String, dateinformat:String) -> std::result::Result<serde_json::Value,String> {
	let all_state_data = get_india_states();
	if let RequestOutput::Works(json_output) = all_state_data{
		let state_id_resp = get_required_state_data(json_output,state);
		if let Ok(state_id_str) = state_id_resp{
			let state_districts_data = get_state_districts(state_id_str);
			if let RequestOutput::Works(districts_json) = state_districts_data{
				let district_id_maybe = get_required_district_data(districts_json,district);
				if let Ok(district_id_str)  = district_id_maybe {
					let district_schedule_maybe = district_vaccination_schedule(district_id_str,dateinformat); 
					if let RequestOutput::Works(district_schedule_json) = district_schedule_maybe{
						return Ok(district_schedule_json);
					}
				else if let RequestOutput::ReadError(e) = district_schedule_maybe{
					return Err(e);
				} else if let RequestOutput::NetworkError(e) = district_schedule_maybe {
					return Err(e);
				} else {
					return Err(String::from("Undefined state 3, needs to be debugged if reaches this state"));
				}
			} else if let Err(e)= district_id_maybe {
				return Err(e);
			} else {
				return Err(String::from("Undefined state 4, needs to be debugged if reaches this state"));
			}
		}
		else if let RequestOutput::ReadError(errorstring) = state_districts_data{
				return Err(errorstring);
			} else if let RequestOutput::NetworkError(errorstring) = state_districts_data{
				return Err(errorstring);
			} else {
				return Err(String::from("Undefined state 2, needs to be debugged if reaches this state"));
			}
		} else if let Err(e) = state_id_resp {
			return Err(e);
		} else {
			return Err(String::from("Undefined state 5, needs to be debugged if reaches this state"));
		}
	} else if let RequestOutput::ReadError(errorstring) = all_state_data {
		Err(errorstring)
	} else if let RequestOutput::NetworkError(errorstring) = all_state_data {
		Err(errorstring)
	} else {
		Err(String::from("Undefined state 1, needs to be debugged if reaches this state"))
	}
}

pub fn parse_json_for_slots(json:serde_json::Value, age:i64, minnum:i64) -> Result<Vec<(String,String,i64,i64)>, std::string::String> {
	let mut result = Vec::new();
	if let serde_json::Value::Object(ref json_dict) = json {
		if let Some(centers_data) = json_dict.get("centers"){
			if let serde_json::Value::Array(centers_data_array) = centers_data{
				for one_center_object in centers_data_array {
					if let serde_json::Value::Object(one_center) = one_center_object {
						if let Some(center_name) = one_center.get("name") {
						if let Some(serde_json::Value::Array(center_slots)) = one_center.get("sessions") {
									for center_slot_object in center_slots{
										if let serde_json::Value::Object(center_slot) = center_slot_object {
											if let Some(serde_json::Value::Number(center_slot_seats)) = center_slot.get("available_capacity") {
												if let Some(available) = center_slot_seats.as_i64() {
													if available >= minnum{
														if let Some(freedate) = center_slot.get("date") {
															if let Some(serde_json::Value::Number(age_limit)) = center_slot.get("min_age_limit") {
																println!("{:?} slots seen at {:?} on {:?} for Age {:?}",center_slot_seats,center_name,freedate,age_limit );
																if let Some(min_age) = age_limit.as_i64() {
																	if age >= min_age{
																		result.push((center_name.to_string(),freedate.to_string(),min_age,available));
																	} else {
																		println!("But not enough slots for {:?} people",minnum );
																	}
																}
																else {
																	println!("But Slot Number isn't clear {:?}",center_slot_seats);
																}
															} else {
																println!("Cannot determine age in slot {:?}", center_slot );
															}

														} else {
															println!("Cannot Determine Date of Free Slot {:?}", center_slot );
														}

													} else {
														continue;
													}
												} else {
													println!("Age cannot be deduced in {:?}", center_slot );
												}
											} else {
												println!("no schedule seen in {:?}", center_slot_object );
											}
																					} else {
											println!("Cannot parse slot {:?}", center_slot_object);
										}
									}
								} else {
									println!("Cannot see slots in  {:?}", one_center );
								}

						} else {
							println!("No Center Name in {:?}",one_center );
							continue;
						}
					} else {
						return Err(String::from("Individual center data not in object form in API response"));
					}
				}
			} else {
				return Err(format!("No center array in JSON {:?}",json.to_string()));
			}
		} else {
			return Err(format!("No centers in JSON, maybe the structure has changed {:?}",json.to_string()));
		}
	} else {
		return Err(format!("Cannot Parse slots JSON {:?}",json.to_string()));
	}
	return Ok(result);
}

pub fn send_whatsapp_message(api_key:String, phone_number:String, center:String, datestring: String, slots: i64) -> RequestOutput {
	let message = format!("{} vaccine slots available at {} on {}",slots,center,datestring).replace(" ","%20");
	let callmeboturl = format!("https://api.callmebot.com/whatsapp.php?source=web&phone={}&apikey={}&text={}",phone_number,api_key,message);
	let resp_maybe = ureq::get(&callmeboturl).call();
	let returnval = match resp_maybe {
		Ok(response) => 
		match response.into_string() {
			Ok(string_response) => RequestOutput::Works(ureq::json!({"request_output":string_response})),
			Err(e) => RequestOutput::ReadError(e.to_string()),
		},
		Err(e) => RequestOutput::NetworkError(e.to_string()) ,
	};
	return returnval;
    }

pub fn send_whatsapp_message_borrowed(api_key:String, phone_number:String, center:&String, datestring: &String, slots: i64) -> RequestOutput {
	let message = format!("{} vaccine slots available at {} on {}",slots,&center,&datestring).replace(" ","%20");
	let callmeboturl = format!("https://api.callmebot.com/whatsapp.php?source=web&phone={}&apikey={}&text={}",phone_number,api_key,message);
	let resp_maybe = ureq::get(&callmeboturl).call();
	let returnval = match resp_maybe {
		Ok(response) => 
		match response.into_string() {
			Ok(string_response) => RequestOutput::Works(ureq::json!({"request_output":string_response})),
			Err(e) => RequestOutput::ReadError(e.to_string()),
		},
		Err(e) => RequestOutput::NetworkError(e.to_string()) ,
	};
	return returnval;
    }