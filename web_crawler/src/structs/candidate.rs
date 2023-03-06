pub struct Candidate {
    fullname: String,
    linkedin: String,
    message: String,
}

impl Candidate {
    pub fn new(fullname: String, linkedin: String, message: String) -> Self {
        Candidate {
            fullname,
            linkedin,
            message,
        }
    }
}