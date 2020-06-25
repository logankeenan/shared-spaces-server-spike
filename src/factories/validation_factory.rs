use std::collections::HashMap;
use validator::{ValidationErrors, ValidationErrorsKind};

pub fn validation_errors_from(validation_errors: Result<(), ValidationErrors>) -> HashMap<String, String> {
    let mut errors = HashMap::new();

    match validation_errors {
        Ok(_) => (),
        Err(e) => {
            for (field_name, validation_error_kind) in e.errors().iter() {
                match validation_error_kind {
                    ValidationErrorsKind::Field(validation_errors) => {
                        for validation_error in validation_errors {
                            let error = validation_error.clone();

                            let option1 = error.message;
                            match option1 {
                                None => {}
                                Some(mess) => {
                                    let string = mess.to_string();
                                    errors.insert(field_name.to_string(), string);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    errors
}