use {
    crate::crates::{
        kafka
    },
    clap::ArgMatches,
    log::{error},
    rdkafka::consumer::{Consumer},
    rdkafka::{
        message::BorrowedMessage,
    },
};

pub async fn consume(args: &ArgMatches, process_message: &dyn Fn(&BorrowedMessage)) -> ! {
    let topics: Vec<&str> = args.values_of("topics").unwrap().collect();
    let consumer = kafka::consumer(args);

    consumer
        .subscribe(&topics)
        .unwrap_or_else(|_| panic!("Failed to subscribe to topics {:?}", topics));

    loop {
        let r = consumer.recv().await;
        match r {
            Err(e) => error!("Kafka error: {}", e),
            Ok(m) => {
                process_message(&m);
            }
        }
    }
}
