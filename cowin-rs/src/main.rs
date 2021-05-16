extern crate clap;
extern crate job_scheduler;
extern crate chrono;
use clap::{Arg, App, SubCommand,crate_authors,crate_version};
mod api_stuff;

use job_scheduler::{JobScheduler, Job};
use std::time::Duration;

fn tracking_process(phone_maybe: Option<&str>, api_key_maybe: Option<&str>, phone_number_input_present: bool, whatsapp_apikey_available: bool, age_user:i64, num_user:i64, pincode_maybe : Option<&str> , state_maybe: Option<&str>, district_maybe: Option<&str>, pincode_input_present: bool, district_input_present : bool, date_today: String ) -> Result<String,String> {
if ! pincode_input_present && ! district_input_present{
	panic!("Need at least PINCODE or District/State to track");
	}
if pincode_input_present {
	if let Some(pincode) = pincode_maybe {
		match api_stuff::pincode_vaccination_schedule(pincode.to_string(),date_today) {
			api_stuff::RequestOutput::Works(j) => match api_stuff::parse_json_for_slots(j,age_user,num_user) {
				Ok(opvec) => if phone_number_input_present && whatsapp_apikey_available {
					if let Some(api_key) = api_key_maybe{
						if let Some(phone_number) = phone_maybe {
								for opslot in &opvec {
								match api_stuff::send_whatsapp_message_borrowed(api_key.to_string(), phone_number.to_string(), &opslot.0, &opslot.1, opslot.3 ){
									api_stuff::RequestOutput::Works(_j) => continue,
									api_stuff::RequestOutput::ReadError(s) => return Err(format!("Error in sending to whatsapp {:?}",s)),
									api_stuff::RequestOutput::NetworkError(s) => return Err(format!("Error in sending to whatsapp {:?}",s)),
								}
								}
						} else {
							return Err(format!("phone number cannot be parsed from {:?}",phone_maybe));
						}
					} else {
						return Err(format!("API Key cannot be parsed from {:?}",api_key_maybe));
					}
					return Ok(format!( "{:?} slots seen, sent on whatsapp to {:?}", opvec.len(), phone_maybe ));
				} else {
					println!("{:?} slots seen", opvec.len() );
					for opslot in &opvec {
						println!("{:?} Slots at {:?} on {:?}", opslot.3, opslot.0, opslot.1 );
					}
					return Ok(format!( "{:?} slots seen", opvec.len() ));
				} ,
				Err(e) => return Err(e),
			},
			api_stuff::RequestOutput::ReadError(s)=> return Err(s),
			api_stuff::RequestOutput::NetworkError(e) => return Err(e),
    	};
	} else {
		return Err(format!("pincode cannot be parsed {:?}",pincode_maybe));
	}
}

if district_input_present{
if let Some(state) = state_maybe {
	if let Some(district) = district_maybe {

    	match api_stuff::get_data_for_state_district(state.to_string(),district.to_string(),date_today){
    		Ok(j) => 
    		//Copy from here
    		 match api_stuff::parse_json_for_slots(j,age_user,num_user) {
				Ok(opvec) => if phone_number_input_present && whatsapp_apikey_available {
					if let Some(api_key) = api_key_maybe{
						if let Some(phone_number) = phone_maybe {
								for opslot in &opvec {
								match api_stuff::send_whatsapp_message_borrowed(api_key.to_string(), phone_number.to_string(), &opslot.0, &opslot.1, opslot.3 ){
									api_stuff::RequestOutput::Works(_j) => continue,
									api_stuff::RequestOutput::ReadError(s) => return Err(format!("Error in sending to whatsapp {:?}",s)),
									api_stuff::RequestOutput::NetworkError(s) => return Err(format!("Error in sending to whatsapp {:?}",s)),
								}
								}
						} else {
							return Err(format!("phone number cannot be parsed from {:?}",phone_maybe));
						}
					} else {
						return Err(format!("API Key cannot be parsed from {:?}",api_key_maybe));
					}
					return Ok(format!( "{:?} slots seen, sent on whatsapp to {:?}", opvec.len(), phone_maybe ));
				} else {
					println!("{:?} slots seen", opvec.len() );
					for opslot in &opvec {
						println!("{:?} Slots at {:?} on {:?}", opslot.3, opslot.0, opslot.1 );
					}
					return Ok(format!( "{:?} slots seen", opvec.len() ));
				} ,
				Err(e) => return Err(e),
			},
    		// To here
    		Err(errstr) => return Err(errstr),
    	};		

	} else {
		return Err(format!("Cannot parse district from {:?}", district_maybe));
	}
} else {
	return Err(format!("Cannot parse state from {:?}", state_maybe ));
}
}
return Err("Unreachable condition Encountered !!".to_string());
}

