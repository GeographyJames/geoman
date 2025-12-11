#[derive(Debug)]
pub struct UserInputDto<'a> {
    pub auth_id: &'a str,
    pub first_name: Option<&'a str>,
    pub last_name: Option<&'a str>,
    pub username: Option<&'a str>,
}
