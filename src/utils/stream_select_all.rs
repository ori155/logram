use futures::{stream, Stream};
use std::iter::IntoIterator;

pub fn stream_select_all<I, T, E>(streams: I) -> Box<Stream<Item = T, Error = E> + Send>
where
    I: IntoIterator,
    I::Item: Stream<Item = T, Error = E> + Send + 'static,
    T: Send + 'static,
    E: Send + 'static,
{
    let init = Box::new(stream::empty()) as Box<Stream<Item = T, Error = E> + Send>;

    streams
        .into_iter()
        .fold(init, |all, part| Box::new(all.select(part)))
}

#[cfg(test)]
mod tests {
    use futures::{stream, Stream};

    use super::stream_select_all;

    #[test]
    fn main() {
        let a = stream::iter_ok::<_, ()>(vec![1, 2]);
        let b = stream::iter_ok::<_, ()>(vec![3, 4]);

        let streams = vec![a, b];
        let mut stream = stream_select_all(streams).wait();

        assert_eq!(stream.next(), Some(Ok(1)));
        assert_eq!(stream.next(), Some(Ok(3)));
        assert_eq!(stream.next(), Some(Ok(2)));
        assert_eq!(stream.next(), Some(Ok(4)));
        assert_eq!(stream.next(), None);
    }
}
