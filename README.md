#SBKafka

Easy consumption of rdkafka

### import
```
use {
    sbkafka::{subscribe,publisher}
};
```

### To consume
```
let consume = subscribe;
let _producer = publisher(&args);
consume(&args,&process_msg).await;
```

#### where process_msg is: 
``` 
fn process_msg(message:&BorrowedMessage) {
    let payload = message
    .payload_view::<str>()
    .unwrap_or(Ok(""))
    .unwrap_or_else(|e| {
        error!("Error while deserializing payload: {:?}", e);
        ""
    });
}
```

### To produce
``` 
let producer = publisher(&args);
```