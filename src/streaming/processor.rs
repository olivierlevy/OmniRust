// src/streaming/processor.rs

use async_trait::async_trait;
use futures_util::stream::Stream; // Corrected import
use futures_util::stream::StreamExt;
use std::pin::Pin;

/// A trait for asynchronous stream processors.
/// Takes an input stream of type `In` and produces an output stream of type `Out`.
#[async_trait]
pub trait StreamProcessor<In, Out>: Send + Sync {
    /// Processes the input stream and returns an output stream.
    async fn process<'a>(&'a self, input: Pin<Box<dyn Stream<Item = In> + Send + 'a>>) -> Pin<Box<dyn Stream<Item = Out> + Send + 'a>>;
}

/// A simple map processor that applies a synchronous function to each element.
pub struct MapProcessor<F, In, Out>
where
    F: Fn(In) -> Out + Send + Sync + Copy + 'static, // 'static because it's stored in the struct
    In: Send + Sync + 'static, // Added Sync
    Out: Send + Sync + 'static, // Added Sync
{
    map_fn: F,
    _phantom_in: std::marker::PhantomData<In>,
    _phantom_out: std::marker::PhantomData<Out>,
}

impl<F, In, Out> MapProcessor<F, In, Out>
where
    F: Fn(In) -> Out + Send + Sync + Copy + 'static,
    In: Send + Sync + 'static, // Added Sync
    Out: Send + Sync + 'static, // Added Sync
{
    pub fn new(map_fn: F) -> Self {
        MapProcessor {
            map_fn,
            _phantom_in: std::marker::PhantomData,
            _phantom_out: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<F, In, Out> StreamProcessor<In, Out> for MapProcessor<F, In, Out>
where
    F: Fn(In) -> Out + Send + Sync + Copy + 'static,
    In: Send + Sync + 'static, // Added Sync
    Out: Send + Sync + 'static, // Added Sync
{
    async fn process<'a>(&'a self, input: Pin<Box<dyn Stream<Item = In> + Send + 'a>>) -> Pin<Box<dyn Stream<Item = Out> + Send + 'a>> {
        let map_fn = self.map_fn; // Copy map_fn to be moved into the async block
        Box::pin(input.map(move |item| map_fn(item)))
    }
}

/// A filter processor that filters elements based on a predicate.
pub struct FilterProcessor<P, Item>
where
    P: Fn(&Item) -> bool + Send + Sync + Copy + 'static,
    Item: Send + Sync + Clone + 'static, // Added Clone
{
    predicate: P,
    _phantom_item: std::marker::PhantomData<Item>,
}

impl<P, Item> FilterProcessor<P, Item>
where
    P: Fn(&Item) -> bool + Send + Sync + Copy + 'static,
    Item: Send + Sync + Clone + 'static, // Added Clone
{
    pub fn new(predicate: P) -> Self {
        FilterProcessor {
            predicate,
            _phantom_item: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<P, Item> StreamProcessor<Item, Item> for FilterProcessor<P, Item>
where
    P: Fn(&Item) -> bool + Send + Sync + Copy + 'static,
    Item: Send + Sync + Clone + 'static, 
{
    async fn process<'a>(&'a self, input: Pin<Box<dyn Stream<Item = Item> + Send + 'a>>) -> Pin<Box<dyn Stream<Item = Item> + Send + 'a>> {
        let predicate_fn = self.predicate; 
        Box::pin(input.filter(move |item: &Item| {
            // Clone the item to ensure the async block owns its data if needed,
            // or if the predicate needs an owned item.
            // However, the predicate P takes &Item.
            // The issue is the lifetime of the reference `item` passed to the closure
            // versus the lifetime of the Future returned by the closure.
            // The `async move` block captures `item` by reference if not explicitly moved.
            // Let's ensure predicate_fn is called with a reference that lives long enough.
            // The item reference from filter is valid for the call to the closure.
            // The async block needs to ensure its captures are valid.
            let p = predicate_fn; // predicate_fn is Copy
            let i = item.clone(); // item is cloned, so `i` is owned by the async block
            async move { p(&i) } // p operates on a reference to the owned `i`
        }))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use futures_util::stream;
    use futures_util::pin_mut; // For pinning the stream

    #[tokio::test]
    async fn test_map_processor() {
        let numbers = vec![1, 2, 3, 4, 5];
        let input_stream = stream::iter(numbers);
        pin_mut!(input_stream); // Pin the stream on the stack

        let map_fn = |x: i32| x * 2;
        let processor = MapProcessor::new(map_fn);

        let output_stream = processor.process(Box::pin(input_stream)).await;
        let results: Vec<i32> = output_stream.collect().await;

        assert_eq!(results, vec![2, 4, 6, 8, 10]);
    }

    #[tokio::test]
    async fn test_map_processor_string() {
        let strings = vec!["hello".to_string(), "world".to_string()];
        let input_stream = stream::iter(strings);
        pin_mut!(input_stream);

        let map_fn = |s: String| s.to_uppercase();
        let processor = MapProcessor::new(map_fn);

        let output_stream = processor.process(Box::pin(input_stream)).await;
        let results: Vec<String> = output_stream.collect().await;

        assert_eq!(results, vec!["HELLO".to_string(), "WORLD".to_string()]);
    }
    
    #[tokio::test]
    async fn test_filter_processor() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let input_stream = stream::iter(numbers);
        pin_mut!(input_stream);

        let predicate = |x: &i32| *x % 2 == 0; // Filter even numbers
        let processor = FilterProcessor::new(predicate);

        let output_stream = processor.process(Box::pin(input_stream)).await;
        let results: Vec<i32> = output_stream.collect().await;

        assert_eq!(results, vec![2, 4, 6]);
    }

    #[tokio::test]
    async fn test_chained_processors() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let input_stream = stream::iter(numbers);
        pin_mut!(input_stream);

        let filter_even = FilterProcessor::new(|x: &i32| *x % 2 == 0);
        let double_values = MapProcessor::new(|x: i32| x * 2);
        
        // Process with filter_even
        let filtered_stream = filter_even.process(Box::pin(input_stream)).await;
        
        // Process the output of filter_even with double_values
        // Note: The stream needs to be collected and re-streamed or handled carefully if it's not Clone.
        // For this test, we'll collect and then create a new stream.
        // In a real pipeline, you'd chain them more directly if the Stream trait allowed.
        // However, our `process` method consumes and returns a new Pin<Box<dyn Stream>>,
        // so we can chain them by passing the output of one as input to the next.

        let doubled_stream = double_values.process(filtered_stream).await;
        let results: Vec<i32> = doubled_stream.collect().await;

        // Expected: filter evens -> [2, 4, 6, 8, 10] -> double -> [4, 8, 12, 16, 20]
        assert_eq!(results, vec![4, 8, 12, 16, 20]);
    }
}
