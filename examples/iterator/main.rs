trait SocialNetwork {
    fn iter_group_public(&'_ self) -> UserPublicIterator<'_>;
}

#[derive(Debug, Clone, PartialEq)]
enum UserGroup {
    PUBLIC,
    PRIVATE,
}

#[derive(Debug, Clone)]
pub struct User {
    name: String,
    group: UserGroup,
}

struct Telegram {
    users: Vec<User>,
}

impl Default for Telegram {
    fn default() -> Self {
        Self {
            users: vec![
                User {
                    name: "James".into(),
                    group: UserGroup::PUBLIC,
                },
                User {
                    name: "John".into(),
                    group: UserGroup::PRIVATE,
                },
                User {
                    name: "Michael".into(),
                    group: UserGroup::PUBLIC,
                },
            ],
        }
    }
}

impl SocialNetwork for Telegram {
    fn iter_group_public(&'_ self) -> UserPublicIterator<'_> {
        UserPublicIterator {
            index: 0,
            user_collection: self,
        }
    }
}

pub struct UserPublicIterator<'a> {
    index: usize,
    user_collection: &'a Telegram,
}

impl Iterator for UserPublicIterator<'_> {
    type Item = User;
    fn next(&mut self) -> Option<Self::Item> {
        // Here is the implementation of your iteration algorithm.
        let users_public: Vec<&User> = self
            .user_collection
            .users
            .iter()
            .filter(|e| e.group == UserGroup::PUBLIC)
            .collect();

        if self.index < users_public.len() {
            let user = users_public[self.index];
            self.index += 1;
            return Some(user.clone());
        }
        None
    }
}

fn main() {
    let telegram = Telegram::default();
    println!("> Telegram user public:");
    for user in telegram.iter_group_public() {
        println!("- {}", user.name);
    }
}
