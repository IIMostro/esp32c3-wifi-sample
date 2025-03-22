use std::{str::FromStr, sync::{Arc, Mutex}};
use embedded_svc::http::client::Client;
use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::{delay::FreeRtos, gpio::PinDriver, prelude::Peripherals, units::ValueType}, http::{client::EspHttpConnection, server::EspHttpServer}, io::utils::try_read_full, nvs::EspDefaultNvsPartition, sys, wifi::{AccessPointConfiguration, BlockingWifi, ClientConfiguration, Configuration, EspWifi}};

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();
    let eventloop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();
    let mut wifi = BlockingWifi::wrap(EspWifi::new(peripherals.modem, eventloop.clone(), Some(nvs)).unwrap(), eventloop).unwrap();
    log::info!("configuration wifi!");

    // sta连接
    // let wifi_configuration = ClientConfiguration{
    //     ssid: heapless::String::from_str("gonewiththewind").unwrap(),
    //     password: heapless::String::from_str("wangtao0303").unwrap(),
    //     ..Default::default()
    // };
    // let config = embedded_svc::wifi::Configuration::Client(wifi_configuration);

    // ap连接
    let config = Configuration::AccessPoint(AccessPointConfiguration{
        ssid: heapless::String::from_str("esp32c3-test").unwrap(),
        password: heapless::String::from_str("1234").unwrap(),
        ..Default::default()
    });

    wifi.set_configuration(&config).unwrap();
    log::info!("start wifi");
    wifi.start().unwrap();

    // WiFi sta连接
    // log::info!("connect wifi!");
    // wifi.connect().unwrap();
    log::info!("wait wifi derive start!!!!");
    wifi.wait_netif_up().unwrap();

    // WiFi sta连接
    // log::info!("get ip:{:?}", wifi.wifi().sta_netif().get_ip_info().unwrap());

    // wifi ap连接
    log::info!("get ip:{:?}", wifi.wifi().ap_netif().get_ip_info().unwrap());
    std::mem::forget(wifi);
    
    // stawifi连接需要阻塞线程

    // loop {
    //     FreeRtos::delay_ms(1000);
    // }

    // http client

    // let mut client = Client::wrap(EspHttpConnection::new(&Default::default()).unwrap());
    // let url = "http://httpbin.org/get";
    // let mut response = client.get(url).unwrap().submit().unwrap();
    // log::info!("response status: {}", response.status());
    // let (_headers, body) = response.split();
    // let mut buf = [0;1024];
    // let br = try_read_full(body, &mut buf).unwrap();
    // let body = str::from_utf8(&buf[0..br]).unwrap();
    // log::info!("response text:{}", body);

    // http server
    
    // let led4 = Arc::new(Mutex::new(PinDriver::output(peripherals.pins.gpio12).unwrap()));
    // let led5 = Arc::new(Mutex::new(PinDriver::output(peripherals.pins.gpio13).unwrap()));

    // let mut server = EspHttpServer::new(&Default::default()).unwrap();
    // server.fn_handler("/", esp_idf_svc::http::Method::Get, move |req| -> Result<(), _> {
    //     return match req.into_ok_response().unwrap().write("index.html".as_bytes()) {
    //         Ok(_) => {
    //             led4.lock().unwrap().toggle().unwrap();
    //             led5.lock().unwrap().toggle().unwrap();
    //             Ok(())
    //         },
    //         Err(err) => Err(err),
    //     }
    // }).unwrap();
    // log::info!("wait connecting!!!!");
    // loop{}
}
