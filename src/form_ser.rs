pub(crate) use reqwest::multipart::Form;

pub trait FormSer {
    fn serialize(&self, key: String, form: Form) -> Form;
}

impl FormSer for bool {
    fn serialize(&self, key: String, form: Form) -> Form {
        form.text(key, self.to_string())
    }
}

impl FormSer for i64 {
    fn serialize(&self, key: String, form: Form) -> Form {
        form.text(key, self.to_string())
    }
}

impl FormSer for f64 {
    fn serialize(&self, key: String, form: Form) -> Form {
        form.text(key, self.to_string())
    }
}

impl FormSer for String {
    fn serialize(&self, key: String, form: Form) -> Form {
        form.text(key, self.to_string())
    }
}

impl<T: FormSer> FormSer for Vec<T> {
    fn serialize(&self, key: String, mut form: Form) -> Form {
        for (i, elem) in self.iter().enumerate() {
            form = elem.serialize(format!("{}[{}]", key, i), form);
        }

        return form;
    }
}

impl<T: FormSer> FormSer for Option<T> {
    fn serialize(&self, key: String, form: Form) -> Form {
        if let Some(s) = self.as_ref() {
            s.serialize(key, form)
        } else {
            form
        }
    }
}

impl<T: FormSer> FormSer for Box<T> {
    fn serialize(&self, key: String, form: Form) -> Form {
        self.as_ref().serialize(key, form)
    }
}
