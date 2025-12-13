use strum::Display;

#[derive(Clone, Debug, Display, PartialEq)]
pub enum Action {
    //
    #[strum(to_string = "Create")]
    Create,

    #[strum(to_string = "Delete")]
    Delete,

    #[strum(to_string = "Edit")]
    Edit,

    #[strum(to_string = "View")]
    View,
}
