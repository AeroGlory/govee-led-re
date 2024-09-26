use tokio::time::{self, Duration};
use btleplug::{self, api::Peripheral, platform::Manager};
use tokio;
use btleplug::api::{Central, Manager as _, ScanFilter};
//use btleplug::Result;

#[tokio::main]
async fn main() {
    let manager = Manager::new().await.unwrap();
    // get the first bluetooth adapter
    let adapters = manager.adapters()
        .await
        .expect("Unable to fetch adapter list.")
        .into_iter()
        .nth(0)
        .expect("Unable to find adapters.");

    //dbg!(&adapters);

    adapters.start_scan(ScanFilter::default()).await.unwrap();

    time::sleep(Duration::from_secs(5)).await;

    /*for i in &adapters.peripherals().await.unwrap() {
        dbg!(&i);
    }
*/
    for p in &adapters.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("GBK_H619E_6FAD"))
        {
            //return Some(p) 
            dbg!(p);
            let light_strip = p;
        }
        else {
            panic!("No light found :<")
        }


    }
}
