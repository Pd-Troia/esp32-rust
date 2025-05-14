use std::sync::{Arc, Mutex};

use esp_idf_hal::{gpio::PinDriver, io::Write, prelude::Peripherals};
use esp_idf_svc::{eventloop::EspSystemEventLoop, http::{server::{Configuration, EspHttpServer, Request}, Method}, nvs::EspDefaultNvsPartition, wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration as WifiConfiguration, EspWifi}};

fn response_HTML(req: Request<&mut esp_idf_svc::http::server::EspHttpConnection<'_>>)->Result<(), esp_idf_hal::io::EspIOError>{
     req.into_ok_response()?.write_all(b"<html><body><a href='/verde'>Verde</a><br><a href='/vermelho'>Vermelho</a></body></html>")  
}

fn connect_wifi(wifi: &mut BlockingWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    // insert your wifi network  
    let SSID  = "";
    let PASSWORD  = "";

    let wifi_configuration: WifiConfiguration = WifiConfiguration::Client(ClientConfiguration {
        ssid:       SSID.try_into().unwrap(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.try_into().unwrap(),
        channel: None,
        ..Default::default()
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start()?;
    log::info!("Wifi started");

    wifi.connect()?;
    log::info!("Wifi connected");

    wifi.wait_netif_up()?;
    log::info!("Wifi netif up");

    Ok(())
}
pub fn web_server()->Result<(),anyhow::Error>{
    let peripherals: Peripherals = Peripherals::take()?;
    let sys_loop: esp_idf_svc::eventloop::EspEventLoop<esp_idf_svc::eventloop::System> = EspSystemEventLoop::take()?;
    let nvs: esp_idf_svc::nvs::EspNvsPartition<esp_idf_svc::nvs::NvsDefault> = EspDefaultNvsPartition::take()?;
    
    let pin_vermelho = Arc::new(Mutex::new(PinDriver::output(peripherals.pins.gpio5)?));
    let pin_verde = Arc::new(Mutex::new(PinDriver::output(peripherals.pins.gpio18)?));
    //wifi 
    let mut wifi: BlockingWifi<EspWifi<'_>> = BlockingWifi::wrap( 
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?, 
        sys_loop.clone()
    )?;    
    let _=connect_wifi(&mut wifi);   
    
    // server
    let mut server: EspHttpServer<'static> = EspHttpServer::new(&Configuration::default())? ;  

    server.fn_handler("/", Method::Get, |req: esp_idf_svc::http::server::Request<&mut esp_idf_svc::http::server::EspHttpConnection<'_>>| {        
        response_HTML(req)        
    })?;  
    server.fn_handler("/verde", Method::Get, move|req: esp_idf_svc::http::server::Request<&mut esp_idf_svc::http::server::EspHttpConnection<'_>>| {        
        pin_verde.lock().unwrap().toggle()?;
        response_HTML(req)        
    })?;  
    server.fn_handler("/vermelho", Method::Get, move|req: esp_idf_svc::http::server::Request<&mut esp_idf_svc::http::server::EspHttpConnection<'_>>| {        
        pin_vermelho.lock().unwrap().toggle()?;
        response_HTML(req)        
    })?;  

     loop{
         std::thread::sleep(std::time::Duration::from_secs(60));
     }

   
}