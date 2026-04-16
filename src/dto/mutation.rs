use serde::{Deserialize, Serialize};

/// Field-level validation error returned by JSON mutation endpoints.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ApiFieldErrorDto {
    pub field: String,
    pub message: String,
}

/// Successful JSON mutation response.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ApiMutationSuccessDto {
    pub message: String,
    pub redirect_to: Option<String>,
}

/// Failed JSON mutation response with optional field-level errors.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ApiMutationErrorDto {
    pub message: String,
    pub field_errors: Vec<ApiFieldErrorDto>,
}

impl Default for ApiMutationErrorDto {
    fn default() -> Self {
        Self {
            message: "Ошибка валидации формы.".to_string(),
            field_errors: Vec::new(),
        }
    }
}