fn main() {
	let matches = App::new("COWIN Command Line Access")
	.author(crate_authors!())
	.version(crate_version!())
	.about("This command line utility helps you search for A COVID jab online in India. Uses Indian government's COWIN public APIs and sends you notification on phone as Whatsapp Message. This tool doesn't store any data locally or on cloud, so don't worry about privacy")
	.before_help("Hope you are keeping safe !")
	.after_help("Let's try and find you COVID 19 resources on command line")
	.arg(Arg::with_name("phone_number")
		.short("P")
		.long("phone")
		.takes_value(true)
		.help("Enter your Phone Number for login(TBD) and WhatsApp alerts {format +91XXXXXXXXXX} .. Optional"))
	.arg(Arg::with_name("callmebot_api_key")
		.short("w")
		.long("callmebotAPIKey")
		.takes_value(true)
		.help("CallMeBot API key in case you want WhatsApp reminders.. Optional. Sign up at https://www.callmebot.com/blog/free-api-whatsapp-messages/ to get WhatsApp API key. Use same number you enter as phone number you enter.")
		)
	.arg(Arg::with_name("age_in_years")
		.short("Y")
		.long("ageYears")
		.takes_value(true)
		.help("Enter your age in years to match slot. Will not forward a 45+ years slot to a person of age between 18-44. Default : 19")
		)
	.arg(Arg::with_name("number_of_people")
		.short("N")
		.long("numberPeople")
		.takes_value(true)
		.help("Number of people in your group to get vaccinated. Will not inform/ping you if Number of slots available < number of people in your group. Default : 1.")
		)
    .subcommand(SubCommand::with_name("trackIn")
    	.about("track the COWIN portal for jabs")
    	.arg(Arg::with_name("pincode")
    		.short("p")
    		.long("pincode")
    		.takes_value(true)
    		.help("PINCODE of the area you want to track vaccination for, takes precedence over District")
    	)
    	.arg(Arg::with_name("district")
    		.short("d")
    		.long("district")
    		.takes_value(true)
    		.help("District where you want to track Jabs in")
    		)
    	.arg(Arg::with_name("state")
    		.short("s")
    		.long("state")
    		.takes_value(true)
    		.help("State for the district where you track jabs in")
    		)
    	.arg(Arg::with_name("ping_every_n_minutes")
			.short("m")
			.long("pingMinutes")
			.takes_value(true)
			.help("Will ping on COWIN portal every on m minutes, default : 5 minutes")
			)
    	)
	.get_matches();

	let phone_maybe = matches.value_of("phone_number");
	let api_key_maybe = matches.value_of("callmebot_api_key");
	let age_maybe = matches.value_of("age_in_years");
	let numpeople_maybe = matches.value_of("number_of_people");
	
	// Parsing age
	let mut age_user:i64 = 19; // Taking user to be by default of 19 years 
	if let Some(age_str) = age_maybe{
		if let Ok(age_number) = age_str.parse::<i64>()
		{
		age_user = age_number;
		} else {
			panic!("Cannot Parse age from {:?}",age_str );
		}
	}
	
	//Parsing Number of people
	let mut num_user:i64 = 1; // Taking user to be by default of 19 years 
	if let Some(numpeople_str) = numpeople_maybe{
		if let Ok(numpeople_number) = numpeople_str.parse::<i64>()
		{
		num_user = numpeople_number;
		} else {
			panic!("Cannot Parse #People from {:?}",numpeople_str );
		}
	}

    let mut phone_number_input_present = false;
    if let Some(_phone_number) = phone_maybe {
    	phone_number_input_present = true;
    }
    let mut whatsapp_apikey_available = false;
    if let Some(_apikey) = api_key_maybe {
    	whatsapp_apikey_available = true;
    }

	if let Some(trackin_matches) = matches.subcommand_matches("trackIn"){
		let pincode_maybe = trackin_matches.value_of("pincode");
		let district_maybe = trackin_matches.value_of("district");
		let state_maybe = trackin_matches.value_of("state");
		let ping_minutes_maybe = trackin_matches.value_of("ping_every_n_minutes");
		
		let mut ping_minutes = "5"; // If one doesnt give Ping Minutes, default value is 5 minutes
		if let Some(ping_minutes_in) = ping_minutes_maybe{
           ping_minutes = ping_minutes_in;
		}
		
		let mut pincode_input_present = false;
		let mut district_input_present = false;
		if let Some(_pincode) = pincode_maybe{
			pincode_input_present = true;
		}
		if let Some(_state) = state_maybe {
			if let Some(_district) = district_maybe{
				district_input_present = true;
			}
		}

		//Check first time and see if any errors
		let mut sched = JobScheduler::new();
		let cron_string = format!("* 1/{} * * * *", ping_minutes );
	    sched.add(Job::new(cron_string.parse().unwrap(), || {
	    	let now = chrono::Utc::now();
			let date_today = now.format("%d-%m-%Y").to_string();
    	    let tr_maybe = tracking_process(phone_maybe, api_key_maybe, phone_number_input_present, whatsapp_apikey_available, age_user, num_user, pincode_maybe, state_maybe, district_maybe, pincode_input_present, district_input_present, date_today);
    	    match tr_maybe {
    	    	Ok(success) => println!("{:?}", success),
    	    	Err(estr) => println!("{:?}", estr ),
    	    }
    	}));

    	loop {
        	sched.tick();

	        std::thread::sleep(Duration::from_millis(500));
    	}
	}
}