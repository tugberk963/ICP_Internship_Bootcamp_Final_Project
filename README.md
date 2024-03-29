# DMS - Decentralized Medical Services - ICP Internship Bootcamp Final Project - Mini Hackathon Version

## Table of Contents
- [Project Name](#project-name)
- [Project Team](#project-team---individual)
- [Project Description](#project-description---purpose)
- [Project Vision](#project-vision)
- [Project Setup](#project-setup)
  - [Setting up IC CDK](#setting-up-ic-cdk)
  - [Setting up Rust](#setting-up-rust)
  - [Starting Server](#starting-server)
  - [Deploying Canisters](#deploying-canisters)
- [Project Tree](#project-tree)
- [Project Functionalities](#project-functionalities)
  - [Testing Process for Hackathon](#testing-process-for-hackathon)
  - [Authentication](#authentication)
    - [Signup](#signup)
    - [Login](#login)
    - [Logout](#logout)
  - [User Functionalities](#user-functionalities)
    - [Editing Personal Profile](#editing-personal-profile)
    - [Viewing Provider Info](#viewing-provider-info) 
    - [Listing Providers](#listing-providers)
    - [Listing Departments](#listing-departments)
    - [Listing Doctors](#listing-doctors)
    - [Listing Dates](#listing-dates)
    - [Listing Times](#listing-times)
    - [Making Appointment](#making-appointment)
    - [Listing Appointments](#listing-appointments)
  - [Provider Functionalities](#provider-functionalities)
    - [Adding Departments](#adding-departments)
    - [Adding Doctors](#adding-doctors)
    - [Adding Dates](#adding-dates)
    - [Adding Times](#adding-times)
    - [Listings](#listings)
    - [Getting Patient Info](#getting-patient-info)
    - [Adding Disease](#adding-disease)
    - [Adding Allergy](#adding-allergy)
    - [Adding Medications](#adding-medications)
    - [Editing Provider Info](#editing-provider-info)
    - [Listing](#listing)
  - [Admin Functionalities](#admin-functionalities)
    - [Listing Signed Users](#listing-signed-users)
    - [Listing Active Sessions](#listing-active-sessions)
    - [Controlling If User Is Logged In](#controlling-if-user-is-logged-in)
    - [Setting Providers](#setting-providers)
    - [Listing Providers](#listing-providers)
    - [Removing Providers](#removing-providers)
- [Further Moves](#further-moves)

## Project Name
<br> <img src="https://github.com/tugberk963/ICP_Internship_Bootcamp_Final_Project/blob/main/logo.png"> <br> </br>
DMS: Medical Service Provider on <b><a href="https://internetcomputer.org/"> Internet Computer </a></b>.
<br> <br> <b> <i> DMS stands for Decentralized Medical Services. </b> </i> 
## Project Team - Individual
I'm Tuğberk Serçe, a 22-year-old Senior Physics Student at Yildiz Technical University. My journey with software began during my high school years and has since evolved into a serious pursuit. I'm currently privileged to serve as a Full Stack Developer Intern at one of a kind IoT company. Specializing in Angular, Typescript, and Python, I'm proficient in various frontend frameworks and multiple programming languages. My fascination with low-level languages fuels my dedication to mastering these skills, propelling me to make meaningful contributions to the world of software development.
## Project Description - Purpose 
DMS aims to revolutionize healthcare services by bringing together healthcare service providers (hospitals, health clinics, etc.) and patients on a unified platform. </br> </br> <strong> <strike> <i>  For final project, some of main functionalities of DMS built on Rust as a back-end canister. </i> </strike> </strong> </br> </br><strong><i>For the ICP Internship Bootcamp Mini Hackathon, some of the core functionalities of DMS have been designed and presented as a Full Stack dApp during the hackathon.</i></strong>
## Project Vision
### Problems DMS aiming to Solve
National healthcare systems are localized, but DMS aims to transform this into a global and on-chain system on the Internet Computer Protocol (ICP). </br> </br>
Healthcare data is currently underutilized and not adequately leveraged by both patients and medical service providers.
</br> </br> The operational structure of the healthcare system has remained stagnant for years, characterized by repetitive tasks.
### The Vision of DMS
The primary vision of the DMS is to establish DMS as the <b>  <i> central platform for healthcare service providers and patients worldwide. </i> </b>
<br>Through the vast data flow integrated into DMS, we empower its AI capabilities, transforming it into an efficient and proactive predictive healthcare service provider. <br> This strategic approach aims to revolutionize healthcare delivery, offering advanced and personalized services to users globally.
### Key Contributions of DMS
### Single Interface for Patients and Healthcare Providers:
DMS provides a unified platform where both patients and healthcare providers can access and manage all medical service operations. Patients can schedule appointments, search for doctors, and view medical records, while healthcare providers can handle tasks such as doctor assignment, appointment tracking, and updating medical records, all through a single interface standing on <b><a href="https://internetcomputer.org/"> Internet Computer </a></b>.
### Medical Record Security:
Patients' medical records are securely stored on the  <b> <a href="https://internetcomputer.org/"> Internet Computer </a> </b>. Data is protected with end-to-end encryption and security protocols, ensuring that access is restricted to authorized users only.
### Utilization of Data for Predictive Healthcare Services:
DMS will collect and leverage data to train machine learning models for predictive healthcare services. These models analyze patterns and trends within the data to make predictions related to patients' health outcomes, potential diseases, or treatment effectiveness.
###  Grant System for Data Sharing:
DMS will implement a grant system where individuals providing health data are rewarded with grants. Hospitals, as the institutions approving this data, receive grants based on the number of patients they approve. These grants contribute to financing the privileged healthcare services offered to DMS users.
### Enhanced Healthcare Services through AI:
By utilizing AI-driven predictive models, DMS will enhance healthcare services by providing proactive and personalized care to patients. For example, the AI model can identify individuals at higher risk for certain health conditions and recommend preventive measures or early interventions to mitigate those risks.
### Improving Healthcare Accessibility and Efficiency:
AI-driven predictive models will optimize resource allocation and improve healthcare service efficiency within DMS. By predicting patient needs and allocating resources accordingly, healthcare providers streamline their operations and ensure better accessibility to healthcare services for all patients.
<br> </br> <img src="https://github.com/tugberk963/ICP_Internship_Bootcamp_Final_Project/blob/main/GrantSystem.png"> <br> </br>
## Project Setup
### Setting up IC CDK
```bash
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```
For more information or platform specific setup instructions see <a href="https://internetcomputer.org/docs/current/developer-docs/getting-started/install/">ICP Docs : IC_CDK </a>
### Setting up Rust
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```
```bash
rustup target add wasm32-unknown-unknown
```
For more information or platform specific setup instructions see <a href="https://internetcomputer.org/docs/current/developer-docs/backend/rust/dev-env" >ICP Docs : Rust</a>
### Starting Server
For starting project on our local. You can simply use the command below.
```bash
cd ICP_Internship_Bootcamp_Final_Project/
```
```bash
dfx start --clean
```
### Deploying Canisters
For deploying canisters and generating our canister interface. You can use the command below.
```bash
cd ICP_Internship_Bootcamp_Final_Project/
```
```bash
dfx deploy
```
Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with
```bash
npm run generate
```
at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `dfx deploy`.
If you are making frontend changes, you can start a development server with
```bash
npm start
```
Which will start a server at `http://localhost:8080`, proxying API requests to the replica at port 4943.
## Project Tree
### Front-end - Svelte
```bash
├── src
│   ├── app.d.ts
│   ├── app.html
│   ├── index.scss
│   ├── lib
│   │   └── canisters.js
│   ├── routes
│   │   ├── +layout.js
│   │   ├── +page.svelte
│   │   ├── allergies
│   │   │   └── +page.svelte
│   │   ├── appointment
│   │   │   └── +page.svelte
│   │   ├── diseases
│   │   │   └── +page.svelte
│   │   ├── home
│   │   │   ├── +page.svelte
│   │   │   └── User.svelte
│   │   ├── manage_patients
│   │   │   └── +page.svelte
│   │   ├── medications
│   │   │   └── +page.svelte
│   │   ├── personal_data
│   │   │   └── +page.svelte
│   │   ├── provider
│   │   │   └── +page.svelte
│   │   ├── provider_management
│   │   │   └── +page.svelte
│   │   ├── provider_profile
│   │   │   └── +page.svelte
│   │   ├── search_patients
│   │   │   └── +page.svelte
│   │   └── visits
│   │       └── +page.svelte
```
### Back-end - Rust
```bash
.
├── Cargo.toml
├── icp_internship_bootcamp_final_project_backend.did 
    ├── administration.rs 
    ├── auth.rs 
    ├── lib.rs 
    ├── provider_utils.rs 
    └── user_utils.rs 
```
## Testing Process for Hackathon
To begin testing the environment as demonstrated in the GIFs below, follow these steps:
```bash
dfx start --clean
```
```bash
dfx deploy
```
```bash
npm start
```
After running these commands, you can test the application by accessing it at http://localhost:8080 in your web browser.
## &#9745; Authentication 
<br> <img src="https://github.com/tugberk963/ICP_Internship_Bootcamp_Final_Project/blob/main/auth.gif"> <br> </br>
## &#9745; Editing Personal Profile
Patients have the ability to edit and manage their personal profiles within the system.
<br> <img src="https://github.com/tugberk963/ICP_Internship_Bootcamp_Final_Project/blob/main/personal_profile.gif"> <br> </br>
## &#9745;Creating a Provider
This function, typically reserved for Admin users, has been made accessible on the user's navbar for convenience during the testing and development phase, as we are utilizing a single identity for these purposes.
<br> <img src="https://github.com/tugberk963/ICP_Internship_Bootcamp_Final_Project/blob/main/setting_provider.gif"> <br> </br>
## &#9745; Editing Provider Profile
Providers  have the capability to edit their name and location within the system.
<br> <img src="https://github.com/tugberk963/ICP_Internship_Bootcamp_Final_Project/blob/main/providerprofile.gif"> <br> </br>
## &#9745; Provider Submodule Management: Departments, Doctors, and Appointment Scheduling
Providers are able to create departments, add doctors to those departments, and manage their appointment schedules, including dates and times, within the system.
<br> <img src="https://github.com/tugberk963/ICP_Internship_Bootcamp_Final_Project/blob/main/creatingappointment.gif"> <br> </br>
## &#9745; Making an Appointment as an User.
Patients have the ability to view active providers on the DMS and can schedule appointments with their desired departments, doctors, dates, and times.
<br> <img src="https://github.com/tugberk963/ICP_Internship_Bootcamp_Final_Project/blob/main/takinganappointment.gif"> <br> </br>
## &#9745; Managing and Viewing Health Data.
Patients have access to view their own health data on the DMS. Meanwhile, providers are able to edit the health data of desired patients or users.
<br> <img src="https://github.com/tugberk963/ICP_Internship_Bootcamp_Final_Project/blob/main/managinghealthdata.gif"> <br> </br>
## &#9745; Searching Patients
Providers have the capability to search for desired patients and view their data within the system.
<br> <img src="https://github.com/tugberk963/ICP_Internship_Bootcamp_Final_Project/blob/main/searchingpatients.gif"> <br> </br>

## Project Functionalities
## Authentication 
### &#9745; Signup
Users can signup to DMS with desired username and password.
```rust
fn sign_up(username: String, password: String) -> Result<(), String>
```
### &#9745; Login
When users login with their username and password, if the credentials are correct. Login function adds their Principal ID to ACTIVE_SESSIONS.
```rust
fn login(username: String, password: String) -> Result<(), String>
```
### &#9745; Logout
When user wants to log off. Their Principal ID will be removed from ACTIVE_SESSIONS.
```rust
fn logout()
```
###  &#9745; Editing Personal Profile
Users can edit their Personal Profile.
```rust
fn edit_user_personal_data(user_id: String, name: String, surname: String, location: String, age: String, height: String, weight: String) -> Result<(), String> 
```
###  &#9745; Viewing Provider Info
Users can view provider info. Provider name, location, departments etc.
```rust
fn get_provider_info(provider_id: String) -> Option<String> 
```
###  &#9745; Listing Doctors
Users can list selected departments doctors.
```rust
fn list_doctors(provider_id: String, department_name: String) -> Vec<String>
```
### &#9745; Listing Dates
Users can list available appointment dates for desired doctor.
```rust
fn list_dates(provider_id: String, department_name: String, doctors_name: String) -> Vec<String>
```
### &#9745; Listing Times
Users can list available appointment times for desired dates on selected doctor.
```rust
fn list_times(provider_id: String, department_name: String, doctors_name: String, selected_date: String) -> Vec<String> 
```
### &#9745; Making Appointment 
Users can make appointment. If provider has the available appointment according to users input. That appointment will be deleted from providers appointment list and will be added into users active appointments list.
```rust
fn make_appointment(provider_id: String, department_name: String, doctor_name: String, date: String, time: String) -> Result <(), String>
```
### &#9745; Listing Appointments
Users can list his active appointments.
```rust
fn list_appointments(user_id: String) -> HashMap<Principal, Vec<AppointmentDetails>> 
```
## Provider Functionalities
In DMS, providers refer to any facility that is a health or medical service provider.
### &#9745; Adding Departments
Providers can add departments to themselves.
```rust
fn add_department(provider_id: String, department_name: String) -> Result<(), String> 
```
### &#9745; Adding Doctors
Providers can add doctors to their departments.
```rust
fn add_doctor(provider_id: String, department_name: String, doctor_name: String) -> Result<(), String>
```
### &#9745; Adding Dates
Providers can add appointment dates to their doctors.
```rust
fn add_date(provider_id: String, department_name: String, doctor_name: String, date: String) -> Result <(), String>
```
### &#9745; Adding Times
Providers can add appointment times to their appointment dates.
```rust
fn add_time(provider_id: String, department_name: String, doctor_name: String, date: String, time: String) -> Result <(), String>
```
### &#9745; Getting Patient Info
Providers can get patient info.
```rust
fn get_patient_info(patient_id: String) -> Option<String>
```
### &#9745; Adding Disease
Providers can add disease to patients' health data.
```rust
fn add_disease(user_id: String, disease_name: String) -> Result<(), String>
```
### &#9745; Adding Allergy
Providers can add allergy to patients' health data.
```rust
fn add_allergy(user_id: String, allergy_name: String) -> Result<(), String>{
```
### &#9745; Adding Medications
Providers can add medications to patients' health data.
```rust
fn add_medication(user_id: String, medication_name: String) -> Result<(), String>{
```
### &#9745; Editing Provider Info
Providers can edit their info.
```rust
fn edit_provider_info(provider_id: String, provider_name: String, provider_location: String) -> Result<(), String>
```

### &#9745; Listings
Providers have access to the same listing functions available to users.
## Admin Functionalities
In the DMS, we require a decision-maker to determine who will become a provider. Therefore, we need an admin user who will fulfill this role. Security checks for this role will be improved. 
### &#9745; Listing Signed Users
Admins can list signed users.
```rust
fn list_signed_users() -> Vec<String>
```
### &#9745; Listing Active Sessions
Admins can list active sessions, logged in users.
```rust
fn list_active_sessions() -> Vec<String> 
```
### &#9745; Controlling If User Is Logged In
Admins can check and see if user is logged in.
```rust
fn is_logged_in(user_id: String) -> bool 
```
### &#9745; Setting Providers
Admins can set providers. When selected, user will be removed from the user list and added to the provider list.
```rust
fn set_provider(user_id: String) -> Result<(), String>
```
### &#9745; Listing Providers
Admins can list current providers on DMS.
```rust
fn list_providers() -> Vec<String> 
```
### &#9745; Removing Providers
Admins can remove a provider from providers lists.
```rust
fn remove_provider(user_id: String) -> Result<(), String> 
```
## Further Moves
### &#9745;  Front-end will be developed using Svelte. 
### &#9745;  Users can view their health data within the system. 
### &#9745;  Users can view their medication data within the system. 
### &#9745;  Providers can update and view patients' health data within the system.  
### &#9745;  Providers can update and view patients' medication data within the system. 
### &#9745;  Providers should be accessible by their name. 
### [ ] Since one of the aims of DMS is to make medical data valuable, accessing and viewing a patient's information must require certain measures. Therefore, the next development goal is to adjust a grant system for this purpose.
### [ ] Providers may have sub-modules for their doctors, etc., so doctors can perform data management and updates with separate accounts that are linked to that provider. This may necessitate an account linking system.
### [ ] Old-school authentication system will be removed. Instead, users will utilize Internet Identity to connect to the DMS.
### [ ] The functionality for mapping to see the nearest DMS providers, etc., will be added.
### [ ] General functionality of the DMS will be enhanced. Health data sections will be expanded, and provider management will be improved.
