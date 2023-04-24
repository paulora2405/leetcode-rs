struct BrowserHistory {
    history: Vec<String>,
    index: usize,
}

impl BrowserHistory {
    fn new(homepage: String) -> Self {
        Self {
            history: vec![homepage],
            index: 0,
        }
    }

    fn visit(&mut self, url: String) {
        if self.index < self.history.len() - 1 {
            self.history.truncate(self.index + 1);
        }
        self.history.push(url);
        self.index += 1;
    }
    fn back(&mut self, steps: i32) -> String {
        self.index = self.index.checked_sub(steps as usize).unwrap_or_default();
        self.history[self.index].clone()
    }

    fn forward(&mut self, steps: i32) -> String {
        self.index = (steps as usize + self.index).min(self.history.len() - 1);
        self.history[self.index].clone()
    }
}

/*
 * Your BrowserHistory object will be instantiated and called as such:
 * let obj = BrowserHistory::new(homepage);
 * obj.visit(url);
 * let ret_2: String = obj.back(steps);
 * let ret_3: String = obj.forward(steps);
 */
/*
BrowserHistory browserHistory = new BrowserHistory("leetcode.com");
browserHistory.visit("google.com");       // You are in "leetcode.com". Visit "google.com"
browserHistory.visit("facebook.com");     // You are in "google.com". Visit "facebook.com"
browserHistory.visit("youtube.com");      // You are in "facebook.com". Visit "youtube.com"
browserHistory.back(1);                   // You are in "youtube.com", move back to "facebook.com" return "facebook.com"
browserHistory.back(1);                   // You are in "facebook.com", move back to "google.com" return "google.com"
browserHistory.forward(1);                // You are in "google.com", move forward to "facebook.com" return "facebook.com"
browserHistory.visit("linkedin.com");     // You are in "facebook.com". Visit "linkedin.com"
browserHistory.forward(2);                // You are in "linkedin.com", you cannot move forward any steps.
browserHistory.back(2);                   // You are in "linkedin.com", move back two steps to "facebook.com" then to "google.com". return "google.com"
browserHistory.back(7);                   // You are in "google.com", you can move back only one step to "leetcode.com". return "leetcode.com"
 */

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn example() {
        let mut browser_history = BrowserHistory::new("leetcode.com".to_string());

        browser_history.visit("google.com".to_string());
        browser_history.visit("facebook.com".to_string());
        browser_history.visit("youtube.com".to_string());
        assert_eq!("facebook.com", browser_history.back(1));
        assert_eq!("google.com", browser_history.back(1));
        assert_eq!("facebook.com", browser_history.forward(1));
        browser_history.visit("linkedin.com".to_string());
        assert_eq!("linkedin.com", browser_history.forward(2));
        assert_eq!("google.com", browser_history.back(2));
        assert_eq!("leetcode.com", browser_history.back(7));
    }
}
