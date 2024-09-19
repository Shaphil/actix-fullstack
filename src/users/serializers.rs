use crate::users::models::UserRequest;
use actix_web::web::Json;
use entity::user::ActiveModel as User;
use sea_orm::ActiveValue;

pub struct UserSerializer {
    pub data: Json<UserRequest>,
}

impl UserSerializer {
    pub fn serialize(&self) -> User {
        let is_active = self.is_active();
        let is_admin = self.is_admin();
        let is_superadmin = self.is_superadmin();

        let user = User {
            username: ActiveValue::Set(self.data.username.clone()),
            firstname: ActiveValue::Set(self.data.firstname.clone()),
            lastname: ActiveValue::Set(self.data.lastname.clone()),
            email: ActiveValue::Set(String::from(self.data.email.clone())),
            password: ActiveValue::Set(self.data.password.clone()),
            is_active: ActiveValue::Set(Option::from(is_active)),
            last_login: ActiveValue::Set(Option::from(self.data.last_login)),
            date_joined: ActiveValue::Set(Option::from(self.data.date_joined)),
            created_at: ActiveValue::Set(Option::from(self.data.created_at)),
            updated_at: ActiveValue::Set(Option::from(self.data.updated_at)),
            is_admin: ActiveValue::Set(Option::from(is_admin)),
            is_superadmin: ActiveValue::Set(Option::from(is_superadmin)),
            ..Default::default()
        };

        user
    }

    fn is_active(&self) -> bool {
        match self.data.is_active {
            None => false,
            Some(_) => true
        }
    }

    fn is_admin(&self) -> bool {
        match self.data.is_admin {
            None => false,
            Some(_) => true
        }
    }

    fn is_superadmin(&self) -> bool {
        match self.data.is_admin {
            None => false,
            Some(_) => true
        }
    }
}