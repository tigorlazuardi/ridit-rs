use std::error::Error;

pub trait OnError<T, E>
where
	E: Error,
{
	fn on_error<F>(self, func: F) -> Result<T, E>
	where
		F: Fn(&E);

	fn log_error(self) -> Result<T, E>;
}

impl<T, E> OnError<T, E> for Result<T, E>
where
	E: Error,
{
	fn on_error<F>(self, func: F) -> Result<T, E>
	where
		F: Fn(&E),
	{
		self.map_err(|err| {
			func(&err);
			err
		})
	}

	fn log_error(self) -> Result<T, E> {
		self.map_err(|err| {
			eprintln!("{}", err);
			err
		})
	}
}
