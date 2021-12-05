use super::from_iterator::{Team, User};
use std::slice::Iter;

impl<'a> Iterator for Team<'a> {
    type Item = &'a User;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

pub struct TeamIter<I> {
    users: I,
}

impl<'u> Team<'u> {
    pub fn iter(&'u self) -> TeamIter<impl Iterator<Item = &'u User>> {
        let users = self.users.iter().map(|user| *user);
        TeamIter { users }
    }
}

impl<'u, I> Iterator for TeamIter<I>
where
    I: Iterator<Item = &'u User>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.users.next()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let users = vec![User::new("Tom", 2), User::new("Jon", 28)];
        let team = users.iter().collect::<Team>();
        let mut team_iter = team.iter();
        println!("{:?}", team);
        let u = team_iter.next();
        assert_eq!(u, Some(&User::new("Tom", 2)));
        let u = team_iter.next();
        assert_eq!(u, Some(&User::new("Jon", 28)));
        let u = team_iter.next();
        assert_eq!(u, None);
    }
}
