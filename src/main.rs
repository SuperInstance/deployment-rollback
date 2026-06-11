use std::time::SystemTime;

#[derive(Debug)]
enum DeployState { Running, RolledBack, Failed }

struct Deployment {
    id: String,
    version: String,
    state: DeployState,
    created_at: SystemTime,
}

impl Deployment {
    fn rollback(&mut self, reason: &str) {
        println!("Rolling back {} ({}) — reason: {}", self.id, self.version, reason);
        self.state = DeployState::RolledBack;
    }
}

fn main() {
    let mut dep = Deployment { id: "dep-001".into(), version: "v2.3.1".into(), state: DeployState::Running, created_at: SystemTime::now() };
    dep.rollback("error rate exceeded 5% threshold");
    println!("Deployment state: {:?}", dep.state);
}
