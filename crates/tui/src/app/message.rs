#[derive(Debug)]
pub(crate) enum Message {
    Quit,

    OpenHome,
    OpenUsers,
    OpenTransactions,
    OpenStats,

    CardScanned(String),
    CloseDialog,
}