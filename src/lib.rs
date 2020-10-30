use std::collections::LinkedList;
use objects::*;
use std::fs;
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc,};
use std::time::SystemTime;
use shared_parser::{Event_parser,time_inside};


//loads a file and creates a linked list containing the entrys from it, list is ordered with the list ordered push function
//this means that the list should be ordered by time.
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

//pushes an event to this list oredered based
//on the time component of the datetime feild
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
			

//a function to show the  days events			
fn show_events(){			
   	let Date = DateTime::<Utc>::from(SystemTime::now()).date();
	let date_filename = format!("{:?}.txt",Date);
	let todays_list = load_days_reminders(date_filename);
	let mut length = todays_list.len();
	let a = 0
	while a < length {
        if time_inside(todays_list[a]) > SystemTime::now().DateTime::<Utc>::from().time(){
			println!("{:?}", show(today_list[a]));
		}
	}
}	

fn show(entry:entrys) -> String {
	entry
	
	

#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_load_days_reminders(){
		let test_string = "/n Todo Title timmy /n Datetime 2014-11-28T12:45:59.324310806Z /n List long elephant /n # Events Title rosy /n DateTime 2014-11-28T12:30:59.324310806Z /n Description it's about elephants /n Attendees many elephants /n # ".to_string();
		fs::write("test_file.txt", test_string);
		let test_list = load_days_reminders("test_file.txt");
		let Title = "rosy".to_string();
		let DateTime = DateTime::from("2014-11-28T12:30:59.324310806Z");
		let Description = "it's about elephants".to_string();
		let Attendees = many elephants 
		assert_eq!(test_list[0], entrys::Events(Events::new(
		    Title,
		    DateTime,
		    Description,
		    Attendees,
		)));
	}
		    	

}
