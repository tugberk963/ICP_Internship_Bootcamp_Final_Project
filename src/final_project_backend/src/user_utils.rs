use crate::{USERS, PROVIDERS, AppointmentDetails, User};
use ic_cdk::{query, update};
use candid::Principal;
use std::collections::HashMap;

#[update]
fn make_appointment(provider_id: String, department_name: String, doctor_name: String, date: String, time: String) -> Result <(), String>{
    let appointment_available: bool = PROVIDERS.with(|providers| {
        let mut providers = providers.borrow_mut();
        if let Some(provider_data) = providers.get_mut(&Principal::from_text(&provider_id).expect("Provider not found")){
            if let Some(department) = provider_data.departments.get_mut(&department_name){
                if let Some(doctor) = department.get_mut(&doctor_name){
                    if let Some(selected_date) = doctor.get_mut(&date){
                        if let Some(index) = selected_date.iter().position(|t| t == &time) {
                            // Make the Appointment if the time slot is available
                            selected_date.remove(index);
                            true
                        } else {
                            false // Time slot not found
                        }
                    }
                    else {
                        // Err("No available date".to_string())
                        false
                    }
                }
                else {
                    // Err("Doctor not found".to_string())
                    false
                }
            }
            else {
                // Err("Department not found".to_string())
                false
            }
        }
        else{
            // Err("Provider not found".to_string())
            false
        }
    });

    if appointment_available{
        USERS.with(|users|
        {
            let mut users = users.borrow_mut(); // Find user id with the ic_cdk caller principal id.
            // Add the appointment which we checked above if it exists.
            match users.get_mut(&ic_cdk::caller()) {
                Some(user) => {
                    let appointment_details = AppointmentDetails {
                        department: department_name.clone(),
                        doctor: doctor_name.clone(),
                        date: date.clone(),
                        time: time.clone(),
                    };

                    if let Some(appointments) = user.appointments.get_mut(&Principal::from_text(provider_id.clone()).expect("Provider id not found")) {
                        // Provider ID exists in the user's appointments hashmap
                        appointments.push(appointment_details);
                    } else {
                        // Provider ID does not exist, insert a new vector
                        user.appointments.insert(Principal::from_text(provider_id.clone()).expect("Provider id not found"), vec![appointment_details]);
                    }                   
                    Ok(())
                }
                None => Err("User not found".to_string())
            }
        }
        )
    }
    else {
        Err("No appointment available with desired info".to_string())
    }
}

#[query]
fn list_appointments(user_id: String) -> <HashMap<Principal, Vec<String>> {
    USERS.with(|users| {
        let users = users.borrow();

        // Extracting Principal from the Result
        let principal_result = Principal::from_text(&user_id);
        if let Ok(principal) = principal_result {
            if let Some(user) = users.get(&principal) {
                // Accessing appointments of the user
                return Some(user.appointments.clone());
            }
        }
        
        None // Return None if user not found or error in Principal extraction
    })
}