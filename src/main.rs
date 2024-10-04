mod light_cmd;

use std::io::{self, Write};
use tokio::time::{self, Duration};
use btleplug::{self, api::{bleuuid::BleUuid, Peripheral as _}, platform::{Adapter, Manager}};
use tokio;
use btleplug::api::{Central, Manager as _, ScanFilter};
use btleplug::platform::Peripheral;
use uuid::uuid;
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
   
   
   
   
   
   /*  let _a = &mut Default::default();
    io::stdin().read_line(_a).expect("Oh well.");
    io::stdout().flush().unwrap();
*/
    let light_strip = find_light(&adapters).await.expect("No lights found");

    light_strip.connect().await.unwrap();

    light_strip.discover_services().await.unwrap();

    let chars = light_strip.characteristics();
    let servs = light_strip.services();

    dbg!(&servs);
    dbg!(&chars);

    let cmd_char = &chars
    .iter()
    .find(|c| c.uuid == uuid!("00010203-0405-0607-0809-0a0b0c0d2b11"))
    .expect("Unable to find characterics");


//Test loop to turn on/off lights repeatedly
loop {
    light_strip.write(cmd_char, &light_cmd::KEEP_ALIVE,  btleplug::api::WriteType::WithoutResponse).await.expect("Unable to send ON command");
    light_strip.write(cmd_char, &light_cmd::TURN_ON,  btleplug::api::WriteType::WithoutResponse).await.expect("Unable to send ON command");
    time::sleep(Duration::from_secs(1)).await;
    light_strip.write(cmd_char, &light_cmd::TURN_OFF,  btleplug::api::WriteType::WithoutResponse).await.expect("Unable to send ON command");
    time::sleep(Duration::from_secs(1)).await;
}

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