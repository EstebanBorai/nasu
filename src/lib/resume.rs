/// Resume from a `Provider.perform` execution, which brings
/// generic details on the performance of the task for an
/// specific `Target`
pub struct Resume {
    /// Host where the task where performed. (E.g. IP Address, domain, path)
    pub host: String,
    /// Response from the target in bytes
    pub fragment: Vec<u8>,
    /// Flag which determines if the task were perfomed with success
    pub is_success: bool,
    /// Moment when the task were performed
    pub issued_at: u64,
}
