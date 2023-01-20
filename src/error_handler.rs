pub trait ErrorHandler<OkType, ErrType>
where
    ErrType: Clone,
{
    fn handle_error(self, error: ErrType) -> Result<OkType, ErrType>;
}

impl<OkType, ErrType> ErrorHandler<OkType, ErrType>
    for tokio::sync::oneshot::Sender<Result<OkType, ErrType>>
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
