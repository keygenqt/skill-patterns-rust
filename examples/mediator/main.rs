use std::{error::Error};

#[derive(PartialEq, Clone)]
enum Role {
    ADMIN,
    USER,
    GUEST,
}

mod out {
    use std::sync::{LazyLock, Mutex};

    static ROLE: LazyLock<Mutex<RoleState>> = LazyLock::new(|| Mutex::new(RoleState::default()));

    pub struct RoleState {
        role: crate::Role
    }

    impl Default for RoleState {
        fn default() -> Self {
            Self { role: crate::Role::GUEST }
        }
    }

    impl RoleState {
        pub fn update(role: crate::Role) {
            ROLE.lock().unwrap().role = role
        }

        pub fn role() -> crate::Role {
            ROLE.lock().unwrap().role.clone()
        }
    }
}

trait Mediator<T: Sender> {
    fn check(&self, data: &T) -> Result<(), Box<dyn Error>>;
}

trait Sender {
    fn allow(&self) -> Vec<Role>;
    fn emit(&self, auth_check: bool) -> Result<(), Box<dyn Error>>;
}

#[derive(Default)]
struct AuthData {}

impl AuthData {
    fn auth(role: Role) {
        out::RoleState::update(role)
    }

    fn role() -> Role {
        out::RoleState::role()
    }
}

impl <T: Sender>Mediator<T> for AuthData {
    fn check(&self, data: &T) -> Result<(), Box<dyn Error>> {
        if Self::role() == Role::GUEST {
            return Err("Guest is not allowed to send mailings!")?;
        }
        if data.allow().contains(&Self::role()) {
            return data.emit(false);
        }
        Err("Role mismatch, unable to send!")?
    }
}

struct SenderUser<'a> {
    mediator: &'a dyn Mediator<Self>
}

impl <'a>SenderUser<'a> {
    fn new(mediator: &'a dyn Mediator<Self>) -> Self {
        Self { mediator }
    }
}

impl <'a>Sender for SenderUser<'a> {
    fn allow(&self) -> Vec<Role> {
        vec![Role::USER, Role::ADMIN]
    }

    fn emit(&self, auth_check: bool) -> Result<(), Box<dyn Error>> {
        if !auth_check {
            // Send data...
            Ok(())
        } else {
            // Mediator check auth state
            self.mediator.check(&self)
        }
    }
}

struct SenderAdmin<'a> {
    mediator: &'a dyn Mediator<Self>
}

impl <'a>SenderAdmin<'a> {
    fn new(mediator: &'a dyn Mediator<Self>) -> Self {
        Self { mediator }
    }
}

impl <'a>Sender for SenderAdmin<'a> {
    fn allow(&self) -> Vec<Role> {
        vec![Role::ADMIN]
    }

    fn emit(&self, auth_check: bool) -> Result<(), Box<dyn Error>> {
        if !auth_check {
            // Send data...
            Ok(())
        } else {
            // Mediator check auth state
            self.mediator.check(&self)
        }
    }
}

fn main() {
    // Default Role::GUEST
    let auth = AuthData::default();

    // Create senders
    let sender_user = SenderUser::new(&auth);
    let sender_admin = SenderAdmin::new(&auth);

    println!("== Role::GUEST ===============");

    // Emit data user
    match sender_user.emit(true) {
        Ok(_) => println!("USER: Send data successfully!"),
        Err(err) => println!("{}", err),
    }

    // Emit data admin
    match sender_admin.emit(true) {
        Ok(_) => println!("ADMIN: Send data successfully!"),
        Err(err) => println!("{}", err),
    }

    AuthData::auth(Role::USER);

    println!("== Role::USER ===============");

    // Emit data user
    match sender_user.emit(true) {
        Ok(_) => println!("USER: Emit data successfully!"),
        Err(err) => println!("{}", err),
    }

    // Emit data admin
    match sender_admin.emit(true) {
        Ok(_) => println!("ADMIN: Send data successfully!"),
        Err(err) => println!("{}", err),
    }

    AuthData::auth(Role::ADMIN);

    println!("== Role::ADMIN ===============");

    // Emit data user
    match sender_user.emit(true) {
        Ok(_) => println!("USER: Send data successfully!"),
        Err(err) => println!("{}", err),
    }

    // Emit data admin
    match sender_admin.emit(true) {
        Ok(_) => println!("ADMIN: Send data successfully!"),
        Err(err) => println!("{}", err),
    }

}
