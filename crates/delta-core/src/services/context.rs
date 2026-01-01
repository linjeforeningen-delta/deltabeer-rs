use crate::ports::{AdminRepo, Clock, TokenRepo, TransactionRepo, UserRepo};

pub struct Ctx<'a> {
    pub users: &'a dyn UserRepo,
    pub transactions: &'a dyn TransactionRepo,
    pub admins: &'a dyn AdminRepo,
    pub tokens: &'a dyn TokenRepo,
    pub clock: &'a dyn Clock,
}

pub trait HasUsers {
    fn users(&self) -> &dyn UserRepo;
}

impl HasUsers for Ctx<'_> {
    fn users(&self) -> &dyn UserRepo {
        self.users
    }
}

pub trait HasTransactions {
    fn transactions(&self) -> &dyn TransactionRepo;
}

impl HasTransactions for Ctx<'_> {
    fn transactions(&self) -> &dyn TransactionRepo {
        self.transactions
    }
}
pub trait HasAdmins {
    fn admins(&self) -> &dyn AdminRepo;
}

impl HasAdmins for Ctx<'_> {
    fn admins(&self) -> &dyn AdminRepo {
        self.admins
    }
}

pub trait HasTokens {
    fn tokens(&self) -> &dyn TokenRepo;
}

impl HasTokens for Ctx<'_> {
    fn tokens(&self) -> &dyn TokenRepo {
        self.tokens
    }
}

pub trait HasClock {
    fn clock(&self) -> &dyn Clock;
}

impl HasClock for Ctx<'_> {
    fn clock(&self) -> &dyn Clock {
        self.clock
    }
}
