
#[tokio::test]
#[cfg(feature = "aws")]
async fn test_bucket_exists() {
    use crate::types::bucket::{Buckets};
    use crate::providers;
    let mut aws_buckets = providers::aws::AwsBuckets::new(
        "us-east-2"
        );
    let resp = aws_buckets.exists(
        String::from("waihona")
        ).await;
    assert!(resp);
}

#[tokio::test]
#[cfg(feature = "aws")]
async fn test_bucket_open() {
    use crate::types::bucket::{Buckets, Bucket};
    use crate::providers;
    let mut aws_buckets = providers::aws::AwsBuckets::new(
        "us-east-2"
        );
    let waihona = aws_buckets.open(
        String::from("waihona"),
        ).await.unwrap();
    let blobs = waihona.list_blobs(None).await;
    println!("{:?}", blobs);
}

#[tokio::test]
#[cfg(feature = "aws")]
async fn test_get_blob() {
    use crate::types::bucket::{Buckets, Bucket};
    use crate::types::blob::{Blob};
    use crate::providers;
    use bytes::Bytes;
    let mut aws_buckets = providers::aws::AwsBuckets::new(
        "us-east-2"
        );
    let waihona = aws_buckets.open(
        String::from("waihona"),
        ).await.unwrap();
    let mut blob = waihona.get_blob(
        "reka-store.txt".to_owned(),
        None
        ).await
        .unwrap();

    let res = blob.read().await.unwrap();
    let res_str = std::str::from_utf8(&res);
    println!("{:?}", res_str);
//    // write data to blob
//    let res_write = waihona.write_blob(
//        "copy-reka.txt".to_owned(),
//        Some(Bytes::from("Hello world"))
//        ).await
//        .unwrap();
//    let cp_blob = waihona.copy_blob(
//        "reka-store.txt".to_owned(),
//        "waihona/copy-reka.txt".to_owned(),
//        None
//        ).await
//        .unwrap();
//    println!("{:?}", cp_blob);
//    cp_blob.copy(
//        "waihona/sec-copy.txt".to_owned(),
//        None
//        ).await;
//    let res = blob.delete().await;
//    println!("{:?}", res);
}
