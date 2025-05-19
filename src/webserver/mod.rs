use std::sync::{Arc, Mutex};

use esp_idf_hal::{gpio::{Gpio18, Gpio5, Output, PinDriver}, io::Write, prelude::Peripherals};
use esp_idf_svc::{eventloop::EspSystemEventLoop, http::{server::{Configuration, EspHttpServer, Request}, Method}, nvs::EspDefaultNvsPartition, wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration as WifiConfiguration, EspWifi}};

fn response_HTML(
    req: Request<&mut esp_idf_svc::http::server::EspHttpConnection<'_>>,
    pin_vermelho:&PinDriver<'_, Gpio5, Output>,
    pin_verde:& PinDriver<'_, Gpio18, Output>
)->Result<(), esp_idf_hal::io::EspIOError>{
    let state_verde= if pin_verde.is_set_high() {  "aceso" } else{ "apagado"}; 
    let state_vermelho= if pin_vermelho.is_set_high() {  "aceso" } else{ "apagado"};
    let html_str = format!("
        <html>
            <body>
                <a href='/verde'>Verde<span class='state_verde'>&nbsp{}</span></a>
                <br>
                <a href='/vermelho'>Vermelho<span class='state_vermelho'>&nbsp{}</span></a>
            </body>
        </html>",
        state_verde,state_vermelho);
    let bhtml = html_str.as_bytes();
    req.into_ok_response()?.write_all(bhtml) 
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
    
    let pin_vermelho: Arc<Mutex<PinDriver<'_, esp_idf_hal::gpio::Gpio5, esp_idf_hal::gpio::Output>>> = Arc::new(Mutex::new(PinDriver::output(peripherals.pins.gpio5)?));
    let pin_verde: Arc<Mutex<PinDriver<'_, esp_idf_hal::gpio::Gpio18, esp_idf_hal::gpio::Output>>> = Arc::new(Mutex::new(PinDriver::output(peripherals.pins.gpio18)?));
    //wifi 
    let mut wifi: BlockingWifi<EspWifi<'_>> = BlockingWifi::wrap( 
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?, 
        sys_loop.clone()
    )?;
 
    let _ =connect_wifi(&mut wifi);   
    
    // server
    let mut server: EspHttpServer<'static> = EspHttpServer::new(&Configuration::default())? ;  

    let pin_verde_1 = Arc::clone(&pin_verde);
    let pin_vermelho_1 = Arc::clone(&pin_vermelho);
    server.fn_handler("/", Method::Get, move |req| {
        let pin_verde = pin_verde_1.lock().unwrap();
        let pin_vermelho = pin_vermelho_1.lock().unwrap();
        response_HTML(req, &*pin_vermelho, &*pin_verde)
    })?;

    let pin_verde_2 = Arc::clone(&pin_verde);
    let pin_vermelho_2 = Arc::clone(&pin_vermelho);
    server.fn_handler("/verde", Method::Get, move |req| {
        let mut pin_verde = pin_verde_2.lock().unwrap();
        let pin_vermelho = pin_vermelho_2.lock().unwrap();
        pin_verde.toggle()?;
        response_HTML(req, &*pin_vermelho, &*pin_verde)
    })?;

    let pin_verde_3 = Arc::clone(&pin_verde);
    let pin_vermelho_3 = Arc::clone(&pin_vermelho);
    server.fn_handler("/vermelho", Method::Get, move |req| {
        let pin_verde = pin_verde_3.lock().unwrap();
        let mut pin_vermelho = pin_vermelho_3.lock().unwrap();
        pin_vermelho.toggle()?;
        response_HTML(req, &*pin_vermelho, &*pin_verde)
    })?;
     loop{
         std::thread::sleep(std::time::Duration::from_secs(60));
     }

   
}