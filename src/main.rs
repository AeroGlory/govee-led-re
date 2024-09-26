use tokio::time::{self, Duration};
use btleplug::{self, api::Peripheral as _, platform::{Adapter, Manager}};
use tokio;
use btleplug::api::{Central, Manager as _, ScanFilter};
use btleplug::platform::Peripheral;
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

let light_strip = find_light(&adapters).await.expect("No lights found");



}

async fn find_light(adapter: &Adapter) -> Option<Peripheral> {
    for p in adapter.peripherals().await.unwrap() {
        if p.properties()
            .await
            .unwrap()
            .unwrap()
            .local_name
            .iter()
            .any(|name| name.contains("GBK_H619E_6FAD"))
        {
            //return Some(p) 
            dbg!(&p);
            return Some(p)
        }
    }

    None
}