use std::collections::LinkedList;
use objects::*;
use std::fs;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc,};
use std::time::SystemTime;
use shared_parser::{Event_parser,time_inside};

fn load_days_reminders(date_filename:String)->LinkedList<entrys>{
	let todays_file_string = match fs::read_to_string (date_filename){
		Ok(a) => Some(a),
		Err(a) => None,
	};
	let mut todays_list: LinkedList<entrys> = LinkedList::new();
	let mut dummy = String::new();  
	let split_todays_file_string:Option<Vec<&str>> = match todays_file_string {
		 Some(a)=>{
			 dummy = a;
			 Some((dummy.split("#").collect::<Vec<&str>>()).clone())
		},
		 None =>None,
	};
	match split_todays_file_string {
		Some(a) => { 
			for element in a {
				let element_vector:Vec<&str> = element.split_whitespace().collect();
				match element_vector[0]{
					"Todo" => {
						todays_list = list_ordered_push(entrys::Todo(Event_parser(element_vector,vec!["Title".to_string(),"DateTime".to_string(),"List".to_string(),"ATTyr".to_string()])), todays_list.clone());
					},
					"Events" => {
						todays_list = list_ordered_push(entrys::Events(Event_parser(element_vector,vec!["Title".to_string(),"DateTime".to_string(),"Description".to_string(),"Attendees".to_string()])), todays_list.clone());
					},
					"Appointments" => {
						todays_list = list_ordered_push(entrys::appointments(Event_parser(element_vector,vec!["Title".to_string(),"DateTime".to_string(),"With_who".to_string(),"Description".to_string()])), todays_list.clone());
					},
					_ =>{},
				}
			}
			return todays_list
	    }
	    None =>  return todays_list
	}
}	

fn list_ordered_push(item:entrys, mut list:LinkedList<entrys>) -> LinkedList<entrys> {
	if list == (LinkedList::new()){
		list.push_back(item);
		return list
	}else{
		let end = list.len();
		let mut position = 0;
		for value in list.iter(){
			let value_time = time_inside(&value);
			let item_time = time_inside(&item);
			if item_time <= value_time {
				if position == 0{
					list.push_front(item);
				}else{
					let mut list_back = list.split_off(position);
					list.push_back(item);
					list.append(&mut list_back);
				}
			    return list
			}
			if position == end -1 {
				list.push_back(item);
				return list
			}
			position += 1;
		}
	}
	return list
}
			
			
fn show_events(){			
   	let Date = DateTime::<Utc>::from(SystemTime::now()).date();
	let date_filename = format!("{:?}.txt",Date);
}	
	
	

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    fn test_load_days_reminders(){
		let test_string = "/n Todo Title timmy /n Datetime /n List long elephant /n # Events Title rosy /n DateTime /n Description it's about elephants /n Attendees many elephants /n # ".to_string();
		fs::write("test_file.txt", test_string);
		load_days_reminders("test_file.txt");
		
	}
		

}
