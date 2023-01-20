pub trait ErrorHandler<OkType, ErrType>
where
    ErrType: Clone,
{
    fn handle_error(self, error: ErrType) -> Result<OkType, ErrType>;
}

impl<ReplierOkType, OkType, ErrType> ErrorHandler<OkType, ErrType>
    for tokio::sync::oneshot::Sender<Result<ReplierOkType, ErrType>>
where
    ErrType: Clone,
{
    fn handle_error(self, error: ErrType) -> Result<OkType, ErrType> {
        match self.send(Err(error.clone())) {
            Ok(_) => (),
            Err(_) => log::error!("failed to reply"),
        }

        Err(error)
    }
}

#[cfg(test)]
#[test]
fn returns_error() {
    const ERROR: &str = "this is the error";

    let (sender, receiver) = tokio::sync::oneshot::channel::<Result<&str, &str>>();
    let result_error: Result<(), &str> = sender.handle_error(ERROR);

    assert_eq!(ERROR, result_error.unwrap_err());
}
