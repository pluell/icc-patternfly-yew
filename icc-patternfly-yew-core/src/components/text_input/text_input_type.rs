
#[derive(Clone, PartialEq)]
pub enum TextInputType
{
    Text,
    Date,
    DatetimeLocal,
    Email,
    Month,
    Number,
    Password,
    Search,
    Tel,
    Time,
    Url,
}

impl std::fmt::Display for TextInputType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            TextInputType::Text => {
                write!(f, "text")
            },
            TextInputType::Date => {
                write!(f, "date")
            },
            TextInputType::DatetimeLocal => {
                write!(f, "datetime-local")
            },
            TextInputType::Email => {
                write!(f, "email")
            },
            TextInputType::Month => {
                write!(f, "month")
            },
            TextInputType::Number => {
                write!(f, "number")
            },
            TextInputType::Password => {
                write!(f, "password")
            },
            TextInputType::Search => {
                write!(f, "search")
            },
            TextInputType::Tel => {
                write!(f, "tel")
            },
            TextInputType::Time => {
                write!(f, "time")
            },
            TextInputType::Url => {
                write!(f, "url")
            },
        }
    }
}
