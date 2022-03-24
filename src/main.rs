// Disable windows console on release builds
#![cfg_attr(windows, cfg_attr(not(debug_assertions), windows_subsystem = "windows"))]

use fltk::prelude::*;
use chrono::prelude::*;
use indexmap::indexmap;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    #[cfg(debug_assertions)]
    println!("DEBUG >>> App: {} v{}", APP_NAME, APP_VERSION);

    // İller
    let cities = indexmap!{
        "Adana" => "9146/adana-icin-namaz-vakti",
        "Adıyaman" => "9158/adiyaman-icin-namaz-vakti",
        "Afyonkarahisar" => "9167/afyonkarahisar-icin-namaz-vakti",
        "Ağrı" => "9185/agri-icin-namaz-vakti",
        "Aksaray" => "9193/aksaray-icin-namaz-vakti",
        "Amasya" => "9198/amasya-icin-namaz-vakti",
        "Ankara" => "9206/ankara-icin-namaz-vakti",
        "Antalya" => "9225/antalya-icin-namaz-vakti",
        "Ardahan" => "9238/ardahan-icin-namaz-vakti",
        "Artvin" => "9246/artvin-icin-namaz-vakti",
        "Aydın" => "9252/aydin-icin-namaz-vakti",
        "Balıkesir" => "9270/balikesir-icin-namaz-vakti",
        "Bartın" => "9285/bartin-icin-namaz-vakti",
        "Batman" => "9288/batman-icin-namaz-vakti",
        "Bayburt" => "9295/bayburt-icin-namaz-vakti",
        "Bilecik" => "9297/bilecik-icin-namaz-vakti",
        "Bingöl" => "9303/bingol-icin-namaz-vakti",
        "Bitlis" => "9311/bitlis-icin-namaz-vakti",
        "Bolu" => "9315/bolu-icin-namaz-vakti",
        "Burdur" => "9327/burdur-icin-namaz-vakti",
        "Bursa" => "9335/bursa-icin-namaz-vakti",
        "Çanakkale" => "9352/canakkale-icin-namaz-vakti",
        "Çankırı" => "9359/cankiri-icin-namaz-vakti",
        "Çorum" => "9370/corum-icin-namaz-vakti",
        "Denizli" => "9392/denizli-icin-namaz-vakti",
        "Diyarbakır" => "9402/diyarbakir-icin-namaz-vakti",
        "Düzce" => "9414/duzce-icin-namaz-vakti",
        "Edirne" => "9419/edirne-icin-namaz-vakti",
        "Elazığ" => "9432/elazig-icin-namaz-vakti",
        "Erzincan" => "9440/erzincan-icin-namaz-vakti",
        "Erzurum" => "9451/erzurum-icin-namaz-vakti",
        "Eskişehir" => "9470/eskisehir-icin-namaz-vakti",
        "Gaziantep" => "9479/gaziantep-icin-namaz-vakti",
        "Giresun" => "9494/giresun-icin-namaz-vakti",
        "Gümüşhane" => "9501/gumushane-icin-namaz-vakti",
        "Hakkari" => "9507/hakkari-icin-namaz-vakti",
        "Hatay" => "20089/hatay-icin-namaz-vakti",
        "Iğdır" => "9522/igdir-icin-namaz-vakti",
        "Isparta" => "9528/isparta-icin-namaz-vakti",
        "İstanbul" => "9541/istanbul-icin-namaz-vakti",
        "İzmir" => "9560/izmir-icin-namaz-vakti",
        "Kahramanmaraş" => "9577/kahramanmaras-icin-namaz-vakti",
        "Karabük" => "9581/karabuk-icin-namaz-vakti",
        "Karaman" => "9587/karaman-icin-namaz-vakti",
        "Kars" => "9594/kars-icin-namaz-vakti",
        "Kastamonu" => "9609/kastamonu-icin-namaz-vakti",
        "Kayseri" => "9620/kayseri-icin-namaz-vakti",
        "Kilis" => "9629/kilis-icin-namaz-vakti",
        "kırıkkale" => "9635/kirikkale-icin-namaz-vakti",
        "Kırklareli" => "9638/kirklareli-icin-namaz-vakti",
        "Kırşehir" => "9646/kirsehir-icin-namaz-vakti",
        "Kocaeli" => "9654/kocaeli-icin-namaz-vakti",
        "Konya" => "9676/konya-icin-namaz-vakti",
        "Kütahya" => "9689/kutahya-icin-namaz-vakti",
        "Malatya" => "9703/malatya-icin-namaz-vakti",
        "Manisa" => "9716/manisa-icin-namaz-vakti",
        "Mardin" => "9726/mardin-icin-namaz-vakti",
        "Mersin" => "9737/mersin-icin-namaz-vakti",
        "Muğla" => "9747/mugla-icin-namaz-vakti",
        "Muş" => "9755/mus-icin-namaz-vakti",
        "Nevşehir" => "9760/nevsehir-icin-namaz-vakti",
        "Niğde" => "9766/nigde-icin-namaz-vakti",
        "Ordu" => "9782/ordu-icin-namaz-vakti",
        "Osmaniye" => "9788/osmaniye-icin-namaz-vakti",
        "Rize" => "9799/rize-icin-namaz-vakti",
        "Sakarya" => "9807/sakarya-icin-namaz-vakti",
        "Samsun" => "9819/samsun-icin-namaz-vakti",
        "Şanlıurfa" => "9831/sanliurfa-icin-namaz-vakti",
        "Siirt" => "9839/siirt-icin-namaz-vakti",
        "Sinop" => "9847/sinop-icin-namaz-vakti",
        "Şırnak" => "9854/sirnak-icin-namaz-vakti",
        "Sivas" => "9868/sivas-icin-namaz-vakti",
        "Tekirdağ" => "9879/tekirdag-icin-namaz-vakti",
        "Tokat" => "9887/tokat-icin-namaz-vakti",
        "Trabzon" => "9905/trabzon-icin-namaz-vakti",
        "Tunceli" => "9914/tunceli-icin-namaz-vakti",
        "Uşak" => "9919/usak-icin-namaz-vakti",
        "Van" => "9930/van-icin-namaz-vakti",
        "Yalova" => "9935/yalova-icin-namaz-vakti",
        "Yozgat" => "9949/yozgat-icin-namaz-vakti",
        "Zonguldak" => "9955/zonguldak-icin-namaz-vakti",
    };

    // FLTK app
    let app = fltk::app::App::default().with_scheme(fltk::app::Scheme::Gtk);
    fltk::app::set_visible_focus(false);

    // FLTK window
    let mut ui_win = fltk::window::Window::default().with_size(270, 140).center_screen().with_label(&(APP_NAME.to_owned() + " v" + APP_VERSION)).center_screen();
    ui_win.make_resizable(false);
    // ui_win.set_icon(Some(fltk::image::PngImage::from_data(include_bytes!("../icon.png")).unwrap()));

    // UI List Cities 
    let mut ui_list_cities = fltk::misc::InputChoice::default().with_size(250, 30).with_pos(10, 10).with_label("");
    ui_list_cities.set_tooltip("Konum Seçiniz");
    ui_list_cities.set_value("Konum Seçiniz");
    for (city, _) in cities.iter() {
        ui_list_cities.add(city);
    }

    // UI Time
    let mut ui_time = fltk::text::TextDisplay::default().with_size(100, 30).with_pos(10, 100).with_label("Saat");
    ui_time.set_buffer(fltk::text::TextBuffer::default());
    ui_time.set_text_size(20);
    fltk::text::TextBuffer::set_text(&mut ui_time.buffer().unwrap(), "     .....");
    let mut ui_time_iftar = fltk::text::TextDisplay::default().with_size(100, 30).with_pos(160, 100).with_label("İftar");
    ui_time_iftar.set_buffer(fltk::text::TextBuffer::default());
    ui_time_iftar.set_text_size(20);
    fltk::text::TextBuffer::set_text(&mut ui_time_iftar.buffer().unwrap(), "     .....   ");

    // let input = fltk::input::Input::default().with_size(250, 30).with_pos(110, 10).with_label("Şehrin URL'si:");

    let mut ui_button_fetch = fltk::button::Button::default().with_size(75, 30).with_pos(98, 55).with_label("Kontrol Et");
    ui_button_fetch.set_callback({
        let mut wn = ui_win.clone();
        move |_| {
            wn.set_cursor(fltk::enums::Cursor::Wait);
            let location_input = ui_list_cities.value().unwrap();

            #[cfg(debug_assertions)]
            println!("DEBUG >>> Girilen şehir: {}", location_input);
    
            let url_diyanet = "https://namazvakitleri.diyanet.gov.tr/tr-TR/";
    
            // Şehir kodu
            let location_url = match cities.get(&location_input[..]) {
                Some(&res) => res,
                _ => {
                    wn.set_cursor(fltk::enums::Cursor::Default);
                    fltk::text::TextBuffer::set_text(&mut ui_time.buffer().unwrap(), "Hatalı");
                    fltk::text::TextBuffer::set_text(&mut ui_time_iftar.buffer().unwrap(), "Konum");
                    #[cfg(debug_assertions)]
                    println!("DEBUG >>> HATA: Girdiğiniz şehir bulunamadı! İsimleri bitişik ve küçük harflerle yazınız.");
                    return;
                },
            };
    
            // Final URL
            let url = &(url_diyanet.to_owned() + location_url)[..];
            let response = match  minreq::get(url).send() {
                Ok(res) => res,
                Err(_) => {
                    wn.set_cursor(fltk::enums::Cursor::Default);
                    fltk::text::TextBuffer::set_text(&mut ui_time.buffer().unwrap(), "Bağlantı");
                    fltk::text::TextBuffer::set_text(&mut ui_time_iftar.buffer().unwrap(), "Hatası");
                    #[cfg(debug_assertions)]
                    println!("DEBUG >>> HATA: Sunucuya bağlanılamadı! İnternete bağlı olduğunuzdan emin olunuz.");
                    return;
                },
            };
    
            // Sunucudan gelen yanıtı yazıya çevirme
            let response = match response.as_str() {
                Ok(res) => res,
                Err(_) => {
                    wn.set_cursor(fltk::enums::Cursor::Default);
                    fltk::text::TextBuffer::set_text(&mut ui_time.buffer().unwrap(), "Encode");
                    fltk::text::TextBuffer::set_text(&mut ui_time_iftar.buffer().unwrap(), "Hatası");
                    #[cfg(debug_assertions)]
                    println!("DEBUG >>> HATA: Sunucudan alınan veri okunamadı! (UTF-8 hatası)");
                    return;
                },
            };
    
            // CSS selector kullanarak iftar saati bilgisini bulma
            let fragment = scraper::Html::parse_fragment(response);
            let selector = match scraper::Selector::parse("div.tpt-cell[data-vakit-name='aksam'] .tpt-time") {
                Ok(res) => res,
                Err(_) => {
                    wn.set_cursor(fltk::enums::Cursor::Default);
                    #[cfg(debug_assertions)]
                    fltk::text::TextBuffer::set_text(&mut ui_time.buffer().unwrap(), "SiteEncode");
                    fltk::text::TextBuffer::set_text(&mut ui_time_iftar.buffer().unwrap(), "Hatası");
                    println!("DEBUG >>> HATA: İftar saati bulunamadı! Tekrar deneyiniz.");
                    return;
                },
            };
            let iftar = fragment.select(&selector).next().unwrap().inner_html();
            #[cfg(debug_assertions)]
            println!("DEBUG >>> iftar: {}", iftar);
    
            // Zaman hesaplamaları
            let now = Local::now();
            #[cfg(debug_assertions)]
            let iftar_split: Vec<&str> = iftar.split(":").collect();
            #[cfg(debug_assertions)]
            let iftar_time = Local.ymd(now.year(), now.month(), now.day()).and_hms(iftar_split[0].parse::<u32>().unwrap(), iftar_split[1].parse::<u32>().unwrap(), 0);
    
            #[cfg(debug_assertions)]
            {
                println!("DEBUG >>> iftar_timestamp: {}", iftar_time);
                println!("DEBUG >>> now_timestamp: {}", now);
                println!("DEBUG >>> kalan_timestamp: {}", Local.timestamp(iftar_time.timestamp() - now.timestamp(), 0));
                println!("DEBUG >>> geçen_timestamp: {}", Local.timestamp(now.timestamp() - iftar_time.timestamp(), 0));
            }
    
            fltk::text::TextBuffer::set_text(&mut ui_time.buffer().unwrap(), &("   ".to_owned() + &now.format("%H:%M").to_string()[..])[..]);
            fltk::text::TextBuffer::set_text(&mut ui_time_iftar.buffer().unwrap(), &("   ".to_string() + &iftar[..])[..]);
            #[cfg(debug_assertions)]
            {
                println!("DEBUG >>> Saat: {}", now.format("%H:%M"));
                println!("DEBUG >>> İftar vakti: {}", iftar);
            }

            wn.set_cursor(fltk::enums::Cursor::Default);
        }
    });

    ui_win.end();
    ui_win.show();
    app.run().unwrap();
}