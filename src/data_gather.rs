use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Record {
    wrong_codes: Vec<String>,
    errors: Vec<String>,
    right_code: String,
    code_path: PathBuf,
}

impl Record {
    pub fn empty() -> Self {
        Record {
            wrong_codes: Vec::new(),
            errors: Vec::new(),
            right_code: String::new(),
            code_path: PathBuf::default(),
        }
    }

    pub fn to_json(&self) -> Option<String> {
        assert_eq!(self.errors.len(), self.wrong_codes.len());
        assert!(!self.right_code.is_empty());
        let mut out = String::new();
        // let num = self.errors.len();

        let it = self.wrong_codes.iter().zip(self.errors.iter());
        // let v: Vec<(&String, &String)> = it.collect();
        // return Some(format_args!("{:?}", v).to_string().to_owned());
        for (wrong_code, error) in it {
            if error.is_empty() {
                continue;
            }
            out.push_str(
                format!(
                    "{{\"wrong_code\": \"{}\", \"error\": \"{}\", \"right_code\": \"{}\"}}\n",
                    wrong_code.replace("\n", "\\n").replace("\t", "\\n").replace("\"", "\\\""),
                    error.replace("\n", "\\n").replace("\t", "\\n").replace("\"", "\\\""),
                    self.right_code.replace("\n", "\\n").replace("\t", "\\n").replace("\"", "\\\"")
                )
                .as_str(),
            );

            // if i < num - 1 {
            //     out.push_str(",\n");
            // }
        }

        if out.is_empty() {
            None
        } else {
            Some(out)
        }
    }

    fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn check_file(&self, path: &PathBuf) -> bool {
        self.code_path.file_name() == path.file_name()
    }

    fn read_wrong_code(&mut self) {
        let code = fs::read(&self.code_path).unwrap();
        self.wrong_codes.push(String::from_utf8(code).unwrap());
    }

    pub fn read_right_code(&mut self) {
        let code = fs::read(&self.code_path).unwrap();
        self.right_code = String::from_utf8(code).unwrap();
    }

    pub fn set_error(&mut self, error: &String) {
        if !self.errors.iter().any(|previous| previous.eq(error)) {
            self.errors.push(error.clone());
            self.read_wrong_code();
        }
    }

    pub fn reset_path(&mut self, code_path: &PathBuf) {
        if !self.code_path.file_name().eq(&code_path.file_name()) {
            self.clear();
        }
        self.code_path = code_path.clone();
    }

    pub fn clear(&mut self) {
        self.code_path = PathBuf::default();
        self.right_code.clear();
        self.wrong_codes.clear();
        self.errors.clear();
    }
}

pub struct DataGather {
    dump_path: PathBuf,
}

impl DataGather {
    pub fn new(path: PathBuf) -> Self {
        DataGather {
            dump_path: path.clone(),
        }
    }

    pub fn push(&self, record: Record) {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .write(true)
            .open(&self.dump_path)
            .unwrap();

        if !record.is_empty() {
            if let Some(data) = record.to_json() {
                file.write(data.as_bytes()).unwrap();
                file.write(b"\n").unwrap();
            }
        }
    }
}
