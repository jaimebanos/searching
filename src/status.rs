pub struct StatusProccess {
    status: String,
    count_download: u32,
    count_errors: u32,
}

impl StatusProccess {
    pub fn new() -> StatusProccess {
        StatusProccess {
            status: String::from(""),
            count_download: 0,
            count_errors: 0,
        }
    }

    pub fn update(&mut self, status: String) {
        self.status = status;
    }

    pub fn update_count_download(&mut self) {
        self.count_download += 1;
    }

    pub fn update_count_errors(&mut self) {
        self.count_errors += 1;
    }

    pub fn read_to_string(&self) -> String {
        format!(
            "Status: {}\nCount download: {}\nCount errors: {}",
            self.status, self.count_download, self.count_errors
        )
    }
}
