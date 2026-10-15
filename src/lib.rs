pub fn add_command(history: &mut std::collections::VecDeque<String>, command: String, k: usize) {
    history.push_back(command);
    if history.len() > k {
        history.pop_front();
    }
}