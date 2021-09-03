use std::error::Error;

pub trait OnError<T, E> {
	fn log_error(self) -> Result<T, E>;
}

impl<T, E> OnError<T, E> for Result<T, E>
where
	E: Error,
{
	fn log_error(self) -> Result<T, E> {
		self.map_err(|err| {
			eprintln!("{:?}", err);
			err
		})
	}
}
