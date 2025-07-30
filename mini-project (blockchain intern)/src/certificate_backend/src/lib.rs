use candid::{CandidType, Deserialize};
use ic_cdk::{query, update};
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Clone, CandidType, Deserialize)]
pub struct Certificate {
    pub student_name: String,
    pub course_name: String,
    pub issue_date: u64,
    pub issuer: String,
}

thread_local! {
    static CERTIFICATES: RefCell<HashMap<Vec<u8>, Certificate>> = RefCell::new(HashMap::new());
}

#[update]
fn issue_certificate(student_name: String, course_name: String) -> Vec<u8> {
    let issue_date = ic_cdk::api::time();
    let issuer = ic_cdk::caller().to_text();
    let cert = Certificate {
        student_name,
        course_name,
        issue_date,
        issuer,
    };

    let key = ic_cdk::api::hash::sha256(&format!("{:?}", cert).as_bytes());
    CERTIFICATES.with(|store| store.borrow_mut().insert(key.clone(), cert));
    key
}

#[query]
fn verify_certificate(hash: Vec<u8>) -> Option<Certificate> {
    CERTIFICATES.with(|store| store.borrow().get(&hash).cloned())
}

#[query]
fn get_all_hashes() -> Vec<Vec<u8>> {
    CERTIFICATES.with(|store| store.borrow().keys().cloned().collect())
}
