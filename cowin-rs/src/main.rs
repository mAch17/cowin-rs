extern crate clap;
use clap::{Arg, App, SubCommand,crate_authors,crate_version};
mod api_stuff;

fn main() {
	let matches = App::new("COWIN Command Line Access")
	.author(crate_authors!())
	.version(crate_version!())
	.about("This command line utility helps you search for A COVID jab online in India. Uses Indian government's COWIN public APIs and sends you notification on phone as SMS.")
	.before_help("Hope you are keeping safe !")
	.after_help("Let's try and find you COVID 19 resources on command line")
	.arg(Arg::with_name("phone_number")
		.short("P")
		.long("phone")
		.help("ENter your Phone Number for login and SMS alerts {format +91XXXXXXXXXX} .. Optional"))
	.arg(Arg::with_name("textlocal_api_key")
		.short("t")
		.long("textlocalAPIKey")
		.help("Text Local API key in case you want SMS reminders.. sign up at https://www.textlocal.in")
		)
    .subcommand(SubCommand::with_name("trackIn")
    	.about("track the COWIN portal for jabs")
    	.arg(Arg::with_name("pincode")
    		.short("p")
    		.long("pincode")
    		.help("PINCODE of the area you want to track vaccination for, takes precedence over District")
    	)
    	.arg(Arg::with_name("district")
    		.short("d")
    		.long("district")
    		.help("District where you want to track Jabs in")
    		)
    	.arg(Arg::with_name("state")
    		.short("s")
    		.long("state")
    		.help("State for the district where you track jabs in")
    		)
    	)
	.get_matches();

	let phone = matches.value_of("phone_number");
	let textlocal = matches.value_of("textlocal_api_key");
	if let Some(trackin_matches) = matches.subcommand_matches("trackIn"){
		let pincode = trackin_matches.value_of("pincode");
		let district = trackin_matches.value_of("district");
		let state = trackin_matches.value_of("state");
		// Take whatever action you want here
	}

	//match api_stuff::get_india_states() {
	//	api_stuff::RequestOutput::Works(j) => println!("Success {:?}",j),
	//	api_stuff::RequestOutput::ReadError(s)=> println!("Error :: {:?}",s),
	//	api_stuff::RequestOutput::NetworkError(e) => println!("Error {:?}",e )
	//};

	//let state_id = String::from("34");
	//match api_stuff::get_state_districts(state_id) {
	//	api_stuff::RequestOutput::Works(j) => println!("Success {:?}",j),
	//	api_stuff::RequestOutput::ReadError(s)=> println!("Error :: {:?}",s),
	//	api_stuff::RequestOutput::NetworkError(e) => println!("Error {:?}",e )
	//}

	//let district_id= String::from("654");
	//let date= String::from("08-05-2021");
	//match api_stuff::district_vaccination_schedule(district_id,date) {
	//	api_stuff::RequestOutput::Works(j) => println!("Success {:?}",j),
	//	api_stuff::RequestOutput::ReadError(s)=> println!("Error :: {:?}",s),
	//	api_stuff::RequestOutput::NetworkError(e) => println!("Error {:?}",e )
	//}

	//let pincode = String::from("273001");
	//let date2 = String::from("13-05-2021");
	//match api_stuff::pincode_vaccination_schedule(pincode,date2) {
	//	api_stuff::RequestOutput::Works(j) => println!("Success {:?}",j.to_string()),
	//	api_stuff::RequestOutput::ReadError(s)=> println!("Error :: {:?}",s),
	//	api_stuff::RequestOutput::NetworkError(e) => println!("Error {:?}",e )	
    //}
    
    //let state = String::from("Uttar Pradesh");
    //let district = String::from("Gorakhpur");
    //match api_stuff::get_data_for_state_district(state,district,String::from("13-05-2021")){
    //	Ok(okjson) => println!("{:?}",okjson.to_string() ),
    //	Err(errstr) => println!("{:?}",errstr)
    //}
    //match api_stuff::get_data_for_state_district(state,district,String::from("13-05-2021")){
    //	Ok(okjson) => println!("{:?}",api_stuff::parse_json_for_slots(okjson,47,2) ),
    //	Err(errstr) => println!("{:?}",errstr)
    //}
    match api_stuff::send_textlocal_sms(String::from("ZDRkNmZkNDIxOTQ1Y2Y3MDY0NjhhZGE3NWIyYTE3OGM="), "+918447844687".to_string(), "ABCD".to_string(), "12-05-2021".to_string(), 45){
    api_stuff::RequestOutput::Works(j) => println!("Success {:?}",j),
	api_stuff::RequestOutput::ReadError(s)=> println!("Error :: {:?}",s),
	api_stuff::RequestOutput::NetworkError(e) => println!("Error {:?}",e )
    }
}