use {lumos_entry::entry::EntrySummary, lumos_sdk::clock::Slot, std::sync::Arc};

pub trait EntryNotifier {
    fn notify_entry(
        &self,
        slot: Slot,
        index: usize,
        entry: &EntrySummary,
        starting_transaction_index: usize,
    );
}

pub type EntryNotifierArc = Arc<dyn EntryNotifier + Sync + Send>;
