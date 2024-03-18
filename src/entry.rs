use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct Entry {
    #[tabled(rename = "Application / Website")]
    pub application: String,
    #[tabled(rename = "Username / Email")]
    pub username: String,
    #[tabled(rename = "Password")]
    pub password: String,
}
