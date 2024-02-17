use candid::{CandidType, Decode, Deserialize, Encode};
//use ic_cdk::api::management_canister::main;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
//use std::io::Read;
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
const MAX_VALUE_SIZE: u32 = 100;

#[derive(CandidType, Deserialize)]
struct Student {
    id: u64,
    name: String,
    age: u32,
    class: String,
}
//Implement storable trait
impl Storable for Student {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]>{  //Converts data to bytes
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self{  //Convert bytes to data
        (Decode!(bytes.as_ref(), Self)).unwrap()
    }
}

//Implement bounded trait
impl BoundedStorable for Student {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    
    static STUDENT_MAP: RefCell<StableBTreeMap<u64, Student, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
    ));

    static PARTICIPATION_PERCENTAGE_MAP: RefCell<StableBTreeMap<u64, u64, Memory>> = RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
    ));
}

#[ic_cdk_macros::update]
fn insert_student(id: u64, value: Student) -> Option<Student> {
    STUDENT_MAP.with(|p| p.borrow_mut().insert(id, value))
}

#[ic_cdk_macros::query]
fn get_student(id: u64) -> Option<Student> {
    STUDENT_MAP.with(|p| p.borrow().get(&id))
}

//import management canister
use ic_cdk::api::management_canister::http_request::{http_request, CanisterHttpRequestArgument, HttpMethod};

// update method using the HTTPS outcalls feature
#[ic_cdk::update]
async fn get_song_titles() -> String {

    // setup the URL for the HTTP GET request
    let url = "https://65b8d3a6b71048505a898b21.mockapi.io/get_song_titles".to_string();

    // prepare headers for the system http_request call
    let request_headers = vec![];

    // setup the HTTP request arguments
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    // make the HTTPS request and wait for the response
    // 1_603_079_600 cycles are needed for this operation
    match http_request(request, 1_603_079_600).await {
        Ok((response,)) => { 
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err((code, message)) => {
            format!(
                "The http_request resulted in an error. Code: {:?}, Message: {}",
                code, message
            )
        }
    }
}

// get song titles using id from the mock API
#[ic_cdk::update]
async fn get_song_titles_by_id(id: String) -> String {
    // setup the URL for the HTTP GET request
    let url = format!("https://65b8d3a6b71048505a898b21.mockapi.io/get_song_titles/{}", id);

    // prepare headers for the system http_request call
    let request_headers = vec![];

    // setup the HTTP request arguments
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    // make the HTTPS request and wait for the response
    // 1_603_079_600 cycles are needed for this operation
    match http_request(request, 1_603_079_600).await {
        Ok((response,)) => { 
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err((code, message)) => {
            format!(
                "The http_request resulted in an error. Code: {:?}, Message: {}",
                code, message
            )
        }
    }
}

// get song tiles using song name from the mock API
#[ic_cdk::update]
async fn get_song_titles_by_name(name: String) -> String {
    // setup the URL for the HTTP GET request
    let url = format!("https://65b8d3a6b71048505a898b21.mockapi.io/get_song_titles?name={}", name);

    // prepare headers for the system http_request call
    let request_headers = vec![];

    // setup the HTTP request arguments
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    // make the HTTPS request and wait for the response
    // 1_603_079_600 cycles are needed for this operation
    match http_request(request, 1_603_079_600).await {
        Ok((response,)) => { 
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err((code, message)) => {
            format!(
                "The http_request resulted in an error. Code: {:?}, Message: {}",
                code, message
            )
        }
    }
}

// get song titles using artist name from the mock API
#[ic_cdk::update]
async fn get_song_titles_by_artist(artist: String) -> String {
    // setup the URL for the HTTP GET request
    let url = format!("https://65b8d3a6b71048505a898b21.mockapi.io/get_song_titles?artist={}", artist);

    // prepare headers for the system http_request call
    let request_headers = vec![];

    // setup the HTTP request arguments
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    // make the HTTPS request and wait for the response
    // 1_603_079_600 cycles are needed for this operation
    match http_request(request, 1_603_079_600).await {
        Ok((response,)) => { 
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err((code, message)) => {
            format!(
                "The http_request resulted in an error. Code: {:?}, Message: {}",
                code, message
            )
        }
    }
}

// get song titles using genre from the mock API
#[ic_cdk::update]
async fn get_song_titles_by_genre(genre: String) -> String {
    // setup the URL for the HTTP GET request
    let url = format!("https://65b8d3a6b71048505a898b21.mockapi.io/get_song_titles?genre={}", genre);

    // prepare headers for the system http_request call
    let request_headers = vec![];

    // setup the HTTP request arguments
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    // make the HTTPS request and wait for the response
    // 1_603_079_600 cycles are needed for this operation
    match http_request(request, 1_603_079_600).await {
        Ok((response,)) => { 
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err((code, message)) => {
            format!(
                "The http_request resulted in an error. Code: {:?}, Message: {}",
                code, message
            )
        }
    }
}
