use candid::Principal;

fn main() {
    let a = Principal::from_text("6qkcf-dx3kf-uy5c5-ohlq2-hrljo-a4xqz-kbpsi-h3p72-gmr4t-usdws-vqe").unwrap();
    let b = Principal::from_text("7de4h-k5kdt-mkuaz-r7gcf-iumnb-4unbn-3nzq2-smemt-uqtnl-eamtp-pae").unwrap();
    let c = Principal::from_text("e6arx-mlkkz-qnxnm-oedka-xws2o-re4ov-bhtgc-jhjp3-riog5-qh6yf-rae").unwrap();
    let d = Principal::from_text("yhy6j-huy54-mkzda-m26hc-yklb3-dzz4l-i2ykq-kr7tx-dhxyf-v2c2g-tae").unwrap();

    let can = Principal::from_text("wzsds-oiaaa-aaaal-aak2q-cai").unwrap();
    // println!("{:?}\n{:?}\n{:?}\n{:?}", a.as_slice().len(), b.as_slice().len(), c.as_slice().len(), d.as_slice().len());
    println!("{:?}", d.as_slice());
    println!("{:?}", can.as_slice());

    let u = u64::from_be_bytes(can.as_slice()[..8].try_into().unwrap());
    println!("{:?}",  u);
}
