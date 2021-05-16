mod api_stuff;

	match api_stuff::get_india_states() {
		api_stuff::RequestOutput::Works(j) => println!("Success {:?}",j),
		api_stuff::RequestOutput::ReadError(s)=> println!("Error :: {:?}",s),
		api_stuff::RequestOutput::NetworkError(e) => println!("Error {:?}",e )
	};

	let state_id = String::from("34");
	match api_stuff::get_state_districts(state_id) {
		api_stuff::RequestOutput::Works(j) => println!("Success {:?}",j),
		api_stuff::RequestOutput::ReadError(s)=> println!("Error :: {:?}",s),
		api_stuff::RequestOutput::NetworkError(e) => println!("Error {:?}",e )
	};

	let district_id= String::from("654");
	let date= String::from("08-05-2021");
	match api_stuff::district_vaccination_schedule(district_id,date) {
		api_stuff::RequestOutput::Works(j) => println!("Success {:?}",j),
		api_stuff::RequestOutput::ReadError(s)=> println!("Error :: {:?}",s),
		api_stuff::RequestOutput::NetworkError(e) => println!("Error {:?}",e )
	};

	let pincode = String::from("273001");
	let date2 = String::from("13-05-2021");
	match api_stuff::pincode_vaccination_schedule(pincode,date2) {
	api_stuff::RequestOutput::Works(j) => println!("Success {:?}",j.to_string()),
	api_stuff::RequestOutput::ReadError(s)=> println!("Error :: {:?}",s),
	api_stuff::RequestOutput::NetworkError(e) => println!("Error {:?}",e )	
    };
    
    let state = String::from("Uttar Pradesh");
    let district = String::from("Gorakhpur");
    match api_stuff::get_data_for_state_district(state,district,String::from("13-05-2021")){
    	Ok(okjson) => println!("{:?}",okjson.to_string() ),
    	Err(errstr) => println!("{:?}",errstr)
    };
    match api_stuff::get_data_for_state_district(state,district,String::from("13-05-2021")){
    	Ok(okjson) => println!("{:?}",api_stuff::parse_json_for_slots(okjson,47,2) ),
    	Err(errstr) => println!("{:?}",errstr)
    };
    match api_stuff::send_whatsapp_message(String::from("XXXX"), "+91XXXXX".to_string(), "ABCD".to_string(), "12-05-2021".to_string(), 45){
    api_stuff::RequestOutput::Works(j) => println!("Success {:?}",j),
	api_stuff::RequestOutput::ReadError(s)=> println!("JSON Read Error :: {:?}",s),
	api_stuff::RequestOutput::NetworkError(e) => println!("Network Error {:?}",e )
    };