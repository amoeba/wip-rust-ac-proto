use std::fmt;

/// Statistics for a category of parsed items
#[derive(Debug, Clone, Copy, Default)]
pub struct CategoryStats {
    pub total: u64,
    pub ok: u64,
    pub errors: u64,
}

impl CategoryStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment_total(&mut self) {
        self.total += 1;
    }

    pub fn increment_ok(&mut self) {
        self.ok += 1;
    }

    pub fn increment_error(&mut self) {
        self.errors += 1;
    }

    pub fn error_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.errors as f64 / self.total as f64) * 100.0
        }
    }

    pub fn ok_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.ok as f64 / self.total as f64) * 100.0
        }
    }
}

impl fmt::Display for CategoryStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "total: {:<8} ok: {:<8} errors: {:<8} ({:.1}% ok)",
            self.total, self.ok, self.errors, self.ok_rate()
        )
    }
}

/// Comprehensive parsing statistics
#[derive(Debug, Clone, Default)]
pub struct ParsingStats {
    pub files: CategoryStats,
    pub packets: CategoryStats,
    pub fragments: CategoryStats,
    pub messages: CategoryStats,
    pub weenies: CategoryStats,
}

impl ParsingStats {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get all categories as a list of (name, stats) tuples
    pub fn categories(&self) -> Vec<(&'static str, &CategoryStats)> {
        vec![
            ("Files", &self.files),
            ("Packets", &self.packets),
            ("Fragments", &self.fragments),
            ("Messages", &self.messages),
            ("Weenies", &self.weenies),
        ]
    }

    /// Get mutable access to a category by name
    pub fn category_mut(&mut self, name: &str) -> Option<&mut CategoryStats> {
        match name {
            "files" => Some(&mut self.files),
            "packets" => Some(&mut self.packets),
            "fragments" => Some(&mut self.fragments),
            "messages" => Some(&mut self.messages),
            "weenies" => Some(&mut self.weenies),
            _ => None,
        }
    }

    /// Print a formatted statistics report
    pub fn print_report(&self) {
        println!("\n{:═^70}", " PARSING STATISTICS REPORT ");
        println!("{:<20} {:<50}", "Category", "Statistics");
        println!("{:-<70}", "");

        for (name, stats) in self.categories() {
            println!("{:<20} {}", name, stats);
        }

        println!("{:═<70}", "");

        // Print summary line with overall error rate
        let total_items = self.files.total
            + self.packets.total
            + self.fragments.total
            + self.messages.total
            + self.weenies.total;
        let total_errors = self.files.errors
            + self.packets.errors
            + self.fragments.errors
            + self.messages.errors
            + self.weenies.errors;

        if total_items > 0 {
            let overall_error_rate = (total_errors as f64 / total_items as f64) * 100.0;
            println!(
                "TOTAL ITEMS: {:<10} TOTAL ERRORS: {:<10} OVERALL ERROR RATE: {:.1}%",
                total_items, total_errors, overall_error_rate
            );
        }
    }

    /// Print a compact one-line summary
    pub fn print_summary(&self) {
        println!(
            "Stats: {} files | {} packets | {} fragments | {} messages | {} weenies",
            self.files.total, self.packets.total, self.fragments.total, self.messages.total, self.weenies.total
        );
    }

    /// Get total count across all categories
    pub fn total_items(&self) -> u64 {
        self.files.total
            + self.packets.total
            + self.fragments.total
            + self.messages.total
            + self.weenies.total
    }

    /// Get total errors across all categories
    pub fn total_errors(&self) -> u64 {
        self.files.errors
            + self.packets.errors
            + self.fragments.errors
            + self.messages.errors
            + self.weenies.errors
    }

    /// Get overall error rate
    pub fn overall_error_rate(&self) -> f64 {
        let total = self.total_items();
        if total == 0 {
            0.0
        } else {
            (self.total_errors() as f64 / total as f64) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_stats() {
        let mut stats = CategoryStats::new();
        stats.increment_total();
        stats.increment_total();
        stats.increment_ok();
        stats.increment_error();

        assert_eq!(stats.total, 2);
        assert_eq!(stats.ok, 1);
        assert_eq!(stats.errors, 1);
        assert_eq!(stats.ok_rate(), 50.0);
        assert_eq!(stats.error_rate(), 50.0);
    }

    #[test]
    fn test_parsing_stats() {
        let mut stats = ParsingStats::new();
        stats.packets.increment_total();
        stats.packets.increment_ok();
        stats.messages.increment_total();
        stats.messages.increment_total();
        stats.messages.increment_ok();
        stats.messages.increment_error();

        assert_eq!(stats.total_items(), 4);
        assert_eq!(stats.total_errors(), 1);
    }
}
