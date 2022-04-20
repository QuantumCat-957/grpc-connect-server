#[derive(Clone,Default,Debug)]
pub(crate) struct TimeRecord {
    start: Vec<u128>,
    end: u128,
}


impl TimeRecord {
    pub(crate) fn new() -> Self {
        Self { start: Vec::new(), end: 0 }
    }

    pub(crate) fn start(&mut self){
        self.start = Vec::new();
        self.start.push(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        );
    }

    pub(crate) fn time_cost(&mut self, tag: &str, explain: &str) -> &mut Self{
        self.end = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let delay = self.end - self.start.pop().unwrap();
        tracing::debug!("[{}] {}, time cost: {}", tag, explain, delay);
        self
    }
}
