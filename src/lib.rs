use rdkafka::producer::FutureProducer;

mod consume;
use {
    consume::consume,
    clap::ArgMatches,
    log::{info},
    rdkafka::{
        message::BorrowedMessage,
    },
    crate::crates::{
        kafka
    },
};
mod crates {
    pub mod kafka;
}
use {rdkafka::util::get_rdkafka_version};

pub async fn subscribe(args: &ArgMatches, process_message: &dyn Fn(&BorrowedMessage)){
    let (ver_n, ver_s) = get_rdkafka_version();
    info!("rd_kafka_Version: 0x{:08x}, {}", ver_n, ver_s);
    info!("sb kafka lib");
    consume(&args,process_message).await;
}

pub async fn publisher(args: &ArgMatches) -> FutureProducer{
    let (ver_n, ver_s) = get_rdkafka_version();
    info!("rd_kafka_Version: 0x{:08x}, {}", ver_n, ver_s);
    info!("sb kafka lib");
    let producer = kafka::producer(args);
    producer
}

