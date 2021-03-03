pub fn hasAccess(&self, _user: AccountId) -> bool {
    self.accessList[_user] || !checkEnabled;
}

pub fn addAccess(&mut self, _user: AccountId) {
    self.onlyOwner();

    if(!self.accessList[_user]) {
        self.accessList[_user] = true;
    }
}

pub fn removeAccess(&mut self, _user: AccountId) {
    self.onlyOwner();

    if(self.accessList[_user]) {
        self.accessList[_user] = false;
    }
}

pub fn enableAccessCheck(&mut self) {
    self.onlyOwner();

    if(!self.checkEnabled) {
        self.checkEnabled = true;
    }
}

pub fn disableAccessCheck(&mut self) {
    self.onlyOwner();

    if(self.checkEnabled) {
        self.checkEnabled = false;
    }
}

fn checkAccess(&self) {
    assert!(self.hasAccess(env::predecessor_account_id()), "No access")
}

fn onlyOwner(&mut self) {
    assert_eq!(self.owner, env::predecessor_account_id(), "Only contract owner can call this method.");
}
