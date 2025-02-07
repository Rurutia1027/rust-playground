use futures::{stream, Stream, StreamExt, TryStreamExt};
use std::char::MAX;
use std::error::Error;
use std::{
    io,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::mpsc;
use tokio::task;
use tokio_stream::wrappers::ReceiverStream;

// Function to sum items using next()
async fn sum_with_next(mut stream: Pin<Box<Counter>>) -> i32 {
    let mut sum = 0;
    while let Some(item) = stream.next().await {
        sum += item;
    }
    sum as i32
}

// Function to sum items using try_next()
async fn sum_with_try_next(
    mut stream: Pin<&mut dyn Stream<Item = Result<i32, io::Error>>>,
) -> Result<i32, Box<dyn Error>> {
    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {
        sum += item;
    }
    Ok(sum)
}

// Function to process items concurrently
async fn jump_around(
    mut stream: Pin<&mut dyn Stream<Item = Result<u8, io::Error>>>,
) -> Result<(), io::Error> {
    const MAX_CONCURRENT_JUMPERS: usize = 100;
    stream
        .try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
            jump_n_times(num).await?;
            report_n_jumps(num).await?;
            Ok(())
        })
        .await
}

async fn jump_n_times(num: u8) -> Result<(), io::Error> {
    println!("Jumping {} times!", num);
    Ok(())
}
async fn report_n_jumps(num: u8) -> Result<(), io::Error> {
    println!("Reported {} jumps", num);
    Ok(())
}

// stream iterator chain

// stream pin dyn explanation

// Custom stream implementation
struct Counter {
    count: usize,
    max: usize,
}

impl Stream for Counter {
    type Item = usize;

    fn poll_next(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        if self.count < self.max {
            self.count += 1;
            Poll::Ready(Some(self.count))
        } else {
            Poll::Ready(None)
        }
    }
}

// Function to demo sending and receiving with mpsc channel
async fn send_recv() {
    // apply for a piece of space with buffer size = 10
    const BUFFER_SIZE: usize = 10;

    // create a communication channel with buffer writer/tx and buffer reader/rx
    // to manipulate the buffered items in the applied memory space
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);

    // create a task pool inside run two operations in async way
    // async task 1 take tx the buffer writer write a value of 1 to the applied piece of memory space
    // async task 2 take tx the buffer write handler write a value of 2 to the space
    // then release the buffer handler via drop(tx)
    task::spawn(async move {
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        drop(tx);
    });

    let mut stream = ReceiverStream::new(rx);
    // here we use the buffer read handler rx to read data from the shared space
    assert_eq!(stream.next().await, Some(1));
    assert_eq!(stream.next().await, Some(2));

    // no more data, next element fetch by the buffer reader handler should be None
    assert!(stream.next().await.is_none());
}

#[cfg(test)]
mod tests {
    use super::*;

    // test-1: test sum_with_next
    #[tokio::test]
    pub async fn test_sum_with_next() {
        let counter = Counter { count: 0, max: 5 };
        let sum = sum_with_next(Box::pin(counter)).await;
        // 0 + 1 + 2 + 3 + 4 + 5 = 15
        assert_eq!(sum, 15);
    }

    #[tokio::test]
    pub async fn test_jump_stream() {
        let jump_stream = stream::iter(vec![Ok(1), Ok(2), Ok(3), Ok(4), Ok(5)]);
        jump_around(Box::pin(jump_stream).as_mut()).await.unwrap();
    }

    #[tokio::test]
    pub async fn test_multi_threads_stream_processing() {
        let BUFFER_SIZE: usize = 10;
        let (tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);
        let handles: Vec<_> = (0..5)
            .map(|i| {
                // clone multiple buffer writer handlers
                // and each handler passing its lambda index value to the shared memory buffer
                let tx_clone = tx.clone();
                task::spawn(async move {
                    tx_clone.send(i).await.unwrap();
                })
            })
            .collect();

        // we need to close the write channel handler here
        // the above cloned writer handler will be released one get out of the scope of the thread
        // but if we not drop/release the tx explicitly
        // rx will be pending in the while loop because the write channel not closed
        // rx take this as a signal that there gonna send some other content from the writer/tx side in the future
        // if so this test will result in while looping and block the complete test
        drop(tx);

        // traverse write handlers and execute handler's closure logic -> write its own index to the buffer space
        for handler in handles {
            handler.await.unwrap();
        }

        let mut received_sum = 0;
        // here we use only one read handler to waiting for received values from the buffer
        // the looping condition is read handler received value not None

        // we need to use ReceiverStream to wrap the buffer read handler
        let mut recv_stream_handler = ReceiverStream::new(rx);

        while let Some(value) = recv_stream_handler.next().await {
            // since data values are passed in async threads
            // value coming may not bb in order
            println!("received value is {value}");
            received_sum += value;
        }

        assert_eq!(received_sum, 10);
    }
}
