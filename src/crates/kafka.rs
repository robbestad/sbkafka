use {
    clap::ArgMatches,
    log::info,
    rdkafka::{
        client::ClientContext,
        config::{ClientConfig, RDKafkaLogLevel},
        consumer::stream_consumer::StreamConsumer,
        consumer::{ConsumerContext, Rebalance},
        producer::FutureProducer,
    },
};
pub struct LoggingContext;

impl ClientContext for LoggingContext {}

impl ConsumerContext for LoggingContext {
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        info!("Pre rebalance {:?}", rebalance);
    }
    fn post_rebalance(&self, rebalance: &Rebalance) {
        info!("Post rebalance {:?}", rebalance);
    }
}

pub fn consumer(args: &ArgMatches) -> StreamConsumer<LoggingContext> {
    let brokers = args.value_of("brokers").unwrap();
    let group_id = args.value_of("group-id").unwrap();
    let consumer: StreamConsumer<LoggingContext> = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set_log_level(RDKafkaLogLevel::Debug)
        .create_with_context(LoggingContext)
        .expect("Consumer creation failed");
    consumer
}
pub fn producer(args: &ArgMatches) -> FutureProducer {
    let brokers = args.value_of("brokers").unwrap();
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");
    producer
}
