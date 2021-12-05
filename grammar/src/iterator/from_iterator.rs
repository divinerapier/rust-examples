#[derive(Debug, Eq, PartialEq)]
pub struct User {
    name: String,
    age: usize,
}

impl User {
    pub fn new<S: Into<String>>(name: S, age: usize) -> Self {
        User {
            name: name.into(),
            age,
        }
    }
}

#[derive(Debug)]
pub struct Team<'u> {
    pub users: Vec<&'u User>,
}

impl<'u> FromIterator<&'u User> for Team<'u> {
    fn from_iter<T: IntoIterator<Item = &'u User>>(iter: T) -> Self {
        let users = iter.into_iter().collect::<Vec<&'u User>>();
        Team { users }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_team_from_iterator() {
        let users = vec![User::new("Tom", 2), User::new("Jon", 28)];
        let team = Team::from_iter(users.iter());
        println!("{:?}", team);
        let team = users.iter().collect::<Team>();
        println!("{:?}", team);
    }
}
